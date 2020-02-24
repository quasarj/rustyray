pub mod tuple;
pub mod color;
pub mod util;
pub mod canvas;
pub use tuple::{Tuple};
pub use color::Color;
pub use canvas::Canvas;

// use std::vec::Vec;

struct Enviornment {
    gravity: Tuple,
    wind: Tuple
}

struct Projectile {
    position: Tuple,
    velocity: Tuple
}

fn main() {
    let start = Tuple::new_point(0.0, 1.0, 0.0);
    let velocity = &Tuple::new_vector(1.0, 1.8, 0.0).normalize() * 11.25;

    let mut p = Projectile { position: start, velocity: velocity };

    let gravity = Tuple::new_vector(0.0, -0.1, 0.0);
    let wind = Tuple::new_vector(-0.01, 0.0, 0.0);

    let e = Enviornment { gravity: gravity, wind: wind };

    // let mut c = Canvas::new(900, 550);
    let mut c = Canvas::new(550, 900);

    while p.position.y > 0.0 {
        let x = p.position.x as i32;
        let y = p.position.y as i32;

        // println!("({},{})", x, y);

        // flip y
        let y_f = c.height - y;

        c.set_pixel(x, y_f, Color::new(1.0, 1.0, 1.0));

        p = tick(&e, &p);
    }

    c.print_ppm();

}

fn tick(env: &Enviornment, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position: position, velocity: velocity }
}
