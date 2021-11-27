use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;


#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64) -> Ray {
        Ray {
            origin: Tuple::point(ox, oy, oz),
            direction: Tuple::vector(dx, dy, dz),
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin.add(&self.direction.multiply(t))
    }

    pub fn transform(&self, m: &Matrix) -> Self {
        Ray {
            origin: m.multiply_tuple(&self.origin),
            direction: m.multiply_tuple(&self.direction),
        }
    }
}

pub fn reflect(ray: &Tuple, normal: &Tuple) -> Tuple {
    ray.subtract(&normal.multiply(2.0).multiply(ray.dot(normal)))
}



#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use crate::material::Material;
    use crate::shapes::shape::Shape;
    use crate::shapes::sphere::Sphere;

    #[test]
    fn test_intersect() {
        let r = Ray::new(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere::default());
        let x = s.intersect(&r);
        assert_eq!(4.0, x[0].t);
        assert_eq!(6.0, x[1].t);

        let r = Ray::new(0.0, 1.0, -5.0, 0.0, 0.0, 1.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere::default());
        let x = s.intersect(&r);
        assert_eq!(5.0, x[0].t);
        assert_eq!(5.0, x[1].t);

        let r = Ray::new(0.0, 2.0, -5.0, 0.0, 0.0, 1.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere::default());
        assert_eq!(0, s.intersect(&r).len());

        let r = Ray::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere::default());
        let x = s.intersect(&r);
        assert_eq!(-1.0, x[0].t);
        assert_eq!(1.0, x[1].t);

        let r = Ray::new(0.0, 0.0, 5.0, 0.0, 0.0, 1.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere::default());
        let x = s.intersect(&r);
        assert_eq!(-6.0, x[0].t);
        assert_eq!(-4.0, x[1].t);
    }

    #[test]
    fn test_transformed_intersect() {
        let r = Ray::new(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let m = Matrix::scaling(2.0, 2.0, 2.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere {
            id: Uuid::new_v4(),
            inv_transformation_transposed: m.invert().transpose(),
            inv_transformation: m.invert(),
            transformation: m,
            material: Material::default()
        });
        let x = s.intersect(&r);
        assert_eq!(3.0, x[0].t);
        assert_eq!(7.0, x[1].t);

        let r = Ray::new(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let m = Matrix::translation(5.0, 0.0, 0.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere {
            id: Uuid::new_v4(),
            inv_transformation_transposed: m.invert().transpose(),
            inv_transformation: m.invert(),
            transformation: m,
            material: Material::default()
        });
        let x = s.intersect(&r);
        assert_eq!(0, x.len());
    }


    #[test]
    fn test_sphere_normal_at() {
        let m = Matrix::translation(0.0, 1.0, 0.0);
        let s: Box<dyn Shape + Sync> = Box::new(Sphere {
            id: Uuid::new_v4(),
            inv_transformation_transposed: m.invert().transpose(),
            inv_transformation: m.invert(),
            transformation: m,
            material: Material::default()
        });
        let p = Tuple::point(0.0, 1.70711, -0.70711);

        let expected = Tuple::vector(0.0, 0.70711, -0.70711);
        assert_eq!(s.normal_at(&p), expected);


        let m = Matrix::scaling(1.0, 0.5, 1.0)
            .multiply_matrix(&Matrix::rotation_z(144.0));
        let s: Box<dyn Shape + Sync> = Box::new(Sphere {
            id: Uuid::new_v4(),
            inv_transformation_transposed: m.invert().transpose(),
            inv_transformation: m.invert(),
            transformation: m,
            material: Material::default()
        });
        let p = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);

        let expected = Tuple::vector(0.0, 0.97014, -0.24254);
        assert_eq!(s.normal_at(&p), expected);
    }

    #[test]
    fn test_reflect() {
        let a = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let expected = Tuple::vector(1.0, 1.0, 0.0);
        assert_eq!(expected, reflect(&a, &n));

        let a = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let expected = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(expected, reflect(&a, &n));
    }

}
