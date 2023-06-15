use macroquad::prelude::*;

pub struct Item {
    pub name: String,
    texture: Texture2D,
    pos: Vec2,
    target: Vec2
}

impl Item {
    pub async fn new(name: &str, path: &str) -> Self {
        let texture: Texture2D = Texture2D::from_image(&load_image(path).await.unwrap());
        let pos: Vec2 = Vec2::new(screen_width() - texture.width(), screen_height());
        Self {
            name: String::from(name),
            texture,
            pos,
            target: pos
        }
    }

    pub fn unequip(&mut self) {
        self.target = Vec2::new(screen_width() - self.texture.width(), screen_height());
    }

    pub fn equip(&mut self) {
        self.target = Vec2::new(screen_width() - self.texture.width(), screen_height() - self.texture.height());
    }

    pub fn update(&mut self) {
        self.pos += (self.target - self.pos) / 5.;
    }

    pub fn render(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}

