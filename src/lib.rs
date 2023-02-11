//! [![github]](https://github.com/Tanja-4732/nimlib)&ensp;[![crates-io]](https://crates.io/crates/nimlib)&ensp;[![docs-rs]](https://docs.rs/nimlib/latest/nimlib)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! NimLib is a Rust library for [Nim games](https://en.wikipedia.org/wiki/Nim): calculate nimbers and possible moves
//!
//! NimLib is work-in-progress at the moment.  
//! Features such as Poker-Nim (coin pools) are not yet implemented.

#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod game;
pub mod moves;
pub mod nimbers;

pub use game::*;
