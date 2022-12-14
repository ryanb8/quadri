use crate::gameboard::BoardState;
use crate::gameboard::PieceState;
pub trait QuadriIORepresentation {
    fn pick_piece_for_opponent(
        &self,
        board_states: Vec<BoardState>,
        piece_states: Vec<PieceState>,
    ) -> usize;
    fn pick_place_for_piece(&self, board_states: Vec<BoardState>, piece_ix: usize) -> usize;
}
