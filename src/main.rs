pub mod tuple;
pub use tuple::{Tuple};


fn main() {
    let t1 = Tuple::new_zero();
    let t2 = Tuple::new(0.0, 2.0, 0.0, 0.0);

    let t3 = &t1 + &t2;
    let t4 = &t1 - &t2;
    let t5 = -&t2;

    let t6 = &t2 * 6.0;


    println!("{}: {:?}", 1, t1);
    println!("{}: {:?}", 2, t2);
    println!("{}: {:?}", 3, t3);
    println!("{}: {:?}", 4, t4);
    println!("{}: {:?}", 5, t5);
    println!("{}: {:?}", 6, t6);

    println!("{}", t2.magnitude());
    println!("{:?}", t2.normalize());
    println!("{:?}", t2.dot(&t1));
    println!("{:?}", t2.cross(&t1));
}
