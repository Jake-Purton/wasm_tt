
use core::f32;
use bevy::{prelude::*, window::WindowResolution};
use wasm_bindgen::prelude::*;
use rand::{rngs::SmallRng, SeedableRng, Rng};

const BOMB_COUNT: u32 = 40;
const BOARD_W: usize = 16;
const BOARD_H: usize = 16;

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
        .insert_resource(Board::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, move_cube))
        .run();
}

#[derive(Resource)]
struct Board {
    board: [[u8; BOARD_H]; BOARD_W],
}

impl Board {
    pub fn new() -> Self {
        
        let mut board: [[u8; 16]; 16] = [[0; BOARD_H]; BOARD_W];
        let mut vec: Vec<(usize, usize)> = Vec::new();

        for x in 0..BOARD_W {
            for y in 0..BOARD_H {
                vec.push((x, y));
            }
        }

        let mut rng = SmallRng::from_entropy();

        for _ in 0..BOMB_COUNT {

            let i = rng.gen_range(0..vec.len());
            
            let (x, y) = vec[i];

            board[x][y] = 1;

            vec.remove(i);
        }

        Self { board }

    }
}

fn setup(mut commands: Commands) {

    let mut rng = SmallRng::from_entropy();

    // Example: randomize cube position
    let x = rng.gen_range(-200.0..200.0);
    let y = rng.gen_range(-200.0..200.0);

    commands.spawn(Camera2d::default());
    commands.spawn((Sprite {
            color: Color::srgb(0.3, 0.7, 0.9),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(x, y, 1.0),
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