# xbytes

The most complete and most correct byte-size formatter in Rust.

[![Crates.io](https://img.shields.io/crates/v/xbytes?label=latest)](https://crates.io/crates/xbytes)
[![Documentation](https://docs.rs/xbytes/badge.svg)](https://docs.rs/xbytes)
[![MIT or Apache 2.0 Licensed](https://img.shields.io/crates/l/xbytes.svg)](#license)
[![Dependency Status](https://deps.rs/crate/xbytes/0.1.1/status.svg)](https://deps.rs/crate/xbytes/0.1.1)

xbytes turns a raw byte count into a human-readable size, and parses one back,
with a level of control no other crate in the ecosystem offers:

- **Exact-fraction arithmetic.** Conversions run through exact rationals, not
  `f64`, so repeated math never drifts and high precision never surfaces float
  noise: `1.5 MiB` at 20 decimals is `1.50000000000000000000 MiB`, not
  `1.49999999...`.
- **A fully-typed `Unit`.** Prefix (SI or IEC) and variant (bit or byte) are
  real enum-backed values, not stringly-typed guesses. `KiB` and `KB` are
  distinct, comparable, round-trippable units.
- **The widest formatting vocabulary anywhere.** Thousands separators, arbitrary
  precision, long unit words, pluralization and capitalization control, bits or
  bytes, SI or IEC, custom spacing, all composable, none reachable from any
  competitor.

## 30-second teach

```toml
[dependencies]
xbytes = "0.1"
```

Build a `ByteSize` from a number and a unit, then reach for the fluent
formatter. `iec()` / `si()` choose the number system; the refiners chain:

```rust
use xbytes::prelude::*;

let size = ByteSize::of(1536, KIBI_BYTE);

assert_eq!(size.to_string(), "1.50 MiB");            // Display default: IEC
assert_eq!(size.iec().to_string(), "1.50 MiB");      // binary, auto-prefixed
assert_eq!(size.si().to_string(), "1.57 MB");        // decimal, auto-prefixed
assert_eq!(size.bits().to_string(), "12 Mib");       // denominated in bits

// refiners chain, in any order
assert_eq!(size.iec().long().precision(3).to_string(), "1.500 MebiBytes");
assert_eq!(size.si().symbol().to_string(), "1.57 MB");
```

Parse the other direction with `str::parse`, thousands separators and all:

```rust
use xbytes::ByteSize;

let a: ByteSize = "1,024 MiB".parse().unwrap();
let b: ByteSize = "1 GiB".parse().unwrap();
assert_eq!(a, b); // exact equality, no rounding
```

## Why xbytes

Here is the same size rendered every way its peers can, and several ways only
xbytes can:

```rust
use xbytes::prelude::*;

let size = ByteSize::of(58375, MEBI_BYTE);

// pin a unit and group the thousands (no other crate does this at all)
assert_eq!(size.iec().pin(MEBI_BYTE).thousands().to_string(), "58,375 MiB");

// any separator you like, including a multi-byte glyph, sliced on char boundaries
assert_eq!(size.iec().pin(MEBI_BYTE).separator("_").to_string(), "58_375 MiB");

// spell it out, first-cap house style
assert_eq!(
    ByteSize::of(2, KILO_BYTE).si().long().with(Format::NoMultiCaps).to_string(),
    "2 Kilobytes",
);
```

### Head to head

| capability | **xbytes** | `bytesize` | `humansize` | `byte-unit` |
|---|:--:|:--:|:--:|:--:|
| binary (IEC) and decimal (SI) | yes | yes | yes | yes |
| bits denomination | yes | yes | no | yes |
| pin a fixed unit | yes | no | yes | yes |
| arbitrary precision | yes | no (1 dp) | yes | yes |
| force / trim trailing fraction | yes | no | yes | partial |
| long unit words (`MebiBytes`) | yes | no | yes | no |
| pluralization control | **yes** | no | no | no |
| capitalization control | **yes** | no | no | no |
| thousands separators | **yes** | no | no | no |
| exact-fraction (drift-free) values | **yes** | no | no | no |

xbytes is a strict superset of every competitor's formatting vocabulary, plus
three capabilities no one else has: thousands separators, a plural/caps engine,
and exact-fraction rendering.

The four cases where that matters, side by side:

1. **Thousands-grouped, fixed-unit output.** `58,375 MiB` with a configurable
   separator. `bytesize`, `humansize`, and `byte-unit` cannot emit a separator
   at all.
2. **High precision with no float noise.** `size.iec().precision(20)` yields
   `1.50000000000000000000 MiB` with true trailing zeros, because the value is
   an exact fraction. The float-backed crates surface rounding artefacts here;
   `bytesize` has no precision knob at all.
3. **Full lexical control in one place.** `1.50 MiB`, `1.50 MB`,
   `1.50 MebiBytes`, `1.50 Mebibytes`, `1.50 M`, or `1.50MiB` are each one
   method call apart. No string post-processing.
4. **Correct exact boundaries.** `1000 B` in binary mode stays `1000 B` (it does
   not wrongly promote to `~0.98 KiB`); `1000 B` in decimal mode is `1 KB`. The
   IEC/SI split is respected exactly.

## The fluent API

Every refiner returns a configured value that implements `Display`, so they
chain in any order.

Mode entries, on `ByteSize`:

| method | renders |
|---|---|
| `.iec()` | binary (power-of-1024), auto-prefixed: `1.50 MiB` |
| `.si()` | decimal (power-of-1000), auto-prefixed: `1.57 MB` |
| `.bits()` | denominated in bits: `12 Mib` |
| `.repr_as(unit)` | pinned to one explicit unit |

Refiners, on the returned value:

| method | effect |
|---|---|
| `.long()` | spell the unit out: `MebiBytes` |
| `.condensed()` | prefix initial only: `M` |
| `.symbol()` | drop the IEC `i`: `MiB` becomes `MB` |
| `.precision(n)` | `n` fractional digits, shown even for whole values |
| `.thousands()` | group the whole part with commas |
| `.separator(s)` | group with a custom separator |
| `.pin(unit)` | re-render at one explicit unit, losslessly |
| `.plural(bool)` | pin the long unit plural or singular |
| `.no_space()` | drop the number-to-unit space |
| `.lower()` | lowercase the unit |

Power users can drop to the `Format` bitflags and `ReprConfigVariant` knobs via
`.with(...)` for anything the refiners do not name (`ForceFraction`,
`NoFraction`, `UpperCaps`, `Spaces(n)`, ...). The refiners are the primary,
documented path; the flags are the engine underneath.

## Feature flags

| flag | default | effect |
|---|---|---|
| `u128` | on | back the count with `u128` (unlocks zetta/yotta) instead of `u64` |
| `lossless` | on | compute in exact fractions instead of `f64`, so conversions do not drift |
| `no-panic` | on | saturate arithmetic on overflow (implies `lossless`) |
| `bits` | off | store the count in bits rather than bytes (flips a few return types, see the docs) |
| `case-insensitive` | off | accept units in any case (`kib`, `MB`, `gIb`) |

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
