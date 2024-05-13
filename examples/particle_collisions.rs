use bevy::prelude::*;
use bevy_xpbd::{resources::Gravity, *};

#[derive(Component)]
struct CameraMarker;

#[derive(Component)]
struct LeftParticle;

#[derive(Component)]
struct RightParticle;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(DefaultPlugins)
        .add_plugins(XPBDPlugin)
        .add_systems(Startup, (spawn_sphere, spawn_camera))
        .run();
}

fn spawn_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let big_sphere_mesh = meshes.add(Sphere::new(1.5).mesh().ico(4).unwrap());
    let small_sphere_mesh = meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap());
    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            // transform: Transform::from_xyz(0., 0., 0.),
            mesh: big_sphere_mesh,
            material: white_material.clone(),
            ..default()
        },
        entity::ParticleBundle::new_with_pos_and_vel_and_mass_and_collider(Vec2::new(-5., 0.), Vec2::new(2.,0.), 3., 1.5),
        LeftParticle,
    ));
    commands.spawn((
        PbrBundle {
            // transform: Transform::from_xyz(0., 0., 0.),
            mesh: small_sphere_mesh,
            material: white_material.clone(),
            ..default()
        },
        entity::ParticleBundle::new_with_pos_and_vel_and_mass_and_collider(Vec2::new(5., 0.), Vec2::new(-2.,0.), 1., 0.5),
        RightParticle,
    ));
}

fn spawn_camera(mut commands: Commands) {
    let camera_bundle = Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 50.).looking_at(Vec3::new(0.,0.,0.), Vec3::Y),
        ..default()
    };
    commands.spawn((camera_bundle, CameraMarker));
}

