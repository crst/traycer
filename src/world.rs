use crate::color::Color;
use crate::light::PointLight;
use crate::linalg::tuple::Tuple;
use crate::ray::{Ray, reflect};
use crate::shapes::shape::Shape;
use crate::utils::EPSILON;

use uuid::Uuid;


static MAX_REC_DEPTH: u8 = 10;


pub struct World {
    pub objects: Vec<Box<dyn Shape + Sync>>,
    pub lights: Vec<PointLight>,
}

impl World {
    #[allow(dead_code)]
    pub fn default() -> World {
        let light = PointLight {
            position: Tuple::point(-10.0, 10.0, -10.0),
            intensity: Color::white(),
        };

        World {
            objects: vec![],
            lights: vec![light],
        }
    }

    pub fn intersect<'a>(&'a self, ray: &'a Ray) -> Vec<Intersection<'a>> {
        let mut result = Vec::new();

        for obj in self.objects.iter() {
            let mut intersections = obj.intersect(&ray);
            result.append(&mut intersections);
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        compute_intersection_data(&mut result, ray);
        compute_n1n2(&mut result);

        result
    }

    pub fn shade_hit(&self, int: Intersection) -> Color {
        let mut result = Color::black();

        for light in self.lights.iter() {
            let is_shadowed = self.is_shadowed(&int.over_point, &light);
            result = result.add(
                &light.lighting(
                    int.object,
                    &int.point,
                    is_shadowed,
                    &int.eyev,
                    &int.normalv
                )
            );
        }

        result
    }

    pub fn color_at(&self, ray: &Ray, rec_depth: u8) -> Color {
        // Find the first object the ray hits.
        let intersections = self.intersect(ray);
        let hit = hit(&intersections);

        // Figure out the color at this pixel.
        let color = match hit {
            Some(int) => self.shade_hit(int),
            None => Color::black(),
        };

        // If the object hit is reflective, spawn another ray in the
        // direction of the reflect vector.
        let reflect_color = match hit {
            Some(int) => self.reflected_color_at(&int, rec_depth),
            None => Color::black(),
        };

        // If the object his is transparent, spawn another refracted
        // ray.
        let refract_color = match hit {
            Some(int) => self.refracted_color_at(&int, rec_depth),
            None => Color::black(),
        };

        // Merge influence of reflections and refractions.
        let added_color = match hit {
            Some(int) => {
                let mat = int.object.get_material();
                if mat.reflective > 0.0 && mat.transparency > 0.0 {
                    let reflectance = schlick(&int);
                    reflect_color.multiply(reflectance)
                        .add(&refract_color.multiply(1.0 - reflectance))
                } else {
                    reflect_color.add(&refract_color)
                }
            },
            None => reflect_color.add(&refract_color),
        };

        // Return final value.
        let result = color.add(&added_color);
        result
    }

    pub fn reflected_color_at(&self, hit: &Intersection, rec_depth: u8) -> Color {
        let mut result = Color::black();
        let factor = hit.object.get_material().reflective;
        if factor > 0.0 && rec_depth < MAX_REC_DEPTH {
            let reflect_ray = Ray {
                origin: hit.over_point,
                direction: hit.reflectv,
            };
            result = self.color_at(&reflect_ray, rec_depth + 1).multiply(factor);
        }
        result
    }

    pub fn refracted_color_at(&self, hit: &Intersection, rec_depth: u8) -> Color {
        let mut result = Color::black();

        if hit.object.get_material().transparency > 0.0 && rec_depth < MAX_REC_DEPTH {
            let n_ratio = hit.n1 / hit.n2;
            let cos_i = hit.eyev.dot(&hit.normalv);
            let sin2_t = (n_ratio * n_ratio) * (1.0 - (cos_i * cos_i));
            if sin2_t < 1.0 {
                // No internal reflection
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction = hit.normalv
                    .multiply(n_ratio * cos_i - cos_t)
                    .subtract(&hit.eyev.multiply(n_ratio));
                let refract_ray = Ray {
                    origin: hit.under_point,
                    direction: direction,
                };
                let factor = hit.object.get_material().transparency;
                result = self.color_at(&refract_ray, rec_depth + 1).multiply(factor);
            }
        }

        result
    }

    pub fn is_shadowed(&self, point: &Tuple, light: &PointLight) -> bool {
        let v = light.position.subtract(point);
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray {
            origin: *point,
            direction: direction,
        };
        let intersections = self.intersect(&ray);
        let h = hit(&intersections);
        match h {
            Some(x) => x.t < distance,
            None => false
        }
    }
}


pub fn compute_intersection_data(result: &mut Vec<Intersection>, r: &Ray) -> () {
    for i in result.iter_mut() {
        i.point = r.position(i.t);
        i.normalv = i.object.normal_at(&i.point);
        i.eyev = r.direction.negate();
        i.inside = i.normalv.dot(&i.eyev) < 0.0;
        if i.inside {
            i.normalv = i.normalv.negate();
        }
        i.over_point = i.point.add(&i.normalv.multiply(EPSILON));
        i.under_point = i.point.subtract(&i.normalv.multiply(EPSILON));
        i.reflectv = reflect(&r.direction, &i.normalv);
    }
}

pub fn compute_n1n2(result: &mut Vec<Intersection>) -> () {
    let mut containers: Vec<&Uuid> = Vec::new();
    for i in 0..(result.len()) {
        if containers.len() == 0 {
            result[i].n1 = 1.0;
        } else {
            let pos = result.iter().position(|&x| x.object.get_id() == containers[containers.len() - 1]).unwrap();
            result[i].n1 = result[pos].object.get_material().refractive_index;
        }

        if containers.contains(&result[i].object.get_id()) {
            let pos = containers.iter().position(|&x| x == result[i].object.get_id()).unwrap();
            containers.remove(pos);
        } else {
            containers.push(result[i].object.get_id());
        }

        if containers.len() == 0 {
            result[i].n2 = 1.0;
        } else {
            let pos = result.iter().position(|&x| x.object.get_id() == containers[containers.len() - 1]).unwrap();
            result[i].n2 = result[pos].object.get_material().refractive_index;
        }
    }
}

pub fn schlick(hit: &Intersection) -> f64 {
    let mut cos = hit.eyev.dot(&hit.normalv);
    if hit.n1 > hit.n2 {
        let n = hit.n1 / hit.n2;
        let sin2_t = (n * n) * (1.0 - (cos * cos));
        if sin2_t > 1.0 {
            return 1.0;
        }
        cos = (1.0 - sin2_t).sqrt();
    }

    let r0 = ((hit.n1 - hit.n2) / (hit.n1 + hit.n2)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}


#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a (dyn Shape + Sync),
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &(dyn Shape + Sync)) -> Intersection {
        let p = Tuple::point(0.0, 0.0, 0.0);
        Intersection {
            t: t,
            object: object,
            point: p,
            over_point: p,
            under_point: p,
            eyev: p,
            normalv: p,
            reflectv: p,
            inside: false,
            n1: 1.0,
            n2: 1.0
        }
    }
}

pub fn hit<'a>(intersections: &'a Vec<Intersection>) -> Option<Intersection<'a>> {
    // Find the first visible intersection. Assuming that
    // intersections are already sorted.
    for &ix in intersections.iter() {
        if ix.t >= 0.0 {
            return Some(ix);
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::test::TestPattern;
    use crate::material::Material;
    use crate::linalg::matrix::Matrix;
    use crate::shapes::{plane::Plane, sphere::Sphere, sphere::get_default_spheres, sphere::get_glass_sphere};
    use crate::utils::equal;

    #[test]
    fn test_default() {
        let mut w = World::default();
        let ds = get_default_spheres();
        w.objects = vec![Box::new(ds[0].clone()), Box::new(ds[1].clone())];

        let r = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let i = w.intersect(&r);

        assert_eq!(i[0].t, 4.0);
        assert_eq!(i[1].t, 4.5);
        assert_eq!(i[2].t, 5.5);
        assert_eq!(i[3].t, 6.0);
    }

    #[test]
    fn test_intersection() {
        let mut w = World::default();
        let ds = get_default_spheres();
        w.objects = vec![Box::new(ds[0].clone()), Box::new(ds[1].clone())];

        let r = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let x = w.intersect(&r);
        assert_eq!(x[0].t, 4.0);
        assert_eq!(x[0].point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(x[0].eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(x[0].normalv, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(x[0].inside, false);

        let r = Ray {
            origin: Tuple::point(0.0, 0.0, 0.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let x = w.intersect(&r);
        assert_eq!(x[3].t, 1.0);
        assert_eq!(x[3].point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(x[3].eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(x[3].normalv, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(x[3].inside, true);
    }

    #[test]
    fn test_color_at() {
        let mut w = World::default();
        let ds = get_default_spheres();
        w.objects = vec![Box::new(ds[0].clone()), Box::new(ds[1].clone())];

        let r = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let c = w.color_at(&r, 0);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));

        w.lights = vec![PointLight {
            position: Tuple::point(0.0, 0.25, 0.0),
            intensity: Color::white(),
        }];
        let r = Ray {
            origin: Tuple::point(0.0, 0.0, 0.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let c = w.color_at(&r, 0);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));


        let mut w = World::default();
        let ds = get_default_spheres();
        w.objects = vec![Box::new(ds[0].clone()), Box::new(ds[1].clone())];
        let r = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };
        let c = w.color_at(&r, 0);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn test_n1n2() {
        let a = Sphere::new(
            Matrix::scaling(2.0, 2.0, 2.0),
            Material::new(
                Some(Color::white()),
                None,
                0.1, 0.9, 0.3, 5.0, 0.0, 1.0, 1.5
            ),
        );

        let b = Sphere::new(
            Matrix::translation(0.0, 0.0, -0.25),
            Material::new(
                Some(Color::white()),
                None,
                0.1, 0.9, 0.3, 5.0, 0.0, 1.0, 2.0
            ),
        );

        let c = Sphere::new(
            Matrix::translation(0.0, 0.0, 0.25),
            Material::new(
                Some(Color::white()),
                None,
                0.1, 0.9, 0.3, 5.0, 0.0, 1.0, 2.5
            ),
        );

        let light = PointLight {
            position: Tuple::point(-0.0, 7.5, -5.0),
            intensity: Color::white(),
        };

        let world = World {
            objects: vec![
                Box::new(a),
                Box::new(b),
                Box::new(c),
            ],
            lights: vec![light],
        };
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -4.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };

        let intersections = world.intersect(&ray);
        let mut n_values = vec![];
        for int in intersections.iter() {
            n_values.push(vec![int.t, int.n1, int.n2]);
        }
        let expected_values = vec![
            vec![2.00, 1.0, 1.5],
            vec![2.75, 1.5, 2.0],
            vec![3.25, 2.0, 2.5],
            vec![4.75, 2.5, 2.5],
            vec![5.25, 2.5, 1.5],
            vec![6.00, 1.5, 1.0],
        ];
        assert_eq!(expected_values, n_values);
    }

    #[test]
    fn test_under_point() {
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };

        let sphere = Sphere::new(
            Matrix::translation(0.0, 0.0, 1.0),
            Material::new(
                Some(Color::white()),
                None,
                0.1, 0.9, 0.3, 5.0, 0.0, 1.0, 1.5
            ),
        );

        let mut intersections = sphere.intersect(&ray);
        compute_intersection_data(&mut intersections, &ray);
        let int = intersections[0];
        assert_eq!(int.t, 5.0);
        assert!(int.under_point.z > EPSILON / 2.0);
        assert!(int.point.z < int.under_point.z);
    }

    #[test]
    fn test_refracted_color_opaque() {
        let mut world = World::default();
        let ds = get_default_spheres();
        world.objects = vec![Box::new(ds[0].clone()), Box::new(ds[1].clone())];

        let shape = Box::new(ds[0].clone());
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let intersections = shape.intersect(&ray);
        for int in intersections.iter() {
            let color = world.refracted_color_at(&int, 0);
            assert_eq!(color, Color::black());
        }
    }

    #[test]
    fn test_refracted_color_max_depth() {
        let mut world = World::default();

        let s1 = Sphere::new(
            Matrix::identity(4),
            Material::new(
                Some(Color::new(0.8, 1.0, 0.6)),
                None,
                0.1, 0.7, 0.2, 200.0, 0.0, 1.0, 1.5
            ),
        );
        let s2 = Sphere::new(
            Matrix::scaling(0.5, 0.5, 0.5),
            Material::default(),
        );
        world.objects = vec![Box::new(s1.clone()), Box::new(s2.clone())];

        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let intersections = s1.intersect(&ray);
        for int in intersections.iter() {
            let color = world.refracted_color_at(&int, 10);
            assert_eq!(color, Color::black());
        }
    }

    #[test]
    fn test_refracted_color_internal_reflection() {
        let mut world = World::default();

        let s1 = Sphere::new(
            Matrix::identity(4),
            Material::new(
                Some(Color::new(0.8, 1.0, 0.6)),
                None,
                0.1, 0.7, 0.2, 200.0, 0.0, 1.0, 1.5
            ),
        );
        let s2 = Sphere::new(
            Matrix::scaling(0.5, 0.5, 0.5),
            Material::default(),
        );
        world.objects = vec![Box::new(s1), Box::new(s2)];

        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, 2.0f64.sqrt() / 2.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };
        let intersections = world.intersect(&ray);
        let color = world.refracted_color_at(&intersections[1], 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn test_refracted_color_for_real() {
        let mut world = World::default();

        let pattern = TestPattern::new(Matrix::identity(4));
        let s1 = Sphere::new(
            Matrix::identity(4),
            Material::new(
                None,
                Some(Box::new(pattern)),
                1.0, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
            ),
        );
        let s2 = Sphere::new(
            Matrix::scaling(0.5, 0.5, 0.5),
            Material::new(
                Some(Color::white()),
                None,
                0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 1.5
            )
        );

        world.objects = vec![Box::new(s1), Box::new(s2)];

        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, 0.1),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };
        let intersections = world.intersect(&ray);
        let n = 2;

        let int = intersections[n];
        let color = world.refracted_color_at(&int, 0);
        assert_eq!(color, Color::new(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn test_refractions() {
        let mut world = World::default();

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

        let mut floor_material = Material::default();
        floor_material.transparency = 0.5;
        floor_material.refractive_index = 1.5;
        let floor = Plane::new(
            Matrix::translation(0.0, -1.0, 0.0),
            floor_material
        );

        let mut ball_material = Material::default();
        ball_material.color = Some(Color::new(1.0, 0.0, 0.0));
        ball_material.ambient = 0.5;
        let ball = Sphere::new(
            Matrix::translation(0.0, -3.5, -0.5),
            ball_material
        );

        world.objects = vec![Box::new(s1), Box::new(s2), Box::new(floor), Box::new(ball)];

        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -3.0),
            direction: Tuple::vector(0.0, -2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0),
        };
        let color = world.color_at(&ray, 0);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn test_schlick_internal_reflection() {
        let glas: Box<dyn Shape + Sync> = Box::new(get_glass_sphere());
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, 2.0f64.sqrt() / 2.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };
        let mut intersections = &mut glas.intersect(&ray);
        compute_intersection_data(&mut intersections, &ray);
        compute_n1n2(&mut intersections);

        let schlick = schlick(&intersections[1]);
        assert_eq!(schlick, 1.0);
    }

    #[test]
    fn test_schlick_perpendicular() {
        let glas: Box<dyn Shape + Sync> = Box::new(get_glass_sphere());
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, 0.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };
        let mut intersections = &mut glas.intersect(&ray);
        compute_n1n2(&mut intersections);
        let schlick = schlick(&intersections[1]);
        assert!(equal(schlick, 0.04));
    }

    #[test]
    fn test_schlick_small_angle() {
        let glas: Box<dyn Shape + Sync> = Box::new(get_glass_sphere());
        let ray = Ray {
            origin: Tuple::point(0.0, 0.99, -2.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let mut intersections = &mut glas.intersect(&ray);
        compute_intersection_data(&mut intersections, &ray);
        compute_n1n2(&mut intersections);
        let schlick = schlick(&intersections[0]);
        assert!(equal(schlick, 0.48873));
    }

    #[test]
    fn test_schlick_both() {
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -3.0),
            direction: Tuple::vector(0.0, -2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0),
        };

        let mut floor_material = Material::default();
        floor_material.reflective = 0.5;
        floor_material.transparency = 0.5;
        floor_material.refractive_index = 1.5;
        let floor = Plane {
            id: Uuid::new_v4(),
            transformation: Matrix::translation(0.0, -1.0, 0.0),
            inv_transformation: Matrix::translation(0.0, -1.0, 0.0).invert(),
            material: floor_material,
        };

        let mut ball_material = Material::default();
        ball_material.color = Some(Color::new(1.0, 0.0, 0.0));
        ball_material.ambient = 0.5;
        let ball = Sphere {
            id: Uuid::new_v4(),
            transformation: Matrix::translation(0.0, -3.5, -0.5),
            inv_transformation: Matrix::translation(0.0, -3.5, -0.5).invert(),
            inv_transformation_transposed: Matrix::translation(0.0, -3.5, -0.5).invert().transpose(),
            material: ball_material,
        };

        let mut world = World::default();
        let ds = get_default_spheres();
        world.objects = vec![
            Box::new(ds[0].clone()),
            Box::new(ds[1].clone()),
            Box::new(floor.clone()),
            Box::new(ball.clone())
        ];

        let color = world.color_at(&ray, 0);
        let expected = Color::new(0.93391, 0.69643, 0.69243);
        assert_eq!(color, expected);
    }
}
