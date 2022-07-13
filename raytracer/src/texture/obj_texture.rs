use super::Texture;
use crate::utility::*;

use image::{ImageBuffer, RgbImage};

pub struct ObjTexture {
    ver_u: f64,
    ver_v: f64,
    du1: f64,
    dv1: f64,
    du2: f64,
    dv2: f64,
    img: RgbImage,
}

impl ObjTexture {
    pub fn new(
        ver_u: f64,
        ver_v: f64,
        du1: f64,
        du2: f64,
        dv1: f64,
        dv2: f64,
        img: RgbImage,
    ) -> Self {
        Self {
            ver_u,
            ver_v,
            du1,
            dv1,
            du2,
            dv2,
            img,
        }
    }
}

impl<'a> Texture for ObjTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        // 根据三角面顶点的uv插值得到击中点的uv
        let pu1 = self.du1 * u;
        let pv1 = self.dv1 * u;
        let pu2 = self.du2 * v;
        let pv2 = self.dv2 * v;

        let u = 1.0 - (pu1 + pu2 + self.ver_u);
        let v = pv1 + pv2 + self.ver_v;

        let i = ((u * self.img.width() as f64) as u32).clamp(0, self.img.width() - 1);
        let j = ((v * self.img.height() as f64) as u32).clamp(0, self.img.height() - 1);

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);
        Color::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}
