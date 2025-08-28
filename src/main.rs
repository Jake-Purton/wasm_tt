mod debug;
mod websocket;
mod boards;

use websocket::init_ws;

use bevy::{
    image::ImageSamplerDescriptor,
    prelude::*,
    window::WindowResolution,
};
use wasm_bindgen::prelude::*;
use std::sync::Mutex;

use crate::{boards::{Board, Cell, OpponentBoard, OpponentCell}, websocket::WebSocketPlugin};

const BOMB_COUNT: u32 = 40;
const BOARD_W: usize = 16;
const BOARD_H: usize = 16;
const PIXELS_PER_CELL: usize = 50;
const MARGIN: f32 = 10.0;

// the mutex that will allow js and rust to communicate 
static TEXT_VALUE: Mutex<Option<String>> = Mutex::new(None);

#[wasm_bindgen]
pub fn set_textbox_value(val: String) {
    console_log!("{}", val);
    *TEXT_VALUE.lock().unwrap() = Some(val.clone());
}

#[derive(Resource)]
pub struct BoardLocked(bool);

pub fn main() {

    init_ws();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(((BOARD_W * PIXELS_PER_CELL) * 2) as f32 + MARGIN, (BOARD_W * PIXELS_PER_CELL) as f32 ),
                        title: "Minesweeper".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
            WebSocketPlugin,
        ))
        .insert_resource(Board::new())
        .insert_resource(OpponentBoard::new())
        .insert_resource(BoardLocked(true))
        .add_systems(Startup, setup)
        .add_systems(Update, (click_cell, update_cells, update_debug, update_opponent_cells))
        .run();
}

#[derive(Component)]
struct DebugText;



fn setup(mut commands: Commands) {

    commands.spawn((
        Text2d::new("Jake is the Best"),
        TextFont {
            font_size: 16.0,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0),
        DebugText,
    ));

    // spawn the board on the left

    for x in 0..BOARD_W {
        for y in 0..BOARD_H {
            let pos_x = x as f32 * PIXELS_PER_CELL as f32 + PIXELS_PER_CELL as f32 / 2.0
                - (BOARD_W as f32 * PIXELS_PER_CELL as f32) / 2.0;
            let pos_y = y as f32 * PIXELS_PER_CELL as f32 + PIXELS_PER_CELL as f32 / 2.0
                - (BOARD_H as f32 * PIXELS_PER_CELL as f32) / 2.0;
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.86, 0.86, 0.86),
                    custom_size: Some(Vec2::splat(PIXELS_PER_CELL as f32)),
                    ..Default::default()
                },
                Transform::from_xyz(pos_x - (PIXELS_PER_CELL * BOARD_W / 2) as f32 - MARGIN / 2.0, pos_y, 1.0),
                Cell { x, y },
                children![
                    Text2d::new("X"),
                    TextFont {
                        font_size: 16.0,
                        ..Default::default()
                    },
                    Transform::from_xyz(0.0, 0.0, 2.0), // slightly above the cell
                ]
            ));
        }
    }

    // spawn the board on the right 

    for x in 0..BOARD_W {
        for y in 0..BOARD_H {
            let pos_x = x as f32 * PIXELS_PER_CELL as f32 + PIXELS_PER_CELL as f32 / 2.0
                - (BOARD_W as f32 * PIXELS_PER_CELL as f32) / 2.0;
            let pos_y = y as f32 * PIXELS_PER_CELL as f32 + PIXELS_PER_CELL as f32 / 2.0
                - (BOARD_H as f32 * PIXELS_PER_CELL as f32) / 2.0;
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.86, 0.86, 0.86),
                    custom_size: Some(Vec2::splat(PIXELS_PER_CELL as f32)),
                    ..Default::default()
                },
                Transform::from_xyz(pos_x + (PIXELS_PER_CELL * BOARD_W / 2) as f32 + MARGIN / 2.0, pos_y, 1.0),
                OpponentCell { x, y },
                children![
                    Text2d::new("X"),
                    TextFont {
                        font_size: 16.0,
                        ..Default::default()
                    },
                    Transform::from_xyz(0.0, 0.0, 2.0), // slightly above the cell
                ]
            ));
        }
    }

    commands.spawn(Camera2d::default());
}

fn update_cells (
    mut text_query: Query<&mut Text2d>,
    children_query: Query<&Children>,
    mut cells: Query<(Entity, &mut Sprite, &Cell)>,
    board: Res<Board>
) {

    for (e, mut sprite, cell) in cells.iter_mut() {
        
        if let Ok(children) = children_query.get(e) {
            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = board.get_text(cell.x, cell.y); // example
                }
            }
        }

        sprite.color = board.get_colour(cell.x, cell.y)
    }
}

fn update_opponent_cells (
    mut text_query: Query<&mut Text2d>,
    children_query: Query<&Children>,
    mut cells: Query<(Entity, &mut Sprite, &OpponentCell)>,
    board: Res<OpponentBoard>
) {

    for (e, mut sprite, cell) in cells.iter_mut() {
        
        if let Ok(children) = children_query.get(e) {
            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = board.get_text(cell.x, cell.y); // example
                }
            }
        }

        sprite.color = board.get_colour(cell.x, cell.y)
    }
}

fn click_cell(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut query: Query<(&Sprite, &Transform, &Cell)>,
    mut board: ResMut<Board>,
    locked_board: Res<BoardLocked>,
) {

    if locked_board.0 {
        return;
    }

    let window = if let Some(window) = windows.iter().next() {
        window
    } else {
        return;
    };

    if !buttons.just_pressed(MouseButton::Left) {
        return
    }

    if let Some(cursor_pos) = window.cursor_position() {
        // Convert cursor position to world position
        let world_x = cursor_pos.x - window.resolution.width() / 2.0;
        let world_y = -(cursor_pos.y - window.resolution.height() / 2.0);

        for (_, transform, cell) in &mut query {
            let cell_pos = transform.translation.truncate();
            let half_size = PIXELS_PER_CELL as f32 / 2.0;
            if (world_x - cell_pos.x).abs() < half_size
                && (world_y - cell_pos.y).abs() < half_size
            {
                board.discover(cell.x, cell.y)
            }
        }

    }
}

fn update_debug (mut q: Query<&mut Text2d, With<DebugText>>) {

    for mut t in q.iter_mut() {
        t.0 = TEXT_VALUE.lock().unwrap().clone().unwrap_or_default();
    }

}