use raycast::prelude as rc;
use macroquad::prelude as mq;
use glam::Vec2;
use std::collections::HashMap;

#[macroquad::main(window_conf)]
async fn main() {
    raycast::util::set_scrw_scrh(800, 800);

    let mut textures: HashMap<char, mq::Image> = HashMap::new();
    textures.insert('0', mq::Image::from_file_with_format(include_bytes!("res/wall.png"), Some(mq::ImageFormat::Png)).unwrap());
    textures.insert('1', mq::Image::from_file_with_format(include_bytes!("res/wall.png"), Some(mq::ImageFormat::Png)).unwrap());
    textures.insert('2', mq::Image::from_file_with_format(include_bytes!("res/wall.png"), Some(mq::ImageFormat::Png)).unwrap());
    textures.insert('3', mq::Image::from_file_with_format(include_bytes!("res/wall.png"), Some(mq::ImageFormat::Png)).unwrap());
    textures.insert('e', mq::Image::from_file_with_format(include_bytes!("res/shrek.png"), Some(mq::ImageFormat::Png)).unwrap());

    let mut map: rc::Map = rc::Map::from_bytes(include_bytes!("res/map"), textures);
    // map.floor_tex(rc::Surface::Texture(mq::Image::from_file_with_format(include_bytes!("res/floor.png"), Some(mq::ImageFormat::Png)).unwrap()));
    // map.ceil_tex(rc::Surface::Texture(mq::Image::from_file_with_format(include_bytes!("res/ceiling.png"), Some(mq::ImageFormat::Png)).unwrap()));
    map.floor_tex(rc::Surface::Color(mq::BEIGE.into()));
    map.ceil_tex(rc::Surface::Color(mq::GRAY.into()));
    map.wall_height('0', 2.);
    // map.wall_height('1', 2.);
    // map.wall_height('2', 3.);
    // map.wall_height('3', 4.);

    let mut entities: Vec<rc::Entity> = map.filter_entities(&['e'], &[(20., 30.)]);

    let mut items: Vec<rc::Item> = vec![
        rc::Item::new("gun", include_bytes!("res/gun.png")),
        rc::Item::new("knife", include_bytes!("res/knife.png")),
    ];
    let mut selected_index: usize = 0;

    let shooting_gun: mq::Texture2D = mq::Texture2D::from_file_with_format(include_bytes!("res/gun-shoot.png"), Some(mq::ImageFormat::Png));

    let mut cam: rc::Ray = rc::Ray::new(Vec2::new(110., 160.), 0.3);

    let mut prev_mpos: (f32, f32) = mq::mouse_position();

    let mut grabbed: bool = true;
    mq::set_cursor_grab(true);
    mq::show_mouse(false);

    let mut last_fps_update: f64 = mq::get_time();
    let mut fps: i32 = mq::get_fps();

    let mut out_img: mq::Image = mq::Image::gen_image_color(
        rc::scrw() as u16,
        rc::scrh() as u16,
        mq::BLACK
    );
    let out_tex: mq::Texture2D = mq::Texture2D::from_image(&out_img);

    loop {
        if mq::is_key_pressed(mq::KeyCode::Tab) {
            grabbed = !grabbed;
            mq::set_cursor_grab(grabbed);
            mq::show_mouse(!grabbed);
        }

        // Controls
        raycast::util::fps_camera_controls(&map, &mut cam, 2.);
        raycast::util::fps_camera_rotation(&mut cam, &mut prev_mpos, 1.);

        // Entity move towards player
        for ent in &mut entities {
            ent.pos = raycast::util::move_towards_collidable(&map, ent.pos, cam.orig, 1.);
        }

        // Shooting mechanic
        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            // Animations
            match selected_index {
                0 => items[selected_index].texswap(&shooting_gun, 0.1),
                1 => items[selected_index].jab(Vec2::new(0., -20.), 0.05),
                _ => ()
            }

            // Raycast
            let ins: rc::Intersection = raycast::cast_ray(&map, entities.iter(), &[], cam);
            match ins.itype {
                rc::IntersectionType::Entity { index, .. } => println!("Hit entity {}", index),
                _ => println!("Hit wall")
            }
        }

        // Equip item
        if mq::is_key_pressed(mq::KeyCode::Key1) {
            selected_index = 0;
            raycast::equip_item(&mut items, "gun");
        }

        if mq::is_key_pressed(mq::KeyCode::Key2) {
            selected_index = 1;
            raycast::equip_item(&mut items, "knife");
        }

        mq::clear_background(mq::BLACK);
        out_img.bytes.fill(0);
        // let all_ents: Vec<&rc::Entity> = entities.iter().collect();
        raycast::render(&map, entities.iter(), cam, rc::Fog::None, &|| 0., &mut out_img);
        out_tex.update(&out_img);

        let (top_x, top_y) = rc::scr_topleft();
        mq::draw_texture(&out_tex, top_x, top_y, mq::WHITE);

        raycast::render_item(&mut items);

        if mq::get_time() - last_fps_update > 0.5 {
            fps = mq::get_fps();
            last_fps_update = mq::get_time();
        }
        mq::draw_text(format!("FPS {}", fps).as_str(), 10., 20., 24., mq::WHITE);

        mq::next_frame().await;
    }
}

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: String::from("Raycast demo"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

