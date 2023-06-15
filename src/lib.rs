pub mod util;
pub mod map;

use util::Ray;
use map::{Map, Intersection};
use macroquad::prelude::*;
use std::f32::consts::PI;

pub fn render(map: &Map, ray: Ray, scrw: i32, scrh: i32) {
    let angle_range: f32 = PI / 3.;
    let start_angle: f32 = ray.angle - angle_range / 2.;

    for i in 0..scrw {
        let angle: f32 = start_angle + (i as f32 / scrw as f32 * angle_range);
        render_wall(map, Ray::new(ray.orig, angle), i, scrh);
    }
}

fn render_wall(map: &Map, ray: Ray, col: i32, scrh: i32) {
    let ins: Intersection = map.cast_ray(ray);
    let h: f32 = (map.tsize as f32 * scrh as f32) / ins.distance;
    let offset: f32 = (scrh as f32 - h) / 2.;
    draw_line(col as f32, offset, col as f32, offset + h, 1., RED);
}

