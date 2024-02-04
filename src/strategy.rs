use rand::{thread_rng, seq::SliceRandom};

use crate::{game::ChessGame, action::Action};

// general implementation is
// try to make an action
// no action -> lost the game
// so Fn(&ChessGame) -> Option<Action>

// just pick a random move from the possible moves
pub fn random_player(game: &ChessGame) -> Option<Action> {
    let moves = game.possible_moves(game.turn);
    if moves.len() > 0 {
        Some(*moves.choose(&mut thread_rng()).unwrap())
    } else {
        None
    }
}

