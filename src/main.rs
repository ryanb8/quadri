use std::error::Error;

static allowed_attribute_values: [i8; 2] = [0, 1];

#[derive(Debug)]
struct AV(i8);  //allowed values

impl AV {
    fn in_allowed_values(value: i8) -> bool {
        let mut is_valid = false;
        for av in allowed_attribute_values {
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
struct GamePiece {
    name: String,
    at1: i8,
    at2: i8,
    at3: i8,
    at4: i8,
}

impl GamePiece {
    fn new(values: &[i8], name: String) -> Result<GamePiece, String> {
        if values.len() != 4 {
            return Err("Need 4 values for a piece".to_string());
        }
        for v in values {
            if !AV::in_allowed_values(*v) {
                return Err(format!("Values for pieces must be in {:?}", allowed_attribute_values));
            }
        }
        Ok(GamePiece {
            name: name,
            at1: values[0],
            at2: values[1],
            at3: values[2],
            at4: values[3]
        })
    }
}

fn all_equal(arr: &[i8; 4]) -> Result<bool, String> {
    let arr_len = arr.len();
    if arr_len == 0 {
        return Err("Array must be non-empty".to_string());
    }
    let mut result = true;
    let mut prior_value = arr[0];
    for i in 1..arr_len {
        let next_value = arr[i];
        if prior_value != next_value {
            result = false;
            break;
        }
        prior_value =  next_value;
    }
    Ok(result)
}

fn pieces_are_quadri(p1: &GamePiece, p2: &GamePiece, p3: &GamePiece, p4: &GamePiece) -> bool {
    let l1 = [p1.at1, p2.at1, p3.at1, p4.at1];
    let l2 = [p1.at2, p2.at2, p3.at2, p4.at2];
    let l3 = [p1.at3, p2.at3, p3.at3, p4.at3];
    let l4 = [p1.at4, p2.at4, p3.at4, p4.at4];

    all_equal(&l1).unwrap() || all_equal(&l2).unwrap() || all_equal(&l3).unwrap() || all_equal(&l4).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let piece1 = GamePiece::new(&[1,1,1,1], "tall_round_dark_full".to_string())?;
    let piece2 = GamePiece::new(&[1,1,1,0], "tall_round_dark_hollow".to_string())?;
    let piece3 = GamePiece::new(&[1,1,0,1], "tall_round_light_full".to_string())?;
    let piece4 = GamePiece::new(&[1,0,1,1], "tall_square_dark_full".to_string())?;
    let piece5 = GamePiece::new(&[0,1,1,1], "short_round_dark_full".to_string())?;

    let a_1_4 = pieces_are_quadri(&piece1, &piece2, &piece3, &piece4);
    let a_2_5 = pieces_are_quadri(&piece5, &piece2, &piece3, &piece4);

    println!("Pieces 1 - 4 are quadri: {}", a_1_4);
    println!("Pieces 2- 5 are quadri: {}", a_2_5);

    Ok(())
}