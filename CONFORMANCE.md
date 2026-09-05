# Conformance

Per-phase conformance rows and golden tests.

This table tracks which normative behaviors are implemented and courted. A row
is marked only when the behavior exists **and** a court demonstrates it; intent
is never credited (per the VOLE methodology).

## Phase A — canonical types

| Behavior | Status | Test |
|----------|--------|------|
| 10-bit canonical sample domain `0..=1023`, stored in `u16` | done | `src/domain.rs` tests |
| Out-of-range sample rejected (typed error) | done | `src/domain.rs` tests |
| Four CFA patterns (`RGGB`/`GRBG`/`GBRG`/`BGGR`) | done | `src/cfa.rs` tests |
| CFA component resolution `(x,y)` correct for every pattern, incl. 2×2/3×3/odd/edges | done | `src/cfa.rs` tests |
| One-pixel shift changes sampled component (CFA awareness §48) | done | `src/cfa.rs` tests |
| Exact rational timing (no `f64` seconds) | done | `src/time.rs` tests |
| Rolling-shutter row time `t0 + y·T_line` exact | done | `src/time.rs` tests |
| Checked arithmetic (no silent wrap) | done | `src/limits.rs`, `src/time.rs` tests |
| Geometry validated against `Limits` before allocation; absurd geometry rejected | done | `src/limits.rs` tests |

## Golden tests

Golden fixtures live under `tests/` and are hashed; see the phase receipt for
the canonical hashes.
