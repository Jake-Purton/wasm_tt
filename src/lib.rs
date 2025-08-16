
use core::f32;
use bevy::{
    asset::RenderAssetUsages, image::ImageSamplerDescriptor, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, window::WindowResolution
};
use wasm_bindgen::prelude::*;
use rand::{rngs::SmallRng, SeedableRng, Rng};

const BOMB_COUNT: u32 = 40;
const BOARD_W: usize = 16;
const BOARD_H: usize = 16;
const PIXELS_PER_CELL: usize = 50;

#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800.0, 800.0),
                    title: "Minesweeper".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            }),
        )
        .insert_resource(Board::new())
        .add_systems(Startup, setup)
        // .add_systems(Update, (rotate_cube, move_cube))
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

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {

    let width = (BOARD_W * PIXELS_PER_CELL) as u32;
    let height = (BOARD_H * PIXELS_PER_CELL) as u32;
    let pixel_count = (width * height) as usize;
    // RGBA8: 4 bytes per pixel
    let mut image_data = vec![255u8; pixel_count * 4];

    for y in 0..height {
        for x in 0..width {
            let cell_x = x as usize / PIXELS_PER_CELL;
            let cell_y = y as usize / PIXELS_PER_CELL;
            let i = ((y * width + x) * 4) as usize;

            let is_light = (cell_x + cell_y) % 2 == 0;
            let color = if is_light {
                [220, 220, 220, 255]
            } else {
                [180, 180, 180, 255]
            };

            image_data[i..i + 4].copy_from_slice(&color);
        }
    }

    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image_data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image = images.add(image);
    // let id = image.id();

    let sprite = Sprite::from_image(image);

    commands.spawn(sprite);

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