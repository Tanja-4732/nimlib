# Changelog

## [Unreleased]

<!-- _No unreleased changes_ -->

### Added

- CLI to access library functions:
- Calculate splits for a given height (with the `splits` subcommand)
  - As formatted text (default)
  - As CSV (with the `--csv` flag or `-c` short flag)
- `--version` and `--help` flags
- `make-rule-set` subcommand to create a JSON string describing a Nim game's rules
  - Use `--take-split-never` or `-n` either followed by a number to specify a rule where a player may take a number of coins from a stack without splitting it in the process
  - Likewise, `--take-split-optional` or `-o` followed by a number specifies a take rule where splitting is optional
  - The `--take-split-always` or `-a` option followed by a number specifies a take rule where splitting is mandatory
  - Using `--allow-any-take` or `-A` creates a rule allowing to take an arbitrary number of coins from a stack, followed by a choice from `never`, `optional`, or `always` to specify if splitting is forbidden, optional, or mandatory
  - The yet-to-be-implemented `--allow-place` or `-p` option creates a rule which allows the player to place coins on a stack; this option is not yet implemented and does not take any arguments
  - `--pretty-print` or `-P` prints the JSON string in a more readable format, with indentation and line breaks
- The `nimber` subcommand to
  - Calculate the nimber(s) for a (set of) stack(s) of given height(s) (using `--print stacks` or `-p stacks`)
  - Calculate the nimber of a position of stacks (using `--print position` or `-p position`)
  - Display them both (default, or by using `--print both` or `-p both`)
  - Rules are taken with the required `--rules` or `-r` flag in the form of a JSON string
  - The `--json` or `-j` flag can be used to print the output as JSON instead of plain text
  - Likewise, the `--json-pretty` or `-J` flag can be used to print the output as pretty-printed JSON
- Logging support with verbosity controls using `--verbose` or `-v` (can be used multiple times), or `--quiet` or `-q` (can be used multiple times)

### Changed

- Applied many `clippy` suggestions

## [0.1.1] - 2023-02-11

### Changed

- Improved readme and documentation
  - Fix links: link to _latest_ to help search engines find the latest version
  - Add a description of the library to the readme
  - Explain how Nim games work

## [0.1.0] - 2023-02-11

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
- Function `calculate_legal_moves`
  - Argument `stacks` was moved from `&Vec<Stack>` to `&[Stack]`, allowing for an optimization in this `calculate_nimber_for_height`
  - Same goes for its `rules` argument, which was moved from `&Vec<NimRule>` to `&[NimRule]`
- Changed return type of `nimbers::calculate_splits` `Vec<(u64, u64)>` from to `Vec<(Stack, Stack)>`

### Removed

- Removed `clear_nimber_cache` from `nimbers` (then named `algo`)
  - The cache now supports multiple rule sets, so clearing the cache is no longer necessary when the rules change
  - Maybe some kind of cache control will be added in the future to reduce memory usage on demand

### Fixed

- Fixed a bug in the way the cache was used, which resulted in incorrect nimbers being returned
  - This was due to the fact that the same cache was used for every rule set
  - Now, a new cache is created for every rule set

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
