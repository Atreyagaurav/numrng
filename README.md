# Number Range

Interprets human redable number range into list of numbers.  For example: `"1-2"` means `[1,2]`, `"1,3:5"` means `[1,3,4,5]`, and so on.

## Format
The number ranges can consist of comma separated list of numbers or ranges.

- `number` ⇒ Single number (e.g. `3`)
- `start:end` ⇒ Inclusive Range with step 1 (e.g. `1:10`)
- `start:step:end` ⇒ Inclusive Range with variable step (e.g. `1:2:10`)

Example: `-10,3:10,14:2:20` => `[-10, 3, 4, 5, 6, 7, 8, 9, 10, 14, 16, 18, 20]`

## Usage
It is useful to input a bunch of similar commands by piping the input with different numbers.

For example in bash `{1..5}` can be expanded to numbers `1` to `5`, but there isn't an easy way to do that for discontinuous ranges. So with this command, you can have that.

with gnu `parallel` you can run something like: `numrng 1-3,7-8 | parallel echo {}`

## Installation
Follow the instructions to [install rust](https://www.rust-lang.org/tools/install). 

Then with cargo do: `cargo build --release`. You'll have the compiled binary. Move `target/release/numrng` to the path.

For Arch Linux, [AUR package `numrng`](https://aur.archlinux.org/packages/) is available.

## Extra
 `_` can be used to separate the numbers without it meaning anything. For example: `1_000_000-1_000_100` will go from 1000000 to 1000100.

For how it works and use in your own program refer to [`number_range`](https://docs.rs/number_range/latest/number_range/) crate for Rust.
