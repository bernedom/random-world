# random-world

[![CI](https://github.com/bernedom/random-world/actions/workflows/rust.yml/badge.svg)](https://github.com/bernedom/random-world/actions)

random-world creates and prints a string by guessing each character of the string until the string matches "Random World!". The application itself serves no useful purposes except for me to learn [rust](https://www.rust-lang.org/). Guessing the characters happens in parallel threads as rust boasts of `fearless concurrency`

Set up rust by following the [getting started instructions](https://www.rust-lang.org/learn/get-started)

example output: 
```
$cargo run
Found another char 'R'  after 56 iterations!
Found another char 'a'  after 103 iterations!
Found another char 'n'  after 319 iterations!
Found another char 'd'  after 411 iterations!
Found another char 'o'  after 463 iterations!
Found another char 'm'  after 474 iterations!
Found another char ' '  after 477 iterations!
Found another char 'W'  after 707 iterations!
Found another char 'o'  after 759 iterations!
Found another char 'r'  after 813 iterations!
Found another char 'l'  after 905 iterations!
Found another char 'd'  after 997 iterations!
Found another char '!'  after 1006 iterations!
Found desired string 'Random World!' after 1007 iterations
```

# build & run

```
cargo build
cargo run
```

# Rust specific features covered in this example

* build & run 
* usage of dependency `rand` and `crossbeam`
* mutable vs non-mutable variables
* functions
* iterators
* closures
* parallel threads using `crossbeam`
* optionals