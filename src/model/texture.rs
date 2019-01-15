use crate::vec3::Vec3;

#[derive(Clone)]
pub enum Texture {
    Solid(Vec3),
    Checker(Vec3, Vec3),
    Noise {
        rand_float: [f32; 256],
        perm_x: [u8; 256],
        perm_y: [u8; 256],
        perm_z: [u8; 256],
    },
    Image(Vec<Vec<[u8; 3]>>),
}

impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                _ => "some texture", // TODO: add variants
            }
        )
    }
}
