# OMNI-SENSE Architecture

**Version:** 0.1 | **Status:** Research

---

## 1. Purpose

OMNI-SENSE is a Cargo workspace of focused crates providing the sensor abstraction, physics processing, atmospheric modeling, multi-modal fusion substrate, coordinate frame management, and time synchronization that four distributed-perception research projects share: AEGIS-MESH, SENTINEL-WEAR, HALO-AD, and TALON-MESH.

The library sits between raw hardware (or hardware simulation) and predictive tracking (PentaTrack). Its output is a stream of `DetectionEvent` structures. Everything above that boundary — tracking logic, decision logic, alerting logic, coordination policy — is application-specific and belongs in the consuming project.

---

## 2. Layer Model

```
┌─────────────────────────────────────────────────────────────────┐
│  Application Layer                                              │
│  AEGIS-MESH | SENTINEL-WEAR | HALO-AD | TALON-MESH             │
│  Decision, coordination, alerting, visualization, policy        │
└───────────────────────────┬─────────────────────────────────────┘
                            │  predicted positions, anomaly flags,
                            │  drift metrics, intercept zones
┌───────────────────────────▼─────────────────────────────────────┐
│  PentaTrack Layer                                               │
│  Predictive tracking: center fields, velocity weighting,        │
│  drift analysis, object-type awareness, homing intercept        │
│  Input: DetectionEvent   Output: CenterNode tree                │
└───────────────────────────┬─────────────────────────────────────┘
                            │  DetectionEvent stream
┌───────────────────────────▼─────────────────────────────────────┐
│  OMNI-SENSE Layer                                               │
│  Sensor traits, physics primitives, atmospheric models,         │
│  IMU fusion, acoustic beamforming and profiling,                │
│  multi-modal fusion substrates, frame transforms, time sync     │
│  Input: raw sensor bytes  Output: DetectionEvent stream         │
└───────────────────────────┬─────────────────────────────────────┘
                            │  raw bytes, vendor protocols
┌───────────────────────────▼─────────────────────────────────────┐
│  Hardware Layer                                                 │
│  mmWave chipsets, LiDAR units, event cameras, IMUs,             │
│  microphone arrays, environmental sensors, PIR, 1D ToF          │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Workspace Structure

The workspace is organized as a set of independently versioned crates. Consuming projects depend only on the crates they need; unused crates incur zero compile cost.

```
omni-sense/
├── crates/
│   ├── omni-sense-core/          Core types, DetectionEvent, sensor traits
│   ├── omni-sense-physics/       ToF, Doppler, beam propagation, IMU math, beamforming
│   ├── omni-sense-atmospherics/  Beer-Lambert, thermal blooming, turbulence
│   ├── omni-sense-frames/        Coordinate frames, transforms, calibration, stabilization
│   ├── omni-sense-time/          Timestamps, clock offset estimation, PTP-style sync
│   ├── omni-sense-radar/         FMCW range-Doppler processing, CFAR detection
│   ├── omni-sense-lidar/         Point cloud processing, scan handling, intensity
│   ├── omni-sense-event/         Event-camera spike-stream processing, trajectory extraction
│   ├── omni-sense-acoustic/      Beamforming pipeline, full-echo profiling, classification
│   ├── omni-sense-imu/           Multi-IMU fusion, body pose estimation, bias correction
│   ├── omni-sense-fusion/        Kalman, EKF, UKF, particle filter, JPDA, MHT, CI
│   ├── omni-sense-replay/        Recording and replay infrastructure
│   ├── omni-sense-drivers-mmwave/    mmWave trait + sim impl
│   ├── omni-sense-drivers-lidar/     LiDAR trait + sim impl
│   ├── omni-sense-drivers-event/     Event camera trait + sim impl
│   ├── omni-sense-drivers-acoustic/  Microphone array trait + sim impl
│   ├── omni-sense-drivers-imu/       IMU trait + sim impl
│   ├── omni-sense-drivers-pir/       PIR trait + sim impl
│   ├── omni-sense-drivers-tof/       1D ToF trait + sim impl
│   ├── omni-sense-drivers-env/       Environmental sensor trait + sim impl
│   ├── omni-sense-drivers-camera/    Camera trait — opt-in feature flag
│   ├── omni-sense-bindings-python/   PyO3 bindings for analysis tooling
│   └── omni-sense-prelude/           Re-export crate for ergonomic imports
```

---

## 4. Design Principles

### 4.1 Trait-based sensor abstraction

Every sensor modality is defined as a Rust trait in `omni-sense-core`. Vendor-specific implementations and simulation implementations both implement the same trait. Application code depends on the trait; switching from simulation to hardware is a one-line change.

### 4.2 Simulation-first

Every trait has a corresponding `Simulated*` implementation gated by the `simulation` feature flag. These produce physically-correct synthetic data parameterized by configurable scenarios. All four consuming projects develop and test their perception pipelines against simulation before any hardware is attached.

### 4.3 DetectionEvent as the contract

The boundary between OMNI-SENSE and PentaTrack is the `DetectionEvent` type defined in `omni-sense-core`. OMNI-SENSE produces it; PentaTrack consumes it. Neither side needs to know the other's internals. The contract includes: position with covariance, velocity with covariance (when available), modality tag, source sensor ID, source frame ID with pose at observation time, monotonic timestamp, classification hints, and confidence.

### 4.4 Camera as opt-in

The `omni-sense-drivers-camera` crate is gated by a Cargo feature flag `camera`. Projects that do not specify this feature have camera-free sensing by construction — no runtime check, no configuration flag. Projects that opt in acknowledge the privacy and policy documentation in `docs/camera-policy.md`. This makes the camera-free posture of AEGIS-MESH (sensing mesh) and SENTINEL-WEAR (sensing mesh) structural rather than configuration-dependent.

### 4.5 No actuator interfaces

OMNI-SENSE contains no trait, type, or function that could be used to command a physical actuator. The library is entirely on the observation side of the sense-decide-act loop.

### 4.6 Feature flags for compile-time footprint control

```toml
[features]
default = ["std"]
std = [...]
no-std = []
simulation = [...]
fusion = [...]
atmospherics = [...]
replay = [...]
python = ["dep:pyo3"]
camera = ["dep:omni-sense-drivers-camera"]
```

A wearable MCU firmware that uses only IMU and radar pulls in `omni-sense-core`, `omni-sense-physics`, `omni-sense-drivers-imu`, and `omni-sense-drivers-mmwave` with `no-std`. Nothing else is compiled.

---

## 5. Data Flow Detail

### 5.1 Active sensor pipeline (LiDAR example)

```
LidarDriver::poll_ranges()
  → Vec<RangeReturn>                          raw range + bearing + intensity
  → omni-sense-lidar::cluster()              group returns into object candidates
  → omni-sense-lidar::centroid()             compute per-cluster centroid + covariance
  → omni-sense-frames::transform_to_world()  apply sensor-to-world transform
  → omni-sense-atmospherics::degrade_range() atmospheric effective range check
  → DetectionEvent { position, velocity:None, modality: Lidar, ... }
```

### 5.2 Active sensor pipeline (mmWave radar example)

```
MmWaveDriver::poll()
  → ChirpFrame                               raw ADC samples
  → omni-sense-radar::FmcwProcessor::process_chirp_frame()
  → RangeDopplerMap                          2D range × velocity map
  → omni-sense-radar::cfar_detect_2d()
  → Vec<CfarDetection>                       peaks above noise floor
  → omni-sense-frames::transform_to_world()
  → DetectionEvent { position, velocity: Some(radial_velocity), modality: MmWaveRadar, ... }
```

### 5.3 IMU pipeline (SENTINEL-WEAR)

```
ImuDriver::poll_imu()
  → ImuSample { accel, gyro, magnetometer? }
  → omni-sense-physics::MadgwickFilter::step_imu()
  → UnitQuaternion                           node orientation
  → omni-sense-imu::MultiImuBodyPose::update()
  → BodyPose { per_segment_quaternion }      full body configuration
  → omni-sense-frames::BodyFrameStabilizer::stabilize()
  → StabilizedBodyFrame                      wearer-motion-subtracted frame
```

### 5.4 Acoustic full-echo pipeline (AEGIS-MESH)

```
AcousticDriver::poll_frame()
  → AcousticFrame { channels × samples }
  → omni-sense-physics::DelayAndSumBeamformer::beamform()
  → DirectionOfArrival                       bearing of sound source
  → omni-sense-physics::full_echo_profile()
  → EchoProfile { peaks, spectral_signature, multi_bounce }
  → omni-sense-acoustic::classify_material()
  → MaterialMatch { class, confidence }
  → DetectionEvent { position:estimated, modality: Acoustic, hints: MaterialClassification }
```

---

## 6. Cross-Project Dependency Map

| OMNI-SENSE crate | AEGIS-MESH | SENTINEL-WEAR | HALO-AD | TALON-MESH |
|---|---|---|---|---|
| omni-sense-core | ✓ | ✓ | ✓ | ✓ |
| omni-sense-physics | ✓ | ✓ | ✓ | ✓ |
| omni-sense-atmospherics | indoor only | — | ✓ | ✓ |
| omni-sense-frames | ✓ | ✓ | ✓ | ✓ |
| omni-sense-time | ✓ | ✓ | ✓ | ✓ |
| omni-sense-radar | ✓ | ✓ | ✓ sim | ✓ sim |
| omni-sense-lidar | ✓ | ✓ | ✓ sim | ✓ sim |
| omni-sense-event | ✓ | ✓ EV | ✓ sim | ✓ sim |
| omni-sense-acoustic | ✓ | ✓ | ✓ sim | ✓ sim |
| omni-sense-imu | node only | ✓ heavy | mast only | — |
| omni-sense-fusion | ✓ | ✓ | ✓ | ✓ |
| omni-sense-replay | ✓ | ✓ | ✓ | ✓ |
| omni-sense-drivers-camera | identity node | pendant opt-in | — | — |

---

## 7. Extending OMNI-SENSE

### Adding a new sensor modality

1. Define the trait in `omni-sense-core/src/traits/`.
2. Add sensor-specific processing in a new `omni-sense-<modality>/` crate.
3. Add driver trait + sim impl in a new `omni-sense-drivers-<modality>/` crate.
4. Add `From<YourSensorOutput> for DetectionEvent` in `omni-sense-core`.
5. Document in `docs/porting-guide.md`.

### Adding a vendor implementation

Vendor-specific drivers live in third-party crates. They implement the appropriate trait from `omni-sense-core` and depend on `omni-sense-core` only (not on processing crates, which live above the driver layer). The project then depends on both the trait processing crate and the vendor driver crate.

---

## 8. Testing Strategy

Every crate has:
- **Unit tests**: physics functions tested against known analytical solutions (documented in `docs/physics-references.md`).
- **Sim integration tests**: full pipeline from sim driver through DetectionEvent, verified against known scenario outputs.
- **Property tests**: using `proptest` for physics functions (commutativity of frame transforms, monotonicity of Beer-Lambert, etc.).
- **Benchmark**: `benches/` for FMCW processing, Kalman filter throughput, beamforming latency.

All tests are deterministic when using the `simulation` feature. Real hardware tests are optional and documented in `docs/porting-guide.md`.
