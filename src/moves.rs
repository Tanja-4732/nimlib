//! Code for handling moves.  
//! This module contains code for handling moves in Nim games,
//! such as calculating the resulting position after a move is applied,
//! determining if a move is valid, and generating all possible moves
//! for a given position.

use crate::{NimAction, NimGame, NimMove, NimRule, NimSplit, PlaceAction, TakeAction};

/// Errors which may occur when applying a move
pub enum MoveError {
    // TODO remove `InvalidMove` and replace it with more specific errors
    /// The move is invalid for the given position
    ///
    /// This error is very generic and is subject to be replaced with more specific errors
    InvalidMove,

    /// The stack index is out of bounds
    NoSuchStack,

    /// The stack does not have enough coins to take (before a possible split)
    NotEnoughCoins,

    /// No rule exists which supports the desired move
    NoSuchRule,

    /// The split is invalid for the given move under ever rule in the specified game
    InvalidSplit,
}

/// Determine if a move is valid for a given position
pub fn check_move(game: &NimGame, mov: &NimMove) -> Result<(), MoveError> {
    match &mov.action {
        crate::NimAction::Take(TakeAction {
            stack_index,
            amount,
            split,
        }) => {
            // Get the stack to take coins from
            let stack = game
                .stacks
                .get(*stack_index)
                .ok_or(MoveError::NoSuchStack)?;

            // Check if a rule can support the desired move (taking)
            let mut supporting_rules = Vec::new();

            for rule in &game.rules {
                let supports = match &rule.take {
                    crate::TakeSize::List(list) => list.contains(amount),
                    crate::TakeSize::Any => true,
                    crate::TakeSize::Place => false,
                };

                if supports {
                    supporting_rules.push(rule);
                    break;
                }
            }

            if supporting_rules.is_empty() {
                return Err(MoveError::NoSuchRule);
            }

            // Check if the stack has enough coins to take
            if stack.0 < *amount {
                return Err(MoveError::NotEnoughCoins);
            }

            // Check if the move is valid for at least one rule (splitting)
            let mut valid = false;
            for rule in supporting_rules {
                match rule.split {
                    crate::Split::Never => {
                        // TODO consider replacing this with a regular if statement
                        if let NimSplit::No = split {
                            valid = true;
                            break;
                        }
                    }
                    crate::Split::Optional => {
                        // TODO consider replacing this with a regular if statement
                        if let NimSplit::Yes(a, b) = split {
                            // FIXME replace `true` with a check if the rule allowing for the split allows taking the `amount` of coins
                            if ((a.0 + b.0 + amount) <= stack.0) && a.0 != 0 && b.0 != 0 && (true) {
                                valid = true;
                                break;
                            }
                        } else {
                            // Splitting is optional and no split was specified, so the move is valid
                            valid = true;
                            break;
                        }
                    }
                    crate::Split::Always => {
                        // TODO consider replacing this with a regular if statement
                        if let NimSplit::Yes(a, b) = split {
                            // FIXME replace `true` with a check if the rule allowing for the split allows taking the `amount` of coins
                            if ((a.0 + b.0 + amount) <= stack.0) && a.0 != 0 && b.0 != 0 && (true) {
                                valid = true;
                                break;
                            }
                        }
                        // No `else` clause as the move is invalid if no split was specified
                    }
                }
            }

            if !valid {
                return Err(MoveError::InvalidSplit);
            }
        }
        crate::NimAction::Place(PlaceAction {
            stack_index,
            amount: _,
        }) => {
            // Get the stack to place coins onto
            let _stack = game
                .stacks
                .get(*stack_index)
                .ok_or(MoveError::NoSuchStack)?;

            return Err(MoveError::InvalidMove);
        }
    }

    Ok(())
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
            stack_index: stack,
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

/// Generate all possible (legal) moves for a given position,
/// according to the rules of the `game`
pub fn enumerate_moves(game: &NimGame) -> Vec<NimMove> {
    let mut moves = Vec::new();

    todo!();

    moves
}
