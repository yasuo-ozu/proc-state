# proc-state crate [![Latest Version]][crates.io] [![Documentation]][docs.rs] [![GitHub Actions]][actions]

[Latest Version]: https://img.shields.io/crates/v/proc-state.svg
[crates.io]: https://crates.io/crates/proc-state
[Documentation]: https://img.shields.io/docsrs/proc-state
[docs.rs]: https://docs.rs/proc-state/latest/proc-state/
[GitHub Actions]: https://github.com/yasuo-ozu/proc-state/actions/workflows/rust.yml/badge.svg
[actions]: https://github.com/yasuo-ozu/proc-state/actions/workflows/rust.yml


## Overview

This Rust library solves a fundamental limitation in Rust's procedural macros where state cannot be shared across macro invocations. Each time a macro is called, it runs in a new, isolated environment, making it impossible to maintain state between calls.

To overcome this, the library introduces the [`Global<T>`] type, where `T` must implement Send and Sync. Additionally, it provides the [`new!()`] macro, which can be invoked in a const context, enabling global state to persist across multiple macro invocations.

By using this library, you can easily manage persistent global state across procedural macro invocations in a safe and efficient manner.

## Caution

This is an experimental and tricky library, which depends deeply on detailed rustc implementations. It may be broken at any time. This macro is intended to be deal with your proc-macro code, which means it only generate codes and it does not affect of other ways. So the interface is defined as safe for now. Use it on your own responsibility.
