use colored::ColoredString;
use colored::Colorize;

use std::collections::HashSet;
use std::io;
use std::io::Write;

use crate::game::TurnState;
use crate::game::WinnerState;
use crate::gameboard::BoardState;
use crate::gameboard::PieceState;
use crate::gameboard::X_DIM;
use crate::gameboard::Y_DIM;
use crate::quadri_io_representation::QuadriIORepresentation;
use crate::utils;

static BOARD_SEPARATOR: &'static str = "+---+---+---+---+";
static EMPTY_SPACE: &'static str = " ";
static RGB_WHITE: (u8, u8, u8) = (255, 255, 255);
static RGB_GREY: (u8, u8, u8) = (255, 204, 0);

pub struct QuadriIORepresentationCLI {
    cli_pieces: Vec<CLIGamePiece>,
    board_letters: Vec<String>,
}

struct CLIGamePiece {
    print: String,
}

impl QuadriIORepresentationCLI {
    pub fn new(all_pieces: Vec<PieceState>) -> QuadriIORepresentationCLI {
        let cli_pieces = all_pieces
            .iter()
            .map(|ps| {
                let this_string = format!(
                    "{}",
                    QuadriIORepresentationCLI::get_piece_print(&ps.piece.ats)
                );

                CLIGamePiece { print: this_string }
            })
            .collect();
        let board_letters_result = (1..17).map(|x| utils::num_to_alpha(x)).collect();
        let board_letters = match board_letters_result {
            Ok(letters) => letters,
            Err(error) => panic!("Problem mapping numbers to letters: {:?}", error),
        };
        QuadriIORepresentationCLI {
            cli_pieces: cli_pieces,
            board_letters: board_letters,
        }
    }
    fn get_piece_print(ats: &Vec<i8>) -> ColoredString {
        let res = {
            if (ats[3], ats[2]) == (0, 0) {
                "▯".to_string()
            } else if (ats[3], ats[2]) == (0, 1) {
                "▮".to_string()
            } else if (ats[3], ats[2]) == (1, 0) {
                "○".to_string()
            } else {
                "●".to_string()
            }
        };

        let mut res_c = {
            if ats[1] == 0 {
                res.red().bold()
            } else {
                res.blue().bold()
            }
        };

        res_c = {
            if ats[0] == 0 {
                res_c.on_truecolor(RGB_WHITE.0, RGB_WHITE.1, RGB_WHITE.2)
            } else {
                res_c.on_truecolor(RGB_GREY.0, RGB_GREY.1, RGB_GREY.2)
            }
        };
        res_c
    }

    fn get_print_board(&self, board_states: &Vec<BoardState>, use_letters: bool) -> String {
        let mut board = String::new();
        for ix in 0..X_DIM {
            board.push_str(BOARD_SEPARATOR);
            board.push_str("\n");
            board.push_str("| ");
            for jx in 0..Y_DIM {
                let this_ind = (ix * Y_DIM) + jx;
                let this_str = match (board_states[this_ind].piece_ix, use_letters) {
                    (None, false) => EMPTY_SPACE,
                    (None, true) => &self.board_letters[this_ind],
                    (Some(p_ix), _) => &self.cli_pieces[p_ix].print,
                };
                board.push_str(this_str);
                board.push_str(" | ");
            }
            board = board.trim().to_string();
            board.push_str("\n");
        }
        board.push_str(BOARD_SEPARATOR);
        board.push_str("\n");
        board
    }
    fn get_available_board_alphas(&self, board_states: &Vec<BoardState>) -> HashSet<String> {
        board_states
            .iter()
            .filter(|bs| !bs.square_full)
            .map(|bs| self.board_letters[bs.square_ix].clone())
            .collect()
    }
    fn print_board(&self, board_states: &Vec<BoardState>, use_letters: bool) -> () {
        let print_board = self.get_print_board(board_states, use_letters);
        println!("{}", print_board);
    }
    fn get_available_piece_ixs(&self, piece_states: &Vec<PieceState>) -> HashSet<usize> {
        piece_states
            .iter()
            .filter(|ps| !ps.on_board)
            .map(|ps| ps.piece_ix)
            .collect()
    }
    fn get_print_piece_bank(&self, piece_states: &Vec<PieceState>) -> String {
        let num_pieces = piece_states.len();
        let middle_ix = (num_pieces as f32 / 2.0).ceil() as usize;
        let mut print_str = String::new();

        for ix in 0..(middle_ix) {
            let secondix = middle_ix + ix;
            if secondix < num_pieces {
                let this_str = format!(
                    "{}\t{}\t\t{}\t{}\n",
                    ix,
                    self.bank_print_or_empty(&piece_states[ix]),
                    secondix,
                    self.bank_print_or_empty(&piece_states[secondix])
                );
                print_str.push_str(&this_str);
            } else {
                let this_str = format!("{}\t{}\n", ix, self.bank_print_or_empty(&piece_states[ix]));
                print_str.push_str(&this_str);
            }
        }
        print_str
    }
    fn bank_print_or_empty(&self, ps: &PieceState) -> &str {
        match ps.on_board {
            true => EMPTY_SPACE,
            false => &self.cli_pieces[ps.piece_ix].print,
        }
    }
    fn print_piece_bank(&self, piece_states: &Vec<PieceState>) -> () {
        let piece_bank_print = self.get_print_piece_bank(piece_states);
        println!("{}", piece_bank_print);
    }
    fn read_chosen_piece(&self, avaliable_piece_ixs: HashSet<usize>) -> usize {
        loop {
            print!("Piece index:\t");
            io::stdout().flush().unwrap();

            let mut choosen_piece = String::new();

            io::stdin()
                .read_line(&mut choosen_piece)
                .expect("Failed to read line");

            if let Ok(v) = choosen_piece.trim_end().parse::<usize>() {
                if avaliable_piece_ixs.contains(&v) {
                    return v;
                } else {
                    println!("Piece index out of bounds - try again")
                }
            } else {
                println!("Unable to parse piece name")
            }
        }
    }
    fn read_chosen_square(&self, available_board_letters: HashSet<String>) -> usize {
        loop {
            print!("Space Label:\t");
            io::stdout().flush().unwrap();

            let mut choosen_space = String::new();

            io::stdin()
                .read_line(&mut choosen_space)
                .expect("Failed to read line");

            let v = choosen_space.trim_end().to_lowercase();

            if available_board_letters.contains(&v) {
                return self.board_letters.iter().position(|s| s == &v).unwrap();
            } else {
                println!("Space lable not valid - try again")
            }
        }
    }
}

impl QuadriIORepresentation for QuadriIORepresentationCLI {
    fn pick_piece_for_opponent(
        &self,
        turn_state: &TurnState,
        board_states: Vec<BoardState>,
        piece_states: Vec<PieceState>,
    ) -> usize {
        //TODO:  tell who's turn it is
        println!("Current Board:");
        self.print_board(&board_states, false);
        println!(
            "Player {}, Pick a piece for opponent to place: ",
            turn_state.current_actor
        );
        let avaliable_piece_ixs = self.get_available_piece_ixs(&piece_states);
        self.print_piece_bank(&piece_states);
        self.read_chosen_piece(avaliable_piece_ixs)
    }
    fn pick_place_for_piece(
        &self,
        turn_state: &TurnState,
        board_states: Vec<BoardState>,
        piece_ix: usize,
    ) -> usize {
        println!(
            "Player {}, you need to place piece {}",
            turn_state.current_actor, self.cli_pieces[piece_ix].print
        );
        println!(
            "Player {}, pick a space on the board:",
            turn_state.current_actor
        );
        let available_board_alphas = self.get_available_board_alphas(&board_states);
        self.print_board(&board_states, true);
        self.read_chosen_square(available_board_alphas)
    }
    fn alert_winner(&self, winner_state: WinnerState, board_states: Vec<BoardState>) -> () {
        let winning_player = winner_state.get_winning_player();
        match winning_player {
            Some(u) => println!("Player {} is winner!", u),
            None => println!("No winner yet; keep playing!"),
        }
        println!("Winning Quadri(s) treating (0,0) as top left corner:");
        self.print_board(&board_states, false);
        let winning_quadris = match winner_state.get_winning_quadris_as_coord() {
            Some(v) => v,
            None => panic!("inacccessible"),
        };
        for v in winning_quadris {
            println!("{:?}", v);
        }
    }
}

//TODO - write some tests
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_fn() -> Result<(), String> {
//         assert_eq!(left, right);
//     }
// }
