use crate::linalg::matrix::Matrix;
use crate::linalg::tuple::Tuple;
use crate::ray::Ray;
use crate::utils::P0;
use crate::world::World;

use rayon::prelude::*;


pub struct Camera {
    pub hsize: i64,
    pub vsize: i64,
    pub field_of_view: f64,
    pub transformation: Matrix,
    pub inv_transformation: Matrix,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
    origin: Tuple,
}

impl Camera {
    pub fn new(hsize: i64, vsize: i64, field_of_view: f64, from: &Tuple, to: &Tuple, up: &Tuple) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect: f64 = hsize as f64 / vsize as f64;

        let mut half_width = half_view * aspect;
        let mut half_height = half_view;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }

        let transformation = view_transform(from, to, up);
        let inv_transformation = transformation.invert();
        let origin = inv_transformation.multiply_tuple(&P0);

        Camera {
            hsize: hsize,
            vsize: vsize,
            field_of_view: field_of_view,
            inv_transformation: inv_transformation,
            transformation: transformation,
            half_width: half_width,
            half_height: half_height,
            pixel_size: (half_width * 2.0) / (hsize as f64),
            origin: origin,
        }
    }

    pub fn ray_for_pixel(&self, x: i64, y: i64) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let w = Tuple::point(world_x, world_y, -1.0);
        let pixel = self.inv_transformation.multiply_tuple(&w);
        let direction = pixel.subtract(&self.origin).normalize();

        Ray {
            origin: self.origin,
            direction: direction,
        }
    }

    pub fn render(&self, world: &World) -> Vec<Vec<[u8; 3]>> {
        let mut image = Vec::new();
        for y in 0..self.vsize {
            let row: Vec<[u8; 3]> = (0..self.hsize).into_par_iter()
                .map(|x| {
                    let ray = self.ray_for_pixel(x, y);
                    world.color_at(&ray, 0).as_rgb()
                }).collect();
            image.push(row);
        }
        image
    }
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = to.subtract(&from).normalize();
    let upn = up.normalize();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let result = Matrix::from_values(
        &vec![
            vec![left.x, left.y, left.z, 0.0],
            vec![true_up.x, true_up.y, true_up.z, 0.0],
            vec![-forward.x, -forward.y, -forward.z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    );
    result.multiply_matrix(&Matrix::translation(-from.x, -from.y, -from.z))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::equal;

    #[test]
    fn test_pixel_size() {
        let cam = Camera::new(
            200,
            125,
            std::f64::consts::PI / 2.0,
            &Tuple::point(0.0, 0.0, 0.0),
            &Tuple::point(0.0, 0.0, 0.0),
            &Tuple::point(0.0, 0.0, 0.0),
        );
        assert_eq!(true, equal(cam.pixel_size, 0.01));

        let cam = Camera::new(
            125,
            200,
            std::f64::consts::PI / 2.0,
            &Tuple::point(0.0, 0.0, 0.0),
            &Tuple::point(0.0, 0.0, 0.0),
            &Tuple::point(0.0, 0.0, 0.0),
        );
        assert_eq!(true, equal(cam.pixel_size, 0.01));
    }

    #[test]
    fn test_view_transform() {
        let from = Tuple::vector(1.0, 3.0, 2.0);
        let to = Tuple::vector(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);
        let m = view_transform(&from, &to, &up);

        let expected = Matrix::from_values(
            &vec![
                vec![-0.50709, 0.50709,  0.67612, -2.36643],
                vec![ 0.76771, 0.60609,  0.12121, -2.82842],
                vec![-0.35857, 0.59761, -0.71714,  0.00000],
                vec![ 0.00000, 0.00000,  0.00000,  1.00000],
            ]
        );
        assert_eq!(m, expected);
    }

}
