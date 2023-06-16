use crate::util::{Ray, Intersection, IntersectionType};
use crate::entity::Entity;
use macroquad::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

pub struct Map {
    layout: String,
    pub(crate) w: f32,
    pub(crate) h: f32,
    pub(crate) tsize: f32,
    pub(crate) textures: HashMap<char, Texture2D>
}

impl Map {
    pub fn new(path: &str, textures: HashMap<char, Texture2D>) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut layout: String = String::new();
        let mut w: f32 = 0.;
        let mut h: f32 = 0.;
        for line in reader.lines() {
            let s: String = line.unwrap();
            layout.push_str(s.as_str());

            h += 1.;
            w = s.len() as f32;
        }

        Self {
            layout,
            w,
            h,
            tsize: 50.,
            textures
        }
    }

    pub fn filter_entities(&mut self, entity_tags: &[char], entity_sizes: &[(f32, f32)]) -> Vec<Entity> {
        let mut res: Vec<Entity> = Vec::new();

        for y in 0..self.h as i32 {
            for x in 0..self.w as i32 {
                if let Some(index) = entity_tags.iter().position(|&e| e == self.at(x, y)) {
                    res.push(Entity::new(Vec2::new(
                        x as f32 * self.tsize + self.tsize / 2.,
                        y as f32 * self.tsize + self.tsize / 2.),
                        self.at(x, y), entity_sizes[index])
                    );
                }
            }
        }

        for e in entity_tags {
            self.layout = self.layout.replace(e.to_string().as_str(), ".");
        }

        res
    }

    pub fn cast_ray(&self, ray: Ray) -> Intersection {
        let h: Intersection = self.cast_ray_h(ray);
        let v: Intersection = self.cast_ray_v(ray);

        if h.distance < v.distance { h } else { v }
    }

    fn cast_ray_h(&self, ray: Ray) -> Intersection {
        let mut closest: Vec2 = Vec2::new(0., 0.);
        closest.y = ray.orig.y - ray.orig.y % self.tsize as f32 +
                        if ray.dir().y > 0. { self.tsize } else { 0. };
        closest.x = ray.orig.x + ((closest.y - ray.orig.y) / f32::tan(ray.angle));

        loop {
            let mut gpos: IVec2 = self.gpos(closest);
            if ray.dir().y < 0. {
                gpos.y -= 1;
            }

            if self.out_of_bounds(gpos) || self.at(gpos.x, gpos.y) != '.' {
                return Intersection::new(IntersectionType::WallHorizontal { gpos }, (closest - ray.orig).length());
            }

            let dy: f32 = if ray.dir().y < 0. { -self.tsize } else { self.tsize } as f32;
            closest.y += dy;
            closest.x += dy / f32::tan(ray.angle);
        }
    }

    fn cast_ray_v(&self, ray: Ray) -> Intersection {
        let mut closest: Vec2 = Vec2::new(0., 0.);
        closest.x = ray.orig.x - ray.orig.x % self.tsize as f32 +
                        if ray.dir().x > 0. { self.tsize } else { 0. };
        closest.y = ray.orig.y + ((closest.x - ray.orig.x) * f32::tan(ray.angle));

        loop {
            let mut gpos: IVec2 = self.gpos(closest);
            if ray.dir().x < 0. {
                gpos.x -= 1;
            }

            if self.out_of_bounds(gpos) || self.at(gpos.x, gpos.y) != '.' {
                return Intersection::new(IntersectionType::WallVertical { gpos }, (closest - ray.orig).length());
            }

            let dx: f32 = if ray.dir().x < 0. { -self.tsize } else { self.tsize } as f32;
            closest.x += dx;
            closest.y += dx * f32::tan(ray.angle);
        }
    }

    pub fn move_collidable(&self, before: Vec2, after: Vec2) -> Vec2 {
        let offset: Vec2 = Vec2::new(
            if after.x - before.x > 0. { 5. } else { -5. },
            if after.y - before.y > 0. { 5. } else { -5. }
        );

        let gpos: IVec2 = self.gpos(before + offset);
        let new_gpos: IVec2 = self.gpos(after + offset);

        Vec2::new(
            if self.at(new_gpos.x, gpos.y) == '.' { after.x } else { before.x },
            if self.at(gpos.x, new_gpos.y) == '.' { after.y } else { before.y }
        )
    }

    pub fn gpos(&self, pos: Vec2) -> IVec2 {
        IVec2::new(
            ((pos.x - pos.x % self.tsize as f32) / self.tsize as f32) as i32,
            ((pos.y - pos.y % self.tsize as f32) / self.tsize as f32) as i32
        )
    }

    pub fn at(&self, gx: i32, gy: i32) -> char {
        self.layout.chars().nth((gy * self.w as i32 + gx) as usize).unwrap_or(' ')
    }

    pub fn out_of_bounds(&self, gpos: IVec2) -> bool {
        gpos.x < 0 || gpos.x >= self.w as i32 || gpos.y < 0 || gpos.y >= self.h as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpos() {
        let map: Map = Map::new("res/map", HashMap::new());
        assert_eq!(map.gpos(Vec2::new(160., 150.)), IVec2::new(3, 3));
        assert_eq!(map.gpos(Vec2::new(200., 140.)), IVec2::new(4, 2));
    }
}

