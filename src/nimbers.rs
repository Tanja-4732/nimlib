//! The inner workings of the nimber calculation algorithms;  
//! also contains [clear_nimber_cache], which you **may need to call yourself** (see [clear_nimber_cache])
//!
//! Useful if you want to calculate nimbers for heights that are not part of a game,
//! or if you want to include just the lower-level nimber calculation algorithms in your project.
//!
//! Includes helper functions like [calculate_splits].

use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;

use crate::{
    game::{NimRule, Split, TakeSize},
    Nimber, Stack,
};

/// The nimber cache is a map from (height, pool_coins) to nimber.
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

// TODO remove this method
/// Clears the cache used by the nimber calculation algorithms.
///
/// This is useful if you want to calculate nimbers for a different set of rules.   
/// Currently, the cache is not cleared automatically, leading to incorrect results
/// if you use different rules for a stack height calculated before,
/// either explicitly or internally.
#[deprecated]
pub(crate) fn clear_nimber_cache() {
    NIMBER_CACHE.write().unwrap().clear();
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
pub fn calculate_splits(height: u64) -> Vec<(Stack, Stack)> {
    // Stacks of height 0 and 1 can't be split
    if height <= 1 {
        return Vec::new();
    }

    let mut splits = Vec::new();

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

macro_rules! with_cache {
    ($rules:expr, $f:expr) => {{
        // dbg!("inserting cache");
        let mut caches = NIMBER_CACHE.write().unwrap();
        let cache = if let Some(cache) = caches.get_mut($rules) {
            cache
        } else {
            caches.insert($rules.clone(), Default::default());
            caches.get_mut($rules).unwrap()
        };

        $f(cache)
    }};
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
///
pub fn calculate_nimber_for_height(height: u64, rules: &Vec<NimRule>, pool_coins: u64) -> Nimber {
    // Check if we've already calculated this nimber
    // if let Some(nimber) = get_cache_for_rules!(rules).get(&(height, pool_coins)) {
    if let Some(nimber) = with_cache!(rules, |cache: &NimberCache| cache
        .get(&(height, pool_coins))
        .map(|n| *n))
    {
        return nimber;
    }

    // TODO handle pool coins correctly
    assert_eq!(pool_coins, 0, "Pool coins not yet supported");

    // Use the MEX (minimum excluded) rule to calculate the nimber
    let mut exclusion_list: Vec<Nimber> = Vec::new();

    // Calculate the nimber for each rule
    // XOR the nimbers resulting from a split
    for NimRule { split, take } in rules {
        match take {
            TakeSize::List(take_sizes) => {
                for take_size in take_sizes {
                    if height >= *take_size {
                        match split {
                            Split::Never => {
                                exclusion_list.push(calculate_nimber_for_height(
                                    height - take_size,
                                    rules,
                                    pool_coins,
                                ));
                            }
                            Split::Optional => {
                                for (a, b) in calculate_splits(height.saturating_sub(*take_size)) {
                                    exclusion_list.push(
                                        calculate_nimber_for_height(a.0, rules, pool_coins)
                                            ^ calculate_nimber_for_height(b.0, rules, pool_coins),
                                    );
                                }

                                exclusion_list.push(calculate_nimber_for_height(
                                    height - take_size,
                                    rules,
                                    pool_coins,
                                ));
                            }
                            Split::Always => {
                                for (a, b) in calculate_splits(height.saturating_sub(*take_size)) {
                                    exclusion_list.push(
                                        calculate_nimber_for_height(a.0, rules, pool_coins)
                                            ^ calculate_nimber_for_height(b.0, rules, pool_coins),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            TakeSize::Any => {
                for h in 1..=height {
                    match split {
                        Split::Never => {
                            exclusion_list.push(calculate_nimber_for_height(
                                height - h,
                                rules,
                                pool_coins,
                            ));
                        }
                        Split::Optional => {
                            for (a, b) in calculate_splits(height.saturating_sub(h)) {
                                exclusion_list.push(
                                    calculate_nimber_for_height(a.0, rules, pool_coins)
                                        ^ calculate_nimber_for_height(b.0, rules, pool_coins),
                                );
                            }

                            exclusion_list.push(calculate_nimber_for_height(
                                height - h,
                                rules,
                                pool_coins,
                            ));
                        }
                        Split::Always => {
                            for (a, b) in calculate_splits(height.saturating_sub(h)) {
                                exclusion_list.push(
                                    calculate_nimber_for_height(a.0, rules, pool_coins)
                                        ^ calculate_nimber_for_height(b.0, rules, pool_coins),
                                );
                            }
                        }
                    }
                }
            }
            TakeSize::Place => {
                // The player can add 1..pool_coins coins to the stack
                // The placed coins are taken from the pool
                for c in 1..=pool_coins {
                    match split {
                        Split::Never => {
                            exclusion_list.push(calculate_nimber_for_height(
                                height + c,
                                rules,
                                pool_coins - c,
                            ));
                        }
                        Split::Optional => {
                            for (a, b) in calculate_splits(height + c) {
                                exclusion_list.push(
                                    calculate_nimber_for_height(a.0, rules, pool_coins - c)
                                        ^ calculate_nimber_for_height(b.0, rules, pool_coins - c),
                                );
                            }

                            exclusion_list.push(calculate_nimber_for_height(
                                height - c,
                                rules,
                                pool_coins - c,
                            ));
                        }
                        Split::Always => {
                            for (a, b) in calculate_splits(height + c) {
                                exclusion_list.push(
                                    calculate_nimber_for_height(a.0, rules, pool_coins - c)
                                        ^ calculate_nimber_for_height(b.0, rules, pool_coins - c),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // Calculate the nimber using the MEX rule
    let mut nimber = Nimber(0);
    while exclusion_list.contains(&nimber) {
        nimber.0 += 1;
    }

    // // Cache the nimber
    // get_cache_for_rules!(rules).insert((height, pool_coins), nimber);
    with_cache!(rules, |cache: &mut NimberCache| {
        cache.insert((height, pool_coins), nimber);
    });

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
