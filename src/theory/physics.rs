use super::geometry::{Transform, Vector};

#[derive(Debug, Default)]
pub struct Physics {
    pub transform: Transform,
    vel: Vector,
    acc: Vector,
    anglular_vel: f32,
    anglular_acc: f32,
}

impl Physics {
    pub fn new(transform: Transform, vel: Vector, acc: Vector, anglular_vel: f32, anglular_acc: f32) -> Self {
        Self { transform, vel, acc, anglular_vel, anglular_acc }
    }

    pub fn tick(&mut self) {
        self.vel += self.acc;
        self.transform.location += self.vel;

        self.anglular_vel += self.anglular_acc;
        self.transform.angle += self.anglular_vel;
    }

    pub fn apply_force(&mut self, force: Vector) {
        self.acc += force;
    }

    pub fn apply_angular_force(&mut self, force: f32) {
        self.anglular_acc += force;
    }
}
