use macroquad::prelude as mq;
use glam::Vec2;

pub enum Animation {
    None,
    EaseIn { target: Vec2 },
    Jab { diff: Vec2, t: f32 },
    TextureSwap { orig: mq::Texture2D, t: f32 }
}

pub struct Item {
    pub name: String,
    texture: mq::Texture2D,
    pos: Vec2,
    animation: Animation,
    animation_start: f64,
}

impl Item {
    pub fn new(name: &str, bytes: &[u8]) -> Self {
        let texture: mq::Texture2D = mq::Texture2D::from_file_with_format(bytes, Some(mq::ImageFormat::Png));
        let pos: Vec2 = Vec2::new(mq::screen_width() - texture.width(), mq::screen_height());
        Self {
            name: String::from(name),
            texture,
            pos,
            animation: Animation::None,
            animation_start: -100.,
        }
    }

    pub fn unequip(&mut self) {
        self.end_animation();
        self.animation = Animation::EaseIn { target: Vec2::new(mq::screen_width() - self.texture.width(), mq::screen_height()) };
        self.animation_start = mq::get_time();
    }

    pub fn equip(&mut self) {
        self.end_animation();
        self.animation = Animation::EaseIn { target: Vec2::new(mq::screen_width() - self.texture.width(), mq::screen_height() - self.texture.height()) };
        self.animation_start = mq::get_time();
    }

    pub fn jab(&mut self, diff: Vec2, t: f32) {
        self.end_animation();
        self.animation = Animation::Jab { diff, t };
        self.animation_start = mq::get_time();
    }

    fn end_jab(&mut self) {
        self.animation = Animation::None;
        self.equip();
    }

    pub fn texswap(&mut self, texture: &mq::Texture2D, t: f32) {
        self.end_animation();
        self.animation = Animation::TextureSwap { orig: self.texture.clone(), t };
        self.texture = texture.clone();
        self.animation_start = mq::get_time();
    }

    fn end_texswap(&mut self) {
        if let Animation::TextureSwap { orig, .. } = &self.animation {
            self.texture = orig.clone();
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
                let elapsed: f64 = mq::get_time() - self.animation_start;
                if elapsed < t as f64 {
                    self.pos += diff;
                } else {
                    self.end_jab();
                }
            },
            Animation::TextureSwap { t, .. } => {
                let elapsed: f64 = mq::get_time() - self.animation_start;
                if elapsed > t as f64 {
                    self.end_texswap();
                }
            }
        }
    }

    pub fn render(&self) {
        mq::draw_texture(&self.texture, self.pos.x, self.pos.y, mq::WHITE);
    }
}
