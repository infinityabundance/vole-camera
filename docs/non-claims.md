# VOLE-Camera — non-claims and epistemic discipline

This document is normative. It is the living statement of what VOLE-Camera is
and — more importantly — what it is **not** allowed to claim. Every receipt,
README, and court result must respect these boundaries so that a simulator
capability is never promoted to a hardware claim, a hardware capture capability
is never promoted to a natural-video storage claim, and deterministic synthetic
noise is never offered as evidence about physical shot noise.

## The research question (§0)

> How much of a captured sensor stream can be represented economically as
> persistent deterministic capture state, camera telemetry, reusable visual
> structure, bounded procedural evolution, and exact predictive models before
> the residual information dominates? And does starting the representation at
> the camera preserve useful deterministic state that is lost when sensor
> observations are first flattened into conventional raster/video frames?

The system must be capable of **falsifying** this thesis. It must not merely
demonstrate favorable synthetic examples.

## Non-claims

1. **No seed magic.** A VOLE-Camera root seed is the canonical root of the
   complete entropy-bearing deterministic representation (`Σ`), not a fixed
   128/256-bit value from which arbitrary video spontaneously emerges (§2).
   We never report a "32-byte video" merely because the root commitment is 32
   bytes; every entropy-bearing byte is counted (§3, §40).

2. **No entropy computation.** "Procedural entropy factorization" is an
   operational term: it separates a deterministically reusable explanation from
   the information still required to reproduce the observation (§4). It does
   not compute Shannon entropy or Kolmogorov complexity. We report state/model/
   residual/raw bytes, never "93% of entropy understood" (§87).

3. **No ML.** No neural scene reconstruction, learned denoiser, learned optical
   flow, model download, or learned codec (§9).

4. **No causal truth.** Camera telemetry is "capture-side explanatory state" or
   "candidate causal state", never proven physical causality (§55). A better
   cost from exposure/gain telemetry means only "telemetry reduced the complete
   exact representation cost in this court" (§205).

5. **No hardware claim from the simulator.** Simulator throughput implies
   nothing about IMX415 realtime on Pi 5 (§203); known-seed simulator noise
   implies nothing about physical shot noise (§204).

6. **No universal compression claim.** The architecture spans a spectrum from
   state-dominant to residual-dominant sources. Residual-dominant results are
   successes to be recorded, not failures to be hidden (§154, §232).

7. **No "no raster" overclaim.** Preferred wording: "VOLE-Camera playback
   requires no mandatory complete intermediate raster-frame representation."
   We do not say "pixels no longer exist"; the sensor and display are sampled
   devices (§206).

8. **No lossy comparison for exact claims.** Sensor-exact comparisons are made
   against lossless baselines; lossy H.264 is a separate profile (§142).

## Epistemic categories (§5)

Every claim is tagged exactly one of:

- `PROVEN` — established by this project's sealed courts.
- `MEASURED` — established by a specific measurement, with receipt.
- `PROPOSED` — a concrete engineering hypothesis awaiting a court.
- `SPECULATIVE` — a plausible direction requiring experiment.

A claim may never be silently upgraded. The category is part of the text.

## The residual is not failure

The remainder after deterministic explanation is the residual entropy-bearing
observation the current model did not explain. It is central output, not
embarrassment (§242).

## Source-native vs inverse (§34–§36)

Two paths are measured separately and never conflated:

- **Source-native**: known scene/camera/sensor state → native ingest → recording.
- **Flattened inverse**: same source → raw Bayer observations → discard source
  state → inverse factorizer → recording.

Both must reconstruct the same canonical raw sensor observation. The flattening
tax `T_flatten = B_inverse / B_native` is reported with all four byte counts,
never only the ratio (§36).

## Directness vs compression (§155)

Direct materialization is an execution-property claim; representation size is a
separate measurement. They are never conflated.
