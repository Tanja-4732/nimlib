//! The inner workings of the nimber calculation algorithms;  
//! also contains [clear_nimber_cache], which you **may need to call yourself** (see [clear_nimber_cache])
//!
//! Useful if you want to calculate nimbers for heights that are not part of a game,
//! or if you want to include just the lower-level nimber calculation algorithms in your project.
//!
//! Includes helper functions like [calculate_splits].

use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;

use crate::game::{NimRule, Split, TakeSize};

lazy_static! {
    static ref NIMBER_CACHE: RwLock<HashMap<(u64, u64), u64>> = Default::default();
}

/// Clears the cache used by the nimber calculation algorithms.
///
/// This is useful if you want to calculate nimbers for a different set of rules.   
/// Currently, the cache is not cleared automatically, leading to incorrect results
/// if you use different rules for a stack height calculated before,
/// either explicitly or internally.
pub fn clear_nimber_cache() {
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
/// use nimlib::algo::calculate_splits;
///
/// assert_eq!(calculate_splits(0), vec![]);
/// assert_eq!(calculate_splits(1), vec![]);
/// assert_eq!(calculate_splits(2), vec![(1, 1)]);
/// assert_eq!(calculate_splits(3), vec![(1, 2)]);
/// assert_eq!(calculate_splits(4), vec![(1, 3), (2, 2)]);
/// assert_eq!(calculate_splits(5), vec![(1, 4), (2, 3)]);
/// assert_eq!(calculate_splits(6), vec![(1, 5), (2, 4), (3, 3)]);
/// ```
pub fn calculate_splits(height: u64) -> Vec<(u64, u64)> {
    // Stacks of height 0 and 1 can't be split
    if height <= 1 {
        return Vec::new();
    }

    let mut splits = Vec::new();

    for i in 1..=height / 2 {
        splits.push((i, height - i));
    }

    splits
}

// # Examples
// ```
// use nimlib::algo::calculate_nimber_for_height;
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
pub fn calculate_nimber_for_height(height: u64, rules: &Vec<NimRule>, pool_coins: u64) -> u64 {
    // Check if we've already calculated this nimber
    if let Some(nimber) = NIMBER_CACHE.read().unwrap().get(&(height, pool_coins)) {
        return *nimber;
    }

    // TODO handle pool coins correctly
    assert_eq!(pool_coins, 0, "Pool coins not yet supported");

    // Use the MEX (minimum excluded) rule to calculate the nimber
    let mut exclusion_list: Vec<u64> = Vec::new();

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
                                        calculate_nimber_for_height(a, rules, pool_coins)
                                            ^ calculate_nimber_for_height(b, rules, pool_coins),
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
                                        calculate_nimber_for_height(a, rules, pool_coins)
                                            ^ calculate_nimber_for_height(b, rules, pool_coins),
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
                                    calculate_nimber_for_height(a, rules, pool_coins)
                                        ^ calculate_nimber_for_height(b, rules, pool_coins),
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
                                    calculate_nimber_for_height(a, rules, pool_coins)
                                        ^ calculate_nimber_for_height(b, rules, pool_coins),
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
                                    calculate_nimber_for_height(a, rules, pool_coins - c)
                                        ^ calculate_nimber_for_height(b, rules, pool_coins - c),
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
                                    calculate_nimber_for_height(a, rules, pool_coins - c)
                                        ^ calculate_nimber_for_height(b, rules, pool_coins - c),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // Calculate the nimber using the MEX rule
    let mut nimber = 0;
    while exclusion_list.contains(&nimber) {
        nimber += 1;
    }

    // Cache the nimber
    NIMBER_CACHE
        .write()
        .unwrap()
        .insert((height, pool_coins), nimber);

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
