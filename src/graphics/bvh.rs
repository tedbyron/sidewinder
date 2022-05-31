use std::cmp::Ordering;

use rand::distributions::Standard;
use rand::prelude::*;

use crate::graphics::{Aabb, Hit, HitRecord, Ray};
use crate::math::Axis;

/// A bounding volume hierarchy.
pub struct Bvh {
    node: Node,
    aabb: Aabb,
}

/// A node in a bounding volume hierarchy.
enum Node {
    Branch { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hit>),
}

impl Bvh {
    /// # Panics
    ///
    /// If a bounding box cannot be calculated.
    pub fn new(
        mut objects: Vec<Box<dyn Hit>>,
        t_start: f64,
        t_end: f64,
        rng: &mut ThreadRng,
    ) -> Self {
        match objects.len() {
            0 => panic!("No objects objects passed to BVH node"),
            1 => {
                let obj = objects.pop().unwrap();
                let aabb = obj
                    .bounding_box(t_start, t_end)
                    .expect("No bounding box in BVH node");

                Self {
                    node: Node::Leaf(obj),
                    aabb,
                }
            }
            _ => {
                let axis = Standard.sample(rng);
                objects.sort_unstable_by(|l, r| Self::box_cmp(l, r, axis));
                let left = Box::new(Self::new(
                    objects.drain(..objects.len() / 2).collect(),
                    t_start,
                    t_end,
                    rng,
                ));
                let right = Box::new(Self::new(objects, t_start, t_end, rng));
                let box_left = left
                    .bounding_box(t_start, t_end)
                    .expect("No bounding box in BVH node");
                let box_right = right
                    .bounding_box(t_start, t_end)
                    .expect("No bounding box in BVH node");

                Self {
                    node: Node::Branch { left, right },
                    aabb: Aabb::surrounding_box(box_left, box_right),
                }
            }
        }
    }

    fn box_cmp(left: &dyn Hit, right: &dyn Hit, axis: Axis) -> Ordering {
        left.bounding_box(0.0, 0.0)
            .expect("No bounding box in BVH node")
            .min[axis]
            .partial_cmp(
                &right
                    .bounding_box(0.0, 0.0)
                    .expect("No bounding box in BVH node")
                    .min[axis],
            )
            .expect("Bounding box values cannot be compared")
    }
}

impl Hit for Bvh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        if !self.aabb.hit(r, t_min, t_max) {
            return None;
        }

        match &self.node {
            Node::Branch { left, right } => {
                let left = left.hit(r, t_min, t_max);

                if left.is_some() {
                    left
                } else {
                    right.hit(
                        r,
                        t_min,
                        match left {
                            Some(ref rec) => rec.t,
                            None => t_max,
                        },
                    )
                }
            }
            Node::Leaf(node) => node.hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.aabb)
    }
}
