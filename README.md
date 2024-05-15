# SNES SameGame (J) solver for "SameGame, Easy"

## Requirement

* x64 CPU with BMI2 instructions
* Some RAM (16 GB will be sufficient)

I tested only on Linux.

## Usage

`solve_all` binary searches the maximum score for the legal boards in the game.
I recommend to specify a initial maximum score for pruning (the known best score is 844).

```sh
cargo --example=solve_all --profile=release-lto -- --prune-score-max=800
```
