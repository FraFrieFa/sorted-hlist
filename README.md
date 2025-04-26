# `sorted-hlist`

A zero-cost, type-level heterogeneous list (HList) implementation in Rust with support for compile-time sorting constraints and intersection operations — powered by [`typenum`](https://docs.rs/typenum).

## Features

- Type-level HLists: `HCons` and `HNil`
- Compile-time enforcement of sortedness (`SortedHList`)
- Type-level set intersection via the `Intersect` trait
- Type-safe macro `mk_hlist!(...)` for building HLists
- No runtime overhead — all type-level logic only

## Example

```rust
use sorted_hlist::{mk_hlist, Intersect};
use typenum::{U1, U2, U3, U4};

// Create type-level HLists
type A = mk_hlist!(U1, U2, U3);
type B = mk_hlist!(U2, U3, U4);

// Compute intersection
type Common = <A as Intersect<B>>::Output;

// Common = mk_hlist!(U2, U3)
```

## Crate Goals

- Use in embedded HALs or systems programming where traits and sets represent hardware capabilities
- Keep things fast, predictable, and compile-time checked
- No dependencies other than `typenum`

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Pull requests, suggestions and improvements welcome!
