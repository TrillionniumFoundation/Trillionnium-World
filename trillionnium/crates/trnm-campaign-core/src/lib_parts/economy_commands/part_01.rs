impl CampaignSaveV1 {
    fn effective_economy_binding(&self) -> EconomyAccountBinding {
        self.economy_account_binding
            .clone()
            .unwrap_or_else(|| EconomyAccountBinding {
                actor_id: self.character.character_id.clone(),
                account_id: "trnm-offline-local-account".to_string(),
                binding_revision: self.revision,
            })
    }

    pub fn bind_cex_economy_account(
        &mut self,
        actor_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.bind_cex_economy_account_atomic_inner(actor_id, account_id)
        })
    }

    fn bind_cex_economy_account_atomic_inner(
        &mut self,
        actor_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<(), CampaignError> {
        let actor_id = actor_id.into();
        let account_id = account_id.into();
        if actor_id.trim().is_empty() || account_id.trim().is_empty() {
            return Err(CampaignError::InvalidState(
                "CEX actor_id and account_id are required".to_string(),
            ));
        }
        let same_binding = self
            .economy_account_binding
            .as_ref()
            .is_some_and(|binding| {
                binding.actor_id == actor_id && binding.account_id == account_id
            });
        if same_binding && self.campaign_id.starts_with("cex-campaign-") {
            self.economy_mode = EconomyMode::CexConnected;
            self.wallet_snapshot.account_id = account_id;
            return self.validate();
        }
        if self
            .economy_account_binding
            .as_ref()
            .is_some_and(|binding| {
                binding.account_id != account_id
                    && (!self.pending_economic_intents.is_empty()
                        || !self.pending_economic_compensations.is_empty()
                        || !self.verified_economic_receipts.is_empty()
                        || !self.pending_tradeable_purchases.is_empty())
            })
        {
            return Err(CampaignError::InvalidState(
                "a CEX account with economic history cannot be rebound to another account"
                    .to_string(),
            ));
        }
        self.campaign_id = Self::cex_scoped_campaign_id(&account_id, &self.campaign_id);
        self.economy_mode = EconomyMode::CexConnected;
        self.economy_account_binding = Some(EconomyAccountBinding {
            actor_id,
            account_id: account_id.clone(),
            binding_revision: self.revision,
        });
        self.wallet_snapshot.account_id = account_id;
        self.revision = self.revision.saturating_add(1);
        self.validate()
    }

    pub fn use_offline_local_economy(&mut self) {
        self.economy_mode = EconomyMode::OfflineLocal;
        self.economy_account_binding = None;
        self.wallet_snapshot.account_id = "trnm-offline-local-account".to_string();
    }

    pub fn economy_asset_semantic(item_id: &str) -> EconomyAssetSemantic {
        if item_id == "trnm-soft-credit" {
            return EconomyAssetSemantic::soft_credit();
        }
        if item_id == "cex-wallet-credit" {
            return EconomyAssetSemantic::wallet_credit();
        }
        if item_id.starts_with("rts-resource:") {
            return EconomyAssetSemantic::temporary_battle_resource(item_id);
        }
        let tradeable = ECONOMY_ITEM_CATALOG
            .iter()
            .find(|item| item.id == item_id)
            .is_some_and(|item| item.material);
        EconomyAssetSemantic {
            asset_id: item_id.to_string(),
            asset_class: if tradeable {
                EconomyAssetClass::TradeableItem
            } else {
                EconomyAssetClass::BoundGameplayItem
            },
            transferability: if tradeable {
                EconomyTransferability::Tradeable
            } else {
                EconomyTransferability::Bound
            },
            settlement_authority: if tradeable {
                CEX_SETTLEMENT_BACKEND_ID.to_string()
            } else {
                "trnm-campaign-core".to_string()
            },
        }
    }

    fn queue_economic_intent(&mut self, draft: EconomicIntentDraft) -> Result<bool, CampaignError> {
        let EconomicIntentDraft {
            kind,
            term_id,
            intent_id,
            binding,
            asset_id,
            quantity,
            amount_credits,
            metadata,
            compensation,
        } = draft;
        let idempotency_key = format!("{}:{}", self.campaign_id, intent_id);
        if self.economic_idempotency_keys.contains(&idempotency_key) {
            return Ok(false);
        }
        let semantic = Self::economy_asset_semantic(&asset_id);
        let intent = EconomicIntent {
            protocol_version: TERM_EXCHANGE_PROTOCOL_VERSION.to_string(),
            intent_id,
            term_id,
            term_version: "v1".to_string(),
            domain: "trnm_game".to_string(),
            kind,
            idempotency_key: EconomyIdempotencyKey {
                scope: self.campaign_id.clone(),
                key: idempotency_key.clone(),
            },
            actors: vec![EconomyActorRef {
                actor_id: binding.actor_id,
                actor_kind: "trnm_player".to_string(),
                account_id: Some(binding.account_id),
            }],
            assets: vec![EconomyAssetRef {
                asset_id,
                asset_kind: format!("{:?}", semantic.asset_class).to_ascii_lowercase(),
                quantity,
                unit: "credits".to_string(),
            }],
            amount_credits: Some(amount_credits),
            currency: Some("wallet_credits".to_string()),
            metadata,
            created_at_epoch: i64::from(self.world_clock.day) * 86_400
                + i64::from(self.world_clock.minute_of_day) * 60,
        };
        intent.validate().map_err(CampaignError::InvalidState)?;
        // The queue slot and its idempotency tombstone form one admission unit.
        // A capacity error must leave the identity retryable.
        if compensation {
            if self.pending_economic_compensations.len() >= 64 {
                return Err(CampaignError::InvalidState(
                    "economic compensation lane reached its bounded capacity".to_string(),
                ));
            }
        } else if self.pending_economic_intents.len() >= 128 {
            return Err(CampaignError::InvalidState(
                "economic outbox reached its bounded capacity".to_string(),
            ));
        }
        self.economic_idempotency_keys.insert(idempotency_key);
        if compensation {
            self.pending_economic_compensations.push(intent);
        } else {
            self.pending_economic_intents.push(intent);
        }
        Ok(true)
    }

    fn record_value_event(
        &mut self,
        event_id: String,
        intent_id: String,
        source: ValueEventSource,
        policy: ValueSettlementPolicy,
        local_soft_credit_delta: i64,
    ) -> Result<(), CampaignError> {
        if self
            .value_events
            .iter()
            .any(|event| event.event_id == event_id)
        {
            return Ok(());
        }
        let intent_id = self.scoped_economic_intent_id(&intent_id);
        let recorded_local_soft_delta = if policy == ValueSettlementPolicy::WalletOnly {
            0
        } else {
            local_soft_credit_delta
        };
        let wallet_credit_delta = match policy {
            ValueSettlementPolicy::LocalSoftOnly => 0,
            ValueSettlementPolicy::WalletOnly => local_soft_credit_delta.max(0),
            ValueSettlementPolicy::DualTrack => {
                let issued = self
                    .wallet_reward_issued_by_day
                    .get(&self.world_clock.day)
                    .copied()
                    .unwrap_or_default();
                local_soft_credit_delta
                    .clamp(0, BATTLE_WALLET_REWARD_PER_EVENT_CAP)
                    .min(BATTLE_WALLET_REWARD_DAILY_CAP.saturating_sub(issued))
            }
        };
        if policy == ValueSettlementPolicy::DualTrack && wallet_credit_delta > 0 {
            self.wallet_reward_issued_by_day
                .entry(self.world_clock.day)
                .and_modify(|issued| *issued = issued.saturating_add(wallet_credit_delta))
                .or_insert(wallet_credit_delta);
        }
        let kind = if wallet_credit_delta > 0 {
            EconomicIntentKind::ReleaseReward
        } else {
            EconomicIntentKind::CompleteContract
        };
        let asset_id = if wallet_credit_delta > 0 {
            "cex-wallet-credit"
        } else {
            "trnm-soft-credit"
        };
        self.queue_economic_intent(EconomicIntentDraft {
            kind,
            term_id: format!("trnm_value_event:{source:?}").to_ascii_lowercase(),
            intent_id: intent_id.clone(),
            binding: self.effective_economy_binding(),
            asset_id: asset_id.to_string(),
            quantity: local_soft_credit_delta.max(0),
            amount_credits: wallet_credit_delta,
            metadata: json!({
                "value_event_id": event_id.clone(),
                "source": source,
                "payout_policy": policy,
                "local_soft_credit_delta": recorded_local_soft_delta,
                "wallet_credit_delta": wallet_credit_delta,
                "double_issuance": policy == ValueSettlementPolicy::DualTrack,
                "wallet_reward_per_event_cap": BATTLE_WALLET_REWARD_PER_EVENT_CAP,
                "wallet_reward_daily_cap": BATTLE_WALLET_REWARD_DAILY_CAP,
                "soft_credit_convertible_to_wallet": false,
            }),
            compensation: false,
        })?;
        self.value_events.push(ValueEventRecord {
            event_id,
            source,
            policy,
            local_soft_credit_delta: recorded_local_soft_delta,
            wallet_credit_delta,
            economic_intent_id: intent_id,
            economic_receipt_id: None,
        });
        if self.economy_mode == EconomyMode::OfflineLocal {
            self.reconcile_economy(&OfflineLocalEconomyBackend, 8)?;
        }
        Ok(())
    }

    fn queue_battle_reward_economy(
        &mut self,
        receipt: &SettlementReceiptV1,
    ) -> Result<(), CampaignError> {
        if receipt.duplicate || receipt.credit_delta <= 0 {
            return Ok(());
        }
        self.record_value_event(
            format!("battle:{}", receipt.battle_id),
            format!("battle-reward:{}", receipt.battle_id),
            ValueEventSource::Battle,
            ValueSettlementPolicy::DualTrack,
            receipt.credit_delta,
        )
    }

    pub fn begin_selected_tradeable_purchase(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.begin_selected_tradeable_purchase_atomic_inner()
        })
    }

    fn begin_selected_tradeable_purchase_atomic_inner(&mut self) -> Result<String, CampaignError> {
        self.begin_selected_tradeable_purchase_with_seller_account(None)
    }

    pub fn begin_selected_tradeable_purchase_with_seller_account(
        &mut self,
        connected_seller_account_id: Option<&str>,
    ) -> Result<String, CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.begin_selected_tradeable_purchase_with_seller_account_atomic_inner(
                connected_seller_account_id,
            )
        })
    }

    fn begin_selected_tradeable_purchase_with_seller_account_atomic_inner(
        &mut self,
        connected_seller_account_id: Option<&str>,
    ) -> Result<String, CampaignError> {
        let region_id = self.require_regional_market()?;
        let item = ECONOMY_ITEM_CATALOG
            .get(self.selected_shop_item_index % ECONOMY_ITEM_CATALOG.len())
            .ok_or_else(|| CampaignError::InvalidState("shop selection is missing".to_string()))?;
        let semantic = Self::economy_asset_semantic(item.id);
        if semantic.transferability != EconomyTransferability::Tradeable {
            return Err(CampaignError::InvalidState(format!(
                "{} is bound to local gameplay and never enters CEX",
                item.display_name
            )));
        }
        let (stock, demand) = self.regional_market_state(region_id, item.id);
        if stock == 0 {
            return Err(CampaignError::InvalidState(format!(
                "{} is out of regional stock",
                item.display_name
            )));
        }
        let price = market_price_with_state(item.id, self.world_clock.day, stock, demand, true)
            .ok_or_else(|| CampaignError::InvalidState("tradeable price is missing".to_string()))?;
        let buyer = self.effective_economy_binding();
        let seller_account_id = match (self.economy_mode, connected_seller_account_id) {
            (EconomyMode::CexConnected, Some(account_id)) if !account_id.trim().is_empty() => {
                account_id.to_string()
            }
            (EconomyMode::CexConnected, _) => {
                return Err(CampaignError::InvalidState(
                    "connected trade requires a configured CEX market account".to_string(),
                ));
            }
            (EconomyMode::OfflineLocal, _) => format!("offline-market:{region_id}"),
        };
        let seller = EconomyAccountBinding {
            actor_id: format!("trnm-market:{region_id}"),
            account_id: seller_account_id,
            binding_revision: self.revision,
        };
        let buyer_account_id = buyer.account_id.clone();
        let seller_account_id_for_metadata = seller.account_id.clone();
        let purchase_id = format!(
            "trade:{}:{}:{}",
            self.campaign_id, item.id, self.reconciliation_cursor
        );
        let reserve_intent_id = format!("{purchase_id}:reserve");
        self.pending_tradeable_purchases
            .push(PendingTradeablePurchase {
                purchase_id: purchase_id.clone(),
                item_id: item.id.to_string(),
                quantity: 1,
                price_wallet_credits: price,
                buyer: buyer.clone(),
                seller,
                stage: TradeablePurchaseStage::ReservePending,
                reserve_intent_id: reserve_intent_id.clone(),
                settle_intent_id: None,
                consume_intent_id: None,
                refund_intent_id: None,
                inventory_rolled_back: false,
            });
        self.queue_economic_intent(EconomicIntentDraft {
            kind: EconomicIntentKind::Reserve,
            term_id: "trnm_tradeable_purchase".to_string(),
            intent_id: reserve_intent_id,
            binding: buyer,
            asset_id: item.id.to_string(),
            quantity: 1,
            amount_credits: price,
            metadata: json!({
                "purchase_id": purchase_id,
                "stage": "reserve",
                "region_id": region_id,
                "buyer_account_id": buyer_account_id,
                "seller_account_id": seller_account_id_for_metadata,
            }),
            compensation: false,
        })?;
        Ok(purchase_id)
    }

    pub fn cancel_tradeable_purchase(&mut self, purchase_id: &str) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.cancel_tradeable_purchase_atomic_inner(purchase_id)
        })
    }

    fn cancel_tradeable_purchase_atomic_inner(
        &mut self,
        purchase_id: &str,
    ) -> Result<(), CampaignError> {
        let index = self
            .pending_tradeable_purchases
            .iter()
            .position(|purchase| purchase.purchase_id == purchase_id)
            .ok_or_else(|| {
                CampaignError::InvalidState("tradeable purchase is missing".to_string())
            })?;
        let mut purchase = self.pending_tradeable_purchases[index].clone();
        if matches!(
            purchase.stage,
            TradeablePurchaseStage::RefundPending | TradeablePurchaseStage::Refunded
        ) {
            return Ok(());
        }
        if purchase.stage == TradeablePurchaseStage::ReservePending {
            self.pending_economic_intents
                .retain(|intent| intent.intent_id != purchase.reserve_intent_id);
            self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::Refunded;
            return Ok(());
        }
        let (kind, escrow_open) = match purchase.stage {
            TradeablePurchaseStage::SellerSettled | TradeablePurchaseStage::BuyerConsumePending => {
                (EconomicIntentKind::Refund, true)
            }
            TradeablePurchaseStage::Consumed => (EconomicIntentKind::Chargeback, true),
            TradeablePurchaseStage::Reserved | TradeablePurchaseStage::SellerSettlementPending => {
                self.pending_economic_intents.retain(|intent| {
                    purchase.settle_intent_id.as_deref() != Some(intent.intent_id.as_str())
                });
                (EconomicIntentKind::Refund, false)
            }
            TradeablePurchaseStage::HardFailed => {
                return Err(CampaignError::InvalidState(
                    "hard-failed purchase requires operator reconciliation".to_string(),
                ));
            }
            TradeablePurchaseStage::ReservePending
            | TradeablePurchaseStage::RefundPending
            | TradeablePurchaseStage::Refunded => unreachable!(),
        };
        if purchase.stage == TradeablePurchaseStage::BuyerConsumePending {
            self.pending_economic_intents.retain(|intent| {
                purchase.consume_intent_id.as_deref() != Some(intent.intent_id.as_str())
            });
        }
        if purchase.stage == TradeablePurchaseStage::Consumed && !purchase.inventory_rolled_back {
            consume_loot(
                &mut self.progression.inventory,
                &purchase.item_id,
                purchase.quantity,
            )?;
            purchase.inventory_rolled_back = true;
            self.pending_tradeable_purchases[index].inventory_rolled_back = true;
        }
        let intent_id = format!("{}:recovery", purchase.purchase_id);
        self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::RefundPending;
        self.pending_tradeable_purchases[index].refund_intent_id = Some(intent_id.clone());
        let mut metadata = json!({
            "stage": if kind == EconomicIntentKind::Chargeback { "chargeback" } else { "refund" },
            "buyer_account_id": purchase.buyer.account_id.clone(),
            "seller_account_id": purchase.seller.account_id.clone(),
            "inventory_rolled_back": purchase.inventory_rolled_back,
        });
        if escrow_open {
            metadata["purchase_id"] = json!(purchase.purchase_id);
        } else {
            metadata["reservation_purchase_id"] = json!(purchase.purchase_id);
        }
        self.queue_economic_intent(EconomicIntentDraft {
            kind,
            term_id: "trnm_tradeable_purchase_recovery".to_string(),
            intent_id,
            binding: purchase.buyer,
            asset_id: purchase.item_id,
            quantity: i64::from(purchase.quantity),
            amount_credits: purchase.price_wallet_credits,
            metadata,
            compensation: true,
        })?;
        Ok(())
    }

    fn apply_verified_economic_receipt(
        &mut self,
        intent: &EconomicIntent,
        receipt: &EconomicReceipt,
    ) -> Result<(), CampaignError> {
        receipt
            .validate_for(intent)
            .map_err(CampaignError::InvalidState)?;
        let bound_account = self.effective_economy_binding().account_id;
        let applies_to_bound_wallet = intent
            .actors
            .first()
            .and_then(|actor| actor.account_id.as_deref())
            == Some(bound_account.as_str());
        let amount = intent.amount_credits.unwrap_or_default().max(0);
        if applies_to_bound_wallet && receipt.allows_progression() {
            match intent.kind {
                EconomicIntentKind::ReleaseReward => {
                    self.wallet_snapshot.available_credits = self
                        .wallet_snapshot
                        .available_credits
                        .saturating_add(amount);
                }
                EconomicIntentKind::Settle => {
                    if intent.term_id == "trnm_tradeable_purchase" {
                        self.wallet_snapshot.reserved_credits =
                            self.wallet_snapshot.reserved_credits.saturating_sub(amount);
                    } else {
                        self.wallet_snapshot.available_credits = self
                            .wallet_snapshot
                            .available_credits
                            .saturating_add(amount);
                    }
                }
                EconomicIntentKind::Reserve => {
                    self.wallet_snapshot.available_credits = self
                        .wallet_snapshot
                        .available_credits
                        .saturating_sub(amount);
                    self.wallet_snapshot.reserved_credits =
                        self.wallet_snapshot.reserved_credits.saturating_add(amount);
                }
                EconomicIntentKind::Consume if intent.term_id != "trnm_tradeable_purchase" => {
                    self.wallet_snapshot.reserved_credits =
                        self.wallet_snapshot.reserved_credits.saturating_sub(amount);
                }
                EconomicIntentKind::Refund => {
                    self.wallet_snapshot.available_credits = self
                        .wallet_snapshot
                        .available_credits
                        .saturating_add(amount);
                    self.wallet_snapshot.reserved_credits =
                        self.wallet_snapshot.reserved_credits.saturating_sub(amount);
                }
                EconomicIntentKind::Chargeback => {
                    self.wallet_snapshot.available_credits = self
                        .wallet_snapshot
                        .available_credits
                        .saturating_add(amount);
                }
                _ => {}
            }
        }

        if receipt.allows_progression() {
            if let Some(event) = self
                .value_events
                .iter_mut()
                .find(|event| event.economic_intent_id == intent.intent_id)
            {
                event.economic_receipt_id = Some(receipt.receipt_id.clone());
            }
        }

        if receipt.allows_progression() {
            if let Some(settlement) = self.settlement_receipts.iter_mut().find(|settlement| {
                settlement.economic_intent_id.as_deref() == Some(intent.intent_id.as_str())
            }) {
                settlement.economic_receipt_id = Some(receipt.receipt_id.clone());
            }
        }

        let Some(index) = self
            .pending_tradeable_purchases
            .iter()
            .position(|purchase| {
                purchase.reserve_intent_id == intent.intent_id
                    || purchase.settle_intent_id.as_deref() == Some(intent.intent_id.as_str())
                    || purchase.consume_intent_id.as_deref() == Some(intent.intent_id.as_str())
                    || purchase.refund_intent_id.as_deref() == Some(intent.intent_id.as_str())
            })
        else {
            return Ok(());
        };
        if !receipt.allows_progression() {
            if receipt.progression_class == ReceiptProgressionClass::HardFail {
                self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::HardFailed;
            }
            return Ok(());
        }
        let purchase = self.pending_tradeable_purchases[index].clone();
        if purchase.reserve_intent_id == intent.intent_id {
            let next_id = format!("{}:settle", purchase.purchase_id);
            self.pending_tradeable_purchases[index].stage =
                TradeablePurchaseStage::SellerSettlementPending;
            self.pending_tradeable_purchases[index].settle_intent_id = Some(next_id.clone());
            self.queue_economic_intent(EconomicIntentDraft {
                kind: EconomicIntentKind::Settle,
                term_id: "trnm_tradeable_purchase".to_string(),
                intent_id: next_id,
                binding: purchase.buyer.clone(),
                asset_id: purchase.item_id.clone(),
                quantity: i64::from(purchase.quantity),
                amount_credits: purchase.price_wallet_credits,
                metadata: json!({
                    "purchase_id": purchase.purchase_id,
                    "stage": "escrow_hold",
                    "reserve_intent_id": purchase.reserve_intent_id,
                    "buyer_account_id": purchase.buyer.account_id,
                    "seller_account_id": purchase.seller.account_id,
                    "seller_reversible_window_seconds": SELLER_REVERSIBLE_WINDOW_SECONDS,
                }),
                compensation: false,
            })?;
        } else if purchase.settle_intent_id.as_deref() == Some(intent.intent_id.as_str()) {
            let next_id = format!("{}:consume", purchase.purchase_id);
            self.pending_tradeable_purchases[index].stage =
                TradeablePurchaseStage::BuyerConsumePending;
            self.pending_tradeable_purchases[index].consume_intent_id = Some(next_id.clone());
            self.queue_economic_intent(EconomicIntentDraft {
                kind: EconomicIntentKind::Consume,
                term_id: "trnm_tradeable_purchase".to_string(),
                intent_id: next_id,
                binding: purchase.buyer.clone(),
                asset_id: purchase.item_id.clone(),
                quantity: i64::from(purchase.quantity),
                amount_credits: purchase.price_wallet_credits,
                metadata: json!({
                    "purchase_id": purchase.purchase_id,
                    "stage": "buyer_consume_and_seller_commit",
                    "buyer_account_id": purchase.buyer.account_id,
                    "seller_account_id": purchase.seller.account_id,
                }),
                compensation: false,
            })?;
        } else if purchase.consume_intent_id.as_deref() == Some(intent.intent_id.as_str()) {
            self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::Consumed;
            merge_loot(
                &mut self.progression.inventory,
                &[LootStack {
                    item_id: purchase.item_id,
                    quantity: purchase.quantity,
                }],
            );
        } else if purchase.refund_intent_id.as_deref() == Some(intent.intent_id.as_str()) {
            self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::Refunded;
        }
        Ok(())
    }

    pub fn reconcile_economy<B: EconomyBackend>(
        &mut self,
        backend: &B,
        max_intents: usize,
    ) -> Result<EconomyReconciliationReport, CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.reconcile_economy_atomic_inner(backend, max_intents)
        })
    }

    fn reconcile_economy_atomic_inner<B: EconomyBackend>(
        &mut self,
        backend: &B,
        max_intents: usize,
    ) -> Result<EconomyReconciliationReport, CampaignError> {
        let mut report = EconomyReconciliationReport::default();
        while (report.attempted as usize) < max_intents {
            let compensation = !self.pending_economic_compensations.is_empty();
            let intent = if compensation {
                self.pending_economic_compensations.first().cloned()
            } else {
                self.pending_economic_intents.first().cloned()
            };
            let Some(intent) = intent else { break };
            report.attempted = report.attempted.saturating_add(1);
            let receipt = match backend.execute(&intent) {
                Ok(receipt) => receipt,
                Err(error) => {
                    report.recoverable_holds = report.recoverable_holds.saturating_add(1);
                    report.last_error = Some(error);
                    break;
                }
            };
            if let Err(error) = receipt.validate_for(&intent) {
                report.hard_failures = report.hard_failures.saturating_add(1);
                report.last_error = Some(error);
                self.economic_dead_letters.push(intent);
                if compensation {
                    self.pending_economic_compensations.remove(0);
                } else {
                    self.pending_economic_intents.remove(0);
                }
                continue;
            }
            if let Some(existing) = self
                .verified_economic_receipts
                .iter_mut()
                .find(|existing| existing.receipt_id == receipt.receipt_id)
            {
                *existing = receipt.clone();
            } else {
                self.verified_economic_receipts.push(receipt.clone());
            }
            match receipt.progression_class {
                ReceiptProgressionClass::ProgressionAllowed
                | ReceiptProgressionClass::TerminalSkip => {
                    if compensation {
                        self.pending_economic_compensations.remove(0);
                    } else {
                        self.pending_economic_intents.remove(0);
                    }
                    self.apply_verified_economic_receipt(&intent, &receipt)?;
                    report.applied = report.applied.saturating_add(1);
                }
                ReceiptProgressionClass::RecoverableHold => {
                    report.recoverable_holds = report.recoverable_holds.saturating_add(1);
                    report.last_error = receipt.reason.clone();
                    break;
                }
                ReceiptProgressionClass::HardFail => {
                    if compensation {
                        self.pending_economic_compensations.remove(0);
                    } else {
                        self.pending_economic_intents.remove(0);
                    }
                    self.economic_dead_letters.push(intent.clone());
                    self.apply_verified_economic_receipt(&intent, &receipt)?;
                    report.hard_failures = report.hard_failures.saturating_add(1);
                }
            }
            self.reconciliation_cursor = self.reconciliation_cursor.saturating_add(1);
        }
        if let Some(binding) = self.economy_account_binding.as_ref() {
            if let Ok(Some(snapshot)) = backend.wallet_snapshot(binding, self.reconciliation_cursor)
            {
                self.wallet_snapshot = snapshot;
            }
        }
        if self.verified_economic_receipts.len() > 256 {
            let keep_from = self.verified_economic_receipts.len() - 256;
            self.verified_economic_receipts.drain(..keep_from);
        }
        report.remaining =
            self.pending_economic_compensations.len() + self.pending_economic_intents.len();
        self.validate()?;
        Ok(report)
    }
}

