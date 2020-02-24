use std::ops;
use crate::util;

#[derive(Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new_zero() -> Tuple {
        Tuple{ x:0.0, y:0.0, z:0.0, w:0.0}
    }
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple{ x:x, y:y, z:z, w:w }
    }
    pub fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Self::new(x, y, z, 0.0)
    }
    pub fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Self::new(x, y, z, 1.0)
    }
    pub fn equals(&self, other: &Self) -> bool {
        util::close_enough(self.x, other.x) &&
        util::close_enough(self.y, other.y) &&
        util::close_enough(self.z, other.z) &&
        util::close_enough(self.w, other.w)
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
    pub fn magnitude(&self) -> f32 {
        (
            self.x.powf(2.0) +
            self.y.powf(2.0) +
            self.z.powf(2.0) +
            self.w.powf(2.0)
        ).sqrt()
    }
    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple::new(
            self.x / mag,
            self.y / mag,
            self.z / mag,
            self.w / mag
        )
    }
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x +
            self.y * other.y +
            self.z * other.z +
            self.w * other.w
    }
    pub fn cross(&self, other: &Self) -> Self {
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}

// TODO: overload equality test
// It's too hard right now, I don't understand what's needed

impl ops::Add<&'_ Tuple> for &Tuple {
    type Output = Tuple;

    fn add(self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl ops::Sub<&'_ Tuple> for &Tuple {
    type Output = Tuple;

    fn sub(self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl ops::Neg for &Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        &Tuple::new_zero() - self
    }
}
impl ops::Mul<f32> for &Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other
        )
    }
}
impl ops::Mul<&'_ Tuple> for &Tuple {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Tuple {
        Tuple::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w
        )
    }
}
impl ops::Div<f32> for &Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other
        )
    }
}

#[cfg(test)]
mod test {
    use super::Tuple;
    use crate::util;

    #[test]
    /// A tuple with w=1.0 is a point
    fn tuple_w1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    /// A tuple with w=0 is a vector
    fn tuple_w0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    /// point() creates tuples with w=1
    fn point_creates_points() {
        let p = Tuple::new_point(4.0, -4.0, 3.0);
        let test_tuple = Tuple::new(4.0, -4.0, 3.0, 1.0);

        assert_eq!(p.equals(&test_tuple), true);

        // ensure it still works
        assert_eq!(p.x, 4.0);
    }

    #[test]
    /// vector() creates tuples with w=0
    fn vector_creates_vectors() {
        let v = Tuple::new_vector(4.0, -4.0, 3.0);
        let test_tuple = Tuple::new(4.0, -4.0, 3.0, 0.0);

        assert_eq!(v.equals(&test_tuple), true);

        // ensure it still works
        assert_eq!(v.x, 4.0);
    }

    #[test]
    /// Adding two tuples
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let test_tuple = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_eq!(
            test_tuple.equals(&(&a1 + &a2)),
            true
        );
    }

    #[test]
    /// Subtracting two points
    fn subtracting_two_points() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);

        let difference = &p1 - &p2;
        let test_tuple = Tuple::new_vector(-2.0, -4.0, -6.0);

        assert_eq!(
            test_tuple.equals(&difference),
            true
        );
    }
    #[test]
    /// Subtracting a vector from a point
    fn subtracting_vector_from_point() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);

        let test_tuple = Tuple::new_point(-2.0, -4.0, -6.0);
        let difference = &p - &v;

        assert_eq!(test_tuple.equals(&difference), true);
    }
    #[test]
    /// Subtracting two vectors
    fn subtracting_two_vectors() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let diff = &v1 - &v2;
        let test = Tuple::new_vector(-2.0, -4.0, -6.0);

        assert_eq!(diff.equals(&test), true);
    }
    #[test]
    /// Subtracting a vector from the zero vector
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::new_zero();
        let v = Tuple::new_vector(1.0, -2.0, 3.0);

        let diff = &zero - &v;
        let test = Tuple::new_vector(-1.0, 2.0, -3.0);

        assert_eq!(diff.equals(&test), true);
    }
    #[test]
    /// Negating a tuple
    fn negating_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let neg = -&a;
        let test = Tuple::new(-1.0, 2.0, -3.0, 4.0);

        assert_eq!(neg.equals(&test), true);
    }
    #[test]
    /// Multiplying a tuple by a scalar
    fn mult_tuple_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let mul = &a * 3.5;
        let test = Tuple::new(3.5, -7.0, 10.5, -14.0);

        assert_eq!(mul.equals(&test), true);
    }
    #[test]
    /// Multiplying a tuple by a fraction
    fn mult_tuple_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let mul = &a * 0.5;
        let test = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(mul.equals(&test), true);
    }
    #[test]
    /// Dividing a tuple by a scalar
    fn div_tuple_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let div = &a / 2.0;
        let test = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(div.equals(&test), true);
    }
    #[test]
    /// Computing the magnitude of vectors
    fn magnitude_of_vectors() {
        let v1 = Tuple::new_vector(1.0, 0.0, 0.0);
        let v2 = Tuple::new_vector(0.0, 1.0, 0.0);
        let v3 = Tuple::new_vector(0.0, 0.0, 1.0);
        let v4 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v5 = Tuple::new_vector(-1.0, -2.0, -3.0);

        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v3.magnitude(), 1.0);
        assert_eq!(v4.magnitude(), (14 as f32).sqrt());
        assert_eq!(v5.magnitude(), (14 as f32).sqrt());
    }
    #[test]
    /// Normalizing vectors
    fn normalizing_vectors() {
        let v1 = Tuple::new_vector(4.0, 0.0, 0.0);
        let v2 = Tuple::new_vector(1.0, 2.0, 3.0);

        let test1 = Tuple::new_vector(1.0, 0.0, 0.0);
        let test2 = Tuple::new_vector(
            1.0 / (14 as f32).sqrt(),
            2.0 / (14 as f32).sqrt(),
            3.0 / (14 as f32).sqrt(),
        );

        assert_eq!(v1.normalize().equals(&test1), true);
        assert_eq!(v2.normalize().equals(&test2), true);
        
        // magnitude of a normalized vec
        let norm = v2.normalize();
        assert_eq!(util::close_enough(norm.magnitude(), 1.0), true);

    }
    #[test]
    /// The dot product of two tuples
    fn dot_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(a.dot(&b), 20.0);
    }
    #[test]
    /// The cross product of two tuples
    fn cross_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        let testa = Tuple::new_vector(-1.0, 2.0, -1.0);
        let testb = Tuple::new_vector(1.0, -2.0, 1.0);

        assert_eq!(a.cross(&b).equals(&testa), true);
        assert_eq!(b.cross(&a).equals(&testb), true);
    }


    #[test]
    /// Test test
    fn test_test() {
        assert_eq!(true, true);
    }
}
