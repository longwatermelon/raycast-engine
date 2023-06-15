use crate::util;
use crate::map::Map;
use macroquad::math::{Vec2, IVec2};
use std::f32::consts::PI;

#[derive(PartialEq)]
pub enum IntersectionType {
    WallHorizontal { gpos: IVec2 },
    WallVertical { gpos: IVec2 },
    Entity { col: f32 }
}

pub struct Intersection {
    pub itype: IntersectionType,
    pub distance: f32
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Vec2,
    pub angle: f32
}

impl Intersection {
    pub fn new(itype: IntersectionType, distance: f32) -> Self {
        Self { itype, distance }
    }

    pub fn wall_gpos(&self) -> IVec2 {
        match self.itype {
            IntersectionType::WallVertical { gpos } |
            IntersectionType::WallHorizontal { gpos } => gpos,
            _ => panic!()
        }
    }

    pub fn entity_col(&self) -> f32 {
        match self.itype {
            IntersectionType::Entity { col } => col,
            _ => panic!()
        }
    }
}

impl Ray {
    pub fn new(orig: Vec2, angle: f32) -> Self {
        Self { orig, angle }
    }

    pub fn along(&self, t: f32) -> Vec2 {
        self.orig + self.dir() * t
    }

    pub fn dir(&self) -> Vec2 {
        Vec2::new(f32::cos(self.angle), f32::sin(self.angle))
    }
}

pub fn restrict_angle(mut angle: f32) -> f32 {
    if angle > 2. * PI {
        angle -= 2. * PI;
    }

    if angle < 0. {
        angle += 2. * PI;
    }

    angle
}

pub fn move_towards_collidable(map: &Map, pos: Vec2, target: Vec2, speed: f32) -> Vec2 {
    let angle: f32 = f32::atan2(target.y - pos.y, target.x - pos.x);
    map.move_collidable(pos, Ray::new(pos, util::restrict_angle(angle)).along(speed))
}

pub fn move_towards(pos: Vec2, target: Vec2, speed: f32) -> Vec2 {
    let angle: f32 = f32::atan2(target.y - pos.y, target.x - pos.x);
    Ray::new(pos, util::restrict_angle(angle)).along(speed)
}

