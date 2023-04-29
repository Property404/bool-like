# Bool-Like

This crate contains an attribute macro `#[bool_like]`. By default, this macro
just implements `std::ops::Not` for a simple two-variant enum. `!` applied to
one variant will produce the other.

Optionally, the sub-attribute `#[into_false]` may be applied to one of the
variants to indicate that variant is equivalent to `false`, and the other
variant is equivalent to `true`. `std::convert::From` will be implemented for
both the enum and `bool`.

```rust
use bool_like::bool_like;

/// An answer to a question.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[bool_like]
enum Answer {
    #[into_false]
    No,
    Yes,
}

assert_eq!(! Answer::No, Answer::Yes);
assert!(bool::from(Answer::Yes));
```

## License

MIT or Apache 2.0
