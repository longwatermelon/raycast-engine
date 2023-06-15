use raycast::map::Map;
use raycast::util::{Ray, Intersection, IntersectionType};
use raycast::entity::Entity;
use raycast::item::Item;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

#[macroquad::main(window_conf)]
async fn main() {
    let mut textures: HashMap<char, Texture2D> = HashMap::new();
    textures.insert('0', Texture2D::from_file_with_format(include_bytes!("res/wall.png"), Some(ImageFormat::Png)));
    textures.insert('e', Texture2D::from_file_with_format(include_bytes!("res/shrek.png"), Some(ImageFormat::Png)));

    let mut map: Map = Map::new("examples/res/map", textures);
    let mut entities: Vec<Entity> = map.filter_entities(&['e']);

    let mut items: Vec<Item> = vec![
        Item::new("gun", "examples/res/gun.png").await,
        Item::new("knife", "examples/res/knife.png").await
    ];
    let mut selected_index: usize = 0;

    let shooting_gun: Texture2D = Texture2D::from_file_with_format(include_bytes!("res/gun-shoot.png"), Some(ImageFormat::Png));

    let mut cam: Ray = Ray::new(Vec2::new(110., 160.), 0.3);

    let mut prev_mx: f32 = mouse_position().0;

    let mut grabbed: bool = true;
    set_cursor_grab(true);
    show_mouse(false);

    loop {
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        // Controls
        raycast::util::fps_camera_controls(&map, &mut cam, 2.);
        raycast::util::fps_camera_rotation(&mut cam, &mut prev_mx, 1.);

        // Entity move towards player
        entities[0].pos = raycast::util::move_towards_collidable(&map, entities[0].pos, cam.orig, 1.);

        // Shooting mechanic
        if is_mouse_button_pressed(MouseButton::Left) {
            // Animations
            match selected_index {
                0 => items[selected_index].texswap(&shooting_gun, 0.1),
                1 => items[selected_index].jab(Vec2::new(0., -20.), 0.05),
                _ => ()
            }

            // Raycast
            let ins: Intersection = raycast::cast_ray(&map, &entities, cam);
            match ins.itype {
                IntersectionType::Entity {..} => println!("Hit entity"),
                _ => println!("Hit wall")
            }
        }

        // Equip item
        if is_key_pressed(KeyCode::Key1) {
            selected_index = 0;
            raycast::equip_item(&mut items, "gun");
        }

        if is_key_pressed(KeyCode::Key2) {
            selected_index = 1;
            raycast::equip_item(&mut items, "knife");
        }

        clear_background(BLACK);
        raycast::render(&map, &entities, cam);
        raycast::render_item(&mut items);
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Raycast demo"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

