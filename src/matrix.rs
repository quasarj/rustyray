use std::ops::Index;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Matrix {
    data: [f32; 16],
    size: i32,
}

impl Index<(i32, i32)> for Matrix {
    type Output = f32;

    fn index(&self, idx: (i32, i32)) -> &Self::Output {
        let (y, x) = idx;
        let pos = y * self.size + x;
        &self.data[pos as usize]
    }
}

impl Matrix {
    pub fn new4x4(initial_data: [f32; 16]) -> Matrix {
        Matrix { size: 4, data: initial_data }
    }
    pub fn new3x3(initial_data: [f32; 9]) -> Matrix {
        let i = initial_data;
        Matrix { 
            size: 3, data: [
                i[0], i[1], i[2],
                i[3], i[4], i[5],
                i[6], i[7], i[8],
                // unused space
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ] 
        }
    }
    pub fn new2x2(initial_data: [f32; 4]) -> Matrix {
        let i = initial_data;
        Matrix { 
            size: 2, data: [
                i[0], i[1], 
                i[2], i[3], 

                // unused space
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ] 
        }
    }
}


#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    /// Constructing and inspecting a 4x4 matrix
    fn matrix4x4_exists() {
        let m = Matrix::new4x4([ 1.0,  2.0,  3.0,  4.0, 
                                 5.5,  6.5,  7.5,  8.5,
                                 9.0, 10.0, 11.0, 12.0,
                                13.5, 14.5, 15.5, 16.5]);


        assert_eq!(m == m, true);


        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    /// Constructing and inspecting a 3x3 matrix
    fn matrix3x3_exists() {

        let m = Matrix::new3x3([-3.0, 5.0, 0.0,
                                 1.0,-2.0,-7.0,
                                 0.0, 1.0, 1.0 ]);

        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }
    #[test]
    /// Constructing and inspecting a 2x2 matrix
    fn matrix2x2_exists() {

        let m = Matrix::new2x2([-3.0,  5.0, 
                                 1.0, -2.0]);

        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }
}
