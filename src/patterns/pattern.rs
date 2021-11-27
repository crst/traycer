use crate::color::Color;
use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::shapes::shape::Shape;


pub trait Pattern {
    fn color_at<'a>(&self, object: &'a (dyn Shape + Sync), p: &Tuple) -> Color;

    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn convert_position<'a>(&self, object: &'a (dyn Shape + Sync), p: &Tuple) -> Tuple {
        let object_pos = object.get_inverse_transformation().multiply_tuple(p);
        self.get_inverse_transformation().multiply_tuple(&object_pos)
    }

    fn box_clone(&self) -> Box<dyn Pattern + Sync>;
}

impl Clone for Box<dyn Pattern + Sync> {
    fn clone(&self) -> Box<dyn Pattern + Sync> {
        self.box_clone()
    }
}
