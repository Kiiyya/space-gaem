use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_handle = meshes.add(Mesh::from(shape::Icosphere::default()));
    let mat_handle = materials.add(StandardMaterial {
        base_color: Color::YELLOW,
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: mat_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}

fn camera_wasd(
    input: Res<Input<KeyCode>>,
    mut cam: Query<(&mut Transform, &PerspectiveProjection)>,
    time: Res<Time>,
) {
    let mut dir = Vec3::ZERO;
    let w = input.pressed(KeyCode::W);
    let a = input.pressed(KeyCode::A);
    let s = input.pressed(KeyCode::S);
    let d = input.pressed(KeyCode::D);
    if (w != s) || (a != d) {
        match (w, s) {
            (true, false) => dir += Vec3::Z,
            (false, true) => dir -= Vec3::Z,
            _ => (),
        }
        match (a, d) {
            (true, false) => dir += Vec3::X,
            (false, true) => dir -= Vec3::X,
            _ => (),
        }

        let motion = dir.normalize() * time.delta_seconds() * 10.0;
        println!("motion: {motion}");

        for (mut tf, _) in cam.iter_mut() {
            tf.translation += motion;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_wasd)
        .run();
}
