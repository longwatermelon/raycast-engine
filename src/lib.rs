pub mod util;
pub mod map;
pub mod entity;

use util::{Ray, Intersection, IntersectionType};
use entity::Entity;
use map::Map;
use macroquad::prelude::*;
use std::f32::consts::PI;

pub fn render(map: &Map, ray: Ray, scrw: i32, scrh: i32, entities: Vec<Entity>) {
    let angle_range: f32 = PI / 3.;
    let start_angle: f32 = ray.angle - angle_range / 2.;

    for i in 0..scrw {
        let angle: f32 = start_angle + (i as f32 / scrw as f32 * angle_range);
        render_wall(map, Ray::new(ray.orig, angle), ray.angle, i, scrh);
        render_entities(map, Ray::new(ray.orig, angle), ray.angle, i, scrh, entities.clone());
    }
}

pub fn render_2d(map: &Map, ray: Ray, scrw: i32, scrh: i32) {
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

    let angle: f32 = ray.angle;
    let ins: Intersection = map.cast_ray(Ray::new(ray.orig, angle));
    let endx: f32 = ox + (ins.distance * f32::cos(angle) * (scrw as f32 / (map.w * map.tsize) as f32));
    let endy: f32 = oy + (ins.distance * f32::sin(angle) * (scrh as f32 / (map.h * map.tsize) as f32));
    draw_line(ox, oy, endx, endy, 3., BLUE);
}

fn render_wall(map: &Map, ray: Ray, cam_angle: f32, col: i32, scrh: i32) {
    let mut ins: Intersection = map.cast_ray(ray);
    let endp: Vec2 = ray.along(ins.distance);
    ins.distance *= f32::cos(util::restrict_angle(cam_angle - ray.angle));

    let h: f32 = (map.tsize as f32 * scrh as f32) / ins.distance;
    let offset: f32 = (scrh as f32 - h) / 2.;

    let texture: &Texture2D = map.textures.get(&map.at(ins.wall_gpos().x, ins.wall_gpos().y)).unwrap();
    let texture_index: f32 = if matches!(ins.itype, IntersectionType::WallHorizontal {..}) {
        endp.x
    } else {
        endp.y
    };

    draw_texture_ex(
        *texture,
        col as f32, offset, WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(1., h)),
            source: Some(
                Rect::new(
                    (texture_index % map.tsize as f32) / map.tsize as f32 * texture.width(),
                    0.,
                    1.,
                    texture.height()
                )
            ),
            ..Default::default()
        }
    );
}

fn render_entities(map: &Map, ray: Ray, cam_angle: f32, col: i32, scrh: i32, entities: Vec<Entity>) {
    // let mut src: Rect = Rect::default();
    // src.x = 0.;
    // src.y = 0.;
    // src.w = 1.;
    // src.h = 100.;

    // let mut dst: Rect = Rect::default();
    // dst.x = col as f32;
    // dst.w = 1.;

    let mut vins: Vec<(Entity, Intersection)> = entities
        .iter()
        .cloned()
        .map(|e| (e.clone(), entity::intersect(ray, e.pos)))
        .filter(|x| x.1.is_some())
        .map(|t| (t.0, t.1.unwrap()))
        .collect();

    // Sort in descending, render farther entities first
    vins.sort_by(|a, b| b.1.distance.partial_cmp(&a.1.distance).unwrap());

    for (ent, ins) in &vins {
        let h: f32 = (25. * scrh as f32) / ins.distance;
        let offset: f32 = scrh as f32 / 2.;

        let src: Rect = Rect::new(
            ins.entity_col() * map.textures.get(&ent.texture).unwrap().width(),
            0.,
            1.,
            map.textures.get(&ent.texture).unwrap().height()
        );
        let dst: Rect = Rect::new(col as f32, offset, 1., h);

        // draw_line(dst.x, dst.y, dst.x, dst.y + dst.h, 1., RED);
        draw_texture_ex(
            *map.textures.get(&ent.texture).unwrap(),
            dst.x, dst.y, WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(dst.w, dst.h)),
                source: Some(src),
                ..Default::default()
            }
        );
    }

    // if let Some(distance) = entity::intersect(ray, Vec2::new(200., 200.)) {
    //     let h: f32 = (25. * scrh as f32) / distance;
    //     let offset: f32 = scrh as f32 / 2.;
    //     dst.y = offset;
    //     dst.h = h;

    //     draw_line(dst.x, dst.y, dst.x, dst.y + dst.h, 1., RED);
        // draw_texture_ex(
        //     Texture2D::from_file_with_format(include_bytes!("../examples/res/shrek.png"), Some(ImageFormat::Png)),
        //     dst.x, dst.y, WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(dst.w, dst.h)),
        //         source: Some(src),
        //         ..Default::default()
        //     }
        // );
    // }
}

