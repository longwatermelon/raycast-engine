use crate::util::{self, Ray};
use macroquad::math::Vec2;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Entity {
    pub pos: Vec2,
    pub texture: char
}

impl Entity {
    pub fn new(pos: Vec2, texture: char) -> Self {
        Self { pos, texture }
    }
}

pub fn intersect(ray: Ray, pos: Vec2) -> Option<f32> {
    let p1: Vec2 = pos + Ray::new(pos, util::restrict_angle(ray.angle - PI / 2.)).along(10.);
    let p2: Vec2 = pos + Ray::new(pos, util::restrict_angle(ray.angle + PI / 2.)).along(10.);

    let v1: Vec2 = ray.orig - p1;
    let v2: Vec2 = p2 - p1;
    let v3: Vec2 = Vec2::new(-ray.dir().y, ray.dir().x);

    let dot: f32 = v2.dot(v3);
    if f32::abs(dot) < 0.00001 {
        return None;
    }

    let t1: f32 = (v2.x * v1.y - v1.x * v2.y) / dot;
    let t2: f32 = v1.dot(v3) / dot;

    if t1 >= 0. && (t2 >= 0. && t2 <= 1.) {
        Some(t1)
    } else {
        None
    }
}

