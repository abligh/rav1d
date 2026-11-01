//! C types that [`libc`] defines only on targets with a C library.
//!
//! `wasm32-unknown-unknown` has no C library, so [`libc`] defines none of
//! these there. Taking them from here lets the crate build for it.

#![allow(non_camel_case_types)]

/// C's `ptrdiff_t`, which is [`isize`] on every target Rust supports.
pub type ptrdiff_t = isize;

/// C's `intptr_t`, which is [`isize`] on every target Rust supports.
pub type intptr_t = isize;

/// C's `uintptr_t`, which is [`usize`] on every target Rust supports.
pub type uintptr_t = usize;

/// C's `off_t`.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub type off_t = libc::off_t;

/// C's `off_t`, which a target with no C library lacks: 64 bits, as
/// `wasm32-wasip1` has it.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub type off_t = i64;
