/*
Very basic tictactoe game just to help learn the Termion crate.
*/
extern crate termion;

use termion::{
    clear, cursor, event::*, input::{MouseTerminal, TermRead}, raw::IntoRawMode,
};

use std::io::{self, Write};

//Create GameBoard struct for game information
struct GameBoard {
    row1: Vec<u8>,
    row2: Vec<u8>,
    row3: Vec<u8>,

    player: u8,
    winner: u8,
    moves: u8,
}

//GameBoard functions
impl GameBoard {
    //Prints the board
    pub fn print_board(&mut self) {
        //Clear the screen and reset the cursor
        print!("{}{}", clear::All, cursor::Goto(1, 1));

        //Set Row1 characters
        let mut row1_chars: Vec<char> = Vec::new();
        for x in self.row1.iter() {
            match x {
                1 => row1_chars.push('X'),
                2 => row1_chars.push('O'),
                _ => row1_chars.push(' '),
            }
        }

        //Set Row2 characters
        let mut row2_chars: Vec<char> = Vec::new();
        for x in self.row2.iter() {
            match x {
                1 => row2_chars.push('X'),
                2 => row2_chars.push('O'),
                _ => row2_chars.push(' '),
            }
        }

        //Set Row3 characters
        let mut row3_chars: Vec<char> = Vec::new();
        for x in self.row3.iter() {
            match x {
                1 => row3_chars.push('X'),
                2 => row3_chars.push('O'),
                _ => row3_chars.push(' '),
            }
        }

        //Print the board
        println!(
            " {} | {} | {}\r",
            row1_chars[0], row1_chars[1], row1_chars[2]
        );
        println!("---|---|---\r");
        println!(
            " {} | {} | {}\r",
            row2_chars[0], row2_chars[1], row2_chars[2]
        );
        println!("---|---|---\r");
        println!(
            " {} | {} | {}\r",
            row3_chars[0], row3_chars[1], row3_chars[2]
        );
        println!(
            "{}Current player: {}{}\r",
            cursor::Goto(14, 1),
            self.player,
            cursor::Goto(1, 6)
        );
    }

    //Handles a player's move
    pub fn make_move(&mut self, x: u16, y: u16) {
        //Determine the index for the row vectors based on x, and initialize move_made
        let mut move_made: bool = false;
        let index = (0.25 * (x as f32)) - 0.5;

        //Match y against valid rows
        //If the row is empty, set it to the current player and flip move_made
        match y {
            1 => {
                if self.space_is_empty(x.clone(), y.clone()) {
                    self.row1[index as usize] = self.player;
                    move_made = true;
                }
            }
            3 => {
                if self.space_is_empty(x.clone(), y.clone()) {
                    self.row2[index as usize] = self.player;
                    move_made = true;
                }
            }
            5 => {
                if self.space_is_empty(x.clone(), y.clone()) {
                    self.row3[index as usize] = self.player;
                    move_made = true;
                }
            }
            _ => {}
        }

        //If a move was made, check for winning players if moves surpass 5.
        if move_made {
            if self.check_winner() & (self.moves >= 5) {
                self.winner = self.player;
            }

            //Switch players
            self.player = match self.player {
                1 => 2,
                2 => 1,
                _ => 0,
            };

            //Increment the move counter and reprint the board
            self.moves += 1;
            self.print_board();
        }
    }

    //Function to check if a space is empty
    pub fn space_is_empty(&mut self, x: u16, y: u16) -> bool {
        //Determine the index for the row vectors
        let index = (0.25 * (x as f32)) - 0.5;

        //If the index is out of the range, do nothing.  Otherwise, check spaces
        if (index < 0.0) | (index > 2.0) {
            false
        } else {
            //If a space is empty, return true.  Otherwise, return false
            match y {
                1 => {
                    if self.row1[index as usize] == 0 {
                        true
                    } else {
                        false
                    }
                }
                3 => {
                    if self.row2[index as usize] == 0 {
                        true
                    } else {
                        false
                    }
                }
                5 => {
                    if self.row3[index as usize] == 0 {
                        true
                    } else {
                        false
                    }
                }
                _ => false
            }
        }
    }

    //Checks for a winning player
    pub fn check_winner(&mut self) -> bool {
        //Create extra vectors and make a duplicate of the rows for safety
        let row1: Vec<u8> = self.row1.clone();
        let row2: Vec<u8> = self.row2.clone();
        let row3: Vec<u8> = self.row3.clone();
        let col1: Vec<u8> = vec![self.row1[0], self.row2[0], self.row3[0]];
        let col2: Vec<u8> = vec![self.row1[1], self.row2[1], self.row3[1]];
        let col3: Vec<u8> = vec![self.row1[2], self.row2[2], self.row3[2]];
        let dia1: Vec<u8> = vec![self.row1[0], self.row2[1], self.row3[2]];
        let dia2: Vec<u8> = vec![self.row3[0], self.row2[1], self.row1[2]];

        //If the vectors contain a winning player, return true.  Otherwise, return false
        if (self.check_vector(row1)) | (self.check_vector(row2)) | (self.check_vector(row3)) {
            true
        } else if (self.check_vector(col1)) | (self.check_vector(col2)) | (self.check_vector(col3))
        {
            true
        } else if (self.check_vector(dia1)) | (self.check_vector(dia2)) {
            true
        } else {
            false
        }
    }

    //Function to check if all vector items are the same, except if they are all 0
    pub fn check_vector(&mut self, check: Vec<u8>) -> bool {
        //Get the sum of the items of a vector
        let sum: u8 = check.iter().sum();

        //If the sum is not 0, but is either 3 or 6, return true.  False if the vector is not entirely the same
        if (check[0] == check[1]) && (check[0] == check[2]) && (sum != 0) {
            true
        } else {
            false
        }
    }
}

fn main() {
    //Define the gameboard for a 3x3 tictactoe game
    let mut game = GameBoard {
        row1: vec![0, 0, 0],
        row2: vec![0, 0, 0],
        row3: vec![0, 0, 0],

        player: 1,
        winner: 1,
        moves: 0,
    };

    //Setup the terminal to handle events and printing
    let stdin = io::stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    //Print the board
    game.print_board();

    //Handles Terminal events
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    game.make_move(x, y);
                    write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
                }
                MouseEvent::Hold(_, _) | MouseEvent::Release(_, _) => {}
            },
            _ => {}
        }

        //If there is a winner and moves are >= 5, print the winner then quit
        if game.check_winner() & (game.moves >= 5) {
            write!(
                stdout,
                "{}Player {} wins the game!\r\n",
                cursor::Goto(1, 7),
                game.winner
            ).unwrap();
            break;
        }

        //If there is no winner and the board is full, print a draw statement then quit
        if !game.check_winner() & (game.moves >= 9) {
            write!(stdout, "{}It's a draw!\r\n", cursor::Goto(1, 7)).unwrap();
            break;
        }

        stdout.flush().unwrap();
    }
}
