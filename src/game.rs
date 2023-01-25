//! The primary game structs are in this module;  
//! For game logic, see [crate::algo].

use serde::{Deserialize, Serialize};

use crate::algo;

/// # A Nim game
///
/// This struct uses [NimRule]s to calculate the nimber of the position.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NimGame {
    /// The rules of the game (e.g. which numbers of coins can be taken)
    pub(crate) rules: Vec<NimRule>,

    /// The stacks of the game, represented as their current heights
    pub(crate) stacks: Vec<Stack>,

    /// The number of coins in the pool of player A  
    /// (ignored for now)
    pub(crate) coins_a: u64,

    /// The number of coins in the pool of player B  
    /// (ignored for now)
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
    /// Create a new Nim game with the given rules and stacks
    ///
    /// # Examples
    ///
    /// ```
    /// use nimlib::{NimGame, NimRule, Split, Stack, TakeSize};
    ///
    /// let simple_rules: Vec<NimRule> = vec![NimRule {
    ///     take: TakeSize::List(vec![1, 2, 3]),
    ///     split: Split::Never,
    /// }];
    ///
    /// let stacks: Vec<Stack> = vec![Stack(10)];
    ///
    /// let game = NimGame::new(simple_rules, stacks);
    /// ```
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

/// Represents a stack of coins; specifically its height.  
/// Simply wraps a [u64].
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Stack(pub u64);

impl Stack {
    /// Calculate the nimber of the stack using the MEX & XOR rules
    ///
    /// For now, `pool_coins` must be 0.
    pub fn calculate_nimber(&self, rules: impl AsRef<Vec<NimRule>>, pool_coins: u64) -> u64 {
        algo::calculate_nimber_for_height(self.0, rules.as_ref(), pool_coins)
    }
}

/// Specifies if a player may/must split a stack into two non-empty stacks after taking coins
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Split {
    /// Splitting the stack is not allowed
    Never,

    /// The stack may be split into two non-empty stacks after taking coins
    Optional,

    /// The stack must be split into two non-empty stacks after taking coins
    Always,
}

/// Specifies the number of coins that can be taken from a stack in a single move according to a rule.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TakeSize {
    /// A list of possible numbers which may be taken from a stack in a single move,
    /// if enough coins are available.
    ///
    /// E.g. `[1, 2, 3]`, `[3, 6, 10]`, or `[42]`
    List(Vec<u64>),

    /// Any number of coins less than or equal to the stack height may be taken.
    Any,

    /// The player may place coins into the stack from their pool (none are taken),  
    /// For use with Poker-Nim
    Place,
}

/// A rule for a Nim game.  
/// This struct specifies a set of possible moves for a player.  
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NimRule {
    /// Specifies the number of coins that can be taken from a stack in a single move
    pub take: TakeSize,

    /// Specifies whether the player may/must split a stack into two stacks
    pub split: Split,
}

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
// pub enum NimRule {
//     Take(NimTakeRule),
//     Place,
// }
