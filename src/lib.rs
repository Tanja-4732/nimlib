//! # NimLib
//!
//! A Rust library for [Nim games](https://en.wikipedia.org/wiki/Nim): calculate nimbers and possible moves
//!
//! NimLib is work-in-progress at the moment.  
//! Features such as Poker-Nim (coin pools) are not yet implemented,  
//! and the move generation algorithm is not yet ported to this library.

#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod game;
pub mod moves;
pub mod nimbers;

pub use game::*;
