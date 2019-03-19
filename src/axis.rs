#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn other(a: Axis, b: Axis) -> Axis {
        use Axis::*;
        match (a, b) {
            (X, Y) => Z,
            (Y, Z) => X,
            (X, Z) => Y,
            (Y, X) => Z,
            (Z, Y) => X,
            (Z, X) => Y,
            _ => panic!("There are two other axis"),
        }
    }
    pub fn other_two(a: Axis) -> (Axis, Axis) {
        use Axis::*;
        match a {
            Z => (X, Y),
            X => (Y, Z),
            Y => (X, Z),
        }
    }
}
