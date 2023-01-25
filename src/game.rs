use serde::{Deserialize, Serialize};

use crate::algo;

/// # A Nim game
///
/// This struct uses [NimRule]s to calculate the nimber of the position.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NimGame {
    pub(crate) rules: Vec<NimRule>,
    pub(crate) stacks: Vec<Stack>,
    pub(crate) coins_a: u64,
    pub(crate) coins_b: u64,
}

impl Default for NimGame {
    fn default() -> Self {
        Self {
            rules: vec![NimRule {
                take: TakeSize::List(vec![1, 2, 3]),
                split: Split::Never,
            }],
            stacks: vec![Stack(10)],
            coins_a: 0,
            coins_b: 0,
        }
    }
}

impl NimGame {
    pub fn new(rules: Vec<NimRule>, stacks: Vec<Stack>) -> Self {
        // TODO allow pool coins to be set

        Self {
            rules,
            stacks,
            ..Default::default()
        }
    }

    /// Calculate the nimber of the position using the MEX & XOR rules
    pub fn calculate_nimber(&self) -> u64 {
        // FIXME handle pool coins

        self.stacks.iter().fold(0, |nimber, stack| {
            nimber ^ stack.calculate_nimber(&self.rules, 0)
        })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Stack(pub u64);

impl Stack {
    pub fn calculate_nimber(&self, rules: impl AsRef<Vec<NimRule>>, pool_coins: u64) -> u64 {
        algo::calculate_nimber_for_height(self.0, rules.as_ref(), pool_coins)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Split {
    Never,
    Optional,
    Always,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TakeSize {
    List(Vec<u64>),
    Any,
    Place,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NimRule {
    pub take: TakeSize,
    pub split: Split,
}

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
// pub enum NimRule {
//     Take(NimTakeRule),
//     Place,
// }