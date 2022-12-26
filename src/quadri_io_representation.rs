use crate::game::TurnState;
use crate::game::WinnerState;
use crate::gameboard::BoardState;
use crate::gameboard::PieceState;

pub trait QuadriIORepresentation {
    fn pick_piece_for_opponent(
        &self,
        turn_state: &TurnState,
        board_states: Vec<BoardState>,
        piece_states: Vec<PieceState>,
    ) -> usize;
    fn pick_place_for_piece(
        &self,
        turn_state: &TurnState,
        board_states: Vec<BoardState>,
        piece_ix: usize,
    ) -> usize;
    fn alert_winner(&self, winner_state: WinnerState, board_states: Vec<BoardState>) -> ();
}
