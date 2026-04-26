//! Code generation internals.
//!
//! The codegen stage converts the optimized DFA into Rust source by:
//! - compressing byte transitions into equivalence classes,
//! - materializing compact transition and acceptance tables, and
//! - emitting a runtime iterator that performs maximal-munch tokenization.

mod compressor;
mod emitter;

pub(crate) use emitter::RustEmitter;
