#![allow(clippy::module_name_repetitions)]

use crate::graphics::Ray;
use crate::math::{Point, Vec3};

use super::material::Material;

/// A record of a ray-object intersection. The `mat` field is a `&dyn Material` to avoid atomic
/// operations in loops (e.g. cloning an `Arc<dyn Material>`).
#[non_exhaustive]
#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
    pub mat: &'a dyn Material,
}

/// The front or back of an object's surface.
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

    /// Get a [`Face`] and outward normal such that the normal always points against the incident
    /// [`Ray`].
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

/// Abstraction for objects whose surface may intersect a [`Ray`].
pub trait Hit: Send + Sync {
    /// Check whether a [`Ray`] intersects `self`, and if it does, get a record of data about the
    /// resulting intersection.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>>;
}

/// Container of singly-owned objects that implement [`Hit`].
#[non_exhaustive]
#[derive(Default)]
pub struct HitList(Vec<Box<dyn Hit>>);

impl HitList {
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Appends a value to the list.
    #[inline]
    pub fn push(&mut self, value: Box<dyn Hit>) {
        self.0.push(value);
    }
}

impl Hit for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in &self.0 {
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

    ( $($x:expr,)* $(,)? ) => {
        {
            let mut tmp = HitList::default();
            $(tmp.push(Box::new($x));)*
            tmp
        }
    };
}
