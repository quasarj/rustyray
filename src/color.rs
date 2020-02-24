use std::ops;
use crate::util;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new_zero() -> Self {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }
    pub fn equals(&self, other: &Self) -> bool {
        util::close_enough(self.red, other.red) &&
        util::close_enough(self.green, other.green) &&
        util::close_enough(self.blue, other.blue)
    }
    pub fn as_ppm(&self) -> String {
        let r: i32 = (self.red * 255.0).ceil() as i32;
        let g: i32 = (self.green * 255.0).ceil() as i32;
        let b: i32 = (self.blue * 255.0).ceil() as i32;


        format!(
            "{} {} {}",
            clamp(0, r, 255),
            clamp(0, g, 255),
            clamp(0, b, 255)
        )
    }
}
impl ops::Add<&'_ Color> for &Color {
    type Output = Color;

    fn add(self, other: &Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}
impl ops::Sub<&'_ Color> for &Color {
    type Output = Color;

    fn sub(self, other: &Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}
impl ops::Mul<f32> for &Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other
        }
    }
}
impl ops::Mul<&'_ Color> for &Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue
        }
    }
}

pub fn clamp(min: i32, val: i32, max: i32) -> i32 {
    assert!(min <= max);
    if val < min {
        min
    }
    else if val > max {
        max
    } else {
        val
    }
}

#[cfg(test)]
mod test {
    use super::Color;

    #[test]
    /// Colors are colory
    fn colors_are() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
    #[test]
    /// Adding two colors
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let add = &c1 + &c2;
        let test = Color::new(1.6, 0.7, 1.0);

        assert_eq!(add.equals(&test), true);
    }
    #[test]
    /// Subtracting two colors
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let add = &c1 - &c2;
        let test = Color::new(0.2, 0.5, 0.5);

        assert_eq!(add.equals(&test), true);
    }
    #[test]
    /// Multiplying two colors
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let add = &c1 * &c2;
        let test = Color::new(0.9, 0.2, 0.04);

        assert_eq!(add.equals(&test), true);
    }
    #[test]
    /// Multiplying a color by scalar
    fn multiply_color_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        let mul = &c * 2.0;
        let test = Color::new(0.4, 0.6, 0.8);

        assert_eq!(mul.equals(&test), true);
    }
}
