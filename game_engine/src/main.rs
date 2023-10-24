use std::ops::{Add, Sub, Mul};
type Real = f64;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: Real,
    y: Real,
    z: Real,
}

impl Vector {
    fn new(x: Real, y: Real, z: Real) -> Self {
        Vector { x, y, z }
    }

    fn magnitude(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn dot_product(&self, other: &Vector) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross_product(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Real> for Vector {
    type Output = Vector;

    fn mul(self, scalar: Real) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

fn main() {
    let v1 = Vector::new(1.0,2.0,3.0);
    println!("v1: {:?}", v1);
    println!("v1 magnitude: {:?}", v1.magnitude());
    println!("Hello, world!");
}
