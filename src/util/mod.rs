mod camera;

use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

pub use camera::Camera;

pub struct RngDist<'a, 'b>
where
    'b: 'a,
{
    pub rng: &'a mut ThreadRng,
    pub dist: &'b Uniform<f64>,
}

impl<'a, 'b> RngDist<'a, 'b> {
    #[inline]
    #[must_use]
    pub fn new(rng: &'a mut ThreadRng, dist: &'b Uniform<f64>) -> Self {
        Self { rng, dist }
    }
}
