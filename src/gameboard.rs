use std::collections::HashMap;
use std::collections::HashSet;

// mod crate:quadri_enumerator;
// use quadri_enm
use crate::quadri_enumerator;
use crate::utils;

pub static X_DIM: usize = 4;
pub static Y_DIM: usize = 4;

#[derive(Debug)]
pub struct Gameboard {
    pub board: Vec<Option<usize>>, //Stored in 1 d, managed rows/columns via helpers
    pub ix_as_alpha: Vec<String>,
    pub active_piece_ixs: HashSet<usize>,
    pub piece_ix_to_board_ix: HashMap<usize, usize>,
    pub empty_spaces: HashSet<[usize; 2]>,
    pub used_spaces: HashSet<[usize; 2]>,
    pub quadri_coords: Vec<Vec<[usize; 2]>>,
}

//TODO: need to look at pub/priv API here
// Unsure if some of these really should be public...
impl Gameboard {
    pub fn coord_to_ix(&self, x: &usize, y: &usize) -> usize {
        x + (y * X_DIM)
    }
    //unused but retain for now
    pub fn coords_to_ixs(&self, coords: &Vec<[usize; 2]>) -> Vec<usize> {
        coords
            .iter()
            .map(|a| self.coord_to_ix(&a[0], &a[1]))
            .collect::<Vec<usize>>()
    }
    pub fn ix_to_coord(&self, ix: &usize) -> [usize; 2] {
        let x = ix % X_DIM;
        let y = (ix - x) / X_DIM;
        [x, y]
    }

    fn get_quadri_coords() -> Vec<Vec<[usize; 2]>> {
        let qe = quadri_enumerator::QuadriEnumerator::new(X_DIM, Y_DIM);
        let mut coord_sets = qe.get_horizontal(4);
        coord_sets.append(&mut qe.get_vertical(4));
        coord_sets.append(&mut qe.get_diagnoals(4));
        coord_sets.append(&mut qe.get_square_corners());
        coord_sets
    }
    pub fn new() -> Gameboard {
        let mut this_board = Vec::<Option<usize>>::new();
        let mut empty_spaces = HashSet::new();
        let mut ix_as_alpha = Vec::<String>::new();
        for ix in 0..X_DIM as usize {
            for jx in 0..Y_DIM as usize {
                this_board.push(None);
                empty_spaces.insert([ix, jx]);
                let this_alpha = utils::num_to_alpha((ix * Y_DIM) + jx + 1).unwrap();
                ix_as_alpha.push(this_alpha);
            }
        }
        let quadri_coords = Gameboard::get_quadri_coords();
        Gameboard {
            board: this_board,
            ix_as_alpha: ix_as_alpha,
            active_piece_ixs: HashSet::new(),
            piece_ix_to_board_ix: HashMap::new(),
            empty_spaces: empty_spaces,
            used_spaces: HashSet::new(),
            quadri_coords: quadri_coords,
        }
    }
    fn has_valid_indicies(&self, x: usize, y: usize) -> bool {
        let mut result = true;
        // 0 indexed - usize enforces < 0
        if x >= X_DIM || y >= Y_DIM {
            result = false;
        }
        result
    }
    fn get_piece_index(&self, x: usize, y: usize) -> Result<Option<usize>, String> {
        if !&self.has_valid_indicies(x, y) {
            return Err("Piece index out of bounds".to_string());
        }
        Ok(self.board[self.coord_to_ix(&x, &y)])
    }
    pub fn get_pieces_by_indicies(
        &self,
        positions: &Vec<[usize; 2]>,
    ) -> Result<Vec<Option<usize>>, String> {
        positions
            .into_iter()
            .map(|&pos| self.get_piece_index(pos[0], pos[1]))
            .collect()
    }
    pub fn place_piece(&mut self, piece_index: usize, x: usize, y: usize) -> Result<usize, String> {
        println!("piece_index: {}, x: {}, y: {}", piece_index, x, y);
        if let Some(_ix) = self.active_piece_ixs.get(&piece_index) {
            return Err("This piece already on board".to_string());
        }
        if self.used_spaces.contains(&[x, y]) {
            return Err("This space already occupied".to_string());
        }
        let board_ix = self.coord_to_ix(&x, &y);
        self.board[board_ix] = Some(piece_index);
        self.active_piece_ixs.insert(piece_index);
        self.piece_ix_to_board_ix.insert(piece_index, board_ix);
        self.empty_spaces.remove(&[x, y]);
        self.used_spaces.insert([x, y]);
        Ok(piece_index)
    }
    //unused but retain for now.
    pub fn remove_piece(&mut self, x: usize, y: usize) -> Result<usize, String> {
        let current_piece_index = self
            .get_piece_index(x, y)?
            .ok_or("Square is already piece-less".to_string())?;
        let board_ix = self.coord_to_ix(&x, &y);
        self.board[board_ix] = None;
        self.active_piece_ixs.remove(&current_piece_index);
        self.piece_ix_to_board_ix.remove(&current_piece_index);
        self.empty_spaces.insert([x, y]);
        self.used_spaces.remove(&[x, y]);
        Ok(current_piece_index)
    }

    pub fn get_board_ix_for_piece_ix(&self, piece_ix: &usize) -> Option<&usize> {
        self.piece_ix_to_board_ix.get(&piece_ix)
    }

    pub fn get_coord_for_piece_ix(&self, piece_ix: &usize) -> Option<[usize; 2]> {
        match self.get_board_ix_for_piece_ix(&piece_ix) {
            None => None,
            Some(u) => Some(self.ix_to_coord(&u)),
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    //TODO - write tests
}
