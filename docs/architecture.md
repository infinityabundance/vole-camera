# VOLE-Camera architecture

Normative module direction and interoperability boundary (brief §6, §11,
§176).

## Position among the four systems

```
VOLE-Camera  = camera-specific layer (this crate): sensor sample domain,
               capture state, rolling-shutter timing, camera telemetry,
               calibration, sensor-space predictors, camera-native root seed,
               camera-specific evidence.
VOLE         = general video representation (optional interop, §11).
EntropyFS    = optional persistence substrate (never required to decode).
DSFB         = zero-authority encoder search governance (never normative decode).
```

VOLE-Camera is a **separate repository** from `infinityabundance/vole`, but
conceptually downstream of it. It is NOT a fork of VOLE internals, NOT a camera
codec unrelated to VOLE, and it does NOT duplicate VOLE's entire visual inverse
engine. It owns only the camera-specific layer and delegates general raster
video semantics to `vole-video` through a clean, optional boundary.

## Single crate (§6)

One Cargo crate, `vole-camera`, lib `vole_camera`, binary `vole-camera`.
No workspace, no ten-crate decomposition; internal Rust modules suffice.

## End-to-end data flow (§1)

```
photons → sensor measurements → raw CFA samples + timing + exposure/gain +
calibration + motion + lens → factorizer → persistent procedural state +
transitions + reusable structure + predictors + exact residual → root-addressed
recording → (playback) root → state → evolution → bounded materialization →
exact residual closure → only requested tile samples → projection → display.
```

## Module direction (internal modules)

```
src/
    domain.rs        canonical sensor sample domain (u16, 10 active bits)
    error.rs         typed CameraError / CameraResult
    limits.rs        typed execution envelope (geometry, depth, counts, bytes)
    time.rs          exact rational ticks (Rational / RationalTime)
    cfa.rs           CFA pattern + photosite component resolution
    raw.rs           RAW10 packed / unpacked boundary adapters (Phase B)
    sensor.rs        sensor profile + timing + rolling-shutter model
    telemetry.rs     capture telemetry records (exposure/gain/timing/motion)
    calibration.rs   persistent sensor calibration state
    simulator/       deterministic scene + sensor + noise + reference (Phase B)
    seed.rs          RootSeed / content identity / UniverseId / ContentId
    format.rs        provisional .volecam grammar (EXPERIMENTAL, NOT STABLE)
    integrity.rs     BLAKE3 domain-separated hashing + integrity records
    index.rs         checkpoints + observation index (seek)
    state.rs         persistent capture/procedural state
    transition.rs    capture-state transitions
    residual.rs      exact widened residual: R = X − X̂, no saturation
    factor/          candidate families, native ingest, inverse factorizer, cost
    materialize.rs   target-oriented sensor materialization
    target.rs        SensorTarget / PresentationTarget
    presentation.rs  integer demosaic + color projection (presentation-only)
    playback.rs      direct player (headless core; window behind `playback` feature)
    evidence.rs      immutable campaign receipts
    hardware/        NON-NORMATIVE transport backends (pi-libcamera, pi-v4l2)
```

`main.rs` holds the CLI. Modules land across Phases B–H; none are created as
empty placeholder theater (§176).

## Hardware boundary is non-normative (§8)

The first Raspberry Pi implementation may use safe Rust bindings to libcamera.
That boundary is a HARDWARE TRANSPORT and does not define recording semantics.
No recording requires libcamera to decode. The `pi-libcamera` cargo feature is
default-OFF and its build (via the `libcamera` crate's `build.rs`) requires
`libcamera.pc` on the host — it is therefore **environment-gated**: the
`--all-features` gate (§208) applies to it only on a machine with libcamera
installed (see `docs/reread-sources.md` and the Phase-J receipt).

## Provisional format (§12)

Until the VOLE universe can represent the sensor domain exactly, recordings use
a provisional `.volecam` research container marked `EXPERIMENTAL` / `NOT STABLE`.
The long-term objective is a VOLE-Camera camera profile expressed as a standard
`.vole` representation; we will not create two permanent competing codecs.

## Safety posture (§7, §90, §210–§212)

`#![forbid(unsafe_code)]` locally. Typed errors only; no panics on hostile
input; checked/widened arithmetic everywhere; geometry validated against
`Limits` before allocation.
