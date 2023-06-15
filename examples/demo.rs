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
    textures.insert('5', Texture2D::from_file_with_format(include_bytes!("res/wall.png"), Some(ImageFormat::Png)));
    textures.insert('e', Texture2D::from_file_with_format(include_bytes!("res/shrek.png"), Some(ImageFormat::Png)));

    let mut map: Map = Map::new("examples/res/map", textures);
    let mut entities: Vec<Entity> = map.filter_entities(&['e']);

    let mut items: Vec<Item> = vec![
        Item::new("gun", "examples/res/gun.png").await,
        Item::new("knife", "examples/res/knife.png").await
    ];

    let mut cam: Ray = Ray::new(Vec2::new(110., 160.), 0.3);

    let mut prev_mx: f32 = mouse_position().0;
    let mut grabbed: bool = true;
    set_cursor_grab(true);
    show_mouse(false);

    loop {
        // Fps camera
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, cam.angle).along(2.));
        }

        if is_key_down(KeyCode::S) {
            cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, cam.angle).along(-2.));
        }

        if is_key_down(KeyCode::A) {
            cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, raycast::util::restrict_angle(cam.angle - PI / 2.)).along(1.));
        }

        if is_key_down(KeyCode::D) {
            cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, raycast::util::restrict_angle(cam.angle - PI / 2.)).along(-1.));
        }

        let mx: f32 = mouse_position().0;
        cam.angle += (mx - prev_mx) / 200.;
        cam.angle = raycast::util::restrict_angle(cam.angle);
        prev_mx = mx;

        // Entity move towards player
        entities[0].pos = raycast::util::move_towards_collidable(&map, entities[0].pos, cam.orig, 1.);

        // Shooting mechanic
        if is_mouse_button_pressed(MouseButton::Left) {
            let ins: Intersection = raycast::cast_ray(&map, &entities, cam);
            match ins.itype {
                IntersectionType::Entity {..} => println!("Hit entity"),
                _ => println!("Hit wall")
            }
        }

        // Equip item
        if is_key_pressed(KeyCode::Key1) {
            raycast::equip_item(&mut items, "gun");
        }

        if is_key_pressed(KeyCode::Key2) {
            raycast::equip_item(&mut items, "knife");
        }

        clear_background(BLACK);
        raycast::render(&map, cam, &entities);
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

