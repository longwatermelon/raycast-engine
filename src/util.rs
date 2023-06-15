use glm::Vec2;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Vec2,
    pub angle: f32
}

impl Ray {
    pub fn new(orig: Vec2, angle: f32) -> Self {
        Self { orig, angle }
    }

    pub fn along(&self, t: f32) -> Vec2 {
        self.orig + self.dir() * t
    }

    pub fn dir(&self) -> Vec2 {
        Vec2::new(f32::cos(self.angle), -f32::sin(self.angle))
    }
}

