# Coordinate Frames in OMNI-SENSE

---

## 1. Overview

Every observation in OMNI-SENSE is expressed in a named coordinate frame. Every transform between frames is explicit, typed, and carries uncertainty. This document defines the frame taxonomy, naming conventions, transform conventions, and the frame tree that OMNI-SENSE maintains.

---

## 2. Frame Taxonomy

### 2.1 World Frame

The fixed external reference frame. For outdoor deployments (HALO-AD, TALON-MESH), this is typically an Earth-Centered Earth-Fixed (ECEF) or local East-North-Up (ENU) frame. For indoor deployments (AEGIS-MESH), this is the home's floor-plan coordinate system (origin at a designated corner, X pointing east along the floor plan, Y pointing north, Z pointing up). For SENTINEL-WEAR, this is the local horizontal frame anchored to the wearer's initial position.

Frame ID convention: `"world"`.

### 2.2 Platform Frame

A frame attached to a moving platform: a vehicle, a mast structure, an aerial node. The platform frame moves and rotates with the platform. The transform from platform to world is time-varying.

Frame ID convention: `"platform/<name>"` e.g. `"platform/mast_north"`.

### 2.3 Sensor Frame

A frame attached to a specific sensor. The transform from sensor to platform is typically fixed (the sensor is bolted to the platform). If the sensor has a steerable head (gimbal, phased array), the sensor frame rotates relative to the platform frame.

Frame ID convention: `"sensor/<platform_name>/<sensor_id>"` e.g. `"sensor/mast_north/radar_0"`.

### 2.4 Body Frame (SENTINEL-WEAR specific)

The coordinate system centered on the wearer's torso with axes aligned to the torso's anatomical orientation. X points forward (anterior), Y points left (sinister), Z points up (superior). The body frame is the primary output frame for SENTINEL-WEAR detections.

Frame ID convention: `"body/<wearer_id>"`.

### 2.5 Stabilized Body Frame

A version of the body frame with the wearer's rotational motion removed. In `Translational` stabilization mode, wearer translation is preserved (the frame moves with the wearer through space) but rotation is removed (the frame axes remain fixed relative to the world). This makes directional alerts intuitive — a detection at bearing 180° (behind) stays at 180° even if the wearer turns, until the object itself moves.

Frame ID convention: `"body_stabilized/<wearer_id>/<mode>"`.

### 2.6 Node Frame (AEGIS-MESH, SENTINEL-WEAR)

The local coordinate frame of a sensing node. For wall-mounted nodes, this is fixed relative to the home. For body-worn nodes, this is fixed relative to the body part it's worn on. Each node's IMU continuously reports the node-to-body transform.

Frame ID convention: `"node/<node_id>"`.

---

## 3. Transform Conventions

### 3.1 Transform representation

A `Transform` carries:
- `from: FrameId` — source frame
- `to: FrameId` — target frame
- `rotation: UnitQuaternion<f32>` — rotation from `from` to `to`
- `translation: Vector3<f32>` — translation of `to` origin expressed in `from` coordinates
- `timestamp: Timestamp` — when this transform was valid
- `covariance: TransformCovariance` — 6×6 pose uncertainty (3 rotation + 3 translation)

### 3.2 Application of transform

For a point `p_from` expressed in frame `from`, the point in frame `to` is:
```
p_to = transform.rotation × p_from + transform.translation
```

For velocity vectors, only the rotation applies (velocity is invariant to translation between frames that are not relatively moving; if frames are relatively moving, the velocity of the frame origin must be subtracted):
```
v_to = transform.rotation × v_from − v_frame_origin_in_to
```

### 3.3 Uncertainty propagation

The covariance of a transformed point is propagated using the linearized formula:
```
P_to = J × P_from × Jᵀ + P_transform (via cross terms)
```
where J is the Jacobian of the transform with respect to the point coordinates. Implementation in `omni-sense-frames::uncertainty_propagation`.

### 3.4 Transform composition

Two transforms A (from → intermediate) and B (intermediate → to) compose as:
```
(B ∘ A).rotation = B.rotation × A.rotation
(B ∘ A).translation = B.rotation × A.translation + B.translation
(B ∘ A).covariance = propagated via compound transform Jacobian
```

---

## 4. The Frame Tree

`omni-sense-frames::FrameTree` maintains the directed acyclic graph of transforms. It supports:

- **add_transform(transform)**: Add or update a transform.
- **lookup(from, to, at_time)**: Find the composed transform from any registered frame to any other, optionally at a historical time. Returns None if no path exists.
- **lookup_with_extrapolation(from, to, at_time)**: As above, but linearly extrapolates if `at_time` is beyond the latest stored transform (use with care; extrapolation accuracy degrades quickly for rotating frames).

The frame tree is updated by sensor drivers (which report the sensor's current pose) and by the calibration subsystem (which estimates node positions from walk-through calibration). In SENTINEL-WEAR, the body-pose estimator in `omni-sense-imu` updates the inter-segment transforms in the frame tree at every IMU sample rate.

---

## 5. Walk-Through Calibration

When a user walks through their home (AEGIS-MESH) or performs a body-pose calibration motion (SENTINEL-WEAR), the `omni-sense-frames::WalkThroughCalibrator` estimates the position of each sensor node relative to the reference frame.

**Algorithm:**
1. Record (detected_user_position, timestamp) pairs from all nodes simultaneously.
2. For each node, collect (range_to_user, timestamp) from its range sensor.
3. For each timestamp, the node's range to the user's known position gives the node's distance from that position. Multiple positions at different times over-constrain the system.
4. Solve as weighted least squares: minimize Σ (range_measured_k − |node_position − user_position_k|)² over node_position.
5. Report confidence based on residual fit quality and the geometric spread of user positions (well-spread positions → better-conditioned system → higher confidence).

**Requirements for convergence:**
- At least 4 user positions where the node has valid range measurements.
- User positions must not all be collinear with the node (collinear positions leave a degree of freedom unresolved).
- Node must be within range of its sensor during the calibration walk.

---

## 6. Body Frame Stabilization (SENTINEL-WEAR)

The `omni-sense-frames::BodyFrameStabilizer` removes wearer motion from observations to produce the stabilized body frame.

### 6.1 Translational mode (default)

**Input:** Observation in node frame; current body-pose estimate from IMU fusion.
**Steps:**
1. Transform observation from node frame to body frame using the node-to-body transform (node's IMU-estimated orientation relative to torso IMU).
2. Subtract the torso's angular velocity component from the observation's apparent velocity.
3. Preserve the torso's translational velocity (observations appear to move in the direction opposite to the wearer's walk — a stationary wall behind the wearer appears to recede as the wearer walks forward).
**Result:** The stabilized frame co-translates with the wearer but does not co-rotate. A human approaching from behind stays at bearing 180° in this frame even if the wearer turns around.

### 6.2 World mode

**Steps:**
1. Transform to body frame (as above).
2. Apply the torso-to-world transform from IMU fusion to express the observation in world coordinates.
**Result:** Full world-frame observations. "Approach from the north" is reported regardless of wearer orientation. Useful for navigation applications.

### 6.3 No stabilization

Raw body frame. Used for diagnostics and calibration.
