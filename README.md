# random-world

random-world creates a string by guessing each character of the string until the string matches "random word!" and then printing it out. The application itself serves no useful purposes except for me to learn [rust](https://www.rust-lang.org/). 

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
* `match` statement
* `optionals` 
* usage of dependency `rand`
* mutable vs non-mutable variables




