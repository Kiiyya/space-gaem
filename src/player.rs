use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::Velocity;

#[derive(Component)]
pub struct Player;

fn camera_wasd(
    input: Res<Input<KeyCode>>,
    mut cam: Query<(&Transform, &Player, &mut Velocity)>,
    time: Res<Time>,
) {
    let w = input.pressed(KeyCode::W);
    let a = input.pressed(KeyCode::A);
    let s = input.pressed(KeyCode::S);
    let d = input.pressed(KeyCode::D);
    let lshift = input.pressed(KeyCode::LShift);
    let space = input.pressed(KeyCode::Space);
    if (w != s) || (a != d) || (lshift != space) {
		let mut dir = Vec3::ZERO;
        match (w, s) { // forward/backward
            (true, false) => dir -= Vec3::Z,
            (false, true) => dir += Vec3::Z,
            _ => (),
        }
        match (a, d) { // left/right
            (true, false) => dir -= Vec3::X,
            (false, true) => dir += Vec3::X,
            _ => (),
        }
		match (space, lshift) { // up/down
            (true, false) => dir += Vec3::Y,
            (false, true) => dir -= Vec3::Y,
            _ => (),
		}

        let dir = dir.normalize_or_zero() * time.delta_seconds() * 30.0;

        for (tf, _, mut vel) in cam.iter_mut() {
			vel.0 += dbg!(tf.rotation * dir);

            // tf.translation += motion;
        }
    }
}

fn camera_look(
	windows: Res<Windows>,
	mut motion: EventReader<MouseMotion>,
	// motion: Res<Events<MouseMotion>>,
	mut query: Query<(&mut Transform, &Player)>,
) {
	let window = windows.get_primary().unwrap();
	if !window.cursor_locked() {
		return;
	}

	for (mut tf, _player) in query.iter_mut() {
		for ev in motion.iter() {
			println!("ev.delta: {}", ev.delta);
			const SCALE: f32 = -0.001;
			let q = Quat::from_euler(EulerRot::XYZ, SCALE * ev.delta.y, SCALE * ev.delta.x, 0.0);
			tf.rotation *= q;
			// Quat
		}
	}
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) || key.just_pressed(KeyCode::LAlt) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

fn setup(_commands: Commands) { }

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(setup)
			.add_system(cursor_grab_system)
			.add_system(camera_look)
			.add_system(camera_wasd);
	}
}
