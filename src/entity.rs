use bevy::prelude::*;
use crate::components::*;
use crate::DELTA_TIME;


#[derive(Bundle)]
pub struct ParticleBundle {
    pub ball: Ball,
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub mass: Mass,
    pub collider: CircleCollider,
    pub vel: Velocity,
    pub pre_solve_vel: PreSolveVel,
    pub restitution: Restitution
}

impl ParticleBundle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            ball: Ball,
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * DELTA_TIME),
            mass: Mass::default(),
            collider: CircleCollider::default(),
            vel: Velocity(vel),
            pre_solve_vel: PreSolveVel::default(),
            restitution: Restitution::default()
        }
    }
    pub fn new_with_pos_and_vel_and_mass(pos: Vec2, vel: Vec2, mass: f32) -> Self {
        Self {
            ball: Ball,
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * DELTA_TIME),
            mass: Mass(mass),
            collider: CircleCollider::default(),
            vel: Velocity(vel),
            pre_solve_vel: PreSolveVel::default(),
            restitution: Restitution::default()
        }
    }

    pub fn new_with_pos_and_vel_and_mass_and_collider(pos: Vec2, vel: Vec2, mass: f32, collider_radius: f32) -> Self {
        Self {
            ball: Ball,
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * DELTA_TIME),
            mass: Mass(mass),
            collider: CircleCollider {radius: collider_radius},
            vel: Velocity(vel),
            pre_solve_vel: PreSolveVel::default(),
            restitution: Restitution::default()
        }
    }
}

impl Default for ParticleBundle {
    fn default() -> Self {
        Self::new_with_pos_and_vel(Vec2::ZERO, Vec2::new(2.,0.))
    }
}

