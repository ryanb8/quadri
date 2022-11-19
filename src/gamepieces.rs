use colored::ColoredString;
use colored::Colorize;
use std::hash::{Hash, Hasher};

use crate::utils;

static ALLOWED_ATTRIBUTE_VALUES: [i8; 2] = [0, 1];
static RGB_WHITE: (u8, u8, u8) = (255, 255, 255);
static RGB_GREY: (u8, u8, u8) = (255, 204, 0);

#[derive(Debug)]
struct AV(i8); //allowed values

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
}

#[derive(Debug, Clone)]
pub struct GamePiece {
    pub name: String,
    pub ats: Vec<i8>,
    pub dim: i8,
    pub print: ColoredString,
}

impl PartialEq for GamePiece {
    fn eq(&self, other: &Self) -> bool {
        self.ats == other.ats
    }
}
impl Eq for GamePiece {}
impl Hash for GamePiece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ats.hash(state);
        self.dim.hash(state);
    }
}

impl GamePiece {
    pub fn new_from_vec(values: Vec<i8>) -> Result<GamePiece, String> {
        let dim: i8 = values.len().try_into().unwrap();
        if dim != 4 {
            return Err("Need 4 values for a piece".to_string());
        }
        for v in &values {
            if !AV::in_allowed_values(*v) {
                return Err(format!(
                    "Values for pieces must be in {:?}",
                    ALLOWED_ATTRIBUTE_VALUES
                ));
            }
        }
        let name = GamePiece::get_piece_name(&values);
        let print = GamePiece::get_piece_print(&values);
        Ok(GamePiece {
            name: name,
            ats: values,
            dim: dim,
            print: print,
        })
    }
    fn get_piece_name(validated_values: &Vec<i8>) -> String {
        let mut res = String::new();
        //2
        if validated_values[1] == 0 {
            res.push_str("red_");
        } else {
            res.push_str("blue_");
        }
        //3
        if validated_values[2] == 0 {
            res.push_str("empty_");
        } else {
            res.push_str("full_");
        }
        //4
        if validated_values[3] == 0 {
            res.push_str("rec_");
        } else {
            res.push_str("circle_");
        }
        //1
        if validated_values[0] == 0 {
            res.push_str("onWhite");
        } else {
            res.push_str("onGrey");
        }
        res
    }
    fn get_piece_print(validated_values: &Vec<i8>) -> ColoredString {
        let res = {
            if (validated_values[3], validated_values[2]) == (0, 0) {
                "▯".to_string()
            } else if (validated_values[3], validated_values[2]) == (0, 1) {
                "▮".to_string()
            } else if (validated_values[3], validated_values[2]) == (1, 0) {
                "○".to_string()
            } else {
                "●".to_string()
            }
        };

        let mut res_c = {
            if validated_values[1] == 0 {
                res.red().bold()
            } else {
                res.blue().bold()
            }
        };

        res_c = {
            if validated_values[0] == 0 {
                res_c.on_truecolor(RGB_WHITE.0, RGB_WHITE.1, RGB_WHITE.2)
            } else {
                res_c.on_truecolor(RGB_GREY.0, RGB_GREY.1, RGB_GREY.2)
            }
        };
        res_c
    }
}

pub struct Pieces {
    pub pieces: Vec<GamePiece>,
}

impl Pieces {
    pub fn new() -> Pieces {
        let num_pieces: usize = 4 * 4;
        let pieces: Vec<GamePiece> = (0..num_pieces)
            .map(|ix| utils::convert_to_binary(ix))
            .map(|v| utils::left_pad(v, 4))
            .map(|v| GamePiece::new_from_vec(v))
            .collect::<Result<Vec<GamePiece>, String>>()
            .unwrap();

        Pieces { pieces: pieces }
    }
    pub fn get_piece_ref(&self, ix: usize) -> &GamePiece {
        //TODO: ensure ix is in correct range
        &self.pieces[ix]
    }
    pub fn get_pieces_refs(&self, ixs: Vec<usize>) -> Vec<&GamePiece> {
        ixs.iter().map(|ix| self.get_piece_ref(*ix)).collect()
    }
}
