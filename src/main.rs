use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Write;

mod utils;
mod gamepiece;
use gamepiece::GamePiece;
use gamepiece::Pieces;

// struct PieceValue2 (i8);

// impl PieceValue2 {
//     fn value(&self) -> i8 {
//         self.0
//     }
// }

// enum PieceValues {
//     One,
//     Two,
//     Three,
//     Four,
// }

// impl PieceValues {
//     fn value(&self) -> i8 {
//         match &self {
//             PieceValues::One => 1 as i8,
//             PieceValues::Two => 2 as i8,
//             PieceValues::Three => 3 as i8,
//             PieceValues::Four => 4 as i8,
//         }
//     }
// }


// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// TODO: Handle errors intelligently, none of this String mess
// TODO: Tests
// TODO: Split code into multiple files

// -- red/blue | capital/lowercase | X/O | underli
// x_ x- X_ X-   <- red
// x_ x- X_ X-   <- blue
// o_ o- O_ O-   <- red
// o_ o- O_ O-   <- blue


// -- red/blue | capital/lowercase | X/O | underline/bold
// +----+----+----+----+
// | x_ | x- | X_ | X- |  <- red
// +----+----+----+----+
// | x_ | x- | X_ | X- |  <- blue
// +----+----+----+----+
// | o_ | o- | O_ | O- |  <- red
// +----+----+----+----+
// | o_ | o- | O_ | O- |  <- blue
// +----+----+----+----+

/// yell  grey  yell  grey
// +---+---+---+---+
// | ○ | ○ | ● | ● |  <- red
// +---+---+---+---+
// | ○ | ○ | ● | ● |  <- blue
// +---+---+---+---+
// | ▯ | ▯ | ▮ | ▮ |  <- red
// +---+---+---+---+
// | ▯ | ▯ | ▮ | ▮ |  <- blue
// +---+---+---+---+


static BOARD_SEPARATOR: &'static str = "+---+---+---+---+";

#[derive(Debug)]
struct Gameboard {
    xdim: usize,
    ydim: usize,
    board: Vec<Option<usize>>,  //Stored in 1 d, managed rows/columns via helpers
    ix_as_alpha: Vec<String>,
    active_ixs: HashSet<usize>,
    empty_spaces: HashSet<[usize; 2]>,
    used_spaces:  HashSet<[usize; 2]>,
    quadri_coords: Vec::<Vec<[usize;2]>>,
}

impl Gameboard {
    fn coord_to_ix(&self, x: &usize, y: &usize) -> usize {
        x + (y * self.xdim)
    }
    //unused but retain for now
    fn coords_to_ixs(&self, coords: &Vec<[usize;2]>) -> Vec<usize> {
        coords
            .iter()
            .map(|a| self.coord_to_ix(&a[0], &a[1]))
            .collect::<Vec<usize>>()
    }
    fn ix_to_coord(&self, ix: &usize) -> [usize; 2] {
        let x = ix % self.xdim;
        let y = (ix - x)/self.xdim;
        [x, y]
    }

    fn get_quadri_coords(&mut self) -> () {
        // 0 indexed for both x and y
        let mut quadri_coords = Vec::<Vec<[usize;2]>>::new();
        let goal_len : usize  = 4;
        //vertical
        for ix in 0..self.xdim {
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
        for jx in 0..self.ydim {
            let this_v =
                (0..self.xdim)
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
                (0..self.xdim)
                .filter_map(|k| {
                    let x : usize = k;
                    let y : usize = jx+k;
                    if x < self.xdim && y < self.ydim { //x >= 0 &&  \y >= 0 per usize
                        return Some([x, y as usize])
                    } else {
                        return None
                    }
                })
                .collect::<Vec<[usize;2]>>();
            let this_v_left =
                (0..self.xdim)
                .filter_map(|k| {
                    let x : usize = k;
                    let y : i32 = jx as i32 - k as i32;
                    if x < self.xdim && y >= 0 && y < self.ydim as i32 { //x >= 0 per usize
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
        for ix in 0..self.xdim {
            let this_v_right =
                (1..self.ydim)  //already handled x ==0
                .filter_map(|k| {
                    let x : usize = ix+k;
                    let y : usize = 0+k;
                    if x < self.xdim && y < self.ydim { //x >= 0 && y >= 0 per usize
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
                    if x < self.xdim && y >= 0 && y < self.ydim as i32 {  //x >= 0 per usize
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
        let max_square_distance = if self.xdim < self.ydim { self.xdim - 1 } else {self.ydim - 1};
        for ix in 0..self.xdim {
            for jx in 0..self.ydim {
                let mut this_squares =
                    (1..max_square_distance+1)
                    .filter_map(|d| {
                        if ix+d >= self.xdim || jx+d >= self.ydim {
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
            xdim: dim_x,
            ydim: dim_y,
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
        if x >= self.xdim || y >= self.ydim {
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

struct Game {
    pieces: Pieces,
    board: Gameboard,
    xdim: usize,
    ydim: usize,
}

impl Game {
    fn new_sq(dim: usize) -> Game {
        Game {
            pieces: Pieces::new(),
            board: Gameboard::new_sq(dim),
            xdim: dim,
            ydim: dim
        }
    }
    fn get_unused_piece_ixs(&self) -> Vec<usize> {
        let pieces_ixs = HashSet::from_iter(0..self.pieces.pieces.len());
        pieces_ixs
            .difference(&self.board.active_ixs)
            .cloned()
            .collect::<Vec<usize>>()
    }
    fn list_available_pieces_for_print_2(&self) -> HashMap<usize, String> {
        let mut sorted_ixs = self.get_unused_piece_ixs();
        sorted_ixs.sort();
        sorted_ixs
            .into_iter()
            .map(|ix| (ix, self.pieces.pieces[ix].print.to_string()))
            .collect::<HashMap<usize, String>>()
    }
    fn choose_a_piece(&self, available_pieces: &Vec<String>) -> String {
        let num_pieces = available_pieces.len();
        let middle_ix = (num_pieces as f32 / 2.0).ceil() as usize;
        let mut print_str = String::new();

        for ix in 0..(middle_ix) {
            let secondix = middle_ix + ix;
            if secondix < num_pieces {
                let this_str = format!("{}\t{}\n", available_pieces[ix], available_pieces[secondix]);
                print_str.push_str(&this_str);
            } else {
                let this_str = format!("{}\n", available_pieces[ix]);
                print_str.push_str(&this_str);
            }
        }
        print_str
    }
    fn read_choosen_piece2(&self, available_pieces_map: &HashMap<usize, String>) -> usize {
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
    fn place_piece_on_choosen_space(&mut self, piece_ix: usize) -> Result<usize, String> {
        loop {
            print!("Space Label:\t");
            io::stdout().flush().unwrap();

            let mut choosen_space = String::new();

            io::stdin()
                .read_line(&mut choosen_space)
                .expect("Failed to read line");

            choosen_space = choosen_space.trim_end().to_lowercase();

            let choosen_space_ix_opt = &self.board.ix_as_alpha
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
                        return Ok(*ix)
                    }
                },
                None => println!("label provided is invlaid - try again")
            }
        }
    }
    fn game_board_string2(&self, pieces: Vec<Option<String>>, labels: Vec<Option<String>>) -> Result<String, String> {
        if pieces.len() != self.xdim * self.ydim || labels.len() != self.xdim * self.ydim {
            return Err("Pieces and labels must be the same size as the board".to_string())
        }

        let mut board = String::new();
        for ix in 0..self.xdim {
            board.push_str(BOARD_SEPARATOR);
            board.push_str("\n");
            board.push_str("| ");
            for jx in 0..self.ydim {
                let this_ind = (ix * self.ydim) + jx;
                match (pieces[this_ind].as_ref(), labels[this_ind].as_ref()) {
                    (Some(s), _)  => board.push_str(&s),
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
    fn pieces_by_position(&self) -> Vec<Option<String>> {
        let string_vec = self.pieces.pieces
            .iter()
            .map(|p| p.print.to_string())
            .collect::<Vec<String>>();

        self.board.board
            .iter()
            .map(move |ix_opt| match ix_opt {
                Some(ix) => Some(string_vec[*ix].clone()),
                None => None
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
        prior_value =  next_value;
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
    for jx in 0..piece_dimension as usize{
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

fn check_for_all_quadris(game: &Game) -> (bool, Vec<Vec<[usize;2]>>) {
    let piece_index_sets = game.board.quadri_coords
        .iter()
        .enumerate()
        .map(|(ix, v)| {
            let this_piece_v_res = game.board.get_pieces_by_indicies(v);
            let this_piece_v_unwrap = this_piece_v_res.unwrap();
            (ix, this_piece_v_unwrap)
        })
        .collect::<Vec<(usize, Vec<Option<usize>>)>>();

    let piece_index_sets2 =
        piece_index_sets
        .iter()
        .filter_map(|(ix, v)| {
            let vprime =
                v
                .iter()
                .filter_map(|x| *x)
                .collect::<Vec<usize>>();
            match vprime.len() {
                4 => Some((*ix, vprime)),
                _ => None
            }
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    let piece_sets = piece_index_sets2
        .iter()
        .map(|(ix, v)| (*ix, game.pieces.get_pieces_refs(v.clone())))
        .collect::<Vec<(usize, Vec<&GamePiece>)>>();
    let quadri_results =
        piece_sets
        .iter()
        .map(|(ix, ps)| {
            let is_quadri = pieces_are_quadri(ps.to_vec()).unwrap();
            (*ix, is_quadri)
        })
        .collect::<Vec<(usize, bool)>>();

    let mut current_quadris = Vec::<Vec<[usize;2]>>::new();
    let mut are_there_quadris = false;
    for tup in quadri_results {
        if tup.1 {
            are_there_quadris = true;
            current_quadris.push(game.board.quadri_coords[tup.0].clone())
        }
    }

    (are_there_quadris, current_quadris)
}



fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new_sq(4);
    loop {
        let available_pieces_map = &game.list_available_pieces_for_print_2();
        if available_pieces_map.len() > 0 {
            let mut available_pieces_v : Vec<(&usize, &String, String)> = available_pieces_map
                .into_iter()
                .map(|(ix, s)| (ix, s, format!("{}\t{}", ix, s)))
                .collect();
            available_pieces_v.sort_by_key(|k| k.0);
            let available_pieces = available_pieces_v.iter().map(|(_ix, _s, ixs)| ixs.clone()).collect();
            println!("Pick a piece for your opponent to place");
            let print_str = &game.choose_a_piece(&available_pieces);
            println!("{}", print_str);
            let choosen_piece_ix = &game.read_choosen_piece2(&available_pieces_map);
            println!("Opponent must place piece {}", &available_pieces_map.get(choosen_piece_ix).ok_or("I screwed up!")?);

            let mut labels = Vec::new();
            let mut empty_labels = Vec::<Option<String>>::new();
            for s in &game.board.ix_as_alpha {
                labels.push(Some(s.to_string()));
                empty_labels.push(None);
            }
            let pieces = &game.pieces_by_position();

            println!("Pick a place for piece {}", &available_pieces_map.get(choosen_piece_ix).ok_or("I screwed up!")?);
            println!("{}", &game.game_board_string2(pieces.to_vec(), labels)?);

            let _ix = &mut game.place_piece_on_choosen_space(*choosen_piece_ix)?;
            let pieces = &game.pieces_by_position();
            println!("Current Board:");
            println!("{}", &game.game_board_string2(pieces.to_vec(), empty_labels)?);
            let (are_quadris, _coords) = check_for_all_quadris(&game);
            if are_quadris {
                println!("Game is done! Winner!");
                break;
            }
        } else {
            println!{"Draw!"};
            break;
        }
    }
    Ok(())
}
