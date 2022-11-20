use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Write;

use crate::gameboard::Gameboard;
use crate::gameboard::X_DIM;
use crate::gameboard::Y_DIM;
use crate::gamepieces::GamePiece;
use crate::gamepieces::Pieces;

static BOARD_SEPARATOR: &'static str = "+---+---+---+---+";

pub struct PieceState {
    piece: GamePiece,
    on_board: bool,
    location_coord: Option<[usize; 2]>,
    location_ix: Option<usize>,
    piece_ix: usize,
}

pub struct Game {
    pub pieces: Pieces,
    pub board: Gameboard,
}

impl Game {
    pub fn new() -> Game {
        Game {
            pieces: Pieces::new(),
            board: Gameboard::new(),
        }
    }
    // external representation only
    pub fn get_game_state(&self) -> Vec<PieceState> {
        let mut piece_states = Vec::<PieceState>::new();
        for (piece_ix, piece) in self.pieces.pieces.iter().enumerate() {
            piece_states.push(PieceState {
                piece: piece.clone(),
                on_board: self.board.piece_ix_to_board_ix.contains_key(&piece_ix),
                location_coord: self.board.get_coord_for_piece_ix(&piece_ix),
                location_ix: self.board.get_board_ix_for_piece_ix(&piece_ix).cloned(),
                piece_ix: piece_ix,
            });
        }
        piece_states
    }

    //internal representation
    fn get_unused_piece_ixs(&self) -> Vec<usize> {
        (0..self.pieces.pieces.len())
            .filter_map(
                |piece_ix| match self.board.piece_ix_to_board_ix.contains_key(&piece_ix) {
                    true => None,
                    false => Some(piece_ix),
                },
            )
            .collect::<Vec<usize>>()
    }
    //external representation
    fn get_unused_pieces(&self) -> Vec<(usize, GamePiece)> {
        let mut sorted_ixs = self.get_unused_piece_ixs();
        sorted_ixs.sort();
        sorted_ixs
            .into_iter()
            .map(|ix| (ix, self.pieces.pieces[ix].clone()))
            .collect::<Vec<(usize, GamePiece)>>()
    }

    //TODO - move to CLI representation - should layer on get_unused_pieces
    pub fn list_available_pieces_for_print_2(&self) -> HashMap<usize, String> {
        let mut sorted_ixs = self.get_unused_piece_ixs();
        sorted_ixs.sort();
        sorted_ixs
            .into_iter()
            .map(|ix| (ix, self.pieces.pieces[ix].print.to_string()))
            .collect::<HashMap<usize, String>>()
    }
    pub fn choose_a_piece(&self, available_pieces: &Vec<String>) -> String {
        let num_pieces = available_pieces.len();
        let middle_ix = (num_pieces as f32 / 2.0).ceil() as usize;
        let mut print_str = String::new();

        for ix in 0..(middle_ix) {
            let secondix = middle_ix + ix;
            if secondix < num_pieces {
                let this_str =
                    format!("{}\t{}\n", available_pieces[ix], available_pieces[secondix]);
                print_str.push_str(&this_str);
            } else {
                let this_str = format!("{}\n", available_pieces[ix]);
                print_str.push_str(&this_str);
            }
        }
        print_str
    }
    pub fn read_choosen_piece2(&self, available_pieces_map: &HashMap<usize, String>) -> usize {
        loop {
            print!("Piece index:\t");
            io::stdout().flush().unwrap();

            let mut choosen_piece = String::new();

            io::stdin()
                .read_line(&mut choosen_piece)
                .expect("Failed to read line");

            if let Ok(v) = choosen_piece.trim_end().parse::<usize>() {
                if available_pieces_map.contains_key(&v) {
                    return v;
                } else {
                    println!("Piece index out of bounds - try again")
                }
            } else {
                println!("Unable to parse piece name")
            }
        }
    }
    pub fn place_piece_on_choosen_space(&mut self, piece_ix: usize) -> Result<usize, String> {
        loop {
            print!("Space Label:\t");
            io::stdout().flush().unwrap();

            let mut choosen_space = String::new();

            io::stdin()
                .read_line(&mut choosen_space)
                .expect("Failed to read line");

            choosen_space = choosen_space.trim_end().to_lowercase();

            let choosen_space_ix_opt = &self
                .board
                .ix_as_alpha
                .iter()
                .position(|a| a.to_string() == choosen_space);

            match choosen_space_ix_opt {
                Some(ix) => {
                    if self.board.board[*ix].is_some() {
                        println!("label provided is invlaid - try again")
                    } else {
                        let [x, y] = self.board.ix_to_coord(ix);
                        println!("{}, {}", x, y);
                        self.board.place_piece(piece_ix, x, y)?;
                        return Ok(*ix);
                    }
                }
                None => println!("label provided is invlaid - try again"),
            }
        }
    }
    pub fn game_board_string2(
        &self,
        pieces: Vec<Option<String>>,
        labels: Vec<Option<String>>,
    ) -> Result<String, String> {
        if pieces.len() != X_DIM * Y_DIM || labels.len() != X_DIM * Y_DIM {
            return Err("Pieces and labels must be the same size as the board".to_string());
        }

        let mut board = String::new();
        for ix in 0..X_DIM {
            board.push_str(BOARD_SEPARATOR);
            board.push_str("\n");
            board.push_str("| ");
            for jx in 0..Y_DIM {
                let this_ind = (ix * Y_DIM) + jx;
                match (pieces[this_ind].as_ref(), labels[this_ind].as_ref()) {
                    (Some(s), _) => board.push_str(&s),
                    (None, Some(l)) => board.push_str(&l),
                    (None, None) => board.push_str(" "),
                }
                board.push_str(" | ");
            }
            board = board.trim().to_string();
            board.push_str("\n");
        }
        board.push_str(BOARD_SEPARATOR);
        board.push_str("\n");
        Ok(board)
    }
    pub fn pieces_by_position(&self) -> Vec<Option<String>> {
        let string_vec = self
            .pieces
            .pieces
            .iter()
            .map(|p| p.print.to_string())
            .collect::<Vec<String>>();

        self.board
            .board
            .iter()
            .map(move |ix_opt| match ix_opt {
                Some(ix) => Some(string_vec[*ix].clone()),
                None => None,
            })
            .collect::<Vec<Option<String>>>()
    }
}

fn all_equal(arr: Vec<i8>) -> Result<bool, String> {
    let arr_len = arr.len();
    if arr_len == 0 {
        return Err("Array must be non-empty".to_string());
    }
    let mut result = true;
    let mut prior_value = arr[0];
    for ix in 1..arr_len {
        let next_value = arr[ix];
        if prior_value != next_value {
            result = false;
            break;
        }
        prior_value = next_value;
    }
    Ok(result)
}

//assume all pieces have same dimensions
fn pieces_are_quadri(pieces: Vec<&GamePiece>) -> Result<bool, String> {
    let number_of_pieces = pieces.len();
    if number_of_pieces == 0 {
        return Err("Must provide at least one piece".to_string());
    }
    let piece_dimension = pieces[0].dim;
    // let mut this_check = Vec::new();
    for jx in 0..piece_dimension as usize {
        let mut this_check = Vec::new();
        for ix in 0..number_of_pieces as usize {
            this_check.push(pieces[ix].ats[jx]);
        }
        if all_equal(this_check)? {
            return Ok(true);
        }
    }
    Ok(false)
}

//TODO - should be an implementation method
pub fn check_for_all_quadris(game: &Game) -> (bool, Vec<Vec<[usize; 2]>>) {
    let piece_index_sets = game
        .board
        .quadri_coords
        .iter()
        .enumerate()
        .map(|(ix, v)| {
            let this_piece_v_res = game.board.get_pieces_by_indicies(v);
            let this_piece_v_unwrap = this_piece_v_res.unwrap();
            (ix, this_piece_v_unwrap)
        })
        .collect::<Vec<(usize, Vec<Option<usize>>)>>();

    let piece_index_sets2 = piece_index_sets
        .iter()
        .filter_map(|(ix, v)| {
            let vprime = v.iter().filter_map(|x| *x).collect::<Vec<usize>>();
            match vprime.len() {
                4 => Some((*ix, vprime)),
                _ => None,
            }
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    let piece_sets = piece_index_sets2
        .iter()
        .map(|(ix, v)| (*ix, game.pieces.get_pieces_refs(v.clone())))
        .collect::<Vec<(usize, Vec<&GamePiece>)>>();
    let quadri_results = piece_sets
        .iter()
        .map(|(ix, ps)| {
            let is_quadri = pieces_are_quadri(ps.to_vec()).unwrap();
            (*ix, is_quadri)
        })
        .collect::<Vec<(usize, bool)>>();

    let mut current_quadris = Vec::<Vec<[usize; 2]>>::new();
    let mut are_there_quadris = false;
    for tup in quadri_results {
        if tup.1 {
            are_there_quadris = true;
            current_quadris.push(game.board.quadri_coords[tup.0].clone())
        }
    }

    (are_there_quadris, current_quadris)
}
