use std::error::Error;
use std::collections::HashMap;

// TODO: Handle errors intelligently, none of this String mess
// TODO: Tests
// TODO: Split code into multiple files

static ALLOWED_ATTRIBUTE_VALUES: [i8; 2] = [0, 1];

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
#[derive(Hash)]
struct GamePiece {
    name: String,
    ats: Vec<i8>,
    dim: i8
}

impl PartialEq for GamePiece {
    fn eq(&self, other: &Self) -> bool {
        self.ats == other.ats
    }
}
impl Eq for GamePiece {}

impl GamePiece {
    fn new_from_array(values: &[i8], name: String) -> Result<GamePiece, String> {
        let dim: i8 = values.len().try_into().unwrap();
        if dim != 4 {
            return Err("Need 4 values for a piece".to_string());
        }
        for v in values {
            if !AV::in_allowed_values(*v) {
                return Err(format!("Values for pieces must be in {:?}", ALLOWED_ATTRIBUTE_VALUES));
            }
        }
        Ok(GamePiece {
            name: name,
            ats: values.to_vec(),
            dim: dim
        })
    }
    fn new_from_vec(values: Vec<i8>, name: String) -> Result<GamePiece, String> {
        let dim: i8 = values.len().try_into().unwrap();
        if dim != 4 {
            return Err("Need 4 values for a piece".to_string());
        }
        for v in &values {
            if !AV::in_allowed_values(*v) {
                return Err(format!("Values for pieces must be in {:?}", ALLOWED_ATTRIBUTE_VALUES));
            }
        }
        Ok(GamePiece {
            name: name,
            ats: values,
            dim: dim
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
}

struct PieceLookup {
    placement: HashMap<GamePiece, (Option<[usize; 2]>, usize)>
    // pieces: Vec<GamePiece>,
}

impl PieceLookup {
    fn new_sq(dim: usize) -> PieceLookup {
        let pieces : Vec<GamePiece> =
            (0..dim)
            .map(|ix| convert_to_binary(ix))
            .map(|v| GamePiece::new_from_vec(v, "placeholder".to_string()))
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
}

// struct PiecePlacement<'a> {
//     placement: HashMap<&'a GamePiece, Option<[usize; 2]>>
// }

// impl PieceList {
//     fn new_sq(dim: usize) -> PiecePool<'a> {
//         let pieces : Vec<GamePiece> = (0..dim)
//             .map(|ix| convert_to_binary(ix))
//             .map(|v| GamePiece::new_from_vec(v, "placeholder".to_string()))
//             .collect::<Result<Vec<GamePiece>, String>>()
//             .unwrap();
//         PieceList {pieces: pieces}
//     }
// }



// impl<'a> PiecePlacement<'a> {
//     fn new_sq(p_list: &PieceList) -> PiecePlacement<'a> {
//         PiecePool {
//             placements: placements
//         }

//         let pieces : Vec<GamePiece> = (0..dim)
//             .map(|ix| convert_to_binary(ix))
//             .map(|v| GamePiece::new_from_vec(v, "placeholder".to_string()))
//             .collect::<Result<Vec<GamePiece>, String>>()
//             .unwrap();

//         // let PieceList = PieceList::new(pieces);

//         let mut placements: HashMap<&'a GamePiece, Option<[usize; 2]>> = HashMap::new();
//         for gp_ref in pieces.iter() {
//             placements.insert(&gp_ref, None);
//         }
//         // let placements: HashMap<&'a GamePiece, Option<[usize; 2]>> =
//         //     pieces_references
//         //         .into_iter()
//         //         .map(|p| (p, None))
//         //         .collect();


//     }
// }


fn convert_to_binary(x: usize) -> Vec<i8> {
    // usize => only 0 or positive integers
    let mut binary = Vec::new();
    let mut v = x;
    while v > 0  {
        let r = v % 2;
        v = (v - r)/2;
        binary.push(r as i8);
    }
    binary.reverse();
    binary
}

// struct Game<'a> {
//     piece_pool: PiecePool<'a>,
//     game_board: Gameboard<'a>
// }

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
    let piece1 = GamePiece::new_from_array(&[1,1,1,1], "tall_round_dark_full".to_string())?;
    let piece2 = GamePiece::new_from_array(&[1,1,1,0], "tall_round_dark_hollow".to_string())?;
    let piece3 = GamePiece::new_from_array(&[1,1,0,1], "tall_round_light_full".to_string())?;
    let piece4 = GamePiece::new_from_array(&[1,0,1,1], "tall_square_dark_full".to_string())?;
    let piece5 = GamePiece::new_from_array(&[0,1,1,1], "short_round_dark_full".to_string())?;

    let a_1_4_p = vec![&piece1, &piece2, &piece3, &piece4];
    let a_2_5_p = vec![&piece5, &piece2, &piece3, &piece4];

    let a_1_4 = pieces_are_quadri(a_1_4_p)?;
    let a_2_5 = pieces_are_quadri(a_2_5_p)?;

    println!("Pieces 1 - 4 are quadri: {:?}", a_1_4);
    println!("Pieces 2- 5 are quadri: {:?}", a_2_5);

    Ok(())
}