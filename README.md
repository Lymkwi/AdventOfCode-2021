# Advent of Code 2021

My solutions for the [Advent Of Code 2021 puzzles](https://adventofcode.com/2021/) written in Rust.

Can be found both on [GitHub](https://github.com/Lymkwi/AdventOfCode-2021) and [my Gitea](https://git.vulpinecitrus.info/Lymkwi/AdventOfCode-2021).

This code in organized with :
 - A top-level crate to run tests and benchmarks
 - Sub-level crates for each day (`dayXX`)
 - A common crate for the common methods

## Summary :

Stars obtained :
```
⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐
⭐⭐⚫⚫⚫⚫⚫⚫⚫⚫
⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫
⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫
⚫⚫⚫⚫⚫⚫⚫⚫⚫⚫
```

Run times (on a ThinkPad X230 with Intel i7-3520M) :
|        | Day 01 | Day 02 | Day 03 | Day 04 | Day 05 |
|--------|--------|--------|--------|--------|--------|
| Part 1 |   50us |  161us |  294us |  462us |   11ms |
| Part 2 |   55us |  160us |  433us |  569us |   22ms |
|        |**Day 06**|**Day 07**|**Day 08**|**Day 09**|**Day 10**|
| Part 1 |   91us |
| Part 2 |  270us |

In order to check those benchmarks, run `cargo bench` on the root crate.

## Sub-level day crates

The sublevel day crates are both executable and libraries. The main logic is
always implemented in `lib.rs` but a main method exists in `main.rs` to read
the data file and show the answers. So, you can go to any day and run
`cargo run` to see the day's answers.
```
dayXX
|- Cargo.toml
`- src/
   |- main.rs
   `- lib.rs
```

### Tests

Every sub-level day crate contains tests for the examples given on that day. You can run `cargo test` in those day crates to see that examples are successfully processed.

## The common crate

The `common` crate defines methods used by multiple day crates :
 - `read_data` : reads the data from the file into a `String`

## Top-level tests and benchmarks

The metrics provided above are computed using `cargo bench` in the top-level crate. That crate also contains tests to check that the results of the computation
are still valid for all days and parts.
