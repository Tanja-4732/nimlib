//! Code for handling moves.  
//! This module contains code for handling moves in Nim games,
//! such as calculating the resulting position after a move is applied,
//! determining if a move is valid, and generating all possible moves
//! for a given position.

use crate::{NimGame, NimMove, PlaceAction, TakeAction};

/// Errors which may occur when applying a move
pub enum MoveError {
    /// The move is invalid for the given position
    InvalidMove,
}

/// Determine if a move is valid for a given position
pub fn check_move(game: &NimGame, mov: &NimMove) -> Result<(), MoveError> {
    match &mov.action {
        crate::NimAction::Take(TakeAction {
            stack,
            amount,
            split,
        }) => {
            todo!()
        }
        crate::NimAction::Place(PlaceAction {
            stack_index,
            amount,
        }) => {
            todo!()
        }
    }
}

/// Calculate the resulting position after a move is applied
///
/// # Arguments
///
/// - `game` - The game state before the move is applied
/// - `mov` - The move to apply
///
/// # Returns
///
/// The resulting game state after the move is applied (if valid),
/// or an error if the move is invalid (see [MoveError])
pub fn apply_move(game: &NimGame, mov: &NimMove) -> Result<NimGame, MoveError> {
    let mut new_game = game.clone();

    match &mov.action {
        crate::NimAction::Take(TakeAction {
            stack,
            amount,
            split,
        }) => {
            todo!()
        }
        crate::NimAction::Place(PlaceAction {
            stack_index,
            amount,
        }) => {
            todo!()
        }
    }

    Ok(new_game)
}

///
pub fn enumerate_moves(game: &NimGame) -> Vec<NimMove> {
    let mut moves = Vec::new();

    todo!();

    moves
}
