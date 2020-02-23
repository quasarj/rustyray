use std::ops;

#[derive(Debug)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn test(&self) -> f32 {
        self.x * self.y * self.z * self.w
    }
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
        self.x == other.x &&
            self.y == other.y &&
            self.z == other.z &&
            self.w == other.w
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

#[cfg(test)]
mod test {
    use super::Tuple;
    #[test]
    fn basics() {
        let t = Tuple::new_zero();

        let test = t.test();

        assert_eq!(test, 0.0);

        let t2 = Tuple{ x:1.0, y:2.0, z:3.0, w:0.0};
        let t3 = &t + &t2;

        assert_eq!(t2.equals(&t3), true);
        assert_eq!(t2.equals(&t), false);

        assert_eq!(t2.magnitude(), 3.7416575);

    }
}
