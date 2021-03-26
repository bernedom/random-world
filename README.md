# random-world

[![CI](https://github.com/bernedom/random-world/actions/workflows/rust.yml/badge.svg)](https://github.com/bernedom/random-world/actions)

random-world creates and prints a string by guessing each character of the string until the string matches "Random World!". The application itself serves no useful purposes except for me to learn [rust](https://www.rust-lang.org/). Guessing the characters happens in parallel threads as rust boasts of `fearless concurrency`

Set up rust by following the [getting started instructions](https://www.rust-lang.org/learn/get-started)

example output: 
```
$cargo run
Found char 'R' at position 0 after 130 iterations!
Found char 'a' at position 1 after 72 iterations!
Found char 'n' at position 2 after 92 iterations!
Found char 'd' at position 3 after 154 iterations!
Found char ' ' at position 6 after 22 iterations!
Found char 'o' at position 4 after 156 iterations!
Found char 'm' at position 5 after 161 iterations!
Found char 'W' at position 7 after 34 iterations!
Found char 'o' at position 8 after 33 iterations!
Found char 'r' at position 9 after 1 iterations!
Found char 'l' at position 10 after 33 iterations!
Found char 'd' at position 11 after 125 iterations!
Found char '!' at position 12 after 328 iterations!
Found desired string 'Random World!'
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