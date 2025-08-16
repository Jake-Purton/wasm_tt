
use core::f32;

use bevy::{prelude::*, window::WindowResolution};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(900.0, 800.0),
                title: "Minesweeper".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, move_cube))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((Sprite {
            color: Color::srgb(0.3, 0.7, 0.9),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        }
    ));
}

fn rotate_cube(mut query: Query<&mut Transform, With<Sprite>>, dt: Res<Time>) {
    let delta = dt.delta().as_secs_f32();

    for mut transform in &mut query {
        transform.rotate(Quat::from_rotation_z(f32::consts::FRAC_PI_2 * delta));
    }
}

fn move_cube(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Sprite>>, dt: Res<Time>) {

    let delta = dt.delta().as_secs_f32();

    if keyboard.pressed(KeyCode::KeyD) {
        for mut transform in query.iter_mut() {
            transform.translation.x += 300.0 * delta
        }
    }

    if keyboard.pressed(KeyCode::KeyA) {
        for mut transform in query.iter_mut() {
            transform.translation.x -= 300.0 * delta
        }
    }

    if keyboard.pressed(KeyCode::KeyW) {
        for mut transform in query.iter_mut() {
            transform.translation.y += 300.0 * delta
        }
    }

    if keyboard.pressed(KeyCode::KeyS) {
        for mut transform in query.iter_mut() {
            transform.translation.y -= 300.0 * delta
        }
    }
}