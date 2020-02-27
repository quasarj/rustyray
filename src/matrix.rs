use std::ops::Index;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Matrix4x4 {
    data: [f32; 16],
}

pub struct Matrix3x3 {
}

pub struct Matrix2x2 {
}

impl Index<(i32, i32)> for Matrix4x4 {
    type Output = f32;

    fn index(&self, idx: (i32, i32)) -> &Self::Output {
        let (y, x) = idx;
        let pos = y * 4 + x;
        &self.data[pos as usize]
    }
}


#[cfg(test)]
mod test {
    use super::Matrix4x4;

    #[test]
    /// Constructing and inspecting a 4x4 matrix
    fn matrix4x4_exists() {
        let m = Matrix4x4 { data: [1.0, 2.0, 3.0, 4.0, 
                                   5.5, 6.5, 7.5, 8.5,
                                   9.0, 10.0, 11.0, 12.0,
                                   13.5, 14.5, 15.5, 16.5] };


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
}
