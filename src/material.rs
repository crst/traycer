use crate::color::Color;
use crate::patterns::pattern::Pattern;


#[derive(Clone)]
pub struct Material {
    pub color: Option<Color>,
    pub pattern: Option<Box<dyn Pattern + Sync>>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new(
        color: Option<Color>,
        pat: Option<Box<dyn Pattern + Sync>>,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64
    ) -> Material {
        Material {
            color: color,
            pattern: pat,
            ambient: ambient,
            diffuse: diffuse,
            specular: specular,
            shininess: shininess,
            reflective: reflective,
            transparency: transparency,
            refractive_index: refractive_index,
        }
    }

    pub fn default() -> Material {
        Material::new(
            Some(Color::white()),
            None,
            0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0
        )
    }

}
