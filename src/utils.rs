use crate::linalg::tuple::Tuple;


// Can't use f64::EPSILON since it's too small.
pub static EPSILON: f64 = 0.001;
pub static P0: Tuple = Tuple { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

pub fn equal(a: f64, b: f64) -> bool {
    return (a - b).abs() < EPSILON;
}
