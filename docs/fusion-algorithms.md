# Fusion Algorithms in OMNI-SENSE

---

## 1. Overview

`omni-sense-fusion` provides multi-sensor and multi-target fusion algorithms. The algorithms are generic over state and measurement types, enabling specialization to each consuming project's data structures without code duplication.

---

## 2. Kalman Filter Family

### 2.1 Standard Kalman Filter (linear systems)

For systems with linear dynamics and Gaussian noise. State dimension N, measurement dimension M, both fixed at compile time via const generics.

**Predict step:**
- x̂⁻ = F × x̂ + B × u
- P⁻ = F × P × Fᵀ + Q

**Update step:**
- K = P⁻ × Hᵀ × (H × P⁻ × Hᵀ + R)⁻¹
- x̂ = x̂⁻ + K × (z − H × x̂⁻)
- P = (I − K × H) × P⁻

**When to use:** Object kinematics modeled as constant velocity or constant acceleration in a locally Cartesian frame. Radar-only single-target tracking over short durations.

### 2.2 Extended Kalman Filter (mildly nonlinear systems)

Linearizes nonlinear process and measurement models around the current estimate via Jacobian. Suitable when the nonlinearity is mild (Jacobian is a reasonable local approximation).

**When to use:** Tracking objects in polar/spherical coordinates where the sensor measures range and bearing directly. Body-frame tracking where the frame is non-inertial. IMU integration where the rotation group is nonlinear.

### 2.3 Unscented Kalman Filter (strongly nonlinear systems)

Uses a set of sigma points propagated through the nonlinear function to estimate the mean and covariance without computing Jacobians. More accurate than EKF for highly nonlinear systems.

**When to use:** Highly curved trajectories where EKF diverges. Long-range tracking where the spherical-to-Cartesian conversion introduces significant nonlinearity.

### 2.4 Particle Filter (non-Gaussian, multi-modal distributions)

Represents the posterior as a weighted set of particles. Each particle is propagated through the (possibly nonlinear, non-Gaussian) dynamics; weights are updated based on measurement likelihood.

**When to use:** Initial acquisition with high uncertainty (broad, possibly multi-modal prior). Ambiguous measurement scenarios where multiple hypotheses must be maintained simultaneously. Classification-uncertain tracks where the track might belong to one of several very different object classes.

---

## 3. Multi-Target Association

### 3.1 Joint Probabilistic Data Association (JPDA)

For each new measurement, computes association probabilities to each existing track, marginalized over all possible association hypotheses. Each track is then updated with a weighted sum of measurements proportional to association probabilities.

**Strengths:** Computationally tractable for moderate numbers of targets and measurements. Handles missed detections gracefully. Naturally represents association uncertainty.

**Limitations:** Tracks may merge under heavy clutter. Does not represent discrete association ambiguity (multiple discrete hypotheses).

**When to use:** AEGIS-MESH multi-node fusion (moderate clutter, indoor). HALO-AD multi-mast track association. TALON-MESH multi-modal sensor association.

### 3.2 Multiple Hypothesis Tracking (MHT)

Maintains an explicit tree of association hypotheses. Each node in the tree represents one possible set of track-to-measurement assignments. Pruning removes low-probability branches.

**Strengths:** Correct handling of discrete association ambiguity. Best performance in dense target scenarios. Can represent track initiation and deletion within the hypothesis tree.

**Limitations:** Exponential worst-case complexity; requires aggressive pruning. More complex to implement correctly than JPDA.

**When to use:** TALON-MESH counter-UAS scenarios with decoy targets. Dense-swarm scenarios where identities matter. Any scenario where track identity confusion is a primary concern.

---

## 4. Covariance Intersection

The primary fusion method for distributed multi-node systems where cross-correlations between node estimates are unknown.

**Formula (two estimates):**
P_CI⁻¹ = ω × P₁⁻¹ + (1−ω) × P₂⁻¹
x_CI = P_CI × (ω × P₁⁻¹ × x₁ + (1−ω) × P₂⁻¹ × x₂)

where ω is chosen to minimize det(P_CI) (or tr(P_CI) for efficiency).

**Generalization to N estimates:** Apply pairwise or use the generalized CI formula with N weights summing to 1.

**Why this matters:** In AEGIS-MESH, multiple ceiling nodes both observe the same person. Their observations are correlated (both use the same room geometry, both may have been calibrated by the same walk-through). If the cross-correlation is unknown and we assume it is zero (naive fusion), the fused estimate is overconfident (P_fused is too small). CI fusion is always consistent: P_CI ≥ P_true_fused regardless of the unknown cross-correlation.

**Used in:** AEGIS-MESH mesh fusion, SENTINEL-WEAR cross-node fusion, HALO-AD multi-mast fusion.

---

## 5. Track-Level Fusion

`omni-sense-fusion::TrackLevelFuser` provides the high-level interface used by all four projects.

**Interface:**
```
TrackLevelFuser::fuse_node_tracks(
    node_tracks: &[(NodeId, Vec<Track>)],
) -> Vec<FusedTrack>
```

**Internally:**
1. Receives per-node tracks (each node has already done per-modality fusion to produce one track per target per node).
2. Applies geometric gating: only consider association between tracks whose predicted positions are within a configurable Mahalanobis distance.
3. Applies JPDA or MHT (configurable) to produce association weights.
4. Applies Covariance Intersection to merge associated tracks.
5. Produces FusedTrack per target with: fused position + covariance, fused velocity + covariance, participating node IDs, association confidence.
