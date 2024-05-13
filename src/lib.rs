use bevy::prelude::*;
pub mod components;
pub mod entity;
pub mod resources;
pub const DELTA_TIME: f32 = 1. / 60.;

use components::*;
use resources::{Contacts, Gravity};

#[derive(Debug, Default)]
pub struct XPBDPlugin;

impl Plugin for XPBDPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(DELTA_TIME.into()))
            .init_resource::<Gravity>()
            .init_resource::<Contacts>()
            .add_systems(
                FixedUpdate,
                (
                    collect_collision_pairs.before(integrate),
                    integrate,
                    solv_pos.after(integrate),
                    update_velocity.after(solv_pos),
                    solv_vel.after(update_velocity),
                    sync_transform.after(solv_vel),
                ),
            );
    }
}

fn sync_transform(mut query: Query<(&mut Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.)
    }
}

fn integrate(
    mut query: Query<(
        &mut Pos,
        &mut PrevPos,
        &mut Velocity,
        &mut PreSolveVel,
        &Mass,
    )>,
    gravity: Res<Gravity>,
) {
    for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
        prev_pos.0 = pos.0;
        let gravitational_force = mass.0 * gravity.0;
        let external_forces = gravitational_force;
        vel.0 += (external_forces / mass.0) * DELTA_TIME;
        pos.0 += vel.0 * DELTA_TIME;
        pre_solve_vel.0 = vel.0;
    }
}

fn update_velocity(mut query: Query<(&mut Pos, &mut PrevPos, &mut Velocity)>) {
    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / DELTA_TIME;
    }
}

fn collect_collision_pairs() {}
fn solv_pos(
    mut query: Query<(Entity, &mut Pos, &CircleCollider, &Mass)>,
    mut contacts: ResMut<Contacts>,
) {
    contacts.0.clear();
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [(entity_a, mut pos_a, circle_a, mass_a), (entity_b, mut pos_b, circle_b, mass_b)],
    ) = iter.fetch_next()
    {
        let ab = pos_b.0 - pos_a.0;
        let combined_radius = circle_a.radius + circle_b.radius;
        if ab.length_squared() < combined_radius.powi(2) {
            let penetration_depth = combined_radius - ab.length();
            let n = ab.normalize();
            let w_a = 1. / mass_a.0;
            let w_b = 1. / mass_b.0;
            let w_sum = w_a + w_b;
            pos_a.0 -= n * penetration_depth * w_a / w_sum;
            pos_b.0 += n * penetration_depth * w_b / w_sum;
            contacts.0.push((entity_a, entity_b));
        }
    }
}
fn solv_vel(query: Query<(&mut Velocity, &PreSolveVel, &Pos, &Mass, &Restitution)>, contacts: Res<Contacts>) {
    for (entity_a, entity_b) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, pos_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, pos_b, mass_b, restitution_b),
        ) = unsafe {
            // Ensure safety
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };
        let n = (pos_b.0 - pos_a.0).normalize();
        let pre_solve_relative_vel = pre_solve_vel_a.0 - pre_solve_vel_b.0;
        let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

        let relative_vel = vel_a.0 - vel_b.0;
        let normal_vel = Vec2::dot(relative_vel, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;

        let w_a = 1. / mass_a.0;
        let w_b = 1. / mass_b.0;
        let w_sum = w_a + w_b;

        vel_a.0 += n * (-normal_vel - restitution * pre_solve_normal_vel) * w_a / w_sum;
        vel_b.0 -= n * (-normal_vel - restitution * pre_solve_normal_vel) * w_b / w_sum;

    
    }
}
