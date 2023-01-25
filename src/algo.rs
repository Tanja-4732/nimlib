use std::{borrow::Borrow, collections::HashMap, sync::RwLock};

use cached::proc_macro::cached;
use lazy_static::lazy_static;

use crate::game::{NimRule, Split, Stack, TakeSize};

lazy_static! {
    // static ref SPLIT_CACHE: RwLock<HashMap<u64, Vec<(u64, u64)>>> = RwLock::new(HashMap::new());
    static ref NIMBER_CACHE: RwLock<HashMap<(u64, u64), u64>> = RwLock::new(HashMap::new());
}

/// Calculate the number of ways to split a number into two parts,
/// where the sum of the parts is the original number,
/// accounting for symmetry.
#[cached]
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

pub fn calculate_nimber_for_height(height: u64, rules: &Vec<NimRule>, pool_coins: u64) -> u64 {
    // Check if we've already calculated this nimber
    if let Some(nimber) = NIMBER_CACHE.read().unwrap().get(&(height, pool_coins)) {
        return *nimber;
    }

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
