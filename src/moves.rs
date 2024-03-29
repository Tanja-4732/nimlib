//! Code for handling moves.  
//! This module contains code for handling moves in Nim games,
//! such as calculating the resulting position after a move is applied,
//! determining if a move is valid, and generating all possible moves
//! for a given position.

use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{
    nimbers::calculate_splits, NimAction, NimGame, NimRule, NimSplit, PlaceAction, Player, Split,
    Stack, TakeAction, TakeSize,
};

/// Errors which may occur when applying a move
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MoveError {
    // TODO remove `InvalidMove` and replace it with more specific errors
    /// The move is invalid for the given position
    ///
    /// This error is very generic and is subject to be replaced with more specific errors
    InvalidMove,

    /// The stack index is out of bounds
    NoSuchStack,

    /// The stack does not have enough coins to take (before a possible split)
    NotEnoughCoinsOnStack,

    /// No rule exists which supports the desired move
    NoSuchRule,

    /// The split is invalid for the given move under ever rule in the specified game
    InvalidSplit,

    /// Player does not have enough coins to place (for [`NimAction::Place`] moves)
    NotEnoughCoinsOnPlayer,
}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::InvalidMove => write!(f, "The move is invalid for the given position"),
            MoveError::NoSuchStack => write!(f, "The stack index is out of bounds"),
            MoveError::NotEnoughCoinsOnStack => {
                write!(f, "The stack does not have enough coins to take")
            }
            MoveError::NoSuchRule => write!(f, "No rule exists which supports the desired move"),
            MoveError::InvalidSplit => write!(f, "The split is invalid for the given move"),
            MoveError::NotEnoughCoinsOnPlayer => {
                write!(f, "Player does not have enough coins to place")
            }
        }
    }
}

impl Error for MoveError {}

/// Determine if a move is valid for a given position
///
/// # Errors
///
/// Returns `Ok(())` if the move is valid, or an error if the move is invalid
/// (see [`MoveError`] for possible errors).
pub fn check_move(game: &NimGame, mov: &NimAction) -> Result<(), MoveError> {
    match mov {
        NimAction::Take(TakeAction {
            stack_index,
            amount,
            split,
            from: _, // Don't need to check this field for `Take` moves
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
                    TakeSize::List(list) => list.contains(amount),
                    TakeSize::Any => true,
                    TakeSize::Place => false,
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
                return Err(MoveError::NotEnoughCoinsOnStack);
            }

            // Check if the move is valid for at least one rule (splitting)
            let mut valid = false;
            for rule in supporting_rules {
                match rule.split {
                    Split::Never => {
                        // TODO consider replacing this with a regular if statement
                        if let NimSplit::No = split {
                            valid = true;
                            break;
                        }
                    }
                    Split::Optional => {
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
                    Split::Always => {
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
        NimAction::Place(PlaceAction {
            stack_index,
            amount,
            from,
        }) => {
            if game.rules.iter().all(|rule| rule.take != TakeSize::Place) {
                return Err(MoveError::NoSuchRule);
            }

            // Get the stack to place coins onto
            let stack = game
                .stacks
                .get(*stack_index)
                .ok_or(MoveError::NoSuchStack)?;

            // Check if the player has sufficient coins to place
            let player_coins = match from {
                crate::Player::A => game.coins_a,
                crate::Player::B => game.coins_b,
            };

            if player_coins < *amount {
                return Err(MoveError::NotEnoughCoinsOnStack);
            }

            return Ok(());
        }
    }

    Ok(())
}

/// The implementation of [`apply_move`] and [`apply_move_unchecked`]
fn apply_move_(game: &mut NimGame, mov: &NimAction, unchecked: bool) -> Result<(), MoveError> {
    // Assure that the move is valid
    if !unchecked {
        check_move(game, mov)?;
    }

    match mov {
        NimAction::Take(TakeAction {
            stack_index,
            amount,
            split,
            from,
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

            if let Some(player) = from {
                // Remove coins from the player's pool
                match player {
                    crate::Player::A => {
                        game.coins_a = game.coins_a.checked_add(*amount).expect("Coin overflow")
                    }
                    crate::Player::B => {
                        game.coins_b = game.coins_b.checked_add(*amount).expect("Coin overflow")
                    }
                };
            }
        }
        NimAction::Place(PlaceAction {
            stack_index,
            amount,
            from,
        }) => {
            // Get the stack to place coins onto
            let stack = game
                .stacks
                .get_mut(*stack_index)
                .ok_or(MoveError::NoSuchStack)?;

            // Place coins onto the stack
            stack.0 += amount;

            // Remove coins from the player's pool
            match from {
                crate::Player::A => {
                    game.coins_a = game.coins_a.checked_sub(*amount).expect(
                        "Coin underflow; this should not happen, as the move was checked before",
                    )
                }
                crate::Player::B => {
                    game.coins_b = game.coins_b.checked_sub(*amount).expect(
                        "Coin underflow; this should not happen, as the move was checked before",
                    )
                }
            };
        }
    }

    Ok(())
}

/// Applies a move to a position, if the move is valid
///
/// The validity of the move is checked with [`check_move`] before applying it.
///
/// # Arguments
///
/// - `game` - The game state before the move is applied
/// - `mov` - The move to apply
///
/// # Errors
///
/// This function returns
/// [`Ok`] with the unit type if the move is valid and was applied successfully,
/// an [`Err`] with the reason why the move is invalid otherwise (see [`MoveError`])
pub fn apply_move(game: &mut NimGame, mov: &NimAction) -> Result<(), MoveError> {
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
///
/// # Errors
///
/// If the move is invalid. See [returns](#returns) above.
pub unsafe fn apply_move_unchecked(game: &mut NimGame, mov: &NimAction) -> Result<(), MoveError> {
    apply_move_(game, mov, true)
}

/// Generate all possible (legal) moves for a given position
///
/// # Arguments
///
/// - `stacks` - The stacks of coins in the position
/// - `rules` - The rules of the game (see [`NimRule`])
/// - `pool_coins` is currently not fully implemented.
///
/// # Returns
///
/// A [`Vec`] of all possible (legal) moves for the given position
/// in the form of [`NimAction`]s.
///
/// The returned value does not reference the given `stacks` or `rules`,
/// only the stack indices and the amount of coins to take are referenced,
/// along with the split of coins if necessary.
///
/// # Example
///
/// ```
/// use nimlib::{moves, NimAction, NimRule, NimSplit, Split, Stack, TakeSize};
///
/// let rules = vec![NimRule {
///     take: TakeSize::List(vec![1, 2, 3]),
///     split: Split::Never,
/// }];
///
/// let stacks = vec![Stack(10)];
///
/// let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0))
///     .into_iter()
///     .map(|mov| {
///         if let NimAction::Take(take) = mov {
///             take
///         } else {
///             panic!("Expected a take action");
///         }
///     })
///     .collect::<Vec<_>>();
///
/// assert_eq!(moves.len(), 3);
///
/// assert_eq!(moves[0].amount, 1);
/// assert_eq!(moves[0].stack_index, 0);
/// assert_eq!(moves[0].split, NimSplit::No);
///
/// assert_eq!(moves[1].amount, 2);
/// assert_eq!(moves[1].stack_index, 0);
/// assert_eq!(moves[1].split, NimSplit::No);
///
/// assert_eq!(moves[2].amount, 3);
/// assert_eq!(moves[2].stack_index, 0);
/// assert_eq!(moves[2].split, NimSplit::No);
/// ```
///
/// # Panics    
///
/// If a rule with [`NimAction::Place`] is encountered which also has
/// [`Split::Optional`] or [`Split::Always`], this function will panic.
///
#[must_use]
pub fn calculate_legal_moves(
    stacks: &[Stack],
    rules: &[NimRule],
    (pool_coins_a, pool_coins_b): (u64, u64),
) -> Vec<NimAction> {
    let mut moves = Vec::new();

    // Iterate over all stacks
    for (s_idx, stack) in stacks.iter().enumerate() {
        // Iterate over all rules
        for NimRule { take, split } in rules {
            match take {
                TakeSize::List(take_sizes) => {
                    for take_size in take_sizes {
                        if stack.0 >= *take_size {
                            match split {
                                Split::Never => {
                                    // Without split
                                    moves.push(NimAction::Take(TakeAction {
                                        stack_index: s_idx,
                                        amount: *take_size,
                                        split: NimSplit::No,
                                        from: None,
                                    }));
                                }
                                Split::Optional => {
                                    // Without split
                                    moves.push(NimAction::Take(TakeAction {
                                        stack_index: s_idx,
                                        amount: *take_size,
                                        split: NimSplit::No,
                                        from: None,
                                    }));

                                    // With split
                                    // Enumerate all possible splits
                                    for (a, b) in
                                        calculate_splits(stack.0.saturating_sub(*take_size))
                                    {
                                        moves.push(NimAction::Take(TakeAction {
                                            stack_index: s_idx,
                                            amount: *take_size,
                                            split: NimSplit::Yes(a, b),
                                            from: None,
                                        }));
                                    }
                                }
                                Split::Always => {
                                    // With split
                                    // Enumerate all possible splits
                                    for (a, b) in
                                        calculate_splits(stack.0.saturating_sub(*take_size))
                                    {
                                        moves.push(NimAction::Take(TakeAction {
                                            stack_index: s_idx,
                                            amount: *take_size,
                                            split: NimSplit::Yes(a, b),
                                            from: None,
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }

                TakeSize::Any => {
                    for h in 1..=stack.0 {
                        match split {
                            Split::Never => {
                                // Without split
                                moves.push(NimAction::Take(TakeAction {
                                    stack_index: s_idx,
                                    amount: h,
                                    split: NimSplit::No,
                                    from: None,
                                }));
                            }
                            Split::Optional => {
                                // Without split
                                moves.push(NimAction::Take(TakeAction {
                                    stack_index: s_idx,
                                    amount: h,
                                    split: NimSplit::No,
                                    from: None,
                                }));

                                // With split
                                // Enumerate all possible splits
                                for (a, b) in calculate_splits(stack.0.saturating_sub(h)) {
                                    moves.push(NimAction::Take(TakeAction {
                                        stack_index: s_idx,
                                        amount: h,
                                        split: NimSplit::Yes(a, b),
                                        from: None,
                                    }));
                                }
                            }
                            Split::Always => {
                                // With split
                                // Enumerate all possible splits
                                for (a, b) in calculate_splits(stack.0.saturating_sub(h)) {
                                    moves.push(NimAction::Take(TakeAction {
                                        stack_index: s_idx,
                                        amount: h,
                                        split: NimSplit::Yes(a, b),
                                        from: None,
                                    }));
                                }
                            }
                        }
                    }
                }

                TakeSize::Place => {
                    // The player can add 1..pool_coins coins to the stack
                    // The placed coins are taken from the pool
                    // FIXME only the coins of player A can be placed; this must not be hardcoded
                    for c in 1..=pool_coins_a {
                        match split {
                            Split::Never => {
                                // Without split
                                moves.push(NimAction::Place(PlaceAction {
                                    stack_index: s_idx,
                                    amount: c,
                                    from: Player::A,
                                }));
                            }
                            Split::Optional | Split::Always => {
                                // TODO consider replacing this panic with a Result or improve the types themselves
                                panic!("Split is not allowed with Place")
                            }
                        }
                    }
                    for c in 1..=pool_coins_b {
                        match split {
                            Split::Never => {
                                // Without split
                                moves.push(NimAction::Place(PlaceAction {
                                    stack_index: s_idx,
                                    amount: c,
                                    from: Player::B,
                                }));
                            }
                            Split::Optional | Split::Always => {
                                // TODO consider replacing this panic with a Result or improve the types themselves
                                panic!("Split is not allowed with Place")
                            }
                        }
                    }
                }
            }
        }
    }

    moves
}
