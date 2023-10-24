use std::ops::{Add, Sub, Mul};
type Real = f64;

//Debug: This enables the usage of the {:?} formatter in println! statements for debugging purposes, allowing you to print the struct with all its fields.
//Clone: This allows you to create a copy of an instance of the struct, making a duplicate of the data.
//Copy: This trait specifies that the data of the type can be copied bit by bit. This is used for types that have simple copy semantics (like integers or floats)
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: Real,
    y: Real,
    z: Real,
    _pad: Real //since Rust does not support private field, this _pad is private and is not supposed to be used
}

impl Vector {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Vector { x, y, z, _pad: 0.0 }
    }

    pub fn default() -> Self {
        Vector { x: 0.0, y: 0.0, z: 0.0, _pad: 0.0}
    }

    pub fn magnitude(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot_product(&self, other: &Vector) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            _pad: 0.0
        }
    }

    pub fn invert(&mut self){
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            _pad: 0.0
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
            _pad: 0.0
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
            _pad: 0.0
        }
    }
}