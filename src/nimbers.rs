//! The inner workings of the nimber calculation algorithm.
//!
//! Useful if you want to calculate nimbers for heights that are not part of a game,
//! or if you want to include just the lower-level nimber calculation algorithms in your project.
//!
//! Includes helper functions like [`calculate_splits`].

use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;

use crate::{game::NimRule, moves, NimAction, NimSplit, Nimber, PlaceAction, Stack, TakeAction};

/// The nimber cache is a map from (`height`, `pool_coins`) to nimber.
///
/// It is only valid for a specific set of rules.
///
/// The pool coins are ignored for now, should always be 0.  
///
/// # Arguments
///
/// (the elements of the map's key-tuple)
///
/// - `height`: The height of the stack
/// - `pool_coins`: The number of coins in the pool
///
/// # Result
///
/// (the value of the map)
///
/// - `nimber`: The nimber of the stack given its height and pool coins
type NimberCache = HashMap<(u64, u64), Nimber>;

lazy_static! {
    static ref NIMBER_CACHE: RwLock<HashMap<Vec<NimRule>, NimberCache>> = Default::default();
}

/// Calculate all possibilities to split a number into two parts,
/// where the sum of the parts is the original number,
/// accounting for symmetry.
///
///
/// # Examples
///
/// ```
/// use nimlib::{nimbers::calculate_splits, Stack};
///
/// assert_eq!(calculate_splits(0), vec![]);
/// assert_eq!(calculate_splits(1), vec![]);
/// assert_eq!(calculate_splits(2), vec![(Stack(1), Stack(1))]);
/// assert_eq!(calculate_splits(3), vec![(Stack(1), Stack(2))]);
/// assert_eq!(calculate_splits(4), vec![(Stack(1), Stack(3)), (Stack(2), Stack(2))]);
/// assert_eq!(calculate_splits(5), vec![(Stack(1), Stack(4)), (Stack(2), Stack(3))]);
/// assert_eq!(calculate_splits(6), vec![(Stack(1), Stack(5)), (Stack(2), Stack(4)), (Stack(3), Stack(3))]);
/// ```
#[must_use]
pub fn calculate_splits(height: u64) -> Vec<(Stack, Stack)> {
    let mut splits = Vec::new();

    // Stacks of height 0 and 1 can't be split
    if height <= 1 {
        return splits;
    }

    for i in 1..=height / 2 {
        splits.push((Stack(i), Stack(height - i)));
    }

    splits
}

// # Examples
// ```
// use nimlib::nimbers::calculate_nimber_for_height;
// use nimlib::game::{NimRule, Split, TakeSize};
//
// let rules = vec![
//    NimRule {
//       take: TakeSize::List(vec![1, 2, 3]),
//      split: Split::Never,
//    },
//    NimRule {
//     take: TakeSize::Any,
//    split: Split::Optional,
//    },
//    ];
// ```

/// Calls a function with the cache for the given rules.
///
/// If the cache doesn't exist yet, it is created.  
/// The cache is locked for the duration of the function call.
fn with_cache<T, F: FnOnce(&mut NimberCache) -> T>(rules: &[NimRule], f: F) -> T {
    let mut caches = NIMBER_CACHE.write().unwrap();
    let cache = if let Some(cache) = caches.get_mut(rules) {
        cache
    } else {
        caches.insert(rules.to_vec(), Default::default());
        caches.get_mut(rules).unwrap()
    };

    f(cache)
}

/// Calculate the nimber of a stack of height `height` given a set of rules
///
/// `pool_coins` is the number of coins in the pool of the current player (must be 0 for now)
///
/// The algorithm makes use of the MEX (minimum excluded) rule to calculate the nimber.  
/// Essentially, all rules are applied to copies of the stack, and the nimbers of the resulting stacks
/// are stored in an _exclusion list_. The nimber of the original stack is the smallest non-negative
/// integer that is not in the exclusion list.
///
/// # Panics
///
/// Panics if `rules` match a [`NimAction::Place`] action.
///
#[must_use]
pub fn calculate_nimber_for_height(height: u64, rules: &[NimRule], pool_coins: u64) -> Nimber {
    // Check if we've already calculated this nimber
    // if let Some(nimber) = get_cache_for_rules!(rules).get(&(height, pool_coins)) {
    if let Some(nimber) = with_cache(rules, |cache| cache.get(&(height, pool_coins)).copied()) {
        return nimber;
    }

    // TODO handle pool coins correctly
    assert_eq!(pool_coins, 0, "Pool coins not yet supported");

    // Use the MEX (minimum excluded) rule to calculate the nimber
    let mut exclusion_list: Vec<Nimber> = Vec::new();

    // Enumerate all possible moves
    // Calculate the nimber for each possible move
    // XOR the nimbers resulting from a split
    for mov in moves::calculate_legal_moves(&[Stack(height)], rules, (pool_coins, 0)) {
        match mov {
            NimAction::Take(TakeAction {
                stack_index: _,
                amount: take,
                split,
                from: _,
            }) => match split {
                NimSplit::Yes(a, b) => {
                    // TODO check if this handles pool coins correctly
                    //  note: probably yes, since we'd want to avoid infinite recursion
                    //  more notes: we're probably missing cases where we could re-distribute coins from one stack to another
                    let nimber_a = calculate_nimber_for_height(a.0, rules, pool_coins);
                    let nimber_b = calculate_nimber_for_height(b.0, rules, pool_coins);
                    exclusion_list.push(nimber_a ^ nimber_b);
                }
                NimSplit::No => {
                    let nimber = calculate_nimber_for_height(height - take, rules, pool_coins);
                    exclusion_list.push(nimber);
                }
            },
            NimAction::Place(PlaceAction {
                stack_index,
                amount,
                from: _,
            }) => {
                // We set the `pool_coins` to 0, since we don't want to get into an infinite loop
                // TODO check if that's correct
                let nimber = calculate_nimber_for_height(height + amount, rules, 0);
                exclusion_list.push(nimber);
            }
        }
    }

    // Calculate the nimber using the MEX rule
    let mut nimber = Nimber(0);
    while exclusion_list.contains(&nimber) {
        nimber.0 += 1;
    }

    // // Cache the nimber
    with_cache(rules, |cache| cache.insert((height, pool_coins), nimber));

    nimber
}

// #[cfg(test)]
// mod tests {
//     use crate::Stack;
//
//     use super::*;
//
//     lazy_static! {}
// }
