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
        render_wall(map, Ray::new(ray.orig, angle), ray.angle, i, scrh);
    }
}

pub fn render_2d(map: &Map, ray: Ray, scrw: i32, scrh: i32) {
    let angle_range: f32 = PI / 3.;
    let start_angle: f32 = ray.angle - angle_range / 2.;

    let w: f32 = scrw as f32 / map.w as f32;
    let h: f32 = scrh as f32 / map.h as f32;
    for y in 0..map.h {
        for x in 0..map.w {
            if map.at(x, y) != '.' {
                draw_rectangle(
                    x as f32 * w,
                    y as f32 * h,
                    w, h,
                    GRAY
                );
                draw_rectangle_lines(
                    x as f32 * w,
                    y as f32 * h,
                    w, h, 1.,
                    BLACK
                );
            }
        }
    }

    let ox: f32 = ray.orig.x * (scrw as f32 / (map.w * map.tsize) as f32);
    let oy: f32 = ray.orig.y * (scrh as f32 / (map.h * map.tsize) as f32);
    draw_rectangle(
        ox - 5., oy - 5.,
        10., 10.,
        GREEN
    );

    // for i in 0..scrw {
        // let angle: f32 = start_angle + (i as f32 / scrw as f32 * angle_range);
        let angle: f32 = ray.angle;
        let ins: Intersection = map.cast_ray(Ray::new(ray.orig, angle));
        let endx: f32 = ox + (ins.distance * f32::cos(angle) * (scrw as f32 / (map.w * map.tsize) as f32));
        let endy: f32 = oy + (ins.distance * f32::sin(angle) * (scrh as f32 / (map.h * map.tsize) as f32));
        draw_line(ox, oy, endx, endy, 3., BLUE);
    // }
}

fn render_wall(map: &Map, ray: Ray, cam_angle: f32, col: i32, scrh: i32) {
    let mut ins: Intersection = map.cast_ray(ray);
    ins.distance *= f32::cos(util::restrict_angle(cam_angle - ray.angle));
    let h: f32 = (map.tsize as f32 * scrh as f32) / ins.distance;
    let offset: f32 = (scrh as f32 - h) / 2.;
    draw_line(col as f32, offset, col as f32, offset + h, 1., RED);
}

