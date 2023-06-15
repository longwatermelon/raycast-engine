use macroquad::prelude::*;
use std::time::Instant;

pub enum Animation {
    None,
    EaseIn { target: Vec2 },
    Jab { diff: Vec2, t: f32 },
    TextureSwap { orig: Texture2D, t: f32 }
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
        self.end_animation();
        self.animation = Animation::EaseIn { target: Vec2::new(screen_width() - self.texture.width(), screen_height()) };
        self.animation_start = Instant::now();
    }

    pub fn equip(&mut self) {
        self.end_animation();
        self.animation = Animation::EaseIn { target: Vec2::new(screen_width() - self.texture.width(), screen_height() - self.texture.height()) };
        self.animation_start = Instant::now();
    }

    pub fn jab(&mut self, diff: Vec2, t: f32) {
        self.end_animation();
        self.animation = Animation::Jab { diff, t };
        self.animation_start = Instant::now();
    }

    fn end_jab(&mut self) {
        self.animation = Animation::None;
        self.equip();
    }

    pub fn texswap(&mut self, texture: &Texture2D, t: f32) {
        self.end_animation();
        self.animation = Animation::TextureSwap { orig: self.texture, t };
        self.texture = texture.clone();
        self.animation_start = Instant::now();
    }

    fn end_texswap(&mut self) {
        if let Animation::TextureSwap { orig, .. } = self.animation {
            self.texture = orig;
        }
    }

    fn end_animation(&mut self) {
        match self.animation {
            Animation::None => (),
            Animation::Jab {..} => self.end_jab(),
            Animation::EaseIn { target } => self.pos = target,
            Animation::TextureSwap {..} => self.end_texswap()
        }
    }

    pub fn update(&mut self) {
        match self.animation {
            Animation::None => (),
            Animation::EaseIn { target } => self.pos += (target - self.pos) / 5.,
            Animation::Jab { diff, t } => {
                let elapsed: f32 = self.animation_start.elapsed().as_millis() as f32 / 1000.;
                if elapsed < t {
                    self.pos += diff;
                } else {
                    self.end_jab();
                }
            },
            Animation::TextureSwap { t, .. } => {
                let elapsed: f32 = self.animation_start.elapsed().as_millis() as f32 / 1000.;
                if elapsed > t {
                    self.end_texswap();
                }
            }
        }
    }

    pub fn render(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}

