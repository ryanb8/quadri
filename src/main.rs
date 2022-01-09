use std::error::Error;

// enum AttributeValue {
//     one(i8) = 1,
//     two(i8) = 2
// }

static allowed_attribute_values: (i8, i8) = (1, 2);

struct GamePiece {
    name: String,
    at1: i8,
    at2: i8,
    at3: i8,
    at4: i8,
}

fn create_game_piece(values: (i8,i8,i8,i8), name: String) -> GamePiece {
    GamePiece {
        name: name,
        at1: values.0,
        at2: values.1,
        at3: values.2,
        at4: values.3
    }
}

fn all_equal(arr: &[i8]) -> Result<bool, String> {
    let arr_len = arr.len();
    if arr_len == 0 {
        return Err("Array must be non-empty".to_string());
    }
    let mut result = true;
    let prior_value = arr[0];
    for i in 1..(arr_len - 1) {
        let next_value = arr[i];
        if prior_value != next_value {
            result = false;
            break;
        }
    }
    Ok(result)
}

fn pieces_are_quadri(p1: &GamePiece, p2: &GamePiece, p3: &GamePiece, p4: &GamePiece) -> bool {
    let l1 = [p1.at1, p2.at1, p3.at1, p4.at1];
    let l2 = [p1.at2, p2.at2, p3.at2, p4.at2];
    let l3 = [p1.at3, p2.at3, p3.at3, p4.at3];
    let l4 = [p1.at4, p2.at4, p3.at4, p4.at4];

    all_equal(&l1).unwrap() || all_equal(&l2).unwrap() || all_equal(&l3).unwrap() || all_equal(&l4).unwrap()
//     p1.at1 + p2.at1 + p3.at1 + p4.at1 == 0 | p1.at1 + p2.at1 + p3.at1 + p4.at1 == 4
//     p1.at2 + p2.at2 + p3.at2 + p4.at2 == 0 | p1.at2 + p2.at2 + p3.at2 + p4.at2 == 4
//     p1.at3 + p2.at3 + p3.at3 + p4.at3 == 0 | p1.at3 + p2.at3 + p3.at3 + p4.at3 == 4
//     p1.at4 + p2.at4 + p3.at4 + p4.at4 == 0 | p1.at4 + p2.at4 + p3.at4 + p4.at4 == 4
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let piece1 = create_game_piece((1,1,1,1), "tall_round_dark_full".to_string());
    let piece2 = create_game_piece((1,1,1,0), "tall_round_dark_hollow".to_string());
    let piece3 = create_game_piece((1,1,0,1), "tall_round_light_full".to_string());
    let piece4 = create_game_piece((1,0,1,1), "tall_square_dark_full".to_string());
    let piece5 = create_game_piece((0,1,1,1), "short_round_dark_full".to_string());

    let a_1_4 = pieces_are_quadri(&piece1, &piece2, &piece3, &piece4);
    let a_2_5 = pieces_are_quadri(&piece5, &piece2, &piece3, &piece4);

    println!("Pieces 1 - 4 are quadri: {}", a_1_4);
    println!("Pieces 2- 5 are quadri: {}", a_2_5);

    Ok(())
}