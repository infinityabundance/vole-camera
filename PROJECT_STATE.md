# Project state

Current phase, sealed phases, known failures, next exact gate, frozen format.

> **Current head: Phase A / B (in progress).** Phase A (repository foundation,
> canonical types) is substantially complete; the Phase B deterministic sensor
> simulator is next. No hardware code exists and none may begin before the
> simulator seal (§239, §190).

## Phase ledger (brief §174)

| Phase | Name | Status |
|-------|------|--------|
| A | Repository / architecture / canonical types | **in progress** |
| B | Deterministic sensor simulator | next |
| C | Source-native camera state recording | — |
| D | Exact residual and fallback | — |
| E | Inverse factorizer | — |
| F | Root seed / content addressing / index | — |
| G | Direct target-oriented playback | — |
| H | Simulator falsification campaign | — |
| I | Raspberry Pi hardware probe + raw oracle | blocked until H sealed |
| J | Rust libcamera raw backend | blocked |
| K | IMX415 offline factorization | blocked |
| L | Live factorization | blocked |
| M | Hardware falsification / first-demo seal | blocked |

## Phase A status

- [x] One-crate `Cargo.toml` (edition 2021, `rust-version = 1.98`).
- [x] `rust-toolchain.toml` (stable, rustfmt, clippy).
- [x] `src/lib.rs` with `#![forbid(unsafe_code)]` + invariants.
- [x] Canonical types: `domain` (u16/10-bit), `cfa` (4 patterns), `time`
      (exact rational ticks), `limits`, `error`.
- [x] `docs/`: `non-claims.md`, `architecture.md`, `reread-sources.md`.
- [x] `.gitignore` (sibling repos and `research/` ignored).
- [x] Git initialized on `main`.
- [ ] `SECURITY.md`, `CONFORMANCE.md`, `README.md`.
- [ ] Phase-A receipt (`docs/phase-a.md`).

## Reread sources (pinned)

`docs/reread-sources.md` records the exact commits of `vole`, `entropyfs`, and
`dsfb` read at implementation time, plus the prior-art paper DOI.

## Known failures / open items

- `cargo check --all-targets --all-features` currently fails on a desktop host
  because the `pi-libcamera` feature pulls the `libcamera` crate whose
  `build.rs` requires `libcamera.pc` (absent off-Pi). This is **expected and
  environment-gated** per §208; the default and `playback` builds are green.
  The full `--all-features` gate is satisfied on a Pi host with libcamera.

## Frozen format decisions

None yet. The provisional `.volecam` grammar is defined in Phase F (§41).

## Next exact gate

Phase A receipt + build gate (fmt / check / clippy `-D warnings` / test
dev+release) ⇒ then Phase B (deterministic sensor simulator + independent
reference, §177–§180).
