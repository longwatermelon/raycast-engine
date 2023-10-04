use crate::util::{self, Ray, Intersection, IntersectionType};
use macroquad::math::Vec2;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Entity {
    pub pos: Vec2,
    pub texture: char,
    pub w: f32,
    pub h: f32
}

impl Entity {
    pub fn new(pos: Vec2, texture: char, size: (f32, f32)) -> Self {
        Self { pos, texture, w: size.0, h: size.1 }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let p1: Vec2 = Ray::new(self.pos, util::restrict_angle(ray.angle - PI / 2.)).along(self.w / 2.);
        let p2: Vec2 = Ray::new(self.pos, util::restrict_angle(ray.angle + PI / 2.)).along(self.w / 2.);

        let v1: Vec2 = ray.orig - p1;
        let v2: Vec2 = p2 - p1;
        let v3: Vec2 = Vec2::new(-ray.dir().y, ray.dir().x);

        let dot: f32 = v2.dot(v3);
        if f32::abs(dot) < 0.00001 {
            return None;
        }

        let t1: f32 = (v2.x * v1.y - v1.x * v2.y) / dot;
        let t2: f32 = v1.dot(v3) / dot;

        if t1 >= 0. && (0f32..=1f32).contains(&t2) {
            let hit: Vec2 = ray.along(t1);
            let dist1: f32 = (hit - p1).length();
            let dist2: f32 = (hit - p2).length();
            let col: f32 = dist1 / (dist1 + dist2);

            Some(Intersection::new(IntersectionType::Entity { index: 0, col }, t1))
        } else {
            None
        }
    }
}
