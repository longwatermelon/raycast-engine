use crate::util;
use crate::map::Map;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq)]
pub enum IntersectionType {
    Wall { gpos: IVec2, face: Direction },
    Entity { index: usize, col: f32 }
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
            IntersectionType::Wall { gpos, .. } => gpos,
            _ => panic!()
        }
    }

    pub fn entity_col(&self) -> f32 {
        match self.itype {
            IntersectionType::Entity { col, .. } => col,
            _ => panic!()
        }
    }

    pub fn entity_index(&self) -> usize {
        match self.itype {
            IntersectionType::Entity { index, .. } => index,
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

pub fn fps_camera_controls(map: &Map, cam: &mut Ray, speed: f32) {
    if is_key_down(KeyCode::W) {
        cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, cam.angle).along(speed));
    }

    if is_key_down(KeyCode::S) {
        cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, cam.angle).along(-speed));
    }

    if is_key_down(KeyCode::A) {
        cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, restrict_angle(cam.angle - PI / 2.)).along(speed / 2.));
    }

    if is_key_down(KeyCode::D) {
        cam.orig = map.move_collidable(cam.orig, Ray::new(cam.orig, restrict_angle(cam.angle - PI / 2.)).along(-speed / 2.));
    }
}

pub fn fps_camera_rotation(cam: &mut Ray, prev_mouse_x: &mut f32, sensitivity: f32) {
    let mx: f32 = mouse_position().0;
    cam.angle += sensitivity * (mx - *prev_mouse_x) / 200.;
    cam.angle = restrict_angle(cam.angle);
    *prev_mouse_x = mx;
}
