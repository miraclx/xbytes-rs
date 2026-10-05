# AGENTS.md

`xbytes`: parse and format byte sizes, both directions, with exact arithmetic underneath. Lib only,
edition 2024, toolchain pinned to 1.97.1. Published: `docs.rs/xbytes`, `readme = "README.md"`, rustdoc
workflow in `.github/workflows/rustdoc.yml`.

- `STYLE.md` is the contract, synced byte-identical from `craft/`. Do not edit the copy; edit the
  canonical and run `just sync`. One deliberate deviation is recorded in `Cargo.toml`, not in the file.
- `miraclx/REVIEW.md` is how this code gets reviewed. `notes/craft-backlog.md` is what is open.

## The trap is the mirror image of the siblings'

This crate has no sibling dependencies. It **is** the sibling: `libxget-rs` builds against the working copy
here through a gitignored path override while its `Cargo.lock` pins a remote rev. A public-surface change
here can break xget's build while xget's CI stays green on the old pin. After touching anything `pub`,
rebuild the consumer (`cargo update -p xbytes` in `libxget-rs`, then its gates).

**Before removing or renaming any `pub` item, grep the consumers.** `Mode`, `Format`, `Unit` and the
`ByteSize` entries are a cross-repo contract today. In-repo zero callers is not evidence.

`unreachable_pub` is not enabled. Check a new `pub` is reachable before adding it.

## Gates

```
cargo clippy --all-targets -- -D warnings                 # default, --all-features, --no-default-features
cargo test --all-targets <features> && cargo test --doc <features>    # seven feature sets in CI
cargo +nightly fmt --check
cargo sort --workspace --check
```

The feature matrix is the gate, not the default build. `Int`, `Float` and four method signatures change
shape across it, so green on defaults proves nothing about `--no-default-features`. CI runs all seven;
run at least the two ends locally.

`Cargo.lock` is tracked here, and `.gitignore` still carries a `Cargo.lock` line that does nothing.
One of the two is wrong; it is on the backlog.

## The store flips on features

`Int` is `u128` under `u128`, `u64` otherwise. `Float` is `f64` without `lossless` and an exact
`fraction::GenericFraction<Int>` with it (`lib.rs`, the aliases). `no-panic` implies `lossless`.

Four `ByteSize` methods change **return type** on the `bits` feature (`bytesize.rs`, `from_bits` /
`from_bytes` / `bit_count` / `byte_count`): infallible on one side, `Result` on the other.
`byte_count_lossy` and `bit_count_lossy` are the feature-stable infallible escape hatches (same file).
A downstream `match` compiles against every set because `ParseError`'s variant set is stable by contract
(`lib.rs`, the case-format variants exist even where they are never produced).

`Ord` on `ByteSizeRepr` exists **only** under `lossless`; without it there is only `PartialOrd`. Ordering
is unit magnitude first, then value, so `1 MiB < 1 MB` by construction.

Arithmetic saturates instead of panicking, `Sub` underflowing to zero; division by zero saturates to the
ceiling (`bytesize.rs`, `division_by_zero_saturates_to_the_ceiling`). `Mul`/`Div` take
`impl TryInto<Int>`, and a right-hand side that does not convert **silently no-ops the scale**
(`bytesize.rs`, the ops macro). That silence is on the backlog.

## Entries and refiners

Mode entries live on `ByteSize`: `.iec()`, `.si()`, `.bits()`, `repr(Mode)`, `repr_as(unit)`
(`bytesize.rs`). Refiners live on the returned value: `.long()`, `.condensed()`, `.symbol()`,
`.precision(n)`, `.thousands()`, `.separator(s)`, `.pin(unit)`, `.plural(bool)`, `.no_space()`,
`.lower()` (same file). Anything they do not name goes through `.with(...)` with `Format` flags and
`ReprConfigVariant` knobs; the README names that escape hatch explicitly.

There is **no** `.decimal()`, `.no_prefix()`, `.per_sec()` or `.repr_with(Mode)`. `Mode::NoPrefix` exists
as a flag and is reachable only as `repr(Mode::NoPrefix)`; decimal rendering is `.si()`; there is no rate
concept at all, and a rate must never be added here (that decision is closed, in the backlog). Do not
"complete the set" without reading the reshape record first.

`.precision(n)` also forces the fraction, so `2 MiB` renders `2.00 MiB`, unless a format spec overrides
it. `pin` rescales losslessly. Forced `1 Bytes` is not expressible by design.

`Display` on `ByteSize` is `repr(Mode::Default)`: binary, auto-prefixed, two decimals.

## Layout

No `mod.rs`. Four flat `<module>.rs`. All 68 tests live in inline `#[cfg(test)] mod tests` blocks
(`bytesize.rs`, `unit.rs`, `prefix.rs`); `numeric.rs` has none. That contradicts the sibling-file rule and
is the largest hygiene debt in the crate. It is on the backlog, not a drive-by.

`#![warn(missing_docs)]` is set at the root. Doctests are bare triple-backtick blocks; every public
refiner carries one. CI runs `cargo test --doc` across the matrix, so a doctest that only passes on one
backend fails the gate. The `f!` / `exec!` test macros exist so assertions run on both stores.

`wildcard_imports` is deliberately **not** denied here. The crate globs its own enum variants
(`use UnitPrefix::*`) on purpose, and `Cargo.toml` says so in a comment. That is the one deliberate
deviation from the contract, and it is why the deny must not be copied in blindly.

## Things you will get wrong if nobody tells you

- **`UnitPrefix` interleaves decimal and binary from discriminant zero.** Kilo, Kibi, Mega, Mebi, in
  order. `is_decimal` is a bit test, `index` is a shift, and a compile-time assertion pins the
  interleaving (`prefix.rs`). Reorder the enum and the build fails, which is the point.
- **`k` is Kilo.** The SI symbol is lowercase `kB`, never `KB`. The parser rejects wrong-case short forms
  under the default feature set, case-insensitively always for long forms, and `KI`/`MI` in both modes.
- **Short `b` versus `B` is exact.** Only the single letters are case-sensitive; `"Bit"`, `"Bits"` and
  `"gigabyte"` all parse.
- **`FromStr` validates thousands grouping on the whole part only**, then parses. Construction truncates
  through `of`, so a sub-byte remainder rounds down.
- **`Unit::Ord` is effective value in bits.** `KILO_BIT < KIBI_BIT < KILO_BYTE`. Do not re-derive it.
- **Golden matrices are the proof.** `unit_format_and_display_symbol` is a 34-row table; the sortedness
  tests pin both orderings; `format_then_parse_round_trips` pins the parse half. Extend the table, do not
  add a fifteenth assertion beside it.

## Features

`default = ["u128", "lossless", "no-panic"]`. `bits` stores the count in bits instead of bytes.
`case-insensitive` relaxes prefix and unit case. `lossless` pulls `fraction`, which leaks into the public
API through the `Float` alias; that leak is the price of exactness and is deliberate.

`rust-version = "1.85"` in `Cargo.toml` but the toolchain is pinned to `1.97.1`. Known inconsistency,
same as the siblings.

## Docs

`README.md` is the contract with the user: quick start, formatting control, the two fluent-API tables,
the feature table, and the named `.with(...)` escape hatch. The tables and the code agree today; keep
them agreeing. `assert_matches!` needs `use std::assert_matches;` on the pinned toolchain (stable since
1.96.0, not in the 2024 prelude). No tape, no GIFs, no demos directory: this crate is proven by the
matrix, not by pictures.
