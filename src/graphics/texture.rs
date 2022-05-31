use crate::math::{Point, Rgb};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point) -> Rgb;
}

/// Creates a `HashMap` with `String` keys and `Arc<dyn Texture>` values.
#[macro_export]
macro_rules! texlist {
    () => {
        use std::collections::HashMap;
        use std::sync::Arc;

        use sidewinder::graphics::Texture;

        HashMap::<String, Arc<dyn Texture>>::default()
    };

    ( $($x:literal : $y:expr),* $(,)? ) => {{
        use std::collections::HashMap;
        use std::sync::Arc;

        use sidewinder::graphics::Texture;

        let mut tmp: HashMap<String, Arc<dyn Texture>> = HashMap::default();
        $(tmp.insert($x.to_string(), Arc::new($y));)*
        tmp
    }};
}

/// A solid color.
#[derive(Clone, Copy)]
pub struct Solid {
    color: Rgb,
}

impl Solid {
    #[inline]
    #[must_use]
    pub const fn new(color: Rgb) -> Self {
        Self { color }
    }
}

impl Texture for Solid {
    #[inline]
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Rgb {
        self.color
    }
}

/// A checkered texture.
pub struct Checkered {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl Checkered {
    #[inline]
    #[must_use]
    pub const fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self { even, odd }
    }

    #[inline]
    #[must_use]
    pub fn from_colors(even: Rgb, odd: Rgb) -> Self {
        Self::new(Box::new(Solid::new(even)), Box::new(Solid::new(odd)))
    }
}

impl Texture for Checkered {
    #[inline]
    fn value(&self, u: f64, v: f64, p: &Point) -> Rgb {
        if (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin() < 0.0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
