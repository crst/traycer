use crate::color::Color;
use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::Shape;
use crate::utils;
use crate::world::Intersection;

use uuid::Uuid;


#[derive(Clone)]
pub struct Plane {
    pub id: Uuid,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
    pub material: Material,
}

impl Plane {
    #[allow(dead_code)]
    pub fn new(transformation: Matrix, material: Material) -> Plane {
        Plane {
            id: Uuid::new_v4(),
            inv_transformation: transformation.invert(),
            transformation: transformation,
            material: material
        }
    }

    pub fn default() -> Plane {
        Plane {
            id: Uuid::new_v4(),
            transformation: Matrix::identity(4),
            inv_transformation: Matrix::identity(4),
            material: Material::new(
                Some(Color::new(0.0, 0.0, 1.0)),
                None,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )
        }
    }
}

impl Shape for Plane {
    fn normal_at(&self, _: &Tuple) -> Tuple {
        let p = Tuple::vector(0.0, 1.0, 0.0);
        self.inv_transformation.multiply_tuple(&p).normalize()
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut result = Vec::new();

        let ray = r.transform(&self.inv_transformation);
        if ray.direction.y.abs() > utils::EPSILON {
            let t = -ray.origin.y / ray.direction.y;
            result.push(Intersection::new(t, self));
        }

        result
    }

    fn uv_coordinates(&self, p: &Tuple) -> Tuple {
        let u = p.x.rem_euclid(1.0);
        let v = p.z.rem_euclid(1.0);
        Tuple::point(u, v, 0.0)
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn set_transformation(&mut self, t: Matrix) {
        self.inv_transformation = t.invert();
        self.transformation = t;
    }
    fn get_transformation(&self) -> &Matrix {
        &self.transformation
    }

    fn get_inverse_transformation(&self) -> &Matrix {
        &self.inv_transformation
    }

    fn set_material(&mut self, mat: Material) {
        self.material = mat;
    }
    fn get_material(&self) -> &Material {
        &self.material
    }
}
