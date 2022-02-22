use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use colored::Colorize;
use colored::ColoredString;
use std::io;
use std::io::Write;
use std::any::type_name;
use std::iter::Zip;

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


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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

static ALLOWED_ATTRIBUTE_VALUES: [i8; 2] = [0, 1];
static ALPHABET_ARRAY: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
static BOARD_SEPARATOR: &'static str = "+---+---+---+---+";
static ROW_TEMPLATE: &'static str = "| {} | {} | {} | {} |";
static EMPTY_SQUARE: &'static str = " ";
static RGB_WHITE: (u8, u8, u8) =  (255, 255, 255);
static RGB_GREY: (u8, u8, u8) = (255, 204, 0);


fn num_to_alpha(n: usize) -> Result<String, String> {
    if n == 0 {
        return Err("num must be greater than 0".to_string());
    }

    let mut v = n;
    let mut alpha = String::new();
    while v > 0  {
        let r = v % 26;
        v = (v - r)/26;
        alpha.push(ALPHABET_ARRAY[r-1]);
    }
    alpha.reverse();
    Ok(alpha)
}

fn convert_to_binary(x: usize) -> Vec<i8> {
    // usize => only 0 or positive integers
    let mut binary = Vec::new();
    let mut v = x;
    if v == 0 {
        binary.push(0);
    }
    while v > 0  {
        let r = v % 2;
        v = (v - r)/2;
        binary.push(r as i8);
    }
    binary.reverse();
    binary
}

fn left_pad(mut v:Vec<i8>, p: i8, dim: usize) -> Vec<i8> {
    let need_len = dim - v.len();
    if need_len <= 0 {
        return v
    }
    let mut new_v: Vec<i8> = Vec::new();
    for ix in 0..need_len {
        new_v.push(0);
    }
    new_v.append(&mut v);
    new_v
}

fn vec_to_name(v: &Vec<i8>) -> Result<String, String> {
    if v.len() != 4 {
        return Err("Need vector of length 4".to_string());
    }
    for val in v {
        if val > &1 || val < &0 {
            return Err("All values must be 1 or 0".to_string())
        }
    }
    let mut res = String::new();
    //2
    if v[1] == 0 {
        res.push_str("red_");
    } else {
        res.push_str("blue_");
    }
    //3
    if v[2] == 0 {
        res.push_str("empty_");
    } else {
        res.push_str("full_");
    }
    //4
    if v[3] == 0 {
        res.push_str("rec_");
    } else {
        res.push_str("circle_");
    }
        //1
    if v[0] == 0 {
        res.push_str("onWhite");
    } else {
        res.push_str("onGrey");
    }
    Ok(res)
}

fn vec_to_print(v: &Vec<i8>) -> Result<ColoredString, String> {
    if v.len() != 4 {
        return Err("Need vector of length 4".to_string());
    }
    for val in v {
        if val > &1 || val < &0 {
            return Err("All values must be 1 or 0".to_string())
        }
    }

    // if v[3] == 0 && v[2] == 0 {

    // } else if v[3] == 0 && v[2] == 1 {

    // }
    let res = match (v[3], v[2]) {
        (0,0) => "▯".to_string(),
        (0,1) => "▮".to_string(),
        (1,0) => "○".to_string(),
        (1,1) => "●".to_string(),
        _ => return Err("All values must be 1 or 0".to_string())
    };

    //2
    let mut res_c = match v[1] {
        0 => res.red().bold(),
        1 => res.blue().bold(),
        _ => return Err("All values must be 1 or 0".to_string())
    };
    //1
    res_c = match v[0] {
        0 => res_c.on_truecolor(RGB_WHITE.0, RGB_WHITE.1, RGB_WHITE.2),
        1 => res_c.on_truecolor(RGB_GREY.0, RGB_GREY.1, RGB_GREY.2),
        _ => return Err("All values must be 1 or 0".to_string())
    };

    Ok(res_c)
}

#[derive(Debug)]
struct AV(i8);  //allowed values

impl AV {
    fn in_allowed_values(value: i8) -> bool {
        let mut is_valid = false;
        for av in ALLOWED_ATTRIBUTE_VALUES {
                if value == av {
                    is_valid = true;
                }
            }
        is_valid
    }
    fn new(value: i8) -> Result<AV, String> {
        let is_valid = AV::in_allowed_values(value);
        if is_valid {
            Ok(AV(value))
        } else {
            Err("Piece value is invalid".to_string())
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct GamePiece {
    name: String,
    ats: Vec<i8>,
    dim: i8,
    print: ColoredString
}

impl PartialEq for GamePiece {
    fn eq(&self, other: &Self) -> bool {
        self.ats == other.ats
    }
}
impl Eq for GamePiece {}
impl Hash for GamePiece{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ats.hash(state);
        self.dim.hash(state);
    }
}

impl GamePiece {
    fn new_from_array(values: &[i8]) -> Result<GamePiece, String> {
        let dim: i8 = values.len().try_into().unwrap();
        if dim != 4 {
            return Err("Need 4 values for a piece".to_string());
        }
        for v in values {
            if !AV::in_allowed_values(*v) {
                return Err(format!("Values for pieces must be in {:?}", ALLOWED_ATTRIBUTE_VALUES));
            }
        }
        let v = values.to_vec();
        let name = vec_to_name(&v)?;
        let print = vec_to_print(&v)?;
        Ok(GamePiece {
            name: name,
            ats: values.to_vec(),
            dim: dim,
            print: print
        })
    }
    fn new_from_vec(values: Vec<i8>) -> Result<GamePiece, String> {
        let dim: i8 = values.len().try_into().unwrap();
        // println!("{:?}", dim);
        // println!("{:?}", values);
        if dim != 4 {
            return Err("Need 4 values for a piece".to_string());
        }
        for v in &values {
            if !AV::in_allowed_values(*v) {
                return Err(format!("Values for pieces must be in {:?}", ALLOWED_ATTRIBUTE_VALUES));
            }
        }
        let name = vec_to_name(&values)?;
        let print = vec_to_print(&values)?;
        Ok(GamePiece {
            name: name,
            ats: values,
            dim: dim,
            print: print
        })
    }
    fn get_values(&self) -> &Vec<i8> {
        &self.ats
    }
    // fn get_print_index(&self) -> String {
    //     let print_index = format!("{})\t{}", self.index.to_string(), self.print);
    //     print_index
    // }
}


#[derive(Debug)]
struct Gameboard {
    xdim: usize,
    ydim: usize,
    // arrangement: Vec<Vec<Option<&'a GamePiece>>>
    // board: Vec<Vec<Option<usize>>>,
    board: Vec<Option<usize>>,  //Stored in 1 d, managed rows/columns via helpers
    ix_as_alpha: Vec<String>,
    active_ixs: HashSet<usize>,
    empty_spaces: HashSet<[usize; 2]>,
    used_spaces:  HashSet<[usize; 2]>,
    quadri_coords: Vec::<Vec<[usize;2]>>,
}

impl Gameboard {
    // fn new(dim_x: usize, dim_y: usize) -> Gameboard {
    //     let mut this_board = Vec::new();
    //     let mut empty_spaces = HashSet::new();
    //     for ix in 0..dim_x as usize {
    //         this_board.push(Vec::new());
    //         for jx in 0..dim_y as usize {
    //             this_board[ix].push(None);
    //             empty_spaces.insert([ix, jx]);
    //         }
    //     }
    //     Gameboard {
    //         xdim: dim_x,
    //         ydim: dim_y,
    //         board: this_board,
    //         active_ixs: HashSet::new(),
    //         empty_spaces: empty_spaces,
    //         used_spaces: HashSet::new()
    //     }
    // }
    fn coord_to_ix(&self, x: &usize, y: &usize) -> usize {
        x + (y * self.xdim)
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
            if (this_v.len() >= goal_len) {
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
                //TODO combine filter map to avoid overflow error
                .map(|k| [0+k, jx+k])
                .filter(|[x, y]| x >= &0 && x < &self.xdim && y >= &0 && y < &self.ydim)
                .collect::<Vec<[usize;2]>>();
            let this_v_left =
                (0..self.xdim)
                //TODO combine filter map to avoid overflow error
                .map(|k| [0+k, jx-k])
                .filter(|[x, y]| x >= &0 && x < &self.xdim && y >= &0 && y < &self.ydim)
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
                //TODO combine filter map to avoid overflow error
                .map(|k| [ix+k, 0+k])
                .filter(|[x, y]| x >= &0 && x < &self.xdim && y >= &0 && y < &self.ydim)
                .collect::<Vec<[usize;2]>>();
            let this_v_left =
                (1..self.ydim)
                //TODO combine filter map to avoid overflow error
                .map(|k| [ix+k, self.ydim - 1 - k]) //already handled x ==0
                .filter(|[x, y]| x >= &0 && x < &self.xdim && y >= &0 && y < &self.ydim)
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
        let max_square_distance = if (self.xdim < self.ydim) { self.xdim - 1 } else {self.ydim - 1};
        for ix in (0..self.xdim) {
            for jx in (0..self.ydim) {
                let mut this_squares =
                    (1..max_square_distance+1)
                    .filter_map(|d| {
                        if ( ix+d >= self.xdim || jx+d >= self.ydim) {
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
                let this_alpha = num_to_alpha(((ix*dim_y) + jx + 1)).unwrap();
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
    fn get_pieces_by_indicies(&self, positions: Vec<[usize; 2]>) -> Result<Vec<Option<usize>>, String> {
        positions
            .into_iter()
            .map(|pos| self.get_piece_index(pos[0], pos[1]))
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
    fn remove_piece(&mut self, x: usize, y: usize) -> Result<usize, String> {
        let current_piece_index = self.get_piece_index(x, y)?.ok_or("Square is already piece-less".to_string())?;
        let board_ix = self.coord_to_ix(&x, &y);
        self.board[board_ix] = None;
        self.active_ixs.remove(&current_piece_index);
        self.empty_spaces.insert([x,y]);
        self.used_spaces.remove(&[x,y]);
        Ok(current_piece_index)
    }
    fn list_empty_spaces(&self) -> Vec<[usize; 2]> {
        self.empty_spaces
            .clone()
            .into_iter()
            .collect::<Vec<[usize;2]>>()
    }
    fn list_used_spaces(&self) -> Vec<[usize; 2]> {
        self.used_spaces
            .clone()
            .into_iter()
            .collect::<Vec<[usize;2]>>()
    }
    // fn new(dim_x: i8, dim_y: i8) -> Result<Gameboard, String>{
    //     if dim_x <= 0 || dim_y <= 0 {
    //         return Err("Dims must be greater than or equal to 1".to_string());
    //     }
    //     let mut this_arrangement = Vec::new();
    //     for ix in 0..dim_x as usize {
    //         this_arrangement.push(Vec::new());
    //         for _jx in 0..dim_y as usize {
    //             this_arrangement[ix].push(None);
    //         }
    //     }
    //     Ok(
    //         Gameboard {
    //             xdim: dim_x as usize,
    //             ydim: dim_y as usize,
    //             arrangement: this_arrangement
    //         }
    //     )
    // }
    // fn validate_indices(&self, x: usize, y: usize) -> Result<(), String> {
    //     if !&self.has_valid_indicies(x, y) {
    //         return Err("Piece index out of bounds".to_string());
    //     }
    //     Ok(())
    // }
    // fn new_sq(dim: i8) -> Result<Gameboard<'a>, String>{
    //     Gameboard::new(dim, dim)
    // }
    // fn list_spaces(&self) -> Vec<(char, (usize, usize))> {
    //     let mut spaces = Vec::new();
    //     for ix in 0..self.xdim {
    //         for jx in 0..self.ydim {
    //             spaces.push((ALPHABET_ARRAY[(ix * jx * self.xdim) + jx], (ix, jx)))
    //         }
    //     }
    //     spaces
    // }
    // fn list_pieces(&self) -> Vec<(usize, &GamePiece)> {
    //     let mut pieces = Vec::new();
    //     for (gp, loc_ix_tup) in &self.placement {
    //         pieces.push((loc_ix_tup.1, gp));
    //     }
    //     pieces
    // }
}

struct Pieces {
    pieces: Vec<GamePiece>
}

impl Pieces {
    fn new_sq(dim: usize) -> Pieces {
        let num_pieces: usize = dim * dim;
        let pieces : Vec<GamePiece> =
            (0..num_pieces)
            .map(|ix| convert_to_binary(ix))
            .map(|v| left_pad(v, 0, 4))
            .map(|v| GamePiece::new_from_vec(v))
            .collect::<Result<Vec<GamePiece>, String>>()
            .unwrap();

        // let pieces = pieces1
        //     .into_iter()
        //     .map(|v| GamePiece::new_from_vec(v))
        //     .collect::<Result<Vec<GamePiece>, String>>()
        //     .unwrap();

        Pieces {
            pieces: pieces
        }
    }
    // fn new_sq(dim: usize) -> PieceLookup {
    //     let num_pieces: usize = dim * dim;
    //     let pieces1 : Vec<Vec<i8>> =
    //         (0..num_pieces)
    //         .map(|ix| convert_to_binary(ix))
    //         .map(|v| left_pad(v, 0, 4))
    //         .collect();

    //     // println!("{:?}", pieces1);

    //     let pieces = pieces1
    //         .into_iter()
    //         .enumerate()
    //         .map(|(i, v)| GamePiece::new_from_vec(v, i as i8))
    //         .collect::<Result<Vec<GamePiece>, String>>()
    //         .unwrap();

    //     let mut placement: HashMap<GamePiece, Option<[usize; 2]>> = HashMap::new();
    //     for gp in pieces.into_iter() {
    //         placement.insert(gp, None);
    //     }
    //     PieceLookup {
    //         placement: placement
    //     }
    // }
    // fn list_pieces(&self) -> Vec<(&Option<[usize; 2]>, &GamePiece)> {
    //     let mut pieces = Vec::new();
    //     for (gp, loc_tup) in &self.placement {
    //         pieces.push((loc_tup, gp));
    //     }
    //     pieces
    // }
    // fn list_available_pieces(&self) -> Vec<&GamePiece> {
    //     let mut pieces = Vec::new();
    //     for (gp, loc_tup) in &self.placement {
    //         if loc_tup.is_none() {
    //             pieces.push(gp);
    //         }
    //     }
    //     pieces.sort_by(|gp1, gp2| gp1.index.partial_cmp(&gp2.index).unwrap());
    //     pieces
    // }
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
            pieces: Pieces::new_sq(dim),
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
    fn list_avail_pieces_for_print(&self) -> Vec<String> {
        let mut sorted_ixs = self.get_unused_piece_ixs();
        sorted_ixs.sort();
        sorted_ixs
            .into_iter()
            .map(|ix| format!("{}", self.pieces.pieces[ix].print))
            .collect()
    }
    fn list_avail_pieces_for_print_with_ix(&self) -> Vec<String> {
        let mut pieces = &self.list_avail_pieces_for_print();
        pieces
            .into_iter()
            .enumerate()
            .map(|(ix, v)| format!("{}\t{}", ix.to_string(), v))
            .collect()
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
    fn read_choosen_piece(&self, available_pieces: &Vec<String>) -> usize {
        loop {
            print!("Piece index:\t");
            io::stdout().flush().unwrap();

            let mut choosen_piece = String::new();

            io::stdin()
                .read_line(&mut choosen_piece)
                .expect("Failed to read line");

            if let Ok(v) = choosen_piece.trim_end().parse::<usize>() {
                if v <= (available_pieces.len() - 1) {
                    return v;
                } else {
                    println!("Piece index out of bounds - try again")
                }
            } else {
                println!("Unable to parse piece name")
            }
        }
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
    fn place_piece_on_choosen_space(&mut self, piece_ix: usize) -> usize {
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
                        self.board.place_piece(piece_ix, x, y);
                        return *ix
                    }
                },
                None => println!("label provided is invlaid - try again")
            }
        }
    }
    fn game_board_string_proto(&self) -> String {
        let mut board = String::new();
        for ix in 0..self.xdim {
            board.push_str(BOARD_SEPARATOR);
            board.push_str("\n");
            board.push_str(ROW_TEMPLATE);
            board.push_str("\n");
        }
        board.push_str(BOARD_SEPARATOR);
        board.push_str("\n");
        board
    }
    fn game_board_string2(&self, pieces: Vec<Option<String>>, labels: Vec<Option<String>>) -> Result<String, String> {
        if pieces.len() != self.xdim * self.ydim || labels.len() != self.xdim * self.ydim {
            return Err("Pieces and labels must be the same size as the board".to_string())
        }

        let mut board = String::new();
        for ix in 0..self.xdim {
            board.push_str(BOARD_SEPARATOR);
            board.push_str("\n");
            // board.push_str(ROW_TEMPLATE);
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
            board.trim();
            board.push_str("\n");
        }
        board.push_str(BOARD_SEPARATOR);
        board.push_str("\n");
        Ok(board)
    }
    // fn game_board_string(&self, pieces: Vec<Option<String>>) -> Result<String, String> {
    //     if pieces.len() != self.xdim * self.ydim {
    //         return Err("pieces array is wrong length".to_string());
    //     }
    //     // TODO: use intersperse_with when fixed:
    //     // https://github.com/rust-lang/rust/issues/79524
    //     // Block Below:
    //     // // let vec_of_strings = self.game_board_string_proto()
    //     // //     .split(BOARD_SEPARATOR)
    //     // //     .into_iter()
    //     // //     .map(|s| s.to_string());
    //     // // let result = pieces.iter()
    //     // //     .map(|x| match x {
    //     // //         Some(s) => s.to_string(),
    //     // //         None => EMPTY_SQUARE.to_string()
    //     // //     })
    //     // //     .intersperse_with(|| vec_of_strings.next().unwrap())
    //     // //     .collect::<Vec<String>>()
    //     // //     .join("");
    //     let board = self.game_board_string_proto();
    //     let board_iter = board
    //         .split(BOARD_SEPARATOR)
    //         .into_iter()
    //         .map(|x| x.to_string());
    //     let mut result = String::new();
    //     for (b, p) in board_iter.zip(pieces) {
    //         result.push_str(&b);
    //         match p {
    //             Some(s) => result.push_str(&s),
    //             None => result.push_str(&EMPTY_SQUARE)
    //         }
    //     }
    //     Ok(result)
    // }
    fn pieces_by_position(&self) -> Vec<Option<String>> {
        // let pieces_by_ix = ;

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


        // self.board.board
        //     .iter()
        //     .map(|ix| {
        //         match ix {
        //             Some(n) => {
        //                 let this_str = &self.pieces.pieces[*n].print.to_string();
        //                 Some(this_str)
        //             },
        //             None => None
        //         }
        //     })
        //     .collect::<Vec<Option<&String>>>()


    }
    // fn game_board_pieces(&self) -> String {
    //     let mut board = &self.game_board_string_proto();
    //     let pieces_str = &self.board.board
    //         .into_iter()
    //         .

    // }




    // fn list_available_pieces(&self) -> Vec<&GamePiece> {
    //     let mut pieces = Vec::new();
    //     for (gp, loc_tup) in &self.placement {
    //         if loc_tup.is_none() {
    //             pieces.push(gp);
    //         }
    //     }
    // }
    // fn new() -> Game<'a> {
    //     let game_board = Gameboard::new_sq(4).unwrap();
    //     Game {
    //         piece_lookup: PieceLookup::new_sq(4),
    //         game_board: game_board
    //     }
    // }

    fn print_current_board(&self) -> String {
        let mut print_str : String = String::new();
        print_str
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



fn main() -> Result<(), Box<dyn Error>> {
    let piece1 = GamePiece::new_from_array(&[1,1,1,1])?;
    let piece2 = GamePiece::new_from_array(&[1,1,1,0])?;
    let piece3 = GamePiece::new_from_array(&[1,1,0,1])?;
    let piece4 = GamePiece::new_from_array(&[1,0,1,1])?;
    let piece5 = GamePiece::new_from_array(&[0,1,1,1])?;

    let a_1_4_p = vec![&piece1, &piece2, &piece3, &piece4];
    let a_2_5_p = vec![&piece5, &piece2, &piece3, &piece4];

    let a_1_4 = pieces_are_quadri(a_1_4_p)?;
    let a_2_5 = pieces_are_quadri(a_2_5_p)?;

    println!("Pieces 1 - 4 are quadri: {:?}", a_1_4);
    println!("Pieces 2- 5 are quadri: {:?}", a_2_5);

    // println!("Piece one values: {}", PieceValues::One.value());

    let mut game = Game::new_sq(4);

    println!("{:?}", game.board.quadri_coords);

    loop {
        let available_pieces_map = &game.list_available_pieces_for_print_2();
        if available_pieces_map.len() > 0 {
            let mut available_pieces_v : Vec<(&usize, &String, String)> = available_pieces_map
                .into_iter()
                .map(|(ix, s)| (ix, s, format!("{}\t{}", ix, s)))
                .collect();
            available_pieces_v.sort_by_key(|k| k.0);
            let available_pieces = available_pieces_v.iter().map(|(ix, s, ixs)| ixs.clone()).collect();
            // let available_pieces = &game.list_avail_pieces_for_print();
            // let available_pieces_w_ix = &game.list_avail_pieces_for_print_with_ix();
            println!("Pick a piece for your opponent to place");
            // let print_str = &game.choose_a_piece(available_pieces_w_ix);
            let print_str = &game.choose_a_piece(&available_pieces);
            println!("{}", print_str);
            // let choosen_piece_ix = &game.read_choosen_piece(&available_pieces);
            let choosen_piece_ix = &game.read_choosen_piece2(&available_pieces_map);
            // loop {
            //     println!("Invalid input - type the index of the piece only.");
            //     choosen_piece_ix_r = &game.read_choosen_piece(&available_pieces).clone();
            // }
            // let choosen_piece_ix = choosen_piece_ix_r.as_ref().unwrap();
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

            let _ix = &mut game.place_piece_on_choosen_space(*choosen_piece_ix);
            let pieces = &game.pieces_by_position();
            println!("Current Board:");
            println!("{}", &game.game_board_string2(pieces.to_vec(), empty_labels)?);
            //TODO: Check for Quadris
            // break if successful.
        } else {
            println!{"Draw!"};
            break;
        }
    }




    // game.board.ix_as_alpha.iter().map(|x| Some(x.to_string())).collect();
    // println!("{:?}", labels);
    // println!("{:?}", (1..(4*4)+1));
    // println!("{}", num_to_alpha(27)?);
    // let board_string = &game.game_board_string2(vec![None; 16], labels)?;
    // println!("{}", board_string);

    // let game_board_string = &game.game_board_string(game.pieces_by_position())?;
    // println!("{}", game_board_string);
    // let print_str2 = &game.

    // let game_print = String::new();


    // println!("{:?}", game.piece_lookup.placement);


    // println!("{} {} !", "it".green().on_truecolor(135, 28, 167), "works".blue().bold().on_green());



    Ok(())
}


// 8642686295
// Thornblade Country Club
// Ford Taurus
// 0dIDM0@wp0GC@dRErcFx