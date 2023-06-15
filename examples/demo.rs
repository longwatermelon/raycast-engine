use raycast::map::Map;
use raycast::util::Ray;
use raycast::entity::Entity;
use macroquad::prelude::*;
use std::collections::HashMap;

#[macroquad::main(window_conf)]
async fn main() {
    let mut textures: HashMap<char, Texture2D> = HashMap::new();
    textures.insert('0', Texture2D::from_file_with_format(include_bytes!("res/wall.png"), Some(ImageFormat::Png)));
    textures.insert('5', Texture2D::from_file_with_format(include_bytes!("res/wall.png"), Some(ImageFormat::Png)));
    textures.insert('e', Texture2D::from_file_with_format(include_bytes!("res/shrek.png"), Some(ImageFormat::Png)));

    let map: Map = Map::new("examples/res/map", textures);
    let mut cam: Ray = Ray::new(Vec2::new(110., 160.), 0.3);

    // let entities: Vec<Entity> = vec![
    //     Entity::new(Vec2::new(200., 200.), 'e'),
    //     Entity::new(Vec2::new(300., 200.), 'e')
    // ];

    let entities: Vec<Entity> = (0..40).map(|i| Entity::new(Vec2::new(200. + i as f32 * 10., 200.), 'e')).collect();

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

        if is_key_down(KeyCode::W) {
            cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, cam.angle).along(2.));
        }

        if is_key_down(KeyCode::Right) {
            cam.angle += 0.1;
        }

        if is_key_down(KeyCode::Left) {
            cam.angle -= 0.1;
        }

        let mx: f32 = mouse_position().0;
        cam.angle += (mx - prev_mx) / 200.;
        prev_mx = mx;

        clear_background(BLACK);

        cam.angle = raycast::util::restrict_angle(cam.angle);
        raycast::render(&map, cam, 800, 800, entities.clone());

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

