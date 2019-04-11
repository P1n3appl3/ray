use super::aabb::AABB;
use super::{HitRecord, Hitable};
use crate::bvh::BVHNode;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
use tobj;

#[macro_export]
macro_rules! mesh {
    ($name:expr, $size:expr, $mat:expr) => {
        Box::new(Mesh::new($name, $size as f32, $mat))
    };
}

#[derive(Debug)]
pub struct Mesh {
    pub faces: BVHNode,
}

impl Mesh {
    fn generate_normals(points: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
        (0..points.len())
            .map(|p| {
                indices
                    .chunks(3)
                    .filter_map(|i| {
                        let (a, b, c) = (i[0] as usize, i[1] as usize, i[2] as usize);
                        if a == p || b == p || c == p {
                            Some((points[b] - points[a]).cross(&(points[c] - points[a])))
                        } else {
                            None
                        }
                    })
                    .fold(Vec3::zero(), |a, b| a + b)
                    .normalize()
            })
            .collect()
    }

    pub fn new(filename: &str, scale: f32, material: Arc<dyn Material>) -> Self {
        let teapot = tobj::load_obj(&std::path::Path::new(filename));
        let (model, _material) = teapot.unwrap();
        // TODO: support objs with more than 1 mesh
        let mesh = &model[0].mesh;
        let points: Vec<Vec3> = mesh
            .positions
            .chunks(3)
            .map(|pos| Vec3::new(pos[0], pos[1], pos[2]) * scale)
            .collect();
        let normals: Vec<Vec3> = if mesh.normals.is_empty() {
            Mesh::generate_normals(&points, &mesh.indices)
        } else {
            mesh.normals
                .chunks(3)
                .map(|pos| Vec3::new(pos[0], pos[1], pos[2]))
                .collect()
        };
        let texture_coords: Vec<Vec3> = if mesh.texcoords.is_empty() {
            vec![Vec3::zero(); points.len() * 3 / 2]
        } else {
            mesh.texcoords
                .chunks(2)
                .map(|pos| Vec3::new(pos[0], pos[1], 0.0))
                .collect()
        };
        let vertices: Vec<Arc<Vertex>> = (0..points.len())
            .map(|i| Arc::new(Vertex::new(points[i], normals[i], texture_coords[i])))
            .collect();
        let mut triangles = mesh
            .indices
            .chunks(3)
            .map(|i| {
                let (a, b, c) = (i[0] as usize, i[1] as usize, i[2] as usize);
                Box::new(Triangle::new(
                    vertices[a].clone(),
                    vertices[b].clone(),
                    vertices[c].clone(),
                    material.clone(),
                )) as Box<dyn Hitable>
            })
            .collect();
        Mesh {
            faces: BVHNode::from(&mut triangles),
        }
    }
}

impl Hitable for Mesh {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }
    fn get_bb(&self) -> AABB {
        self.faces.get_bb()
    }
}

#[derive(Debug)]
pub struct Vertex {
    pos: Vec3,
    normal: Vec3,
    // TODO: replace with Vec2
    texture: Vec3,
}

impl Vertex {
    pub fn new(pos: Vec3, normal: Vec3, texture: Vec3) -> Vertex {
        Vertex {
            pos,
            normal,
            texture,
        }
    }
}

impl From<Vec3> for Vertex {
    fn from(v: Vec3) -> Vertex {
        Vertex {
            pos: v,
            normal: Vec3::zero(),
            texture: Vec3::zero(),
        }
    }
}

#[derive(Debug)]
pub struct Triangle {
    v0: Arc<Vertex>,
    v1: Arc<Vertex>,
    v2: Arc<Vertex>,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(
        v0: Arc<Vertex>,
        v1: Arc<Vertex>,
        v2: Arc<Vertex>,
        material: Arc<dyn Material>,
    ) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            material,
        }
    }
}

impl Hitable for Triangle {
    /// Solving ray triangle intersection with Möller-Trumbore algorithm:
    /// Triangle verticies A B and C
    /// Triangle edges E1 = B - A and E2 = C - A
    /// Ray origin O and direction D
    /// Barycentric coords u and v express P = (1-u-v)A * uB * vC
    /// Relative position of ray origin to vertex A is called T = (O-A)
    /// Solving the system using Cramers rule:
    ///
    /// t        1      |T E1 E2|        (note that scalar tripple product
    /// u  = ---------  |D T  E2|         |A B C| is equal to AxB.C and
    /// v    |D E1 E2|  |D E1 T |         also A.BxC)
    ///
    /// if 0 <= u <= 1 and 0 <= u + v <= 1 then the collision is valid
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.v1.pos - self.v0.pos;
        let edge2 = self.v2.pos - self.v0.pos;
        let d_cross_e2 = r.dir.cross(&edge2);
        let det = edge1.dot(&d_cross_e2);
        if det.abs() <= std::f32::EPSILON {
            // ray is parallel to plane
            return None;
        }
        //TODO: only back-face cull for dielectrics
        if det <= 0.0 {
            // intersection is with the back-face of the triangle
            return None;
        }
        let tvec = r.origin - self.v0.pos;
        let u = tvec.dot(&d_cross_e2) / det;
        if u <= 0.0 || u >= 1.0 {
            return None;
        }
        let t_cross_e1 = tvec.cross(&edge1);
        let v = r.dir.dot(&t_cross_e1) / det;
        if v <= 0.0 || u + v >= 1.0 {
            return None;
        }
        let t = edge2.dot(&t_cross_e1) / det;
        if t < t_min || t > t_max {
            return None;
        }
        let w = 1.0 - u - v;
        let texture_coords =
            self.v0.texture * w + self.v1.texture * u + self.v2.texture * v;
        Some(HitRecord {
            t,
            u: texture_coords.x,
            v: texture_coords.y,
            point: self.v0.pos + edge1 * u + edge2 * v,
            // interpolate normal between vertex normals
            normal: self.v0.normal * w + self.v1.normal * u + self.v2.normal * v,
            material: self.material.as_ref(),
        })
    }
    fn get_bb(&self) -> AABB {
        let min = self
            .v0
            .pos
            .piecewise_min(&self.v1.pos)
            .piecewise_min(&self.v2.pos);
        let max = self
            .v0
            .pos
            .piecewise_max(&self.v1.pos)
            .piecewise_max(&self.v2.pos);
        AABB::new(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::aabb::AABB;
    use crate::material::specular::Specular;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TRI: Triangle = Triangle::new(
            Arc::new(Vec3::new(0, 0, 0).into()),
            Arc::new(Vec3::new(0, 4, 0).into()),
            Arc::new(Vec3::new(4, 0, 0).into()),
            Arc::new(Specular::new(Vec3::from(1), 0.0)),
        );
    }

    #[test]
    fn test_bb() {
        assert_eq!(
            TRI.get_bb(),
            AABB::new(Vec3::new(0, 0, 0), Vec3::new(4, 4, 0))
        );
    }
    #[test]
    fn test_hit() {
        let hit = TRI
            .hit(
                Ray::new(Vec3::new(1, 1, -2), Vec3::new(0, 0, 1)),
                0.0,
                std::f32::MAX,
            )
            .unwrap();
        assert_eq!(hit.t, 2.0);
        assert_eq!(hit.point, Vec3::new(1, 1, 0));
        let hit = TRI
            .hit(
                Ray::new(Vec3::new(1, 0, -1), Vec3::new(0, 1, 1)),
                0.0,
                std::f32::MAX,
            )
            .unwrap();
        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.point, Vec3::new(1, 1, 0));
    }
    #[test]
    fn test_back_hit() {
        assert!(TRI
            .hit(
                Ray::new(Vec3::new(1, 1, 2), Vec3::new(0, 0, -1)),
                0.0,
                std::f32::MAX,
            )
            .is_none());
    }
    #[test]
    fn test_miss() {
        let hit = TRI.hit(
            Ray::new(Vec3::new(1, 1, 2), Vec3::new(0, 0, -1)),
            0.0,
            std::f32::MAX,
        );
        assert!(hit.is_none());
    }
}
