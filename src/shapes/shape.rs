use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::material::Material;
use crate::ray::Ray;
use crate::world::Intersection;

use uuid::Uuid;


pub trait Shape {
    fn normal_at(&self, p: &Tuple) -> Tuple;
    fn intersect(&self, r: &Ray) -> Vec<Intersection>;
    fn uv_coordinates(&self, p: &Tuple) -> Tuple;

    fn get_id(&self) -> &Uuid;

    fn set_transformation(&mut self, t: Matrix);
    fn get_transformation(&self) -> &Matrix;
    fn get_inverse_transformation(&self) -> &Matrix;

    fn set_material(&mut self, mat: Material);
    fn get_material(&self) -> &Material;
}

