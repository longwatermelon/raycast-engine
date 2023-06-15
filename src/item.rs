use macroquad::prelude::*;
use std::time::Instant;

pub enum Animation {
    None,
    Jab { diff: Vec2, t: f32 },
    EaseIn { target: Vec2 }
}

pub struct Item {
    pub name: String,
    texture: Texture2D,
    pos: Vec2,
    animation: Animation,
    animation_start: Instant
}

impl Item {
    pub async fn new(name: &str, path: &str) -> Self {
        let texture: Texture2D = Texture2D::from_image(&load_image(path).await.unwrap());
        let pos: Vec2 = Vec2::new(screen_width() - texture.width(), screen_height());
        Self {
            name: String::from(name),
            texture,
            pos,
            animation: Animation::None,
            animation_start: Instant::now()
        }
    }

    pub fn unequip(&mut self) {
        self.animation = Animation::EaseIn { target: Vec2::new(screen_width() - self.texture.width(), screen_height()) };
        self.animation_start = Instant::now();
    }

    pub fn equip(&mut self) {
        self.animation = Animation::EaseIn { target: Vec2::new(screen_width() - self.texture.width(), screen_height() - self.texture.height()) };
        self.animation_start = Instant::now();
    }

    pub fn jab(&mut self, diff: Vec2, t: f32) {
        self.animation = Animation::Jab { diff, t };
        self.animation_start = Instant::now();
    }

    pub fn update(&mut self) {
        match self.animation {
            Animation::None => (),
            Animation::Jab { diff, t } => {
                let elapsed: f32 = self.animation_start.elapsed().as_millis() as f32 / 1000.;
                if elapsed < t {
                    self.pos += diff;
                } else {
                    self.equip();
                }
            },
            Animation::EaseIn { target } => self.pos += (target - self.pos) / 5.
        }
    }

    pub fn render(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}

