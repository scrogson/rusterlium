# Rusterlium

> An experimental crate for building Erlang NIFs (native implemented functions)
> with Rust.

I want to see how far I can go to provide the most ergonomic API for building
NIFs. Rustler is a good start, but I think we can go a bit further.

## Goals

### Automatic Encode/Decode

Given that all NIFs have the same signature:

```c
static ERL_NIF_TERM fun(ErlNifEnv* env, int argc, const ERL_NIF_TERM argv[])
```

It would be nice to provide a way to define a Rust function that would only be
concerned with Rust types:

```rust
fn add(_env: Env, a: i64, b: i64) -> i64 {
  a + b
}
```

The arguments (`a` and `b` in this case) would automatically be decoded from
`ERL_NIF_TERM` to an `i64`. The return type (`i64`) would automatically be
encoded into the appropriate `ERL_NIF_TERM`.

Essentially, arguments must implement `Decode` while all return types must
implement `Encode`.
