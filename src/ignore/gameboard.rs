use std::collections::HashMap;

use crate::gamepieces::GamePiece;
use crate::quadri_enumerator;
use crate::utils;

pub static X_DIM: usize = 4;
pub static Y_DIM: usize = 4;

struct QuadriIdentifier<'a, 'b> {
    pub ix: usize,
    pub coords: &'a Vec<[usize; 2]>,
    pieces: Vec<Option<&'b GamePiece>>,
}

impl QuadriIdentifier<'_, '_> {
    fn new<'a, 'b>(
        gbap: &'b GameboardAndPieces,
        ix: usize,
        coords: &'a Vec<[usize; 2]>,
    ) -> QuadriIdentifier<'a, 'b> {
        QuadriIdentifier {
            ix: ix,
            coords: coords,
            pieces: gbap
                .get_pieces_at_coords(coords)
                .expect("Don't supply invalid coords"),
        }
    }
    fn unwraped_pieces(&self) -> Vec<&GamePiece> {
        self.pieces.iter().filter_map(|&x| x).collect()
    }
    pub fn pieces_are_quadri(&self) -> bool {
        // More generic than just 4 pieces - checks that all pieces (1 or more)
        // have the same value in at least one position
        let number_of_pieces = self.pieces.len();
        if number_of_pieces != X_DIM {
            panic!("Must provide at least one piece")
        }

        let unwrapped_pieces = self.unwraped_pieces();
        if unwrapped_pieces.len() != self.pieces.len() {
            return false;
        }

        let piece_dimension = unwrapped_pieces[0].dim;
        for jx in 0..piece_dimension as usize {
            let mut is_quadri = true;
            for ix in 1..number_of_pieces as usize {
                if unwrapped_pieces[ix - 1].ats[jx] != unwrapped_pieces[ix].ats[jx] {
                    is_quadri = false;
                    break;
                }
            }
            if is_quadri {
                return true;
            }
        }
        false
    }

    //     fn pieces_are_quadri_old(&self, uw_pieces: Vec<&GamePiece>) -> bool {
    //         // More generic than just 4 pieces - checks that all pieces (1 or more)
    //         // have the same value in at least one position
    //         let number_of_pieces = pieces.len();
    //         if number_of_pieces != X_DIM {
    //             panic!("Must provide at least one piece")
    //         }
    //         let piece_dimension = pieces[0].dim;
    //         for jx in 0..piece_dimension as usize {
    //             let mut is_quadri = true;
    //             for ix in 1..number_of_pieces as usize {
    //                 if pieces[ix - 1].ats[jx] != pieces[ix].ats[jx] {
    //                     is_quadri = false;
    //                     break;
    //                 }
    //             }
    //             if is_quadri {
    //                 return true;
    //             }
    //         }
    //         false
    //     }

    //         //     // for each quadri location
    //         // // if there are four pieces
    //         // // get the pieces by ref and check if quadri
    //         // // if quadri, append to ouput list
    //         // let pieces_at_coords: (usize, Vec<Vec<Option<&GamePiece>>>) = self
    //         //     .quadri_coords
    //         //     .iter()
    //         //     .enumerate()
    //         //     .map(|ix, coords| (ix, self.get_pieces_at_coords(coords)))
    //         //     .collect::(usize, <Result<Vec<Vec<Option<&GamePiece>>>, String>)()
    //         //     .expect("You can't give invalid coordiates");

    //         // let pieces_at_coords_with_full_values: Vec<Vec<&GamePiece>> = pieces_at_coords
    //         //     .iter()
    //         //     .filter_map(|opt_pieces| {
    //         //         let len = opt_pieces.len();
    //         //         let unwrapped_pieces: Vec<&GamePiece> =
    //         //             opt_pieces.iter().filter_map(|&x| x).collect();

    //         //         match len == unwrapped_pieces.len() {
    //         //             true => Some(unwrapped_pieces),
    //         //             false => None,
    //         //         }
    //         //     })
    //         //     .collect();
}

#[derive(Debug, Clone)]
pub struct GameboardAndPieces {
    pub board: Vec<Option<usize>>, //Stored in 1 d, managed rows/columns via helpers
    pub pieces: Vec<GamePiece>,
    pub ix_as_alpha: Vec<String>,
    pub piece_ix_to_board_ix: HashMap<usize, usize>,
    pub quadri_coords: Vec<Vec<[usize; 2]>>,
}

#[derive(Debug, Clone)]
pub struct PieceState<'a> {
    pub piece: &'a GamePiece,
    pub on_board: bool,
    pub location_coord: Option<[usize; 2]>,
    pub piece_ix: usize,
}

#[derive(Debug, Clone)]
pub struct BoardState<'a> {
    pub square_ix: usize,
    pub location_coord: [usize; 2],
    pub square_full: bool,
    pub piece: Option<&'a GamePiece>,
}

// impl PieceState

//TODO: need to look at pub/priv API here
// Unsure if some of these really should be public...

// API
// - Create board - check
// place piece - check
// remove piece - check
// get piece at coord + multiples - check
// get coord of piece + multiples - check
// check for quadris and return quadris

impl GameboardAndPieces {
    pub fn new() -> GameboardAndPieces {
        let mut this_board = Vec::<Option<usize>>::new();
        // let mut empty_spaces = HashSet::new();
        let mut ix_as_alpha = Vec::<String>::new();
        for ix in 0..X_DIM as usize {
            for jx in 0..Y_DIM as usize {
                this_board.push(None);
                // empty_spaces.insert([ix, jx]);
                let this_alpha = utils::num_to_alpha((ix * Y_DIM) + jx + 1).unwrap();
                ix_as_alpha.push(this_alpha);
            }
        }
        GameboardAndPieces {
            board: this_board,
            pieces: GameboardAndPieces::create_pieces(),
            ix_as_alpha: ix_as_alpha,
            // active_piece_ixs: HashSet::new(),
            piece_ix_to_board_ix: HashMap::new(),
            // empty_spaces: empty_spaces,
            // used_spaces: HashSet::new(),
            quadri_coords: GameboardAndPieces::get_quadri_coords(),
        }
    }

    // State getters by reference
    pub fn get_piece_ix_at_coord(&self, x: usize, y: usize) -> Result<Option<usize>, String> {
        if !&self.has_valid_indicies(x, y) {
            return Err("Piece index out of bounds".to_string());
        }
        Ok(self.board[self.coord_to_ix(&x, &y)])
    }
    pub fn get_piece_ixs_by_coords(
        &self,
        positions: &Vec<[usize; 2]>,
    ) -> Result<Vec<Option<usize>>, String> {
        positions
            .into_iter()
            .map(|&pos| self.get_piece_ix_at_coord(pos[0], pos[1]))
            .collect()
    }
    pub fn get_piece_at_coord(&self, x: usize, y: usize) -> Result<Option<&GamePiece>, String> {
        match self.get_piece_ix_at_coord(x, y)? {
            Some(piece_ix) => Ok(Some(&self.pieces[piece_ix])),
            none => Ok(None),
        }
    }
    pub fn get_pieces_at_coords(
        &self,
        positions: &Vec<[usize; 2]>,
    ) -> Result<Vec<Option<&GamePiece>>, String> {
        positions
            .into_iter()
            .map(|&pos| self.get_piece_at_coord(pos[0], pos[1]))
            .collect()
    }

    // Actions
    pub fn place_piece(&mut self, piece_index: usize, x: usize, y: usize) -> Result<usize, String> {
        println!("piece_index: {}, x: {}, y: {}", piece_index, x, y);
        if self.piece_is_placed(&piece_index) {
            return Err("This piece already on board".to_string());
        }
        if self.space_is_full(x, y) {
            return Err("This space already occupied".to_string());
        }
        let board_ix = self.coord_to_ix(&x, &y);
        self.board[board_ix] = Some(piece_index);
        // self.active_piece_ixs.insert(piece_index);
        self.piece_ix_to_board_ix.insert(piece_index, board_ix);
        // self.empty_spaces.remove(&[x, y]);
        // self.used_spaces.insert([x, y]);
        Ok(piece_index)
    }
    //unused but retain for now.
    pub fn remove_piece(&mut self, x: usize, y: usize) -> Result<usize, String> {
        let current_piece_index = self
            .get_piece_ix_at_coord(x, y)?
            .ok_or("Square is already piece-less".to_string())?;
        let board_ix = self.coord_to_ix(&x, &y);
        self.board[board_ix] = None;
        // self.active_piece_ixs.remove(&current_piece_index);
        self.piece_ix_to_board_ix.remove(&current_piece_index);
        // self.empty_spaces.insert([x, y]);
        // self.used_spaces.remove(&[x, y]);
        Ok(current_piece_index)
    }

    pub fn get_piece_states(&self) -> Vec<PieceState> {
        self.pieces
            .iter()
            .enumerate()
            .map(|(ix, p)| {
                let this_location_coord = match self.get_board_ix_for_piece_ix(&piece_ix) {
                    Some(u) => Some(self.ix_to_coord(u)),
                    None => None,
                };

                PieceState {
                    piece: p,
                    on_board: !this_location_coord.is_none(),
                    location_coord: this_location_coord,
                    piece_ix: ix,
                }
            })
            .collect()
    }

    pub fn get_board_states(&self) -> Vec<BoardState> {
        self.board
            .iter()
            .enumerate()
            .map(|(ix, s)| {
                let this_piece = match s {
                    Some(u) => Some(&self.pieces[*u]),
                    None => None,
                };

                BoardState {
                    square_ix: ix,
                    location_coord: self.ix_to_coord(&ix),
                    square_full: s.is_some(),
                    piece: this_piece,
                }
            })
            .collect()
    }

    // quadri checks
    fn check_all_quadris(&self) -> (bool, Vec<Vec<[usize; 2]>>) {
        let quadri_validators: Vec<QuadriIdentifier> = self
            .quadri_coords
            .iter()
            .enumerate()
            .map(|(ix, coords)| QuadriIdentifier::new(self, ix, coords))
            .collect();

        let valid_quadris: Vec<Vec<[usize; 2]>> = quadri_validators
            .iter()
            .filter_map(|qv| match qv.pieces_are_quadri() {
                true => Some(qv.coords.clone()),
                false => None,
            })
            .collect();

        (valid_quadris.len() > 0, valid_quadris)
        // for each quadri location
        // if there are four pieces
        // get the pieces by ref and check if quadri
        // // if quadri, append to ouput list
        // let pieces_at_coords: (usize, Vec<Vec<Option<&GamePiece>>>) = self
        //     .quadri_coords
        //     .iter()
        //     .enumerate()
        //     .map(|ix, coords| (ix, self.get_pieces_at_coords(coords)))
        //     .collect::(usize, <Result<Vec<Vec<Option<&GamePiece>>>, String>)()
        //     .expect("You can't give invalid coordiates");

        // let pieces_at_coords_with_full_values: Vec<Vec<&GamePiece>> = pieces_at_coords
        //     .iter()
        //     .filter_map(|opt_pieces| {
        //         let len = opt_pieces.len();
        //         let unwrapped_pieces: Vec<&GamePiece> =
        //             opt_pieces.iter().filter_map(|&x| x).collect();

        //         match len == unwrapped_pieces.len() {
        //             true => Some(unwrapped_pieces),
        //             false => None,
        //         }
        //     })
        //     .collect();

        //     .filter_map(|pieces|

        //         match pieces {
        //             Some(ps) =>
        //         })
        //     .collect();

        // let

        // (true, self.quadri_coords.clone())
        // let piece_index_sets = game
        //     .board
        //     .quadri_coords
        //     .iter()
        //     .enumerate()
        //     .map(|(ix, v)| {
        //         let this_piece_v_res = game.board.get_piece_ixs_by_coords(v);
        //         let this_piece_v_unwrap = this_piece_v_res.unwrap();
        //         (ix, this_piece_v_unwrap)
        //     })
        //     .collect::<Vec<(usize, Vec<Option<usize>>)>>();

        // let piece_index_sets2 = piece_index_sets
        //     .iter()
        //     .filter_map(|(ix, v)| {
        //         let vprime = v.iter().filter_map(|x| *x).collect::<Vec<usize>>();
        //         match vprime.len() {
        //             4 => Some((*ix, vprime)),
        //             _ => None,
        //         }
        //     })
        //     .collect::<Vec<(usize, Vec<usize>)>>();
        // let piece_sets = piece_index_sets2
        //     .iter()
        //     .map(|(ix, v)| (*ix, game.get_pieces_refs(v.clone())))
        //     .collect::<Vec<(usize, Vec<&GamePiece>)>>();
        // let quadri_results = piece_sets
        //     .iter()
        //     .map(|(ix, ps)| {
        //         let is_quadri = pieces_are_quadri(ps.to_vec()).unwrap();
        //         (*ix, is_quadri)
        //     })
        //     .collect::<Vec<(usize, bool)>>();

        // let mut current_quadris = Vec::<Vec<[usize; 2]>>::new();
        // let mut are_there_quadris = false;
        // for tup in quadri_results {
        //     if tup.1 {
        //         are_there_quadris = true;
        //         current_quadris.push(game.board.quadri_coords[tup.0].clone())
        //     }
        // }

        // (are_there_quadris, current_quadris)
    }

    // constructors:
    fn get_quadri_coords() -> Vec<Vec<[usize; 2]>> {
        let qe = quadri_enumerator::QuadriEnumerator::new(X_DIM, Y_DIM);
        let mut coord_sets = qe.get_horizontal(4);
        coord_sets.append(&mut qe.get_vertical(4));
        coord_sets.append(&mut qe.get_diagnoals(4));
        coord_sets.append(&mut qe.get_square_corners());
        coord_sets
    }
    fn create_pieces() -> Vec<GamePiece> {
        let num_pieces: usize = 4 * 4;
        (0..num_pieces)
            .map(|ix| utils::convert_to_binary(ix))
            .map(|v| utils::left_pad(v, 4))
            .map(|v| GamePiece::new_from_vec(v))
            .collect::<Result<Vec<GamePiece>, String>>()
            .unwrap()
    }

    // Internal utils
    fn coord_to_ix(&self, x: &usize, y: &usize) -> usize {
        x + (y * X_DIM)
    }
    //unused but retain for now
    fn coords_to_ixs(&self, coords: &Vec<[usize; 2]>) -> Vec<usize> {
        coords
            .iter()
            .map(|a| self.coord_to_ix(&a[0], &a[1]))
            .collect::<Vec<usize>>()
    }
    //TODO - I don't think this should be public.
    pub fn ix_to_coord(&self, ix: &usize) -> [usize; 2] {
        let x = ix % X_DIM;
        let y = (ix - x) / X_DIM;
        [x, y]
    }
    fn has_valid_indicies(&self, x: usize, y: usize) -> bool {
        let mut result = true;
        // 0 indexed - usize enforces < 0
        if x >= X_DIM || y >= Y_DIM {
            result = false;
        }
        result
    }
    fn get_board_ix_for_piece_ix(&self, piece_ix: &usize) -> Option<&usize> {
        self.piece_ix_to_board_ix.get(&piece_ix)
    }
    fn piece_is_placed(&self, piece_ix: &usize) -> bool {
        match self.get_board_ix_for_piece_ix(piece_ix) {
            Some(u) => true,
            None => false,
        }
    }
    fn space_is_full(&self, x: usize, y: usize) -> bool {
        match self.board[self.coord_to_ix(&x, &y)] {
            Some(piece_ix) => true,
            None => false,
        }
    }
    // fn all_spaces_are_full(&self, coords: &Vec<[usize; 2]>) -> bool {
    //     coords
    //         .iter()
    //         .map(|coord| self.space_is_full(coord[0], coord[1]))
    //         .all(|x| x)
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO; figure out fixtures in rust. We hacking it for now.
    fn get_coords() -> Vec<[usize; 2]> {
        vec![[0, 0], [0, 1], [0, 2], [0, 3]]
    }

    fn get_opt_pieces_that_are_quadri() -> Vec<Option<GamePiece>> {
        vec![
            Some(GamePiece::new_from_vec(vec![0, 0, 0, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 1, 0, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 0, 1, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 0, 0, 1]).expect("this is valid")),
        ]
    }
    fn get_pieces_that_are_quadri() -> Vec<GamePiece> {
        vec![
            GamePiece::new_from_vec(vec![0, 0, 0, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 1, 0, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 0, 1, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 0, 0, 1]).expect("this is valid"),
        ]
    }

    fn get_opt_pieces_that_are_not_quadri_but_full() -> Vec<Option<GamePiece>> {
        vec![
            Some(GamePiece::new_from_vec(vec![1, 0, 0, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 1, 0, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 0, 1, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 0, 0, 1]).expect("this is valid")),
        ]
    }
    fn get_pieces_that_are_not_quadri_but_full() -> Vec<GamePiece> {
        vec![
            GamePiece::new_from_vec(vec![1, 0, 0, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 1, 0, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 0, 1, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 0, 0, 1]).expect("this is valid"),
        ]
    }

    fn get_opt_pieces_that_are_not_quadri_with_empty() -> Vec<Option<GamePiece>> {
        vec![
            None,
            Some(GamePiece::new_from_vec(vec![0, 1, 0, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 0, 1, 0]).expect("this is valid")),
            Some(GamePiece::new_from_vec(vec![0, 0, 0, 1]).expect("this is valid")),
        ]
    }
    fn get_pieces_that_are_not_quadri_with_empty() -> Vec<GamePiece> {
        vec![
            GamePiece::new_from_vec(vec![0, 1, 0, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 0, 1, 0]).expect("this is valid"),
            GamePiece::new_from_vec(vec![0, 0, 0, 1]).expect("this is valid"),
        ]
    }

    #[test]
    fn test_pieces_are_quadri() {
        let coords = get_coords();
        let pieces_are_quadri = get_pieces_that_are_quadri();
        let opt_ref_pieces_are_quadri: Vec<Option<&GamePiece>> =
            pieces_are_quadri.iter().map(|gp| Some(gp)).collect();

        let q = QuadriIdentifier {
            ix: 1,
            coords: &coords,
            pieces: opt_ref_pieces_are_quadri,
        };
        assert_eq!(q.pieces_are_quadri(), true)
    }
    // fn test_quadri_identifier_unrwap_pieces_1() {
    //     let coords = get_coords();
    //     let pieces_are_quadri = get_pieces_that_are_quadri();
    //     let opt_ref_pieces_are_quadri: Vec<Option<&GamePiece>> =
    //         pieces_are_quadri.iter().map(|gp| Some(gp)).collect();
    //     let expected = get_pieces_that_are_quadri();

    //     let q = QuadriIdentifier {
    //         ix: 1,
    //         coords: &coords,
    //         pieces: opt_pieces_are_quadri,
    //     };

    //     let actual = q.unwraped_pieces();
    //     assert_eq!(actual, expected)
    // }
}
