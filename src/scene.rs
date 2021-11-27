use std::collections::HashMap;
use std::fs;

use crate::camera::Camera;
use crate::color::Color;
use crate::patterns::{
    pattern::Pattern,
    checkers::CheckersPattern,
    gradient::GradientPattern,
    ring::RingPattern,
    stripe::StripePattern
};
use crate::light::PointLight;
use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::material::Material;
use crate::shapes::{shape::Shape, sphere::Sphere, plane::Plane, cube::Cube, cylinder::Cylinder};
use crate::world::World;

use serde::{Deserialize};


#[derive(Clone, Debug, Deserialize)]
pub struct SceneCamera {
    pub width: i64,
    pub height: i64,
    pub field_of_view: f64,
    pub from: Vec<f64>,
    pub to: Vec<f64>,
    pub up: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SceneLight {
    pub position: Vec<f64>,
    pub intensity: Vec<f64>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScenePattern {
    pub pattern: String,
    pub a: Option<Vec<f64>>,
    pub b: Option<Vec<f64>>,
    pub transformations: Option<Vec<SceneTransformation>>
}

#[derive(Clone, Debug, Deserialize)]
pub struct SceneMaterial {
    pub color: Option<Vec<f64>>,
    pub pattern: Option<String>,
    pub ambient: Option<f64>,
    pub diffuse: Option<f64>,
    pub specular: Option<f64>,
    pub shininess: Option<f64>,
    pub reflective: Option<f64>,
    pub transparency: Option<f64>,
    pub refractive_index: Option<f64>
}

#[derive(Clone, Debug, Deserialize)]
pub struct SceneTransformation {
    pub defined_transformation: Option<String>,
    pub transformation: Option<String>,
    pub parameters: Option<Vec<f64>>
}

#[derive(Clone, Debug, Deserialize)]
pub struct SceneObject {
    pub shape: String,

    // Cylinder specific parameters
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
    pub closed: Option<bool>,

    pub material: Option<String>,
    pub color: Option<Vec<f64>>,
    pub pattern: Option<String>,
    pub ambient: Option<f64>,
    pub diffuse: Option<f64>,
    pub specular: Option<f64>,
    pub reflective: Option<f64>,
    pub shininess: Option<f64>,
    pub transparency: Option<f64>,
    pub refractive_index: Option<f64>,
    pub transformations: Option<Vec<SceneTransformation>>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Scene {
    pub camera: SceneCamera,
    pub lights: Vec<SceneLight>,
    pub patterns: Option<HashMap<String, ScenePattern>>,
    pub materials: Option<HashMap<String, SceneMaterial>>,
    pub transformations: Option<HashMap<String, Vec<SceneTransformation>>>,
    pub objects: Option<HashMap<String, SceneObject>>
}


pub fn parse_scene(source: &String) -> Scene {
    let contents = fs::read_to_string(source)
        .expect("Could not read the scene file!");
    let result: Scene = serde_json::from_str(&contents).unwrap();
    result
}

pub fn make_camera(scene: &Scene) -> Camera {
    Camera::new(
        scene.camera.width,
        scene.camera.height,
        scene.camera.field_of_view,
        &Tuple::point(
            scene.camera.from[0],
            scene.camera.from[1],
            scene.camera.from[2],
        ),
        &Tuple::point(
            scene.camera.to[0],
            scene.camera.to[1],
            scene.camera.to[2],
        ),
        &Tuple::point(
            scene.camera.up[0],
            scene.camera.up[1],
            scene.camera.up[2],
        )
    )
}

pub fn make_transformation(transformations: &Vec<SceneTransformation>, map: &HashMap<String, Matrix>) -> Matrix {
    let mut result = Matrix::identity(4);

    for cur in transformations.iter() {

        let next = match &cur.defined_transformation {
            Some(key) => map[key].clone(),
            None => {
                let p = cur.parameters.as_ref().unwrap();
                match cur.transformation.as_ref().unwrap().as_str() {
                    "translate" => Matrix::translation(p[0], p[1], p[2]),
                    "scale" => Matrix::scaling(p[0], p[1], p[2]),
                    "rotate-x" => Matrix::rotation_x(p[0]),
                    "rotate-y" => Matrix::rotation_y(p[0]),
                    "rotate-z" => Matrix::rotation_z(p[0]),
                    "shear" => Matrix::shearing(p[0], p[1], p[2], p[3], p[4], p[5]),
                    _ => panic!("Undefined transformation: {:?}!", cur.transformation)
                }
            }
        };

        result = result.multiply_matrix(&next);
    }

    result
}

pub fn make_pattern(pat: &ScenePattern, map: &HashMap<String, Matrix>) -> Box<dyn Pattern + Sync> {
    let transformation = make_transformation(
        &pat.transformations.as_ref().unwrap(),
        map
    );
    let a = pat.a.as_ref().unwrap();
    let b = pat.b.as_ref().unwrap();
    let result: Box<dyn Pattern + Sync> = match pat.pattern.as_str() {
        "checkers" => Box::new(CheckersPattern::new(
            Color::from_vec(a),
            Color::from_vec(b),
            transformation
        )),
        "ring" => Box::new(RingPattern::new(
            Color::from_vec(a),
            Color::from_vec(b),
            transformation
        )),
        "stripe" => Box::new(StripePattern::new(
            Color::from_vec(a),
            Color::from_vec(b),
            transformation
        )),
        "gradient" => Box::new(GradientPattern::new(
            Color::from_vec(a),
            Color::from_vec(b),
            transformation
        )),
        _ => panic!("Undefined pattern: {:?}", pat.pattern)
    };
    result
}

pub fn make_world(scene: &Scene) -> World {
    let mut lights: Vec<PointLight> = vec![];

    for l in scene.lights.iter() {
        let light = PointLight {
            position: Tuple::point_from_vec(&l.position),
            intensity: Color::from_vec(&l.intensity),
        };
        lights.push(light);
    }

    let mut transformations: HashMap<String, Matrix> = HashMap::new();
    for (name, value) in scene.transformations.as_ref().unwrap_or(&HashMap::new()).iter() {
        let tmp = make_transformation(&value, &transformations);
        transformations.insert(name.clone(), tmp);
    }

    let mut patterns: HashMap<String, Box<dyn Pattern + Sync>> = HashMap::new();
    for (name, pattern) in scene.patterns.as_ref().unwrap_or(&HashMap::new()).iter() {
        let tmp = make_pattern(&pattern, &transformations);
        patterns.insert(name.clone(), tmp);
    }

    let mut materials: HashMap<String, Material> = HashMap::new();
    for (name, material) in scene.materials.as_ref().unwrap_or(&HashMap::new()).iter() {
        let mut tmp = Material::default();

        if let Some(v) = material.color.as_ref() {
            tmp.color = Some(Color::new(v[0], v[1], v[2]));
        } else {
            if let Some(v) = material.pattern.as_ref() {
                // TODO: should allow directly defined patterns as well.
                tmp.color = None;
                tmp.pattern = Some(patterns[v].clone());
            }
        }
        tmp.ambient = material.ambient.unwrap_or(tmp.ambient);
        tmp.diffuse = material.diffuse.unwrap_or(tmp.diffuse);
        tmp.specular = material.specular.unwrap_or(tmp.specular);
        tmp.shininess = material.shininess.unwrap_or(tmp.shininess);
        tmp.reflective = material.reflective.unwrap_or(tmp.reflective);
        tmp.transparency = material.transparency.unwrap_or(tmp.transparency);
        tmp.refractive_index = material.refractive_index.unwrap_or(tmp.refractive_index);

        materials.insert(name.clone(), tmp);
    }


    let mut objects: Vec<Box<dyn Shape + Sync>> = vec![];
    for (_, value) in scene.objects.as_ref().unwrap_or(&HashMap::new()).iter() {
        let mut m = match &value.material {
            Some(key) => if let Some(k) = materials.get(key) {
                k.clone()
            } else {
                panic!("Undefined material: {:?}", key);
            },
            None => Material::default(),
        };
        // TODO: abstract color/pattern assignment.
        if let Some(v) = value.color.as_ref() {
            m.color = Some(Color::new(v[0], v[1], v[2]));
        } else {
            // TODO: should allow directly defined patterns as well.
            if let Some(v) = value.pattern.as_ref() {
                m.color = None;
                m.pattern = Some(patterns[v].clone());
            }
        }

        m.ambient = value.ambient.unwrap_or(m.ambient);
        m.diffuse = value.diffuse.unwrap_or(m.diffuse);
        m.specular = value.specular.unwrap_or(m.diffuse);
        m.shininess = value.shininess.unwrap_or(m.shininess);
        m.reflective = value.reflective.unwrap_or(m.reflective);
        m.transparency = value.transparency.unwrap_or(m.transparency);
        m.refractive_index = value.refractive_index.unwrap_or(m.refractive_index);


        let mut object: Box<dyn Shape + Sync> = match value.shape.as_ref() {
            "sphere" => Box::new(Sphere::default()),
            "plane" => Box::new(Plane::default()),
            "cube" => Box::new(Cube::default()),
            "cylinder" => {
                let mut c = Box::new(Cylinder::default());
                c.min_y = value.min_y.unwrap_or(c.min_y);
                c.max_y = value.max_y.unwrap_or(c.max_y);
                c.closed = value.closed.unwrap_or(c.closed);
                c
            },
            _ => panic!("Undefined shape: {:?}!", value.shape)
        };

        object.set_material(m);
        let tmp = make_transformation(
            &value.transformations.as_ref().unwrap_or(&vec![]),
            &transformations
        );
        object.set_transformation(tmp);

        objects.push(object);
    }

    let world = World {
        objects: objects,
        lights: lights,
    };

    world
}
