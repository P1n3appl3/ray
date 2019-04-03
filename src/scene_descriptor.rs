use crate::background::Background;
use crate::scene::Scene;
use serde_tuple::Deserialize_tuple;

pub fn load_scene<T: Background>(_file: &str) -> Scene<T> {
    unimplemented!();
}
