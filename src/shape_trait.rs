use std::f64::consts;

pub trait Shape {
    fn perimeter(&self) -> f64;
}

pub struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {
    pub fn new(length: f64, width: f64) -> Rectangle {
        Self { length, width }
    }
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f64 {
        (self.length + self.width) * 2.
    }
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Self { radius }
    }
}

impl Shape for Circle {
    fn perimeter(&self) -> f64 {
        2. * consts::PI * self.radius
    }
}

pub struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Triangle {
    pub fn new(side1: f64, side2: f64, side3: f64) -> Triangle {
        assert!(side1 + side2 > side3 && side2 + side3 > side1 && side3 + side1 > side2);
        Self {
            side1,
            side2,
            side3,
        }
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}
