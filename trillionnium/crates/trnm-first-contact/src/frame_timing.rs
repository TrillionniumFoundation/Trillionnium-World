use bevy::{
    prelude::{Res, ResMut, Resource, Time},
    time::Real,
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        OnceLock,
    },
    thread::{self, ThreadId},
    time::Instant,
};

const FRAME_TIME_BUCKET_WIDTH_MS: f64 = 0.5;
const FRAME_TIME_BUCKET_COUNT: usize = 2_001;
const DEFAULT_WARMUP_SECONDS: f64 = 5.0;
const DEFAULT_MIN_MEASUREMENT_SECONDS: f64 = 10.0;
const DEFAULT_MIN_FRAME_SAMPLES: u64 = 300;
const DEFAULT_MIN_AVERAGE_FPS: f64 = 60.0;
const DEFAULT_MIN_ONE_PERCENT_LOW_FPS: f64 = 30.0;
const DEFAULT_MAX_FRAME_DELTA_MS: f64 = 100.0;
const DEFAULT_MIN_REAL_TIME_COVERAGE_RATIO: f64 = 0.90;
const DEFAULT_MAX_REAL_TIME_COVERAGE_RATIO: f64 = 1.10;
const DEFAULT_MIN_INPUT_ACK_SAMPLES: u64 = 1;
const DEFAULT_MAX_INPUT_ACK_MS: f64 = 1_000.0;

static RENDER_UPDATE_THREAD: OnceLock<ThreadId> = OnceLock::new();
static RENDER_UPDATE_THREAD_CHANGES: AtomicU64 = AtomicU64::new(0);
static INSTRUMENTED_NETWORK_IO_AFTER_RENDER_START: AtomicU64 = AtomicU64::new(0);
static INSTRUMENTED_NETWORK_IO_ON_RENDER_THREAD: AtomicU64 = AtomicU64::new(0);

pub(super) fn note_instrumented_network_io() {
    let current = thread::current().id();
    if let Some(render_thread) = RENDER_UPDATE_THREAD.get() {
        INSTRUMENTED_NETWORK_IO_AFTER_RENDER_START.fetch_add(1, Ordering::Relaxed);
        if render_thread == &current {
            INSTRUMENTED_NETWORK_IO_ON_RENDER_THREAD.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn register_render_update_thread() {
    let current = thread::current().id();
    if let Some(render_thread) = RENDER_UPDATE_THREAD.get() {
        if render_thread != &current {
            RENDER_UPDATE_THREAD_CHANGES.fetch_add(1, Ordering::Relaxed);
        }
    } else {
        let _ = RENDER_UPDATE_THREAD.set(current);
    }
}

#[derive(Debug)]
struct FrameTimingHistogram {
    buckets: [u64; FRAME_TIME_BUCKET_COUNT],
    frame_count: u64,
    total_delta_ms: f64,
    max_delta_ms: f64,
    frames_over_16_67ms: u64,
    frames_over_33_33ms: u64,
    frames_over_100ms: u64,
}

impl Default for FrameTimingHistogram {
    fn default() -> Self {
        Self {
            buckets: [0; FRAME_TIME_BUCKET_COUNT],
            frame_count: 0,
            total_delta_ms: 0.0,
            max_delta_ms: 0.0,
            frames_over_16_67ms: 0,
            frames_over_33_33ms: 0,
            frames_over_100ms: 0,
        }
    }
}

impl FrameTimingHistogram {
    fn record(&mut self, delta_ms: f64) {
        if !delta_ms.is_finite() || delta_ms <= 0.0 {
            return;
        }
        let bucket = ((delta_ms / FRAME_TIME_BUCKET_WIDTH_MS).ceil() as usize)
            .saturating_sub(1)
            .min(FRAME_TIME_BUCKET_COUNT - 1);
        self.buckets[bucket] = self.buckets[bucket].saturating_add(1);
        self.frame_count = self.frame_count.saturating_add(1);
        self.total_delta_ms += delta_ms;
        self.max_delta_ms = self.max_delta_ms.max(delta_ms);
        if delta_ms > 1_000.0 / 60.0 {
            self.frames_over_16_67ms = self.frames_over_16_67ms.saturating_add(1);
        }
        if delta_ms > 1_000.0 / 30.0 {
            self.frames_over_33_33ms = self.frames_over_33_33ms.saturating_add(1);
        }
        if delta_ms > 100.0 {
            self.frames_over_100ms = self.frames_over_100ms.saturating_add(1);
        }
    }

    fn average_fps(&self) -> f64 {
        if self.total_delta_ms <= 0.0 {
            0.0
        } else {
            self.frame_count as f64 * 1_000.0 / self.total_delta_ms
        }
    }

    fn percentile_ms(&self, quantile: f64) -> f64 {
        if self.frame_count == 0 {
            return 0.0;
        }
        let rank = (self.frame_count as f64 * quantile.clamp(0.0, 1.0))
            .ceil()
            .max(1.0) as u64;
        let mut observed = 0_u64;
        for (index, count) in self.buckets.iter().enumerate() {
            observed = observed.saturating_add(*count);
            if observed >= rank {
                if index == FRAME_TIME_BUCKET_COUNT - 1 {
                    return self.max_delta_ms;
                }
                return ((index + 1) as f64 * FRAME_TIME_BUCKET_WIDTH_MS).min(self.max_delta_ms);
            }
        }
        self.max_delta_ms
    }

    fn slowest_fraction_average_ms(&self, fraction: f64) -> f64 {
        if self.frame_count == 0 {
            return 0.0;
        }
        let target = (self.frame_count as f64 * fraction.clamp(0.0, 1.0))
            .ceil()
            .max(1.0) as u64;
        let mut remaining = target;
        let mut total_ms = 0.0;
        for (index, count) in self.buckets.iter().enumerate().rev() {
            if remaining == 0 {
                break;
            }
            let take = remaining.min(*count);
            if take == 0 {
                continue;
            }
            let bucket_upper_ms = if index == FRAME_TIME_BUCKET_COUNT - 1 {
                self.max_delta_ms
            } else {
                (index + 1) as f64 * FRAME_TIME_BUCKET_WIDTH_MS
            };
            total_ms += bucket_upper_ms * take as f64;
            remaining -= take;
        }
        total_ms / target as f64
    }

    fn one_percent_low_fps(&self) -> f64 {
        let slowest_one_percent_average_ms = self.slowest_fraction_average_ms(0.01);
        if slowest_one_percent_average_ms <= 0.0 {
            0.0
        } else {
            1_000.0 / slowest_one_percent_average_ms
        }
    }

    fn miss_ratio(&self, missed_frames: u64) -> f64 {
        if self.frame_count == 0 {
            0.0
        } else {
            missed_frames as f64 / self.frame_count as f64
        }
    }
}

#[derive(Debug, Default)]
struct TimingSamples {
    count: u64,
    total_ms: f64,
    max_ms: f64,
}

impl TimingSamples {
    fn record(&mut self, sample_ms: f64) {
        if !sample_ms.is_finite() || sample_ms < 0.0 {
            return;
        }
        self.count = self.count.saturating_add(1);
        self.total_ms += sample_ms;
        self.max_ms = self.max_ms.max(sample_ms);
    }

    fn average_ms(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_ms / self.count as f64
        }
    }
}

#[derive(Resource, Debug)]
pub(super) struct OnlineFrameTiming {
    evidence_path: PathBuf,
    warmup_started_at: Instant,
    warmup_seconds: f64,
    min_measurement_seconds: f64,
    min_frame_samples: u64,
    min_average_fps: f64,
    min_one_percent_low_fps: f64,
    max_frame_delta_ms: f64,
    min_real_time_coverage_ratio: f64,
    max_real_time_coverage_ratio: f64,
    require_network_thread_evidence: bool,
    require_input_ack_evidence: bool,
    min_input_ack_samples: u64,
    max_input_ack_ms: f64,
    measurement_started_at: Option<Instant>,
    frames: FrameTimingHistogram,
    network_round_trips: TimingSamples,
    input_to_durable_acks: TimingSamples,
    update_started_at: Option<Instant>,
    main_thread_updates_over_100ms: u64,
    max_main_thread_update_ms: f64,
    write_accumulator: f32,
}

impl OnlineFrameTiming {
    pub(super) fn from_env() -> Option<Self> {
        std::env::var_os("TRNM_ONLINE_FRAME_TIMING_PATH").map(|path| Self {
            evidence_path: PathBuf::from(path),
            warmup_started_at: Instant::now(),
            warmup_seconds: positive_f64_env(
                "TRNM_ONLINE_FRAME_TIMING_WARMUP_SECONDS",
                DEFAULT_WARMUP_SECONDS,
            ),
            min_measurement_seconds: positive_f64_env(
                "TRNM_ONLINE_FRAME_TIMING_MIN_SECONDS",
                DEFAULT_MIN_MEASUREMENT_SECONDS,
            ),
            min_frame_samples: positive_u64_env(
                "TRNM_ONLINE_FRAME_TIMING_MIN_SAMPLES",
                DEFAULT_MIN_FRAME_SAMPLES,
            ),
            min_average_fps: positive_f64_env(
                "TRNM_ONLINE_MIN_AVERAGE_FPS",
                DEFAULT_MIN_AVERAGE_FPS,
            ),
            min_one_percent_low_fps: positive_f64_env(
                "TRNM_ONLINE_MIN_ONE_PERCENT_LOW_FPS",
                DEFAULT_MIN_ONE_PERCENT_LOW_FPS,
            ),
            max_frame_delta_ms: positive_f64_env(
                "TRNM_ONLINE_MAX_FRAME_DELTA_MS",
                DEFAULT_MAX_FRAME_DELTA_MS,
            ),
            min_real_time_coverage_ratio: positive_f64_env(
                "TRNM_ONLINE_MIN_REAL_TIME_COVERAGE_RATIO",
                DEFAULT_MIN_REAL_TIME_COVERAGE_RATIO,
            ),
            max_real_time_coverage_ratio: positive_f64_env(
                "TRNM_ONLINE_MAX_REAL_TIME_COVERAGE_RATIO",
                DEFAULT_MAX_REAL_TIME_COVERAGE_RATIO,
            ),
            require_network_thread_evidence: bool_env(
                "TRNM_ONLINE_REQUIRE_NETWORK_THREAD_EVIDENCE",
                false,
            ),
            require_input_ack_evidence: bool_env("TRNM_ONLINE_REQUIRE_INPUT_ACK_EVIDENCE", false),
            min_input_ack_samples: positive_u64_env(
                "TRNM_ONLINE_MIN_INPUT_ACK_SAMPLES",
                DEFAULT_MIN_INPUT_ACK_SAMPLES,
            ),
            max_input_ack_ms: positive_f64_env(
                "TRNM_ONLINE_MAX_INPUT_ACK_MS",
                DEFAULT_MAX_INPUT_ACK_MS,
            ),
            measurement_started_at: None,
            frames: FrameTimingHistogram::default(),
            network_round_trips: TimingSamples::default(),
            input_to_durable_acks: TimingSamples::default(),
            update_started_at: None,
            main_thread_updates_over_100ms: 0,
            max_main_thread_update_ms: 0.0,
            write_accumulator: 0.0,
        })
    }

    pub(super) fn record_command_ack(
        &mut self,
        network_round_trip_ms: f64,
        input_to_durable_ack_ms: Option<f64>,
    ) {
        self.network_round_trips.record(network_round_trip_ms);
        if let Some(input_to_durable_ack_ms) = input_to_durable_ack_ms {
            self.input_to_durable_acks.record(input_to_durable_ack_ms);
        }
    }

    fn report(&self, measurement_elapsed_ms: f64) -> serde_json::Value {
        let average_fps = self.frames.average_fps();
        let one_percent_low_fps = self.frames.one_percent_low_fps();
        let real_time_coverage_ratio = if measurement_elapsed_ms <= 0.0 {
            0.0
        } else {
            self.frames.total_delta_ms / measurement_elapsed_ms
        };
        let measurement_valid = measurement_elapsed_ms >= self.min_measurement_seconds * 1_000.0
            && self.frames.frame_count >= self.min_frame_samples
            && real_time_coverage_ratio >= self.min_real_time_coverage_ratio
            && real_time_coverage_ratio <= self.max_real_time_coverage_ratio;
        let frame_cadence_passed = measurement_valid
            && average_fps >= self.min_average_fps
            && one_percent_low_fps >= self.min_one_percent_low_fps
            && self.frames.max_delta_ms <= self.max_frame_delta_ms
            && self.frames.frames_over_100ms == 0;
        let instrumented_network_io_after_render_start =
            INSTRUMENTED_NETWORK_IO_AFTER_RENDER_START.load(Ordering::Relaxed);
        let network_requests_on_render_thread =
            INSTRUMENTED_NETWORK_IO_ON_RENDER_THREAD.load(Ordering::Relaxed);
        let render_update_thread_changes = RENDER_UPDATE_THREAD_CHANGES.load(Ordering::Relaxed);
        let network_thread_evidence_passed = !self.require_network_thread_evidence
            || (instrumented_network_io_after_render_start > 0
                && network_requests_on_render_thread == 0
                && render_update_thread_changes == 0);
        let input_ack_evidence_passed = !self.require_input_ack_evidence
            || (self.input_to_durable_acks.count >= self.min_input_ack_samples
                && self.input_to_durable_acks.max_ms <= self.max_input_ack_ms);
        let update_chain_passed =
            self.main_thread_updates_over_100ms == 0 && self.max_main_thread_update_ms <= 100.0;
        serde_json::json!({
            "contract_version": "trnm_online_render_frame_timing_v3",
            "clock": "bevy_time_real",
            "write_mode": "same_directory_atomic_rename",
            "warmup_seconds": self.warmup_seconds,
            "minimum_measurement_seconds": self.min_measurement_seconds,
            "minimum_frame_samples": self.min_frame_samples,
            "measurement_elapsed_ms": measurement_elapsed_ms,
            "observed_real_frame_time_ms": self.frames.total_delta_ms,
            "real_time_coverage_ratio": real_time_coverage_ratio,
            "measurement_valid": measurement_valid,
            "frame_count": self.frames.frame_count,
            "average_fps": average_fps,
            "one_percent_low_fps": one_percent_low_fps,
            "one_percent_low_method": "reciprocal_of_average_slowest_ceil_one_percent_real_frame_deltas",
            "p50_frame_delta_ms": self.frames.percentile_ms(0.50),
            "p95_frame_delta_ms": self.frames.percentile_ms(0.95),
            "p99_frame_delta_ms": self.frames.percentile_ms(0.99),
            "max_frame_delta_ms": self.frames.max_delta_ms,
            "frames_over_16_67ms": self.frames.frames_over_16_67ms,
            "frames_over_33_33ms": self.frames.frames_over_33_33ms,
            "frames_over_100ms": self.frames.frames_over_100ms,
            "frame_budget_60fps_miss_ratio": self.frames.miss_ratio(self.frames.frames_over_16_67ms),
            "frame_budget_30fps_miss_ratio": self.frames.miss_ratio(self.frames.frames_over_33_33ms),
            "targets": {
                "minimum_average_fps": self.min_average_fps,
                "minimum_one_percent_low_fps": self.min_one_percent_low_fps,
                "maximum_frame_delta_ms": self.max_frame_delta_ms,
                "minimum_real_time_coverage_ratio": self.min_real_time_coverage_ratio,
                "maximum_real_time_coverage_ratio": self.max_real_time_coverage_ratio,
                "recommended_fps": 60.0,
            },
            "main_thread_updates_over_100ms": self.main_thread_updates_over_100ms,
            "max_main_thread_update_ms": self.max_main_thread_update_ms,
            "update_chain_passed": update_chain_passed,
            "network_thread_instrumentation": {
                "contract_version": "trnm_native_network_thread_instrumentation_v1",
                "required": self.require_network_thread_evidence,
                "instrumented_io_calls_after_render_start": instrumented_network_io_after_render_start,
                "io_calls_on_render_thread": network_requests_on_render_thread,
                "render_update_thread_changes": render_update_thread_changes,
                "passed": network_thread_evidence_passed,
            },
            "network_requests_on_render_thread": network_requests_on_render_thread > 0,
            "network_main_thread_passed": network_thread_evidence_passed && update_chain_passed,
            "network_command_round_trip": {
                "samples": self.network_round_trips.count,
                "average_ms": self.network_round_trips.average_ms(),
                "max_ms": self.network_round_trips.max_ms,
            },
            "native_input_to_durable_ack": {
                "required": self.require_input_ack_evidence,
                "minimum_samples": self.min_input_ack_samples,
                "maximum_ms": self.max_input_ack_ms,
                "samples": self.input_to_durable_acks.count,
                "average_ms": self.input_to_durable_acks.average_ms(),
                "max_ms": self.input_to_durable_acks.max_ms,
                "passed": input_ack_evidence_passed,
            },
            "frame_cadence_passed": frame_cadence_passed,
            "passed": frame_cadence_passed
                && update_chain_passed
                && network_thread_evidence_passed
                && input_ack_evidence_passed,
        })
    }
}

fn positive_f64_env(key: &str, default: f64) -> f64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| value.is_finite() && *value > 0.0)
        .unwrap_or(default)
}

fn positive_u64_env(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn bool_env(key: &str, default: bool) -> bool {
    match std::env::var(key).as_deref() {
        Ok("1" | "true" | "TRUE") => true,
        Ok("0" | "false" | "FALSE") => false,
        _ => default,
    }
}

fn write_report_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("frame-timing.json");
    let temporary = parent.join(format!(".{file_name}.{}.tmp", std::process::id()));
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&temporary, path)
}

pub(super) fn begin_online_frame_timing(timing: Option<ResMut<OnlineFrameTiming>>) {
    register_render_update_thread();
    if let Some(mut timing) = timing {
        timing.update_started_at = Some(Instant::now());
    }
}

pub(super) fn record_online_frame_timing(
    time: Res<Time<Real>>,
    timing: Option<ResMut<OnlineFrameTiming>>,
) {
    let Some(mut timing) = timing else {
        return;
    };
    if timing.warmup_started_at.elapsed().as_secs_f64() < timing.warmup_seconds {
        timing.update_started_at = None;
        return;
    }
    if timing.measurement_started_at.is_none() {
        timing.measurement_started_at = Some(Instant::now());
        timing.update_started_at = None;
        return;
    }
    timing.frames.record(time.delta_secs_f64() * 1_000.0);
    if let Some(started_at) = timing.update_started_at.take() {
        let update_ms = started_at.elapsed().as_secs_f64() * 1_000.0;
        timing.max_main_thread_update_ms = timing.max_main_thread_update_ms.max(update_ms);
        if update_ms > 100.0 {
            timing.main_thread_updates_over_100ms =
                timing.main_thread_updates_over_100ms.saturating_add(1);
        }
    }
    timing.write_accumulator += time.delta_secs();
    if timing.write_accumulator < 0.5 {
        return;
    }
    timing.write_accumulator = 0.0;
    let measurement_elapsed_ms = timing
        .measurement_started_at
        .map(|started| started.elapsed().as_secs_f64() * 1_000.0)
        .unwrap_or_default();
    let report = timing.report(measurement_elapsed_ms);
    if let Ok(bytes) = serde_json::to_vec_pretty(&report) {
        let _ = write_report_atomically(&timing.evidence_path, &bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_timing() -> OnlineFrameTiming {
        OnlineFrameTiming {
            evidence_path: PathBuf::new(),
            warmup_started_at: Instant::now(),
            warmup_seconds: 0.1,
            min_measurement_seconds: 5.0,
            min_frame_samples: 60,
            min_average_fps: 60.0,
            min_one_percent_low_fps: 30.0,
            max_frame_delta_ms: 100.0,
            min_real_time_coverage_ratio: 0.9,
            max_real_time_coverage_ratio: 1.1,
            require_network_thread_evidence: false,
            require_input_ack_evidence: false,
            min_input_ack_samples: 1,
            max_input_ack_ms: 1_000.0,
            measurement_started_at: None,
            frames: FrameTimingHistogram::default(),
            network_round_trips: TimingSamples::default(),
            input_to_durable_acks: TimingSamples::default(),
            update_started_at: None,
            main_thread_updates_over_100ms: 0,
            max_main_thread_update_ms: 0.0,
            write_accumulator: 0.0,
        }
    }

    #[test]
    fn histogram_reports_real_percentiles_and_slowest_one_percent_average() {
        let mut timing = FrameTimingHistogram::default();
        for _ in 0..99 {
            timing.record(16.0);
        }
        timing.record(40.0);

        assert!((timing.average_fps() - 61.58).abs() < 0.1);
        assert_eq!(timing.percentile_ms(0.50), 16.0);
        assert_eq!(timing.percentile_ms(0.95), 16.0);
        assert_eq!(timing.percentile_ms(0.99), 16.0);
        assert_eq!(timing.max_delta_ms, 40.0);
        assert!((timing.one_percent_low_fps() - 25.0).abs() < 0.1);
        assert_eq!(timing.frames_over_33_33ms, 1);
    }

    #[test]
    fn report_fails_closed_for_short_or_sub_sixty_fps_measurements() {
        let mut timing = test_timing();
        for _ in 0..100 {
            timing.frames.record(50.0);
        }

        let short = timing.report(4_999.0);
        assert_eq!(short["measurement_valid"], false);
        assert_eq!(short["passed"], false);

        let slow = timing.report(5_000.0);
        assert_eq!(slow["measurement_valid"], true);
        assert_eq!(slow["frame_cadence_passed"], false);
        assert_eq!(slow["passed"], false);
    }

    #[test]
    fn report_rejects_a_real_frame_over_the_hard_stall_limit() {
        let mut timing = test_timing();
        timing.min_frame_samples = 1_000;
        for _ in 0..10_000 {
            timing.frames.record(10.0);
        }
        timing.frames.record(150.0);
        let elapsed = timing.frames.total_delta_ms;

        let report = timing.report(elapsed);
        assert_eq!(report["measurement_valid"], true);
        assert!(report["average_fps"].as_f64().unwrap() > 60.0);
        assert!(report["one_percent_low_fps"].as_f64().unwrap() > 30.0);
        assert_eq!(report["frames_over_100ms"], 1);
        assert_eq!(report["passed"], false);
    }

    #[test]
    fn input_ack_requirement_fails_closed_without_a_bound_sample() {
        let mut timing = test_timing();
        timing.require_input_ack_evidence = true;
        for _ in 0..600 {
            timing.frames.record(16.0);
        }
        let elapsed = timing.frames.total_delta_ms;
        assert_eq!(timing.report(elapsed)["passed"], false);

        timing.record_command_ack(120.0, Some(180.0));
        assert_eq!(
            timing.report(elapsed)["native_input_to_durable_ack"]["passed"],
            true
        );
        assert_eq!(timing.report(elapsed)["passed"], true);
    }
}
