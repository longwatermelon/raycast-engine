use raycast::map::Map;
use raycast::util::Ray;
use macroquad::prelude::*;
use glm::Vec2;

#[macroquad::main(window_conf)]
async fn main() {
    let map: Map = Map::new("res/map");
    let mut cam: Ray = Ray::new(Vec2::new(110., 160.), 0.3);

    loop {
        clear_background(BLACK);

        raycast::render(&map, cam, 800, 800);
        raycast::render_2d(&map, cam, 800, 800);
        cam.angle += 0.01;
        cam.angle = raycast::util::restrict_angle(cam.angle);

        next_frame().await;
        // break;
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

