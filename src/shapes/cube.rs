use crate::color::Color;
use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::Shape;
use crate::utils::{EPSILON, equal};
use crate::world::Intersection;

use uuid::Uuid;


#[derive(Clone)]
pub struct Cube {
    pub id: Uuid,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
    pub material: Material,
}

impl Cube {
    #[allow(dead_code)]
    pub fn new(transformation: Matrix, material: Material) -> Cube {
        Cube {
            id: Uuid::new_v4(),
            inv_transformation: transformation.invert(),
            transformation: transformation,
            material: material,
        }
    }

    pub fn default() -> Cube {
        Cube {
            id: Uuid::new_v4(),
            transformation: Matrix::identity(4),
            inv_transformation: Matrix::identity(4),
            material: Material::new(
                Some(Color::new(0.0, 0.0, 1.0)),
                None,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0
            ),
        }
    }

    pub fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let mut tmin = tmin_numerator * f64::INFINITY;
        let mut tmax = tmax_numerator * f64::INFINITY;
        if direction.abs() > EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        }

        if tmin > tmax {
            return (tmax, tmin);
        }

        (tmin, tmax)
    }


}

impl Shape for Cube {
    fn normal_at(&self, p: &Tuple) -> Tuple {
        let op = self.inv_transformation.multiply_tuple(p);

        let maxc = op.x.abs().max(op.y.abs()).max(op.z.abs());

        if equal(maxc, op.x.abs()) {
            return Tuple::vector(op.x, 0.0, 0.0).normalize();
        }
        if equal(maxc, op.y.abs()) {
            return Tuple::vector(0.0, op.y, 0.0).normalize();
        }

        Tuple::vector(0.0, 0.0, op.z).normalize()
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut result = Vec::new();

        let ray = r.transform(&self.inv_transformation);

        let (xtmin, xtmax) = self.check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = self.check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = self.check_axis(ray.origin.z, ray.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin <= tmax {
            result.push(Intersection::new(tmin, self));
            result.push(Intersection::new(tmax, self));
        }

        result
    }

    fn uv_coordinates(&self, p: &Tuple) -> Tuple {
        // TODO
        Tuple::point(p.x, p.y, p.z)
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
