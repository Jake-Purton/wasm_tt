use bevy::prelude::*;

use crate::{console_log, BOARD_H, BOARD_W};

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
    board: Option<[[u8; BOARD_H]; BOARD_W]>,
    // bombs: u8,
}

impl OpponentBoard {
    pub fn new() -> Self {
        // let board: [[u8; 16]; 16] = [[0; BOARD_H]; BOARD_W];
        Self { board: None }
    }

    pub fn discover (&mut self, x: usize, y: usize) {

        let board = match self.board.as_mut() {
            Some(b) => b,
            None => {
                console_log!("SOMETHING WRONG BOARD IS NONE");
                return;
            }
        };
        
        // if already discovered ignore
        if board[x][y] & 0b0001_0000 > 0 {
            return;
        }

        board[x][y] |= 0b0001_0000; // mark as discovered

        if board[x][y] & 0b0000_1111 == 0 {
            // discover all the cells around it too
            let x = x as isize;
            let y = y as isize;

            for i in x-1..=x+1 {
                for j in y-1..=y+1 {
                    if i < 0 || j < 0 || i >= BOARD_W as isize || j >= BOARD_H as isize {
                        continue;
                    }
                    self.discover(i as usize, j as usize);
                }
            }
        }

    }

    pub fn start (&mut self, bombs: Vec<u8>) {

        let mut board: [[u8; 16]; 16] = [[0; BOARD_H]; BOARD_W];
        let vec: Vec<(usize, usize)> = bombs
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as usize, chunk[1] as usize))
            .collect();


        for i in vec {
            board[i.0][i.1] = 0b0010_0000;
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
        self.board = Some(board);
    }

    pub fn get_text (&self, x: usize, y: usize) -> String {

        if let Some(board) = self.board {
            if board[x][y] & 0b0001_0000 > 0 {
                if board[x][y] & 0b0000_1111 > 0 {
                    return (board[x][y] & 0b0000_1111).to_string();
                }
            }
        }

        return "".into();
    }

    pub fn get_colour (&self, x: usize, y: usize) -> Color {

        let mut offset = -0.1;

        if (x + y) % 2 == 0 {
            offset = 0.1
        }

        if let Some(board) = self.board {
            if board[x][y] & 0b0001_0000 > 0 {
                if board[x][y] & 0b0010_0000 > 0 {
                    return Color::srgb(0.9 + offset, 0.3 + offset, 0.3 + offset)
                }
    
                return Color::srgb(0.5 + offset , 0.3 + offset , 0.6 + offset)
            }
        }



        return Color::srgb(0.6 + offset , 0.4 + offset , 0.4 + offset)
    }
}

// the local board
#[derive(Resource)]
pub struct Board {
    board: Option<[[u8; BOARD_H]; BOARD_W]>,
    pub locked: bool,
    bombs: u8,
    // 00X0_XXXX = not yet clicked
    // 00X1_XXXX = already discovered
    // 001X_XXXX = bomb
    // 0-9 = bomb nearby

}

impl Board {
    pub fn start (&mut self, bombs: Vec<u8>) {
        console_log!("Board starting");

        let mut board: [[u8; 16]; 16] = [[0; BOARD_H]; BOARD_W];
        let vec: Vec<(usize, usize)> = bombs
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as usize, chunk[1] as usize))
            .collect();


        for i in vec {
            board[i.0][i.1] = 0b0010_0000;
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
        // Pretty print the board
        for y in 0..BOARD_H {
            let mut row = String::new();
            for x in 0..BOARD_W {
            let cell = board[x][y];
            let ch = if cell & 0b0010_0000 > 0 {
                'B' // Bomb
            } else {
                let num = cell & 0b0000_1111;
                if num > 0 {
                std::char::from_digit(num as u32, 10).unwrap_or(' ')
                } else {
                '.' // Discovered empty
                }
            };

            row.push(ch);
            row.push(' ');
            }
            console_log!("{}", row);
        }

        self.board = Some(board);
    }

    pub fn is_locked(&self) -> bool {
        return self.locked;
    }

    pub fn new() -> Self {
        Self { board: None, bombs: 0, locked: true }
    }

    pub fn get_text (&self, x: usize, y: usize) -> String {

        if let Some(board) = self.board {
            if board[x][y] & 0b0001_0000 > 0 {
                if board[x][y] & 0b0000_1111 > 0 {
                    return (board[x][y] & 0b0000_1111).to_string();
                }
            }
        }

        return "".into();
    }

    pub fn discover (&mut self, x: usize, y: usize) {

        let board = match self.board.as_mut() {
            Some(b) => b,
            None => {
                console_log!("SOMETHING WRONG BOARD IS NONE");
                return;
            }
        };
        
        // if already discovered ignore
        if board[x][y] & 0b0001_0000 > 0 {
            return;
        }

        // if it is a bomb increase the bomb count
        if board[x][y] & 0b0010_0000 > 0 {
            self.bombs += 1;
        }

        board[x][y] |= 0b0001_0000; // mark as discovered

        if board[x][y] & 0b0000_1111 == 0 {
            // discover all the cells around it too
            let x = x as isize;
            let y = y as isize;

            for i in x-1..=x+1 {
                for j in y-1..=y+1 {
                    if i < 0 || j < 0 || i >= BOARD_W as isize || j >= BOARD_H as isize {
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

        if let Some(board) = self.board {
            if board[x][y] & 0b0001_0000 > 0 {
                if board[x][y] & 0b0010_0000 > 0 {
                    return Color::srgb(0.9 + offset, 0.3 + offset, 0.3 + offset)
                }
    
                return Color::srgb(0.5 + offset , 0.3 + offset , 0.6 + offset)
            }
        }

        return Color::srgb(0.3 + offset , 0.7 + offset , 0.4 + offset)
    }
}