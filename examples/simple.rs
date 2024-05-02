use bevy::prelude::*;
use bevy_xpbd::*;

#[derive(Component)]
struct CameraMarker;


fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_sphere, spawn_camera))
        .add_systems(Update, (simulate,sync_transform))
        .run();
}

fn spawn_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap());
    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            mesh: sphere_mesh.clone(),
            material: white_material.clone(),
            ..Default::default()
        },
        BallBundle::default(),
    ));
}

fn spawn_camera(mut commands: Commands) {
    let camera_bundle = Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 10.).looking_at(Vec3::new(0.,0.,0.), Vec3::Y),
        // transform: Transform::from_translation(Vec3::new(0.,0.,-20.)),
        ..default()
    };
    commands.spawn((camera_bundle, CameraMarker));
}

fn simulate(mut query: Query<(&mut Pos, &mut PrevPos), With<Ball>>){
    for (mut pos, mut prev_pos) in query.iter_mut() {
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME;
        prev_pos.0 = pos.0;
        pos.0 += velocity * DELTA_TIME;
        println!("pos: {}", pos.0);
    }
}

fn sync_transform(
    mut query: Query<(&mut Transform, &Pos)>
) {
    for (mut transform, pos ) in query.iter_mut() {
        transform.translation = pos.0
    }
}
