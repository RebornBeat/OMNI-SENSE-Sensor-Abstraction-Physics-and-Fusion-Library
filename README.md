# OMNI-SENSE — Sensor Abstraction, Physics, and Fusion Library

**A Rust-first, hardware-agnostic sensing library providing sensor traits, physics primitives, atmospheric models, multi-modal fusion substrates, coordinate frame management, and time synchronization for distributed perception research. OMNI-SENSE is the sensor-layer companion to PentaTrack — where PentaTrack handles predictive tracking logic, OMNI-SENSE handles sensor acquisition, physics-correct signal processing, atmospheric modeling, multi-modal fusion, and detection-event production.**

[![License: MIT](https://img.shields.io/badge/Code-MIT-blue.svg)](#license)
[![Language: Rust 2021](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](#requirements)
[![Status: Research](https://img.shields.io/badge/Status-Research%20%2F%20Educational-orange.svg)](#scope-and-disclaimers)
[![No-Std: Optional](https://img.shields.io/badge/no__std-Optional-green.svg)](#platform-support)

---

## Scope and Disclaimers

OMNI-SENSE is a **research and education** library. It is not certified for safety-critical or life-safety use. It is not a medical device. It is not weapons-grade software. It is a substrate for academic and hobbyist research into distributed perception systems.

**OMNI-SENSE explicitly does not contain:**

- Drivers, control loops, or interfaces for any actuator, weapon, or kinetic mechanism.
- Fire-control logic or any decision logic that closes a loop on real-world physical effects.
- Any code that issues commands to physical effectors of any kind.
- Export-controlled (ITAR / EAR / Wassenaar) technical data.
- Cryptography at controlled strengths.

**OMNI-SENSE is sensor-side only.** It produces `DetectionEvent` streams. What downstream code does with them is the consumer's responsibility, governed by the consumer's own ethics, legal, and architectural commitments. The `omni-sense-drivers-camera` crate, which provides the conventional camera trait, is opt-in via feature flag and is documented with privacy and policy considerations explicitly. Projects that do not depend on it have camera-free sensing by construction.

The library implements physics-correct signal processing using openly published academic and government-research literature: LOWTRAN/MODTRAN at unrestricted precision, published Mie/Rayleigh scattering theory, published thermal-blooming literature (Smith 1977, Gebhardt 1976), published radar-processing texts, published IMU-fusion literature (Madgwick 2010, Mahony 2008), published Kalman/EKF/UKF theory, published JPDA and MHT theory, published covariance-intersection theory. No proprietary or controlled parameters are embedded.

---

## Why OMNI-SENSE Exists

The four research projects in this ecosystem — AEGIS-MESH, SENTINEL-WEAR, HALO-AD, and TALON-MESH — share a substantial sensing substrate. Each must read raw bytes from sensors, apply physics-correct signal processing, compensate for atmospheric and environmental conditions, transform detections into a common coordinate frame, fuse multi-modal observations, and hand off structured detections to PentaTrack for predictive tracking.

Without a shared library, each project would re-implement this stack. The result would be:

- Duplicated bugs, four times the maintenance burden.
- Divergent physics implementations that prevent meaningful cross-project comparison.
- Per-project sensor abstractions that lock each project to its initial hardware choice.
- Re-derivation of well-established algorithms (Kalman filtering, FMCW range-Doppler processing, Madgwick IMU fusion, Beer-Lambert attenuation) in four codebases.

OMNI-SENSE provides the shared substrate. This mirrors the relationship between LLVM and Clang, between the RustCrypto `digest` trait and individual hash crates, and between PentaTrack and the projects that consume it. **Common substrate; specialized consumers.**

---

## Relationship to PentaTrack

OMNI-SENSE and PentaTrack are paired layers. They are designed to compose, but neither depends on the other:

```
┌────────────────────────────────────────────────────────────┐
│  Application Layer                                         │
│  AEGIS-MESH | SENTINEL-WEAR | HALO-AD | TALON-MESH         │
│                                                            │
│  Application-specific decision, coordination, alerting,    │
│  visualization, scenario logic, policy enforcement.        │
└────────────────────────────────────────────────────────────┘
                          ▲
                          │ predicted positions, drift,
                          │ anomalies, intercept zones
                          │
┌────────────────────────────────────────────────────────────┐
│  PentaTrack — Predictive Tracking                          │
│                                                            │
│  Multi-center prediction, velocity weighting, drift        │
│  analysis, object-type awareness, homing intercept.        │
│                                                            │
│  Consumes: DetectionEvent stream                           │
│  Produces: CenterNode tree, predicted positions,           │
│            drift metrics, anomaly flags                    │
└────────────────────────────────────────────────────────────┘
                          ▲
                          │ DetectionEvent stream
                          │
┌────────────────────────────────────────────────────────────┐
│  OMNI-SENSE — Sensor Abstraction, Physics, Fusion          │
│                                                            │
│  Sensor traits, physics primitives, atmospheric models,    │
│  multi-modal fusion, frame transforms, time sync.          │
│                                                            │
│  Consumes: raw sensor bytes / vendor SDKs                  │
│  Produces: DetectionEvent stream with uncertainty,         │
│            timestamps, frame metadata, classification      │
│            hints                                           │
└────────────────────────────────────────────────────────────┘
                          ▲
                          │ raw bytes, vendor protocols
                          │
┌────────────────────────────────────────────────────────────┐
│  Hardware Layer                                            │
│  mmWave chipsets, LiDAR units, event cameras, IMUs,        │
│  microphone arrays, environmental sensors, PIR, ToF.       │
└────────────────────────────────────────────────────────────┘
```

**The contract between the sensor layer and the tracking layer is the `DetectionEvent` structure.** OMNI-SENSE produces it; PentaTrack consumes it. Neither layer needs to know about the other's internals. PentaTrack can be used with any detection source that produces `DetectionEvent`; OMNI-SENSE can feed any tracking layer that consumes `DetectionEvent`.

A `DetectionEvent` carries:

- Position estimate with full 3x3 covariance.
- Velocity estimate with full 3x3 covariance, when the producing sensor supports velocity readout.
- Modality tag identifying the producing sensor type (radar, lidar, acoustic, event, imu-derived, etc.).
- Source sensor identifier.
- Source frame identifier and pose at observation time.
- Timestamp (monotonic event time and wall-clock time).
- Classification hints: modality-specific features (Doppler signature, point-cloud cluster characteristics, acoustic spectral profile, etc.).
- Confidence score in [0, 1].
- Optional raw-data reference for deferred processing or replay.

---

## Architecture

OMNI-SENSE is organized as a Cargo workspace of focused crates. Each crate is independently versioned and feature-flagged so consumers import only what they need.

### Workspace structure

```
omni-sense/                              (workspace root)
├── omni-sense-core/                     (core types, DetectionEvent, traits)
├── omni-sense-physics/                  (ToF, Doppler, beams, full-echo)
├── omni-sense-atmospherics/             (Beer-Lambert, blooming, turbulence)
├── omni-sense-frames/                   (coordinate frames, transforms)
├── omni-sense-time/                     (synchronization, timestamping)
├── omni-sense-fusion/                   (Kalman, EKF, UKF, JPDA, MHT, CI)
├── omni-sense-imu/                      (Madgwick, Mahony, bias correction)
├── omni-sense-acoustic/                 (beamforming, full-echo profiling)
├── omni-sense-radar/                    (FMCW range-Doppler, CFAR)
├── omni-sense-lidar/                    (point-cloud processing, ToF)
├── omni-sense-event/                    (event-camera processing)
├── omni-sense-drivers-mmwave/           (mmWave radar trait + sim impl)
├── omni-sense-drivers-lidar/            (LiDAR trait + sim impl)
├── omni-sense-drivers-event/            (event-camera trait + sim impl)
├── omni-sense-drivers-acoustic/         (microphone array trait + sim impl)
├── omni-sense-drivers-imu/              (IMU trait + sim impl)
├── omni-sense-drivers-pir/              (PIR trait + sim impl)
├── omni-sense-drivers-tof/              (1D ToF trait + sim impl)
├── omni-sense-drivers-env/              (environmental sensor trait + sim)
├── omni-sense-drivers-camera/           (camera trait — opt-in feature flag)
├── omni-sense-replay/                   (recording and replay infrastructure)
├── omni-sense-bindings-python/          (PyO3 bindings for analysis tooling)
└── omni-sense-prelude/                  (re-export crate for ergonomic imports)
```

### Why workspace crates

- **Compile times**: consumers building a wearable that uses only IMU and mmWave should not pay compile cost for unused LiDAR and acoustic processing.
- **Feature isolation**: the camera crate's opt-in posture is enforced at the dependency level, not at runtime.
- **Vendor flexibility**: vendor-specific implementations can live in third-party crates that depend on the trait crates without forcing the trait crates to grow.
- **Test isolation**: each crate is independently testable, with simulation implementations as test substrates.

---

## The Sensor Trait Hierarchy

OMNI-SENSE defines a hierarchy of sensor traits in `omni-sense-core`. Vendor implementations live in driver crates (`omni-sense-drivers-*`) and implement these traits. Application code depends on the traits, not on specific vendor implementations.

### Base trait

```rust
pub trait Sensor: Send {
    fn id(&self) -> SensorId;
    fn modality(&self) -> Modality;
    fn frame(&self) -> FrameId;
    fn pose(&self) -> Pose;
    fn health(&self) -> SensorHealth;
    fn initialize(&mut self) -> Result<(), SensorError>;
    fn shutdown(&mut self) -> Result<(), SensorError>;
}
```

Every sensor reports its identity, modality, the coordinate frame it observes in, its pose relative to its parent frame, and health status. Initialization and shutdown are explicit so that resource lifecycle is visible.

### Specialized traits by modality

```rust
pub trait RangingSensor: Sensor {
    fn poll_ranges(&mut self) -> Result<Vec<RangeReturn>, SensorError>;
    fn nominal_range(&self) -> Meters;
    fn beam_pattern(&self) -> BeamPattern;
    fn range_resolution(&self) -> Meters;
}

pub trait VelocitySensor: Sensor {
    fn poll_velocities(&mut self) -> Result<Vec<VelocityReturn>, SensorError>;
    fn velocity_resolution(&self) -> MetersPerSecond;
    fn unambiguous_velocity_range(&self) -> (MetersPerSecond, MetersPerSecond);
}

pub trait EventSensor: Sensor {
    fn poll_events(&mut self) -> Result<Vec<PixelEvent>, SensorError>;
    fn resolution(&self) -> (u32, u32);
    fn contrast_threshold(&self) -> f32;
    fn dynamic_range_db(&self) -> f32;
}

pub trait AcousticSensor: Sensor {
    fn poll_frame(&mut self) -> Result<AcousticFrame, SensorError>;
    fn array_geometry(&self) -> ArrayGeometry;
    fn sample_rate(&self) -> u32;
    fn channel_count(&self) -> u32;
}

pub trait ImuSensor: Sensor {
    fn poll_imu(&mut self) -> Result<ImuSample, SensorError>;
    fn accelerometer_range_g(&self) -> f32;
    fn gyroscope_range_dps(&self) -> f32;
    fn has_magnetometer(&self) -> bool;
    fn sample_rate_hz(&self) -> f32;
}

pub trait BinarySensor: Sensor {
    fn poll_state(&mut self) -> Result<BinaryEvent, SensorError>;
    fn debounce_duration(&self) -> Duration;
}

pub trait EnvironmentalSensor: Sensor {
    fn poll_environment(&mut self) -> Result<EnvironmentSample, SensorError>;
    fn measured_quantities(&self) -> &[EnvironmentQuantity];
}

pub trait ImagingSensor: Sensor {
    // Conventional camera. Opt-in via feature flag.
    // Documented with privacy and policy considerations.
    fn poll_frame(&mut self) -> Result<ImageFrame, SensorError>;
    fn resolution(&self) -> (u32, u32);
    fn frame_rate_hz(&self) -> f32;
}
```

### Composite traits

Some sensors expose multiple capabilities. mmWave radar provides both range and velocity. Composite traits express this:

```rust
pub trait MmWaveRadar: RangingSensor + VelocitySensor {
    fn fmcw_config(&self) -> FmcwConfig;
    fn antenna_pattern(&self) -> AntennaPattern;
}

pub trait LidarWithIntensity: RangingSensor {
    fn poll_with_intensity(&mut self) -> Result<Vec<IntensityReturn>, SensorError>;
}

pub trait ScanningLidar: RangingSensor {
    fn scan_period(&self) -> Duration;
    fn scan_phase(&self) -> f32;  // [0, 1] within current scan
}

pub trait FlashLidar: RangingSensor {
    // Full-frame illumination, no scan cycle.
}

pub trait SolidStateLidar: RangingSensor {
    // No mechanical scan. May still have electronic steering.
    fn steering_angle(&self) -> Option<(f32, f32)>;
}
```

### Simulation implementations

Every trait has a corresponding simulation implementation under the `simulated` feature flag. These are first-class citizens — they generate physically-correct synthetic data based on configurable scenarios. Every consuming project can run end-to-end on simulated sensors before any hardware is attached.

```rust
use omni_sense::drivers::lidar::SimulatedScanningLidar;

let mut lidar = SimulatedScanningLidar::new()
    .with_scan_period(Duration::from_millis(100))
    .with_max_range(50.0)
    .with_noise_model(NoiseModel::gaussian(0.02))
    .with_scenario(scenario_path)
    .build();

lidar.initialize()?;
let returns = lidar.poll_ranges()?;
```

The simulation framework feeds the same data structures the real drivers produce. Switching from simulated to real hardware is a one-line change.

---

## Physics Primitives

The `omni-sense-physics` crate provides the math that every sensor pipeline needs. These primitives are pure functions — no state, no side effects, fully testable.

### Time-of-flight

```rust
pub fn time_of_flight_distance(
    pulse_time: Nanoseconds,
    return_time: Nanoseconds,
    medium: PropagationMedium,
) -> Meters;
```

Handles light (speed of light in air, with refractive-index correction for non-vacuum) and sound (speed of sound, with temperature and humidity correction). The `PropagationMedium` enum encodes the relevant correction factors.

### Doppler velocity

```rust
pub fn doppler_radial_velocity(
    transmitted_freq: Hertz,
    received_freq: Hertz,
    wave_speed: MetersPerSecond,
) -> MetersPerSecond;
```

Returns the radial component of velocity along the sensor-target line, positive for approach. The `omni-sense-radar` crate composes this with FMCW chirp processing to produce a full range-Doppler map.

### FMCW range-Doppler processing

```rust
pub struct FmcwProcessor {
    config: FmcwConfig,
    window: WindowFunction,
}

impl FmcwProcessor {
    pub fn process_chirp_frame(
        &self,
        chirp: &ChirpFrame,
    ) -> RangeDopplerMap;
    
    pub fn detect_targets(
        &self,
        map: &RangeDopplerMap,
        cfar: &CfarConfig,
    ) -> Vec<RadarDetection>;
}
```

This is the workhorse for mmWave radar. Range-Doppler processing produces a 2D map indexed by range bin and velocity bin; CFAR detection identifies peaks above the local noise floor. Every project that uses mmWave consumes this primitive.

### CFAR detection

```rust
pub enum CfarVariant {
    CellAveraging { num_training: usize, num_guard: usize },
    OrderStatistic { rank: usize, num_training: usize, num_guard: usize },
    AdaptiveCfar { adaptation_rate: f32 },
}

pub fn cfar_detect_2d(
    range_doppler_map: &RangeDopplerMap,
    config: &CfarConfig,
) -> Vec<CfarDetection>;
```

Three CFAR variants: standard cell-averaging, order-statistic (better in clutter-edge scenarios), and adaptive (adjusts threshold based on local statistics). The choice is configurable per scenario.

### Beam divergence and propagation

```rust
pub fn gaussian_beam_radius(
    initial_radius: Meters,
    distance: Meters,
    divergence_half_angle: Radians,
    beam_quality: f32,  // M^2 factor; 1.0 for ideal beam
) -> Meters;

pub fn diffraction_limited_divergence(
    wavelength: Meters,
    waist_radius: Meters,
) -> Radians;

pub fn beam_power_density_at_range(
    total_power: Watts,
    beam_radius: Meters,
) -> WattsPerSquareMeter;
```

For LiDAR effective range computation, for HALO-AD's coverage geometry, for any sensor whose performance depends on beam parameters.

### Full-echo acoustic profiling

```rust
pub struct EchoProfile {
    pub peaks: Vec<EchoPeak>,
    pub decay_envelope: Vec<f32>,
    pub spectral_signature: Spectrum,
    pub multi_bounce_structure: MultiBounce,
}

pub fn full_echo_profile(
    waveform: &[f32],
    sample_rate: u32,
    config: &EchoConfig,
) -> EchoProfile;

pub fn classify_material(
    profile: &EchoProfile,
    library: &MaterialLibrary,
) -> Vec<MaterialMatch>;
```

This is the substrate for AEGIS-MESH's material classification (glass break vs. ceramic break vs. plastic crack) and for any acoustic-event classification system. The math is identical across applications.

### IMU integration

```rust
pub fn integrate_orientation(
    current: UnitQuaternion,
    angular_velocity: AngularVelocity,
    dt: Seconds,
) -> UnitQuaternion;

pub fn integrate_position(
    current_position: Vector3,
    current_velocity: Vector3,
    accel_world: Vector3,
    dt: Seconds,
) -> (Vector3, Vector3);

pub fn gravity_compensate(
    accel_body: Vector3,
    orientation: UnitQuaternion,
    gravity: Vector3,  // Typically (0, 0, -9.81) m/s^2
) -> Vector3;

pub fn estimate_imu_biases(
    samples: &[ImuSample],
    duration: Duration,
) -> ImuBiases;
```

The pipeline: estimate biases during a known-stationary period, integrate orientation continuously, gravity-compensate accelerometer to recover linear acceleration, integrate that to recover velocity and position. Position estimates drift over time (dead reckoning); they need external aiding (radar, LiDAR, GPS) for long-term accuracy. The library provides the primitives; applications choose the aiding strategy.

### Madgwick and Mahony filters

```rust
pub struct MadgwickFilter {
    pub state: UnitQuaternion,
    pub beta: f32,  // Filter gain
}

impl MadgwickFilter {
    pub fn step_imu(
        &mut self,
        sample: ImuSample,
        dt: Seconds,
    ) -> UnitQuaternion;
    
    pub fn step_marg(  // Magnetometer-aided
        &mut self,
        sample: ImuSample,
        magnetometer: Vector3,
        dt: Seconds,
    ) -> UnitQuaternion;
}

pub struct MahonyFilter {
    pub state: UnitQuaternion,
    pub kp: f32,  // Proportional gain
    pub ki: f32,  // Integral gain
    pub bias_estimate: Vector3,
}

impl MahonyFilter {
    pub fn step_imu(
        &mut self,
        sample: ImuSample,
        dt: Seconds,
    ) -> UnitQuaternion;
    
    pub fn step_marg(
        &mut self,
        sample: ImuSample,
        magnetometer: Vector3,
        dt: Seconds,
    ) -> UnitQuaternion;
}
```

Both filters are public-domain, well-characterized in published literature, and computationally efficient. Madgwick is preferred for low-power applications; Mahony is preferred when bias estimation matters. Applications can run multiple filters in parallel and compare.

### Beamforming

```rust
pub struct DelayAndSumBeamformer {
    pub array_geometry: ArrayGeometry,
}

impl DelayAndSumBeamformer {
    pub fn beamform_direction(
        &self,
        frame: &AcousticFrame,
        azimuth: Radians,
        elevation: Radians,
    ) -> Vec<f32>;
}

pub struct MusicBeamformer {
    pub array_geometry: ArrayGeometry,
    pub source_count: usize,
}

impl MusicBeamformer {
    pub fn estimate_directions(
        &self,
        frame: &AcousticFrame,
    ) -> Vec<Direction>;
}

pub struct EsritEstimator { ... }
```

Three beamformers: delay-and-sum (simple, robust, low resolution), MUSIC (high resolution, requires source count), ESPRIT (closed-form, no spectral search). AEGIS-MESH and SENTINEL-WEAR use these for direction-of-arrival of acoustic events.

---

## Atmospheric and Environmental Models

The `omni-sense-atmospherics` crate provides physics-correct models for how the medium between sensor and target affects propagation.

### Beer-Lambert transmission

```rust
pub fn beer_lambert_transmission(
    extinction_coefficient: PerMeter,
    path_length: Meters,
) -> Transmission;  // [0.0, 1.0]

pub fn double_pass_transmission(
    extinction_coefficient: PerMeter,
    path_length: Meters,
) -> Transmission;  // For active sensors (LiDAR, radar)
```

The double-pass form is critical for active sensors where the signal traverses the path twice (emitter → target → receiver).

### Atmospheric profiles

```rust
pub enum AtmosphericProfile {
    Clear,
    Hazy { visibility_km: f32 },
    LightFog { visibility_m: f32 },
    HeavyFog { visibility_m: f32 },
    LightRain { rate_mm_hr: f32 },
    HeavyRain { rate_mm_hr: f32 },
    LightSnow,
    HeavySnow,
    Smoke { density: f32 },
    Dust { density: f32 },
    Custom { extinction_per_wavelength: BTreeMap<Meters, PerMeter> },
}

impl AtmosphericProfile {
    pub fn extinction_at_wavelength(&self, wavelength: Meters) -> PerMeter;
    pub fn effective_range(&self, sensor: &impl RangingSensor, threshold: f32) -> Meters;
}
```

Each profile encodes the physically-correct extinction coefficient as a function of wavelength. Visible-band sensors fail in fog; near-IR (1.5 μm) penetrates haze better; longer IR (10 μm) penetrates further still; mmWave radar is essentially unaffected by atmospheric water.

### Thermal blooming

```rust
pub struct ThermalBloomingModel {
    pub absorption_coefficient: PerMeter,
    pub wind_speed: MetersPerSecond,
    pub wind_direction: Radians,
}

impl ThermalBloomingModel {
    pub fn effective_beam_radius(
        &self,
        nominal_radius: Meters,
        beam_power: Watts,
        path_length: Meters,
        dwell_time: Seconds,
    ) -> Meters;
}
```

For HALO-AD's high-power coverage analysis. The model is parameterized to remain at academic precision; no controlled-threshold parameters are embedded.

### Turbulence modeling

```rust
pub struct TurbulenceProfile {
    pub structure_constant_per_altitude: Vec<(Meters, f32)>,
}

impl TurbulenceProfile {
    pub fn fried_parameter(
        &self,
        wavelength: Meters,
        path_length: Meters,
        elevation_angle: Radians,
    ) -> Meters;
    
    pub fn beam_spread(
        &self,
        wavelength: Meters,
        path_length: Meters,
        aperture: Meters,
    ) -> Radians;
}
```

For long-range optical sensing. Fried parameter `r₀` characterizes turbulence strength; beam spread combines turbulence with diffraction.

### Adaptive optics modeling

```rust
pub struct AdaptiveOpticsCorrection {
    pub control_bandwidth: Hertz,
    pub correction_quality: f32,  // [0.0, 1.0]
}

impl AdaptiveOpticsCorrection {
    pub fn apply_to_beam(
        &self,
        nominal_beam: Beam,
        turbulence: &TurbulenceProfile,
    ) -> Beam;
}
```

Optional correction layer for HALO-AD-class scenarios where adaptive optics is in scope. Models the bandwidth limitation and residual error of practical AO systems.

---

## Coordinate Frames and Transforms

The `omni-sense-frames` crate provides strongly-typed coordinate frame management.

### Frame types

```rust
pub struct FrameId(String);  // Unique identifier

pub trait Frame {
    fn id(&self) -> &FrameId;
    fn parent(&self) -> Option<&FrameId>;
}

// Built-in frames
pub struct WorldFrame;
pub struct PlatformFrame { id: FrameId, parent: FrameId }  // Vehicle, mast
pub struct BodyFrame { id: FrameId }  // For SENTINEL-WEAR torso
pub struct SensorFrame { id: FrameId, parent: FrameId }
pub struct StabilizedFrame { id: FrameId, parent: FrameId, stabilization: StabilizationMode }
```

### Transforms

```rust
pub struct Transform {
    pub from: FrameId,
    pub to: FrameId,
    pub rotation: UnitQuaternion,
    pub translation: Vector3,
    pub timestamp: Timestamp,
    pub uncertainty: TransformCovariance,
}

impl Transform {
    pub fn apply(&self, point: Point3) -> Point3;
    pub fn apply_with_uncertainty(&self, point_with_cov: PointWithCovariance) -> PointWithCovariance;
    pub fn invert(&self) -> Transform;
    pub fn compose(&self, other: &Transform) -> Result<Transform, TransformError>;
}
```

Transforms carry uncertainty. Composition propagates uncertainty correctly using the linearized formula.

### Frame tree

```rust
pub struct FrameTree {
    transforms: HashMap<(FrameId, FrameId), Transform>,
    hierarchy: HashMap<FrameId, FrameId>,
}

impl FrameTree {
    pub fn add_transform(&mut self, transform: Transform);
    pub fn lookup(&self, from: &FrameId, to: &FrameId, at_time: Timestamp) -> Option<Transform>;
    pub fn lookup_with_extrapolation(&self, from: &FrameId, to: &FrameId, at_time: Timestamp) -> Option<Transform>;
}
```

A frame tree maintains all known transforms and supports lookups at arbitrary timestamps with linear extrapolation. AEGIS-MESH uses this to maintain node-to-home transforms; SENTINEL-WEAR uses it to maintain node-to-body and body-to-world transforms; HALO-AD uses it to maintain mast-to-zone transforms.

### Auto-calibration via walking trilateration

```rust
pub struct WalkThroughCalibrator {
    nodes: Vec<NodeId>,
    voxel_resolution: f32,
}

impl WalkThroughCalibrator {
    pub fn record_user_position(&mut self, position: Point3, timestamp: Timestamp);
    pub fn record_node_detection(&mut self, node_id: NodeId, detection: RangeReturn);
    pub fn solve(&self) -> HashMap<NodeId, NodePosition>;
}
```

The same algorithm applies to AEGIS-MESH (walking through a home to locate ceiling-mounted nodes) and to SENTINEL-WEAR (walking with a wearable mesh to estimate body geometry). Centralizing it means fixing it once.

### Body-frame stabilization

```rust
pub enum StabilizationMode {
    World,           // Full world-frame, all wearer rotation subtracted
    Translational,   // Wearer rotation subtracted, translation preserved (default for SENTINEL-WEAR)
    None,            // Raw torso frame
}

pub struct BodyFrameStabilizer {
    pub mode: StabilizationMode,
    pub torso_imu_id: SensorId,
}

impl BodyFrameStabilizer {
    pub fn stabilize_observation(
        &self,
        observation: PointInFrame,
        torso_imu_state: &ImuState,
    ) -> PointInFrame;
}
```

This is the centerpiece of SENTINEL-WEAR's body-frame architecture. By implementing it once in OMNI-SENSE, it becomes available to every project that wants stabilized observations.

---

## Time Synchronization

The `omni-sense-time` crate provides primitives for synchronized timestamping across distributed sensors.

### Monotonic timestamps

```rust
pub struct Timestamp {
    pub monotonic_ns: u64,    // Monotonic counter, never goes backward
    pub wall_clock_ns: i64,   // System wall clock
}

impl Timestamp {
    pub fn now() -> Timestamp;
    pub fn from_monotonic_ns(ns: u64) -> Timestamp;
}
```

Sensor events use monotonic timestamps for ordering; wall-clock is recorded for log correlation but not used for ordering decisions.

### PTP-style offset estimation

```rust
pub struct ClockOffsetEstimator {
    pub local_clock: ClockId,
    pub remote_clock: ClockId,
}

impl ClockOffsetEstimator {
    pub fn record_exchange(&mut self, t1: Timestamp, t2: Timestamp, t3: Timestamp, t4: Timestamp);
    pub fn current_offset(&self) -> ClockOffset;
    pub fn current_drift_rate(&self) -> f32;
}
```

For mesh networks (AEGIS-MESH multi-node, SENTINEL-WEAR body-area, HALO-AD multi-mast). The estimator tracks both offset and drift rate, enabling sub-millisecond synchronization on local networks.

### Timestamp arithmetic

```rust
impl Timestamp {
    pub fn duration_since(&self, earlier: Timestamp) -> Duration;
    pub fn add_duration(&self, d: Duration) -> Timestamp;
    pub fn aligns_with(&self, other: Timestamp, tolerance: Duration) -> bool;
}
```

For matching observations across sensors with imperfect synchronization.

---

## Fusion Substrates

The `omni-sense-fusion` crate provides multi-sensor and multi-target fusion algorithms. Each algorithm is generic over state and measurement types so it can be specialized to project-specific data.

### Kalman family

```rust
pub struct KalmanFilter<const N: usize, const M: usize> {
    pub state: SVector<f32, N>,
    pub covariance: SMatrix<f32, N, N>,
    pub state_transition: SMatrix<f32, N, N>,
    pub measurement: SMatrix<f32, M, N>,
    pub process_noise: SMatrix<f32, N, N>,
    pub measurement_noise: SMatrix<f32, M, M>,
}

impl<const N: usize, const M: usize> KalmanFilter<N, M> {
    pub fn predict(&mut self, dt: Seconds);
    pub fn update(&mut self, measurement: SVector<f32, M>);
    pub fn current_state(&self) -> &SVector<f32, N>;
}

pub struct ExtendedKalmanFilter<const N: usize, const M: usize, F: NonlinearModel> { ... }
pub struct UnscentedKalmanFilter<const N: usize, const M: usize, F: NonlinearModel> { ... }
```

Standard Kalman for linear systems; Extended for nonlinear systems with Jacobians; Unscented for highly nonlinear systems where Jacobians are inaccurate. State and measurement dimensions are statically sized; the underlying linear algebra uses `nalgebra`.

### Particle filter

```rust
pub struct ParticleFilter<S, M, F: ParticleModel<S, M>> {
    pub particles: Vec<Particle<S>>,
    pub model: F,
}

impl<S, M, F: ParticleModel<S, M>> ParticleFilter<S, M, F> {
    pub fn predict(&mut self, dt: Seconds);
    pub fn update(&mut self, measurement: M);
    pub fn estimate_state(&self) -> S;
    pub fn effective_sample_size(&self) -> f32;
    pub fn resample(&mut self);
}
```

For non-Gaussian state distributions where Kalman variants are inappropriate. Multi-modal posteriors common in initial-acquisition and ambiguity scenarios.

### Joint Probabilistic Data Association

```rust
pub struct JpdaTracker<S, M> {
    pub tracks: Vec<Track<S>>,
    pub gate_threshold: f32,
}

impl<S, M> JpdaTracker<S, M> {
    pub fn update_with_measurements(&mut self, measurements: Vec<M>, dt: Seconds);
}
```

For multi-target scenarios where each measurement might come from any track. Computes association probabilities marginalized over all hypotheses, then updates each track with a weighted sum of measurements.

### Multi-Hypothesis Tracking

```rust
pub struct MhtTracker<S, M> {
    pub hypothesis_tree: HypothesisTree<S>,
    pub pruning_threshold: f32,
    pub gating_threshold: f32,
}

impl<S, M> MhtTracker<S, M> {
    pub fn update_with_measurements(&mut self, measurements: Vec<M>, dt: Seconds);
    pub fn best_hypothesis(&self) -> &Hypothesis<S>;
}
```

For scenarios where holding multiple association hypotheses is preferable to committing prematurely. More expensive than JPDA but more accurate when the data is genuinely ambiguous.

### Covariance intersection

```rust
pub fn covariance_intersection(
    estimates: &[(SVector<f32, N>, SMatrix<f32, N, N>)],
) -> (SVector<f32, N>, SMatrix<f32, N, N>);
```

For fusing estimates with unknown correlation. Critical for distributed multi-node fusion where cross-correlations cannot be tracked. Used in HALO-AD multi-mast fusion, AEGIS-MESH multi-node fusion, SENTINEL-WEAR cross-node fusion.

### Track-level fusion

```rust
pub struct TrackLevelFuser {
    pub association_strategy: AssociationStrategy,
    pub fusion_method: FusionMethod,
}

impl TrackLevelFuser {
    pub fn fuse_node_tracks(
        &self,
        node_tracks: &[(NodeId, Vec<Track>)],
    ) -> Vec<FusedTrack>;
}
```

The high-level fusion interface used by AEGIS-MESH, SENTINEL-WEAR, and HALO-AD. Combines track-level data from multiple nodes into a unified track set.

---

## Replay and Logging Infrastructure

The `omni-sense-replay` crate provides reproducibility infrastructure.

### Recording

```rust
pub struct SensorRecorder {
    pub output_path: PathBuf,
    pub format: RecordingFormat,  // MessagePack, CBOR, custom binary
}

impl SensorRecorder {
    pub fn record_event<T: Serialize>(&mut self, event: T) -> Result<(), RecordError>;
    pub fn record_detection(&mut self, detection: DetectionEvent);
    pub fn record_imu(&mut self, sample: ImuSample);
    // ... per-modality methods
}
```

### Replay

```rust
pub struct SensorReplay {
    pub input_path: PathBuf,
}

impl SensorReplay {
    pub fn replay_realtime(&self) -> impl Iterator<Item = ReplayEvent>;
    pub fn replay_max_speed(&self) -> impl Iterator<Item = ReplayEvent>;
    pub fn seek_to(&mut self, timestamp: Timestamp);
}
```

Recorded scenarios are reproducible. For research, this is essential — every claimed result can be verified by replaying the recorded scenario.

### Cross-project format

A standard recording format is shared across projects. AEGIS-MESH recordings can be replayed in HALO-AD's analysis tools; SENTINEL-WEAR recordings can be analyzed with the same tooling as TALON-MESH recordings. This enables comparative research that would otherwise require building separate tooling for each project.

---

## Platform Support

OMNI-SENSE supports multiple deployment targets via feature flags:

```toml
[dependencies]
omni-sense = { version = "0.1", features = ["std", "fusion", "atmospherics"] }
```

### Feature flags

- `std` (default): standard library support, threading, file I/O, networking.
- `no-std`: bare-metal embedded support. No allocator required for core types; allocator-required features are gated.
- `simulation`: simulation implementations of all sensor traits.
- `fusion`: Kalman, EKF, UKF, JPDA, MHT, particle filter.
- `atmospherics`: atmospheric and environmental models.
- `replay`: recording and replay infrastructure.
- `python`: PyO3 bindings (requires Python 3.10+).
- `camera`: opt-in conventional camera trait. Off by default.

### Platform examples

- **Embedded MCU (Cortex-M, RISC-V)**: `no-std`, `simulation` off, `fusion` (Kalman only), `atmospherics` off, custom drivers via traits.
- **Edge controller (Jetson, RPi)**: `std`, `simulation` for testing, `fusion`, `atmospherics`, project-specific driver crates.
- **Workstation (research)**: all features.
- **Cloud analytics**: `replay` + `python` for Jupyter analysis.

---

## Quick Start

### Minimal use (just sensor abstraction)

```rust
use omni_sense::prelude::*;
use omni_sense_drivers_lidar::SimulatedScanningLidar;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lidar = SimulatedScanningLidar::new()
        .with_scan_period(Duration::from_millis(100))
        .with_max_range(50.0)
        .build();
    
    lidar.initialize()?;
    
    loop {
        let returns = lidar.poll_ranges()?;
        for r in returns {
            println!("Range: {:?} at bearing {:?}", r.range, r.bearing);
        }
    }
}
```

### Full pipeline (sensor → fusion → tracking)

```rust
use omni_sense::prelude::*;
use omni_sense_fusion::TrackLevelFuser;
use pentatrack::PentaTracker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up sensors
    let mut radar = SimulatedMmWaveRadar::new().build();
    let mut lidar = SimulatedScanningLidar::new().build();
    let mut imu = SimulatedImu::new().build();
    
    radar.initialize()?;
    lidar.initialize()?;
    imu.initialize()?;
    
    // Set up fusion
    let mut fuser = TrackLevelFuser::new()
        .with_association_strategy(AssociationStrategy::JPDA)
        .with_fusion_method(FusionMethod::CovarianceIntersection)
        .build();
    
    // Set up tracker
    let mut tracker = PentaTracker::new(TrackingConfig::default());
    
    loop {
        // Sensor polling
        let radar_dets = radar.poll_velocities()?
            .into_iter()
            .map(DetectionEvent::from)
            .collect::<Vec<_>>();
        
        let lidar_dets = lidar.poll_ranges()?
            .into_iter()
            .map(DetectionEvent::from)
            .collect::<Vec<_>>();
        
        let imu_state = imu.poll_imu()?;
        
        // Fusion
        let detections = fuser.fuse(vec![
            (radar.id(), radar_dets),
            (lidar.id(), lidar_dets),
        ])?;
        
        // Tracking
        for det in detections {
            tracker.update(det);
        }
        
        // Read predictions
        for track in tracker.active_tracks() {
            println!("Track {}: {:?}", track.id, track.predicted_position());
        }
    }
}
```

---

## Repository Layout

```
omni-sense/
├── Cargo.toml                                   (workspace root)
├── README.md                                    (this file)
├── LICENSE
├── CHANGELOG.md
├── CONTRIBUTING.md
├── rust-toolchain.toml
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── docs.yml
├── docs/
│   ├── architecture.md
│   ├── design-principles.md
│   ├── physics-references.md
│   ├── coordinate-frames.md
│   ├── time-sync.md
│   ├── fusion-algorithms.md
│   ├── camera-policy.md
│   └── porting-guide.md
├── crates/
│   ├── omni-sense-core/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── sensor.rs
│   │   │   ├── modality.rs
│   │   │   ├── detection_event.rs
│   │   │   ├── error.rs
│   │   │   ├── health.rs
│   │   │   ├── pose.rs
│   │   │   └── traits/
│   │   │       ├── mod.rs
│   │   │       ├── ranging.rs
│   │   │       ├── velocity.rs
│   │   │       ├── event.rs
│   │   │       ├── acoustic.rs
│   │   │       ├── imu.rs
│   │   │       ├── binary.rs
│   │   │       ├── environmental.rs
│   │   │       └── imaging.rs
│   │   └── tests/
│   │       ├── trait_compliance.rs
│   │       └── error_propagation.rs
│   ├── omni-sense-physics/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── time_of_flight.rs
│   │   │   ├── doppler.rs
│   │   │   ├── beam.rs
│   │   │   ├── full_echo.rs
│   │   │   ├── imu_integration.rs
│   │   │   ├── madgwick.rs
│   │   │   ├── mahony.rs
│   │   │   └── beamforming/
│   │   │       ├── mod.rs
│   │   │       ├── delay_and_sum.rs
│   │   │       ├── music.rs
│   │   │       └── esprit.rs
│   │   └── tests/
│   │       ├── tof_correctness.rs
│   │       ├── doppler_correctness.rs
│   │       ├── imu_filter_convergence.rs
│   │       └── beamform_accuracy.rs
│   ├── omni-sense-atmospherics/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── beer_lambert.rs
│   │   │   ├── profiles.rs
│   │   │   ├── thermal_blooming.rs
│   │   │   ├── turbulence.rs
│   │   │   └── adaptive_optics.rs
│   │   └── tests/
│   │       ├── transmission_correctness.rs
│   │       ├── blooming_correctness.rs
│   │       └── turbulence_correctness.rs
│   ├── omni-sense-frames/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── frame.rs
│   │   │   ├── transform.rs
│   │   │   ├── frame_tree.rs
│   │   │   ├── stabilizer.rs
│   │   │   ├── calibration.rs
│   │   │   └── uncertainty_propagation.rs
│   │   └── tests/
│   │       ├── transform_composition.rs
│   │       ├── frame_tree_lookup.rs
│   │       └── calibration_convergence.rs
│   ├── omni-sense-time/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── timestamp.rs
│   │   │   ├── clock_offset.rs
│   │   │   ├── ptp_style_estimator.rs
│   │   │   └── duration.rs
│   │   └── tests/
│   │       ├── monotonicity.rs
│   │       └── offset_estimator_convergence.rs
│   ├── omni-sense-radar/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── fmcw.rs
│   │   │   ├── range_doppler_map.rs
│   │   │   ├── cfar.rs
│   │   │   └── radar_detection.rs
│   │   └── tests/
│   │       ├── fmcw_processing.rs
│   │       └── cfar_detection.rs
│   ├── omni-sense-lidar/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── point_cloud.rs
│   │   │   ├── scan_processing.rs
│   │   │   └── intensity.rs
│   │   └── tests/
│   ├── omni-sense-event/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── event_processing.rs
│   │   │   ├── trajectory_extraction.rs
│   │   │   └── motion_event.rs
│   │   └── tests/
│   ├── omni-sense-acoustic/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── frame.rs
│   │   │   ├── beamforming_pipeline.rs
│   │   │   ├── full_echo_pipeline.rs
│   │   │   └── classification.rs
│   │   └── tests/
│   ├── omni-sense-imu/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── sample.rs
│   │   │   ├── bias_estimation.rs
│   │   │   ├── multi_imu.rs
│   │   │   └── body_pose.rs
│   │   └── tests/
│   ├── omni-sense-fusion/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── kalman.rs
│   │   │   ├── ekf.rs
│   │   │   ├── ukf.rs
│   │   │   ├── particle_filter.rs
│   │   │   ├── jpda.rs
│   │   │   ├── mht.rs
│   │   │   ├── covariance_intersection.rs
│   │   │   ├── track_level_fuser.rs
│   │   │   ├── association.rs
│   │   │   └── gating.rs
│   │   └── tests/
│   │       ├── kalman_correctness.rs
│   │       ├── ekf_nonlinear.rs
│   │       ├── jpda_multitarget.rs
│   │       └── ci_combination.rs
│   ├── omni-sense-replay/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── recorder.rs
│   │   │   ├── replay.rs
│   │   │   └── format.rs
│   │   └── tests/
│   ├── omni-sense-drivers-mmwave/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-lidar/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-event/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-acoustic/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-imu/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-pir/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-tof/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-env/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   └── simulated.rs
│   │   └── tests/
│   ├── omni-sense-drivers-camera/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── trait_impl.rs
│   │   │   ├── simulated.rs
│   │   │   └── policy.rs
│   │   └── tests/
│   ├── omni-sense-bindings-python/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── lib.rs
│   │   ├── pyproject.toml
│   │   └── python/
│   │       └── omni_sense/
│   │           ├── __init__.py
│   │           ├── analysis.py
│   │           └── visualization.py
│   └── omni-sense-prelude/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── examples/
│   ├── basic_radar.rs
│   ├── basic_lidar.rs
│   ├── basic_imu_fusion.rs
│   ├── multi_modal_fusion.rs
│   ├── walking_calibration.rs
│   ├── body_frame_stabilization.rs
│   ├── atmospheric_attenuation.rs
│   ├── multi_mast_track_fusion.rs
│   ├── replay_scenario.rs
│   └── full_pipeline_with_pentatrack.rs
├── benches/
│   ├── fmcw_processing_bench.rs
│   ├── kalman_bench.rs
│   ├── beamforming_bench.rs
│   └── fusion_bench.rs
└── scripts/
    ├── format-all.sh
    ├── lint-all.sh
    └── publish-workspace.sh
```

---

## Integration with Downstream Projects

This section is the most important for ecosystem coherence. It specifies how each project consumes OMNI-SENSE.

### AEGIS-MESH

AEGIS-MESH uses OMNI-SENSE for:

- **mmWave radar** detection at choke points and in volume nodes (`omni-sense-drivers-mmwave`).
- **Solid-state LiDAR** geometry in volume nodes (`omni-sense-drivers-lidar` + `omni-sense-lidar`).
- **PIR** binary motion at choke points (`omni-sense-drivers-pir`).
- **Microphone arrays** in volume nodes for full-echo material classification (`omni-sense-acoustic`).
- **Coordinate frames** for node-to-home transforms (`omni-sense-frames`).
- **Time synchronization** across mesh nodes (`omni-sense-time`).
- **Track-level fusion** for multi-modal observations (`omni-sense-fusion`).
- **Walk-through calibration** for automatic node placement (`omni-sense-frames::WalkThroughCalibrator`).
- **Recording/replay** for scenario reproducibility (`omni-sense-replay`).

The Identity Layer in AEGIS-MESH does NOT depend on `omni-sense-drivers-camera`. The Identity Node is implemented in the `aegis-identity-node` crate and runs on physically isolated hardware. The omni-sense-drivers-camera crate is opt-in and would only be used if a research build explicitly enables imagery — distinct from AEGIS-MESH's core architecture.

### SENTINEL-WEAR

SENTINEL-WEAR uses OMNI-SENSE for:

- **mmWave radar** for primary motion detection in body frame (`omni-sense-drivers-mmwave`).
- **IMU** in every node for body-pose estimation (`omni-sense-drivers-imu` + `omni-sense-imu` + `omni-sense-physics::madgwick`).
- **Microphone array** in pendant for acoustic event detection (`omni-sense-drivers-acoustic`).
- **Short-range LiDAR/ToF** for close-range geometry (`omni-sense-drivers-lidar` or `omni-sense-drivers-tof`).
- **Environmental sensors** for context (`omni-sense-drivers-env`).
- **Body-frame coordinate management** (`omni-sense-frames::BodyFrameStabilizer`).
- **Multi-IMU body pose estimation** (`omni-sense-imu::body_pose`).
- **Walking calibration** for node positions (`omni-sense-frames::WalkThroughCalibrator`).
- **Body-area network time sync** (`omni-sense-time::ClockOffsetEstimator`).
- **Track-level fusion** for multi-node body-frame perception (`omni-sense-fusion::TrackLevelFuser`).
- **For the extreme-velocity research track**: Doppler radar with high-frequency CW processing (`omni-sense-radar::cw_doppler`) and event-based vision (`omni-sense-drivers-event` + `omni-sense-event`).

The Visual Identification Module (Pendant Node) opts in to `omni-sense-drivers-camera` with explicit policy documentation. Sensing nodes (bracelets, anklets, belt) do not depend on the camera crate.

### HALO-AD

HALO-AD uses OMNI-SENSE for:

- **mmWave radar** simulation at masts (`omni-sense-drivers-mmwave` simulation impls).
- **Scanning LiDAR** simulation (`omni-sense-drivers-lidar` simulation impls).
- **Event-based vision** simulation (`omni-sense-drivers-event` simulation impls).
- **Coverage geometry** primitives (beam divergence, propagation; `omni-sense-physics`).
- **Atmospheric models** (Beer-Lambert, thermal blooming, turbulence; `omni-sense-atmospherics`).
- **Coordinate frames** for mast-to-zone transforms (`omni-sense-frames`).
- **Time synchronization** for simulator-grade multi-mast sync (`omni-sense-time`).
- **Multi-mast covariance intersection fusion** (`omni-sense-fusion::covariance_intersection`).
- **Track-level fusion** across the array (`omni-sense-fusion::TrackLevelFuser`).
- **Recording/replay** for scenario libraries (`omni-sense-replay`).

HALO-AD is simulation-first. It uses the simulation implementations of all sensor traits exclusively.

### TALON-MESH

TALON-MESH uses OMNI-SENSE for:

- **Multi-modal sensor simulation** for counter-UAS perception (`omni-sense-drivers-mmwave`, `omni-sense-drivers-acoustic`, `omni-sense-drivers-lidar`, etc., all in simulation mode).
- **Atmospheric models** (`omni-sense-atmospherics`).
- **Line-of-sight ray-marching** (`omni-sense-frames` for voxel grids; ray-marching utilities to be added).
- **Multi-sensor fusion** (`omni-sense-fusion`).
- **PentaTrack predictive-center field** (consuming `DetectionEvent` from OMNI-SENSE).
- **Simulator harness coordination** with `omni-sense-replay`.

The TALON-MESH effector abstraction is project-specific and does NOT depend on OMNI-SENSE — the effector layer is the policy/decision boundary, not a sensor concern.

### Composition pattern

All four projects follow the same composition pattern:

```
project_name/
├── Cargo.toml                    # depends on omni-sense crates
├── src/
│   ├── lib.rs
│   ├── application/              # project-specific logic
│   ├── policy/                   # project-specific policy
│   ├── coordination/             # project-specific coordination
│   └── output/                   # project-specific output
└── tests/
    ├── integration_with_omni_sense.rs
    ├── integration_with_pentatrack.rs
    └── full_scenario.rs
```

OMNI-SENSE provides sensors, physics, fusion, frames, and time. PentaTrack provides predictive tracking. The project provides everything else.

---

## Civilian Transfer Applications

OMNI-SENSE is general infrastructure. Beyond the four research projects, it directly supports:

- **Multi-cobot pick-and-place coordination** (multi-modal fusion + frame transforms + time sync).
- **Multi-camera sports tracking** (without camera dependencies for non-imagery awareness; with `omni-sense-drivers-camera` for player identification — opt-in, policy-documented).
- **Wildlife monitoring sensor networks** (multi-modal fusion + atmospheric models).
- **Industrial safety wearables** (full IMU stack + body-frame management + multi-node fusion).
- **Eldercare and aging-in-place research** (camera-free fusion + fall detection algorithms).
- **Search-and-rescue UAS coordination** (multi-modal fusion + atmospheric models for adverse weather).
- **Smart-building HVAC and lighting** (occupancy fusion + privacy-preserving coverage).
- **Maritime navigation** (radar fusion + atmospheric models).
- **Air-traffic-management surveillance** (multi-sensor fusion + cooperative-uncooperative integration).
- **Multi-arm robotic surgery** (frame transforms + IMU fusion + uncertainty propagation).
- **Telescope-array siting and operation** (coverage geometry + atmospheric models).
- **Physical therapy and rehabilitation research** (multi-IMU body pose).
- **Cellular base-station planning** (coverage geometry + frame transforms — civilian RF planning).

The library is application-agnostic by design.

---

## Roadmap

### Phase 1 — Core Foundation (current)

- [ ] `omni-sense-core` — sensor traits, DetectionEvent, modality types, error types
- [ ] `omni-sense-physics` — ToF, Doppler, Madgwick, Mahony
- [ ] `omni-sense-frames` — frame management, transforms, calibration
- [ ] `omni-sense-time` — timestamps, clock offset estimation
- [ ] Simulation implementations of all sensor traits
- [ ] Basic test suite proving correctness

### Phase 2 — Atmospheric and Fusion (next)

- [ ] `omni-sense-atmospherics` — Beer-Lambert, blooming, turbulence
- [ ] `omni-sense-fusion` — Kalman, EKF, UKF, particle filter, JPDA, MHT, CI
- [ ] Track-level fusion infrastructure
- [ ] Walking calibration algorithm

### Phase 3 — Modality-Specific Pipelines

- [ ] `omni-sense-radar` — FMCW range-Doppler, CFAR
- [ ] `omni-sense-lidar` — point cloud processing, scan handling
- [ ] `omni-sense-event` — event camera processing
- [ ] `omni-sense-acoustic` — full-echo profiling, beamforming pipeline
- [ ] `omni-sense-imu` — multi-IMU body pose

### Phase 4 — Replay and Tooling

- [ ] `omni-sense-replay` — recording, replay, format
- [ ] Cross-project scenario library
- [ ] Comparative analysis tools
- [ ] Python bindings for analysis (`omni-sense-bindings-python`)

### Phase 5 — Hardware Drivers

Vendor-specific driver crates (third-party or first-party) implementing the trait surface for actual hardware. Out of scope for the core library; downstream projects can develop these as needed.

---

## Requirements

- Rust Edition 2021 (1.75+).
- `nalgebra` for linear algebra.
- `tokio` (or async runtime equivalent) for async sensor polling (gated by `std`).
- `serde` for serialization.
- `chrono` for wall-clock timestamps.
- `anyhow` for error handling in examples.

For `no-std` targets:
- `nalgebra` (with `no-std` feature).
- `serde` (with `no-std` feature).
- Project-provided allocator if needed.

---

## License

MIT for code. CC BY 4.0 for documentation.

OMNI-SENSE adopts the same dual-license posture as the projects that depend on it. Hardware-specific implementations in third-party driver crates may have their own licenses.

---

## Contributing

OMNI-SENSE is the infrastructure substrate for distributed perception research. Contributions are welcome in:

- Sensor trait coverage (new modalities, new specialized variants).
- Vendor driver implementations (in third-party crates).
- Physics and fusion algorithm correctness improvements.
- Test coverage and benchmarks.
- Documentation, especially physics references and porting guides.

PRs that violate the scope (actuator drivers, fire-control logic, controlled cryptography, ITAR-relevant parameters) will be closed. The scope is the substrate for sensing; what consumers do above that line is their concern.
