// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::io;
// use std::io::Write;

// use crate::gameboard::BoardState;
// use crate::gameboard::GameboardAndPieces;
// use crate::gameboard::PieceState;
// use crate::gameboard::X_DIM;
// use crate::gameboard::Y_DIM;
// use crate::gamepieces::GamePiece;
// use crate::utils;

// static BOARD_SEPARATOR: &'static str = "+---+---+---+---+";
// static PHASE_PICK: &'static str = "pick_piece_for_opponent";
// static PHASE_PLACE: &'static str = "place_piece_on_board";

// // #[derive(Debug, Clone)]
// // pub struct PieceState {
// //     piece: GamePiece,
// //     on_board: bool,
// //     location_coord: Option<[usize; 2]>,
// //     piece_ix: usize,
// // }
// #[derive(Debug, Clone)]
// pub struct TurnState {
//     turn: usize,
//     phase: String,
//     current_actor: usize,
// }
// #[derive(Debug, Clone)]
// pub struct WinnerState {
//     winner: Option<usize>,
//     winning_quadris: Option<Vec<Vec<[usize; 2]>>>,
// }
// #[derive(Debug, Clone)]
// pub struct GameState<'a> {
//     piece_states: Vec<PieceState<'a>>,
//     turn_state: TurnState,
//     winner_state: WinnerState,
// }
// #[derive(Debug, Clone)]
// pub struct Game {
//     // pieces: Vec<GamePiece>,
//     board_and_pieces: GameboardAndPieces,
//     turn_state: TurnState,
// }

// // pub struct GamePlayCLI(Game);

// pub struct GamePlayCLI {
//     game: Game
// }

// impl GamePlayCLI {
//     pub fn new_cli_game() -> GamePlayCLI {
//         GamePlayCLI {
//             game: Game::start_game()
//         }
//     }
//     pub fn play_game(&self) -> () {

//     }
//     fn report_turn() -> (){}
//     fn show_board(&self) -> () {
//         let board_print = self.get_print_str_for_board();
//         println!("{}", board_print);
//     }
//     fn get_print_str_for_board(&self) -> String {
//         let piece_states = self.game.bo
//         self.game.board_and_pieces.

//         let mut board = String::new();
//         for ix in 0..X_DIM {
//             board.push_str(BOARD_SEPARATOR);
//             board.push_str("\n");
//             board.push_str("| ");
//             for jx in 0..Y_DIM {
//                 let this_ind = (ix * Y_DIM) + jx;
//                 match (pieces[this_ind].as_ref(), labels[this_ind].as_ref()) {
//                     (Some(s), _) => board.push_str(&s),
//                     (None, Some(l)) => board.push_str(&l),
//                     (None, None) => board.push_str(" "),
//                 }
//                 board.push_str(" | ");
//             }
//             board = board.trim().to_string();
//             board.push_str("\n");
//         }
//         board.push_str(BOARD_SEPARATOR);
//         board.push_str("\n");
//     }
//     fn show_available_pieces(&self) -> () {
//         let available_pieces_map = self.game.list_available_pieces_as_map();
//         let piece_print = GamePlayCLI::get_print_str_for_pieces(available_pieces_map);
//         println!("{}", piece_print);
//     }
//     fn get_print_str_for_pieces(available_pieces_map: HashMap<usize, &GamePiece>) -> String {
//         available_pieces_map
//             .iter()
//             .map(|(k, v)| format!("{}\t{}", k, v.print))
//             .collect::<Vec<String>>()
//             .join("\n")
//     }
//     fn pick_piece_for_opponent() -> (){};
//     fn pick_place_for_piece() -> (){};
// }

// // public api:
// // start game
// // get game state
// // - list available pieces
// // - list board and pieces
// // - list turn /move
// // - any winners
// // pick piece for opponent
// // place piece on board - option to return winners or not

// impl TurnState {
//     pub fn setup() -> TurnState {
//         TurnState {
//             turn: 1,
//             phase: PHASE_PICK.to_string(),
//             current_actor: 1,
//         }
//     }
//     pub fn increment_turn(&mut self) -> () {
//         self.increment_actor();
//         self.increment_turn_and_phase();
//     }
//     fn increment_actor(&mut self) -> () {
//         self.current_actor = match self.current_actor {
//             1 => 2,
//             2 => 1,
//             _ => panic!("Quadri is a two player game"),
//         }
//     }
//     fn increment_turn_and_phase(&mut self) -> () {
//         if self.phase == PHASE_PICK.to_string() {
//             self.phase = PHASE_PLACE.to_string();
//         } else {
//             self.turn = self.turn + 1;
//             self.phase = PHASE_PICK.to_string();
//         }
//     }
// }

// impl Game {
//     // Setup
//     pub fn start_game() -> Game {
//         Game {
//             // pieces: Game::create_pieces(),
//             board_and_pieces: GameboardAndPieces::new(),
//             turn_state: TurnState::setup(),
//         }
//     }
//     // fn create_pieces() -> Vec<GamePiece> {
//     //     let num_pieces: usize = 4 * 4;
//     //     (0..num_pieces)
//     //         .map(|ix| utils::convert_to_binary(ix))
//     //         .map(|v| utils::left_pad(v, 4))
//     //         .map(|v| GamePiece::new_from_vec(v))
//     //         .collect::<Result<Vec<GamePiece>, String>>()
//     //         .unwrap()
//     // }

//     // Get states
//     // fn get_unused_piece_ixs(&self) -> Vec<usize> {
//     //     (0..self.pieces.len())
//     //         .filter_map(
//     //             |piece_ix| match self.board.piece_ix_to_board_ix.contains_key(&piece_ix) {
//     //                 true => None,
//     //                 false => Some(piece_ix),
//     //             },
//     //         )
//     //         .collect::<Vec<usize>>()
//     // }
//     // //external representation
//     // pub fn get_unused_pieces(&self) -> Vec<(usize, GamePiece)> {
//     //     let mut sorted_ixs = self.get_unused_piece_ixs();
//     //     sorted_ixs.sort();
//     //     sorted_ixs
//     //         .into_iter()
//     //         .map(|ix| (ix, self.pieces[ix].clone()))
//     //         .collect::<Vec<(usize, GamePiece)>>()
//     // }

//     pub fn get_game_state(&self) -> GameState {
//         GameState {
//             piece_states: self.board_and_pieces.get_piece_states(),
//             turn_state: self.get_turn_and_actor(),
//             winner_state: self.get_winners(),
//         }
//     }

//     // pub fn get_piece_states(&self) -> Vec<PieceState> {
//     //     let mut piece_states = Vec::<PieceState>::new();
//     //     for (piece_ix, piece) in self.board.pieces.iter().enumerate() {
//     //         piece_states.push(PieceState {
//     //             piece: piece.clone(),
//     //             on_board: self.board.piece_ix_to_board_ix.contains_key(&piece_ix),
//     //             location_coord: self.board.get_coord_for_piece_ix(&piece_ix),
//     //             piece_ix: piece_ix,
//     //         });
//     //     }
//     //     piece_states
//     // }

//     pub fn get_turn_and_actor(&self) -> TurnState {
//         self.turn_state.clone()
//     }

//     //TODO actually implement this.
//     pub fn get_winners(&self) -> WinnerState {
//         WinnerState {
//             winner: None,
//             winning_quadris: None,
//         }
//     }

//     // fn pieces_are_quadri(pieces: Vec<&GamePiece>) -> Result<bool, String> {
//     //     // More generic than just 4 pieces - checks that all pieces (1 or more)
//     //     // have the same value in at least one position
//     //     let number_of_pieces = pieces.len();
//     //     if number_of_pieces != X_DIM {
//     //         return Err("Must provide at least one piece".to_string());
//     //     }
//     //     let piece_dimension = pieces[0].dim;
//     //     for jx in 0..piece_dimension as usize {
//     //         let mut is_quadri = true;
//     //         for ix in 1..number_of_pieces as usize {
//     //             if pieces[ix - 1].ats[jx] != pieces[ix].ats[jx] {
//     //                 is_quadri = false;
//     //                 break;
//     //             }
//     //         }
//     //         if is_quadri {
//     //             return Ok(true);
//     //         }
//     //     }
//     //     Ok(false)
//     // }

//     // fn check_all_quadris(game: &Game) -> (bool, Vec<Vec<[usize; 2]>>) {
//     //     // for each quadri location
//     //     // if there are four pieces
//     //     // get the pieces by ref and check if quadri
//     //     // if quadri, append to ouput list
//     //     let quadri_coords = &game.board.quadri_coords;

//     //     let piece_index_sets = game
//     //         .board
//     //         .quadri_coords
//     //         .iter()
//     //         .enumerate()
//     //         .map(|(ix, v)| {
//     //             let this_piece_v_res = game.board.get_piece_ixs_by_coords(v);
//     //             let this_piece_v_unwrap = this_piece_v_res.unwrap();
//     //             (ix, this_piece_v_unwrap)
//     //         })
//     //         .collect::<Vec<(usize, Vec<Option<usize>>)>>();

//     //     let piece_index_sets2 = piece_index_sets
//     //         .iter()
//     //         .filter_map(|(ix, v)| {
//     //             let vprime = v.iter().filter_map(|x| *x).collect::<Vec<usize>>();
//     //             match vprime.len() {
//     //                 4 => Some((*ix, vprime)),
//     //                 _ => None,
//     //             }
//     //         })
//     //         .collect::<Vec<(usize, Vec<usize>)>>();
//     //     let piece_sets = piece_index_sets2
//     //         .iter()
//     //         .map(|(ix, v)| (*ix, game.get_pieces_refs(v.clone())))
//     //         .collect::<Vec<(usize, Vec<&GamePiece>)>>();
//     //     let quadri_results = piece_sets
//     //         .iter()
//     //         .map(|(ix, ps)| {
//     //             let is_quadri = pieces_are_quadri(ps.to_vec()).unwrap();
//     //             (*ix, is_quadri)
//     //         })
//     //         .collect::<Vec<(usize, bool)>>();

//     //     let mut current_quadris = Vec::<Vec<[usize; 2]>>::new();
//     //     let mut are_there_quadris = false;
//     //     for tup in quadri_results {
//     //         if tup.1 {
//     //             are_there_quadris = true;
//     //             current_quadris.push(game.board.quadri_coords[tup.0].clone())
//     //         }
//     //     }

//     //     (are_there_quadris, current_quadris)
//     // }

//     // fn get_piece_ref(&self, ix: usize) -> &GamePiece {
//     //     //TODO: ensure ix is in correct range
//     //     &self.pieces[ix]
//     // }
//     // fn get_pieces_refs(&self, ixs: Vec<usize>) -> Vec<&GamePiece> {
//     //     ixs.iter().map(|ix| self.get_piece_ref(*ix)).collect()
//     // }

//     pub fn list_available_pieces(&self) -> Vec<(usize, &GamePiece)> {
//         self.board_and_pieces
//             .get_piece_states()
//             .iter()
//             .filter(|&x| !x.on_board)
//             .map(|x| (x.piece_ix, x.piece))
//             .collect()
//     }

//     pub fn list_available_pieces_as_map(&self) -> HashMap<usize, &GamePiece> {
//         self.board_and_pieces
//             .get_piece_states()
//             .iter()
//             .filter(|&x| !x.on_board)
//             .map(|x| (x.piece_ix, x.piece))
//             .collect()
//     }

//     // pub fn list_board_squares(&self) -> Vec<BoardState> {
//     //     self.board_and_pieces
//     //         .get_board_states()
//     //         .iter()
//     //         .enumerate()
//     // }

//     pub fn list_board_squares_as_map(&self) -> HashMap<[usize; 2], Option<&GamePiece>> {
//         self.board_and_pieces
//             .get_board_states()
//             .iter()
//             .map(|bs| (bs.location_coord, bs.piece))
//             .collect()
//     }

//     //TODO - move to CLI representation - should layer on get_unused_pieces
//     pub fn list_available_pieces_for_print_2(&self) -> HashMap<usize, String> {
//         self.board_and_pieces
//             .get_piece_states()
//             .iter()
//             .filter(|&x| !x.on_board)
//             .map(|x| (x.piece_ix, x.piece.print.to_string()))
//             .collect()
//     }

//     pub fn choose_a_piece(&self, available_pieces: &Vec<String>) -> String {
//         let num_pieces = available_pieces.len();
//         let middle_ix = (num_pieces as f32 / 2.0).ceil() as usize;
//         let mut print_str = String::new();

//         for ix in 0..(middle_ix) {
//             let secondix = middle_ix + ix;
//             if secondix < num_pieces {
//                 let this_str =
//                     format!("{}\t{}\n", available_pieces[ix], available_pieces[secondix]);
//                 print_str.push_str(&this_str);
//             } else {
//                 let this_str = format!("{}\n", available_pieces[ix]);
//                 print_str.push_str(&this_str);
//             }
//         }
//         print_str
//     }
//     pub fn read_choosen_piece2(&self, available_pieces_map: &HashMap<usize, String>) -> usize {
//         loop {
//             print!("Piece index:\t");
//             io::stdout().flush().unwrap();

//             let mut choosen_piece = String::new();

//             io::stdin()
//                 .read_line(&mut choosen_piece)
//                 .expect("Failed to read line");

//             if let Ok(v) = choosen_piece.trim_end().parse::<usize>() {
//                 if available_pieces_map.contains_key(&v) {
//                     return v;
//                 } else {
//                     println!("Piece index out of bounds - try again")
//                 }
//             } else {
//                 println!("Unable to parse piece name")
//             }
//         }
//     }

//     //TODO - ix_as_alpha hsould be in the CLI represenrtaion, not here
//     pub fn place_piece_on_choosen_space(&mut self, piece_ix: usize) -> Result<usize, String> {
//         loop {
//             print!("Space Label:\t");
//             io::stdout().flush().unwrap();

//             let mut choosen_space = String::new();

//             io::stdin()
//                 .read_line(&mut choosen_space)
//                 .expect("Failed to read line");

//             choosen_space = choosen_space.trim_end().to_lowercase();

//             let choosen_space_ix_opt = &self
//                 .board_and_pieces
//                 .ix_as_alpha
//                 .iter()
//                 .position(|a| a.to_string() == choosen_space);

//             match choosen_space_ix_opt {
//                 Some(ix) => {
//                     if self.board_and_pieces.board[*ix].is_some() {
//                         println!("label provided is invlaid - try again")
//                     } else {
//                         let [x, y] = self.board_and_pieces.ix_to_coord(ix);
//                         println!("{}, {}", x, y);
//                         self.board_and_pieces.place_piece(piece_ix, x, y)?;
//                         return Ok(*ix);
//                     }
//                 }
//                 None => println!("label provided is invlaid - try again"),
//             }
//         }
//     }
//     pub fn game_board_string2(
//         &self,
//         pieces: Vec<Option<String>>,
//         labels: Vec<Option<String>>,
//     ) -> Result<String, String> {
//         if pieces.len() != X_DIM * Y_DIM || labels.len() != X_DIM * Y_DIM {
//             return Err("Pieces and labels must be the same size as the board".to_string());
//         }

//         let mut board = String::new();
//         for ix in 0..X_DIM {
//             board.push_str(BOARD_SEPARATOR);
//             board.push_str("\n");
//             board.push_str("| ");
//             for jx in 0..Y_DIM {
//                 let this_ind = (ix * Y_DIM) + jx;
//                 match (pieces[this_ind].as_ref(), labels[this_ind].as_ref()) {
//                     (Some(s), _) => board.push_str(&s),
//                     (None, Some(l)) => board.push_str(&l),
//                     (None, None) => board.push_str(" "),
//                 }
//                 board.push_str(" | ");
//             }
//             board = board.trim().to_string();
//             board.push_str("\n");
//         }
//         board.push_str(BOARD_SEPARATOR);
//         board.push_str("\n");
//         Ok(board)
//     }
//     pub fn pieces_by_position(&self) -> Vec<Option<String>> {
//         let string_vec = self
//             .board_and_pieces
//             .pieces
//             .iter()
//             .map(|p| p.print.to_string())
//             .collect::<Vec<String>>();

//         self.board_and_pieces
//             .board
//             .iter()
//             .map(move |ix_opt| match ix_opt {
//                 Some(ix) => Some(string_vec[*ix].clone()),
//                 None => None,
//             })
//             .collect::<Vec<Option<String>>>()
//     }
// }

// // Default imp
// pub trait GamePlay {
//     fn list_available_pieces(&self) -> ();
//     fn show_board(&self) -> ();
//     fn pick_piece_for_opponent(&self) -> usize;
//     fn pick_place_for_piece(&self) -> usize;

// }

// impl GamePlayCLI {
//     /*
//     pub fn play_game() -> () {
//         let mut game = Game::start_game();
//         loop {
//             let available_pieces_map = &game.list_available_pieces_for_print_2();
//             if available_pieces_map.len() > 0 {
//                 let mut available_pieces_v: Vec<(&usize, &String, String)> = available_pieces_map
//                     .into_iter()
//                     .map(|(ix, s)| (ix, s, format!("{}\t{}", ix, s)))
//                     .collect();
//                 available_pieces_v.sort_by_key(|k| k.0);
//                 let available_pieces = available_pieces_v
//                     .iter()
//                     .map(|(_ix, _s, ixs)| ixs.clone())
//                     .collect();
//                 println!("Pick a piece for your opponent to place");
//                 let print_str = &game.choose_a_piece(&available_pieces);
//                 println!("{}", print_str);
//                 let choosen_piece_ix = &game.read_choosen_piece2(&available_pieces_map);
//                 println!(
//                     "Opponent must place piece {}",
//                     &available_pieces_map
//                         .get(choosen_piece_ix)
//                         .ok_or("I screwed up!")?
//                 );

//                 let mut labels = Vec::new();
//                 let mut empty_labels = Vec::<Option<String>>::new();
//                 for s in &game.board.ix_as_alpha {
//                     labels.push(Some(s.to_string()));
//                     empty_labels.push(None);
//                 }
//                 let pieces = &game.pieces_by_position();

//                 println!(
//                     "Pick a place for piece {}",
//                     &available_pieces_map
//                         .get(choosen_piece_ix)
//                         .ok_or("I screwed up!")?
//                 );
//                 println!("{}", &game.game_board_string2(pieces.to_vec(), labels)?);

//                 let _ix = &mut game.place_piece_on_choosen_space(*choosen_piece_ix)?;
//                 let pieces = &game.pieces_by_position();
//                 println!("Current Board:");
//                 println!(
//                     "{}",
//                     &game.game_board_string2(pieces.to_vec(), empty_labels)?
//                 );
//                 //TODO - this should be inside the game struct
//                 let (are_quadris, _coords) = check_for_all_quadris(&game);
//                 if are_quadris {
//                     println!("Game is done! Winner!");
//                     break;
//                 }
//             } else {
//                 println! {"Draw!"};
//                 break;
//             }
//         }
//     }
//     */
//     // fn list_available_pieces(&self) -> () {
//     //     self.0.
//     // }
// }

// fn all_equal(arr: Vec<i8>) -> Result<bool, String> {
//     let arr_len = arr.len();
//     if arr_len == 0 {
//         return Err("Array must be non-empty".to_string());
//     }
//     let mut result = true;
//     let mut prior_value = arr[0];
//     for ix in 1..arr_len {
//         let next_value = arr[ix];
//         if prior_value != next_value {
//             result = false;
//             break;
//         }
//         prior_value = next_value;
//     }
//     Ok(result)
// }

// //assume all pieces have same dimensions

// // //TODO - should be an implementation method
// // pub fn check_for_all_quadris(game: &Game) -> (bool, Vec<Vec<[usize; 2]>>) {
// //     let piece_index_sets = game
// //         .board
// //         .quadri_coords
// //         .iter()
// //         .enumerate()
// //         .map(|(ix, v)| {
// //             let this_piece_v_res = game.board.get_piece_ixs_by_coords(v);
// //             let this_piece_v_unwrap = this_piece_v_res.unwrap();
// //             (ix, this_piece_v_unwrap)
// //         })
// //         .collect::<Vec<(usize, Vec<Option<usize>>)>>();

// //     let piece_index_sets2 = piece_index_sets
// //         .iter()
// //         .filter_map(|(ix, v)| {
// //             let vprime = v.iter().filter_map(|x| *x).collect::<Vec<usize>>();
// //             match vprime.len() {
// //                 4 => Some((*ix, vprime)),
// //                 _ => None,
// //             }
// //         })
// //         .collect::<Vec<(usize, Vec<usize>)>>();
// //     let piece_sets = piece_index_sets2
// //         .iter()
// //         .map(|(ix, v)| (*ix, game.get_pieces_refs(v.clone())))
// //         .collect::<Vec<(usize, Vec<&GamePiece>)>>();
// //     let quadri_results = piece_sets
// //         .iter()
// //         .map(|(ix, ps)| {
// //             let is_quadri = pieces_are_quadri(ps.to_vec()).unwrap();
// //             (*ix, is_quadri)
// //         })
// //         .collect::<Vec<(usize, bool)>>();

// //     let mut current_quadris = Vec::<Vec<[usize; 2]>>::new();
// //     let mut are_there_quadris = false;
// //     for tup in quadri_results {
// //         if tup.1 {
// //             are_there_quadris = true;
// //             current_quadris.push(game.board.quadri_coords[tup.0].clone())
// //         }
// //     }

// //     (are_there_quadris, current_quadris)
// // }
