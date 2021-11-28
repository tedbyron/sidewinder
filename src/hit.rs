#![allow(clippy::module_name_repetitions)]

use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
    Front,
    Back,
}

impl Default for Face {
    fn default() -> Self {
        Self::Back
    }
}

impl HitRecord {
    // #[inline]
    // #[must_use]
    // pub const fn new(p: Point, normal: Vec3, t: f64, face: Face) -> Self {
    //     Self { p, normal, t, face }
    // }

    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        if r.direction.dot(outward_normal) < 0.0 {
            self.face = Face::Front;
            self.normal = outward_normal;
        } else {
            self.face = Face::Back;
            self.normal = -outward_normal;
        };
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[non_exhaustive]
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct HitList<T: Hit> {
    inner: Vec<Rc<T>>,
}

impl<T: Hit> HitList<T> {
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline]
    pub fn push(&mut self, value: Rc<T>) {
        self.inner.push(value);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp = rec;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.inner {
            if object.hit(r, t_min, closest_so_far, &mut tmp) {
                hit_anything = true;
                closest_so_far = tmp.t;
            }
        }

        hit_anything
    }
}
