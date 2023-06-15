use macroquad::math::Vec2;
use std::f32::consts::PI;

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
        Vec2::new(f32::cos(self.angle), f32::sin(self.angle))
    }
}

pub fn restrict_angle(mut angle: f32) -> f32 {
    if angle > 2. * PI {
        angle -= 2. * PI;
    }

    if angle < 0. {
        angle += 2. * PI;
    }

    angle
}

