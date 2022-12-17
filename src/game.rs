static PHASE_PICK: &'static str = "pick_piece_for_opponent";
static PHASE_PLACE: &'static str = "place_piece_on_board";

use crate::quadri_io_representation_cli::QuadriIORepresentationCLI;
use crate::{gameboard::GameboardAndPieces, quadri_io_representation::QuadriIORepresentation};

#[derive(Debug, Clone)]
pub struct TurnState {
    turn: usize,
    phase: String,
    current_actor: usize,
}
#[derive(Debug, Clone)]
pub struct WinnerState {
    winner: Option<usize>,
    winning_quadris: Option<Vec<Vec<[usize; 2]>>>,
}
#[derive(Debug, Clone)]
pub struct Game<T: QuadriIORepresentation> {
    // pieces: Vec<GamePiece>,
    board_and_pieces: GameboardAndPieces,
    turn_state: TurnState,
    representation_io: T,
}

// // public api:
// // start game
// // get game state
// // - list available pieces
// // - list board and pieces
// // - list turn /move
// // - any winners
// // pick piece for opponent
// // place piece on board - option to return winners or not

impl TurnState {
    pub fn setup() -> TurnState {
        TurnState {
            turn: 1,
            phase: PHASE_PICK.to_string(),
            current_actor: 1,
        }
    }
    pub fn increment_turn(&mut self) -> () {
        self.increment_actor();
        self.increment_turn_and_phase();
    }
    fn increment_actor(&mut self) -> () {
        self.current_actor = match self.current_actor {
            1 => 2,
            2 => 1,
            _ => panic!("Quadri is a two player game"),
        }
    }
    fn increment_turn_and_phase(&mut self) -> () {
        if self.phase == PHASE_PICK.to_string() {
            self.phase = PHASE_PLACE.to_string();
        } else {
            self.turn = self.turn + 1;
            self.phase = PHASE_PICK.to_string();
        }
    }
}

impl<T: QuadriIORepresentation> Game<T> {
    pub fn new_cli_game() -> Game<QuadriIORepresentationCLI> {
        let gb = GameboardAndPieces::new();
        let piece_states = gb.get_piece_states();
        let repr = QuadriIORepresentationCLI::new(piece_states);

        Game {
            board_and_pieces: gb,
            turn_state: TurnState::setup(),
            representation_io: repr,
        }
    }

    //     // Setup
    //     pub fn start_game() -> Game {
    //         Game {
    //             // pieces: Game::create_pieces(),
    //             board_and_pieces: GameboardAndPieces::new(),
    //             turn_state: TurnState::setup(),
    //         }
    //     }

    //     pub fn get_game_state(&self) -> GameState {
    //         GameState {
    //             piece_states: self.board_and_pieces.get_piece_states(),
    //             turn_state: self.get_turn_and_actor(),
    //             winner_state: self.get_winners(),
    //         }
    //     }

    //     pub fn get_turn_and_actor(&self) -> TurnState {
    //         self.turn_state.clone()
    //     }

    //TODO actually implement this.
    pub fn get_winners(&self) -> WinnerState {
        WinnerState {
            winner: None,
            winning_quadris: None,
        }
    }
}

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
