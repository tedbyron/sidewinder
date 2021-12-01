#![allow(clippy::module_name_repetitions)]

use crate::graphics::Ray;
use crate::math::{Point, Vec3};

use super::material::Material;

#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
    pub mat: &'a dyn Material,
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum Face {
    Front,
    Back,
}

impl<'a> HitRecord<'a> {
    #[inline]
    #[must_use]
    pub fn new(p: Point, normal: Vec3, t: f64, face: Face, mat: &'a dyn Material) -> Self {
        Self {
            p,
            normal,
            t,
            face,
            mat,
        }
    }

    #[inline]
    #[must_use]
    pub fn face_normal(r: &Ray, outward_normal: Vec3) -> (Face, Vec3) {
        if r.direction.dot(outward_normal) < 0.0 {
            (Face::Front, outward_normal)
        } else {
            (Face::Back, -outward_normal)
        }
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>>;
}

#[non_exhaustive]
#[derive(Default)]
pub struct HitList {
    inner: Vec<Box<dyn Hit>>,
}

impl HitList {
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline]
    pub fn push(&mut self, value: Box<dyn Hit>) {
        self.inner.push(value);
    }
}

impl Hit for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in &self.inner {
            match object.hit(r, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    rec = Some(hit);
                }
                None => continue,
            }
        }

        rec
    }
}

#[macro_export]
macro_rules! hitlist {
    () => {
        Hitlist::default()
    };
    ($($x:expr,)*) => {
        {
            let mut tmp = HitList::default();
            $(tmp.push(Box::new($x));)*
            tmp
        }
    };
    ($($x:expr),*) => {
        sidewinder::hitlist![$($x,)*]
    };
}
