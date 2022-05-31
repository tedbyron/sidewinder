//! Scene setup for book 2, section 5.1.

use std::sync::Arc;

use sidewinder::graphics::{HitList, Lambertian, Noise, Perlin};
use sidewinder::math::Point;
use sidewinder::object::Sphere;

#[allow(dead_code)]
pub fn two_perlin_spheres() -> HitList {
    let tex = Arc::new(Noise::new(Perlin::new()));
    let mat = Arc::new(Lambertian::new(tex));

    sidewinder::hitlist![
        Sphere::new(Point::newi(0, -1000, 0), 1000.0, mat.clone()),
        Sphere::new(Point::newi(0, 2, 0), 2.0, mat),
    ]
}
