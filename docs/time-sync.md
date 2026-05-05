# Time Synchronization in OMNI-SENSE

---

## 1. The Problem

A distributed sensor mesh produces observations from multiple nodes at slightly different times, measured by slightly different clocks. Without synchronization:

- A target's position computed from simultaneous observations at different nodes uses measurements that are not actually simultaneous.
- Track fusion using data from multiple nodes produces incorrect association when timestamps are wrong.
- Replay of recorded scenarios is not reproducible if timestamps are inconsistent.

OMNI-SENSE provides the primitives to establish and maintain synchronized timestamps across a sensor mesh.

---

## 2. Timestamp Structure

Every OMNI-SENSE event carries a `Timestamp`:

```
Timestamp {
    monotonic_ns: u64    // Monotonic counter in nanoseconds. Never goes backward.
                         // Anchored to a local clock base; not calendar time.
    wall_clock_ns: i64   // System wall clock in nanoseconds since Unix epoch.
                         // Used for log correlation only, not for ordering.
}
```

All ordering decisions use `monotonic_ns`. `wall_clock_ns` is recorded for human readability and log correlation but is never used to order events.

---

## 3. Clock Offset Estimation

The `omni-sense-time::ClockOffsetEstimator` estimates the offset and drift rate between two clocks using a four-timestamp exchange (similar to IEEE 1588 PTP):

```
t1: sender records time of transmission
t2: receiver records time of receipt
t3: receiver records time of response transmission
t4: sender records time of response receipt
```

From these four timestamps:
- **One-way delay:** delay = ((t4 − t1) − (t3 − t2)) / 2
- **Clock offset:** offset = ((t2 − t1) + (t3 − t4)) / 2

The estimator accumulates multiple exchanges and fits a linear model to track drift rate, enabling offset prediction between exchanges.

**Achievable accuracy:**
- Over a local Wi-Fi or BLE mesh: sub-millisecond offset, measured in tens of microseconds with sufficient exchange rate.
- Over body-area-network (BAN): tens of microseconds with BLE, sub-microsecond with UWB.
- Residual error sources: network jitter (mitigated by median filtering), transmission timestamp resolution, processing latency on receiver.

---

## 4. Mesh Time Sync Architecture

For AEGIS-MESH and HALO-AD (Wi-Fi mesh):
1. The edge controller acts as the grandmaster clock.
2. Each node runs a `ClockOffsetEstimator` against the edge controller.
3. Nodes attach their `offset_from_controller` to each transmitted detection.
4. The edge controller applies the inverse offset to produce controller-clock timestamps.

For SENTINEL-WEAR (BAN mesh):
1. The belt node (primary compute) acts as the grandmaster clock.
2. Each wearable node runs a `ClockOffsetEstimator` against the belt node.
3. Same offset correction applied at the belt node.

For HALO-AD and TALON-MESH (simulator):
1. The simulator maintains a single monotonic simulation clock.
2. All simulated sensors share this clock exactly (no offset estimation needed in sim).
3. `omni-sense-time::SimulatorClock` provides the deterministic simulation clock.

---

## 5. Timestamp Arithmetic

```rust
// Duration between two timestamps
let duration = t2.duration_since(t1);

// Adding a duration to a timestamp
let t_future = t.add_duration(duration);

// Checking if two events are approximately simultaneous
if t1.aligns_with(t2, tolerance) { ... }

// Sorting a collection of events by time
events.sort_by_key(|e| e.timestamp.monotonic_ns);
```

---

## 6. Sensor Event Timestamping

Each sensor driver is responsible for recording the timestamp of each observation at the earliest possible point in the processing pipeline:

- **Hardware interrupt:** If the sensor triggers an interrupt, timestamp at interrupt entry.
- **DMA transfer complete:** If data arrives via DMA, timestamp at transfer completion.
- **Poll return:** If using polling, timestamp immediately upon return from the sensor read call, before any processing.

Late timestamping (after processing) introduces processing latency into the timestamp, which corrupts the synchronization. The `omni-sense-core::Sensor` trait documents this requirement.
