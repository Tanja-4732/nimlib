# NimLib

[![github]](https://github.com/Tanja-4732/nimlib)&ensp;[![crates-io]](https://crates.io/crates/nimlib)&ensp;[![docs-rs]](https://docs.rs/nimlib/latest/nimlib)

A Rust library for [Nim games](https://en.wikipedia.org/wiki/Nim): calculate nimbers and possible moves

## A game called Nim

[_Nim_](https://en.wikipedia.org/wiki/Nim) refers to a set of (mathematical/strategy) games where two players take turns in removing coins from stacks of coins.
The player who makes the last legal move wins.

Every Nim game consists of a set of rules, which define how many coins may be removed from a stack in a single move.
Each move affects exactly one stack of coins, possibly requiring the stack to be split into two non-empty stacks.

A version of Nim called _Poker-Nim_ is played with the addition of coin-pools, where removed coins are kept in the player's pool,
to be placed in future moves, if allowed by the rules, and desired by the player.

## About the library

This library calculates the nimbers for stacks of a given height, and the possible moves for a given position.

### Nimbers

The _nimber_ of a stack can be thought of as some kind of score or evaluation of how good this position is for the player who is to move.  
See the [`nimlib::nimbers`](https://docs.rs/nimlib/latest/nimlib/nimbers/index.html) module for functions for calculating nimbers.

A nimber of 0 means that the player who is to move has no winning strategy, and will lose the game, no matter what move they make, given their opponent plays optimally.  
Conversely, a nimber greater than 0 means that the player who is to move has a winning strategy, and will win the game, no matter what move their opponent makes, given the player to move plays optimally.

### Moves

The [`nimlib::moves`](https://docs.rs/nimlib/latest/nimlib/moves/index.html) module provides functions for calculating the possible moves for a given position.

A position simply refers to a list/set of stacks, the nimber of a position is the XOR-sum of the nimbers of the stacks in the position,
the rules regarding winning conditions for the entire position are the same as for the individual stacks.

## License

[![GNU Lesser General Public License v3.0](https://www.gnu.org/graphics/lgplv3-with-text-154x68.png)](https://www.gnu.org/licenses/lgpl-3.0.html)

Copyright (C) 2023 [@Tanja-4732](https://github.com/Tanja-4732)

NimLib is free software: you can redistribute it and/or modify it under the terms of the [GNU Lesser General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

NimLib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Lesser General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Lesser General Public License](/LICENSE.md) along with NimLib. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/lgpl-3.0.html>.

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
