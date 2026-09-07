__TRNM_SLOT_0__
    fn sigterm_drain_classifies_commands_as_retryable_service_unavailable() {
        assert!(command_drain_error(false, false).is_none());
        for error in [
            command_drain_error(true, false).unwrap(),
            command_drain_error(false, true).unwrap(),
        ] {
            assert_eq!(error.status, StatusCode::SERVICE_UNAVAILABLE);
            assert!(error.body.recoverable);
            let response = error.into_response();
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(
                response.headers().get(header::RETRY_AFTER),
                Some(&HeaderValue::from_static("1"))
            );
        }
    }

    #[test]
    fn production_authority_clock_is_exactly_ten_hz() {
        let interval = production_authority_tick_interval();
        assert_eq!(TICKS_PER_SECOND, 10);
        assert_eq!(interval, Duration::from_millis(100));
        assert_eq!(interval.saturating_mul(1_800), Duration::from_secs(180));
    }

    #[test]
    fn accelerated_clock_is_explicitly_test_only() {
        assert_eq!(
            resolve_authority_tick_interval(None, false).unwrap(),
            Duration::from_millis(100)
        );
        assert!(resolve_authority_tick_interval(Some(50), false).is_err());
        assert_eq!(
            resolve_authority_tick_interval(Some(20), true).unwrap(),
            Duration::from_millis(20)
        );
        assert!(resolve_authority_tick_interval(Some(0), true).is_err());
    }

    #[test]
    fn readiness_fails_closed_on_authority_clock_degradation() {
        let tick_interval = Duration::from_millis(100);
        let snapshot = |drift_ticks: f64,
                        lateness_ticks: f64,
                        last_wake_age_ms: f64,
                        sample_count: usize| AuthorityClockSnapshot {
            elapsed_ms: 1_000.0,
            wake_count: 10,
            cumulative_drift_ticks: 0.0,
            window_sample_count: sample_count,
            window_drift_ticks: Some(drift_ticks),
            latest_lateness_ticks: Some(lateness_ticks),
            max_recent_lateness_ticks: Some(lateness_ticks),
            last_wake_age_ms: Some(last_wake_age_ms),
        };

        assert!(authority_clock_is_operational(
            Some(&snapshot(1.99, 1.99, 199.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(authority_clock_is_operational(
            Some(&snapshot(-1.99, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(2.0, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(0.0, 2.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(0.0, 0.0, 200.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(0.0, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES - 1)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(f64::NAN, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(None, tick_interval));
    }

    #[test]
    fn authority_clock_window_self_heals_after_transient_stall() {
        let telemetry = AuthorityClockTelemetry::default();
        let tick_interval = Duration::from_millis(100);
        let started_at = Instant::now();
        telemetry.reset(started_at);

        for tick in 1..=AUTHORITY_CLOCK_WINDOW_TICKS + 1 {
            let observed_at = started_at + tick_interval.saturating_mul(tick as u32);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let healthy = telemetry
            .snapshot(
                started_at
                    + tick_interval.saturating_mul((AUTHORITY_CLOCK_WINDOW_TICKS + 1) as u32),
                tick_interval,
            )
            .unwrap();
        assert!(authority_clock_is_operational(
            Some(&healthy),
            tick_interval
        ));

        let scheduled_at =
            started_at + tick_interval.saturating_mul((AUTHORITY_CLOCK_WINDOW_TICKS + 2) as u32);
        let recovered_epoch = started_at + Duration::from_secs(60);
        telemetry.record_wake(scheduled_at, recovered_epoch, tick_interval);
        let stalled = telemetry.snapshot(recovered_epoch, tick_interval).unwrap();
        assert!(!authority_clock_is_operational(
            Some(&stalled),
            tick_interval
        ));

        for tick in 1..=AUTHORITY_CLOCK_WINDOW_TICKS + 1 {
            let observed_at = recovered_epoch + tick_interval.saturating_mul(tick as u32);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let recovered = telemetry
            .snapshot(
                recovered_epoch
                    + tick_interval.saturating_mul((AUTHORITY_CLOCK_WINDOW_TICKS + 1) as u32),
                tick_interval,
            )
            .unwrap();
        assert!(recovered.cumulative_drift_ticks < -500.0);
        assert_eq!(recovered.window_drift_ticks, Some(0.0));
        assert!(authority_clock_is_operational(
            Some(&recovered),
            tick_interval
        ));
    }

    #[test]
    fn authority_clock_window_rejects_sustained_slow_cadence() {
        let telemetry = AuthorityClockTelemetry::default();
        let tick_interval = Duration::from_millis(100);
        let started_at = Instant::now();
        telemetry.reset(started_at);

        for tick in 1..=10_u32 {
            let observed_at = started_at + Duration::from_millis(u64::from(tick) * 150);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let snapshot = telemetry
            .snapshot(started_at + Duration::from_millis(1_500), tick_interval)
            .unwrap();
        assert_eq!(snapshot.window_drift_ticks, Some(-4.5));
        assert!(!authority_clock_is_operational(
            Some(&snapshot),
            tick_interval
        ));
    }

    #[test]
    fn match_actor_clock_self_heals_and_allows_one_bounded_command_publication_window() {
        let tick_interval = Duration::from_millis(100);
        let telemetry = AuthorityClockTelemetry::default();
        let started_at = Instant::now();
        telemetry.reset(started_at);
        telemetry.record_wake(
            started_at + tick_interval,
            started_at + Duration::from_secs(3),
            tick_interval,
        );
        for tick in 1..=AUTHORITY_CLOCK_WINDOW_TICKS + 1 {
            let observed_at =
__TRNM_SLOT_2__
__TRNM_SLOT_3__
