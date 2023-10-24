mod vector;
use vector::Vector;

fn main() {
    let v1 = Vector::new(1.0,2.0,3.0);
    let v2 = Vector::new(3.0,4.0,5.0);

    println!("v1: {:?}", v1);
    println!("v1 magnitude: {:?}", v1.magnitude());
    println!("v1 + v2 = {:?}", v1 + v2);

    println!("Hello, world!");
}
