# Camera Policy

This document specifies the policy for camera sensor integration in OMNI-SENSE. It applies to the `omni-sense-drivers-camera` crate and to any downstream project that depends on it.

---

## 1. The Architectural Distinction

OMNI-SENSE's camera trait (`ImagingSensor`) provides a conventional imaging capability — producing frames with spatial, color, or intensity information suitable for face recognition, object identification, and visual classification. This is distinct from every other sensor modality in OMNI-SENSE in one critical way: imagery is uniquely suitable for identifying specific individuals without their knowledge or consent.

Radar, LiDAR, acoustic, IMU, PIR, and environmental sensors can all detect presence, motion, trajectory, and activity class without creating data that could identify a specific human face or body. Camera sensors can. This difference is the reason the camera trait is opt-in via feature flag and documented separately.

---

## 2. The Feature Flag

```toml
# In Cargo.toml of the consuming project:
[dependencies]
omni-sense = { version = "0.1", features = ["camera"] }
```

Projects that do not specify the `camera` feature cannot produce `ImageFrame` values. There is no runtime check involved — the `ImagingSensor` trait and `ImageFrame` type simply do not exist in their compilation. Camera-free sensing is enforced at compile time.

---

## 3. How Downstream Projects Use the Camera Feature

### AEGIS-MESH

The AEGIS-MESH sensing mesh (always-on nodes, LiDAR nodes) does not depend on `omni-sense-drivers-camera`. These nodes are camera-free by construction.

The AEGIS-MESH identity node is a physically separate piece of hardware with its own Cargo build configuration. This build depends on `omni-sense-drivers-camera`. The identity node's firmware explicitly reads a hardware-configurable flag (kill-switch GPIO state) at boot and halts if the physical kill switch is disengaged.

The identity node's output is `IdentificationEvent` — not `ImageFrame`. Frames are processed locally and never transmitted over the mesh. The classification result (face recognized, object classified) is what reaches the edge controller.

### SENTINEL-WEAR

The SENTINEL-WEAR wearable mesh (bracelet, anklet, eyewear, belt nodes) does not depend on `omni-sense-drivers-camera`.

The pendant node optionally depends on `omni-sense-drivers-camera` when configured for visual identification. The pendant's identification sensor output is processed locally — face embeddings or object classification results are produced on-device and only the classification result is transmitted over the BAN. Raw frames do not leave the pendant.

### HALO-AD

No dependency on `omni-sense-drivers-camera`. The simulation uses event-sensor and radar simulation implementations.

### TALON-MESH

No dependency on `omni-sense-drivers-camera`. EO sensors in the counter-UAS simulation are modeled as detection+classification outputs, not imagery.

---

## 4. Design Requirements for Opt-In Camera Use

Any project that opts in to the camera feature must:

1. **Hardware isolation.** The imaging sensor must be on hardware that can be physically disconnected or powered off independently of the sensing mesh.

2. **Kill switch.** An accessible physical switch that cuts power or data to the imaging sensor. Firmware reads the kill-switch state at boot and at a configurable interval during operation.

3. **On-device processing.** Raw frames must not leave the device over any network interface. Only classification results (embeddings, recognized labels, object classes) are transmitted.

4. **No persistent storage of raw frames.** The device does not write raw frames to any storage medium. Classification results may be stored locally in the audit log.

5. **Opt-in activation.** The imaging sensor is in a dormant state by default. Activation requires an explicit user action (in Privacy-First mode) or an explicit trigger from the sensing pipeline under a policy configured by the user (in Security-First mode).

6. **User-visible audit.** Every activation of the imaging sensor is recorded in the local audit log with a timestamp and trigger cause.
