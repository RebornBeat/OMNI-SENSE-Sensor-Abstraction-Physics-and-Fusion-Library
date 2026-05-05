# Porting Guide — Adding Hardware Drivers

This document describes how to add support for a specific hardware sensor to OMNI-SENSE. All vendor-specific driver crates are third-party — they are not part of the OMNI-SENSE workspace but depend on `omni-sense-core` traits.

---

## 1. Identify the Appropriate Trait

Determine which OMNI-SENSE sensor trait your hardware implements:

- **mmWave radar (FMCW):** Implement `MmWaveRadar: RangingSensor + VelocitySensor`.
- **Scanning LiDAR:** Implement `ScanningLidar: RangingSensor`.
- **Solid-state LiDAR:** Implement `SolidStateLidar: RangingSensor`.
- **Flash LiDAR:** Implement `FlashLidar: RangingSensor`.
- **Event camera:** Implement `EventSensor`.
- **Microphone array:** Implement `AcousticSensor`.
- **IMU:** Implement `ImuSensor`.
- **PIR:** Implement `BinarySensor`.
- **1D ToF:** Implement `RangingSensor` (single-beam variant).
- **Environmental sensor:** Implement `EnvironmentalSensor`.
- **Conventional camera:** Implement `ImagingSensor` (requires `camera` feature).

---

## 2. Create a Crate

```bash
cargo new omni-sense-driver-<vendor>-<model> --lib
```

Depend on `omni-sense-core`:

```toml
[dependencies]
omni-sense-core = "0.1"
embedded-hal = { version = "1.0", optional = true }  # for no_std targets
tokio = { version = "1", optional = true }             # for std async targets
```

---

## 3. Implement the Sensor Base Trait

```rust
use omni_sense_core::{Sensor, SensorId, Modality, FrameId, Pose, SensorHealth, SensorError};

pub struct MyVendorSensor { /* hardware interface */ }

impl Sensor for MyVendorSensor {
    fn id(&self) -> SensorId { self.id.clone() }
    fn modality(&self) -> Modality { Modality::MmWaveRadar }
    fn frame(&self) -> FrameId { self.frame_id.clone() }
    fn pose(&self) -> Pose { self.pose }
    fn health(&self) -> SensorHealth { self.health }
    fn initialize(&mut self) -> Result<(), SensorError> { /* vendor init */ }
    fn shutdown(&mut self) -> Result<(), SensorError> { /* vendor shutdown */ }
}
```

---

## 4. Implement the Modality Trait

```rust
use omni_sense_core::{RangingSensor, RangeReturn, VelocitySensor, VelocityReturn};

impl RangingSensor for MyVendorSensor {
    fn poll_ranges(&mut self) -> Result<Vec<RangeReturn>, SensorError> {
        // Read raw data from hardware
        // Convert to Vec<RangeReturn> { range_m, bearing_rad, elevation_rad, intensity, timestamp }
    }
    fn nominal_range(&self) -> f32 { 30.0 /* meters */ }
    fn beam_pattern(&self) -> omni_sense_core::BeamPattern { /* ... */ }
    fn range_resolution(&self) -> f32 { 0.05 /* meters */ }
}

impl VelocitySensor for MyVendorSensor {
    fn poll_velocities(&mut self) -> Result<Vec<VelocityReturn>, SensorError> {
        // Run FMCW processing via omni-sense-radar, or use vendor SDK if the
        // vendor provides processed Range-Doppler output
    }
    // ...
}
```

---

## 5. Timestamp Discipline

Record the timestamp at the earliest possible point:

```rust
fn poll_ranges(&mut self) -> Result<Vec<RangeReturn>, SensorError> {
    let timestamp = Timestamp::now(); // Record BEFORE any processing
    let raw = self.hardware.read_chirp_frame()?; // Hardware read
    let processed = process(raw); // Then process
    Ok(processed.into_iter().map(|r| RangeReturn { timestamp, ..r }).collect())
}
```

---

## 6. Testing

Against the simulation implementation:

```rust
#[test]
fn vendor_driver_produces_same_range_format_as_sim() {
    let mut sim = SimulatedScanningLidar::new().build();
    sim.initialize().unwrap();
    let sim_ranges = sim.poll_ranges().unwrap();
    
    // Your vendor driver in test mode
    let mut vendor = MyVendorSensor::new_in_test_mode();
    vendor.initialize().unwrap();
    let vendor_ranges = vendor.poll_ranges().unwrap();
    
    // Structure must be identical; values will differ
    assert_eq!(sim_ranges[0].bearing.is_finite(), vendor_ranges[0].bearing.is_finite());
    // etc.
}
```

---

## 7. Publishing

Vendor driver crates are published independently to crates.io. Name them `omni-sense-driver-<vendor>-<product>` e.g. `omni-sense-driver-ti-iwr6843`. Document the hardware setup (wiring, firmware requirements, calibration) in the crate's README.
