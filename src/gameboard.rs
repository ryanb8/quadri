use std::collections::HashSet;

static X_DIM: usize = 4;
static Y_DIM: usize = 4;
static BOARD_SEPARATOR: &'static str = "+---+---+---+---+";

#[derive(Debug)]
pub struct Gameboard {
    board: Vec<Option<usize>>,  //Stored in 1 d, managed rows/columns via helpers
    ix_as_alpha: Vec<String>,
    active_ixs: HashSet<usize>,
    empty_spaces: HashSet<[usize; 2]>,
    used_spaces:  HashSet<[usize; 2]>,
    quadri_coords: Vec::<Vec<[usize;2]>>,
}

impl Gameboard {
    pub fn coord_to_ix(&self, x: &usize, y: &usize) -> usize {
        x + (y * X_DIM)
    }
    //unused but retain for now
    pub fn coords_to_ixs(&self, coords: &Vec<[usize;2]>) -> Vec<usize> {
        coords
            .iter()
            .map(|a| self.coord_to_ix(&a[0], &a[1]))
            .collect::<Vec<usize>>()
    }
    fn ix_to_coord(&self, ix: &usize) -> [usize; 2] {
        let x = ix % X_DIM;
        let y = (ix - x)/X_DIM;
        [x, y]
    }

    fn get_quadri_coords(&mut self) -> () {
        // 0 indexed for both x and y
        let mut quadri_coords = Vec::<Vec<[usize;2]>>::new();
        let goal_len : usize  = 4;
        //vertical
        for ix in 0..X_DIM {
            let this_v =
                (0..self.ydim)
                .map(|y| [ix, y])
                .collect::<Vec<[usize;2]>>();
            if this_v.len() >= goal_len {
                let mut these_vs = Vec::<Vec<[usize;2]>>::new();
                for ind in 0..(this_v.len()-(goal_len-1)) {
                    these_vs.push((&this_v[ind..ind+goal_len]).to_vec())
                }
                quadri_coords.append(&mut these_vs);
            }
        }
        //horizontal
        for jx in 0..Y_DIM {
            let this_v =
                (0..X_DIM)
                .map(|x| [x, jx])
                .collect::<Vec<[usize;2]>>();
            if this_v.len() >= goal_len {
                let mut these_vs = Vec::<Vec<[usize;2]>>::new();
                for ind in 0..(this_v.len()-(goal_len - 1)) {
                    these_vs.push((&this_v[ind..ind+goal_len]).to_vec())
                }
                quadri_coords.append(&mut these_vs);
            }
        }
        //diagonals
        //top-line diagonals
        for jx in 0..self.ydim {
            let this_v_right =
                (0..X_DIM)
                .filter_map(|k| {
                    let x : usize = k;
                    let y : usize = jx+k;
                    if x < X_DIM && y < Y_DIM { //x >= 0 &&  \y >= 0 per usize
                        return Some([x, y as usize])
                    } else {
                        return None
                    }
                })
                .collect::<Vec<[usize;2]>>();
            let this_v_left =
                (0..X_DIM)
                .filter_map(|k| {
                    let x : usize = k;
                    let y : i32 = jx as i32 - k as i32;
                    if x < X_DIM && y >= 0 && y < Y_DIM as i32 { //x >= 0 per usize
                        return Some([x, y as usize])
                    } else {
                        return None
                    }
                })
                .collect::<Vec<[usize;2]>>();
            if this_v_left.len() >= goal_len {
                let mut these_vs = Vec::<Vec<[usize;2]>>::new();
                for ind in 0..(this_v_left.len()-(goal_len-1)) {
                    these_vs.push((&this_v_left[ind..ind+goal_len]).to_vec())
                }
                quadri_coords.append(&mut these_vs);
            }
            if this_v_right.len() >= goal_len {
                let mut these_vs = Vec::<Vec<[usize;2]>>::new();
                for ind in 0..(this_v_right.len()-(goal_len-1)) {
                    these_vs.push((&this_v_right[ind..ind+goal_len]).to_vec())
                }
                quadri_coords.append(&mut these_vs);
            }
        }
        // Left side diagonals
        // Right side diagnoals
        for ix in 0..X_DIM {
            let this_v_right =
                (1..self.ydim)  //already handled x ==0
                .filter_map(|k| {
                    let x : usize = ix+k;
                    let y : usize = 0+k;
                    if x < X_DIM && y < Y_DIM { //x >= 0 && y >= 0 per usize
                        return Some([ix+k, 0+k])
                    } else {
                        return None
                    }
                })
                .collect::<Vec<[usize;2]>>();
            let this_v_left =
                (1..self.ydim)
                .filter_map(|k| {
                    let x : usize= ix + k;
                    let y : i32 = self.ydim as i32 - 1 - k as i32;
                    if x < X_DIM && y >= 0 && y < Y_DIM as i32 {  //x >= 0 per usize
                        return Some([x, y as usize])
                    } else {
                        return None
                    }
                })
                .collect::<Vec<[usize;2]>>();
            if this_v_left.len() >= goal_len {
                let mut these_vs = Vec::<Vec<[usize;2]>>::new();
                for ind in 0..(this_v_left.len()-(goal_len-1)) {
                    these_vs.push((&this_v_left[ind..ind+goal_len]).to_vec())
                }
                quadri_coords.append(&mut these_vs);
            }
            if this_v_right.len() >= goal_len {
                let mut these_vs = Vec::<Vec<[usize;2]>>::new();
                for ind in 0..(this_v_right.len()-(goal_len-1)) {
                    these_vs.push((&this_v_right[ind..ind+goal_len]).to_vec())
                }
                quadri_coords.append(&mut these_vs);
            }
        }

        //squares
        let max_square_distance = if X_DIM < Y_DIM { X_DIM - 1 } else {Y_DIM - 1};
        for ix in 0..X_DIM {
            for jx in 0..Y_DIM {
                let mut this_squares =
                    (1..max_square_distance+1)
                    .filter_map(|d| {
                        if ix+d >= X_DIM || jx+d >= Y_DIM {
                            return None
                        }
                        let this_square_a = [
                            [ix,jx],
                            [ix+d, jx],
                            [ix, jx + d],
                            [ix+d, jx + d]
                        ];
                        let this_square = this_square_a.to_vec();
                        Some(this_square)
                    })
                    .collect::<Vec<Vec<[usize;2]>>>();
                    quadri_coords.append(&mut this_squares);
            }
        }
        self.quadri_coords = quadri_coords;
    }
    fn new(dim_x: usize, dim_y: usize) -> Gameboard {
        let mut this_board = Vec::<Option<usize>>::new();
        let mut empty_spaces = HashSet::new();
        let mut ix_as_alpha = Vec::<String>::new();
        for ix in 0..dim_x as usize {
            for jx in 0..dim_y as usize {
                this_board.push(None);
                empty_spaces.insert([ix, jx]);
                let this_alpha = utils::num_to_alpha((ix*dim_y) + jx + 1).unwrap();
                ix_as_alpha.push(this_alpha);
            }
        }
        let mut g = Gameboard {
            board: this_board,
            ix_as_alpha: ix_as_alpha,
            active_ixs: HashSet::new(),
            empty_spaces: empty_spaces,
            used_spaces: HashSet::new(),
            quadri_coords: Vec::<Vec<[usize;2]>>::new(),
        };
        g.get_quadri_coords();
        g
    }
    fn new_sq(dim: usize) -> Gameboard {
        Gameboard::new(dim, dim)
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
    fn get_pieces_by_indicies(&self, positions: &Vec<[usize; 2]>) -> Result<Vec<Option<usize>>, String> {
        positions
            .into_iter()
            .map(|&pos| self.get_piece_index(pos[0], pos[1]))
            .collect()
    }
    fn place_piece(&mut self, piece_index: usize, x: usize, y: usize) -> Result<usize, String> {
        println!("piece_index: {}, x: {}, y: {}", piece_index, x, y);
        if let Some(_ix) = self.active_ixs.get(&piece_index) {
            return Err("This piece already on board".to_string());
        }
        if self.used_spaces.contains(&[x, y]) {
            return Err("This space already occupied".to_string());
        }
        let board_ix = self.coord_to_ix(&x, &y);
        self.board[board_ix] = Some(piece_index);
        self.active_ixs.insert(piece_index);
        self.empty_spaces.remove(&[x, y]);
        self.used_spaces.insert([x,y]);
        Ok(piece_index)
    }
    //unused but retain for now.
    fn remove_piece(&mut self, x: usize, y: usize) -> Result<usize, String> {
        let current_piece_index = self.get_piece_index(x, y)?.ok_or("Square is already piece-less".to_string())?;
        let board_ix = self.coord_to_ix(&x, &y);
        self.board[board_ix] = None;
        self.active_ixs.remove(&current_piece_index);
        self.empty_spaces.insert([x,y]);
        self.used_spaces.remove(&[x,y]);
        Ok(current_piece_index)
    }
}