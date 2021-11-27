use crate::color::Color;
use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::patterns::pattern::Pattern;
use crate::shapes::shape::Shape;


#[derive(Debug, Clone)]
pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
}

impl GradientPattern {
    #[allow(dead_code)]
    pub fn new(a: Color, b: Color, transformation: Matrix) -> GradientPattern {
        GradientPattern {
            a: a,
            b: b,
            inv_transformation: transformation.invert(),
            transformation: transformation,
        }
    }
}

impl Pattern for GradientPattern {
    fn color_at<'a>(&self, object: &'a (dyn Shape + Sync), p: &Tuple) -> Color {
        let pattern_pos = self.convert_position(object, p);
        self.a.add(&self.b.subtract(&self.a)
                   .multiply(pattern_pos.x - pattern_pos.x.floor()))
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
