pub mod util;
pub mod map;
pub mod entity;
pub mod item;
pub mod prelude;

use util::{Ray, Intersection, IntersectionType, Direction};
use entity::Entity;
use map::{Map, Surface};
use item::Item;
use macroquad::prelude as mq;
use glam::{Vec2, Vec3, IVec2};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub enum Fog {
    None,
    /// distance
    Point(f32),
    /// distance, radius
    Directional(f32, f32),
}

pub fn render(map: &Map, entities: &[&Entity], ray: Ray, fog: Fog, out_img: &mut mq::Image) {
    // let scrdim: IVec2 = IVec2::new(mq::screen_width() as i32, mq::screen_height() as i32);
    let vins: Vec<(Intersection, f32)> = cast_rays(map, ray);

    for (x, (ins, angle)) in vins.iter().enumerate() {
        let mut cast_ray: Ray = Ray::new(ray.orig, *angle);
        cast_ray.vangle = ray.vangle;

        let wall_res = render_wall(map, ins, cast_ray, x as i32, fog, out_img);
        render_floor_and_ceil_yrange(map, cast_ray, ins, x as i32, wall_res.0, util::scrh(), -1, fog, &map.floor_tex, out_img);
        render_floor_and_ceil_yrange(map, cast_ray, ins, x as i32, 0, wall_res.1 as i32, 1, fog, &map.ceil_tex, out_img);
        render_entities(map, cast_ray, x as i32, entities, ins.distance, fog, out_img);
    }

    if let Fog::Directional(_, radius) = fog {
        let imgdims = (out_img.width(), out_img.height());
        let out_data: &mut [[u8; 4]] = out_img.get_image_data_mut();
        for y in 0..imgdims.1 {
            for x in 0..imgdims.0 {
                let r: f32 = Vec2::new(x as f32, y as f32).distance(Vec2::new(util::scrw() as f32 / 2., util::scrh() as f32 / 2.));
                let brightness: f32 = (1. - r / radius).max(0.).min(1.);

                let index: usize = y * imgdims.0 + x;
                let color: [u8; 4] = out_data[index];
                out_data[index] = [color[0], color[1], color[2], (brightness * color[3] as f32) as u8];
            }
        }
    }
}

/// Ignores entities
// Returns [(Wall intersection, angle)]
fn cast_rays(map: &Map, ray: Ray) -> Vec<(Intersection, f32)> {
    let angle_range: f32 = PI / 3.;
    let start_angle: f32 = ray.angle - angle_range / 2.;

    let mut res: Vec<(Intersection, f32)> = Vec::new();
    for i in 0..util::scrw() {
        let angle: f32 = start_angle + (i as f32 / util::scrh() as f32 * angle_range);
        let mut ins: Intersection = map.cast_ray(Ray::new(ray.orig, angle));
        ins.fisheye_distance *= f32::cos(util::restrict_angle(angle - ray.angle));
        res.push((ins, angle));
    }

    res
}

/// Returns (wall bottom, wall top)
fn render_wall(map: &Map, ins: &Intersection, ray: Ray, x: i32, fog: Fog, out_img: &mut mq::Image) -> (i32, i32) {
    let gpos: IVec2 = ins.wall_gpos();
    let hmul: f32 = *map.wall_heights.get(&map.at(gpos.x, gpos.y)).unwrap_or(&1.);

    let floor_level: f32 = (util::scrh() as f32 / 2.) * (1. + f32::tan(-ray.vangle) / f32::tan(1. / 2.));
    let h: i32 = ((map.tsize * hmul * util::scrh() as f32) / ins.fisheye_distance) as i32;
    let offset: i32 = floor_level as i32 - (h / 2) - (((hmul - 1.) / (hmul * 2.)) * h as f32) as i32;

    let texture: &mq::Image = map.textures.get(&map.at(ins.wall_gpos().x, ins.wall_gpos().y)).unwrap();
    let IntersectionType::Wall { face, .. } = ins.itype else { unreachable!() };
    // Horizontal walls only have endp.x change, vertical walls only have endp.y change
    // Horizontal walls collide by north and south
    let endp: Vec2 = ray.along(ins.distance);
    let (texture_index, shading) = if matches!(face, Direction::South | Direction::North) {
        (endp.x, 0.8)
    } else {
        (endp.y, 1.)
    };
    let fog: f32 = if !matches!(fog, Fog::None) {
        calculate_fog(fog, ins.distance)
    } else {
        shading
    };

    let srcx: u32 = ((texture_index % map.tsize) / map.tsize * texture.width() as f32) as u32;

    let y0: i32 = offset.max(0).min(out_img.height() as i32);
    let y1: i32 = (offset + h).min(out_img.height() as i32);

    let mut out_i: usize = y0 as usize * out_img.width() + x as usize;
    let out_di: usize = out_img.width();
    let out_data: &mut [[u8; 4]] = out_img.get_image_data_mut();
    let tex_data: &[[u8; 4]] = texture.get_image_data();

    for y in y0..y1 {
        let srcy: u32 = (((y - offset) as f32 / h as f32) * texture.height() as f32) as u32;
        let mut color: [u8; 4] = tex_data[srcy as usize * texture.width() + srcx as usize];
        if color[3] > 0 {
            color[0] = (fog * color[0] as f32) as u8;
            color[1] = (fog * color[1] as f32) as u8;
            color[2] = (fog * color[2] as f32) as u8;
            out_data[out_i] = color;
        }
        out_i += out_di;
    }

    (offset + h, offset)
}

fn render_floor_and_ceil_yrange(map: &Map, ray: Ray, ins: &Intersection, x: i32, y0: i32, y1: i32, pitch_direction: i32, fog: Fog, surface: &Surface, out_img: &mut mq::Image) {
    // From wall bottom to screen bottom
    let y0: i32 = y0.max(0).min(util::scrh());
    let y1: i32 = y1.max(0).min(util::scrh());

    let mut out_i: usize = y0 as usize * out_img.width() + x as usize;
    let out_di: usize = out_img.width();
    let out_data: &mut [[u8; 4]] = out_img.get_image_data_mut();
    let surf_data: &[[u8; 4]] = match surface {
        Surface::Texture(img) => img.get_image_data(),
        Surface::Color(_) => &[], // Doesn't matter what goes here, won't be used anyways
    };

    for y in y0..y1 {
        // Find ray angles corresponding to screen pixel
        let ha: f32 = (x as f32 / util::scrw() as f32) - 0.5;
        let va: f32 = (y as f32 / util::scrh() as f32) - 0.5;
        // x = ray.angle, y = angle looking down, z is useless
        let dir: Vec3 = Vec3::new(ha, f32::sin(va + ray.vangle), 1.).normalize();

        // How many `dir.y` it takes to get to the floor
        let gpos: IVec2 = ins.wall_gpos();
        let wall_h: f32 = *map.wall_heights.get(&map.at(gpos.x, gpos.y)).unwrap_or(&1.);
        let dist_to_wall: f32 = if pitch_direction > 0 {
            map.tsize * wall_h - map.tsize / 2.
        } else {
            map.tsize / 2.
        };
        let tvert: f32 = -pitch_direction as f32 * dist_to_wall / dir.y;
        let new_pos: Vec2 = ray.along(tvert);
        let distance: f32 = ray.orig.distance(new_pos);
        let fog: f32 = calculate_fog(fog, distance);

        // Rendering
        let color: [u8; 4] = match surface {
            Surface::Texture(texture) => {
                let tc: Vec2 = new_pos % Vec2::new(texture.width() as f32, texture.height() as f32);
                let mut color: [u8; 4] = surf_data[tc.y as usize * texture.width() + tc.x as usize];
                color[3] = (fog * 255.) as u8;
                color
            }
            Surface::Color(color) => [
                (color.r * 255.) as u8,
                (color.g * 255.) as u8,
                (color.b * 255.) as u8,
                (fog * 255.) as u8,
            ],
        };

        out_data[out_i] = color;
        out_i += out_di;
    }
}

fn render_entities(map: &Map, ray: Ray, col: i32, entities: &[&Entity], wall_dist: f32, fog: Fog, out_img: &mut mq::Image) {
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

    let floor_level: f32 = (util::scrh() as f32 / 2.) * (1. + f32::tan(-ray.vangle) / f32::tan(1. / 2.));
    for (ent, ins) in &vins {
        let h: f32 = (ent.h * util::scrh() as f32) / ins.distance;
        let middle_h: f32 = (map.tsize / 2. * util::scrh() as f32) / ins.distance;
        let offset: f32 = floor_level + middle_h - h;

        let h: i32 = h as i32;
        let offset: i32 = offset as i32;

        let srcx: u32 = (ins.entity_col() * map.textures.get(&ent.texture).unwrap().width() as f32) as u32;
        let texture: &mq::Image = map.textures.get(&ent.texture).unwrap();

        let fog: f32 = calculate_fog(fog, ins.distance);

        let y0: i32 = offset.max(0);
        let y1: i32 = (offset + h).min(out_img.height() as i32);

        let mut out_i: usize = y0 as usize * out_img.width() + col as usize;
        let out_di: usize = out_img.width();
        let out_data: &mut [[u8; 4]] = out_img.get_image_data_mut();
        let tex_data: &[[u8; 4]] = texture.get_image_data();

        for y in y0..y1 {
            let srcy: u32 = (((y - offset) as f32 / h as f32) * texture.height() as f32) as u32;
            let mut color: [u8; 4] = tex_data[srcy as usize * texture.width() + srcx as usize];

            if color[3] > 0 {
                color[3] = (fog * 255.) as u8;
                out_data[out_i] = color;
            }

            out_i += out_di;
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

fn calculate_fog(fog: Fog, distance: f32) -> f32 {
    match fog {
        Fog::None => 1.,
        Fog::Point(dist) | Fog::Directional(dist, _) => 1. - f32::min(distance / dist, 1.),
    }
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

pub fn cast_ray(map: &Map, entities: &[&Entity], ray: Ray) -> Intersection {
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
