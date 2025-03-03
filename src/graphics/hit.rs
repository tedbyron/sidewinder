use crate::{
    graphics::{Aabb, Material, Ray},
    math::{Point, Vec3},
};

/// Abstraction for objects whose surface may intersect a [`Ray`].
pub trait Hit: Send + Sync {
    /// Check whether a [`Ray`] intersects `self`, and if it does, get a record of data about the
    /// resulting intersection.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>>;
    fn bounding_box(&self, t_start: f64, t_end: f64) -> Option<Aabb>;
}

impl Hit for Box<dyn Hit> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        (**self).hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t_start: f64, t_end: f64) -> Option<Aabb> {
        (**self).bounding_box(t_start, t_end)
    }
}

/// A record of a ray-object intersection. The `mat` field is a `&dyn Material` to avoid atomic
/// operations in loops (e.g. cloning an `Arc<dyn Material>`).
pub struct HitRecord<'a> {
    pub point: Point,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub face: Face,
}

/// The front or back of an object's surface.
pub enum Face {
    Front,
    Back,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point,
        normal: Vec3,
        mat: &'a dyn Material,
        t: f64,
        u: f64,
        v: f64,
        face: Face,
    ) -> Self {
        Self {
            point,
            normal,
            mat,
            t,
            u,
            v,
            face,
        }
    }

    /// Get a [`Face`] and outward normal such that the normal always points against the incident
    /// [`Ray`].
    pub fn face_normal(r: &Ray, outward_normal: Vec3) -> (Face, Vec3) {
        if r.direction.dot(outward_normal) < 0.0 {
            (Face::Front, outward_normal)
        } else {
            (Face::Back, -outward_normal)
        }
    }
}

/// Container of singly-owned objects that implement [`Hit`].
pub type HitList = Vec<Box<dyn Hit>>;

impl Hit for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                rec = Some(hit);
            }
        }

        rec
    }

    fn bounding_box(&self, t_start: f64, t_end: f64) -> Option<Aabb> {
        let mut box_ = Aabb::new(Vec3::ZERO, Vec3::ZERO);
        let mut first_box = true;

        for object in self {
            match object.bounding_box(t_start, t_end) {
                Some(bounding_box) => {
                    box_ = if first_box {
                        bounding_box
                    } else {
                        box_.surrounding_box(bounding_box)
                    };
                    first_box = false;
                }
                None => return None,
            }
        }

        Some(box_)
    }
}

/// Creates a `Vec` of objects that implement [`Hit`].
#[macro_export]
macro_rules! hitlist {
    ( $($x:expr),* $(,)? ) => {
        {
            let mut list = HitList::default();
            $(list.push(Box::new($x));)*
            list
        }
    };
}
