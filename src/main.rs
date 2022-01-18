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

static ALLOWED_ATTRIBUTE_VALUES: [i8; 2] = [0, 1];
static ALPHABET_ARRAY: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
static BOARD_SEPARATOR: &'static str = "+---+---+---+---+";
static ROW_TEMPLATE: &'static str = "| {} | {} | {} | {} |";
static EMPTY_SQUARE: &'static str = "   ";
static RGB_WHITE: (u8, u8, u8) =  (255, 255, 255);
static RGB_GREY: (u8, u8, u8) = (170, 170, 170);

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
        0 => res.red(),
        1 => res.blue(),
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
        println!("{:?}", dim);
        println!("{:?}", values);
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
}


#[derive(Debug)]
struct Gameboard<'a> {
    xdim: usize,
    ydim: usize,
    arrangement: Vec<Vec<Option<&'a GamePiece>>>
}

impl<'a> Gameboard<'a> {
    fn new(dim_x: i8, dim_y: i8) -> Result<Gameboard<'a>, String>{
        if dim_x <= 0 || dim_y <= 0 {
            return Err("Dims must be greater than or equal to 1".to_string());
        }
        let mut this_arrangement = Vec::new();
        for ix in 0..dim_x as usize {
            this_arrangement.push(Vec::new());
            for _jx in 0..dim_y as usize {
                this_arrangement[ix].push(None);
            }
        }
        Ok(
            Gameboard {
                xdim: dim_x as usize,
                ydim: dim_y as usize,
                arrangement: this_arrangement
            }
        )
    }
    fn has_valid_indicies(&self, x: usize, y: usize) -> bool {
        let mut result = true;
        // 0 indexed - usize enforces < 0
        if x >= self.xdim || y >= self.ydim {
            result = false;
        }
        result
    }
    // fn validate_indices(&self, x: usize, y: usize) -> Result<(), String> {
    //     if !&self.has_valid_indicies(x, y) {
    //         return Err("Piece index out of bounds".to_string());
    //     }
    //     Ok(())
    // }
    fn new_sq(dim: i8) -> Result<Gameboard<'a>, String>{
        Gameboard::new(dim, dim)
    }
    fn get_piece(&self, x: usize, y: usize) -> Result<Option<&GamePiece>, String> {
        if !&self.has_valid_indicies(x, y) {
            return Err("Piece index out of bounds".to_string());
        }
        Ok(self.arrangement[x][y])
    }
    fn get_pieces(&self, positions: Vec<[usize; 2]>) -> Result<Vec<Option<&GamePiece>>, String> {
        positions
            .into_iter()
            .map(|pos| self.get_piece(pos[0], pos[1]))
            .collect()
    }
    fn place_piece(&mut self, piece: &'a GamePiece, x: usize, y: usize) -> Result<(), String> {
        let current_piece = &self.get_piece(x, y)?;

        if let Some(_game_piece) = current_piece {
            return Err("Piece already on board".to_string());
        }
        self.arrangement[x][y] = Some(piece);
        Ok(())
    }
    fn remove_piece(&mut self, x: usize, y: usize) -> Result<(), String> {
        let current_piece = self.get_piece(x, y)?;

        if let None = current_piece {
            return Err("Square is already piece-less".to_string());
        }
        self.arrangement[x][y] = None;
        Ok(())
    }
    fn list_spaces(&self) -> Vec<(char, (usize, usize))> {
        let mut spaces = Vec::new();
        for ix in 0..self.xdim {
            for jx in 0..self.ydim {
                spaces.push((ALPHABET_ARRAY[(ix * jx * self.xdim) + jx], (ix, jx)))
            }
        }
        spaces
    }
    // fn list_pieces(&self) -> Vec<(usize, &GamePiece)> {
    //     let mut pieces = Vec::new();
    //     for (gp, loc_ix_tup) in &self.placement {
    //         pieces.push((loc_ix_tup.1, gp));
    //     }
    //     pieces
    // }
}

struct PieceLookup {
    placement: HashMap<GamePiece, (Option<[usize; 2]>, usize)>
    // pieces: Vec<GamePiece>,
}

impl PieceLookup {
    fn new_sq(dim: usize) -> PieceLookup {
        let pieces1 : Vec<Vec<i8>> =
            (0..dim)
            .map(|ix| convert_to_binary(ix))
            .map(|v| left_pad(v, 0, 4))
            .collect();

        println!("{:?}", pieces1);

        let pieces = pieces1
            .into_iter()
            .map(|v| GamePiece::new_from_vec(v))
            .collect::<Result<Vec<GamePiece>, String>>()
            .unwrap();

        let mut placement: HashMap<GamePiece, (Option<[usize; 2]>, usize)> = HashMap::new();
        for (ix, gp) in pieces.into_iter().enumerate() {
            placement.insert(gp, (None, ix));
        }
        PieceLookup {
            placement: placement
        }
    }
    fn list_pieces(&self) -> Vec<(usize, &GamePiece)> {
        let mut pieces = Vec::new();
        for (gp, loc_ix_tup) in &self.placement {
            pieces.push((loc_ix_tup.1, gp));
        }
        pieces
    }
    fn list_available_pieces(&self) -> Vec<(usize, &GamePiece)> {
        let mut pieces = Vec::new();
        for (gp, loc_ix_tup) in &self.placement {
            if loc_ix_tup.0.is_none() {
                pieces.push((loc_ix_tup.1, gp));
            }
        }
        pieces
    }
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

struct Game<'a> {
    piece_lookup: PieceLookup,
    game_board: Gameboard<'a>
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        let game_board = Gameboard::new_sq(4).unwrap();
        Game {
            piece_lookup: PieceLookup::new_sq(4),
            game_board: game_board
        }
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

    let game = Game::new();


    Ok(())
}