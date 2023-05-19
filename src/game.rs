use std::{eprintln, println};

use rand::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Sign {
    O,
    X,
}

pub struct Game {
    player: Sign,
    ai: Sign,
    moves: Sign,
    winner: Option<Sign>,
    state: [[Option<Sign>; 3]; 3],
}

impl Game {
    pub fn new(player_sign: Option<Sign>) -> Self {
        let player = player_sign.unwrap_or(Sign::O);
        let ai = if player == Sign::O { Sign::X } else { Sign::O };

        Game {
            player,
            ai,
            winner: None,
            moves: Sign::X,
            state: [[None; 3]; 3],
        }
    }

    pub fn start(&mut self) {
        loop {
            self.tick();
        }
    }

    fn next_move(&mut self) {
        self.moves = match &self.moves {
            Sign::X => Sign::O,
            Sign::O => Sign::X,
        };
    }

    fn get_cell(&mut self, id: usize) -> Option<Sign> {
        if id < 1 || id > 9 {
            return None;
        };

        let row = ((id / 3) as f64).ceil() as usize;
        let col = (id - 1) % 3;

        self.state[row][col]
    }

    fn set_cell(&mut self, id: usize, value: Sign) -> Option<()> {
        if id < 1 || id > 9 {
            return None;
        };

        let row = (((id - 1) / 3) as f64).ceil() as usize;

        let col = (id - 1) % 3;

        if self.state[row][col].is_some() {
            println!("Cell is taken");
            return None;
        }

        self.state[row][col] = Some(value);
        Some(())
    }

    fn ai_move(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rnd_num: usize = rng.gen_range(1..9);

        loop {
            if &self.get_cell(rnd_num) == &None {
                break;
            }

            rnd_num = rng.gen_range(1..9);
        }

        while let None = self.set_cell(rng.gen_range(1..9), self.ai) {}
    }

    fn player_move(&mut self, cell: usize) -> Option<()> {
        self.set_cell(cell, self.player)
    }

    fn prompt_player(&mut self) {
        let mut input = String::new();
        println!("Your move (1-9):");

        let answer = std::io::stdin().read_line(&mut input);

        match answer {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed getting user input: {}", e);
            }
        }

        let cell_id: usize = input.trim().parse().expect("Invalid input");

        if let None = &self.player_move(cell_id) {
            println!("Can't select this field.");
            self.prompt_player();
        }
    }

    fn check_horizontal(&self) -> Option<Sign> {
        let mut wins: Option<Sign> = None;

        for row in 0..self.state.len() {
            let last = self.state[row][0];

            if wins.is_some() {
                break;
            }

            for cell in 1..self.state[row].len() {
                if self.state[row][cell] != last {
                    break;
                } else if cell == 2 {
                    wins = last;
                }
            }
        }

        wins
    }

    fn check_vertical(&self) -> Option<Sign> {
        let mut wins: Option<Sign> = None;

        for col in 0..2 {
            let last = self.state[0][col];
            if wins.is_some() {
                break;
            }

            for row in 1..2 {
                if self.state[row][col] != last {
                    break;
                } else if row == 2 {
                    wins = last;
                }
            }
        }

        wins
    }

    fn check_diagonal(&self) -> Option<Sign> {
        let mut last = self.state[0][0];

        if self.state[1][1] == self.state[2][2] && last == self.state[1][1] {
            return last;
        }

        last = self.state[0][2];

        if self.state[1][1] == self.state[2][0] && last == self.state[1][1] {
            return last;
        }

        return None;
    }

    fn check_win(&self) -> Option<Sign> {
        let horizontal = self.check_horizontal();
        let vertical = self.check_vertical();
        let diag = self.check_diagonal();

        let first_some = [horizontal, vertical, diag]
            .iter()
            .find(|x| x.is_some())
            .map(|x| x.unwrap());

        first_some
    }

    fn check_draw(&self) -> bool {
        self.state
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
    }
    fn reset(&mut self) {
        let ai = self.player;

        self.winner = None;
        self.player = self.ai;
        self.ai = ai;
        self.state = [[None; 3]; 3];

        self.draw();
    }

    fn draw(&self) {
        let empty_row: &str = "   |   |   ";
        let divider_row: &str = "___|___|___";
        println!("------ TIC-TAC-TOE ------");
        for row in 0..self.state.len() {
            let row_signs: Vec<String> =
                [self.state[row][0], self.state[row][1], self.state[row][2]]
                    .iter()
                    .map(|x| get_sign(x))
                    .collect::<Vec<String>>();
            println!("{}", empty_row);
            println!(" {} | {} | {} ", row_signs[0], row_signs[1], row_signs[2]);
            if row != self.state.len() - 1 {
                println!("{}", divider_row);
            } else {
                println!("{}", empty_row);
            }
        }
    }

    fn tick(&mut self) {
        self.draw();

        self.check_win();
        if let Some(winner) = self.check_win() {
            self.winner = Some(winner);

            let mut input = String::new();
            let winner_name = if winner == self.player {
                "You"
            } else {
                "Computer"
            };

            println!(
                "Game is over, {} won! Press enter to play again.",
                winner_name
            );

            std::io::stdin().read_line(&mut input).unwrap();

            self.reset();
        } else if self.check_draw() {
            let mut input = String::new();

            println!("Game is over, finished with a DRAW! Press enter to play again.");

            std::io::stdin().read_line(&mut input).unwrap();

            self.reset();
        }

        self.next_move();

        if self.moves == self.player {
            self.prompt_player();
        } else {
            self.ai_move();
        }
    }
}
pub fn get_sign(input: &Option<Sign>) -> String {
    if let Some(v) = input {
        return match v {
            Sign::O => "O".to_string(),
            Sign::X => "X".to_string(),
        };
    }
    " ".to_string()
}
