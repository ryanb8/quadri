use crate::gamepieces::GamePiece;
use crate::quadri_enumerator;
use crate::utils;

pub static X_DIM: usize = 4;
pub static Y_DIM: usize = 4;

struct QuadriIdentifier<'a, 'b> {
    pub coords: &'a Vec<[usize; 2]>,
    pieces: Vec<Option<&'b GamePiece>>,
}

impl QuadriIdentifier<'_, '_> {
    fn new<'a, 'b>(
        gbap: &'b GameboardAndPieces,
        coords: &'a Vec<[usize; 2]>,
    ) -> QuadriIdentifier<'a, 'b> {
        QuadriIdentifier {
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

        let piece_dimension = unwrapped_pieces[0].ats.len();
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
}

#[derive(Debug, Clone)]
pub struct GameboardAndPieces {
    board: Vec<Option<usize>>, //Stored in 1 d, managed rows/columns via helpers
    bank: Vec<Option<usize>>,
    pieces: Vec<GamePiece>,
    quadri_coords: Vec<Vec<[usize; 2]>>,
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
    pub piece_ix: Option<usize>,
}

impl GameboardAndPieces {
    pub fn new() -> GameboardAndPieces {
        // let mut this_board: Vec<Option<usize>> = (0..(4 * 4)).map(|_| None).collect();
        GameboardAndPieces {
            board: (0..(X_DIM * Y_DIM)).map(|_| None).collect(),
            bank: (0..(X_DIM * Y_DIM) as usize).map(|i| Some(i)).collect(),
            pieces: GameboardAndPieces::create_pieces(),
            quadri_coords: GameboardAndPieces::get_quadri_coords(),
        }
    }

    // Actions
    //TODO - write Test
    pub fn place_piece(&mut self, piece_index: usize, board_index: usize) -> Result<usize, String> {
        //TODO - this really shouldlnt' return a result (or we need to handle the result effectivelly
        // With the CLI representation, the representation enforces that the piece-index and board_index are valid
        // I'm unsure how this will generalize with a gui...
        println!(
            "DEBUG: piece_index: {}, board_index: {}",
            piece_index, board_index
        );
        if self.piece_is_placed(&piece_index) {
            return Err("This piece already on board".to_string());
        }
        if self.space_is_full(board_index) {
            return Err("This space already occupied".to_string());
        }
        self.board[board_index] = Some(piece_index);
        self.bank[piece_index] = None;
        Ok(piece_index)
    }

    //unused but retain for now.
    //TODO - write Test
    #[allow(dead_code)]
    pub fn remove_piece(&mut self, board_index: usize) -> Result<usize, String> {
        let current_piece_index = self
            .get_piece_ix_at_board_ix(board_index)?
            .ok_or("Square is already piece-less".to_string())?;
        self.board[board_index] = None;
        self.bank[current_piece_index] = Some(current_piece_index);
        Ok(current_piece_index)
    }

    //TODO - write Test
    pub fn get_piece_states(&self) -> Vec<PieceState> {
        // From Board
        let board_piece_states: Vec<PieceState> = self
            .board
            .iter()
            .enumerate()
            .filter_map(|(b_ix, p_ix)| match p_ix {
                Some(u) => Some((b_ix, u)),
                None => None,
            })
            .map(|(b_ix, p_ix)| PieceState {
                piece: &self.pieces[*p_ix],
                on_board: true,
                location_coord: Some(self.ix_to_coord(&b_ix)),
                piece_ix: *p_ix,
            })
            .collect();

        // From bank
        let bank_piece_states: Vec<PieceState> = self
            .bank
            .iter()
            .filter_map(|u| match u {
                Some(uv) => Some(uv),
                None => None,
            })
            .map(|p_ix| PieceState {
                piece: &self.pieces[*p_ix],
                on_board: false,
                location_coord: None,
                piece_ix: *p_ix,
            })
            .collect();

        let mut piece_states = [board_piece_states, bank_piece_states].concat();

        piece_states.sort_by(|a, b| a.piece_ix.cmp(&b.piece_ix));
        piece_states
    }

    //TODO - write Test
    pub fn get_board_states(&self) -> Vec<BoardState> {
        self.board
            .iter()
            .enumerate()
            .map(|(ix, s)| {
                let this_piece_ix = match s {
                    Some(u) => Some(*u),
                    None => None,
                };

                let this_piece = match this_piece_ix {
                    Some(u) => Some(&self.pieces[u]),
                    None => None,
                };

                BoardState {
                    square_ix: ix,
                    location_coord: self.ix_to_coord(&ix),
                    square_full: s.is_some(),
                    piece: this_piece,
                    piece_ix: this_piece_ix,
                }
            })
            .collect()
    }

    //TODO - write Test(s)
    pub fn check_all_quadris(&self) -> (bool, Vec<Vec<[usize; 2]>>) {
        let quadri_validators: Vec<QuadriIdentifier> = self
            .quadri_coords
            .iter()
            .map(|coords| QuadriIdentifier::new(self, coords))
            .collect();

        let valid_quadris: Vec<Vec<[usize; 2]>> = quadri_validators
            .iter()
            .filter_map(|qv| match qv.pieces_are_quadri() {
                true => Some(qv.coords.clone()),
                false => None,
            })
            .collect();

        (valid_quadris.len() > 0, valid_quadris)
    }

    // State getters by reference
    //TODO - write Test
    fn get_piece_ix_at_board_ix(&self, board_index: usize) -> Result<Option<usize>, String> {
        if !&self.is_valid_board_index(board_index) {
            return Err("Board index out of bounds".to_string());
        }
        Ok(self.board[board_index])
    }
    fn get_piece_ix_at_coord(&self, x: usize, y: usize) -> Result<Option<usize>, String> {
        let board_index = self.coord_to_ix(&x, &y);
        self.get_piece_ix_at_board_ix(board_index)
    }
    //TODO - write Test
    fn get_piece_at_index(&self, board_index: usize) -> Result<Option<&GamePiece>, String> {
        match self.board[board_index] {
            Some(piece_ix) => Ok(Some(&self.pieces[piece_ix])),
            None => Ok(None),
        }
    }
    fn get_piece_at_coord(&self, x: usize, y: usize) -> Result<Option<&GamePiece>, String> {
        self.get_piece_at_index(self.coord_to_ix(&x, &y))
    }
    //TODO - write Test
    fn get_pieces_at_coords(
        &self,
        positions: &Vec<[usize; 2]>,
    ) -> Result<Vec<Option<&GamePiece>>, String> {
        positions
            .into_iter()
            .map(|&pos| self.get_piece_at_coord(pos[0], pos[1]))
            .collect()
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
    //TODO - write Test
    fn coord_to_ix(&self, x: &usize, y: &usize) -> usize {
        x + (y * X_DIM)
    }
    //unused but retain for now
    //TODO - write Test
    fn coords_to_ixs(&self, coords: &Vec<[usize; 2]>) -> Vec<usize> {
        coords
            .iter()
            .map(|a| self.coord_to_ix(&a[0], &a[1]))
            .collect::<Vec<usize>>()
    }
    //TODO - write Test
    fn coords_to_alpha(&self, coords: &Vec<[usize; 2]>) -> Vec<String> {
        let ixs = self.coords_to_ixs(coords);
        let alphas: Result<Vec<String>, String> =
            ixs.iter().map(|ix| utils::num_to_alpha(*ix)).collect();

        match alphas {
            Ok(v) => v,
            Err(_v) => panic!("should be inaccessible"),
        }
    }
    //TODO - write Test
    fn ix_to_coord(&self, ix: &usize) -> [usize; 2] {
        let x = ix % X_DIM;
        let y = (ix - x) / X_DIM;
        [x, y]
    }
    //TODO - write Test
    fn has_valid_coord(&self, x: usize, y: usize) -> bool {
        self.is_valid_board_index(self.coord_to_ix(&x, &y))
    }
    fn is_valid_board_index(&self, board_index: usize) -> bool {
        board_index < X_DIM * Y_DIM
    }
    //TODO - write Test
    fn piece_is_placed(&self, piece_ix: &usize) -> bool {
        match self.bank[*piece_ix] {
            Some(_u) => false,
            None => true,
        }
    }
    //TODO - write Test
    fn space_is_full_by_coord(&self, x: usize, y: usize) -> bool {
        match self.board[self.coord_to_ix(&x, &y)] {
            Some(_piece_ix) => true,
            None => false,
        }
    }
    fn space_is_full(&self, board_ix: usize) -> bool {
        match self.board[board_ix] {
            Some(_piece_ix) => true,
            None => false,
        }
    }
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
            coords: &coords,
            pieces: opt_ref_pieces_are_quadri,
        };
        assert_eq!(q.pieces_are_quadri(), true)
    }
}
