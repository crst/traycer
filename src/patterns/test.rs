use crate::color::Color;
use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::patterns::pattern::Pattern;
use crate::shapes::shape::Shape;


#[derive(Debug, Clone)]
pub struct TestPattern {
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
}

impl TestPattern {
    #[allow(dead_code)]
    pub fn new(transformation: Matrix) -> TestPattern {
        TestPattern {
            inv_transformation: transformation.invert(),
            transformation: transformation,
        }
    }
}

impl Pattern for TestPattern {
    fn color_at<'a>(&self, object: &'a (dyn Shape + Sync), p: &Tuple) -> Color {
        let px = self.convert_position(object, p);
        Color::new(px.x, px.y, px.z)
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
