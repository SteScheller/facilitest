# facilitest

`facilitest` aims to facilitate writing tests (pun intended) by relieving you of writing boilerplate code and providing additional functionality such as parametrized tests.

# Usage

Run `cargo add facilitest` or add this to your `Cargo.toml`:

```toml
[dependencies]
facilitest = "1.0.0"
```

Add this to your test module:

```rust
use facilitest::*;
```

# Example

Test a function with a set of different arguments:

```
test_p! {
    <function identifier>,
    (
        <test case label>: (<input arguments>), <expected result>
        ...
    )
}
```
**Note**: *test case labels* must be valid identifiers. Prefix numbers with an underscore if you
want to numerate them.

```rust
#[cfg(test)]
mod example {
    use facilitest::*;

    test_p! {
        std::cmp::max,
        (
            _0: (-10, -1), -1
            _1: (-10, 10), 10
            _2: ("a", "A"), "a"
        )
    }
}
```

```
$ cargo test

...

running 3 tests
test example::test_max_0 ... ok
test example::test_max_1 ... ok
test example::test_max_2 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
