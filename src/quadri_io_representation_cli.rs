use colored::ColoredString;
use colored::Colorize;

use std::collections::HashSet;
use std::hash::Hash;
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
    pub(crate) cli_pieces: Vec<CLIGamePiece>,
    pub(crate) board_letters: Vec<String>,
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

    fn letter_ind_to_ind(u: usize) -> usize {
        let jx = u % 4;
        let ix = ((X_DIM * Y_DIM) - X_DIM - (u - jx)) / X_DIM;
        QuadriIORepresentationCLI::ix_jx_to_ind(ix, jx)
    }

    fn ind_to_letter_ind(u: usize) -> usize {
        let jx = u % 4;
        let ix = (u - jx) / X_DIM;
        QuadriIORepresentationCLI::ix_jx_to_letter_ind(ix, jx)
    }

    fn ix_jx_to_letter_ind(ix: usize, jx: usize) -> usize {
        X_DIM * Y_DIM - (X_DIM * (ix + 1)) + jx
    }

    fn ix_jx_to_ind(ix: usize, jx: usize) -> usize {
        (ix * X_DIM) + jx
    }

    fn get_print_board(
        &self,
        board_states: &Vec<BoardState>,
        use_letters: bool,
        fill_coords: Option<Vec<[usize; 2]>>,
    ) -> String {
        let limited_coords = fill_coords.is_some();
        let valid_coords = match fill_coords {
            Some(c) => HashSet::from_iter(c.into_iter()),
            None => HashSet::<[usize; 2]>::new(),
        };

        let mut board = String::new();
        for ix in 0..X_DIM {
            let mut this_line = String::new();
            this_line.push_str(BOARD_SEPARATOR);
            this_line.push_str("\n");
            this_line.push_str("| ");
            for jx in 0..Y_DIM {
                let this_coord = [ix, jx];
                let this_ind = QuadriIORepresentationCLI::ix_jx_to_ind(ix, jx);
                let this_letter_ind = QuadriIORepresentationCLI::ix_jx_to_letter_ind(ix, jx);
                let mut force_empty = false;

                if limited_coords & !valid_coords.contains(&this_coord) {
                    force_empty = true;
                }

                let this_str = match (board_states[this_ind].piece_ix, use_letters, force_empty) {
                    (None, false, _) => EMPTY_SPACE,
                    (None, true, _) => &self.board_letters[this_letter_ind],
                    (Some(p_ix), _, false) => &self.cli_pieces[p_ix].print,
                    (Some(_p_ix), _, true) => EMPTY_SPACE,
                };

                this_line.push_str(this_str);
                this_line.push_str(" | ");
            }
            this_line = this_line.trim().to_string();
            board = this_line + "\n" + &board;
        }
        board.push_str(BOARD_SEPARATOR);
        board.push_str("\n");
        board
    }
    fn get_available_board_alphas(&self, board_states: &Vec<BoardState>) -> HashSet<String> {
        board_states
            .iter()
            .filter(|bs| !bs.square_full)
            .map(|bs| {
                self.board_letters[QuadriIORepresentationCLI::ind_to_letter_ind(bs.square_ix)]
                    .clone()
            })
            .collect()
    }
    fn print_board(
        &self,
        board_states: &Vec<BoardState>,
        use_letters: bool,
        fill_coords: Option<Vec<[usize; 2]>>,
    ) -> () {
        let print_board = self.get_print_board(board_states, use_letters, fill_coords);
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
                    self.bank_print_or_empty(&piece_states[secondix]).trim_end()
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
                let letter_ind = self.board_letters.iter().position(|s| s == &v).unwrap();
                return QuadriIORepresentationCLI::letter_ind_to_ind(letter_ind);
            } else {
                println!("Space label not valid - try again")
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
        println!("Current Board:");
        self.print_board(&board_states, false, None);
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
        self.print_board(&board_states, true, None);
        self.read_chosen_square(available_board_alphas)
    }
    fn alert_winner(&self, winner_state: WinnerState, board_states: Vec<BoardState>) -> () {
        let winning_player = winner_state.winner;
        match winning_player {
            Some(u) => println!("Player {} is winner!", u),
            None => println!("No winner yet; keep playing!"),
        }
        println!("Final board:");
        self.print_board(&board_states, false, None);

        let winning_quadris = match winner_state.winning_quadris_coords {
            Some(v) => v,
            None => panic!("inacccessible"),
        };
        println!("Winning Quadri(s):");
        for v in winning_quadris {
            self.print_board(&board_states, false, Some(v));
        }
    }
}

#[cfg(test)]
mod test_Quadri_io_representation_cli {

    use indoc::formatdoc;
    use std::collections::HashMap;

    use crate::gameboard::GameboardAndPieces;
    use crate::gamepieces::GamePiece;

    use super::*;

    fn get_starting_board_state_and_piece_state<'a>(
        pieces: &'a Vec<GamePiece>,
    ) -> (Vec<BoardState<'a>>, Vec<PieceState<'a>>) {
        let board_states = pieces
            .iter()
            .enumerate()
            .map(|(ix, p)| BoardState {
                square_ix: ix,
                location_coord: GameboardAndPieces::ix_to_coord(&ix),
                square_full: false,
                piece: None,
                piece_ix: None,
            })
            .collect();

        let piece_states = pieces
            .iter()
            .enumerate()
            .map(|(ix, p)| PieceState {
                piece: p,
                on_board: false,
                location_coord: None,
                piece_ix: ix,
            })
            .collect();

        (board_states, piece_states)
    }

    fn get_test_board_states_and_piece_states<'a>(
        pieces: &'a Vec<GamePiece>,
    ) -> (
        Vec<BoardState<'a>>,
        Vec<PieceState<'a>>,
        HashMap<usize, ColoredString>,
    ) {
        let (mut board_states, mut piece_states) = get_starting_board_state_and_piece_state(pieces);

        let mut pp = HashMap::new();

        board_states[0].square_full = true;
        board_states[0].piece = Some(&pieces[0]);
        board_states[0].piece_ix = Some(0);
        piece_states[0].on_board = true;
        piece_states[0].location_coord = Some([0, 0]);
        pp.insert(
            0,
            QuadriIORepresentationCLI::get_piece_print(&piece_states[0].piece.ats),
        );

        board_states[5].square_full = true;
        board_states[5].piece = Some(&pieces[10]);
        board_states[5].piece_ix = Some(10);
        piece_states[10].on_board = true;
        piece_states[10].location_coord = Some([1, 1]);
        pp.insert(
            5,
            QuadriIORepresentationCLI::get_piece_print(&piece_states[10].piece.ats),
        );

        (board_states, piece_states, pp)
    }

    #[test]
    fn test_print() {
        let vec1: Vec<i8> = vec![0, 0, 0, 0];
        let vec1_actual = QuadriIORepresentationCLI::get_piece_print(&vec1);
        let vec1_expected =
            "▯"
                .to_string()
                .red()
                .bold()
                .on_truecolor(RGB_WHITE.0, RGB_WHITE.1, RGB_WHITE.2);
        assert_eq!(vec1_actual, vec1_expected, "testing all zeros");
        let vec2: Vec<i8> = vec![1, 1, 1, 1];
        let vec2_actual = QuadriIORepresentationCLI::get_piece_print(&vec2);
        let vec2_expected = "●"
            .to_string()
            .blue()
            .bold()
            .on_truecolor(RGB_GREY.0, RGB_GREY.1, RGB_GREY.2);
        assert_eq!(vec2_actual, vec2_expected, "testing all ones");
    }

    #[test]
    fn test_letter_ind_to_ind() {
        assert_eq!(
            QuadriIORepresentationCLI::ind_to_letter_ind(5),
            9,
            "testting 1,1"
        );
        assert_eq!(
            QuadriIORepresentationCLI::ind_to_letter_ind(6),
            10,
            "testting 1,2"
        );
        assert_eq!(
            QuadriIORepresentationCLI::ind_to_letter_ind(14),
            2,
            "testting 3,2"
        );
    }
    #[test]
    fn test_ind_to_letter_ind() {
        assert_eq!(
            QuadriIORepresentationCLI::letter_ind_to_ind(9),
            5,
            "testting 1,1"
        );
        assert_eq!(
            QuadriIORepresentationCLI::letter_ind_to_ind(10),
            6,
            "testting 1,2"
        );
        assert_eq!(
            QuadriIORepresentationCLI::letter_ind_to_ind(2),
            14,
            "testting 3,2"
        );
    }
    #[test]
    fn test_get_print_board_basic() {
        let pieces = GameboardAndPieces::create_pieces();
        let (board_states, piece_states, piece_prints_hm) =
            get_test_board_states_and_piece_states(&pieces);
        let repr = QuadriIORepresentationCLI::new(piece_states);

        let expected: String = formatdoc! {"
            +---+---+---+---+
            |   |   |   |   |
            +---+---+---+---+
            |   |   |   |   |
            +---+---+---+---+
            |   | {v5} |   |   |
            +---+---+---+---+
            | {v0} |   |   |   |
            +---+---+---+---+
            ",
            v0 = piece_prints_hm.get(&0).get_or_insert(&"wrong".red()).to_string(),
            v5 = piece_prints_hm.get(&5).get_or_insert(&"wrong".red()).to_string()
        };

        let actual = repr.get_print_board(&board_states, false, None);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_print_board_w_alpha() {
        let pieces = GameboardAndPieces::create_pieces();
        let (board_states, piece_states, piece_prints_hm) =
            get_test_board_states_and_piece_states(&pieces);
        let repr = QuadriIORepresentationCLI::new(piece_states);

        let expected: String = formatdoc! {"
            +---+---+---+---+
            | a | b | c | d |
            +---+---+---+---+
            | e | f | g | h |
            +---+---+---+---+
            | i | {v5} | k | l |
            +---+---+---+---+
            | {v0} | n | o | p |
            +---+---+---+---+
            ",
            v0 = piece_prints_hm.get(&0).get_or_insert(&"wrong".red()).to_string(),
            v5 = piece_prints_hm.get(&5).get_or_insert(&"wrong".red()).to_string()
        };

        let actual = repr.get_print_board(&board_states, true, None);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_print_board_w_selected() {
        let pieces = GameboardAndPieces::create_pieces();
        let (board_states, piece_states, piece_prints_hm) =
            get_test_board_states_and_piece_states(&pieces);
        let repr = QuadriIORepresentationCLI::new(piece_states);

        let expected: String = formatdoc! {"
            +---+---+---+---+
            |   |   |   |   |
            +---+---+---+---+
            |   |   |   |   |
            +---+---+---+---+
            |   | {v5} |   |   |
            +---+---+---+---+
            |   |   |   |   |
            +---+---+---+---+
            ",
            v5 = piece_prints_hm.get(&5).get_or_insert(&"wrong".red()).to_string()
        };

        let actual = repr.get_print_board(&board_states, false, Some(vec![[1, 1]]));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_print_board_w_selected_alpha() {
        let pieces = GameboardAndPieces::create_pieces();
        let (board_states, piece_states, piece_prints_hm) =
            get_test_board_states_and_piece_states(&pieces);
        let repr = QuadriIORepresentationCLI::new(piece_states);

        let expected: String = formatdoc! {"
            +---+---+---+---+
            | a | b | c | d |
            +---+---+---+---+
            | e | f | g | h |
            +---+---+---+---+
            | i | {v5} | k | l |
            +---+---+---+---+
            |   | n | o | p |
            +---+---+---+---+
            ",
            v5 = piece_prints_hm.get(&5).get_or_insert(&"wrong".red()).to_string()
        };

        let actual = repr.get_print_board(&board_states, true, Some(vec![[1, 1]]));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_available_board_alphas() {
        let pieces = GameboardAndPieces::create_pieces();
        let (board_states, piece_states, piece_prints_hm) =
            get_test_board_states_and_piece_states(&pieces);
        let repr = QuadriIORepresentationCLI::new(piece_states);

        let actual = repr.get_available_board_alphas(&board_states);
        let expected: HashSet<String> = HashSet::<String>::from([
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            "k".to_string(),
            "l".to_string(),
            "n".to_string(),
            "o".to_string(),
            "p".to_string(),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_available_piece_ixs() {
        let pieces = GameboardAndPieces::create_pieces();
        let (_board_states, piece_states, _piece_prints_hm) =
            get_test_board_states_and_piece_states(&pieces);
        let repr = QuadriIORepresentationCLI::new(piece_states.clone());

        let actual = repr.get_available_piece_ixs(&piece_states);
        let expected: HashSet<usize> =
            HashSet::<usize>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_print_piece_bank() {
        let pieces = GameboardAndPieces::create_pieces();

        let mut piece_states: Vec<PieceState> = pieces
            .iter()
            .enumerate()
            .map(|(ix, p)| PieceState {
                piece: p,
                on_board: true,
                location_coord: Some(GameboardAndPieces::ix_to_coord(&ix)),
                piece_ix: ix,
            })
            .collect();

        piece_states[0].on_board = false;
        piece_states[0].location_coord = None;
        piece_states[1].on_board = false;
        piece_states[1].location_coord = None;
        piece_states[8].on_board = false;
        piece_states[8].location_coord = None;

        let repr = QuadriIORepresentationCLI::new(piece_states.clone());

        let actual = repr.get_print_piece_bank(&piece_states);
        let expected: String = formatdoc! {"
        0\t{v0}\t\t8\t{v8}
        1\t{v1}\t\t9\t
        2\t \t\t10\t
        3\t \t\t11\t
        4\t \t\t12\t
        5\t \t\t13\t
        6\t \t\t14\t
        7\t \t\t15\t
        ",
            v0= QuadriIORepresentationCLI::get_piece_print(&piece_states[0].piece.ats),
            v1= QuadriIORepresentationCLI::get_piece_print(&piece_states[1].piece.ats),
            v8= QuadriIORepresentationCLI::get_piece_print(&piece_states[8].piece.ats)
        };
        assert_eq!(actual, expected);
    }
}
