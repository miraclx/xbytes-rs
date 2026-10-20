# STYLE.md

_How the code is built. This file is the style contract and is identical in every crate of this workspace that carries it, so it is written for a reader who has seen none of them. A reviewer, human or machine, does not raise a pattern that is written here. A pattern that is not written here is not a rule yet: when review raises the same thing twice, it is added here._

## Philosophy
- **Correctness first, cleverness last.** Choose the plainest construct that is correct. Restraint is what review rewards.
- **Parse, don't validate.** Turn unstructured input into a typed value once, at the edge; everything downstream receives values that are already valid.
- **Make invalid states unrepresentable.** Put the invariant in the type system so a mistake is a compile error. Assume a mistake will be made, and make it fail to compile.
- **Every abstraction earns its place,** and a caller can trust it without reading its body.
- **Performance is designed in from the first line,** not measured in and attached at the end.

## Types & domain modeling
- **Newtype every identifier and every scalar that carries an invariant.** `ChunkId(u32)`, `ByteCount(u64)`. Never a raw `u32`/`u64`/`String` where two values of the same Rust type are not interchangeable. This removes argument-swap mistakes.
- **Prefer an enum to a `bool` or a stringly-typed field,** matched exhaustively at every decision point, so a new variant is a compile error everywhere it has to be handled.
- **Name a field when the name carries information, and destructure rather than count.** `&self.0` is fine on a one-line accessor. In denser code a bare `.0`/`.1` costs the reader a count, so destructure with a name: `let Self(value) = self`. Keep a newtype a tuple struct unless a named field says something (`Segment { style, color }`); `Name(String)` as reflexive wrapping renames nothing.
- **Never carry in a float a quantity that has to be exact.** Pick the representation where the arithmetic is exact: integer minor units for money, so 450 is four-fifty and not 4.499999999999999; exact rationals for a byte count that must survive repeated scaling (`fraction::GenericFraction<u64>`). State the rounding rule once, put the unit in the type name or the field name, and handle overflow with explicit checked or saturating methods.
- **`::new` only when construction has logic** that must run to produce a valid `Self`. A `new` that assigns the fields one for one is a struct literal or a `From` impl.
- **Encode a constraint in the type** wherever a runtime `if` would otherwise enforce it, and use `const { assert!(..) }` for a relationship known at build time.
- **`#[must_use]` where dropping the return is a bug** (a builder, a freshly built value, a result that must be acted on), and not everywhere else. Paired with `unused_must_use = "deny"`.
- **In a trait impl, write the associated type in the signature** (`fn from_fixture(value: Self::Row)`), not the concrete type, so changing the associated type propagates to every signature.

## Three representations: external, internal, stored
- **An external type is not an internal type is not a stored type.** Keep three distinct shapes and convert explicitly at each boundary.
  - **External**, what a foreign library hands you and the shape a file or a network message arrives in: `reqwest::Response`, `serde_json::Value`, a parsed config file. Optionality, unknown fields and format churn belong here.
  - **Internal**, plain Rust with no derive from any external library, no serde, no I/O. It answers to correctness only.
  - **Stored**, what a file on disk holds, with its own encoding and its own version tag, owned by the code that writes it.
- **Do not put two concerns' derives on one struct.** A type carrying a serde derive and a storage derive couples the file format to the transport and lets each one's constraints leak into the other. The conversions live at the boundary as `From`/`TryFrom` or as a named function, never as derives on the internal type.
- **A family of the same conversion is a local trait, not a bag of free functions.** When several functions convert the same way (a foreign struct to an internal value, a foreign error to yours), group them under one small local trait with an associated type naming the counterpart. The trait names the relationship, keeps the impls in one place, and, when both types are foreign to your crate, is the only way past the orphan rule.

## Ownership, memory & performance
- **No clone or allocation that is not needed.** Borrow; stream; iterate instead of collecting.
- **Make a clone visible at the callsite:** `T::clone(&x)`, not `x.clone()`. A clone is a deliberate cost and should read as one.
- **Do not take a reference to a `Copy` type.** Take `u64`, not `&u64`.
- **Take the least committal type that works; return the most useful one.** Accept `impl AsRef<str>`, `&str` or `impl IntoIterator`; return `impl Iterator` rather than a `Vec` unless you already hold a `Vec`. Preallocating for a caller that streams is waste.
- **Batch async work** instead of issuing one call per item.
- **Prefer a channel to shared state, and take a lock only where shared state demands one.** `Mutex` and `RwLock` carry runtime ownership tracking and invite deadlock, so every lock carries a comment naming why it is there.

## Error handling
- **Libraries: `thiserror`.** Typed, enumerated, matchable kinds. The error type is part of the API.
- **Binaries: `eyre::Result`,** always path-qualified (`eyre::Result<T>`) and never imported as `Result`, so it stays visibly distinct from `core::result::Result`. `thiserror` does not capture a backtrace and does not attach context; `eyre` does. A binary may also name a foreign error type that is not a `std::error::Error`, `anyhow::Error` from a subprocess wrapper being the case here, for exactly one job: mapping it into its own typed error at the seam.
- **Carry the cause by reference through the source chain** (`std::error::Error::source`), the foreign error inside your variant, as with `ParseIntError`. Do not stringify the cause away.
- **Error messages are lowercase and carry no trailing punctuation,** because they compose into larger chains: `invalid byte range`, not `Invalid byte range.`
- **No `.unwrap()` or `.expect()` outside tests,** enforced by `clippy::unwrap_used` and `clippy::expect_used` at deny. Panic only where an invariant makes the branch unreachable, with a message naming the invariant.
- **Never panic on input that came from outside the program.** Return an error.
- **At a protocol boundary, map the typed error to that protocol's own code,** never to one generic internal code. HTTP: `412` when the resource changed under you, `409` on a conflicting concurrent write, `429` when a limit is reached. gRPC: `FailedPrecondition`, `Aborted`, `ResourceExhausted`.

## Control flow
- **Guard clause and early return.** Handle the error or the edge case first; keep the main path un-nested.
- **`let ... else`** when the else branch does not use the bindings; `if let` when it does.
- **Order a short-circuit chain cheapest and most likely to decide first.**
- **`match`, `if let` or `?` over `is_some()` followed by `unwrap()`.** Use the value the condition already proved is there.
- **No `bool` parameter that changes behavior.** Split the function or take an enum.

## Command-line interfaces
- **A CLI is a module tree.** `main` parses and dispatches; each command group is a module and each command its own file (`download.rs`, then `download/audio.rs`), and every command owns an `async fn run(self, ..)` that consumes it. Dispatch is a `match` that calls `run`. A binary with one command and no subcommands is the exception and may stay one file; say so where the binary is declared.
- **Let the parser produce the typed value.** A field is the domain type (`Duration`, `Checksum`, `ByteCount`), not a `String` the handler re-parses: parsing happens once, through the parser's value parser or the type's `FromStr`, so `run` receives values that are already valid. Two exceptions, each named where it appears: a value that cannot be resolved without I/O, a URL that decides which backend to use, stays text on the argument struct and is converted once in one named function; and a flag that must interoperate with an external tool keeps that tool's spelling, because the format is the interface.
- **Give a command a short alias** (`list` for `ls`) through the parser's own alias support, when the command has one.

## Concurrency & persistence
- **Put the guard next to the state it protects.** The type that owns the shared state owns the `Mutex`, `RwLock` or atomic, so no callsite can forget it. A callsite that locks is a sign the lock is in the wrong place.
- **Name each primitive and the reason for it,** in a comment where it is taken: a `fetch_add` on a shared counter, one `Mutex` inside a renderer, a version field compared before a write. If a `tokio::sync::Mutex` is deliberately held across an `await`, say so there.
- **Keep the guarded region small,** and hold no guard across an external call whose duration you do not control, unless the comment says why.
- **State a later run reads back is versioned and written atomically:** write a temporary, rename it into place, tag it so a future format change is detectable. Validate it against the current resource before its bytes are reused: if the resource carries a validator and the stored one disagrees, discard the partial and start again. Never splice new bytes onto a resource that changed.

## Comments & docs
- **Why, not what.** The code already says what it does. A comment that restates the next line will drift out of date, so delete it.
- **`///` on every public item and on every enforced invariant,** saying why the invariant exists at the point it is enforced.
- **No em dashes** in prose anywhere: comments, docs, README, commit messages, pull request text.

## Code layout & readability
- **Read downward.** Put the high-level item first and the helpers it calls below it, so a reader following a detail keeps reading down and never scrolls back up to assemble context.
- **No `mod.rs`.** `<module>.rs`, with submodules in a sibling `<module>/` directory.
- **A layer touches only its own concern, and the layering runs one way:** pure computation, then the type that owns I/O, then the binary that wires them together. No layer reaches back up. The layer that fetches bytes names no transport: a `Source` is `probe` and `fetch` over a byte range and a byte stream. The layer that moves bytes names no file format; where it must hand bytes to something durable, a scratch file a later run resumes from, that choice is made once in one named function, so no other part of the layer builds a path.
- **Imports follow the `StdExternalCrate` group order,** blank-line separated, in this sequence. `rustfmt.toml` sets `group_imports = "StdExternalCrate"` and `imports_granularity = "Module"`, both nightly-only options:
  1. Standard library (`std`, or `core` where the item is in `core`)
  2. External crates
  3. This crate's items (`crate::`, `super::`)
  4. `mod ...;` declarations
  5. Items from those modules, referenced bare (`use rows::Row;`), never `use self::rows::Row;`
- **One `use` per module path.** Allowed: `use core::future::{pending, Future};`, `use core::pin::Pin;`, `use std::time::Duration;`. Not allowed: `use core::{future::{pending, Future}, pin::Pin};`, `use std::{thread, time::Duration};`
- **Qualify one level where it reads better than the leaf:** `use std::io;` then `io::Error`; `use tokio::sync::{mpsc, oneshot};` then `mpsc::channel`.
- **Path-qualify a derive macro from its crate:** `#[derive(thiserror::Error, Debug)]`, never a bare `Error` brought in by a `use`.
- **No wildcard import** (`use foo::*`). Name every item, so a reader knows where it came from. `clippy::wildcard_imports` is deny; a crate that must glob its own enum variants sets the lint aside in `Cargo.toml` and says why in a comment.
- **`cargo sort --workspace --check`** keeps `Cargo.toml` dependencies ordered.

## Tests
- **Test the pure part first,** because a pure function needs no process, no socket and no temporary directory to be exercised.
- **Unit tests live in a sibling `<module>_tests.rs`,** not inline in the module. A library target declares them as `mod foo_tests;`; a binary target needs `#[path = "foo_tests.rs"] mod foo_tests;` to reach a private item. Integration tests live in `tests/`, and use an in-process fake or mock server and a temporary directory rather than a live network.
- **Cover zero, one, many and the error** for every behavior.
- **Assert on the invariant, not on the shape.** For an enum or an error variant use `assert!(matches!(..), "expected {got:?}")`, or `use std::assert_matches;` and `assert_matches!(..)` with a message; both are stable on the pinned toolchain, `assert_matches` having stabilised in 1.96.0. Use `const { assert!(..) }` for a relationship between constants.
- **Test concurrency with a barrier,** releasing N tasks at once and asserting the invariant holds afterwards, rather than asserting that it would.
- **Name the test for what it asserts,** as a sentence: `a_source_that_ignores_the_range_is_a_typed_error`. Share setup through builders, not by copying it.

## Observability
- **`tracing`, with a span per operation,** carrying the identifier that ties that operation's events together, a chunk index, a job id or a request id.
- **Structured fields, never string formatting:** `tracing::warn!(chunk = index, attempt, resume_from = offset, error = %error, "chunk failed")`, not `tracing::warn!("chunk {} failed on attempt {}", index, attempt)`.

## Serialization
- **A JSON enum that needs a discriminator gets an explicit `kind` field** holding the variant name in `PascalCase`, `{"kind": "Range", "start": 0, "end": 10}`, not the externally tagged `{"Range": {"start": 0, "end": 10}}`. The explicit form leaves room for fields the variants share; the tagged form does not.

## Security
- **A secret does not outlive its use.** A `String` or `Vec<u8>` holding a token stays readable in freed memory until it is overwritten, so put it in a type that zeroes on drop (`zeroize::Zeroizing`, `secrecy::SecretString`) or hand it straight to the foreign call that needs it and keep nothing.

## Tooling & dependencies
- **Before every commit, all four of these are clean. This is the definition of done:** `cargo +nightly fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `cargo sort --workspace --check`. The `+nightly` is required: `rustfmt.toml` sets two nightly-only options, and a stable `cargo fmt` warns that it cannot set them and then checks nothing they govern. `unwrap_used`, `expect_used`, `wildcard_imports` and `unused_must_use` are deny; `std_instead_of_core` is warn; CI sets `RUSTFLAGS: -D warnings`, so there a warning is a failure.
- **Prefer `core::` to `std::`** for anything that exists in `core`. Do not route through `alloc::`: if the item is not in `core`, use `std::`. `clippy::std_instead_of_core` reports this. Where its suggestion is itself unstable, as with `core::io::ErrorKind` (rust#154046), scope an `#[allow]` with a comment rather than following it.
- **Use an explicitly sized integer** (`u64`, `u32`) wherever the width is part of the meaning. `usize` is for an index into a collection.
- **Define a macro only with a reason that survives being written down.**
- **Add a dependency only when the standard library and the crates already present cannot do the job,** and name the gap in a comment where it is added.

## Commits & PRs
- **One commit is one step, and it builds and passes on its own.** Its message says what it does and what it makes possible next.
- **A pull request body has these headings:** Motivation / What this changes / How it was tested / Compatibility / Follow-ups. State the invariants the change relies on, so that review does not have to ask for them.
- **Keep the diff small.** An "and also" section means it was two pull requests.

## House rules
_Each of these was raised in review. A rule goes here when review raises the same thing twice._

- **Bind an operation to a type; a free function is a last resort.** Most operations belong on a type,
  composable and reusable. Model an operation as a value you build and then consume
  (`ByteSize::of(value, Unit::mib())`, `Output::tee(file, writer)`,
  `Bar::new(..).with_style(..).with_target(..)`), and let a one-shot operation take `self` by value.
  Reserve a bare free function for a genuinely standalone pure helper, and even then prefer a local
  trait for a family of conversions (see the three-representations section).
- **A one-shot private pipeline may stay free functions.** When several stages run once in sequence,
  thread the same few values through them, and they never leave the one task, so their shared state is
  `Rc` or `Cell` rather than a reusable object, then leaving them as free functions is right. Moving
  them onto a `Self` whose `&self` holds those same threaded values moves parameters into fields and
  buys no composability. This is a narrow exception, not permission to scatter helpers.
- **Name a foreign implementation once, at the composition root; everything after that speaks your
  own type.** A binary names the concrete backend, a specific HTTP client, an object-store SDK or a
  subprocess, exactly once, where it builds its root value. Every operation after that is generic over
  your own trait. A file outside `main` that imports the backend crate is the leak.
- **`tokio::io` is imported one level qualified:** `use tokio::io;` then `io::AsyncRead`, `io::copy`,
  `io::duplex`, `io::split`, `io::WriteHalf`. An extension trait whose methods you call but whose names
  you never write comes in anonymously: `use tokio::io::AsyncReadExt as _;`.
