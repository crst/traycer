use crate::color::Color;
use crate::linalg::tuple::Tuple;
use crate::ray;
use crate::shapes::{shape::Shape};


#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    #[allow(dead_code)]
    pub fn new() -> PointLight {
        PointLight {
            intensity: Color::white(),
            position: Tuple::point(0.0, 0.0, 0.0),
        }
    }

    pub fn lighting<'a>(&self, object: &'a (dyn Shape + Sync), pos: &Tuple, is_shadowed: bool, eyev: &Tuple, normv: &Tuple) -> Color {
        // Determine base color depending on object color/pattern and
        // ambient light.
        let material = object.get_material();
        let mut color = Color::black();
        if let Some(c) = material.color {
            color = c;
        } else if let Some(p) = material.pattern.as_ref() {
            color = p.color_at(object, pos);
        }
        let effective_color = color.multiply_color(&self.intensity);
        color = effective_color.multiply(material.ambient);


        // Check if light is on the side of the surface.
        let lightv = self.position.subtract(&pos).normalize();
        let light_dot_normal = lightv.dot(&normv);
        if light_dot_normal >= 0.0 && !is_shadowed {
            // If so, and the surface is not shadowed, compute diffuse
            // and specular light as well.
            let diffuse = effective_color.multiply(material.diffuse)
                .multiply(light_dot_normal);

            let reflectv = ray::reflect(&lightv.negate(), &normv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            let mut specular = Color::black();
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = self.intensity.multiply(material.specular).multiply(factor);
            }
            color = color.add(&diffuse).add(&specular);
        }

        color
    }
}


#[cfg(test)]
mod tests {
    use super::PointLight;
    use crate::color::Color;
    use crate::linalg::tuple::Tuple;
    use crate::shapes::sphere::Sphere;

    #[test]
    fn test_lighting() {
        let pos = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::white(),
            position: Tuple::point(0.0, 0.0, -10.0),
        };
        let s = Sphere::default();
        let result = light.lighting(&s, &pos, false, &eyev, &normv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));

        let eyev = Tuple::vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Tuple::point(0.0, 0.0, -10.0),
        };
        let s = Sphere::default();
        let result = light.lighting(&s, &pos, false, &eyev, &normv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Tuple::point(0.0, 10.0, -10.0),
        };
        let s = Sphere::default();
        let result = light.lighting(&s, &pos, false, &eyev, &normv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));

        let eyev = Tuple::vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Tuple::point(0.0, 10.0, -10.0),
        };
        let s = Sphere::default();
        let result = light.lighting(&s, &pos, false, &eyev, &normv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Tuple::point(0.0, 0.0, 10.0),
        };
        let s = Sphere::default();
        let result = light.lighting(&s, &pos, false, &eyev, &normv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
