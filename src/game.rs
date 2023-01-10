static PHASE_PICK: &'static str = "pick_piece_for_opponent";
static PHASE_PLACE: &'static str = "place_piece_on_board";

use crate::gameboard::BoardState;
use crate::gameboard::PieceState;
use crate::quadri_io_representation_cli::QuadriIORepresentationCLI;
use crate::{gameboard::GameboardAndPieces, quadri_io_representation::QuadriIORepresentation};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct TurnState {
    pub turn: usize,
    pub phase: String,
    pub current_actor: usize,
}
#[derive(Debug, Clone)]
pub struct WinnerState {
    complete: bool,
    winner: Option<usize>,
    winning_quadris_coords: Option<Vec<Vec<[usize; 2]>>>,
}
#[derive(Debug, Clone)]
pub struct Game<T: QuadriIORepresentation> {
    // pieces: Vec<GamePiece>,
    board_and_pieces: GameboardAndPieces,
    turn_state: TurnState,
    representation_io: T,
}

impl WinnerState {
    pub fn is_complete(&self) -> bool {
        self.complete
    }
    pub fn get_winning_player(&self) -> Option<usize> {
        self.winner.clone()
    }
    pub fn get_winning_quadris_as_coord(&self) -> Option<Vec<Vec<[usize; 2]>>> {
        self.winning_quadris_coords.clone()
    }
}

impl TurnState {
    pub fn setup() -> TurnState {
        TurnState {
            turn: 1,
            phase: PHASE_PICK.to_string(),
            current_actor: 1,
        }
    }
    pub fn increment_turn(&mut self) -> () {
        if self.phase == PHASE_PICK {
            self.increment_actor();
        }
        self.increment_turn_and_phase();
    }
    fn increment_actor(&mut self) -> () {
        if self.current_actor == 1 {
            self.current_actor = 2;
        } else {
            self.current_actor = 1;
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

impl Game<QuadriIORepresentationCLI> {
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
}

impl<T: QuadriIORepresentation> Game<T> {
    pub fn play_game(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let piece_ix = self.representation_io.pick_piece_for_opponent(
                &self.turn_state,
                self.get_board_states(),
                self.get_piece_states(),
            );
            self.turn_state.increment_turn();
            let board_ix = self.representation_io.pick_place_for_piece(
                &self.turn_state,
                self.get_board_states(),
                piece_ix,
            );
            self.place_piece(piece_ix, board_ix);
            let ws = self.get_winner_state();
            if ws.is_complete() {
                println!("Game is done! Winner!");
                self.representation_io
                    .alert_winner(ws, self.get_board_states());
                break;
            }
            self.turn_state.increment_turn();
        }
        Ok(())
    }

    fn get_piece_states(&self) -> Vec<PieceState> {
        self.board_and_pieces.get_piece_states()
    }

    fn get_board_states(&self) -> Vec<BoardState> {
        self.board_and_pieces.get_board_states()
    }

    fn place_piece(&mut self, piece_ix: usize, board_ix: usize) -> usize {
        //TODO - this has been tweaked to not return a result, but realistically we need to work on cleaning
        // the API between this method and the associated `board_and_pieces.place_piece` method.
        // I don't really want this to retry; that responsbility should be the onus of the representation/IO set
        // I don't know how that will generalize
        loop {
            let response = self.board_and_pieces.place_piece(piece_ix, board_ix);
            match response {
                Ok(u) => return u,
                Err(s) => println!("debug pick_piece: {}", s),
            }
        }
    }

    fn check_for_quadris(&self) -> (bool, Vec<Vec<[usize; 2]>>) {
        self.board_and_pieces.check_all_quadris()
    }
    fn get_winner_state(&self) -> WinnerState {
        let (are_quadris, quadri_coords) = self.check_for_quadris();
        match are_quadris {
            true => WinnerState {
                complete: true,
                winner: Some(self.turn_state.current_actor),
                winning_quadris_coords: Some(quadri_coords),
            },
            false => WinnerState {
                complete: false,
                winner: None,
                winning_quadris_coords: None,
            },
        }
    }
}
