# STYLE.md

_How this codebase is built. It is the contract: reviewers (human or machine) should never spend a comment on a pattern documented here. All in all, it should be **beautiful to read** and tell a story as you scroll._

## Philosophy
- **Correctness first, cleverness never.** The boring right tool beats the impressive wrong one. Restraint is the senior signal.
- **Parse, don't validate.** Turn unstructured input into a strictly-typed instance once, at the edge; the rest of the code receives things that are already valid by construction.
- **Make illegal states unrepresentable.** Push invariants into the type system so the compiler catches mistakes and guides future development. Assume human error is inevitable; the software should guide the next person away from the cliff.
- **Every abstraction earns its keep,** and a reader must be able to offload context onto it and trust it without reading its insides.
- **Performance is a first-class concern from line one,** never an afterthought bolted on later.

## Types & domain modeling
- **Newtype every ID and domain scalar.** `UserId(Uuid)`, `Money`, `Quantity`. Never raw `Uuid`/`i64`/`String` across a boundary. Kills primitive obsession and argument-swap bugs.
- **Prefer enums to bools.** State is an `enum` with exhaustive `match`, not a `bool` or a stringly-typed field. A new variant then forces a compile error at every decision site.
- **Positional field access reads fine when trivial, murky when dense.** A one-line accessor (`&self.0`) is perfectly clear. But in denser logic a bare `.0`/`.1` is as opaque as an unlabeled eighth function argument, so destructure with a *name* there (`let Self(value) = self`) and let the reader stop counting positions. Keep newtypes as tuple structs; reach for a named field only when the name carries real information (`Money { minor_units }`), not as reflexive wrapping (`Name(String)`). The same instinct prefers named or struct arguments over long positional parameter lists.
- **`Money`:** integer minor units inside, **never floating point**, explicit currency, one documented rounding rule, overflow handled by explicit checked/saturating methods.
- **`::new` only when construction has real logic** that must run to produce a valid `Self`. If it just assigns the fields 1:1, use a struct literal or `Default`/`From` instead. A `new` that adds nothing is noise.
- **Encode constraints in the type system** wherever a conditional or a validation would otherwise live at runtime. **Compile-time assertions** (`const { assert!(..) }`) for invariants knowable at build time.
- **`#[must_use]` where ignoring a return is a bug** (builders, freshly-constructed domain values, a result that must be acted on), only where relevant, not reflexively. Complemented by `unused_must_use = deny`.
- **In trait impls, name associated types via `Self::Assoc` in signatures**, not the concrete type: `fn from_row(value: Self::Row)`, not `fn from_row(value: i64)`. The associated type stays the single source of truth, so changing it propagates to every signature.

## Three representations: wire, domain, storage
- **A domain type is not a wire type is not a DB row.** Keep three distinct representations and map explicitly between them at the boundaries.
  - **Wire**, the codec-generated types (transport concern: field numbers, optionality, serialization quirks).
  - **Domain**, pure Rust, the currency of the business logic. Carries **no codec derives at all**, no ORM/row derives, no serde. It answers to correctness, not to any transport or storage format.
  - **Storage**, row structs owned by the repo layer, carrying the persistence derives.
- **Do not compound codec derives onto one struct.** A single type wearing both a row-mapping derive and the wire/serde derives couples persistence and transport into the domain and lets each layer's constraints leak into the others. Conversions live at the boundary (`From`/`TryFrom` in the repo and the service layer), never as derives on the domain type.
- **A cohesive class of conversions is a local trait, not a bag of free functions.** When several functions perform the same kind of transformation (domain to wire, row to domain, primitive to scalar), group them under one small local trait with an associated type naming the counterpart, rather than scattering `to_x` / `from_x` free functions. The trait names the relationship, keeps the impls discoverable together, and, when the two types live in different crates, sidesteps the orphan rule that forbids a foreign `From` / `TryFrom` (e.g. a `FromRow` trait for storage, a `ToWire`/`FromWire` pair for the transport interface).

## Ownership, memory & performance
- **No unnecessary clones or allocations.** Prefer borrowing; prefer streaming and iteration over materializing collections.
- **Make clones explicit and greppable:** `T::clone(&x)`, not `x.clone()`. A clone should be visible at the callsite as a deliberate cost.
- **Don't take references to `Copy` types.** Take `u64`, not `&u64`.
- **Receive minimally, expose maximally.** Accept the least-committal type that works (`impl AsRef<str>`, `&str`, `impl IntoIterator`); return the most useful/maximal one. **Return `impl Iterator` over a `Vec`** unless you already hold a `Vec`, let the consumer decide whether to allocate; preallocating for a consumer that streams is waste.
- **Amortize async operations by batching** rather than issuing them one at a time.
- **Share memory by communicating** (channels), don't communicate by sharing memory. **Be wary of `Mutex`/`RwLock`**, they carry ownership-tracking overhead and invite deadlock; lean on the type system and message-passing first.

## Error handling
- **Libraries: `thiserror`.** Typed, enumerated, matchable error kinds, errors are part of the API.
- **Binaries: `eyre::Result`** (thiserror loses backtrace information, so it stays out of bins). Use it **path-qualified** (`eyre::Result<T>`); never `use eyre::Result`, keep it visibly distinct from `core::result::Result`.
- **The underlying error kind is contained and returned by reference** via the source chain (`std::error::Error::source`), `ParseIntError` inside your variant is the textbook shape. Don't stringify away the cause.
- **Error messages are lowercase and carry no trailing punctuation**, they *will* compose into larger chains, so `invalid quantity` not `Invalid quantity.`
- **No `.unwrap()` / `.expect()` in non-test code** (clippy-denied). Panic only on a genuinely unreachable invariant, with a message saying why it's unreachable.
- **Never panic on bad input**, return an error.
- **At a transport boundary**, map typed errors to precise status codes for that protocol (e.g. `FailedPrecondition`, `Aborted`, `ResourceExhausted`), never a blanket `Internal`.

## Control flow
- **Guard clauses and early returns.** Handle the edge/error first; keep the happy path un-nested.
- **`let ... else`** when the else branch is independent of the conditional's bindings; `if let` when it needs them.
- **Short-circuit** conditionals; order predicates cheapest/most-likely-to-decide first.
- **`match` / `if let` / `?` over `is_some()` + `unwrap()`.** Use the value you just proved exists.
- **No boolean parameters** that flip behavior, split the function or take an enum.

## Command-line interfaces
- **A CLI is a module tree, not one file.** `main` parses and dispatches; each command group is a module and each leaf command its own file (`user.rs`, then `user/create.rs`), and every command owns an `async fn run(self, ...)` that consumes it. Dispatch is a `match` that delegates to the command's `run`.
- **Let the argument parser hand you already-valid domain instances.** A command field is a domain type (a `UserId`, a `Price`, a `Currency`), never a `String` the handler re-parses. Parsing happens once, at the parser boundary (clap's value parser via the type's `FromStr`), so `run` receives domain values: the strictly-typed core is the whole point. Never leak an internal representation into a flag; `--price 4.50` parses to a `Price`, not `--price-minor 450`.
- **Give commands short aliases** (`list`/`ls`, `create`/`new`) through the parser's alias support.

## Concurrency & persistence
- **All correctness-critical coordination lives in the database**, not in app-memory locks, so the service tier is stateless and horizontally scalable.
- **Typed repository trait per aggregate** (`UserRepo`, `OrderRepo`, `PaymentRepo`); **the query layer is fully contained behind them.** The rest of the codebase sees only domain types. A local/embedded store follows the same shape with its own trait.
- **Transactions are explicit and short**; no network call is held open inside a DB transaction.
- **Concurrency primitives are named and intentional** and each gets a why-comment: guarded atomic `UPDATE ... WHERE qty >= n`, `UNIQUE` for idempotency, optimistic `version` for concurrent edits.

## Comments & docs
- **Why, not what.** Code is the most reliable description of the logic; a comment that merely restates the next line is unconstrained, will drift, and becomes a lie. Delete it.
- **`///` on every public item and every enforced invariant.** Document *why* the invariant exists at the point it's enforced.
- **No em dashes** anywhere in prose (comments, docs, README, commit messages, PR text).

## Code layout & readability
- **Top-down story.** Lay items out so meaning is discovered reading downward: a high-level item on line 1 references helpers defined below it, so a reader chasing a detail reads *on* until satisfied, then exits, never scrolls up to assemble context first.
- **No `mod.rs`.** Use `<module>.rs` with submodules in a sibling `<module>/` directory.
- **Clear separation of concerns:** pure domain (DB-free) < repos < service handlers (thin) < transport.
- **Imports follow the `StdExternalCrate` group order**, blank-line separated, in this exact sequence (rustfmt `group_imports = "StdExternalCrate"`):
  1. Standard library (`std` / `core` / `alloc`)
  2. External crates
  3. Symbols from the local crate and parent module (`crate::`, `super::`)
  4. Local module declarations (`mod ...;`)
  5. Symbols from those local modules (optional), referenced bare, `use rows::UserRow;`, never `use self::rows::UserRow;`. The `self::` prefix is noise; the bare path resolves to the child module already in scope.
- **Module import granularity** (rustfmt `imports_granularity = "Module"`): one `use` per module path; never collapse different submodules of a crate into one braced block.
  - Allowed: `use core::future::{pending, Future};` / `use core::pin::Pin;` / `use std::time::Duration;`
  - Not allowed: `use core::{future::{pending, Future}, pin::Pin};` / `use std::{thread, time::Duration};`
- **Qualify one level** where it reads better than pulling the leaf in: `use std::io;` → `io::Error`; `use tokio::sync::{mpsc, oneshot};` → `mpsc::channel`; `use tokio::time;` → `time::Duration`, `time::sleep`.
- **Path-qualify derive macros from crates:** `#[derive(thiserror::Error, Debug)]`, not a bare `Error` brought in by `use`.
- **Never wildcard-import** (`use foo::*`); bring every name in explicitly so a reader always knows where a symbol comes from. Enforced by `clippy::wildcard_imports = deny`.
- **`cargo sort`** keeps `Cargo.toml` dependencies ordered.

## Tests
- **Test-first for the domain layer** (it's pure, so tests are fast and DB-free).
- **Unit tests in a separate `<module>_tests.rs` file**, not inline in the module. Integration tests (real transport + database + external mocks via testcontainers) in `tests/`.
- **Cover zero / one / many / error** per behavior. Prefer **`assert_matches!`** for enum/error assertions; it is nightly-unstable, so on stable use `assert!(matches!(...))`.
- **Assert on invariants**, including **compile-time assertions** for constant relationships.
- **Concurrency is tested with a barrier** (N tasks released together) asserting the invariant holds, not hoped at.
- **Names read like documentation:** `checkout_decline_leaves_order_open_and_inventory_untouched`. DRY setup via shared builders.

## Observability
- **`tracing` with spans.** Key operations open a span carrying the correlation id (idempotency key, request id) so an operation is traceable end-to-end.
- **Structured fields, never string formatting:** `info!(order_id = %id, "checkout started")`, not `info!("checkout started for {id}")`.

## Serialization
- **Tagged enums when using JSON:** `{ "kind": "...", "data": ... }` with the variant name in **PascalCase** under `kind`, not `{ "<variant>": <data> }`.

## Security
- **`zeroize` sensitive data** so secrets don't linger in freed memory.

## Tooling & dependencies
- **`cargo fmt`, `cargo clippy -D warnings`, `cargo sort`** all clean before every commit, part of the definition of done.
- **Prefer `core::` over `std::`** wherever the item exists in `core`. Do not prefer `alloc::` over `std::`: if an item lives only in `alloc` (not `core`), just use `std::`. Enforced by `clippy::std_instead_of_core` (restriction lint, `warn`).
- **Explicitly-sized integer types over arch-specific** (`u64`/`u32`, not `usize`) wherever the width is semantic rather than a container index.
- **Macros defined sparingly.** Great power, great responsibility: only with irrefutable rationale.
- **Dependencies introduced sparingly**, every one indisputable and absolutely necessary.

## Commits & PRs (read like a story)
- **Each commit is one coherent step, builds and passes on its own,** and its message says what it did *and* what it sets up, earlier commits visibly lay the groundwork a later feature clicks into.
- **PR body uses the house template:** Motivation / In this PR / Test Plan / Backwards compatibility / Future Work. Narrate the correctness invariants so review questions are pre-answered.
- **Small, reviewable diffs.** An "and also" section means it was two PRs.

## House rules (flagged in review)
_Preferences called out during review. Add to this as more are flagged; reviewers should never have
to flag the same smell twice._

- **Bind behaviour to types; free functions are a smell.** Nearly everything should be a method on a
  type, composable and consumable. Model an operation as a type you construct and consume
  (`Digest::of(reader)`, `Encoder::new(w, r).send(header, &item, source)`); a one-shot op takes `self`
  by value. Reserve bare
  free functions for genuinely standalone pure helpers, and even then prefer a local trait for a
  cohesive family of conversions (see the wire/domain/storage section).
- **A one-shot private pipeline may stay as free functions.** When several stages run once in sequence,
  thread the same handful of values, and never escape a single task (so their shared state is `Rc`/`Cell`,
  not a reusable object), leaving them as free functions is fine. Making them methods on a `Self` whose
  `&self` only holds those same threaded values relocates parameters into fields without buying
  composability. This is a narrow exception, not license to scatter helpers.
- **Wrap a foreign stack once, at the composition root; everything downstream speaks your own vocabulary.**
  An app names a concrete external implementation (a specific transport, driver, or backend) exactly once,
  where it builds its root object. Every subsequent operation is generic over your own traits. If a file
  imports the concrete backend crate outside `main`, that is a leak.
- **Keep layer concerns pure.** A layer touches only its own concern and knows nothing of the layers around
  it: a byte-moving layer knows nothing of files, paths, filenames, temp files, or the filesystem (its
  sources and sinks are `AsyncRead`/`AsyncWrite` the caller supplies); naming and temp-then-rename are the
  application's job. Each layer touches only its own concern.
- **`tokio::io` is imported one level qualified:** `use tokio::io;` then `io::AsyncRead`, `io::copy`,
  `io::duplex`, `io::split`, `io::WriteHalf`. Extension traits whose methods you call but whose names
  you do not use come in anonymously: `use tokio::io::AsyncReadExt as _;`.
