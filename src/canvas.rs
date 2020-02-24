use crate::color::Color;
use std::mem;

#[derive(Debug)]
pub struct Canvas {
    pixels: Vec<Color>,
    pub width: i32,
    pub height: i32,
    length: i32
}

impl Canvas {
    pub fn new(height: i32, width: i32) -> Self {
        Canvas {
            height: height,
            width: width,
            length: height * width,
            pixels: vec![Color::new_zero(); (height * width) as usize]
        }
    }
    pub fn get_pixel(&self, x: i32, y: i32) -> &Color {
        let pos = y * self.width + x;
        &self.pixels[pos as usize]
    }
    pub fn set_pixel(&mut self, x: i32, y: i32, pixel: Color) -> () {
        let pos = y * self.width + x;
        mem::replace::<Color>(&mut self.pixels[pos as usize], pixel);
    }
    pub fn test(&self) -> () {
        println!("{:?}", self.pixels);
    }

    pub fn print_ppm(&self) -> () {
        println!("{}", "P3");
        println!("{} {}", self.width, self.height);
        println!("{}", 255);

        for pixel in &self.pixels {
            println!("{}", pixel.as_ppm());
        }
    }
}

#[cfg(test)]
mod test {
    use super::Canvas;
    use crate::color::Color;

    #[test]
    /// Test creating
    fn canvas_is() {
        let mut c = Canvas::new(10, 10);

        assert_eq!(c.height, 10);
        assert_eq!(c.width, 10);
        assert_eq!(c.length, 10 * 10);

        c.set_pixel(1, 2, Color::new_zero());
    }
    #[test]
    /// writing pixels to canvas
    fn canvas_write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.set_pixel(2, 3, red);
        assert_eq!(c.get_pixel(2, 3).equals(&red), true);
    }


    #[test]
    /// Test test
    fn test_test() {
        assert_eq!(true, true);
    }
}
