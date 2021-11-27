use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::Shape;
use crate::utils::EPSILON;
use crate::world::Intersection;

use uuid::Uuid;


#[derive(Clone)]
pub struct Cylinder {
    pub id: Uuid,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
    pub material: Material,

    pub min_y: f64,
    pub max_y: f64,
    pub closed: bool,
}

impl Cylinder {
    #[allow(dead_code)]
    pub fn new(transformation: Matrix, material: Material, min_y: f64, max_y: f64, closed: bool) -> Cylinder {
        Cylinder {
            id: Uuid::new_v4(),
            inv_transformation: transformation.invert(),
            transformation: transformation,
            material: material,
            min_y: min_y,
            max_y: max_y,
            closed: closed,
        }
    }

    pub fn default() -> Cylinder {
        Cylinder {
            id: Uuid::new_v4(),
            inv_transformation: Matrix::identity(4),
            transformation: Matrix::identity(4),
            material: Material::default(),
            min_y: 0.0,
            max_y: 1.0,
            closed: false,
        }
    }

    pub fn check_cap(&self, r: &Ray, t: f64) -> bool {
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        (x * x + z * z) <= 1.0
    }

    pub fn intersect_caps(&self, r: &Ray) -> Vec<Intersection> {
        let mut result = Vec::new();
        if self.closed && r.direction.y.abs() > EPSILON {
            let t1 = (self.min_y - r.origin.y) / r.direction.y;
            if self.check_cap(r, t1) {
                result.push(Intersection::new(t1, self));
            }

            let t2 = (self.max_y - r.origin.y) / r.direction.y;
            if self.check_cap(r, t2) {
                result.push(Intersection::new(t2, self));
            }
        }
        result
    }
}

impl Shape for Cylinder {
    fn normal_at(&self, p: &Tuple) -> Tuple {
        let op = self.inv_transformation.multiply_tuple(p);

        let dist = (op.x * op.x) + (op.z * op.z);

        let mut result = Tuple::vector(op.x, 0.0, op.z);
        if dist < 1.0 && op.y >= self.max_y - EPSILON {
            result = self.inv_transformation
                .multiply_tuple(&Tuple::vector(0.0, 1.0, 0.0));
        }
        if dist < 1.0 && op.y <= self.min_y + EPSILON {
            result = self.inv_transformation
                .multiply_tuple(&Tuple::vector(0.0, -1.0, 0.0));
        }

        result.normalize()
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut result = Vec::new();

        let ray = r.transform(&self.get_inverse_transformation());
        let a = (ray.direction.x * ray.direction.x) + (ray.direction.z * ray.direction.z);

        if a.abs() > EPSILON {
            let b = (2.0 * ray.origin.x * ray.direction.x) +
                (2.0 * ray.origin.z * ray.direction.z);
            let c = (ray.origin.x * ray.origin.x) + (ray.origin.z * ray.origin.z) - 1.0;
            let discriminant = (b * b) - (4.0 * a * c);

            if discriminant >= 0.0 {
                let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
                let y1 = ray.origin.y + t1 * ray.direction.y;
                if self.min_y <= y1 && y1 <= self.max_y {
                    result.push(Intersection::new(t1, self));
                }

                let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
                let y2 = ray.origin.y + t2 * ray.direction.y;
                if self.min_y <= y2 && y2 <= self.max_y {
                    result.push(Intersection::new(t2, self));
                }
            }
        }

        let mut caps = self.intersect_caps(&ray);
        result.append(&mut caps);

        result
    }

    fn uv_coordinates(&self, p: &Tuple) -> Tuple {
        if self.closed && false {
            // TODO: case for caps.
            let u = p.x.rem_euclid(1.0);
            let v = p.z.rem_euclid(1.0);
            return Tuple::point(u, v, 0.0);
        } else {
            let theta = p.x.atan2(p.z);
            let raw_u = theta / (2.0 * std::f64::consts::PI);
            let u = 1.0 - (raw_u + 0.5);
            let v = p.y.rem_euclid(1.0);
            return Tuple::point(u, v, 0.0);
        }
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
