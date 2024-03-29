use super::geometry::Transform;
use rapier2d::na::Vector2;
use rapier2d::prelude::*;

use crate::theory::geometry::rotate_vec2;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct RigidBodyProperty {
    pub mass: f32,
    pub size: (f32, f32),
    pub initial_transform: Transform,
}

#[derive(Debug)]
pub struct Physics(RigidBodyHandle);

#[derive(Debug)]
pub struct PhysicsController<'a>(pub &'a mut RigidBody);

impl<'a> PhysicsController<'a> {
    pub fn apply_force(&mut self, at: Option<(f32, f32)>, vector: (f32, f32)) {
        match at {
            Some(at) => self.0.add_force_at_point(
                tuple_to_vec(vector),
                (tuple_to_vec(at) + self.0.position().translation.vector).into(),
                true,
            ),
            None => self.0.apply_impulse(tuple_to_vec(vector), true),
        }
    }

    pub fn apply_force_locally(&mut self, at: (f32, f32), vector: (f32, f32)) {
        let angle = self.0.rotation().angle();

        let at = rotate_vec2(angle, at);
        let vector = rotate_vec2(angle, vector);

        self.apply_force(Some(at), vector);
    }

    pub fn to_transform(&self) -> Transform {
        Transform {
            location: (self.0.translation().x, self.0.translation().y),
            angle: self.0.rotation().angle(),
        }
    }
}

pub struct PhysicalWorld {
    rigidbody_set: RigidBodySet,
    physics_pipeline: PhysicsPipeline,
    gravity: Vector2<Real>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl Default for PhysicalWorld {
    fn default() -> Self {
        PhysicalWorld::new()
    }
}

impl Debug for PhysicalWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PhysicalWorld: {}>", self.rigidbody_set.len())
    }
}

impl PhysicalWorld {
    pub fn new() -> Self {
        PhysicalWorld {
            rigidbody_set: RigidBodySet::default(),
            physics_pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, 0.0],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn register(&mut self, property: RigidBodyProperty) -> Physics {
        let rigidbody = RigidBodyBuilder::dynamic()
            .translation(tuple_to_vec(property.initial_transform.location))
            .rotation(property.initial_transform.angle)
            .additional_mass(property.mass)
            .build();

        let collider =
            ColliderBuilder::cuboid(property.size.0 / 2.0, property.size.1 / 2.0).build();

        let handle = self.rigidbody_set.insert(rigidbody);

        self.collider_set
            .insert_with_parent(collider, handle, &mut self.rigidbody_set);

        Physics(handle)
    }

    pub fn tick(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigidbody_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );
    }

    pub fn get(&mut self, physics: &mut Physics) -> Option<PhysicsController> {
        self.rigidbody_set.get_mut(physics.0).map(PhysicsController)
    }
}

fn tuple_to_vec(tuple: (f32, f32)) -> Vector2<Real> {
    vector![tuple.0, tuple.1]
}
