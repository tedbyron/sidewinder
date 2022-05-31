//! Scene setup for book 1 and book 2 until section 4.4.

use std::sync::Arc;

use rand::distributions::Uniform;
use rand::prelude::Distribution;
use sidewinder::graphics::{Checkered, Dielectric, HitList, Lambertian, Metallic, Solid};
use sidewinder::math::{Point, Rgb, Vec3};
use sidewinder::object::{MovingSphere, Sphere};
use sidewinder::rng::CLOSED_OPEN_01;

#[allow(dead_code)]
pub fn setup() -> HitList {
    let textures = sidewinder::texlist![
        "ground": Checkered::from_colors(Rgb::newf(0.2, 0.3, 0.1), Rgb::new_all(0.9)),
        "lambertian": Solid::new(Rgb::newf(0.4, 0.2, 0.1)),
    ];
    let mats = sidewinder::matlist![
        "ground": Lambertian::new(textures["ground"].clone()),
        "dielectric": Dielectric::new(1.5),
        "lambertian": Lambertian::new(textures["lambertian"].clone()),
        "metallic": Metallic::new(Rgb::newf(0.7, 0.6, 0.5), 0.0),
    ];
    let mut world = sidewinder::hitlist![
        Sphere::new(Point::newi(0, -1000, 0), 1000.0, mats["ground"].clone()),
        Sphere::new(Point::newi(0, 1, 0), 1.0, mats["dielectric"].clone()),
        Sphere::new(Point::newi(-4, 1, 0), 1.0, mats["lambertian"].clone()),
        Sphere::new(Point::newi(4, 1, 0), 1.0, mats["metallic"].clone()),
    ];

    let mut rng = rand::thread_rng();
    let uniform_0_p5 = Uniform::<f64>::from(0.0..0.5);
    let uniform_p5_1 = Uniform::<f64>::from(0.5..1.0);
    let offset = Point::newf(4.0, 0.2, 0.0);

    for a in -11..11 {
        let a = f64::from(a);

        for b in -11..11 {
            let b = f64::from(b);

            let choose_mat = CLOSED_OPEN_01.sample(&mut rng);
            let center = Point::newf(
                0.9_f64.mul_add(CLOSED_OPEN_01.sample(&mut rng), a),
                0.2,
                0.9_f64.mul_add(CLOSED_OPEN_01.sample(&mut rng), b),
            );

            if (center - offset).len() > 0.9 {
                match choose_mat {
                    n if n < 0.8 => {
                        let albedo =
                            Arc::new(Solid::new(Rgb::random(&mut rng) * Rgb::random(&mut rng)));
                        let center_end =
                            center + Vec3::newf(0.0, uniform_0_p5.sample(&mut rng), 0.0);
                        let sphere = MovingSphere::new(
                            center,
                            center_end,
                            0.0,
                            1.0,
                            0.2,
                            Arc::new(Lambertian::new(albedo)),
                        );

                        world.push(Box::new(sphere));
                    }
                    n if n < 0.95 => {
                        let albedo = Rgb::random_in(&uniform_p5_1, &mut rng);
                        let blur = uniform_0_p5.sample(&mut rng);
                        let sphere =
                            Sphere::new(center, 0.2, Arc::new(Metallic::new(albedo, blur)));

                        world.push(Box::new(sphere));
                    }
                    _ => {
                        let sphere = Sphere::new(center, 0.2, mats["dielectric"].clone());

                        world.push(Box::new(sphere));
                    }
                };
            }
        }
    }

    world
}
