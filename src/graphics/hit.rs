#![allow(clippy::module_name_repetitions)]

use crate::graphics::Ray;
use crate::math::{Point, Vec3};

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
    #[inline]
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

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
