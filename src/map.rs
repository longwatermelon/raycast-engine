use crate::util::Ray;
use glm::{IVec2, Vec2};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::f32::consts::PI;

pub struct Intersection {
    pub gpos: IVec2,
    pub distance: f32
}

pub struct Map {
    layout: String,
    w: i32,
    h: i32,
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
        // let v: Intersection = self.cast_ray_v(ray);

        // if h.distance < v.distance { h } else { v }
        h
    }

    fn cast_ray_h(&self, ray: Ray) -> Intersection {
        let mut closest: Vec2 = Vec2::new(0., 0.);
        closest.y = ray.orig.y - ray.orig.y.rem_euclid(self.tsize as f32) +
                        if ray.angle > PI { self.tsize as f32 } else { 0. };
        closest.x = ray.orig.x + ((closest.y - ray.orig.y) / -f32::tan(ray.angle));

        loop {
            let mut gpos: IVec2 = self.gpos(closest);
            if ray.angle < PI {
                gpos.y -= 1;
            }

            if self.out_of_bounds(gpos) || self.at(gpos.x, gpos.y) != '.' {
                return Intersection::new(gpos, glm::length(closest - ray.orig));
            }

            let dy: f32 = if ray.angle < PI { -self.tsize } else { self.tsize } as f32;
            closest.y += dy;
            closest.x += dy / -f32::tan(ray.angle);
        }
    }

    fn cast_ray_v(&self, ray: Ray) -> Intersection {
        todo!()
    }

    pub fn gpos(&self, pos: Vec2) -> IVec2 {
        let x: i32 = pos.x as i32;
        let y: i32 = pos.y as i32;
        IVec2::new(
            (x - x % self.tsize as i32) / self.tsize as i32,
            (y - y % self.tsize as i32) / self.tsize as i32
        )
    }

    pub fn at(&self, gx: i32, gy: i32) -> char {
        self.layout.chars().nth((gy * self.w + gx) as usize).unwrap()
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

