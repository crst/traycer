use crate::color::Color;
use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::patterns::pattern::Pattern;
use crate::shapes::shape::Shape;


#[derive(Debug, Clone)]
pub struct CheckersPattern {
    pub a: Color,
    pub b: Color,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
}

impl CheckersPattern {
    pub fn new(a: Color, b: Color, transformation: Matrix) -> CheckersPattern {
        CheckersPattern {
            a: a,
            b: b,
            inv_transformation: transformation.invert(),
            transformation: transformation,
        }
    }
}

impl Pattern for CheckersPattern {
    fn color_at<'a>(&self, object: &'a (dyn Shape + Sync), p: &Tuple) -> Color {
        let px = self.convert_position(object, p);
        let pattern_pos = object.uv_coordinates(&px);
        let n =
            (2.0 * pattern_pos.x).floor() +
            (2.0 * pattern_pos.y).floor() +
            (2.0 * pattern_pos.z).floor();
        match n.round() as i64 % 2 {
            0 => self.a,
            _ => self.b,
        }
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation
    }
    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inv_transformation
    }

    fn box_clone(&self) -> Box<dyn Pattern + Sync> {
        Box::new((*self).clone())
    }
}
