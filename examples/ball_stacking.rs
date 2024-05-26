use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_xpbd::{
    components::{BoxCollider, CircleCollider, Pos},
    entity::{ParticleBundle, StaticBoxBundle},
    XPBDPlugin,
};
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins,
            XPBDPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (spawn_camera, spawn_balls))
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 10.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn spawn_balls(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let sphere = meshes.add(Sphere::new(1.).mesh().ico(4).unwrap());
    let blue = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.4, 0.6),
        unlit: true,
        ..default()
    });
    let size = Vec2::new(20., 2.);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Rectangle::new(1., 1.).mesh()),
            material: blue.clone(),
            transform: Transform::from_scale(size.extend(1.)),
            ..default()
        },
        StaticBoxBundle {
            pos: Pos(Vec2::new(0., -4.)),
            collider: BoxCollider { size },
            ..default()
        },
    ));
    let radius = 0.15;
    let stacks = 5;
    for i in 0..15 {
        for j in 0..stacks {
            let pos = Vec2::new(
                (j as f32 - stacks as f32 / 2.) * 2.5 * radius,
                2. * radius * i as f32 - 2.,
            );
            let vel = Vec2::ZERO;
            commands.spawn((
                PbrBundle {
                    mesh: sphere.clone(),
                    material: blue.clone(),
                    transform: Transform {
                        scale: Vec3::splat(radius),
                        translation: pos.extend(0.),
                        ..default()
                    },
                    ..default()
                },
                ParticleBundle {
                    collider: CircleCollider { radius },
                    ..ParticleBundle::new_with_pos_and_vel(pos, vel)
                },
            ));
        }
    }
}
