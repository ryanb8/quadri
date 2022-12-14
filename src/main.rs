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

fn main() -> Result<(), Box<dyn Error>> {
    println! {"hey world"};
    Ok(())
}
