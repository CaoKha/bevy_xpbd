use bevy::prelude::*;
pub mod components;
pub mod entity;
pub mod resources;
pub const DELTA_TIME: f32 = 1. / 60.;

use components::*;
use resources::{CollisionPairs, Contacts, Gravity, StaticContacts};

#[derive(Debug, Default)]
pub struct XPBDPlugin;

impl Plugin for XPBDPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(DELTA_TIME.into()))
            .init_resource::<Gravity>()
            .init_resource::<CollisionPairs>()
            .init_resource::<Contacts>()
            .init_resource::<StaticContacts>()
            .add_systems(
                FixedUpdate,
                (
                    collect_collision_pairs.before(integrate),
                    integrate,
                    clear_contacts.before(solve_pos),
                    solve_pos.after(integrate),
                    solve_pos_statics.after(integrate),
                    solve_pos_static_boxes.after(integrate),
                    update_velocity.after(solve_pos),
                    solve_vel.after(update_velocity),
                    solve_vel_statics.after(update_velocity),
                    sync_transform.after(solve_vel),
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

fn collect_collision_pairs(
    query: Query<(Entity, &Pos, &Velocity, &CircleCollider)>,
    mut collision_pairs: ResMut<CollisionPairs>,
) {
    collision_pairs.0.clear();
    let k = 2.;
    let safety_margin_factor = k * DELTA_TIME;
    let safety_margin_factor_sqr = safety_margin_factor.powi(2);
    unsafe {
        for (entity_a, pos_a, vel_a, circle_a) in query.iter_unsafe() {
            let vel_a_sqr = vel_a.0.length_squared();
            for (entity_b, pos_b, vel_b, circle_b) in query.iter_unsafe() {
                if entity_a <= entity_b {
                    continue;
                }
                let ab = pos_b.0 - pos_a.0;
                let vel_b_sqr = vel_b.0.length_squared();
                let safety_margin_sqr = safety_margin_factor_sqr * (vel_a_sqr + vel_b_sqr);
                let combined_radius = circle_a.radius + circle_b.radius + safety_margin_sqr.sqrt();
                let ab_sqr_len = ab.length_squared();
                if ab_sqr_len < combined_radius.powi(2) {
                    collision_pairs.0.push((entity_a, entity_b))
                }
            }
        }
    }
}
fn solve_pos(
    query: Query<(&mut Pos, &CircleCollider, &Mass)>,
    collision_pairs: ResMut<CollisionPairs>,
) {
    for (entity_a, entity_b) in collision_pairs.0.iter() {
        let ((mut pos_a, circle_a, mass_a), (mut pos_b, circle_b, mass_b)) = unsafe {
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(*entity_a).unwrap(),
                query.get_unchecked(*entity_b).unwrap(),
            )
        };

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
        }
    }
}
fn solve_vel(
    query: Query<(&mut Velocity, &PreSolveVel, &Mass, &Restitution)>,
    contacts: Res<Contacts>,
) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, mass_b, restitution_b),
        ) = unsafe {
            // Ensure safety
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };
        let pre_solve_relative_vel = pre_solve_vel_a.0 - pre_solve_vel_b.0;
        let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

        let relative_vel = vel_a.0 - vel_b.0;
        let normal_vel = Vec2::dot(relative_vel, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;

        let w_a = 1. / mass_a.0;
        let w_b = 1. / mass_b.0;
        let w_sum = w_a + w_b;

        let restitution_velocity = (-restitution * pre_solve_normal_vel).min(0.);
        let vel_impulse = n * ((-normal_vel + restitution_velocity) / w_sum);

        vel_a.0 += vel_impulse * w_a;
        vel_b.0 -= vel_impulse * w_b;
    }
}

fn solve_vel_statics(
    mut dynamics: Query<(&mut Velocity, &PreSolveVel, &Restitution), With<Mass>>,
    statics: Query<&Restitution, Without<Mass>>,
    contacts: Res<StaticContacts>,
) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (mut vel_a, pre_solve_vel_a, restitution_a) = dynamics.get_mut(entity_a).unwrap();
        let restitution_b = statics.get(entity_b).unwrap();
        let pre_solve_normal_vel = Vec2::dot(pre_solve_vel_a.0, n);
        let normal_vel = Vec2::dot(vel_a.0, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;
        vel_a.0 += n * (-normal_vel + (-restitution * pre_solve_normal_vel).min(0.));
    }
}

fn solve_pos_statics(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &CircleCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>,
) {
    for (entity_a, mut pos_a, circle_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, circle_b) in statics.iter() {
            let ab = pos_b.0 - pos_a.0;
            let combined_radius = circle_a.radius + circle_b.radius;
            let ab_sqr_len = ab.length_squared();
            if ab_sqr_len < combined_radius * combined_radius {
                let ab_length = ab_sqr_len.sqrt();
                let penetration_depth = combined_radius - ab_length;
                let n = ab / ab_length;
                pos_a.0 -= n * penetration_depth;
                contacts.0.push((entity_a, entity_b, n));
            }
        }
    }
}

fn clear_contacts(mut contacts: ResMut<Contacts>, mut static_contacts: ResMut<StaticContacts>) {
    contacts.0.clear();
    static_contacts.0.clear();
}

fn solve_pos_static_boxes(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &BoxCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>,
) {
    for (entity_a, mut pos_a, circle_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, box_b) in statics.iter() {
            let box_to_circle = pos_a.0 - pos_b.0;
            let box_to_circle_abs = box_to_circle.abs();
            let half_extents = box_b.size / 2.;
            let corner_to_center = box_to_circle_abs - half_extents;
            let r = circle_a.radius;
            if corner_to_center.x > r || corner_to_center.y > r {
                continue;
            }
            let s = box_to_circle.signum();

            let (n, penetration_depth) = if corner_to_center.x > 0. && corner_to_center.y > 0. {
                // Corner case
                let corner_to_center_sqr = corner_to_center.length_squared();
                if corner_to_center_sqr > r * r {
                    continue;
                }
                let corner_dist = corner_to_center_sqr.sqrt();
                let penetration_depth = r - corner_dist;
                let n = corner_to_center / corner_dist * -s;
                (n, penetration_depth)
            } else if corner_to_center.x > corner_to_center.y {
                // Closer to vertical edge
                (Vec2::X * -s.x, -corner_to_center.x + r)
            } else {
                (Vec2::Y * -s.y, -corner_to_center.y + r)
            };

            pos_a.0 -= n * penetration_depth;
            contacts.0.push((entity_a, entity_b, n));
        }
    }
}
