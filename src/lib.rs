pub mod axis;
pub mod background;
pub mod bvh;
pub mod camera;
pub mod geometry;
pub mod material;
pub mod ray;
pub mod scene;
pub mod texture;
pub mod vec3;

#[macro_export]
macro_rules! use_all {
    () => {
        use ray::axis::Axis;
        use ray::background::Gradient;
        use ray::bvh::BVHNode;
        use ray::camera::Camera;
        use ray::geometry::{
            mesh::Mesh, prism::Prism, rect::Rect, sphere::Sphere, transform::*, Hitable,
        };
        use ray::material::{
            dielectric::Dielectric, diffuse::Diffuse, isotropic::Isotropic, light::Light,
            specular::Specular,
        };
        use ray::scene::*;
        use ray::texture::{
            checker::Checkered, gradient::SimpleGradient, image::*, perlin::*, solid::Solid,
        };
        use ray::vec3::Vec3;
        use std::sync::Arc;
    };
}
