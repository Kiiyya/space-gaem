#![allow(clippy::type_complexity)]
use bevy::prelude::*;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
struct Body {
    mass: f32,
}

#[derive(Component)]
struct Star {
    color: Color,
    luminosity: f32,
    radius: f32,
}

#[derive(Component)]
struct Planet {
    color: Color,
    radius: f32,
}

/// List of entities which affect this entity gravitationally.
#[derive(Component, Default)]
struct Gravity {
    affectors: Vec<Entity>,
}

fn body_gravities(
    time: Res<Time>,
    mut query: Query<(&Body, &Gravity, &mut Velocity, &Transform)>,
) {
    for (body, grav, vel, tf) in query.iter() {
        let mut force = Vec3::ZERO;

        // collect the force exerted on this entity by any entity in our affectors.
        for ent in grav.affectors.iter() {
            if let Ok((body2, _, _, tf2)) = query.get(*ent) {
                let dist = tf2.translation - tf.translation;
                if let Some(dist) = dist.try_normalize() {
                    let magnitude = body.mass * body2.mass / dist.length_squared();
                    force += magnitude * dist;
                }
            }
        }

        vel.0 += force * time.delta_seconds();
    }
}

/// Applies velocity.
fn body_movement(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (vel, mut tf) in query.iter_mut() {
        tf.translation += time.delta_seconds() * vel.0;
        println!("Body position: {}", tf.translation);
    }
}


fn setup_bodies_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Star, Option<&Transform>), Added<Star>>,
) {
    for (ent, star, tf) in query.iter() {
		// let sphere_handle = meshes.add(Mesh::from(shape::Cube { size: star.radius }));
        let sphere_handle = meshes.add(Mesh::from(shape::Icosphere{ radius: star.radius, ..Default::default() }));
        let mat_handle = materials.add(StandardMaterial {
            base_color: star.color,
            ..Default::default()
        });
        commands.entity(ent)
            .insert_bundle(PbrBundle {
                mesh: sphere_handle,
                material: mat_handle,
                ..Default::default()
            })
            .insert_bundle(PointLightBundle {
            transform: tf.copied().unwrap_or_default(),
                point_light: PointLight {
                    intensity: star.luminosity,
                    color: star.color,
                    shadows_enabled: true,
                    ..Default::default()
                },
                ..Default::default()
            });
    }
}

fn setup_bodies_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Planet, Option<&Transform>), Added<Planet>>,
) {
    for (ent, planet, tf) in query.iter() {
        // let sphere_handle = meshes.add(Mesh::from(shape::Cube { size: planet.radius }));
        let sphere_handle = meshes.add(Mesh::from(shape::Icosphere{ radius: planet.radius, ..Default::default() }));
        let mat_handle = materials.add(StandardMaterial {
            base_color: planet.color,
            ..Default::default()
        });

        commands.entity(ent).insert_bundle(PbrBundle {
            mesh: sphere_handle,
            material: mat_handle,
            transform: tf.copied().unwrap_or_default(),
            ..Default::default()
        });
    }
}

// fn orbits(
//     time: Res<Time>,
//     mut query: Query<(&Body, &mut Transform, &Rotating)>,
// ) {
//     for (_body, mut tf, rot) in query.iter_mut() {
//         tf.rotate(Quat::from_rotation_y(time.delta_seconds() * rot.period * 6.0));
//     }
// }

fn setup(mut commands: Commands) {
    let sun = commands
        .spawn()
        .insert(Star { color: Color::YELLOW, luminosity: 1000.0, radius: 1.0 })
        .insert(Body { mass: 1.0 })
        .insert(Velocity(Vec3::ZERO))
        .insert(Gravity { affectors: vec![] }) // nothing affects the sun
        .id();

    commands.spawn()
        .insert(Body { mass: 0.1 })
        .insert(Velocity(1.2 * Vec3::Z))
        .insert(Planet { color: Color::BLUE, radius: 0.2 })
        .insert(Gravity { affectors: vec![sun] })
        .insert(Transform::from_xyz(2.0, 0.0, 0.0));

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        point_light: PointLight {
            intensity: 100.0,
            ..Default::default()
        },
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

        let motion = dir.normalize() * time.delta_seconds() * 3.0;

        for (mut tf, _) in cam.iter_mut() {
            tf.translation += motion;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_wasd)

        // .add_system(orbits)
        .add_system(setup_bodies_planets)
        .add_system(setup_bodies_stars)
        .add_system(body_movement)
        .add_system(body_gravities)

        .run();
}
