use raycast::map::Map;
use raycast::util::Ray;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let map: Map = Map::new("res/map");
    let mut cam: Ray = Ray::new(Vec2::new(110., 160.), 0.3);

    loop {
        if is_key_down(KeyCode::W) {
            cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, cam.angle).along(2.));
        }

        if is_key_down(KeyCode::Right) {
            cam.angle += 0.1;
        }

        if is_key_down(KeyCode::Left) {
            cam.angle -= 0.1;
        }

        clear_background(BLACK);

        cam.angle = raycast::util::restrict_angle(cam.angle);
        raycast::render(&map, cam, 800, 800);

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

