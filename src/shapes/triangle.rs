use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::Shape;
use crate::utils::EPSILON;
use crate::world::Intersection;

use uuid::Uuid;


#[derive(Clone)]
pub struct Triangle {
    pub id: Uuid,
    pub original_p1: Tuple,
    pub original_p2: Tuple,
    pub original_p3: Tuple,
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,
    pub normal: Tuple,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
    pub material: Material,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple, transformation: Matrix, material: Material) -> Triangle {
        let mut result = Triangle {
            id: Uuid::new_v4(),
            original_p1: p1,
            original_p2: p2,
            original_p3: p3,
            p1: p1,
            p2: p2,
            p3: p3,
            e1: p1,
            e2: p1,
            normal: p1,
            inv_transformation: transformation.invert(),
            transformation: transformation,
            material: material,
        };
        result.update();
        result
    }

    pub fn update(&mut self) {
        self.p1 = self.transformation.multiply_tuple(&self.original_p1);
        self.p2 = self.transformation.multiply_tuple(&self.original_p2);
        self.p3 = self.transformation.multiply_tuple(&self.original_p3);

        self.e1 = self.p2.subtract(&self.p1);
        self.e2 = self.p3.subtract(&self.p1);
        self.normal = self.p2.cross(&self.p1).normalize();
    }
}

impl Shape for Triangle {
    fn normal_at(&self, _p: &Tuple) -> Tuple {
        self.normal.clone()
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut result = Vec::new();

        // Ray is parallel to triangle.
        let dir_cross_e2 = r.direction.cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);
        if det.abs() < EPSILON {
            return result;
        }

        // Ray misses over p3-p1.
        let f = 1.0 / det;
        let p1_to_origin = r.origin.subtract(&self.p1);
        let u = f * p1_to_origin.dot(&dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return result;
        }

        // Ray misses over p1-p2 or p2-p3.
        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * r.direction.dot(&origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return result;
        }

        // Ray hits the triangle.
        let t = f * self.e2.dot(&origin_cross_e1);
        result.push(Intersection::new(t, self));

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
        self.update();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            Matrix::identity(4),
            Material::default(),
        );

        let e1 = Tuple::vector(-1.0, -1.0, 0.0);
        let e2 = Tuple::vector(1.0, -1.0, 0.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        assert_eq!(t.e1, e1);
        assert_eq!(t.e2, e2);
        assert_eq!(t.normal, normal);
    }

    #[test]
    fn test_ray_miss() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            Matrix::identity(4),
            Material::default(),
        );
        let r = Ray {
            origin: Tuple::point(0.0, -1.0, -2.0),
            direction: Tuple::vector(0.0, 1.0, 0.0)
        };
        let ix = t.intersect(&r);
        assert_eq!(ix.len(), 0);

        let r = Ray {
            origin: Tuple::point(1.0, 1.0, -2.0),
            direction: Tuple::vector(0.0, 0.0, 1.0)
        };
        let ix = t.intersect(&r);
        assert_eq!(ix.len(), 0);

        let r = Ray {
            origin: Tuple::point(-1.0, 1.0, -2.0),
            direction: Tuple::vector(0.0, 0.0, 1.0)
        };
        let ix = t.intersect(&r);
        assert_eq!(ix.len(), 0);

        let r = Ray {
            origin: Tuple::point(1.0, -1.0, -2.0),
            direction: Tuple::vector(0.0, 0.0, 1.0)
        };
        let ix = t.intersect(&r);
        assert_eq!(ix.len(), 0);
    }

    #[test]
    fn test_ray_hit() {
        let t = Triangle::new(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            Matrix::identity(4),
            Material::default(),
        );
        let r = Ray {
            origin: Tuple::point(0.0, 0.5, -2.0),
            direction: Tuple::vector(0.0, 0.0, 1.0)
        };
        let ix = t.intersect(&r);
        assert_eq!(ix.len(), 1);
        assert_eq!(ix[0].t, 2.0);
    }
}
