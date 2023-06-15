use crate::util::Ray;
use macroquad::math::{Vec2, IVec2};
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Intersection {
    pub gpos: IVec2,
    pub distance: f32
}

pub struct Map {
    layout: String,
    pub(crate) w: i32,
    pub(crate) h: i32,
    pub(crate) tsize: i32
}

impl Intersection {
    pub fn new(gpos: IVec2, distance: f32) -> Self {
        Self { gpos, distance }
    }
}

impl Default for Intersection {
    fn default() -> Self {
        Self { gpos: IVec2::new(0, 0), distance: 0. }
    }
}

impl Map {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut layout: String = String::new();
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        for line in reader.lines() {
            let s: String = line.unwrap();
            layout.push_str(s.as_str());

            h += 1;
            w = s.len() as i32;
        }

        Self {
            layout,
            w,
            h,
            tsize: 50
        }
    }

    pub fn cast_ray(&self, ray: Ray) -> Intersection {
        let h: Intersection = self.cast_ray_h(ray);
        let v: Intersection = self.cast_ray_v(ray);

        if h.distance < v.distance { h } else { v }
    }

    fn cast_ray_h(&self, ray: Ray) -> Intersection {
        let mut closest: Vec2 = Vec2::new(0., 0.);
        closest.y = ray.orig.y - ray.orig.y % self.tsize as f32 +
                        if ray.dir().y > 0. { self.tsize } else { 0 } as f32;
        closest.x = ray.orig.x + ((closest.y - ray.orig.y) / f32::tan(ray.angle));

        loop {
            let mut gpos: IVec2 = self.gpos(closest);
            if ray.dir().y < 0. {
                gpos.y -= 1;
            }

            if self.out_of_bounds(gpos) || self.at(gpos.x, gpos.y) != '.' {
                return Intersection::new(gpos, (closest - ray.orig).length());
            }

            let dy: f32 = if ray.dir().y < 0. { -self.tsize } else { self.tsize } as f32;
            closest.y += dy;
            closest.x += dy / f32::tan(ray.angle);
        }
    }

    fn cast_ray_v(&self, ray: Ray) -> Intersection {
        let mut closest: Vec2 = Vec2::new(0., 0.);
        closest.x = ray.orig.x - ray.orig.x % self.tsize as f32 +
                        if ray.dir().x > 0. { self.tsize } else { 0 } as f32;
        closest.y = ray.orig.y + ((closest.x - ray.orig.x) * f32::tan(ray.angle));

        loop {
            let mut gpos: IVec2 = self.gpos(closest);
            if ray.dir().x < 0. {
                gpos.x -= 1;
            }

            if self.out_of_bounds(gpos) || self.at(gpos.x, gpos.y) != '.' {
                return Intersection::new(gpos, (closest - ray.orig).length());
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
        self.layout.chars().nth((gy * self.w + gx) as usize).unwrap_or(' ')
    }

    pub fn out_of_bounds(&self, gpos: IVec2) -> bool {
        gpos.x < 0 || gpos.x >= self.w || gpos.y < 0 || gpos.y >= self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpos() {
        let map: Map = Map::new("res/map");
        assert_eq!(map.gpos(Vec2::new(160., 150.)), IVec2::new(3, 3));
        assert_eq!(map.gpos(Vec2::new(200., 140.)), IVec2::new(4, 2));
    }
}

