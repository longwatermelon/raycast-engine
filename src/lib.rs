pub mod util;
pub mod map;
pub mod entity;
pub mod item;

use util::{Ray, Intersection, IntersectionType, Direction};
use entity::Entity;
use map::Map;
use item::Item;
use macroquad::prelude as mq;
use glam::{Vec2, Vec3};
use std::f32::consts::PI;

pub fn render(map: &Map, entities: &[Entity], ray: Ray, fog: Option<f32>, out_img: &mut mq::Image) {
    let vins: Vec<(Intersection, f32)> = cast_rays(map, ray);
    for (x, (ins, angle)) in vins.iter().enumerate() {
        let mut cast_ray: Ray = Ray::new(ray.orig, *angle);
        cast_ray.vangle = ray.vangle;

        let wall_res = render_wall(map, ins, cast_ray, x as i32, fog, out_img);
        render_floor_and_ceil(map, cast_ray, x as i32, wall_res, fog, out_img);
        render_entities(map, cast_ray, x as i32, entities, ins.distance, fog, out_img);
    }
}

/// Ignores entities
// Returns [(Wall intersection, angle)]
fn cast_rays(map: &Map, ray: Ray) -> Vec<(Intersection, f32)> {
    let angle_range: f32 = PI / 3.;
    let start_angle: f32 = ray.angle - angle_range / 2.;

    let mut res: Vec<(Intersection, f32)> = Vec::new();
    for i in 0..mq::screen_width() as i32 {
        let angle: f32 = start_angle + (i as f32 / mq::screen_width() * angle_range);
        let mut ins: Intersection = map.cast_ray(Ray::new(ray.orig, angle));
        ins.fisheye_distance *= f32::cos(util::restrict_angle(angle - ray.angle));
        res.push((ins, angle));
    }

    res
}

/// Returns (wall bottom, wall top)
fn render_wall(map: &Map, ins: &Intersection, ray: Ray, x: i32, fog: Option<f32>, out_img: &mut mq::Image) -> (i32, i32) {
    let floor_level: f32 = (mq::screen_height() / 2.) * (1. + f32::tan(-ray.vangle) / f32::tan(1. / 2.));
    let h: i32 = ((map.tsize * mq::screen_height()) / ins.fisheye_distance) as i32;
    let offset: i32 = floor_level as i32 - (h / 2);

    let texture: &mq::Image = map.textures.get(&map.at(ins.wall_gpos().x, ins.wall_gpos().y)).unwrap();
    let IntersectionType::Wall { face, .. } = ins.itype else { unreachable!() };
    // Horizontal walls only have endp.x change, vertical walls only have endp.y change
    // Horizontal walls collide by north and south
    let endp: Vec2 = ray.along(ins.distance);
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

    let srcx: u32 = ((texture_index % map.tsize) / map.tsize * texture.width() as f32) as u32;
    for y in offset.max(0)..(offset + h).min(out_img.height() as i32) {
        let srcy: u32 = (((y - offset) as f32 / h as f32) * texture.height() as f32) as u32;
        let color: mq::Color = texture.get_pixel(srcx, srcy);
        out_img.set_pixel(
            x as u32, y as u32,
            mq::Color::new(
                color.r * shading,
                color.g * shading,
                color.b * shading,
                color.a
            )
        );
    }

    (offset + h, offset)
}

fn render_floor_and_ceil(map: &Map, ray: Ray, x: i32, rend_wall_result: (i32, i32), fog: Option<f32>, out_img: &mut mq::Image) {
    // From wall bottom to screen bottom
    // let fov: f32 = PI / 2.;
    let fov: f32 = 1.;
    for y in rend_wall_result.0.max(0).min(mq::screen_height() as i32)..(mq::screen_height() as i32) {
        // Find ray angles corresponding to screen pixel
        let ha: f32 = (x as f32 / mq::screen_width()) * fov - (fov / 2.);
        let va: f32 = (y as f32 / mq::screen_height()) * fov - (fov / 2.);
        // x = ray.angle, y = angle looking down, z is useless
        let dir: Vec3 = Vec3::new(ha, f32::sin(va + ray.vangle), 1.).normalize();

        // How many `dir.y` it takes to get to the floor
        let tvert: f32 = (map.tsize / 2.) / dir.y;
        let new_pos: Vec2 = ray.along(tvert);
        let distance: f32 = ray.orig.distance(new_pos);
        let fog: f32 = calculate_fog(fog.unwrap(), distance);

        let texture = map.textures.get(&'0').unwrap();
        let tc: Vec2 = new_pos % Vec2::new(texture.width() as f32, texture.height() as f32);

        let color: mq::Color = texture.get_pixel(tc.x as u32, tc.y as u32);
        out_img.set_pixel(x as u32, y as u32, mq::Color::new(
            fog * color.r,
            fog * color.g,
            fog * color.b,
            color.a,
        ));
    }
}

fn render_entities(map: &Map, ray: Ray, col: i32, entities: &[Entity], wall_dist: f32, fog: Option<f32>, out_img: &mut mq::Image) {
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

    let floor_level: f32 = (mq::screen_height() / 2.) * (1. + f32::tan(-ray.vangle) / f32::tan(1. / 2.));
    for (ent, ins) in &vins {
        let h: f32 = (ent.h * mq::screen_height()) / ins.distance;
        let middle_h: f32 = (map.tsize / 2. * mq::screen_height()) / ins.distance;
        let offset: f32 = floor_level + middle_h - h;

        let h: i32 = h as i32;
        let offset: i32 = offset as i32;

        let srcx: u32 = (ins.entity_col() * map.textures.get(&ent.texture).unwrap().width() as f32) as u32;
        let texture: &mq::Image = map.textures.get(&ent.texture).unwrap();

        let shading: f32 = if let Some(fog) = fog {
            calculate_fog(fog, ins.distance)
        } else {
            1.
        };

        for y in offset.max(0)..(offset + h).min(out_img.height() as i32) {
            let srcy: u32 = (((y - offset) as f32 / h as f32) * texture.height() as f32) as u32;
            let color: mq::Color = texture.get_pixel(srcx, srcy);

            if color.a > 0. {
                out_img.set_pixel(
                    col as u32, y as u32,
                    mq::Color::new(
                        color.r * shading,
                        color.g * shading,
                        color.b * shading,
                        color.a
                    )
                );
            }
        }
    }
}

pub fn render_2d(map: &Map, ray: Ray) {
    let w: f32 = mq::screen_width() / map.w;
    let h: f32 = mq::screen_height() / map.h;
    for y in 0..map.h as i32 {
        for x in 0..map.w as i32 {
            if map.at(x, y) != '.' {
                mq::draw_rectangle(
                    x as f32 * w,
                    y as f32 * h,
                    w, h,
                    mq::GRAY
                );
                mq::draw_rectangle_lines(
                    x as f32 * w,
                    y as f32 * h,
                    w, h, 1.,
                    mq::BLACK
                );
            }
        }
    }

    let ox: f32 = ray.orig.x * (mq::screen_width() / (map.w * map.tsize));
    let oy: f32 = ray.orig.y * (mq::screen_height() / (map.h * map.tsize));
    mq::draw_rectangle(
        ox - 5., oy - 5.,
        10., 10.,
        mq::GREEN
    );

    let angle: f32 = ray.angle;
    let ins: Intersection = map.cast_ray(Ray::new(ray.orig, angle));
    let endx: f32 = ox + (ins.distance * f32::cos(angle) * (mq::screen_width() / (map.w * map.tsize)));
    let endy: f32 = oy + (ins.distance * f32::sin(angle) * (mq::screen_height() / (map.h * map.tsize)));
    mq::draw_line(ox, oy, endx, endy, 3., mq::BLUE);
}

fn calculate_fog(fog: f32, distance: f32) -> f32 {
    1. - f32::min(distance / fog, 1.)
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
        if item.name == item_name {
            item.equip();
        }
    }
}

pub fn cast_ray(map: &Map, entities: &[Entity], ray: Ray) -> Intersection {
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
