use std::vec::Vec;

use crate::utils::{equal};
use crate::linalg::tuple::Tuple;

use rand::prelude::*;


#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows: rows,
            cols: cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize, min_val: f64, max_val: f64) -> Matrix {
        let mut rng = rand::thread_rng();
        let mut m = Vec::with_capacity(rows);
        let range = max_val - min_val;
        for _ in 0..rows {
            let mut n = Vec::with_capacity(cols);
            for _ in 0..cols {
                n.push((rng.gen::<f64>() * range) + min_val);
            }
            m.push(n);
        }
        Matrix {
            rows: rows,
            cols: cols,
            data: m,
        }
    }

    pub fn from_values(values: &Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: values.len(),
            cols: values[0].len(),
            data: values.clone(),
        }
    }

    pub fn identity(n: usize) -> Matrix {
        let mut result = Matrix::new(n, n);
        for i in 0..n {
            result.data[i][i] = 1.0;
        }
        result
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    pub fn multiply_matrix(&self, other: &Matrix) -> Self {
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..(self.rows) {
            for j in 0..(other.cols) {
                for k in 0..(self.cols) {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn multiply_tuple(&self, other: &Tuple) -> Tuple {
        let mut r = [0.0, 0.0, 0.0, 0.0];
        for i in 0..(self.rows) {
            r[i] += self.data[i][0] * other.x;
            r[i] += self.data[i][1] * other.y;
            r[i] += self.data[i][2] * other.z;
            r[i] += self.data[i][3] * other.w;
        }
        Tuple::new(r[0], r[1], r[2], r[3])
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Self {
        let (new_rows, new_cols) = (self.rows - 1, self.cols - 1);
        let mut data = Vec::with_capacity(new_rows);

        for (i, row_data) in self.data.iter().enumerate() {
            if i != row {
                let mut tmp = Vec::with_capacity(new_cols);
                for (j, &value) in row_data.iter().enumerate() {
                    if j != column {
                        tmp.push(value);
                    }
                }
                data.push(tmp);
            }
        }

        Matrix {
            rows: new_rows,
            cols: new_cols,
            data: data,
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.rows != self.cols {
            panic!("Trying to compute the determinant of a non-square ({:?}x{:?}) matrix!", self.rows, self.cols);
        }

        let n = self.rows;
        if n == 1 {
            return self.data[0][0]
        }
        if n == 2 {
            return self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
        }

        let mut result = 0.0;
        for i in 0..n {
            let sign = match i % 2 {
                0 => 1.0,
                _ => -1.0
            };
            result += sign * self.data[0][i] * self.submatrix(0, i).determinant();
        }
        result
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let sign = match (row + column) % 2 {
            0 => 1.0,
            _ => -1.0
        };
        sign * self.submatrix(row, column).determinant()
    }

    pub fn is_invertible(&self) -> bool {
        !equal(self.determinant(), 0.0)
    }

    pub fn invert(&self) -> Self {
        if !self.is_invertible() {
            panic!("Calling invert() on a non-invertible matrix!");
        }

        let det = self.determinant();
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..(self.rows) {
            for j in 0..(self.cols) {
                result.data[j][i] = self.cofactor(i, j) / det;
            }
        }
        result
    }

}


impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if !equal(self.data[i as usize][j as usize], other.data[i as usize][j as usize]) {
                    return false;
                }
            }
        }
        true
    }
}
impl Eq for Matrix {}


impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.data[0][3] = x;
        m.data[1][3] = y;
        m.data[2][3] = z;
        m
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.data[0][0] = x;
        m.data[1][1] = y;
        m.data[2][2] = z;
        m
    }

    pub fn rotation_x(deg: f64) -> Matrix {
        let rad = (deg * std::f64::consts::PI) / 180.0;
        let mut m = Matrix::identity(4);
        m.data[1][1] = rad.cos();
        m.data[1][2] = -rad.sin();
        m.data[2][1] = rad.sin();
        m.data[2][2] = rad.cos();
        m
    }

    pub fn rotation_y(deg: f64) -> Matrix {
        let rad = (deg * std::f64::consts::PI) / 180.0;
        let mut m = Matrix::identity(4);
        m.data[0][0] = rad.cos();
        m.data[0][2] = rad.sin();
        m.data[2][0] = -rad.sin();
        m.data[2][2] = rad.cos();
        m
    }

    pub fn rotation_z(deg: f64) -> Matrix {
        let rad = (deg * std::f64::consts::PI) / 180.0;
        let mut m = Matrix::identity(4);
        m.data[0][0] = rad.cos();
        m.data[0][1] = -rad.sin();
        m.data[1][0] = rad.sin();
        m.data[1][1] = rad.cos();
        m
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut m = Matrix::identity(4);
        m.data[0][1] = xy;
        m.data[0][2] = xz;
        m.data[1][0] = yx;
        m.data[1][2] = yz;
        m.data[2][0] = zx;
        m.data[2][1] = zy;
        m
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let (rows, cols) = (4, 4);
        let mut a: Matrix = Matrix::new(rows, cols);
        let mut b: Matrix = Matrix::new(rows, cols);

        let mut rng = rand::thread_rng();
        for i in 0..(rows as usize) {
            for j in 0..(cols as usize) {
                let value = rng.gen::<f64>();
                a.data[i][j] = value;
                b.data[i][j] = value;
            }
        }

        assert_eq!(a, b);
    }

    #[test]
    fn test_identity() {
        for _ in 0..10 {
            let n = 4;

            let m = Matrix::random(n as usize, n as usize, -10.0, 10.0);
            let u = Matrix::identity(n as usize);

            assert_eq!(m, m.multiply_matrix(&u));
        }
    }

    #[test]
    fn test_transpose() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let rows = rng.gen::<u8>();
            let cols = rng.gen::<u8>();

            let m = Matrix::random(rows as usize, cols as usize, -10.0, 10.0);

            assert_eq!(m, m.transpose().transpose());
        }
    }

    #[test]
    fn test_transpose_identity() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let n = rng.gen::<u8>();
            let u = Matrix::identity(n as usize);

            assert_eq!(u, u.transpose());
        }
    }

    #[test]
    fn test_determinant() {
        let m = Matrix::from_values(
            &vec![
                vec![1.0, 2.0, 6.0],
                vec![-5.0, 8.0, -4.0],
                vec![2.0, 6.0, 4.0],
            ]
        );
        assert_eq!(m.determinant(), -196.0);


        let m = Matrix::from_values(
            &vec![
                vec![-2.0, -8.0, 3.0, 5.0],
                vec![-3.0, 1.0, 7.0, 3.0],
                vec![1.0, 2.0, -9.0, 6.0],
                vec![-6.0, 7.0, 7.0, -9.0],
            ]
        );
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_invert() {
        let mut c = 0;
        for _ in 0..100 {
            let n = 4;

            let m = Matrix::random(n as usize, n as usize, -10.0, 10.0);
            if m.is_invertible() {
                let n = m.invert();
                if n.is_invertible() {
                    c += 1;
                    assert_eq!(m, n.invert());
                }
            }
        }
        assert_ne!(c, 0);
    }

    #[test]
    fn test_multiply_invert() {
        let mut counter = 0;
        let n = 4;
        for _ in 0..100 {

            let a = Matrix::random(n as usize, n as usize, -10.0, 10.0);
            let b = Matrix::random(n as usize, n as usize, -10.0, 10.0);
            let c = a.multiply_matrix(&b);

            if b.is_invertible() {
                assert_eq!(a, c.multiply_matrix(&b.invert()));
                counter += 1;
            }
        }
        assert_ne!(counter, 0);
    }

}
