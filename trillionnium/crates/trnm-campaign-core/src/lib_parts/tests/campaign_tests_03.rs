    #[test]
    fn chapter_scenes_require_three_player_beats_and_caravans_exist_in_rooms() {
        let mut campaign = CampaignSaveV1 {
            pending_main_story_chapter: Some(MainStoryChapter::MirrorCityOaths),
            room: CampaignRoom::MirrorSquare,
            ..CampaignSaveV1::default()
        };
        for expected_step in [1, 2] {
            let advance = campaign.advance_pending_main_story_scene().unwrap();
            assert!(matches!(
                advance,
                MainStorySceneAdvance::SceneBeat { step, .. } if step == expected_step
            ));
            assert!(campaign.main_story_decisions.is_empty());
        }
        assert!(matches!(
            campaign.advance_pending_main_story_scene().unwrap(),
            MainStorySceneAdvance::ChapterResolved(_)
        ));
        assert_eq!(campaign.main_story_decisions.len(), 1);

        campaign.wait_in_town(120).unwrap();
        let room_id = campaign.active_regional_caravans[0]
            .current_room_id()
            .unwrap()
            .to_string();
        campaign.room = CampaignRoom::from_id(&room_id).unwrap();
        let caravan_id = campaign.active_regional_caravans[0].caravan_id.clone();
        campaign.interact_with_visible_caravan(true).unwrap();
        let caravan = campaign
            .active_regional_caravans
            .iter()
            .find(|caravan| caravan.caravan_id == caravan_id)
            .unwrap();
        assert!(caravan.guarded_by_player);
        assert_eq!(caravan.incident.as_deref(), Some("player_escort"));
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum FaultBackend {
        Recoverable,
        CorruptReceipt,
    }

    impl EconomyBackend for FaultBackend {
        fn backend_id(&self) -> &str {
            "fault-backend"
        }

        fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
            let mut receipt = EconomicReceipt::from_intent(
                format!("fault:{}", intent.intent_id),
                intent,
                "fault-backend",
                SettlementBackendKind::LocalTest,
                ReceiptStatus::FailedNetwork,
                intent.created_at_epoch,
            );
            receipt.reason = Some("simulated transport outage".to_string());
            if *self == Self::CorruptReceipt {
                receipt.intent_id = "wrong-intent".to_string();
            }
            Ok(receipt)
        }
    }

    struct CompensationFirstBackend;

    impl EconomyBackend for CompensationFirstBackend {
        fn backend_id(&self) -> &str {
            "compensation-first-test"
        }

        fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
            if matches!(
                intent.kind,
                EconomicIntentKind::Refund | EconomicIntentKind::Chargeback
            ) {
                return OfflineLocalEconomyBackend.execute(intent);
            }
            let mut receipt = EconomicReceipt::from_intent(
                format!("blocked:{}", intent.intent_id),
                intent,
                self.backend_id(),
                SettlementBackendKind::LocalTest,
                ReceiptStatus::FailedNetwork,
                intent.created_at_epoch,
            );
            receipt.reason = Some("regular FIFO head is deliberately unavailable".to_string());
            Ok(receipt)
        }
    }

    #[test]
    fn revision_twelve_separates_value_events_and_caps_wallet_issuance() {
        let campaign = CampaignSaveV1::default();
        assert_eq!(campaign.schema_revision, 12);
        assert_eq!(campaign.economy_mode, EconomyMode::OfflineLocal);
        assert_eq!(
            CampaignSaveV1::economy_asset_semantic("trnm-soft-credit").transferability,
            EconomyTransferability::Bound
        );
        assert_eq!(
            CampaignSaveV1::economy_asset_semantic("salvaged-alloy").transferability,
            EconomyTransferability::Tradeable
        );
        assert_eq!(
            CampaignSaveV1::economy_asset_semantic("rts-resource:cyan").transferability,
            EconomyTransferability::Ephemeral
        );

        let mut revision_ten_complete_epilogue = CampaignSaveV1 {
            schema_revision: 10,
            ending_epilogue_progress: 3,
            ending_epilogue_complete: true,
            ..CampaignSaveV1::default()
        };
        revision_ten_complete_epilogue.ensure_gameplay_defaults();
        assert_eq!(revision_ten_complete_epilogue.schema_revision, 12);
        assert_eq!(revision_ten_complete_epilogue.ending_epilogue_progress, 4);
        revision_ten_complete_epilogue.validate().unwrap();
    }

    #[test]
    fn cex_campaign_and_intent_scopes_are_account_isolated_and_stable() {
        let mut first = CampaignSaveV1::default();
        let mut second = CampaignSaveV1::default();
        first
            .bind_cex_economy_account("player-one", "11111111-1111-1111-1111-111111111111")
            .unwrap();
        second
            .bind_cex_economy_account("player-two", "22222222-2222-2222-2222-222222222222")
            .unwrap();
        assert_ne!(first.campaign_id, second.campaign_id);
        let first_scope = first.campaign_id.clone();
        first
            .bind_cex_economy_account("player-one", "11111111-1111-1111-1111-111111111111")
            .unwrap();
        assert_eq!(first.campaign_id, first_scope);

        for campaign in [&mut first, &mut second] {
            campaign
                .record_value_event(
                    "shared-local-event".to_string(),
                    "shared-local-intent".to_string(),
                    ValueEventSource::Battle,
                    ValueSettlementPolicy::DualTrack,
                    10,
                )
                .unwrap();
        }
        assert_ne!(
            first.pending_economic_intents[0].intent_id,
            second.pending_economic_intents[0].intent_id
        );
        assert_ne!(
            first.pending_economic_intents[0].idempotency_key.scope,
            second.pending_economic_intents[0].idempotency_key.scope
        );
        assert!(first.pending_economic_intents[0]
            .intent_id
            .starts_with(&first.campaign_id));
    }

    #[test]
    fn battle_wallet_issuance_is_bounded_for_a_full_simulated_year() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .bind_cex_economy_account("player-one", "11111111-1111-1111-1111-111111111111")
            .unwrap();
        let mut total_wallet = 0_i64;
        for day in 1..=365_u32 {
            campaign.world_clock.day = day;
            for event in 0..8 {
                campaign
                    .record_value_event(
                        format!("annual-battle:{day}:{event}"),
                        format!("annual-battle-intent:{day}:{event}"),
                        ValueEventSource::Battle,
                        ValueSettlementPolicy::DualTrack,
                        80,
                    )
                    .unwrap();
                total_wallet += campaign.value_events.last().unwrap().wallet_credit_delta;
            }
            assert_eq!(campaign.wallet_reward_issued_by_day[&day], 300);
            campaign.pending_economic_intents.clear();
            campaign.economic_idempotency_keys.clear();
            campaign.value_events.clear();
        }
        assert_eq!(total_wallet, 365 * BATTLE_WALLET_REWARD_DAILY_CAP);
        assert_eq!(
            CampaignSaveV1::economy_asset_semantic("trnm-soft-credit").transferability,
            EconomyTransferability::Bound
        );
        campaign.ensure_gameplay_defaults();
        campaign.validate().unwrap();
    }

    #[test]
    fn value_events_make_local_wallet_and_dual_track_issuance_explicit() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .bind_cex_economy_account("player-one", "11111111-1111-1111-1111-111111111111")
            .unwrap();
        let fixtures = [
            (
                "quest",
                ValueEventSource::RegionalQuest,
                ValueSettlementPolicy::LocalSoftOnly,
                11,
            ),
            (
                "chapter",
                ValueEventSource::Chapter,
                ValueSettlementPolicy::LocalSoftOnly,
                22,
            ),
            (
                "ending",
                ValueEventSource::Ending,
                ValueSettlementPolicy::WalletOnly,
                33,
            ),
            (
                "battle",
                ValueEventSource::Battle,
                ValueSettlementPolicy::DualTrack,
                44,
            ),
            (
                "future-trade",
                ValueEventSource::PlayerTrade,
                ValueSettlementPolicy::WalletOnly,
                55,
            ),
        ];
        for (id, source, policy, amount) in fixtures {
            campaign
                .record_value_event(
                    format!("value:{id}"),
                    format!("value-intent:{id}"),
                    source,
                    policy,
                    amount,
                )
                .unwrap();
        }

        assert_eq!(campaign.value_events.len(), 5);
        assert_eq!(campaign.pending_economic_intents.len(), 5);
        assert_eq!(campaign.value_events[0].wallet_credit_delta, 0);
        assert_eq!(campaign.value_events[2].local_soft_credit_delta, 0);
        assert_eq!(campaign.value_events[3].local_soft_credit_delta, 44);
        assert_eq!(campaign.value_events[3].wallet_credit_delta, 44);
        assert_eq!(
            campaign.pending_economic_intents[0].kind,
            EconomicIntentKind::CompleteContract
        );
        assert_eq!(
            campaign.pending_economic_intents[3].kind,
            EconomicIntentKind::ReleaseReward
        );
        assert_eq!(
            campaign.pending_economic_intents[3].metadata["double_issuance"],
            json!(true)
        );
        assert_eq!(
            campaign.pending_economic_intents[4].metadata["double_issuance"],
            json!(false)
        );
    }

    #[test]
    fn economy_outbox_is_exactly_once_and_fail_closed_across_reload() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .bind_cex_economy_account("player-one", "account-one")
            .unwrap();
        let receipt = SettlementReceiptV1 {
            contract_version: SETTLEMENT_RECEIPT_CONTRACT.to_string(),
            battle_id: "battle-economy-1".to_string(),
            seed_hash: "seed".to_string(),
            result_hash: "result".to_string(),
            campaign_revision_before: 1,
            campaign_revision_after: 2,
            outcome: BattleOutcome::Victory,
            experience_delta: 10,
            reputation_delta: 1,
            credit_delta: 80,
            loot_delta: Vec::new(),
            injury_delta_by_unit: BTreeMap::new(),
            economic_intent_id: Some("battle-reward:battle-economy-1".to_string()),
            economic_receipt_id: None,
            duplicate: false,
        };
        campaign.queue_battle_reward_economy(&receipt).unwrap();
        campaign.queue_battle_reward_economy(&receipt).unwrap();
        assert_eq!(campaign.pending_economic_intents.len(), 1);
        let report = campaign
            .reconcile_economy(&FaultBackend::Recoverable, 4)
            .unwrap();
        assert_eq!(report.recoverable_holds, 1);
        assert_eq!(campaign.pending_economic_intents.len(), 1);
        assert_eq!(campaign.wallet_snapshot.available_credits, 0);

        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("economy.json"));
        store.save_atomic(&campaign).unwrap();
        let mut loaded = store.load().unwrap();
        assert_eq!(loaded.pending_economic_intents.len(), 1);
        let report = loaded
            .reconcile_economy(&OfflineLocalEconomyBackend, 4)
            .unwrap();
        assert_eq!(report.applied, 1);
        assert_eq!(loaded.wallet_snapshot.available_credits, 80);
        assert!(loaded.pending_economic_intents.is_empty());
        assert_eq!(loaded.verified_economic_receipts.len(), 2);
        loaded
            .queue_battle_reward_economy(&receipt)
            .expect("duplicate queue is a no-op");
        assert!(loaded.pending_economic_intents.is_empty());
    }

    #[test]
    fn tradeable_purchase_runs_reserve_settle_consume_and_corrupt_receipts_dead_letter() {
        let selected_shop_item_index = ECONOMY_ITEM_CATALOG
            .iter()
            .position(|item| item.material)
            .unwrap();
        let mut campaign = CampaignSaveV1 {
            room: CampaignRoom::MarketWindPavilion,
            selected_shop_item_index,
            ..CampaignSaveV1::default()
        };
        let item_id = ECONOMY_ITEM_CATALOG[campaign.selected_shop_item_index]
            .id
            .to_string();
        campaign.begin_selected_tradeable_purchase().unwrap();
        let report = campaign
            .reconcile_economy(&OfflineLocalEconomyBackend, 8)
            .unwrap();
        assert_eq!(report.applied, 3);
        assert_eq!(
            campaign.pending_tradeable_purchases[0].stage,
            TradeablePurchaseStage::Consumed
        );
        assert!(campaign
            .progression
            .inventory
            .iter()
            .any(|loot| loot.item_id == item_id));

        let binding = campaign.effective_economy_binding();
        campaign
            .queue_economic_intent(EconomicIntentDraft {
                kind: EconomicIntentKind::ReleaseReward,
                term_id: "corrupt_test".to_string(),
                intent_id: "corrupt-test-intent".to_string(),
                binding,
                asset_id: "cex-wallet-credit".to_string(),
                quantity: 1,
                amount_credits: 1,
                metadata: json!({}),
                compensation: false,
            })
            .unwrap();
        let report = campaign
            .reconcile_economy(&FaultBackend::CorruptReceipt, 1)
            .unwrap();
        assert_eq!(report.hard_failures, 1);
        assert_eq!(campaign.economic_dead_letters.len(), 1);
    }

    #[test]
    fn compensation_lane_bypasses_regular_head_and_rolls_back_consumed_inventory() {
        let selected_shop_item_index = ECONOMY_ITEM_CATALOG
            .iter()
            .position(|item| item.material)
            .unwrap();
        let mut campaign = CampaignSaveV1 {
            room: CampaignRoom::MarketWindPavilion,
            selected_shop_item_index,
            ..CampaignSaveV1::default()
        };
        let item_id = ECONOMY_ITEM_CATALOG[selected_shop_item_index]
            .id
            .to_string();
        let purchase_id = campaign.begin_selected_tradeable_purchase().unwrap();
        campaign
            .reconcile_economy(&OfflineLocalEconomyBackend, 8)
            .unwrap();
        assert_eq!(
            campaign.pending_tradeable_purchases[0].stage,
            TradeablePurchaseStage::Consumed
        );
        assert!(campaign
            .progression
            .inventory
            .iter()
            .any(|loot| loot.item_id == item_id));

        campaign
            .queue_economic_intent(EconomicIntentDraft {
                kind: EconomicIntentKind::ReleaseReward,
                term_id: "blocked_regular_value_event".to_string(),
                intent_id: "blocked-regular-intent".to_string(),
                binding: campaign.effective_economy_binding(),
                asset_id: "cex-wallet-credit".to_string(),
                quantity: 1,
                amount_credits: 1,
                metadata: json!({}),
                compensation: false,
            })
            .unwrap();
        campaign.cancel_tradeable_purchase(&purchase_id).unwrap();
        assert!(!campaign
            .progression
            .inventory
            .iter()
            .any(|loot| loot.item_id == item_id));
        assert_eq!(campaign.pending_economic_compensations.len(), 1);
        assert_eq!(campaign.pending_economic_intents.len(), 1);

        let report = campaign
            .reconcile_economy(&CompensationFirstBackend, 2)
            .unwrap();
        assert_eq!(report.applied, 1);
        assert_eq!(report.recoverable_holds, 1);
        assert!(campaign.pending_economic_compensations.is_empty());
        assert_eq!(campaign.pending_economic_intents.len(), 1);
        assert_eq!(
            campaign.pending_tradeable_purchases[0].stage,
            TradeablePurchaseStage::Refunded
        );
        assert!(campaign.pending_tradeable_purchases[0].inventory_rolled_back);
    }

    #[test]
    fn connected_trade_requires_an_explicit_cex_market_account() {
        let selected_shop_item_index = ECONOMY_ITEM_CATALOG
            .iter()
            .position(|item| item.material)
            .unwrap();
        let mut campaign = CampaignSaveV1 {
            room: CampaignRoom::MarketWindPavilion,
            selected_shop_item_index,
            ..CampaignSaveV1::default()
        };
        campaign
            .bind_cex_economy_account("player-one", "11111111-1111-1111-1111-111111111111")
            .unwrap();
        assert!(campaign.begin_selected_tradeable_purchase().is_err());
        campaign
            .begin_selected_tradeable_purchase_with_seller_account(Some(
                "22222222-2222-2222-2222-222222222222",
            ))
            .unwrap();
        assert_eq!(
            campaign.pending_tradeable_purchases[0].seller.account_id,
            "22222222-2222-2222-2222-222222222222"
        );
    }

    #[test]
    fn v1_settings_migrate_to_subtitles_controls_and_master_volume_preference() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("settings.json");
        fs::write(
            &path,
            br#"{"contract_version":"trnm_player_settings_v1","low_motion":true,"input_mode":"keyboard_only"}"#,
        )
        .unwrap();
        let settings = PlayerSettingsStore::new(path).load_or_default().unwrap();
        assert_eq!(settings.contract_version, PLAYER_SETTINGS_CONTRACT);
        assert!(settings.subtitles);
        assert_eq!(settings.master_volume_percent, 80);
        assert_eq!(settings.control_scheme, ControlScheme::Classic);
    }

    fn economic_draft(
        campaign: &CampaignSaveV1,
        intent_id: impl Into<String>,
        compensation: bool,
    ) -> EconomicIntentDraft {
        EconomicIntentDraft {
            kind: EconomicIntentKind::CompleteContract,
            term_id: "command_atomicity_regression".to_string(),
            intent_id: intent_id.into(),
            binding: campaign.effective_economy_binding(),
            asset_id: "trnm-soft-credit".to_string(),
            quantity: 1,
            amount_credits: 0,
            metadata: json!({"test": "command_atomicity"}),
            compensation,
        }
    }

    fn fill_economic_lane(campaign: &mut CampaignSaveV1, compensation: bool, capacity: usize) {
        for index in 0..capacity {
            let request = economic_draft(
                campaign,
                format!("atomicity-fill-{compensation}-{index}"),
                compensation,
            );
            assert!(campaign
                .queue_economic_intent(request)
                .expect("lane fill remains valid"));
        }
    }

    fn assert_campaign_error_preserves_bytes<T: std::fmt::Debug>(
        campaign: &mut CampaignSaveV1,
        operation: impl FnOnce(&mut CampaignSaveV1) -> Result<T, CampaignError>,
    ) -> CampaignError {
        let before = serde_json::to_vec(campaign).expect("campaign state serializes");
        let error = operation(campaign).expect_err("operation must fail");
        assert_eq!(
            before,
            serde_json::to_vec(campaign).expect("campaign state still serializes"),
            "campaign command returned Err after changing persistent state"
        );
        error
    }

    #[test]
    fn full_economic_lanes_leave_idempotency_identity_retryable() {
        for (compensation, capacity, expected) in [
            (false, 128_usize, "economic outbox"),
            (true, 64_usize, "compensation lane"),
        ] {
            let mut campaign = CampaignSaveV1::default();
            fill_economic_lane(&mut campaign, compensation, capacity);
            let retry_id = format!("atomicity-retry-{compensation}");
            let error = assert_campaign_error_preserves_bytes(&mut campaign, |candidate| {
                candidate.queue_economic_intent(economic_draft(
                    candidate,
                    retry_id.clone(),
                    compensation,
                ))
            });
            assert!(error.to_string().contains(expected));
            if compensation {
                campaign.pending_economic_compensations.pop();
            } else {
                campaign.pending_economic_intents.pop();
            }
            assert!(campaign
                .queue_economic_intent(economic_draft(&campaign, retry_id, compensation))
                .expect("same identity remains retryable after capacity is released"));
        }
    }

    #[test]
    fn settlement_queue_failure_rolls_back_result_progression_and_daily_budget() {
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle(map()).unwrap();
        let result = terminal_result(&seed, BattleOutcome::Victory);
        fill_economic_lane(&mut campaign, false, 128);
        let error = assert_campaign_error_preserves_bytes(&mut campaign, |candidate| {
            candidate.submit_battle_result(result)
        });
        assert!(error.to_string().contains("economic outbox"));
        assert_eq!(campaign.phase, CampaignPhase::BattlePending);
        assert!(!campaign
            .wallet_reward_issued_by_day
            .contains_key(&campaign.world_clock.day));
    }

    #[test]
    fn reserve_failure_leaves_no_orphan_tradeable_purchase() {
        let selected_shop_item_index = ECONOMY_ITEM_CATALOG
            .iter()
            .position(|item| item.material)
            .expect("catalog has one tradeable item");
        let mut campaign = CampaignSaveV1 {
            room: CampaignRoom::MarketWindPavilion,
            selected_shop_item_index,
            ..CampaignSaveV1::default()
        };
        fill_economic_lane(&mut campaign, false, 128);
        let error = assert_campaign_error_preserves_bytes(&mut campaign, |candidate| {
            candidate.begin_selected_tradeable_purchase()
        });
        assert!(error.to_string().contains("economic outbox"));
        assert!(campaign.pending_tradeable_purchases.is_empty());
    }

    #[test]
    fn compensation_capacity_failure_restores_inventory_and_purchase_stage() {
        let selected_shop_item_index = ECONOMY_ITEM_CATALOG
            .iter()
            .position(|item| item.material)
            .expect("catalog has one tradeable item");
        let mut campaign = CampaignSaveV1 {
            room: CampaignRoom::MarketWindPavilion,
            selected_shop_item_index,
            ..CampaignSaveV1::default()
        };
        let purchase_id = campaign.begin_selected_tradeable_purchase().unwrap();
        campaign
            .reconcile_economy(&OfflineLocalEconomyBackend, 8)
            .unwrap();
        assert_eq!(
            campaign.pending_tradeable_purchases[0].stage,
            TradeablePurchaseStage::Consumed
        );
        fill_economic_lane(&mut campaign, true, 64);
        let error = assert_campaign_error_preserves_bytes(&mut campaign, |candidate| {
            candidate.cancel_tradeable_purchase(&purchase_id)
        });
        assert!(error.to_string().contains("compensation lane"));
        assert_eq!(
            campaign.pending_tradeable_purchases[0].stage,
            TradeablePurchaseStage::Consumed
        );
    }

    struct InvalidWalletSnapshotBackend;

    impl EconomyBackend for InvalidWalletSnapshotBackend {
        fn backend_id(&self) -> &str {
            "invalid-wallet-snapshot-test"
        }

        fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
            OfflineLocalEconomyBackend.execute(intent)
        }

        fn wallet_snapshot(
            &self,
            binding: &EconomyAccountBinding,
            cursor: u64,
        ) -> Result<Option<WalletSnapshot>, String> {
            Ok(Some(WalletSnapshot {
                account_id: binding.account_id.clone(),
                available_credits: -1,
                reserved_credits: 0,
                observed_at_cursor: cursor,
            }))
        }
    }

    #[test]
    fn invalid_reconciliation_snapshot_rolls_back_queue_receipts_wallet_and_cursor() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .bind_cex_economy_account("atomicity-player", "atomicity-account")
            .unwrap();
        let request = economic_draft(&campaign, "atomicity-invalid-wallet", false);
        campaign.queue_economic_intent(request).unwrap();
        let error = assert_campaign_error_preserves_bytes(&mut campaign, |candidate| {
            candidate.reconcile_economy(&InvalidWalletSnapshotBackend, 1)
        });
        assert!(error.to_string().contains("economy outbox"));
    }
