impl CampaignSaveV1 {
    pub fn cycle_regional_quest_approach(&mut self) -> Result<QuestApproach, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.cycle_regional_quest_approach_atomic_inner()
        })
    }

    fn cycle_regional_quest_approach_atomic_inner(
        &mut self,
    ) -> Result<QuestApproach, CampaignError> {
        self.require_town()?;
        let runtime = self.active_regional_quest_runtime.as_mut().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        runtime.approach = runtime.approach.next();
        self.revision += 1;
        Ok(runtime.approach)
    }

    pub fn fail_active_regional_quest(&mut self, reason: &str) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.fail_active_regional_quest_atomic_inner(reason)
        })
    }

    fn fail_active_regional_quest_atomic_inner(
        &mut self,
        reason: &str,
    ) -> Result<(), CampaignError> {
        let quest_id = self.active_regional_quest_id.clone().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .expect("active regional quest remains catalog bound");
        let failures = self
            .regional_quest_failure_counts
            .entry(quest_id.clone())
            .or_default();
        *failures = failures.saturating_add(1);
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(quest_runtime_rule(definition.archetype).failure_reputation);
        self.regional_quest_states
            .insert(quest_id.clone(), QuestState::Failed);
        self.progression
            .world_flags
            .insert(format!("regional_quest_{quest_id}_failed_{}", *failures));
        let authored_failure = quest_narrative(&quest_id)
            .map(|narrative| narrative.failure)
            .unwrap_or(reason);
        self.combat_log.push(CombatLogBeat {
            kind: "quest_failure".to_string(),
            text: format!("{} failed: {reason}. {authored_failure}", definition.title,),
        });
        self.active_regional_quest_id = None;
        self.active_regional_quest_step = 0;
        self.active_regional_quest_runtime = None;
        self.revision += 1;
        Ok(())
    }

    fn region_id_for_room_id(room_id: &str) -> &'static str {
        match room_id {
            GLASS_BASIN_WAYHOUSE_ROOM
            | DEEP_RELAY_ROOM
            | GLASS_REED_MARSH_ROOM
            | BASIN_OBSERVATORY_ROOM => "glass_basin",
            MOON_BRIDGE_ROOM
            | EMBER_ORCHARD_EDGE_ROOM
            | ASH_BEACON_FIELD_ROOM
            | CINDER_REFUGE_ROOM => "ashen_fringe",
            OUTER_SIGNAL_ROAD_ROOM | RELAY_QUARTER_ROOM => "signal_road",
            _ => "mirror_city",
        }
    }

    pub fn current_market_region_id(&self) -> Option<&'static str> {
        match self.room {
            CampaignRoom::MarketWindPavilion => Some("mirror_city"),
            CampaignRoom::RelayQuarter => Some("signal_road"),
            CampaignRoom::GlassBasinWayhouse => Some("glass_basin"),
            CampaignRoom::CinderRefuge => Some("ashen_fringe"),
            _ => None,
        }
    }

    fn require_regional_market(&self) -> Result<&'static str, CampaignError> {
        self.require_town()?;
        self.current_market_region_id().ok_or_else(|| {
            CampaignError::InvalidState(
                "regional trading requires a city pavilion, relay, wayhouse or refuge market"
                    .to_string(),
            )
        })
    }

    pub fn regional_market_state(&self, region_id: &str, item_id: &str) -> (u16, i16) {
        (
            self.regional_market_stock
                .get(region_id)
                .and_then(|stock| stock.get(item_id))
                .copied()
                .unwrap_or_default(),
            self.regional_market_demand
                .get(region_id)
                .and_then(|demand| demand.get(item_id))
                .copied()
                .unwrap_or_default(),
        )
    }

    fn set_regional_market_state(
        &mut self,
        region_id: &str,
        item_id: &str,
        stock: u16,
        demand: i16,
    ) {
        self.regional_market_stock
            .entry(region_id.to_string())
            .or_default()
            .insert(item_id.to_string(), stock.min(99));
        self.regional_market_demand
            .entry(region_id.to_string())
            .or_default()
            .insert(item_id.to_string(), demand.clamp(-20, 20));
        if region_id == "mirror_city" {
            self.market_stock.insert(item_id.to_string(), stock.min(99));
            self.market_demand
                .insert(item_id.to_string(), demand.clamp(-20, 20));
        }
    }

    fn caravan_route(from_region: &str, to_region: &str) -> Vec<String> {
        let hub = |region: &str| match region {
            "signal_road" => RELAY_QUARTER_ROOM,
            "glass_basin" => GLASS_BASIN_WAYHOUSE_ROOM,
            "ashen_fringe" => CINDER_REFUGE_ROOM,
            _ => CARAVAN_YARD_ROOM,
        };
        let mut route = vec![hub(from_region).to_string()];
        for waypoint in [
            OUTER_SIGNAL_ROAD_ROOM,
            MOON_BRIDGE_ROOM,
            ASH_BEACON_FIELD_ROOM,
        ] {
            if waypoint != route[0] && waypoint != hub(to_region) {
                route.push(waypoint.to_string());
            }
        }
        route.push(hub(to_region).to_string());
        route
    }

    pub fn visible_regional_caravan(&self) -> Option<&RegionalCaravanState> {
        self.active_regional_caravans
            .iter()
            .find(|caravan| caravan.current_room_id() == Some(self.room.id()))
    }

    pub fn interact_with_visible_caravan(
        &mut self,
        protect: bool,
    ) -> Result<String, CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.interact_with_visible_caravan_atomic_inner(protect)
        })
    }

    fn interact_with_visible_caravan_atomic_inner(
        &mut self,
        protect: bool,
    ) -> Result<String, CampaignError> {
        self.require_town()?;
        let room_id = self.room.id().to_string();
        let index = self
            .active_regional_caravans
            .iter()
            .position(|caravan| caravan.current_room_id() == Some(room_id.as_str()))
            .ok_or_else(|| {
                CampaignError::InvalidState("no caravan is currently visible here".to_string())
            })?;
        let caravan = &mut self.active_regional_caravans[index];
        let text = if protect {
            caravan.guarded_by_player = true;
            caravan.risk = caravan.risk.saturating_sub(4);
            caravan.integrity = caravan.integrity.saturating_add(15).min(100);
            caravan.incident = Some("player_escort".to_string());
            self.progression.world_flags.insert(format!(
                "caravan_{}_escorted",
                caravan.caravan_id.replace('-', "_")
            ));
            self.character.attributes.reputation =
                self.character.attributes.reputation.saturating_add(2);
            format!(
                "The player escorts {} toward {}; risk falls to {}.",
                caravan.caravan_id, caravan.to_region_id, caravan.risk
            )
        } else {
            let seized = caravan.quantity.min(1);
            caravan.quantity = caravan.quantity.saturating_sub(seized);
            caravan.integrity = caravan.integrity.saturating_sub(25);
            caravan.incident = Some("player_seizure".to_string());
            self.progression.credits += i64::from(seized) * 12;
            self.character.attributes.reputation =
                self.character.attributes.reputation.saturating_sub(3);
            self.progression.world_flags.insert(format!(
                "caravan_{}_seized",
                caravan.caravan_id.replace('-', "_")
            ));
            for bond in self.npc_bonds.values_mut() {
                *bond = bond.saturating_sub(1).clamp(-100, 100);
            }
            format!(
                "The player seizes {seized} {} from {}; regional witnesses remember it.",
                caravan.item_id, caravan.caravan_id
            )
        };
        self.combat_log.push(CombatLogBeat {
            kind: "caravan_encounter".to_string(),
            text: text.clone(),
        });
        self.revision += 1;
        Ok(text)
    }

    fn run_regional_logistics(&mut self) {
        let mut travelling = std::mem::take(&mut self.active_regional_caravans);
        for mut caravan in travelling.drain(..) {
            if caravan.route_room_ids.is_empty() {
                caravan.route_room_ids =
                    Self::caravan_route(&caravan.from_region_id, &caravan.to_region_id);
            }
            caravan.progress_legs = caravan.progress_legs.saturating_add(1);
            caravan.route_index = (caravan.route_index + 1).min(caravan.route_room_ids.len() - 1);
            if caravan.risk >= 7
                && !caravan.guarded_by_player
                && caravan.route_index + 1 < caravan.route_room_ids.len()
                && caravan.incident.is_none()
            {
                caravan.integrity = caravan.integrity.saturating_sub(35);
                caravan.quantity = caravan.quantity.saturating_sub(1);
                caravan.incident = Some("road_ambush".to_string());
                self.progression.world_flags.insert(format!(
                    "caravan_{}_ambushed",
                    caravan.caravan_id.replace('-', "_")
                ));
            }
            if caravan.route_index + 1 < caravan.route_room_ids.len() {
                self.active_regional_caravans.push(caravan);
                continue;
            }
            let delivered = caravan
                .quantity
                .saturating_sub(u16::from(caravan.integrity < 50));
            let (stock, demand) =
                self.regional_market_state(&caravan.to_region_id, &caravan.item_id);
            self.set_regional_market_state(
                &caravan.to_region_id,
                &caravan.item_id,
                stock.saturating_add(delivered),
                demand.saturating_sub(delivered as i16),
            );
            self.regional_logistics.push(RegionalMarketTransfer {
                item_id: caravan.item_id,
                from_region_id: caravan.from_region_id,
                to_region_id: caravan.to_region_id,
                quantity: delivered,
                day: self.world_clock.day,
            });
            if self.regional_logistics.len() > 64 {
                let keep_from = self.regional_logistics.len() - 64;
                self.regional_logistics.drain(..keep_from);
            }
        }
        let item = &ECONOMY_ITEM_CATALOG[(self.world_clock.day as usize
            + usize::from(self.world_clock.minute_of_day / 120))
            % ECONOMY_ITEM_CATALOG.len()];
        let mut regions = MARKET_REGION_IDS
            .into_iter()
            .map(|region| {
                let (stock, demand) = self.regional_market_state(region, item.id);
                (region, stock, demand)
            })
            .collect::<Vec<_>>();
        regions.sort_by_key(|(_, stock, demand)| (i32::from(*stock) - i32::from(*demand), *stock));
        let Some(&(to_region, to_stock, _to_demand)) = regions.first() else {
            return;
        };
        let Some(&(from_region, from_stock, from_demand)) = regions.last() else {
            return;
        };
        if from_region == to_region || from_stock <= to_stock.saturating_add(1) {
            return;
        }
        self.set_regional_market_state(
            from_region,
            item.id,
            from_stock - 1,
            from_demand.saturating_add(1),
        );
        self.active_regional_caravans.push(RegionalCaravanState {
            caravan_id: format!(
                "caravan-{}-{}-{}-{}",
                self.world_clock.day, self.world_clock.minute_of_day, from_region, to_region
            ),
            item_id: item.id.to_string(),
            from_region_id: from_region.to_string(),
            to_region_id: to_region.to_string(),
            quantity: 1,
            progress_legs: 0,
            risk: ((self.world_clock.day
                + u32::from(self.world_clock.minute_of_day / 60)
                + item.id.len() as u32)
                % 10) as u8,
            route_room_ids: Self::caravan_route(from_region, to_region),
            route_index: 0,
            integrity: default_caravan_integrity(),
            guarded_by_player: false,
            incident: None,
        });
        if self.regional_logistics.len() > 64 {
            self.regional_logistics.remove(0);
        }
    }

    pub fn buy_regional_item(&mut self, item_id: &str) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.buy_regional_item_atomic_inner(item_id)
        })
    }

    fn buy_regional_item_atomic_inner(&mut self, item_id: &str) -> Result<(), CampaignError> {
        let region_id = self.require_regional_market()?;
        let definition = ECONOMY_ITEM_CATALOG
            .iter()
            .find(|definition| definition.id == item_id)
            .ok_or_else(|| CampaignError::InvalidState(format!("unknown shop item: {item_id}")))?;
        let (stock, demand) = self.regional_market_state(region_id, item_id);
        if stock == 0 {
            return Err(CampaignError::InvalidState(format!(
                "{} is out of stock until local production recovers",
                definition.display_name
            )));
        }
        let price = market_price_with_state(item_id, self.world_clock.day, stock, demand, true)
            .expect("catalog item has a market price");
        if self.progression.credits < price {
            return Err(CampaignError::InvalidState(format!(
                "{} costs {} credits",
                definition.display_name, price
            )));
        }
        self.progression.credits -= price;
        self.set_regional_market_state(region_id, item_id, stock - 1, demand + 2);
        if definition.material {
            merge_loot(
                &mut self.progression.inventory,
                &[LootStack {
                    item_id: item_id.to_string(),
                    quantity: 1,
                }],
            );
        } else if let Some(item) = trillionnium_inventory_item_for(
            &self.character.matrix_user_id,
            item_id,
            &format!("{region_id}_shop"),
            None,
            (self.world_clock.day * 24 * 60 + u32::from(self.world_clock.minute_of_day)) as i64,
        ) {
            let instance_id = item.item_instance_id.clone();
            self.character.inventory_items.push(item);
            if let Some(condition) = ItemCondition::new(item_id) {
                self.item_conditions.insert(instance_id, condition);
            }
        } else {
            merge_loot(
                &mut self.progression.inventory,
                &[LootStack {
                    item_id: item_id.to_string(),
                    quantity: 1,
                }],
            );
        }
        self.revision += 1;
        Ok(())
    }

    pub fn selected_shop_item(&self) -> &'static trnm_rpg_core::EconomyItemDefinition {
        &ECONOMY_ITEM_CATALOG[self.selected_shop_item_index % ECONOMY_ITEM_CATALOG.len()]
    }

    pub fn shop_selection_label(&self) -> String {
        let item = self.selected_shop_item();
        let region_id = self.current_market_region_id().unwrap_or("mirror_city");
        let (stock, demand) = self.regional_market_state(region_id, item.id);
        format!(
            "{} @ {} | buy {} / sell {} credits | stock {} demand {:+} | durability {}{} | day {}",
            item.display_name,
            region_id,
            market_price_with_state(item.id, self.world_clock.day, stock, demand, true)
                .unwrap_or(item.buy_price),
            market_price_with_state(item.id, self.world_clock.day, stock, demand, false)
                .unwrap_or(item.buy_price / 2),
            stock,
            demand,
            item.max_durability,
            if item.material { " | material" } else { "" },
            self.world_clock.day,
        )
    }

    pub fn cycle_shop_item(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_shop_item_atomic_inner())
    }

    fn cycle_shop_item_atomic_inner(&mut self) -> Result<String, CampaignError> {
        self.require_regional_market()?;
        self.selected_shop_item_index =
            (self.selected_shop_item_index + 1) % ECONOMY_ITEM_CATALOG.len();
        self.revision += 1;
        Ok(self.selected_shop_item().id.to_string())
    }

    pub fn buy_selected_shop_item(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.buy_selected_shop_item_atomic_inner())
    }

    fn buy_selected_shop_item_atomic_inner(&mut self) -> Result<String, CampaignError> {
        let item_id = self.selected_shop_item().id.to_string();
        self.buy_regional_item(&item_id)?;
        Ok(item_id)
    }

    pub fn sell_selected_shop_item(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.sell_selected_shop_item_atomic_inner())
    }

    fn sell_selected_shop_item_atomic_inner(&mut self) -> Result<String, CampaignError> {
        let region_id = self.require_regional_market()?;
        let item = self.selected_shop_item();
        if item.material {
            consume_loot(&mut self.progression.inventory, item.id, 1)?;
        } else {
            let index = self
                .character
                .inventory_items
                .iter()
                .rposition(|owned| owned.item_id == item.id)
                .ok_or_else(|| {
                    CampaignError::InvalidState(format!("you do not own {}", item.display_name))
                })?;
            let removed = self.character.inventory_items.remove(index);
            self.item_conditions.remove(&removed.item_instance_id);
            self.character
                .equipment_slots
                .retain(|_, instance_id| instance_id != &removed.item_instance_id);
        }
        let (stock, demand) = self.regional_market_state(region_id, item.id);
        let price = market_price_with_state(item.id, self.world_clock.day, stock, demand, false)
            .expect("catalog item has a market price");
        self.progression.credits += price;
        self.set_regional_market_state(region_id, item.id, stock.saturating_add(1), demand - 2);
        self.progression.world_flags.insert(format!(
            "market_sale_{}_day_{}",
            item.id, self.world_clock.day
        ));
        self.revision += 1;
        Ok(item.id.to_string())
    }

    pub fn craft_regional_item(&mut self, recipe_id: &str) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.craft_regional_item_atomic_inner(recipe_id)
        })
    }

    fn craft_regional_item_atomic_inner(&mut self, recipe_id: &str) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::WorkshopGate)?;
        let recipe = CRAFTING_RECIPES
            .iter()
            .find(|recipe| recipe.id == recipe_id)
            .ok_or_else(|| CampaignError::InvalidState(format!("unknown recipe: {recipe_id}")))?;
        if !self
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == recipe.required_skill_id)
        {
            return Err(CampaignError::InvalidState(format!(
                "recipe requires skill {}",
                recipe.required_skill_id
            )));
        }
        for (item_id, quantity) in recipe.ingredients {
            if !self
                .progression
                .inventory
                .iter()
                .any(|stack| stack.item_id == *item_id && stack.quantity >= *quantity)
            {
                return Err(CampaignError::InvalidState(format!(
                    "recipe is missing {quantity}x {item_id}"
                )));
            }
        }
        for (item_id, quantity) in recipe.ingredients {
            consume_loot(&mut self.progression.inventory, item_id, *quantity)?;
            let demand = self
                .market_demand
                .entry((*item_id).to_string())
                .or_default();
            *demand = demand
                .saturating_add(i16::try_from(*quantity).unwrap_or(i16::MAX))
                .min(20);
        }
        let item = trillionnium_inventory_item_for(
            &self.character.matrix_user_id,
            recipe.output_item_id,
            "iron_workshop_crafting",
            None,
            (self.world_clock.day * 24 * 60 + u32::from(self.world_clock.minute_of_day)) as i64,
        )
        .ok_or_else(|| {
            CampaignError::InvalidState(
                "crafted item is missing from the typed item catalog".to_string(),
            )
        })?;
        let instance_id = item.item_instance_id.clone();
        self.character.inventory_items.push(item);
        if let Some(condition) = ItemCondition::new(recipe.output_item_id) {
            self.item_conditions.insert(instance_id, condition);
        }
        self.progression
            .world_flags
            .insert(format!("crafted_{}", recipe.output_item_id));
        self.revision += 1;
        Ok(())
    }

    pub fn selected_recipe(&self) -> &'static trnm_rpg_core::CraftingRecipe {
        &CRAFTING_RECIPES[self.selected_recipe_index % CRAFTING_RECIPES.len()]
    }

    pub fn recipe_selection_label(&self) -> String {
        let recipe = self.selected_recipe();
        let ingredients = recipe
            .ingredients
            .iter()
            .map(|(item, count)| format!("{count}x {item}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{} -> {} | {} | requires {}",
            recipe.id, recipe.output_item_id, ingredients, recipe.required_skill_id
        )
    }

    pub fn cycle_recipe(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_recipe_atomic_inner())
    }

    fn cycle_recipe_atomic_inner(&mut self) -> Result<String, CampaignError> {
        self.require_room(CampaignRoom::WorkshopGate)?;
        self.selected_recipe_index = (self.selected_recipe_index + 1) % CRAFTING_RECIPES.len();
        self.revision += 1;
        Ok(self.selected_recipe().id.to_string())
    }

    pub fn craft_selected_recipe(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.craft_selected_recipe_atomic_inner())
    }

    fn craft_selected_recipe_atomic_inner(&mut self) -> Result<String, CampaignError> {
        let recipe_id = self.selected_recipe().id.to_string();
        self.craft_regional_item(&recipe_id)?;
        Ok(recipe_id)
    }

    pub fn cycle_and_equip_owned_item(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.cycle_and_equip_owned_item_atomic_inner()
        })
    }

    fn cycle_and_equip_owned_item_atomic_inner(&mut self) -> Result<String, CampaignError> {
        self.require_town()?;
        if self.character.inventory_items.is_empty() {
            return Err(CampaignError::InvalidState(
                "no owned equipment is available".to_string(),
            ));
        }
        self.selected_inventory_index =
            (self.selected_inventory_index + 1) % self.character.inventory_items.len();
        let selected = self.character.inventory_items[self.selected_inventory_index].clone();
        let slot = selected.slot.clone();
        if slot.trim().is_empty() {
            return Err(CampaignError::InvalidState(format!(
                "{} is not equippable",
                selected.display_name
            )));
        }
        for item in &mut self.character.inventory_items {
            if item.equipped_slot.as_deref() == Some(slot.as_str()) {
                item.equipped_slot = None;
            }
            if item.item_instance_id == selected.item_instance_id {
                item.equipped_slot = Some(slot.clone());
            }
        }
        self.character
            .equipment_slots
            .insert(slot, selected.item_instance_id.clone());
        self.revision += 1;
        Ok(selected.display_name)
    }

    pub fn repair_all_equipment(&mut self) -> Result<i64, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.repair_all_equipment_atomic_inner())
    }

    fn repair_all_equipment_atomic_inner(&mut self) -> Result<i64, CampaignError> {
        self.require_room(CampaignRoom::WorkshopGate)?;
        let cost = self
            .item_conditions
            .values()
            .map(ItemCondition::repair_cost)
            .sum::<i64>();
        if cost == 0 {
            return Ok(0);
        }
        if self.progression.credits < cost {
            return Err(CampaignError::InvalidState(format!(
                "repairing equipment costs {cost} credits"
            )));
        }
        self.progression.credits -= cost;
        for condition in self.item_conditions.values_mut() {
            condition.repair();
        }
        self.revision += 1;
        Ok(cost)
    }

    pub fn equip_starter_weapon(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.equip_starter_weapon_atomic_inner())
    }

    fn equip_starter_weapon_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        self.selected_loadout = LoadoutPreset::Guard;
        self.apply_selected_loadout()?;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_loadout(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_loadout_atomic_inner())
    }

    fn cycle_loadout_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.character.equipment_slots.contains_key("weapon") {
            self.selected_loadout = self.selected_loadout.next();
        }
        self.apply_selected_loadout()?;
        self.revision += 1;
        Ok(())
    }

    fn apply_selected_loadout(&mut self) -> Result<(), CampaignError> {
        self.character.equipment_slots.clear();
        for item in &mut self.character.inventory_items {
            item.equipped_slot = None;
        }
        for item_id in self.selected_loadout.item_ids() {
            self.character
                .equip_item_by_id(item_id, self.revision as i64 + 1)
                .ok_or_else(|| {
                    CampaignError::InvalidState(format!(
                        "loadout item {item_id} is missing from inventory"
                    ))
                })?;
        }
        Ok(())
    }

    pub fn select_party(&mut self, party_ids: Vec<String>) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.select_party_atomic_inner(party_ids)
        })
    }

    fn select_party_atomic_inner(&mut self, party_ids: Vec<String>) -> Result<(), CampaignError> {
        self.require_town()?;
        if party_ids.len() != 4 || party_ids.first().map(String::as_str) != Some("hero") {
            return Err(CampaignError::InvalidState(
                "party must contain the hero plus exactly three companions".to_string(),
            ));
        }
        let unique = party_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let available = self
            .party
            .iter()
            .filter(|member| member.available)
            .map(|member| member.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        if unique.len() != 4 || !unique.is_subset(&available) {
            return Err(CampaignError::InvalidState(
                "selected party contains duplicates or unavailable members".to_string(),
            ));
        }
        self.active_party_ids = party_ids;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_party_preset(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_party_preset_atomic_inner())
    }

    fn cycle_party_preset_atomic_inner(&mut self) -> Result<(), CampaignError> {
        let presets = [
            ["hero", "aya", "mako", "tess"],
            ["hero", "aya", "nia", "sol"],
            ["hero", "mako", "brann", "tess"],
        ];
        let current = self
            .active_party_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let index = presets
            .iter()
            .position(|preset| preset.as_slice() == current.as_slice())
            .map(|index| (index + 1) % presets.len())
            .unwrap_or(0);
        for offset in 0..presets.len() {
            let candidate = presets[(index + offset) % presets.len()]
                .iter()
                .map(|id| (*id).to_string())
                .collect();
            if self.select_party(candidate).is_ok() {
                return Ok(());
            }
        }
        Err(CampaignError::InvalidState(
            "no complete party preset is currently available".to_string(),
        ))
    }

    pub fn cycle_party_member(&mut self, companion_slot: usize) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.cycle_party_member_atomic_inner(companion_slot)
        })
    }

    fn cycle_party_member_atomic_inner(
        &mut self,
        companion_slot: usize,
    ) -> Result<(), CampaignError> {
        self.require_town()?;
        if !(1..=3).contains(&companion_slot) {
            return Err(CampaignError::InvalidState(
                "companion slot must be 1, 2 or 3".to_string(),
            ));
        }
        let candidates = self
            .party
            .iter()
            .filter(|member| member.available && member.unit_id != "hero")
            .map(|member| member.unit_id.clone())
            .collect::<Vec<_>>();
        let current = &self.active_party_ids[companion_slot];
        let start = candidates
            .iter()
            .position(|candidate| candidate == current)
            .unwrap_or(0);
        for offset in 1..=candidates.len() {
            let candidate = &candidates[(start + offset) % candidates.len()];
            if !self
                .active_party_ids
                .iter()
                .any(|active| active == candidate)
            {
                self.active_party_ids[companion_slot] = candidate.clone();
                self.revision += 1;
                return Ok(());
            }
        }
        Err(CampaignError::InvalidState(
            "no unselected companion is available".to_string(),
        ))
    }

    pub fn spar_with_mentor(&mut self) -> Result<SparringReport, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.spar_with_mentor_atomic_inner())
    }

}

impl CampaignSaveV1 {
    fn spar_with_mentor_atomic_inner(&mut self) -> Result<SparringReport, CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        if !self.trained_with_mentor {
            return Err(CampaignError::InvalidState(
                "complete one training session before sparring".to_string(),
            ));
        }
        let report = resolve_mentor_sparring(
            &self.character.attributes,
            &[
                SparringAction::Guard,
                SparringAction::InnerPower,
                SparringAction::Strike,
                SparringAction::InnerPower,
            ],
        );
        if !self
            .progression
            .world_flags
            .contains("mentor_sparring_completed")
        {
            self.progression
                .world_flags
                .insert("mentor_sparring_completed".to_string());
            self.progression.experience += 20;
            self.npc_relationships
                .get_mut("street-compass-sifu")
                .expect("mentor relationship exists")
                .apply(RelationshipAction::Spar);
            if report.outcome == SparringOutcome::Victory {
                self.faction_rank = FactionRank::Disciple;
                self.character.sect_id = Some("signal-road-school".to_string());
                self.character.title = "Signal Road Disciple".to_string();
            }
        }
        self.last_sparring = Some(report.clone());
        self.revision += 1;
        Ok(report)
    }

    pub fn talk_to_relay_smith(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.talk_to_relay_smith_atomic_inner())
    }

    fn talk_to_relay_smith_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        let relation = self
            .npc_relationships
            .get_mut("relay-smith-brann")
            .expect("relay smith relationship exists");
        if relation.interactions == 0 {
            relation.apply(RelationshipAction::Talk);
            relation.apply(RelationshipAction::CompleteMission);
        } else {
            relation.apply(RelationshipAction::Talk);
        }
        if self.active_title == Some(BuildTitle::RelayRunner) {
            relation.apply(RelationshipAction::CompleteMission);
        }
        self.faction_rank = self.faction_rank.max(FactionRank::Envoy);
        self.revision += 1;
        Ok(())
    }

    pub fn recruit_relay_smith(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.recruit_relay_smith_atomic_inner())
    }

    fn recruit_relay_smith_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        let relation = self
            .npc_relationships
            .get_mut("relay-smith-brann")
            .expect("relay smith relationship exists");
        if !relation.can_recruit(8) {
            return Err(CampaignError::InvalidState(
                "Brann requires 8 trust before recruitment".to_string(),
            ));
        }
        relation.recruited = true;
        self.party
            .iter_mut()
            .find(|member| member.unit_id == "brann")
            .expect("Brann roster entry exists")
            .available = true;
        self.progression
            .world_flags
            .insert("brann_recruited".to_string());
        self.revision += 1;
        Ok(())
    }

    pub fn heal_party(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.heal_party_atomic_inner())
    }

    fn heal_party_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.party.iter().all(|member| member.injury_level == 0) {
            return Err(CampaignError::InvalidState(
                "the active roster has no injuries to treat".to_string(),
            ));
        }
        let used_tonic = if let Some(stack) = self
            .progression
            .inventory
            .iter_mut()
            .find(|stack| stack.item_id == "field-tonic-kit" && stack.quantity > 0)
        {
            stack.quantity -= 1;
            true
        } else {
            false
        };
        self.progression
            .inventory
            .retain(|stack| stack.quantity > 0);
        if !used_tonic {
            let clinic_cost = if self.active_title == Some(BuildTitle::ForgeMaster) {
                FIELD_CLINIC_CREDIT_COST - 15
            } else {
                FIELD_CLINIC_CREDIT_COST
            };
            if self.progression.credits < clinic_cost {
                return Err(CampaignError::InvalidState(format!(
                    "field clinic costs {clinic_cost} credits"
                )));
            }
            self.progression.credits -= clinic_cost;
        }
        for member in &mut self.party {
            member.injury_level = member.injury_level.saturating_sub(1);
            if member.injury_level < 4 {
                member.available = true;
            }
        }
        self.revision += 1;
        Ok(())
    }

    pub fn equip_relay_core(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.equip_relay_core_atomic_inner())
    }

    fn equip_relay_core_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        const ITEM_ID: &str = "relay-core-fragment";
        if !self
            .character
            .inventory_items
            .iter()
            .any(|item| item.item_id == ITEM_ID)
        {
            let stack = self
                .progression
                .inventory
                .iter_mut()
                .find(|stack| stack.item_id == ITEM_ID && stack.quantity > 0)
                .ok_or_else(|| {
                    CampaignError::InvalidState(
                        "secure the relay core before equipping its fragment".to_string(),
                    )
                })?;
            stack.quantity -= 1;
            let item = trillionnium_inventory_item_for(
                &self.character.matrix_user_id,
                ITEM_ID,
                "first_contact_victory_loot",
                None,
                self.revision as i64 + 1,
            )
            .ok_or_else(|| {
                CampaignError::InvalidState("relay core item catalog entry is missing".to_string())
            })?;
            self.character.inventory_items.push(item);
            self.progression
                .inventory
                .retain(|stack| stack.quantity > 0);
        }
        self.character
            .equip_item_by_id(ITEM_ID, self.revision as i64 + 1)
            .ok_or_else(|| {
                CampaignError::InvalidState("relay core could not be equipped".to_string())
            })?;
        self.revision += 1;
        Ok(())
    }

    pub fn accept_first_contact_quest(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.accept_first_contact_quest_atomic_inner()
        })
    }

    fn accept_first_contact_quest_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.mentor_met || !self.trained_with_mentor {
            return Err(CampaignError::InvalidState(
                "mentor dialogue and training are required before deployment".to_string(),
            ));
        }
        if !self.character.equipment_slots.contains_key("weapon") {
            return Err(CampaignError::InvalidState(
                "equip a weapon before deployment".to_string(),
            ));
        }
        let first_contact_secured = self
            .progression
            .world_flags
            .contains("first_contact_secured");
        let convoy_secured = self
            .progression
            .world_flags
            .contains("convoy_exodus_secured");
        let mirror_siege_secured = self
            .progression
            .world_flags
            .contains("mirror_siege_secured");
        if self.quest_state == QuestState::Completed && first_contact_secured {
            self.active_mission = if self.progression.aftershock_completions == 0 {
                CampaignMission::AftershockPatrol
            } else if !convoy_secured {
                CampaignMission::ConvoyExodus
            } else if !mirror_siege_secured {
                CampaignMission::MirrorSiege
            } else {
                match self.active_mission {
                    CampaignMission::IronDeltaSkirmish
                    | CampaignMission::NightWatchCrossingSkirmish
                    | CampaignMission::GlassBasinSkirmish
                    | CampaignMission::EmberOrchardSkirmish
                    | CampaignMission::SaltMarshSkirmish
                    | CampaignMission::CinderCrownSkirmish => self.active_mission,
                    _ => CampaignMission::AftershockPatrol,
                }
            };
        } else if self.quest_state == QuestState::Available {
            self.active_mission = CampaignMission::FirstContact;
        } else if !matches!(self.quest_state, QuestState::Failed | QuestState::Withdrawn) {
            return Err(CampaignError::InvalidState(
                "no campaign mission is currently available".to_string(),
            ));
        }
        self.quest_state = QuestState::Accepted;
        self.revision += 1;
        Ok(())
    }

    /// Opens a fully independent skirmish lane without granting campaign
    /// completion flags. The battle still uses the current character, normal
    /// BattleSeed hashing and the same one-time RPG settlement path.
    pub fn prepare_standalone_skirmish(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.prepare_standalone_skirmish_atomic_inner()
        })
    }

    fn prepare_standalone_skirmish_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.pending_battle.is_some() {
            return Err(CampaignError::InvalidState(
                "finish the pending battle before configuring a skirmish".to_string(),
            ));
        }
        self.room = CampaignRoom::ExpeditionGate;
        self.active_mission = CampaignMission::IronDeltaSkirmish;
        self.skirmish_setup.enabled = true;
        self.quest_state = QuestState::Accepted;
        self.progression
            .world_flags
            .insert("standalone_skirmish_accessed".to_string());
        self.progression
            .world_flags
            .insert("expedition_gate_open".to_string());
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_endgame_mission(&mut self) -> Result<CampaignMission, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_endgame_mission_atomic_inner())
    }

    fn cycle_endgame_mission_atomic_inner(&mut self) -> Result<CampaignMission, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            return Err(CampaignError::InvalidState(
                "secure Mirror Siege before opening skirmish operations".to_string(),
            ));
        }
        self.active_mission = match self.active_mission {
            CampaignMission::AftershockPatrol => CampaignMission::IronDeltaSkirmish,
            CampaignMission::IronDeltaSkirmish => CampaignMission::NightWatchCrossingSkirmish,
            CampaignMission::NightWatchCrossingSkirmish => CampaignMission::GlassBasinSkirmish,
            CampaignMission::GlassBasinSkirmish => CampaignMission::EmberOrchardSkirmish,
            CampaignMission::EmberOrchardSkirmish => CampaignMission::SaltMarshSkirmish,
            CampaignMission::SaltMarshSkirmish => CampaignMission::CinderCrownSkirmish,
            _ => CampaignMission::AftershockPatrol,
        };
        self.skirmish_setup.enabled = matches!(
            self.active_mission,
            CampaignMission::IronDeltaSkirmish
                | CampaignMission::NightWatchCrossingSkirmish
                | CampaignMission::GlassBasinSkirmish
                | CampaignMission::EmberOrchardSkirmish
                | CampaignMission::SaltMarshSkirmish
                | CampaignMission::CinderCrownSkirmish
        );
        self.revision += 1;
        Ok(self.active_mission)
    }

    pub fn cycle_standalone_skirmish_map(&mut self) -> Result<CampaignMission, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.cycle_standalone_skirmish_map_atomic_inner()
        })
    }

    fn cycle_standalone_skirmish_map_atomic_inner(
        &mut self,
    ) -> Result<CampaignMission, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled
            || !self
                .progression
                .world_flags
                .contains("standalone_skirmish_accessed")
        {
            return Err(CampaignError::InvalidState(
                "standalone skirmish setup is not active".to_string(),
            ));
        }
        self.active_mission = match self.active_mission {
            CampaignMission::IronDeltaSkirmish => CampaignMission::NightWatchCrossingSkirmish,
            CampaignMission::NightWatchCrossingSkirmish => CampaignMission::GlassBasinSkirmish,
            CampaignMission::GlassBasinSkirmish => CampaignMission::EmberOrchardSkirmish,
            CampaignMission::EmberOrchardSkirmish => CampaignMission::SaltMarshSkirmish,
            CampaignMission::SaltMarshSkirmish => CampaignMission::CinderCrownSkirmish,
            _ => CampaignMission::IronDeltaSkirmish,
        };
        self.revision += 1;
        Ok(self.active_mission)
    }

    pub fn cycle_skirmish_faction(&mut self) -> Result<CampaignFaction, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_skirmish_faction_atomic_inner())
    }

    fn cycle_skirmish_faction_atomic_inner(&mut self) -> Result<CampaignFaction, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring factions".to_string(),
            ));
        }
        self.skirmish_setup.player_faction = self.skirmish_setup.player_faction.opponent();
        self.skirmish_setup.enemy_faction = self.skirmish_setup.player_faction.opponent();
        self.revision += 1;
        Ok(self.skirmish_setup.player_faction)
    }

    pub fn cycle_skirmish_resources(&mut self) -> Result<u32, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_skirmish_resources_atomic_inner())
    }

    fn cycle_skirmish_resources_atomic_inner(&mut self) -> Result<u32, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring resources".to_string(),
            ));
        }
        self.skirmish_setup.starting_resources = match self.skirmish_setup.starting_resources {
            100..=299 => 300,
            300..=499 => 500,
            _ => 200,
        };
        self.revision += 1;
        Ok(self.skirmish_setup.starting_resources)
    }

    pub fn cycle_skirmish_victory_mode(&mut self) -> Result<SkirmishVictoryMode, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.cycle_skirmish_victory_mode_atomic_inner()
        })
    }

    fn cycle_skirmish_victory_mode_atomic_inner(
        &mut self,
    ) -> Result<SkirmishVictoryMode, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring victory".to_string(),
            ));
        }
        self.skirmish_setup.victory_mode = self.skirmish_setup.victory_mode.next();
        self.revision += 1;
        Ok(self.skirmish_setup.victory_mode)
    }

    pub fn cycle_skirmish_simulation_seed(&mut self) -> Result<u64, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.cycle_skirmish_simulation_seed_atomic_inner()
        })
    }

    fn cycle_skirmish_simulation_seed_atomic_inner(&mut self) -> Result<u64, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring the seed".to_string(),
            ));
        }
        self.skirmish_setup.simulation_seed = match self.skirmish_setup.simulation_seed {
            1 => 2,
            2 => 3,
            _ => 1,
        };
        self.revision += 1;
        Ok(self.skirmish_setup.simulation_seed)
    }

    pub fn start_first_contact_battle(
        &mut self,
        map: BattleMapSeedV1,
    ) -> Result<BattleSeedV1, CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.start_first_contact_battle_atomic_inner(map)
        })
    }

    fn start_first_contact_battle_atomic_inner(
        &mut self,
        map: BattleMapSeedV1,
    ) -> Result<BattleSeedV1, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if self.quest_state != QuestState::Accepted {
            return Err(CampaignError::InvalidState(
                "accept the First Contact quest before deployment".to_string(),
            ));
        }
        map.validate()?;
        let expedition_readiness = self.commit_expedition_preparation()?;
        let next_revision = self.revision + 1;
        let equipment_ids = equipped_item_ids(&self.character);
        let campaign_level = self.progression.level;
        let reputation = self.character.attributes.reputation;
        let party = self
            .active_party_ids
            .iter()
            .enumerate()
            .map(|(index, unit_id)| {
                let member = self
                    .party
                    .iter()
                    .find(|member| &member.unit_id == unit_id)
                    .expect("validated active party member exists");
                let skills = if member.unit_id == "hero" {
                    self.character.skill_ids.clone()
                } else {
                    member.skill_ids.clone()
                };
                let member_equipment = if member.unit_id == "hero" {
                    equipment_ids.clone()
                } else {
                    Vec::new()
                };
                let skill_rank = skills
                    .iter()
                    .filter_map(|skill| self.progression.skill_progress.get(skill))
                    .map(|progress| progress.rank)
                    .max()
                    .unwrap_or(1);
                let attributes = if member.unit_id == "hero" {
                    &self.character.attributes
                } else {
                    &member.attributes
                };
                let unit_level = if member.unit_id == "hero" {
                    campaign_level
                } else {
                    1 + (member.experience / 120) as u32
                };
                let mut stats = map_rpg_to_rts_stats(
                    attributes,
                    skill_rank,
                    &member_equipment,
                    member.injury_level,
                );
                apply_conditional_equipment_affixes(
                    &mut stats,
                    &member_equipment,
                    self.character_origin,
                    self.build_path,
                    self.active_title,
                );
                apply_campaign_growth(&mut stats, unit_level, reputation);
                apply_expedition_readiness(&mut stats, &expedition_readiness);
                if member.unit_id == "hero" {
                    apply_regional_skills_and_sect(
                        &mut stats,
                        &skills,
                        current_sect(&self.character),
                    );
                }
                BattleUnitSeedV1 {
                    unit_id: member.unit_id.clone(),
                    display_name: member.display_name.clone(),
                    role: member.role.clone(),
                    spawn_slot: format!("party_{index}"),
                    persistent: member.persistent,
                    injury_level: member.injury_level,
                    skill_ids: skills,
                    equipment_ids: member_equipment.clone(),
                    veteran_rank: member.veteran_rank,
                    stats,
                }
            })
            .collect();
        let map_id = self.active_mission.map_id();
        let mission = MissionDefinition::for_mission(self.active_mission, &map);
        let mut seed = BattleSeedV1 {
            contract_version: BATTLE_SEED_CONTRACT.to_string(),
            battle_id: format!("{map_id}-{next_revision:08}"),
            campaign_revision: next_revision,
            map_id: map_id.to_string(),
            rules_version: FIRST_CONTACT_RULES_VERSION.to_string(),
            map,
            party,
            mission,
            difficulty: self.difficulty,
            character_origin: self.character_origin,
            build_path: self.build_path,
            active_title: self.active_title,
            sect_id: self.character.sect_id.clone(),
            regional_skill_bonus_permille: self
                .character
                .skill_ids
                .iter()
                .filter_map(|skill_id| SKILL_CATALOG.iter().find(|skill| skill.id == skill_id))
                .map(|skill| skill.rts_modifier_permille)
                .sum::<u16>(),
            field_build_cost_permille: if self.active_title == Some(BuildTitle::ForgeMaster) {
                800
            } else {
                1000
            },
            expedition_readiness,
            skirmish: if matches!(
                self.active_mission,
                CampaignMission::IronDeltaSkirmish
                    | CampaignMission::NightWatchCrossingSkirmish
                    | CampaignMission::GlassBasinSkirmish
                    | CampaignMission::EmberOrchardSkirmish
                    | CampaignMission::SaltMarshSkirmish
                    | CampaignMission::CinderCrownSkirmish
            ) {
                let mut setup = self.skirmish_setup.clone();
                setup.enabled = true;
                setup
            } else {
                SkirmishSetup::default()
            },
            seed_hash: String::new(),
        };
        seed.seed_hash = seed.computed_hash()?;
        seed.validate()?;
        self.revision = next_revision;
        self.phase = CampaignPhase::BattlePending;
        self.pending_battle = Some(PendingBattleV1 {
            seed: seed.clone(),
            result: None,
        });
        Ok(seed)
    }

    pub fn stage_battle_result(&mut self, result: BattleResultV1) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.stage_battle_result_atomic_inner(result)
        })
    }

    fn stage_battle_result_atomic_inner(
        &mut self,
        result: BattleResultV1,
    ) -> Result<(), CampaignError> {
        if self.settled_battle_ids.contains(&result.battle_id) {
            return Ok(());
        }
        if self.phase != CampaignPhase::BattlePending {
            return Err(CampaignError::InvalidState(
                "no battle is awaiting a result".to_string(),
            ));
        }
        let pending = self.pending_battle.as_mut().ok_or_else(|| {
            CampaignError::InvalidState("pending battle payload is missing".to_string())
        })?;
        result.validate_against(&pending.seed)?;
        pending.result = Some(result);
        self.phase = CampaignPhase::PostBattlePending;
        self.revision += 1;
        Ok(())
    }

    pub fn submit_battle_result(
        &mut self,
        result: BattleResultV1,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.submit_battle_result_atomic_inner(result)
        })
    }

    fn submit_battle_result_atomic_inner(
        &mut self,
        result: BattleResultV1,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        if let Some(existing) = self.receipt_for(&result.battle_id) {
            if existing.seed_hash != result.seed_hash
                || existing.result_hash != result.computed_hash()?
            {
                return Err(CampaignError::Integrity(
                    "replayed battle id carries a different result payload".to_string(),
                ));
            }
            return Ok(SettlementReceiptV1::duplicate_from(existing, self.revision));
        }
        self.stage_battle_result(result)?;
        self.apply_pending_settlement()
    }

    pub fn apply_pending_settlement(&mut self) -> Result<SettlementReceiptV1, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.apply_pending_settlement_atomic_inner())
    }

}

