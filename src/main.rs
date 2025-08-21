mod debug;

use debug::log;

use bevy::{
    image::ImageSamplerDescriptor,
    prelude::*,
    window::WindowResolution,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use wasm_bindgen::prelude::*;
use std::sync::Mutex;

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

pub fn main() {

    console_log!("hello jakey");

    App::new()
        .add_plugins(
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
        )
        .insert_resource(Board::new())
        .insert_resource(OpponentBoard::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (click_cell, update_cells))
        .run();
}

#[derive(Component)]
struct DebugText;

#[derive(Component)]
struct OpponentCell {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Resource)]
struct OpponentBoard {
    board: [[u8; BOARD_H]; BOARD_W],
    bombs: u8,
}

impl OpponentBoard {
    pub fn new() -> Self {
        let board: [[u8; 16]; 16] = [[0; BOARD_H]; BOARD_W];
        Self { board, bombs: 0 }
    }
}

// the local board
#[derive(Resource)]
struct Board {
    board: [[u8; BOARD_H]; BOARD_W],
    bombs: u8,
    // 00X0_XXXX = not yet clicked
    // 00X1_XXXX = already discovered
    // 001X_XXXX = bomb
    // 0-9 = bomb nearby

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

            board[x][y] = 0b0010_0000;

            vec.remove(i);
        }

        for x in 0..BOARD_W {
            for y in 0..BOARD_H {
                let mut bomb_count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < BOARD_W as isize && ny >= 0 && ny < BOARD_H as isize {
                            let possible_bomb = board[nx as usize][ny as usize];
                            if (possible_bomb & 0b0010_0000) > 0 {
                                bomb_count += 1;
                            }
                        }
                    }
                }

                board[x][y] += bomb_count as u8;
            }
        }

        println!("{:?}", board);

        Self { board, bombs: 0 }
    }

    fn get_text (&self, x: usize, y: usize) -> String {
        if self.board[x][y] & 0b0001_0000 > 0 {
            if self.board[x][y] & 0b0000_1111 > 0 {
                return (self.board[x][y] & 0b0000_1111).to_string();
            }
        }

        return "".into();
    }

    fn discover (&mut self, x: usize, y: usize) {
        
        // if already discovered ignore
        if self.board[x][y] & 0b0001_0000 > 0 {
            return;
        }
        
        // if it is a bomb increase the bomb count
        if self.board[x][y] & 0b0010_0000 > 0 {
            self.bombs += 1;
        }
        
        self.board[x][y] |= 0b0001_0000; // mark as discovered

        if self.board[x][y] & 0b0000_1111 == 0 {
            // discover all the cells around it too

            let x = x as isize;
            let y = y as isize;

            for i in x-1..=x+1 {
                for j in y-1..=y+1 {
                    if i < 0 || j < 0 || i >= BOARD_W as isize|| j >= BOARD_H as isize {
                        continue;
                    }

                    self.discover(i as usize, j as usize);
                    
                }
            }
        }

    }

    fn get_colour (&self, x: usize, y: usize) -> Color {

        let mut offset = -0.1;

        if (x + y) % 2 == 0 {
            offset = 0.1
        }

        if self.board[x][y] & 0b0001_0000 > 0 {
            if self.board[x][y] & 0b0010_0000 > 0 {
                return Color::srgb(0.9 + offset, 0.3 + offset, 0.3 + offset)
            }

            return Color::srgb(0.5 + offset , 0.3 + offset , 0.6 + offset)
        }

        return Color::srgb(0.3 + offset , 0.7 + offset , 0.4 + offset)
    }
}

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

fn click_cell(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut query: Query<(&Sprite, &Transform, &Cell)>,
    mut board: ResMut<Board>
) {
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