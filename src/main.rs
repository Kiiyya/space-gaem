#![allow(clippy::type_complexity)]
use bevy::prelude::*;
use player::{PlayerPlugin, Player};
use rand::Rng;

mod player;

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
struct FarStar {
    color: Color,
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
    query_affectors: Query<(&Body, &Transform)>,
) {
    for (body, grav, mut vel, tf) in query.iter_mut() {
        let mut force = Vec3::ZERO;

        // collect the force exerted on this entity by any entity in our affectors.
        for ent in grav.affectors.iter() {
            if let Ok((body2, tf2)) = query_affectors.get(*ent) {
                let dist = tf2.translation - tf.translation;
                if let Some(dist) = dist.try_normalize() {
                    let magnitude = body.mass * body2.mass / dist.length_squared();
                    force += magnitude * dist;
                }
            }
        }

        vel.0 += 0.1 * force * time.delta_seconds() / body.mass;
    }
}

/// Applies velocity.
fn body_movement(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (vel, mut tf) in query.iter_mut() {
        tf.translation += time.delta_seconds() * vel.0;
        // println!("Body position: {}", tf.translation);
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
        let sphere_handle = meshes.add(Mesh::from(shape::Icosphere{ radius: star.radius, subdivisions: 5 }));
        let mat_handle = materials.add(StandardMaterial {
            base_color: star.color,
            emissive: star.color,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
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
                    radius: star.radius,
                    range: 100000000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
    }
}

fn setup_bodies_far_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &FarStar, Option<&Transform>), Added<FarStar>>,
) {
    for (ent, star, tf) in query.iter() {
		// let sphere_handle = meshes.add(Mesh::from(shape::Cube { size: star.radius }));
        let sphere_handle = meshes.add(Mesh::from(shape::Icosphere{ radius: star.radius, subdivisions: 0 }));
        let mat_handle = materials.add(StandardMaterial {
            base_color: star.color,
            emissive: star.color,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..Default::default()
        });
        commands.entity(ent)
            .insert_bundle(PbrBundle {
                mesh: sphere_handle,
                material: mat_handle,
                transform: tf.copied().unwrap_or_default(),
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
            metallic: 0.1,
            reflectance: 0.3,
            perceptual_roughness: 0.7,
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

fn setup(mut commands: Commands) {



    let star1_id = commands.spawn()
        .insert(Star { color: Color::rgb(1.0, 0.3, 0.0), luminosity: 100000.0, radius: 6.0 })
        .insert(Body { mass: 100.0 })
        .insert(Velocity(5.0 * Vec3::Z))
        // .insert(Gravity { affectors: vec![] })
        .insert(Transform::from_xyz(-10.0, 0.0, 0.0))
        .id();

    let star2_id = commands.spawn()
        .insert(Star { color: Color::rgb(1.0, 0.1, 0.0), luminosity: 60000.0, radius: 4.0 })
        .insert(Body { mass: 50.0 })
        .insert(Velocity(-10.0 * Vec3::Z))
        .insert(Gravity { affectors: vec![star1_id] })
        .insert(Transform::from_xyz(10.0, 0.0, 0.0))
        .id();

    // the earth
    let earth_id = commands.spawn()
        .insert(Body { mass: 4.0 })
        .insert(Velocity(40.0 * Vec3::Z))
        .insert(Planet { color: Color::BLUE, radius: 1.5 })
        .insert(Gravity { affectors: vec![star1_id, star2_id] })
        .insert(Transform::from_xyz(50.0, 0.0, 0.0))
        .id();

    // jupiter
    let jupiter_id = commands.spawn()
        .insert(Body { mass: 15.0 })
        .insert(Velocity(40.0 * Vec3::Z))
        .insert(Planet { color: Color::ORANGE, radius: 2.0 })
        .insert(Gravity { affectors: vec![star1_id, star2_id] })
        .insert(Transform::from_xyz(100.0, 0.0, 0.0))
        .id();

    // juptiter moon
    let jupiter_moon_id = commands.spawn()
        .insert(Body { mass: 0.3 })
        .insert(Velocity(43.0 * Vec3::Z))
        .insert(Planet {color: Color::CYAN, radius: 1.0 })
        .insert(Gravity { affectors: vec![star1_id, star2_id, jupiter_id]})
        .insert(Transform::from_xyz(104.0, 0.0, 0.0))
        .id();

    // pluto
    let pluto_id = commands.spawn()
        .insert(Body { mass: 0.01 })
        .insert(Velocity(10.0 * Vec3::Z))
        .insert(Planet { color: Color::GRAY, radius: 1.0 })
        .insert(Gravity { affectors: vec![star1_id, star2_id] })
        .insert(Transform::from_xyz(150.0, 0.0, 0.0))
        .id();

    commands.entity(star1_id)
        .insert(Gravity { affectors: vec![star2_id] });


    // // light
    // commands.spawn_bundle(PointLightBundle {
    //     transform: Transform::from_xyz(0.0, 5.0, 0.0),
    //     point_light: PointLight {
    //         intensity: 100.0,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    let mut rng = rand::thread_rng();

    for _ in 0..5000 {
        commands.spawn()
            .insert(FarStar { color: Color::WHITE, radius: 2.0 })
            .insert(Transform::from_xyz(
                gen_sign() * rng.gen_range(100.0 .. 500.0),
                gen_sign() * rng.gen_range(100.0 .. 500.0),
                gen_sign() * rng.gen_range(100.0 .. 500.0),
            ));
    }

    // Player & Camera
    commands
		.spawn_bundle(PerspectiveCameraBundle {
			transform: Transform::from_xyz(94.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
			..Default::default()
		})
		.insert(Player)
        .insert(Velocity(35.0 * Vec3::Z))
        .insert(Body { mass: 0.0000001 })
		.insert(Gravity { affectors: vec![star1_id, star2_id, earth_id, jupiter_id, jupiter_moon_id, pluto_id] });
}

fn gen_sign() -> f32 {
    let mut rng = rand::thread_rng();
    if rng.gen() {
        -1.0
    } else {
        1.0
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)

        // .add_plugin(ConfigCam)
        // .insert_resource(MovementSettings { dist: 5.0, ..Default::default() })
        // .insert_resource(PlayerSettings {
        //     pos: Vec3::new(0.0, 300.0, 0.0),
        // })

        .add_startup_system(setup)

        // .add_system(orbits)
        .add_system(setup_bodies_planets)
        .add_system(setup_bodies_stars)
        .add_system(setup_bodies_far_stars)
        .add_system(body_movement)
        .add_system(body_gravities)

        .run();
}
