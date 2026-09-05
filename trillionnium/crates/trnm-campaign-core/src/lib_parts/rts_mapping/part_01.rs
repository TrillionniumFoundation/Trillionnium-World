pub fn typed_equipment_modifier(item_id: &str) -> TypedEquipmentModifier {
    let mut modifier = TypedEquipmentModifier {
        item_id: item_id.to_string(),
        max_hp: 0,
        damage: 0,
        armor: 0,
        move_speed_milli: 0,
        attack_interval_ticks: 0,
        evasion_permille: 0,
        energy: 0,
        ability_range: 0,
    };
    match item_id {
        "route-guard-staff" => {
            modifier.damage = 4;
            modifier.armor = 1;
            modifier.ability_range = 1;
        }
        "street-compass-bracer" => {
            modifier.move_speed_milli = 80;
            modifier.evasion_permille = 25;
        }
        "night-watch-cloak" => {
            modifier.move_speed_milli = 120;
            modifier.evasion_permille = 45;
        }
        "iron-workshop-blade" | "market-wind-sword" => modifier.damage = 7,
        "raid-signal-drum" => {
            modifier.energy = 20;
            modifier.ability_range = 2;
        }
        "field-tonic-kit" => modifier.max_hp = 20,
        "relay-core-fragment" => {
            modifier.armor = 2;
            modifier.energy = 25;
            modifier.ability_range = 2;
        }
        "evidence-wrap-case" => {
            modifier.energy = 10;
            modifier.ability_range = 1;
        }
        "reinforced-staff" => {
            modifier.damage = 10;
            modifier.armor = 2;
            modifier.ability_range = 2;
        }
        "signal-lamellar" => {
            modifier.max_hp = 30;
            modifier.armor = 5;
        }
        "watcher-boots" => {
            modifier.move_speed_milli = 180;
            modifier.evasion_permille = 60;
        }
        "field-medic-satchel" => {
            modifier.max_hp = 15;
            modifier.energy = 35;
            modifier.ability_range = 1;
        }
        "compass-thread-coat" => {
            modifier.armor = 2;
            modifier.move_speed_milli = 140;
            modifier.evasion_permille = 50;
        }
        "emberglass-lens" => {
            modifier.energy = 30;
            modifier.ability_range = 3;
        }
        "cistern-seal-kit" => {
            modifier.max_hp = 25;
            modifier.armor = 3;
        }
        "ashward-tonic" => {
            modifier.max_hp = 18;
            modifier.evasion_permille = 20;
        }
        _ => {}
    }
    modifier
}

fn apply_conditional_equipment_affixes(
    stats: &mut RtsUnitStats,
    equipment_ids: &[String],
    origin: CharacterOrigin,
    build_path: BuildPath,
    title: Option<BuildTitle>,
) {
    for item_id in equipment_ids {
        let (condition, hp, damage, armor, speed, evasion, energy, range) = match item_id.as_str() {
            "route-guard-staff" => (
                EquipmentAffixCondition::Origin(CharacterOrigin::Balanced),
                18,
                0,
                2,
                0,
                0,
                0,
                0,
            ),
            "night-watch-cloak" => (
                EquipmentAffixCondition::BuildPath(BuildPath::Windrunner),
                0,
                0,
                0,
                90,
                35,
                0,
                0,
            ),
            "raid-signal-drum" => (
                EquipmentAffixCondition::Origin(CharacterOrigin::Artisan),
                0,
                0,
                0,
                0,
                0,
                30,
                1,
            ),
            "relay-core-fragment" => (
                EquipmentAffixCondition::MasteryTitle(BuildTitle::ForgeMaster),
                0,
                3,
                2,
                0,
                0,
                0,
                0,
            ),
            _ => continue,
        };
        if condition.active(origin, build_path, title) {
            stats.max_hp = stats.max_hp.saturating_add(hp);
            stats.damage = stats.damage.saturating_add(damage);
            stats.armor = stats.armor.saturating_add(armor);
            stats.move_speed_milli = stats.move_speed_milli.saturating_add(speed);
            stats.evasion_permille = stats.evasion_permille.saturating_add(evasion).min(500);
            stats.energy = stats.energy.saturating_add(energy);
            stats.ability_range = stats.ability_range.saturating_add(range);
        }
    }
}

fn remove_origin_bonus(origin: CharacterOrigin, attributes: &mut TrillionniumAttributes) {
    match origin {
        CharacterOrigin::Balanced => {
            attributes.physique = attributes.physique.saturating_sub(2);
            attributes.resolve = attributes.resolve.saturating_sub(2);
        }
        CharacterOrigin::Artisan => {
            attributes.craft = attributes.craft.saturating_sub(4);
            attributes.insight = attributes.insight.saturating_sub(1);
        }
        CharacterOrigin::Scout => {
            attributes.agility = attributes.agility.saturating_sub(4);
            attributes.insight = attributes.insight.saturating_sub(1);
        }
    }
}

pub fn map_rpg_to_rts_stats(
    attributes: &TrillionniumAttributes,
    skill_rank: u16,
    equipment_ids: &[String],
    injury_level: u8,
) -> RtsUnitStats {
    let derived = attributes.derived_stats();
    let mut stats = RtsUnitStats {
        max_hp: derived.max_hp as u32,
        damage: 8 + attributes.force as u32 * 2,
        armor: 2 + attributes.physique as u32 / 4 + attributes.resolve as u32 / 5,
        move_speed_milli: 850 + attributes.agility as u32 * 22,
        attack_interval_ticks: (22_i32 - attributes.agility as i32 / 2).max(8) as u32,
        evasion_permille: (attributes.agility * 8).min(300),
        energy: derived.inner_energy as u32,
        ability_range: 2 + attributes.insight as u32 / 5,
        skill_power_permille: (1000 + skill_rank as u32 * 80).min(1800) as u16,
    };
    for item_id in equipment_ids {
        let modifier = typed_equipment_modifier(item_id);
        stats.max_hp = add_signed(stats.max_hp, modifier.max_hp, 1);
        stats.damage = add_signed(stats.damage, modifier.damage, 1);
        stats.armor = add_signed(stats.armor, modifier.armor, 0);
        stats.move_speed_milli = add_signed(stats.move_speed_milli, modifier.move_speed_milli, 100);
        stats.attack_interval_ticks = add_signed(
            stats.attack_interval_ticks,
            modifier.attack_interval_ticks,
            4,
        );
        stats.evasion_permille =
            add_signed(stats.evasion_permille as u32, modifier.evasion_permille, 0).min(500) as u16;
        stats.energy = add_signed(stats.energy, modifier.energy, 0);
        stats.ability_range = add_signed(stats.ability_range, modifier.ability_range, 1);
    }
    if injury_level > 0 {
        let penalty = 100_u32.saturating_sub(injury_level as u32 * 12).max(52);
        stats.max_hp = (stats.max_hp * penalty / 100).max(1);
        stats.move_speed_milli = (stats.move_speed_milli * penalty / 100).max(100);
    }
    stats
}

fn apply_campaign_growth(stats: &mut RtsUnitStats, level: u32, reputation: i32) {
    let growth = level.saturating_sub(1).min(12);
    let morale = reputation.clamp(0, 40) as u32;
    stats.max_hp = stats
        .max_hp
        .saturating_add(growth.saturating_mul(14))
        .saturating_add(morale / 2);
    stats.damage = stats.damage.saturating_add(growth.saturating_mul(2));
    stats.armor = stats.armor.saturating_add(growth / 2);
    stats.energy = stats
        .energy
        .saturating_add(growth.saturating_mul(4))
        .saturating_add(morale / 2);
}

fn apply_expedition_readiness(stats: &mut RtsUnitStats, readiness: &ExpeditionReadiness) {
    let stamina_permille = 700_u32.saturating_add(u32::from(readiness.stamina) * 3);
    stats.max_hp = (stats.max_hp.saturating_mul(stamina_permille) / 1000).max(1);
    stats.move_speed_milli =
        (stats.move_speed_milli.saturating_mul(stamina_permille) / 1000).max(100);
    match readiness.preparation {
        ExpeditionPreparation::Supplied => {
            stats.energy = stats.energy.saturating_add(30);
        }
        ExpeditionPreparation::Shortcut => {
            stats.move_speed_milli = stats.move_speed_milli.saturating_add(120);
            stats.evasion_permille = stats.evasion_permille.saturating_add(25).min(500);
        }
        ExpeditionPreparation::Immediate | ExpeditionPreparation::Rested => {}
    }
}

fn apply_regional_skills_and_sect(
    stats: &mut RtsUnitStats,
    skill_ids: &[String],
    sect: Option<SectId>,
) {
    for skill in skill_ids
        .iter()
        .filter_map(|skill_id| SKILL_CATALOG.iter().find(|skill| skill.id == skill_id))
    {
        match skill.effect {
            trnm_rpg_core::SkillEffect::Damage => {
                stats.damage = stats
                    .damage
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 25)
            }
            trnm_rpg_core::SkillEffect::Guard => {
                stats.armor = stats
                    .armor
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 35)
            }
            trnm_rpg_core::SkillEffect::Mobility => {
                stats.move_speed_milli = stats
                    .move_speed_milli
                    .saturating_add(u32::from(skill.rts_modifier_permille))
            }
            trnm_rpg_core::SkillEffect::Recon => {
                stats.ability_range = stats.ability_range.saturating_add(1)
            }
            trnm_rpg_core::SkillEffect::Healing => {
                stats.energy = stats
                    .energy
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 2)
            }
            trnm_rpg_core::SkillEffect::Construction => {
                stats.max_hp = stats
                    .max_hp
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 3)
            }
            trnm_rpg_core::SkillEffect::Economy => {
                stats.skill_power_permille = stats
                    .skill_power_permille
                    .saturating_add(skill.rts_modifier_permille / 2)
            }
            trnm_rpg_core::SkillEffect::Diplomacy => {
                stats.energy = stats
                    .energy
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 4)
            }
        }
    }
    match sect {
        Some(SectId::StreetCompass) => {
            stats.move_speed_milli = stats.move_speed_milli.saturating_add(100);
            stats.ability_range = stats.ability_range.saturating_add(1);
        }
        Some(SectId::IronWorkshop) => {
            stats.max_hp = stats.max_hp.saturating_add(24);
            stats.armor = stats.armor.saturating_add(3);
        }
        Some(SectId::NightWatch) => {
            stats.damage = stats.damage.saturating_add(3);
            stats.evasion_permille = stats.evasion_permille.saturating_add(50).min(500);
        }
        None => {}
    }
}

fn require_supplies(
    supplies: &ExpeditionSupplyState,
    rations: u8,
    water: u8,
) -> Result<(), CampaignError> {
    if supplies.rations < rations || supplies.water < water {
        Err(CampaignError::InvalidState(format!(
            "expedition requires {rations} ration(s) and {water} water"
        )))
    } else {
        Ok(())
    }
}

fn add_signed(value: u32, delta: i32, minimum: u32) -> u32 {
    (value as i64 + delta as i64).max(minimum as i64) as u32
}

fn equipped_item_ids(character: &WorldTrillionniumCharacter) -> Vec<String> {
    let equipped_instances = character
        .equipment_slots
        .values()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut ids = character
        .inventory_items
        .iter()
        .filter(|item| equipped_instances.contains(item.item_instance_id.as_str()))
        .map(|item| item.item_id.clone())
        .collect::<Vec<_>>();
    ids.sort();
    ids
}

fn character_item_conditions(
    character: &WorldTrillionniumCharacter,
) -> BTreeMap<String, ItemCondition> {
    character
        .inventory_items
        .iter()
        .filter_map(|item| {
            ItemCondition::new(item.item_id.clone())
                .filter(|condition| condition.max_durability > 0)
                .map(|condition| (item.item_instance_id.clone(), condition))
        })
        .collect()
}

fn current_sect(character: &WorldTrillionniumCharacter) -> Option<SectId> {
    match character.sect_id.as_deref()? {
        "signal-road-school" | "street_compass_society" => Some(SectId::StreetCompass),
        "iron_workshop_gate" => Some(SectId::IronWorkshop),
        "night_watch_alliance" => Some(SectId::NightWatch),
        _ => None,
    }
}

fn merge_loot(inventory: &mut Vec<LootStack>, loot: &[LootStack]) {
    for incoming in loot {
        if let Some(existing) = inventory
            .iter_mut()
            .find(|existing| existing.item_id == incoming.item_id)
        {
            existing.quantity = existing.quantity.saturating_add(incoming.quantity);
        } else {
            inventory.push(incoming.clone());
        }
    }
    inventory.sort_by(|left, right| left.item_id.cmp(&right.item_id));
}

fn consume_loot(
    inventory: &mut Vec<LootStack>,
    item_id: &str,
    quantity: u16,
) -> Result<(), CampaignError> {
    let stack = inventory
        .iter_mut()
        .find(|stack| stack.item_id == item_id && stack.quantity >= quantity)
        .ok_or_else(|| CampaignError::InvalidState(format!("missing loot item {item_id}")))?;
    stack.quantity -= quantity;
    inventory.retain(|stack| stack.quantity > 0);
    Ok(())
}

fn canonical_json_hash<T: Serialize>(value: &T) -> Result<String, CampaignError> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

