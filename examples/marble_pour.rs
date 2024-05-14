use bevy::prelude::*;
use bevy_xpbd::{
    components::{BoxCollider, CircleCollider, Pos},
    entity::{ParticleBundle, StaticBoxBundle},
    XPBDPlugin, DELTA_TIME,
};
use rand::random;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(Time::<Fixed>::from_seconds(DELTA_TIME.into()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (480., 360.).into(),
                // prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(XPBDPlugin)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, (spawn_marble, despawn_marble))
        .run()
}

#[derive(Component)]
struct CameraMarker;

#[derive(Debug, Resource)]
struct Materials {
    blue: Handle<StandardMaterial>,
}

#[derive(Debug, Resource)]
struct Meshes {
    sphere: Handle<Mesh>,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Sphere::new(1.).mesh().ico(4).unwrap());
    let blue = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.4, 0.6),
        unlit: true,
        ..default()
    });
    // let radius = 15.;
    let size = Vec2::new(5., 2.);
    commands.insert_resource(Meshes {
        sphere: sphere.clone(),
    });

    commands.insert_resource(Materials { blue: blue.clone() });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Rectangle::new(1., 1.).mesh()),
            material: blue.clone(),
            transform: Transform::from_scale(size.extend(1.)),
            ..default()
        },
        StaticBoxBundle {
            pos: Pos(Vec2::new(0., -3.)),
            collider: BoxCollider { size },
            ..default()
        },
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 10.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        CameraMarker,
    ));
}

fn spawn_marble(mut commands: Commands, materials: Res<Materials>, meshes: Res<Meshes>) {
    let radius = 0.1;
    let pos = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * 0.5 + Vec2::Y * 3.;
    let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
    commands.spawn((
        PbrBundle {
            mesh: meshes.sphere.clone(),
            material: materials.blue.clone(),
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

fn despawn_marble(mut commands: Commands, query: Query<(Entity, &Pos)>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -20. {
            commands.entity(entity).despawn();
        }
    }
}
