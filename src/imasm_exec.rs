// ─── imasm_exec.rs ──────────────────────────────────────────────────────────
// The payload-carrying x86-64 lift and its interpreter, both already built and
// proven end to end in vox_core (`vox run <symbol> --args a,b <file>`). This
// file only re-exports them, matching the pattern `vox.rs` and `vox_decode.rs`
// already use for the rest of the crate.
pub use vox_core::imasm_module::{emit, words};
pub use vox_core::imasm_vm::{Machine, Stop};
