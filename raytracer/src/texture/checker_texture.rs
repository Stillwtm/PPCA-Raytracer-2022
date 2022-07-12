use super::{solid_color::SolidColor, Texture};
use crate::utility::*;

#[derive(Clone, Copy)]
pub struct CheckerTexture<T1, T2>
where
    T1: Texture,
    T2: Texture,
{
    odd: T1,
    even: T2,
}

impl<T1: Texture, T2: Texture> CheckerTexture<T1, T2> {
    pub fn new(odd: T1, even: T2) -> Self {
        Self { odd, even }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn new_form_color(odd: Color, even: Color) -> Self {
        Self {
            odd: SolidColor::new(odd),
            even: SolidColor::new(even),
        }
    }
}

impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines.is_sign_negative() {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
