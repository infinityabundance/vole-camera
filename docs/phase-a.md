# Phase A — repository foundation and canonical types

## Deliverable

A single-crate, single-binary Rust foundation for VOLE-Camera with the canonical
camera-domain types, the normative non-claims/architecture documents, gitignore
of the sibling research repositories, and a green build gate.

## What was built

- `Cargo.toml`: crate `vole-camera` (lib `vole_camera`, bin `vole-camera`);
  `edition = 2021`, `rust-version = 1.98` (matching the current VOLE
  toolchain); `autobins = false`; features `default` (empty),
  `playback` (winit + softbuffer), `pi-libcamera` (libcamera, non-normative);
  a commented `vole-video` interop dependency pending API review (§168).
- `rust-toolchain.toml`: stable + rustfmt + clippy.
- `src/lib.rs`: `#![forbid(unsafe_code)]`; module declarations; crate-level
  documentation of the invariants and epistemic discipline.
- `src/domain.rs`: canonical 10-bit sensor sample domain (`SensorSample`,
  `0..=1023` stored in `u16`, §17).
- `src/cfa.rs`: four CFA patterns with correct component resolution (`CfaPattern`,
  `CfaComponent`, §16) and CFA-awareness tests (§48).
- `src/time.rs`: exact rational timing (`Rational`, `RationalTime`, §19) and the
  rolling-shutter row-time test (§20).
- `src/limits.rs`: typed execution envelope (`Limits`, §89) with checked
  geometry validation (§211–§212).
- `src/error.rs`: typed `CameraError` / `CameraResult` (§90, §233).
- `docs/`: `non-claims.md`, `architecture.md`, `reread-sources.md`.
- `SECURITY.md`, `CONFORMANCE.md`, `PROJECT_STATE.md`, `README.md` (§207 language).
- `.gitignore`: ignores `entropyfs/`, `dsfb/`, `vole/`, `research/` and build
  artifacts.

## Reread sources (pinned)

`docs/reread-sources.md` records the exact commits read at implementation time:

- `vole` → `dcf9f8cec44af9da3ccc6df4317238d96224cc1f`
- `entropyfs` → `cdb4ee3bed6c6ce8ed2ef6fbc882d65989a498ab`
- `dsfb` → `aa6acd4fdbf1bb30c05ae95bbb41124441191a12`

Prior-art paper: de Beer (2026), DOI `10.5281/zenodo.22312959`.

## Courts (passed)

All canonical-type tests pass:

- 10-bit domain accepts `0..=1023`, rejects out-of-range (`domain.rs`).
- All four CFA 2×2 blocks resolve correctly; pattern repeats on odd dimensions;
  parse round-trips; one-pixel shifts change component (`cfa.rs`).
- Rational reduction, zero-denominator rejection, exact add/mul, rolling-shutter
  row time (`time.rs`).
- Limits reject zero and absurd geometry, accept IMX415 geometry, checked
  product (`limits.rs`).

## Negative controls / limitations

- `cargo check --all-targets --all-features` fails on a desktop host because
  the `pi-libcamera` feature pulls the `libcamera` crate whose `build.rs`
  requires `libcamera.pc`. This is **environment-gated and expected** (§208):
  the default and `playback` builds are green; the all-features gate is
  satisfied on a Pi host with libcamera installed. No simulator capability or
  normative path depends on this.
- No simulator, recording, or playback code exists yet (Phases B–H).

## Gate

```
cargo fmt --check                       PASS
cargo check --all-targets               PASS
cargo check --features playback         PASS
cargo clippy --all-targets --all-features -- -D warnings   (see limitation)
cargo test                              PASS
cargo test --features playback          PASS
```

## Verdict

Phase A foundation **sealed** except the environment-gated `pi-libcamera`
all-features leg, which is deferred to the Phase J hardware host by design.
