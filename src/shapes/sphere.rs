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
pub struct Sphere {
    pub id: Uuid,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
    pub material: Material,
    pub inv_transformation_transposed: Matrix
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(transformation: Matrix, material: Material) -> Sphere {
        let inv = transformation.invert();
        Sphere {
            id: Uuid::new_v4(),
            inv_transformation_transposed: inv.transpose(),
            inv_transformation: inv,
            transformation: transformation,
            material: material,
        }
    }

    pub fn default() -> Sphere {
        Sphere {
            id: Uuid::new_v4(),
            transformation: Matrix::identity(4),
            inv_transformation: Matrix::identity(4),
            material: Material::default(),
            inv_transformation_transposed: Matrix::identity(4).transpose(),
        }
    }
}

impl Shape for Sphere {
    fn normal_at(&self, p: &Tuple) -> Tuple {
        let op = self.inv_transformation.multiply_tuple(p);
        let on = op.subtract(&utils::P0);
        let mut wn = self.inv_transformation_transposed.multiply_tuple(&on);
        wn.w = 0.0;
        wn.normalize()
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut result = Vec::new();

        let ray = r.transform(&self.inv_transformation);
        let shape_to_ray = ray.origin.subtract(&utils::P0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&shape_to_ray);
        let c = shape_to_ray.dot(&shape_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            result.push(Intersection::new(t1, self));

            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            result.push(Intersection::new(t2, self));
        }

        result
    }

    fn uv_coordinates(&self, p: &Tuple) -> Tuple {
        let theta = p.x.atan2(p.z);
        let radius = p.magnitude();
        let phi = (p.y / radius).acos();

        let raw_u = theta / (2.0 * std::f64::consts::PI);
        let u = 1.0 - (raw_u - 0.5);
        let v = 1.0 - (phi / std::f64::consts::PI);

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

#[allow(dead_code)]
pub fn get_default_spheres() -> Vec<Sphere> {
    let s1 = Sphere::new(
        Matrix::identity(4),
        Material::new(
            Some(Color::new(0.8, 1.0, 0.6)),
            None,
            0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
        ),
    );
    let s2 = Sphere::new(
        Matrix::scaling(0.5, 0.5, 0.5),
        Material::default(),
    );
    vec![s1, s2]
}

#[allow(dead_code)]
pub fn get_glass_sphere() -> Sphere {
    let mut result = Sphere::default();
    result.material.transparency = 1.0;
    result.material.refractive_index = 1.5;
    result
}
