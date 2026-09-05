# VOLE-Camera

Sensor-native procedural entropy factorization and direct visual materialization.

<p align="center">
  <img src="assets/vole.png" alt="VOLE-Camera logo" width="313">
</p>

> de Beer, R. (2026). VOLE-Camera: Sensor-Native Procedural Entropy Factorization and Direct Visual Materialization - Broad Prior-Art Technical Disclosure and Research Architecture (Version v1.0). Zenodo. <https://doi.org/10.5281/zenodo.22312959>

> **VOLE-Camera is an experimental native-Rust implementation of sensor-native
> procedural entropy factorization. Its current proof is a deterministic
> simulated sensor pipeline; physical-camera claims remain unsealed.**

VOLE-Camera is a separate research repository from
[`infinityabundance/vole`](https://github.com/infinityabundance/vole) (VOLE —
Video Object Layer Engine), but conceptually downstream of it. It owns the
**camera-specific layer**: the sensor sample domain, capture state,
rolling-shutter timing, camera telemetry, calibration state, sensor-space
predictors, a camera-native root seed, and camera-specific evidence. It does
not fork VOLE and does not duplicate its general visual inverse engine.

## The research question

> How much of a captured sensor stream can be represented economically as
> persistent deterministic capture state, camera telemetry, reusable visual
> structure, bounded procedural evolution, and exact predictive models before
> the residual information dominates? And does starting the representation at
> the camera preserve useful deterministic state that is lost when sensor
> observations are first flattened into conventional raster/video frames?

The system is built to be **falsifiable**, not to demonstrate favorable
synthetic examples.

## Status

Current head: **Phase A / B in progress** (see `PROJECT_STATE.md`). No hardware
code exists yet; it may not begin before the simulator seal (§239). The first
complete deliverable is `vole-camera demo`, which runs entirely on an ordinary
Linux desktop without a camera (§13).

## Prior art

See `docs/reread-sources.md` for the pinned versions of the sibling research
repositories and the prior-art paper (de Beer, 2026, DOI
`10.5281/zenodo.22312959`). `docs/non-claims.md` is the normative statement of
what this project may and may not claim.

## License

Dual-licensed under the MIT or Apache-2.0 terms, at your option.
