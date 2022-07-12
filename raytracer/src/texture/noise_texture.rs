use std::borrow::{Borrow, BorrowMut};
use std::mem::swap;
use std::ops::RangeFull;

use super::Texture;
use crate::utility::*;

use rand::Rng;

#[derive(Clone, Copy)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (p.z * self.scale + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

////////////////////////////////Perlin////////////////////////////////

const POINT_COUNT: usize = 256;

#[derive(Clone, Copy)]
struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    fn permute(p: &mut [usize]) {
        let mut rng = rand::thread_rng();
        for i in (0..p.len()).rev() {
            let target = rng.gen_range(0..=i);
            swap(&mut &p[i], &mut &p[target]);
        }
    }

    fn perlin_generate_from() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Perlin::permute(&mut p);
        p
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermitian Smoothing
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = di as f64;
                    let j = dj as f64;
                    let k = dk as f64;
                    let weight_v = Vec3::new(u - i, v - j, w - k);
                    accum += (i * u + (1. - i) * (1. - u))
                        * (j * v + (1. - j) * (1. - v))
                        * (k * w + (1. - k) * (1. - w))
                        * Vec3::dot(&c[di][dj][dk], &weight_v);
                }
            }
        }
        accum
    }

    pub fn new() -> Self {
        let mut ranvec = [Vec3::default(); POINT_COUNT];

        for i in ranvec.iter_mut() {
            *i = Vec3::rand_vec_range(-1.0, 1.0).unit_vector();
        }
        Self {
            ranvec,
            perm_x: Perlin::perlin_generate_from(),
            perm_y: Perlin::perlin_generate_from(),
            perm_z: Perlin::perlin_generate_from(),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Perlin::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, mut p: Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        accum.abs()
    }
}
