//! Code for handling moves.  
//! This module contains code for handling moves in Nim games,
//! such as calculating the resulting position after a move is applied,
//! determining if a move is valid, and generating all possible moves
//! for a given position.

use crate::{
    nimbers::calculate_splits, NimAction, NimGame, NimMove, NimRule, NimSplit, PlaceAction,
    TakeAction,
};

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

/// The implementation of [`apply_move`] and [`apply_move_unchecked`]
fn apply_move_(game: &mut NimGame, mov: &NimMove, unchecked: bool) -> Result<(), MoveError> {
    // Assure that the move is valid
    if !unchecked {
        check_move(&game, &mov)?;
    }

    match &mov.action {
        crate::NimAction::Take(TakeAction {
            stack_index,
            amount,
            split,
        }) => {
            // Get the stack to take coins from
            let stack = game
                .stacks
                .get_mut(*stack_index)
                .ok_or(MoveError::NoSuchStack)?;

            // Take coins from the stack
            stack.0 -= amount;

            // Split the coins if necessary
            if let NimSplit::Yes(a, b) = split {
                // Insert stacks `a` and `b` into `stacks` at position `stack_index`
                // And remove the original stack at `stack_index`
                game.stacks
                    .splice(*stack_index..=*stack_index, [*a, *b].into_iter());
            }
        }
        crate::NimAction::Place(PlaceAction {
            stack_index,
            amount,
        }) => {
            // Get the stack to place coins onto
            let stack = game
                .stacks
                .get_mut(*stack_index)
                .ok_or(MoveError::NoSuchStack)?;

            // Place coins onto the stack
            stack.0 += amount;
        }
    }

    Ok(())
}

/// Applies a move to a position, if the move is valid
///
/// # Arguments
///
/// - `game` - The game state before the move is applied
/// - `mov` - The move to apply
///
/// # Returns
///
/// [`Ok`] with the unit type if the move is valid and was applied successfully,
/// an [`Err`] with the reason why the move is invalid otherwise (see [`MoveError`])
pub fn apply_move(game: &mut NimGame, mov: &NimMove) -> Result<(), MoveError> {
    apply_move_(game, mov, false)
}

/// Applies a move to a position, even if the move is invalid
///
/// # Arguments
///
/// - `game` - The game state before the move is applied
/// - `mov` - The move to apply
///
/// # Returns
///
/// [`Ok`] with the unit type if the move is valid and was applied successfully,
/// an [`Err`] otherwise, usually [`MoveError::NoSuchStack`] (see [`MoveError`])
///
/// # Safety
///
/// While this function does not perform _traditionally_ unsafe operations,
/// applying moves without checking them for validity can lead to unexpected
/// behaviour. This function is therefore marked as unsafe;
/// but this is possibly subject to change.
///
/// Please note that the bounds checks of the [`Vec`] indices are not disabled by this function.
pub unsafe fn apply_move_unchecked(game: &mut NimGame, mov: &NimMove) -> Result<(), MoveError> {
    apply_move_(game, mov, true)
}

/// Generate all possible (legal) moves for a given position,
/// according to the rules of the `game`
pub fn enumerate_moves(game: &NimGame) -> Vec<NimAction> {
    let mut moves = Vec::new();

    // Iterate over all stacks
    for (s_idx, stack) in game.stacks.iter().enumerate() {
        // Iterate over all rules
        for rule in &game.rules {
            match &rule.take {
                crate::TakeSize::List(list) => todo!(),
                crate::TakeSize::Any => {
                    match rule.split {
                        crate::Split::Never => {
                            // Without split
                            for coins in 1..=stack.0 {
                                moves.push(NimAction::Take(TakeAction {
                                    stack_index: s_idx,
                                    amount: coins,
                                    split: NimSplit::No,
                                }));
                            }
                        }
                        crate::Split::Optional => {
                            // Without split
                            for coins in 1..=stack.0 {
                                moves.push(NimAction::Take(TakeAction {
                                    stack_index: s_idx,
                                    amount: coins,
                                    split: NimSplit::No,
                                }));
                            }

                            // With split
                            for coins in 1..=(stack.0.saturating_sub(2)) {
                                // Enumerate all possible splits
                                for split in calculate_splits(stack.0 - coins) {
                                    moves.push(NimAction::Take(TakeAction {
                                        stack_index: s_idx,
                                        amount: coins,
                                        split: NimSplit::Yes(split.0, split.1),
                                    }));
                                }
                            }
                        }
                        crate::Split::Always => {
                            // With split
                            for coins in 1..=(stack.0.saturating_sub(2)) {
                                // Enumerate all possible splits
                                for split in calculate_splits(stack.0 - coins) {
                                    moves.push(NimAction::Take(TakeAction {
                                        stack_index: s_idx,
                                        amount: coins,
                                        split: NimSplit::Yes(split.0, split.1),
                                    }));
                                }
                            }
                        }
                    }
                }
                crate::TakeSize::Place => todo!(),
            }
        }
    }

    moves
}
