fn is_aftershock_map(map_id: &str) -> bool {
    matches!(map_id, "aftershock_patrol" | "first_contact_aftershock")
}

fn default_support_role() -> String {
    "support".to_string()
}

fn is_continuous_order(kind: RtsOrderKind) -> bool {
    matches!(
        kind,
        RtsOrderKind::Move
            | RtsOrderKind::AttackMove
            | RtsOrderKind::Patrol
            | RtsOrderKind::Harvest
            | RtsOrderKind::Capture
            | RtsOrderKind::Attack
            | RtsOrderKind::FocusFire
            | RtsOrderKind::Hold
    )
}

fn reveal_from(
    seed: &BattleSeedV1,
    origin: BattleGridPoint,
    radius: i16,
    visible: &mut BTreeSet<BattleGridPoint>,
) {
    let mut frontier = VecDeque::from([(origin, 0_i16)]);
    let mut visited = BTreeSet::from([origin]);
    while let Some((tile, steps)) = frontier.pop_front() {
        visible.insert(tile);
        if steps >= radius {
            continue;
        }
        for next in neighbors(tile) {
            if !seed.map.in_bounds(next) || !visited.insert(next) {
                continue;
            }
            if seed.map.passable(next) {
                frontier.push_back((next, steps + 1));
            } else {
                visible.insert(next);
            }
        }
    }
}

fn formation_target_for(
    center: BattleGridPoint,
    index: usize,
    formation_id: &str,
    seed: &BattleSeedV1,
) -> BattleGridPoint {
    let offsets: &[(i16, i16)] = match formation_id {
        "party_line" => &[(-1, 0), (0, 0), (1, 0), (2, 0)],
        "party_column" => &[(0, -1), (0, 0), (0, 1), (0, 2)],
        "party_wedge" => &[(0, 0), (-1, 1), (1, 1), (0, 2)],
        _ => &[(0, 0)],
    };
    let (x, y) = offsets[index % offsets.len()];
    let candidate = BattleGridPoint::new(center.x + x, center.y + y);
    if seed.map.passable(candidate) {
        candidate
    } else {
        center
    }
}

fn nearest_passable(seed: &BattleSeedV1, target: BattleGridPoint) -> Option<BattleGridPoint> {
    if seed.map.passable(target) {
        return Some(target);
    }
    neighbors(target)
        .into_iter()
        .find(|candidate| seed.map.passable(*candidate))
}

fn signature_skill(unit: &SimUnit) -> &'static str {
    for skill in [
        "field_mend",
        "relay_overcharge",
        "inner_flame",
        "wind_step",
        "iron_guard",
    ] {
        if unit.skill_ids.iter().any(|candidate| candidate == skill) {
            return skill;
        }
    }
    "iron_guard"
}

fn formation_positions(start: BattleGridPoint, seed: &BattleSeedV1) -> Vec<BattleGridPoint> {
    let candidates = [
        start,
        BattleGridPoint::new(start.x + 1, start.y),
        BattleGridPoint::new(start.x, start.y - 1),
        BattleGridPoint::new(start.x + 1, start.y - 1),
    ];
    candidates
        .into_iter()
        .map(|candidate| {
            if seed.map.passable(candidate) {
                candidate
            } else {
                start
            }
        })
        .collect()
}

