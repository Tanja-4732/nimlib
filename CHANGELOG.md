# Changelog

## [Unreleased]

### Added

- Module `moves`
  - Enum `MoveError`, a list of possible errors when applying or calculating moves
  - `check_move` is a function which checks if a move is legal for a given position
  - The function `calculate_legal_moves` which calculates all legal moves from a given position
  - `apply_move` is a function which applies a move to a position, changing the position
  - The unsafe version of which is `apply_move_unchecked` which does not check if the move is legal
- Struct `Nimber` which represents a nimber and simply wraps a `u64`
  - Used to avoid confusion of heights and nimbers using the type system
- Enum `NimAction` which represents a move in a Nim game
- Structs `TakeAction` and `PlaceAction` which are used in `NimAction`
- Enum `NimSplit`, the result of a split: either `Yes` with two new stacks or `No`
- Many unit tests for both the old code, and the new

### Changed

- Renamed module `algo` to `nimbers`
- Function `calculate_nimber_for_height` in `nimbers`
  - Now returns a `Nimber` instead of a `u64`
  - Now takes a slice of `NimRule` instead of a hard-coded vector `&Vec<NimRule>`
  - Moved internal code to `moves::calculate_legal_moves` (code deduplication)
- Changed return type of `nimbers::calculate_splits` `Vec<(u64, u64)>` from to `Vec<(Stack, Stack)>`

## [0.0.1] - 2023-01-25

### Added

- This is the initial release of NimLib
  - Version `0.0.0` was a placeholder release without any code
  - Version `0.0.1` is the first real release
- Module `algo`
  - The `calculate_nimber_for_height` function which calculates the nimber for a given height
  - The `calculate_splits` function which calculates all possible splits for a given height, accounting for symmetry
  - The module also contains the function `clear_nimber_cache` which has/had to be called every time the rules changed
- Struct `NimStack` which represents a stack and simply wraps a `u64`
- Struct `NimRule` representing a rule in a Nim game
- Struct `NimGame` representing a Nim game, containing a list of rules and a list of stacks
- Enum `Split`, used in `NimRule`
- Enum `TakeSize` which is used in `NimRule`
