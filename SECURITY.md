# Security

Hostile-input contract, limits, and trusted-environment scope.

## Safety posture

The normative VOLE-Camera implementation is `#![forbid(unsafe_code)]`. A
conforming decoder treats every input — including a `.volecam` recording — as
hostile and must never panic, exhaust the stack, hang, allocate based on
unchecked lengths, or overflow coordinates. Every failure returns a typed
[`CameraError`] (`src/error.rs`).

## Execution envelope (`src/limits.rs`)

Every declared geometry, bit depth, observation count, object count,
calibration size, transition count, tile count, dependency depth, and tile
byte size is validated against [`Limits`] **before** any allocation. Geometry
products are computed with checked arithmetic and rejected on overflow, so a
"4 billion × 4 billion sensor" declaration cannot drive out-of-memory behavior
(§211–§212).

## Non-Turing-complete representation (§42)

No arbitrary executable language, no general shaders, no bytecode VM. Every
generator/predictor is finite, versioned, bounded, and typed.

## Explicit drop / gap policy (§113–§114, §198)

The exact capture court never silently drops observations. A missing or
late sensor observation is receipted; a gap fails the continuous-capture court.

## Hardware transport boundary

The `pi-libcamera` feature is an optional, non-normative transport. It is never
present in the decode path. A `.volecam` recording contains everything needed
to interpret the media and never depends on `/etc/libcamera`, device tree, or
driver source being present at playback time (§129).

## Reporting

Security-relevant findings (panics, overflows, unbounded allocations) belong in
this file and in a phase receipt, and are regression-pinned by a hostile-input
test court.
