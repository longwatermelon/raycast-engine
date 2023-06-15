use raycast::map::Map;
use raycast::util::Ray;
use macroquad::prelude::*;
use glm::Vec2;

#[macroquad::main(window_conf)]
async fn main() {
    let map: Map = Map::new("res/map");
    let cam: Ray = Ray::new(Vec2::new(160., 160.), 1.5);

    loop {
        clear_background(BLACK);

        raycast::render(&map, cam, 800, 600);

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Raycast demo"),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

