# xbytes

Convert between raw byte counts and human-readable sizes, in both directions.

[![Crates.io](https://img.shields.io/crates/v/xbytes?label=latest)](https://crates.io/crates/xbytes)
[![Documentation](https://docs.rs/xbytes/badge.svg)](https://docs.rs/xbytes)
[![MIT or Apache 2.0 Licensed](https://img.shields.io/crates/l/xbytes.svg)](#license)
[![Dependency Status](https://deps.rs/crate/xbytes/0.1.1/status.svg)](https://deps.rs/crate/xbytes/0.1.1)

xbytes turns a raw byte count into a human-readable size, and parses one back,
with fine control over how the size is written:

- **Exact-fraction arithmetic.** Conversions run through exact rationals, not
  `f64`, so repeated math never drifts and the digits are exact: `1.5 MiB` at 20
  decimals is `1.50000000000000000000 MiB`, not `1.49999999...`.
- **A fully-typed `Unit`.** Prefix (SI or IEC) and variant (bit or byte) are
  real enum-backed values, not strings. `KiB` and `KB` are distinct, comparable,
  round-trippable units.
- **A broad formatting vocabulary.** Thousands separators, arbitrary precision,
  long unit words, pluralization and capitalization control, bits or bytes, SI or
  IEC, custom spacing, all composable.

## Quick start

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
assert_eq!(a, b);                                     // exact equality, no rounding
assert_eq!(a.byte_count_lossy(), 1024 * 1024 * 1024); // pull the raw count back out

// plain bytes have no prefix
assert_eq!(ByteSize::of(512, BYTE).to_string(), "512 B");
```

## Formatting control

The same size, rendered several ways:

```rust
use xbytes::prelude::*;

let size = ByteSize::of(58375, MEBI_BYTE);

// pin a unit and group the thousands
assert_eq!(size.iec().pin(MEBI_BYTE).thousands().to_string(), "58,375 MiB");

// any separator you like, including a multi-byte glyph, sliced on char boundaries
assert_eq!(size.iec().pin(MEBI_BYTE).separator("_").to_string(), "58_375 MiB");

// spell it out, first-cap house style
assert_eq!(
    ByteSize::of(2, KILO_BYTE).si().long().with(Format::NoMultiCaps).to_string(),
    "2 Kilobytes",
);
```

High precision stays exact: 20 decimals give true trailing zeros
(`1.50000000000000000000 MiB`), because the value is an exact fraction rather
than an `f64`. Binary and decimal boundaries stay distinct: `1000 B` stays
`1000 B` in binary mode and becomes `1 KB` in decimal.

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
