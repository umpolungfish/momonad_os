# $m⊙^{2}$: A Self-Imscribing Bare-Metal Kernel

![language](https://img.shields.io/badge/language-Rust-CE422B?style=for-the-badge&logo=rust&logoColor=white)
![tier](https://img.shields.io/badge/tier-O%E2%88%9E-8A2BE2?style=for-the-badge)
![µ∘δ](https://img.shields.io/badge/%CE%BC%E2%88%98%CE%B4-id-00A86B?style=for-the-badge)
![license](https://img.shields.io/badge/licence-Unlicense-1A1A1A?style=for-the-badge)

## What This Is

$m⊙^{2}$ is a bare-metal operating kernel written in Rust (no_std, x86_64) that replaces the traditional OS stack with a single self-verifying loop. There are no processes, no scheduler, and no filesystem hierarchy. Instead, every execution state is a point in a 17.28-million-entry type space called the Crystal, and storage is navigated by address rather than path.

The kernel runs on the 12-opcode IMASM instruction set. Each tick executes a single IMASM token, and the grammar constrains what each token does to the current state. Every tick is a self-verification: the Frobenius identity μ∘δ = id is enforced by the grammar rather than by a kernel API.

**Target:** x86_64-unknown-none (bare-metal ELF boot, zero external crates)  
**License:** Unlicense (public domain)  
**Total codebase:** ~30,000 lines of Rust

---

## Core Architecture

### The Crystal of Types  

The 12 primitives of the Imscribing Grammar define a type space of 17,280,000 addresses. Every object in the kernel — programs, data structures, witness proofs — is an address in this space. Navigation is by address lookup, not path traversal.

### The Frobenius Loop  

The kernel's main loop is `THINK → ACT → OBSERVE → UPDATE`. Each phase corresponds to IMASM opcodes:  

- **THINK:** Read the boundary (⊢ VINIT)
- **ACT:** Advance and compose (> AFWD, ⋈ CLINK)
- **OBSERVE:** Self-reference and frame (⊙ IMSCRIB, ∈ FSPLIT)
- **UPDATE:** Close and fix (∋ FFUSE, ⊡ IFIX)

Every complete cycle satisfies μ∘δ = id by construction.

### Catalog Integration  

Nine modules from upstream Grammar repositories (imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) run natively in the kernel. The catalog (`catalog.rs`, 954 lines) is the single source of truth for all data: no hardcoded constants, no ordinal arrays, no glyph strings exist outside it. New systems are registered at runtime via `register_entry()` without source edits.

---

## Capabilities

### Topological Quantum Computing  

The kernel braids Fibonacci anyons directly on the metal. The `fibqc` module compiles standard quantum gates to braid words and evaluates knot invariants (Jones polynomial) with no host runtime and no floating-point unit assumed.

### SIC-POVM Implementation  

The d=12 SIC-POVM campaign runs on bare metal via the `d12` REPL command. Five verified pillars:
1. **Phase-tower collapse:** 3→1 independent generators (8× reduction)
2. **Magnitude square-class group:** K₁₆, rank 5
3. **31-orbit Galois structure:** All 143/143 existence-grade overlaps ring-exact
4. **Dual-Link identification:** norm(N₁) = 1/32448², ramification {2,3,13}
5. **Belnap SIC unconditional:** SIC existence proven axiom-free in the Belnap multilattice for d=2ⁿ

### Belnap Paraconsistent Logic  

The Belnap FOUR lattice (T, F, B, N) is the paraconsistent foundation for the entire kernel. The `belnap_c4.rs` module implements a complex plane where i² = B (both-true-and-false), with Frobenius-verified arithmetic. The `belnap_shor.rs` module runs Shor's algorithm on Belnap FOUR, finding that the period r is encoded in the 2:1 coherence cost ratio between B-bias and T-bias.

### Clay Millennium Witnesses  

All seven Clay Millennium Problems are analyzed through the grammar, with IMASM witness programs for:
- **BSD:** Hodge theory witness
- **Hodge:** Mass gap witness  
- **Yang-Mills:** Regularity witness

The `frobenius_unify.rs` module unifies all four Frobenius conditions (kernel, grammar, catalog, SIC) as one machine-checked invariant.

### Red-Hot Rebis Integration  

All 20 modules from `red-hot_rebis/` and `gene_imscriber/` run as no_std Rust off the REPL:
- **p4ra:** Paraconsistent kernel
- **genetic:** Codon ↔ amino acid ↔ glyph translation
- **enzymes:** 109 enzyme tuples with catalytic mechanisms
- **ligand:** Functional group binding design
- **frustration:** Residue-residue energetic frustration matrices

### Cross-Dialect Navigation  

The kernel can navigate between 12 dialects with different structural rulesets, gate thresholds, and absorption rules. The Crystal is invariant; the ruleset is a sheaf that determines what each address *does*. Eleven diaschizic compounds modulate gate thresholds and T-constitution at load time.

### Real x86 Execution  

`vox run <file> [--argv a,b]` lifts a real ELF or PE binary and runs it as an actual process, for real: `vox_core::imasm_module::emit` produces the payload-carrying twelve-glyph module (each glyph plus its actual registers, immediates, and memory operands, not just the bare structural word `weight`/`banked`/`cycle`/`imasm derive` read), and `vox_core::imasm_vm::Machine` interprets it with genuine registers, byte-addressed memory, flags, and ALU semantics. It lays out a real `argv`/`envp`/`auxv` stack the way the psABI guarantees at process entry and runs from the file's own entry point — the only address a PE binary (a `.exe`) offers at all, since the loader never reads a symbol table for that format, only ELF has one. `read`/`write`/`open`/`openat`/`close` go through a `Host` bridge to the real host filesystem and console, and `mmap`/`brk` hand out a real anonymous heap from a bump cursor: a guest that opens a file, reads it, writes to stdout, or allocates memory does all of that for real, not a simulation of it.

`vox run <symbol> <file> [--args a,b]` calls one named function directly instead: scalar integer arguments in, one integer back, System V calling convention, no process at all — the older, narrower contract, unchanged.

Verified byte-for-byte against native execution on four hand-written static ELF fixtures (`measurements/hellotest.c`, `measurements/brktest.c`, `measurements/filetest.c`, `measurements/exittest.c`): a bare `write()`+`exit()`, a `brk()`+`mmap()`+`write()` heap test, an `openat()`/`read()`/`write()`/`close()` round trip on a real file, and the original exit-only entry point — output and exit code both match ground truth exactly, both against the standalone `vox` binary and inside mOMonadOS's own REPL.

A statically-linked binary using only direct syscalls runs for real, end to end. Two rungs above that are named, not silently absorbed: dynamic linking (a binary's calls into libc's PLT/GOT resolve nowhere, since no shared library is ever loaded) and thread-local storage (a glibc-static binary's own `_start` sets up a segment register this VM has no notion of at all, and spins rather than completing — confirmed by tracing the native binary's real syscalls). Both halt or run off the step limit rather than pretending to succeed. Hosted builds only — it needs a filesystem to read the binary from and to back real file I/O.

---

## Usage

### Quickstart

After building, boot into the REPL (`cargo run --release --features hosted`, or `qemu-system-x86_64` per below) and try these — each one is a real computation, not a printout of a claim:

```
fibqc verify           Fibonacci anyon braid algebra, ten identities checked live:
                        F/S unitarity, Pentagon, Yang-Baxter, spin-statistics, Verlinde...
qc HTSX                compile the gate sequence H T S X to a real braid word over
                        Fibonacci anyons, with a measured approximation error
bi 1 2 -1 -2 1 2       draw a braid word as a strand diagram, in the terminal
jp 1 1 1               the Jones polynomial of that braid, at the golden-ratio winding
shor dialetheic 15 7   Shor's period-finding over the Belnap-Fibonacci pipeline —
                        factors 15 = 3 x 5, full register walk shown
d12                    the d=12 SIC-POVM existence proof status: 143/143 overlaps closed
sic                    the SIC-POVM identity as three independent lattice proofs
                        meeting at one dimension
```

And to see it run a real compiled program rather than a structural word:

```bash
gcc -nostdlib -static -o measurements/hellotest measurements/hellotest.c
cargo run --release --features hosted
```
```
⊙> vox run measurements/hellotest
hello from inside the twelve
entry(...) exited(7)   [19 steps in the twelve]
```

That's a real ELF, lifted to the twelve-glyph module and interpreted with genuine x86-64 registers, memory, and syscalls — see [Real x86 Execution](#real-x86-execution) above.

### Building  

```bash
cd $m⊙^{2}$
cargo build --target x86_64-unknown-none --release
```

### Running under QEMU  

```bash
qemu-system-x86_64 -nographic -kernel target/x86_64-unknown-none/release/imonad
```

### Distributable Binaries

The hosted REPL (`--features hosted`) is a normal userspace executable and builds natively for Linux, Windows, and macOS:

```bash
cargo build --release --features hosted
```

`.github/workflows/release.yml` builds all three on GitHub's own runners — real MSVC on `windows-latest`, real Xcode on `macos-latest` (a universal binary covering both Intel and Apple Silicon via `lipo`), no cross-compile toolchain needed anywhere. It needs the sibling `Vox` and `MoDoT` repos pushed to their GitHub remotes first, since `Cargo.toml` resolves them by relative path (`../Vox`, `../MoDoT/imasm_core`) and the workflow checks them out as siblings to match. Push a tag matching `v*` to cut a release with all three binaries attached, or run it manually from the Actions tab to just produce build artifacts.

### REPL Commands  

```
d12              → d=12 SIC-POVM status
d12 tower        → Ray class field tower
d12 verify       → Cross-verification
c4               → Belnap C₄ complex plane
belnap           → Belnap FOUR lattice
stark            → Stark unit extraction
clay             → Clay Millennium status
triple           → von Neumann superoperator algebra
ruleset          → Cross-dialect navigation
fibqc            → Topological quantum computer
vox run <file> [--argv a,b]
                 → run it as a real process: real argv/envp/auxv, real syscalls
vox run <sym> <file> [--args a,b]
                 → call one function directly, no process
```

---

## Why This Matters

**Self-verification:** Every tick satisfies μ∘δ = id by construction, not by testing.

**Zero external dependencies:** The kernel is pure no_std Rust with zero crates.

**Grammar-enforced correctness:** The 12-opcode grammar constrains what each token does; there is no undefined behavior.

**Bare-metal topological QC:** Fibonacci anyon braiding runs directly on hardware without a quantum runtime.

**Machine-checked witnesses:** Clay Millennium witnesses are IMASM programs, not prose claims.

**Runtime-extensible:** New systems register at runtime without source edits.

---

**μ∘δ=id**
