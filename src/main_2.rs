use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use colored::Colorize;
use colored::ColoredString;

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
static EMPTY_SQUARE: &'static str = "   ";
static RGB_WHITE: (u8, u8, u8) =  (255, 255, 255);
static RGB_GREY: (u8, u8, u8) = (255, 204, 0);

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
        res.push_str("capital_");
    } else {
        res.push_str("lowercase_");
    }
    //4
    if v[3] == 0 {
        res.push_str("x_");
    } else {
        res.push_str("o_");
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
        (0,0) => "x".to_string(),
        (0,1) => "X".to_string(),
        (1,0) => "o".to_string(),
        (1,1) => "O".to_string(),
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
            print: print,
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

struct Pieces {
    pieces: Vec<GamePiece>
}

impl Pieces {
    fn new_sq(dim: usize) -> Pieces {
        let num_pieces: usize = dim * dim;
        let pieces1 : Vec<Vec<i8>> =
            (0..num_pieces)
            .map(|ix| convert_to_binary(ix))
            .map(|v| left_pad(v, 0, 4))
            .collect();

        let pieces = pieces1
            .into_iter()
            .map(|v| GamePiece::new_from_vec(v))
            .collect::<Result<Vec<GamePiece>, String>>()
            .unwrap();

        Pieces {
            pieces: pieces
        }
    }
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

struct Gameboard {
    board: Vec<Vec<Option<usize>>>
}

impl Gameboard {
    fn new(dim_x: usize, dim_y: usize) -> Gameboard {
        let mut this_board = Vec::new();
        for ix in 0..dim_x as usize {
            this_board.push(Vec::new());
            for _jx in 0..dim_y as usize {
                this_board[ix].push(None);
            }
        }
        Gameboard {
            board: this_board
        }
    }
    fn new_sq(dim: usize) -> Gameboard {
        Gameboard::new(dim, dim)
    }

}

struct Game {
    pieces: Pieces,
    board: Vec<Vec<Option<usize>>>,
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
    fn list_available_pieces(&self) -> Vec<&GamePiece> {
        let mut pieces = Vec::new();
        for (gp, loc_tup) in &self.placement {
            if loc_tup.is_none() {
                pieces.push(gp);
            }
        }
}