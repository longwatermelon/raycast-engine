use raycast::map::Map;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let _map: Map = Map::new("res/map");
    loop {
        clear_background(BLACK);

        raycast::render();

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

