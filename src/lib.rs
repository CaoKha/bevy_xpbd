use bevy::prelude::*;

pub const DELTA_TIME: f32 = 1. / 60.;

#[derive(Component)]
pub struct Pos(pub Vec3);

#[derive(Component)]
pub struct PrevPos(pub Vec3);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    pub name: Ball,
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub v: Velocity,
}

impl BallBundle {
    fn new() -> Self {
        Self {
            name: Ball,
            pos: Pos(Vec3::ZERO),
            prev_pos: PrevPos(Vec3::ZERO - Vec3::new(2., 0., 0.) * DELTA_TIME),
            v: Velocity(Vec3::new(1., 0., 0.)),
        }
    }
}

impl Default for BallBundle {
    fn default() -> Self {
        Self::new()
    }
}
