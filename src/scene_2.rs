//! Scene setup for book 2, section 4.4.

use std::sync::Arc;

use sidewinder::graphics::{Checkered, HitList, Lambertian};
use sidewinder::math::{Point, Rgb};
use sidewinder::object::Sphere;

#[allow(dead_code)]
pub fn two_spheres() -> HitList {
    let checkered = Arc::new(Checkered::from_colors(
        Rgb::newf(0.2, 0.3, 0.1),
        Rgb::new_all(0.9),
    ));
    let mat = Arc::new(Lambertian::new(checkered));

    sidewinder::hitlist![
        Sphere::new(Point::newi(0, -10, 0), 10.0, mat.clone()),
        Sphere::new(Point::newi(0, 10, 0), 10.0, mat),
    ]
}
