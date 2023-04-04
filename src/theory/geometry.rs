use std::ops::{Add, Mul, AddAssign, MulAssign};

#[derive(Clone, Debug, Default)]
pub struct Transform {
    pub location: (f32, f32),
    pub angle: f32
}

impl Transform {
    pub fn new(location: (f32, f32), angle: f32) -> Self {
        Self { location, angle }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector(pub f32, pub f32);

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_vec = self;
        new_vec += rhs;

        new_vec
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut new_vec = self;
        new_vec *= rhs;

        new_vec
    }
}

impl From<(f32, f32)> for Vector {
    fn from(value: (f32, f32)) -> Self {
        Vector(value.0, value.1)
    }
}

impl From<Vector> for ggez::glam::Vec2 {
    fn from(val: Vector) -> Self {
        ggez::glam::vec2(val.0, val.1)
    }
}
