use std::error::Error;

// use quadri::game::check_for_all_quadris;
// use quadri::game::Game;

// // struct PieceValue2 (i8);

// // impl PieceValue2 {
// //     fn value(&self) -> i8 {
// //         self.0
// //     }
// // }

// // enum PieceValues {
// //     One,
// //     Two,
// //     Three,
// //     Four,
// // }

// // impl PieceValues {
// //     fn value(&self) -> i8 {
// //         match &self {
// //             PieceValues::One => 1 as i8,
// //             PieceValues::Two => 2 as i8,
// //             PieceValues::Three => 3 as i8,
// //             PieceValues::Four => 4 as i8,
// //         }
// //     }
// // }

// // fn print_type_of<T>(_: &T) {
// //     println!("{}", std::any::type_name::<T>())
// // }

// // TODO: Handle errors intelligently, none of this String mess
// // TODO: Tests
// // TODO: Split code into multiple files

// // -- red/blue | capital/lowercase | X/O | underli
// // x_ x- X_ X-   <- red
// // x_ x- X_ X-   <- blue
// // o_ o- O_ O-   <- red
// // o_ o- O_ O-   <- blue

// // -- red/blue | capital/lowercase | X/O | underline/bold
// // +----+----+----+----+
// // | x_ | x- | X_ | X- |  <- red
// // +----+----+----+----+
// // | x_ | x- | X_ | X- |  <- blue
// // +----+----+----+----+
// // | o_ | o- | O_ | O- |  <- red
// // +----+----+----+----+
// // | o_ | o- | O_ | O- |  <- blue
// // +----+----+----+----+

// /// yell  grey  yell  grey
// // +---+---+---+---+
// // | ○ | ○ | ● | ● |  <- red
// // +---+---+---+---+
// // | ○ | ○ | ● | ● |  <- blue
// // +---+---+---+---+
// // | ▯ | ▯ | ▮ | ▮ |  <- red
// // +---+---+---+---+
// // | ▯ | ▯ | ▮ | ▮ |  <- blue
// // +---+---+---+---+

// //TODO - move game to standlone module
// //TODO -  Clean up main and make it better
// // TODO - Extract out CLI components (print statements, etc) - make it a CLI frontend
// // TODO - Svelte based actual front end?

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::start_game();
    loop {
        let available_pieces_map = &game.list_available_pieces_for_print_2();
        if available_pieces_map.len() > 0 {
            let mut available_pieces_v: Vec<(&usize, &String, String)> = available_pieces_map
                .into_iter()
                .map(|(ix, s)| (ix, s, format!("{}\t{}", ix, s)))
                .collect();
            available_pieces_v.sort_by_key(|k| k.0);
            let available_pieces = available_pieces_v
                .iter()
                .map(|(_ix, _s, ixs)| ixs.clone())
                .collect();
            println!("Pick a piece for your opponent to place");
            let print_str = &game.choose_a_piece(&available_pieces);
            println!("{}", print_str);
            let choosen_piece_ix = &game.read_choosen_piece2(&available_pieces_map);
            println!(
                "Opponent must place piece {}",
                &available_pieces_map
                    .get(choosen_piece_ix)
                    .ok_or("I screwed up!")?
            );

            let mut labels = Vec::new();
            let mut empty_labels = Vec::<Option<String>>::new();
            for s in &game.board.ix_as_alpha {
                labels.push(Some(s.to_string()));
                empty_labels.push(None);
            }
            let pieces = &game.pieces_by_position();

            println!(
                "Pick a place for piece {}",
                &available_pieces_map
                    .get(choosen_piece_ix)
                    .ok_or("I screwed up!")?
            );
            println!("{}", &game.game_board_string2(pieces.to_vec(), labels)?);

            let _ix = &mut game.place_piece_on_choosen_space(*choosen_piece_ix)?;
            let pieces = &game.pieces_by_position();
            println!("Current Board:");
            println!(
                "{}",
                &game.game_board_string2(pieces.to_vec(), empty_labels)?
            );
            //TODO - this should be inside the game struct
            let (are_quadris, _coords) = check_for_all_quadris(&game);
            if are_quadris {
                println!("Game is done! Winner!");
                break;
            }
        } else {
            println! {"Draw!"};
            break;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println! {"hey world"};
    Ok(())
}
