use rapier2d::na::{Point2, Vector2};
use rapier2d::prelude::*;
use crate::scece::game::input::Control;
use super::geometry::{Transform};

#[derive(Debug)]
pub struct RigidBodyProperty {
    pub mass: f32,
    pub size: (f32, f32),
    pub initial_transform: Transform
}

#[derive(Debug)]
pub struct Physics(RigidBodyHandle);

#[derive(Debug)]
pub struct PhysicsController<'a>(&'a mut RigidBody);

impl<'a> PhysicsController<'a> {
    pub(self) fn new(physics: &'a mut RigidBody) -> Self {
        PhysicsController(physics)
    }

    pub fn apply_force(&mut self, at: Option<(f32, f32)>, vector: (f32, f32)) {
        match at {
            Some(at) => self.0.add_force_at_point(
                tuple_to_vec(vector),
                tuple_to_vec(at).into(),
                true
            ),
            None => self.0.add_force(tuple_to_vec(vector), true)
        }
    }
}

pub struct PhysicalWorld(RigidBodySet);

impl PhysicalWorld {
    pub fn new() -> Self {
        PhysicalWorld(RigidBodySet::default())
    }

    pub fn register(&mut self, property: RigidBodyProperty) -> Physics {
        let rigidbody = RigidBodyBuilder::dynamic()
            .translation(tuple_to_vec(property.initial_transform.location))
            .rotation(property.initial_transform.angle)
            .additional_mass(property.mass)
            .build();

        Physics(self.0.insert(rigidbody))
    }

    pub fn get(&mut self, physics: &mut Physics) -> Option<PhysicsController> {
        self.0.get_mut(physics.0).map(PhysicsController)
    }
}

fn tuple_to_vec(tuple: (f32, f32)) -> Vector2<Real> {
    vector![tuple.0, tuple.1]
}
