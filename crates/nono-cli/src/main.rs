//! nono CLI binary.
//!
//! All implementation lives in the `nono_cli` library crate (see `src/lib.rs`).
//! This binary is a thin wrapper so the CLI logic can also be embedded as a
//! library.

fn main() {
    nono_cli::run();
}
