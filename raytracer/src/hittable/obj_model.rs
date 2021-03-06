use std::sync::Arc;

use super::hittable_list::HittableList;
use super::triangle::Triangle;
use super::{hittable_list, HitRecord, Hittable};
use crate::bvh::aabb::AABB;
use crate::bvh::bvh_node::BvhNode;
use crate::material::Material;
use crate::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
};
use crate::texture::image_texture::ImageTexture;
use crate::texture::obj_texture::ObjTexture;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::utility::*;
use console::style;
use image::RgbImage;
use rand::Rng;
use tobj;

pub struct ObjModel {
    triangles: BvhNode,
    pub center: Point3,
}

impl ObjModel {
    pub fn new_from_file<T>(file_obj: &str, scale: f64, mat: T) -> Self
    where
        T: Material + Send + Sync + Clone + 'static,
    {
        println!("π° Imortinging model...");

        let mut tris_list = HittableList::default();

        match tobj::load_obj(
            file_obj,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        ) {
            Ok((models, mats)) => {
                for (model_idx, model) in models.iter().enumerate() {
                    let mesh = &model.mesh;

                    for idx in 0..mesh.indices.len() / 3 {
                        let i = mesh.indices[idx * 3] as usize;
                        let j = mesh.indices[idx * 3 + 1] as usize;
                        let k = mesh.indices[idx * 3 + 2] as usize;

                        // // ε€ηζθ΄¨
                        // let mat = Lambertian::new(SolidColor::new(Color::new(0.5, 0.5, 0.5)));

                        // ε€ηζ¨‘εδΈθ§ι’
                        tris_list.add(Arc::new(Triangle::new(
                            [
                                Point3::new(
                                    mesh.positions[3 * i] as f64,
                                    mesh.positions[3 * i + 1] as f64,
                                    mesh.positions[3 * i + 2] as f64,
                                ) * scale,
                                Point3::new(
                                    mesh.positions[3 * j] as f64,
                                    mesh.positions[3 * j + 1] as f64,
                                    mesh.positions[3 * j + 2] as f64,
                                ) * scale,
                                Point3::new(
                                    mesh.positions[3 * k] as f64,
                                    mesh.positions[3 * k + 1] as f64,
                                    mesh.positions[3 * k + 2] as f64,
                                ) * scale,
                            ],
                            mat.clone(),
                        )));
                    }
                }
            }
            Err(_) => println!(
                "  Error: {}{}",
                style("Failed to load OBJ file: ").red(),
                style(file_obj).yellow()
            ),
        }

        let triangles = BvhNode::new_from_list(&mut tris_list, 0.0, 1.0);
        Self {
            center: (triangles.node_box.maximum + triangles.node_box.minimum) / 2.0,
            triangles,
        }
    }

    pub fn new_from_file_with_texture(file_obj: &str, scale: f64, file_texture: &str) -> Self {
        println!("π° Imortinging model...");

        let mut tris_list = HittableList::default();

        let mut img = RgbImage::default();
        img = match image::open(file_texture) {
            Ok(img) => img,
            Err(_) => panic!("Couldn't open file: {}", file_texture),
        }
        .into_rgb8();

        match tobj::load_obj(
            file_obj,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        ) {
            Ok((models, mats)) => {
                for (model_idx, model) in models.iter().enumerate() {
                    let mesh = &model.mesh;

                    for idx in 0..mesh.indices.len() / 3 {
                        let i = mesh.indices[idx * 3] as usize;
                        let j = mesh.indices[idx * 3 + 1] as usize;
                        let k = mesh.indices[idx * 3 + 2] as usize;

                        // ε€ηηΊΉη
                        let u1 = mesh.texcoords[2 * i] as f64;
                        let v1 = mesh.texcoords[2 * i + 1] as f64;
                        let u2 = mesh.texcoords[2 * j] as f64;
                        let v2 = mesh.texcoords[2 * j + 1] as f64;
                        let u3 = mesh.texcoords[2 * k] as f64;
                        let v3 = mesh.texcoords[2 * k + 1] as f64;
                        let text = ObjTexture::new(
                            u1,
                            v1,
                            u2 - u1,
                            u3 - u1,
                            v2 - v1,
                            v3 - v1,
                            img.clone(),
                        );

                        // ε€ηζθ΄¨
                        let mat = Lambertian::new(text);

                        // ε€ηζ¨‘εδΈθ§ι’
                        tris_list.add(Arc::new(Triangle::new(
                            [
                                Point3::new(
                                    mesh.positions[3 * i] as f64,
                                    mesh.positions[3 * i + 1] as f64,
                                    mesh.positions[3 * i + 2] as f64,
                                ) * scale,
                                Point3::new(
                                    mesh.positions[3 * j] as f64,
                                    mesh.positions[3 * j + 1] as f64,
                                    mesh.positions[3 * j + 2] as f64,
                                ) * scale,
                                Point3::new(
                                    mesh.positions[3 * k] as f64,
                                    mesh.positions[3 * k + 1] as f64,
                                    mesh.positions[3 * k + 2] as f64,
                                ) * scale,
                            ],
                            mat,
                        )));
                    }
                }
            }
            Err(_) => println!(
                "  Error: {}{}",
                style("Failed to load OBJ file: ").red(),
                style(file_obj).yellow()
            ),
        }

        let triangles = BvhNode::new_from_list(&mut tris_list, 0.0, 1.0);
        Self {
            center: (triangles.node_box.maximum + triangles.node_box.minimum) / 2.0,
            triangles,
        }
    }

    pub fn new_from_file_with_rand_texture(file_obj: &str, scale: f64, file_texture: &str) -> Self {
        println!("π° Imortinging model...");

        let mut tris_list = HittableList::default();

        let mut rng = rand::thread_rng();

        let mut img = RgbImage::default();
        img = match image::open(file_texture) {
            Ok(img) => img,
            Err(_) => panic!("Couldn't open file: {}", file_texture),
        }
        .into_rgb8();

        match tobj::load_obj(
            file_obj,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        ) {
            Ok((models, mats)) => {
                for (model_idx, model) in models.iter().enumerate() {
                    let mesh = &model.mesh;

                    for idx in 0..mesh.indices.len() / 3 {
                        let i = mesh.indices[idx * 3] as usize;
                        let j = mesh.indices[idx * 3 + 1] as usize;
                        let k = mesh.indices[idx * 3 + 2] as usize;

                        // ε€ηηΊΉη
                        let u1 = rng.gen::<f64>();
                        let v1 = rng.gen::<f64>();
                        let u2 = rng.gen::<f64>();
                        let v2 = rng.gen::<f64>();
                        let u3 = rng.gen::<f64>();
                        let v3 = rng.gen::<f64>();
                        let text = ObjTexture::new(
                            u1,
                            v1,
                            u2 - u1,
                            u3 - u1,
                            v2 - v1,
                            v3 - v1,
                            img.clone(),
                        );

                        // ε€ηζθ΄¨
                        let mat = Lambertian::new(text);

                        // ε€ηζ¨‘εδΈθ§ι’
                        tris_list.add(Arc::new(Triangle::new(
                            [
                                Point3::new(
                                    mesh.positions[3 * i] as f64,
                                    mesh.positions[3 * i + 1] as f64,
                                    mesh.positions[3 * i + 2] as f64,
                                ) * scale,
                                Point3::new(
                                    mesh.positions[3 * j] as f64,
                                    mesh.positions[3 * j + 1] as f64,
                                    mesh.positions[3 * j + 2] as f64,
                                ) * scale,
                                Point3::new(
                                    mesh.positions[3 * k] as f64,
                                    mesh.positions[3 * k + 1] as f64,
                                    mesh.positions[3 * k + 2] as f64,
                                ) * scale,
                            ],
                            mat,
                        )));
                    }
                }
            }
            Err(_) => println!(
                "  Error: {}{}",
                style("Failed to load OBJ file: ").red(),
                style(file_obj).yellow()
            ),
        }

        let triangles = BvhNode::new_from_list(&mut tris_list, 0.0, 1.0);
        Self {
            center: (triangles.node_box.maximum + triangles.node_box.minimum) / 2.0,
            triangles,
        }
    }
}

impl Hittable for ObjModel {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        self.triangles.bounding_box(st_time, ed_time)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.triangles.hit(r, t_min, t_max)
    }
}
