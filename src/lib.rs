pub mod util;
pub mod map;
pub mod entity;
pub mod item;

use util::{Ray, Intersection, IntersectionType, Direction};
use entity::Entity;
use map::Map;
use item::Item;
use macroquad::prelude::*;
use std::f32::consts::PI;

pub fn render(map: &Map, entities: &Vec<Entity>, ray: Ray, fog: Option<f32>) {
    let angle_range: f32 = PI / 3.;
    let start_angle: f32 = ray.angle - angle_range / 2.;

    for i in 0..screen_width() as i32 {
        let angle: f32 = start_angle + (i as f32 / screen_width() * angle_range);
        let wall_dist: f32 = render_wall(map, Ray::new(ray.orig, angle), ray.angle, i, fog);
        render_entities(map, Ray::new(ray.orig, angle), i, entities, wall_dist, fog);
    }
}

pub fn render_2d(map: &Map, ray: Ray) {
    let w: f32 = screen_width() / map.w as f32;
    let h: f32 = screen_height() / map.h as f32;
    for y in 0..map.h as i32 {
        for x in 0..map.w as i32 {
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

    let ox: f32 = ray.orig.x * (screen_width() as f32 / (map.w * map.tsize) as f32);
    let oy: f32 = ray.orig.y * (screen_height() as f32 / (map.h * map.tsize) as f32);
    draw_rectangle(
        ox - 5., oy - 5.,
        10., 10.,
        GREEN
    );

    let angle: f32 = ray.angle;
    let ins: Intersection = map.cast_ray(Ray::new(ray.orig, angle));
    let endx: f32 = ox + (ins.distance * f32::cos(angle) * (screen_width() as f32 / (map.w * map.tsize) as f32));
    let endy: f32 = oy + (ins.distance * f32::sin(angle) * (screen_height() as f32 / (map.h * map.tsize) as f32));
    draw_line(ox, oy, endx, endy, 3., BLUE);
}

pub fn render_item(items: &mut Vec<Item>) {
    for item in items {
        item.update();
        item.render();
    }
}

/// Animated swap of items
pub fn equip_item(items: &mut Vec<Item>, item_name: &str) {
    for item in items {
        item.unequip();
        if item.name == String::from(item_name) {
            item.equip();
        }
    }
}

pub fn cast_ray(map: &Map, entities: &Vec<Entity>, ray: Ray) -> Intersection {
    let map_ins: Intersection = map.cast_ray(ray);

    let mut ent_ins: Intersection = Intersection::new(IntersectionType::Entity { index: 0, col: 0. }, f32::INFINITY);
    for (i, ent) in entities.iter().enumerate() {
        if let Some(ins) = ent.intersect(ray) {
            if ins.distance < ent_ins.distance {
                ent_ins = ins;
                if let IntersectionType::Entity { index, .. } = &mut ent_ins.itype {
                    *index = i;
                }
            }
        }
    }

    if map_ins.distance < ent_ins.distance { map_ins } else { ent_ins }
}

fn calculate_fog(fog: f32, distance: f32) -> f32 {
    1. - f32::min(distance / fog, 1.)
}

fn render_wall(map: &Map, ray: Ray, cam_angle: f32, col: i32, fog: Option<f32>) -> f32 {
    let mut ins: Intersection = map.cast_ray(ray);
    let endp: Vec2 = ray.along(ins.distance);
    ins.distance *= f32::cos(util::restrict_angle(cam_angle - ray.angle));

    let h: f32 = (map.tsize as f32 * screen_height() as f32) / ins.distance;
    let offset: f32 = (screen_height() as f32 - h) / 2.;

    let texture: &Texture2D = map.textures.get(&map.at(ins.wall_gpos().x, ins.wall_gpos().y)).unwrap();
    let IntersectionType::Wall { face, .. } = ins.itype else { unreachable!() };
    // Horizontal walls only have endp.x change, vertical walls only have endp.y change
    // Horizontal walls collide by north and south
    let texture_index: f32 = if matches!(face, Direction::South | Direction::North) {
        endp.x
    } else {
        endp.y
    };

    let shading: f32 = if let Some(fog) = fog {
        calculate_fog(fog, ins.distance)
    } else {
        1.
    };

    draw_texture_ex(
        *texture,
        col as f32, offset, Color::new(shading, shading, shading, 1.),
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

    ins.distance
}

fn render_entities(map: &Map, ray: Ray, col: i32, entities: &Vec<Entity>, wall_dist: f32, fog: Option<f32>) {
    let mut vins: Vec<(Entity, Intersection)> = entities
        .iter()
        .cloned()
        .map(|e| (e.clone(), e.intersect(ray)))
        .filter(|x| x.1.is_some())
        .filter(|x| x.1.as_ref().unwrap().distance < wall_dist)
        .map(|t| (t.0, t.1.unwrap()))
        .collect();

    // Sort in descending, render farther entities first
    vins.sort_by(|a, b| b.1.distance.partial_cmp(&a.1.distance).unwrap());

    for (ent, ins) in &vins {
        let h: f32 = (ent.h * screen_height()) / ins.distance;
        let middle_h: f32 = (map.tsize / 2. * screen_height()) / ins.distance;
        // let offset: f32 = (screen_height() as f32 - h) / 2.;
        let offset: f32 = screen_height() / 2. + middle_h - h;

        let src: Rect = Rect::new(
            ins.entity_col() * map.textures.get(&ent.texture).unwrap().width(),
            0.,
            1.,
            map.textures.get(&ent.texture).unwrap().height()
        );
        let dst: Rect = Rect::new(col as f32, offset, 1., h);

        let shading: f32 = if let Some(fog) = fog {
            calculate_fog(fog, ins.distance)
        } else {
            1.
        };
        draw_texture_ex(
            *map.textures.get(&ent.texture).unwrap(),
            dst.x, dst.y, Color::new(shading, shading, shading, 1.),
            DrawTextureParams {
                dest_size: Some(Vec2::new(dst.w, dst.h)),
                source: Some(src),
                ..Default::default()
            }
        );
    }
}