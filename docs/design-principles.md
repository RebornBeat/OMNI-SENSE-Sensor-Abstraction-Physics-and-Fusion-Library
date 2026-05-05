# OMNI-SENSE Design Principles

These principles govern every architectural decision in OMNI-SENSE. When in doubt about any design choice, these principles are the tiebreaker.

---

## Principle 1: The Boundary is DetectionEvent

Everything below the `DetectionEvent` contract is OMNI-SENSE. Everything above it is the consumer's responsibility. The DetectionEvent type must remain stable across minor versions; changes require a major version bump and migration guide.

A DetectionEvent carries everything PentaTrack needs and nothing it does not. It does not carry raw sensor data, video frames, audio waveforms, or application-specific interpretation. It carries: position with covariance, velocity with covariance (optional), modality, source, frame, timestamp, classification hints, and confidence.

Classification hints are modality-specific metadata attached to the detection: acoustic material profile, radar Doppler spectral signature, RF protocol class. These are inputs to application-layer classifiers, not OMNI-SENSE's conclusion about what the object is.

---

## Principle 2: Sim Parity with Real

Every real sensor capability has a simulation equivalent. The simulation is not a stub that returns zeros — it is a physics-correct model that generates data with the same statistical properties as the real sensor under the same conditions. A test written against simulation is a valid test of the real pipeline.

This requires simulation implementations to model: noise (per-axis Gaussian, non-Gaussian where documented), range limits (with realistic fall-off near max range), failure modes (sensor dropout, interference), environmental sensitivity (atmospheric attenuation, temperature drift for IMU), and motion artifacts (scan latency for scanning LiDAR, Doppler ambiguity for FMCW radar).

---

## Principle 3: Physics Correctness Over Simplicity

OMNI-SENSE uses the published physics. Beer-Lambert attenuation with wavelength-dependent extinction, not a generic "atmospheric loss" scalar. Gaussian beam divergence with M² factor, not a linear spread approximation. Madgwick/Mahony filter with tuned beta/gain, not dead-reckoning integration. FMCW chirp processing with proper range-Doppler formulation, not a simplified range-only approximation.

Simplifications may be offered as configuration options (e.g., "simple" vs. "full-physics" atmospheric model for fast simulation) but the full-physics implementation is always available and always the reference.

---

## Principle 4: Type-Level Enforcement of Constraints

The camera crate is a feature flag, not a runtime setting. The effector interface does not exist in this library. The "no raw sensor egress" constraint is enforced by the type system: there is no `AcousticDriver::poll_raw_waveform()` method that returns audio outside an explicitly gated API.

Where a constraint cannot be enforced by the type system, it is enforced by module visibility: private internals that could violate constraints are never exposed publicly.

---

## Principle 5: Coordinate Frames are Explicit and Typed

Every position, every velocity, every bearing is associated with a named coordinate frame. Operations that combine observations from different frames require an explicit transform. There is no "default frame" into which observations silently appear.

The `FrameId` type wraps a string that uniquely identifies the frame. `Transform` carries its source and destination frame IDs. The `FrameTree` enforces that transforms are always consistent with the declared hierarchy.

---

## Principle 6: Uncertainty is First-Class

Every position estimate carries a 3×3 covariance matrix. Every transform carries a 6×6 pose covariance. Fusion operations propagate uncertainty correctly. A `DetectionEvent` without a covariance is a bug, not a simplification.

This matters because downstream consumers (PentaTrack, application-layer anomaly detectors) make decisions based on uncertainty. A detection with high covariance should be treated differently from a detection with low covariance. If OMNI-SENSE discards uncertainty, the consumer cannot recover it.

---

## Principle 7: No Allocator Required for Core Operations

`omni-sense-core`, `omni-sense-physics`, and the per-modality processing crates must compile with `no_std + no alloc` for the core mathematical operations. Fixed-size arrays and stack-allocated data structures are used where practical. Dynamic allocation is gated by the `std` feature.

This constraint enables use of OMNI-SENSE physics primitives in firmware on MCUs without heap allocators, while still providing full functionality on standard platforms.

---

## Principle 8: Camera is Additive, Not Foundational

The camera trait (`ImagingSensor`) is never referenced in the core sensing pipeline. It is a separate crate, gated by a feature flag, that adds identification capability to an otherwise camera-free system. A project that does not depend on `omni-sense-drivers-camera` is guaranteed, at compile time, that no camera frame will be produced, processed, or transmitted.

This is the structural implementation of the privacy posture described in `docs/camera-policy.md`. Privacy is enforced by the dependency graph, not by policy.
