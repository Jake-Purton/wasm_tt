use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{BOARD_H, BOARD_W, BOMB_COUNT};

#[derive(Component)]
pub struct OpponentCell {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct OpponentBoard {
    board: [[u8; BOARD_H]; BOARD_W],
    // bombs: u8,
}

impl OpponentBoard {
    pub fn new() -> Self {
        let board: [[u8; 16]; 16] = [[0; BOARD_H]; BOARD_W];
        Self { board, /*bombs: 0*/ }
    }

    pub fn get_text (&self, x: usize, y: usize) -> String {
        if self.board[x][y] & 0b0001_0000 > 0 {
            if self.board[x][y] & 0b0000_1111 > 0 {
                return (self.board[x][y] & 0b0000_1111).to_string();
            }
        }

        return "".into();
    }

    pub fn get_colour (&self, x: usize, y: usize) -> Color {

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

        return Color::srgb(0.6 + offset , 0.4 + offset , 0.4 + offset)
    }
}

// the local board
#[derive(Resource)]
pub struct Board {
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

        Self { board, bombs: 0 }
    }

    pub fn get_text (&self, x: usize, y: usize) -> String {
        if self.board[x][y] & 0b0001_0000 > 0 {
            if self.board[x][y] & 0b0000_1111 > 0 {
                return (self.board[x][y] & 0b0000_1111).to_string();
            }
        }

        return "".into();
    }

    pub fn discover (&mut self, x: usize, y: usize) {
        
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

    pub fn get_colour (&self, x: usize, y: usize) -> Color {

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