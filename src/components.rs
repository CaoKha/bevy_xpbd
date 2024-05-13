use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Ball;

#[derive(Component, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug)]
pub struct Velocity(pub(crate) Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.) // Default to 1 kg
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

#[derive(Component, Debug, Default)]
pub struct PreSolveVel(pub(crate) Vec2);

#[derive(Component, Debug)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}