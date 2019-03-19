#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn other(a: Axis, b: Axis) -> Axis {
        match (a, b) {
            (X, Y) => Axis::Z,
            (Y, Z) => Axis::X,
            (X, Z) => Axis::Y,
            (Y, X) => Axis::Z,
            (Z, Y) => Axis::X,
            (Z, X) => Axis::Y,
        }
    }
}
