//! VOLE-Camera — sensor-native procedural entropy factorization and direct
//! visual materialization.
//!
//! # What this crate is
//!
//! VOLE-Camera is an experimental native-Rust implementation of **sensor-native
//! procedural entropy factorization**. It moves the primary persisted
//! representation of captured visual information away from an obligatory
//! sequence of completed image frames and toward a bounded, deterministic
//! representation of **capture-side state, reusable visual structure, time
//! evolution, procedural generators, predictive models, and exact residual
//! information**.
//!
//! The central factorization is operational, not ontological:
//!
//! ```text
//! X = M(U, Σ, V) ⊕_ρ R
//! ```
//!
//! where `X` is the declared canonical observation, `U` the versioned
//! deterministic universe, `Σ` the procedural/capture-state description, `V`
//! the requested view, `M` the deterministic materializer, and `R` the exact
//! residual required to close the difference. When the capture is strongly
//! determined by compact state, `R` may be small or zero; when the observation
//! contains substantial innovation, `R` dominates. Compression may result, but
//! compression ratio is **not** the defining abstraction.
//!
//! # What this crate is NOT
//!
//! This crate does **not** claim that a tiny fixed-size seed reproduces
//! arbitrary camera footage. It does not use ML. It does not claim to compute
//! Shannon entropy or Kolmogorov complexity. It does not claim that synthetic
//! simulator demonstrations transfer to physical sensors. Every such boundary
//! is enforced in code and documentation.
//!
//! # Epistemic discipline
//!
//! Every document and receipt distinguishes `PROVEN`, `MEASURED`, `PROPOSED`,
//! and `SPECULATIVE` claims. A simulator capability is never promoted to a
//! hardware claim; a hardware capture capability is never promoted to a
//! natural-video storage claim; deterministic synthetic noise is never offered
//! as evidence about physical shot noise.
//!
//! # Safety posture
//!
//! The normative implementation forbids `unsafe` code (`#![forbid(unsafe_code)]`).
//! A conforming decoder treats every input as hostile: it returns typed,
//! deterministic [`CameraError`]s under [`crate::limits::Limits`] and never
//! panics, exhausts the stack, overflows coordinates, or grows without bound.
//!
//! # Architecture notes
//!
//! - The **hardware acquisition boundary** (a future `pi-libcamera` feature)
//!   is NON-NORMATIVE. No recording requires libcamera to decode.
//! - A `.volecam` recording is a **provisional research container**, explicitly
//!   marked `EXPERIMENTAL` / `NOT STABLE`, until the corresponding VOLE
//!   universe can represent the sensor domain exactly (§12).
//! - The direct player reports, and the headless core enforces, that no
//!   mandatory complete intermediate raster frame is staged (§70).

#![forbid(unsafe_code)]

pub mod cfa;
pub mod domain;
pub mod error;
pub mod limits;
pub mod time;

pub use cfa::{CfaComponent, CfaPattern};
pub use domain::{SensorSample, SENSOR_ACTIVE_BITS, SENSOR_MAX_VALUE};
pub use error::{CameraError, CameraResult};
pub use limits::Limits;
pub use time::{Rational, RationalTime};
