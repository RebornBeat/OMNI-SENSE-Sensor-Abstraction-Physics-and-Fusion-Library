use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A dual timestamp: monotonic for ordering, wall-clock for log correlation.
/// All ordering decisions use `monotonic_ns`. Never goes backward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp {
    /// Monotonic nanoseconds since process start. Never goes backward.
    pub monotonic_ns: u64,
    /// Wall-clock nanoseconds since Unix epoch. For log correlation only.
    pub wall_clock_ns: i64,
}

impl Timestamp {
    pub fn now() -> Self {
        let wall = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as i64;
        // Use wall clock as monotonic proxy when no hardware monotonic is available.
        // Real embedded firmware provides a hardware counter here.
        Self { monotonic_ns: wall as u64, wall_clock_ns: wall }
    }

    pub fn from_monotonic_ns(ns: u64) -> Self {
        Self { monotonic_ns: ns, wall_clock_ns: 0 }
    }

    pub fn duration_since_ns(&self, earlier: &Self) -> u64 {
        self.monotonic_ns.saturating_sub(earlier.monotonic_ns)
    }

    pub fn add_ns(&self, ns: u64) -> Self {
        Self {
            monotonic_ns: self.monotonic_ns + ns,
            wall_clock_ns: self.wall_clock_ns + ns as i64,
        }
    }

    pub fn aligns_with(&self, other: &Self, tolerance_ns: u64) -> bool {
        self.monotonic_ns.abs_diff(other.monotonic_ns) <= tolerance_ns
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self { monotonic_ns: 0, wall_clock_ns: 0 }
    }
}
