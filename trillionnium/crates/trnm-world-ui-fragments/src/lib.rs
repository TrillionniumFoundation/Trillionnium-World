//! Rust-owned HTML fragments for Trillionnium World.

use trnm_world_command::world_transition_decision;
use trnm_world_domain::{WorldState, WORLD_TRANSITION_SEMANTICS_CONTRACT};
use trnm_world_projection::{
    WorldHomeProjection, WorldRoutePreviewItem, WorldRouteTaskGraphView,
    WORLD_RUST_UI_FRAGMENT_CONTRACT,
};

pub const WORLD_UI_FRAGMENT_RENDERER: &str = "rust_world_ui_renderer";
pub const WORLD_RUST_OWNED_UI_SHELL_CONTRACT: &str = "trillionnium_world_rust_owned_ui_shell_v1";

pub fn render_home_fragment(projection: &WorldHomeProjection) -> String {
    format!(
        r#"<section id="trillionnium-world-standalone-shell" data-render-owner="{owner}" data-source-of-truth="{source}" data-fragment-contract="{contract}" data-node-count="{nodes}" data-route-count="{routes}" data-npc-count="{npcs}" data-task-count="{tasks}"><h1>Trillionnium World</h1><p>{prompt}</p></section>"#,
        owner = WORLD_UI_FRAGMENT_RENDERER,
        source = escape_attr(&projection.source_of_truth),
        contract = WORLD_RUST_UI_FRAGMENT_CONTRACT,
        nodes = projection.node_count,
        routes = projection.route_count,
        npcs = projection.npc_count,
        tasks = projection.task_count,
        prompt = escape_html(&projection.first_action_prompt),
    )
}

pub fn render_keypad_buttons_fragment(state: &WorldState, current_node_id: &str) -> String {
    let current_node = state.node(current_node_id);
    ["7", "8", "9", "4", "5", "6", "1", "2", "3"]
        .iter()
        .map(|key| {
            let (glyph, label_en, label_zh) = world_keypad_direction_label(key);
            let movement_transition = current_node.map(|node| world_transition_decision(state, node, key));
            let fallback_direction = world_keypad_direction_candidates(key)
                .first()
                .copied()
                .unwrap_or("blocked");
            let target_node_id = movement_transition
                .as_ref()
                .and_then(|transition| transition.to_node_id.as_deref())
                .unwrap_or_default();
            let direction = movement_transition
                .as_ref()
                .map(|transition| transition.direction.as_str())
                .unwrap_or(fallback_direction);
            let disabled = movement_transition
                .as_ref()
                .map(|transition| !transition.accepted)
                .unwrap_or(true);
            let transition_status = movement_transition
                .as_ref()
                .map(|transition| transition.transition_status.as_str())
                .unwrap_or("blocked");
            let transition_kind = movement_transition
                .as_ref()
                .map(|transition| transition.transition_kind.as_str())
                .unwrap_or("blocked_terrain");
            let transition_result = movement_transition
                .as_ref()
                .map(|transition| transition.result.as_str())
                .unwrap_or("blocked_terrain");
            let blocked_reason = movement_transition
                .as_ref()
                .and_then(|transition| transition.blocked_reason.as_deref())
                .unwrap_or("");
            format!(
                "<button id=\"world-keypad-{key}\" type=\"button\" class=\"world-keypad-button{blocked_class}\" data-keypad-key=\"{key}\" data-rust-owned-ui-contract=\"{ui_contract}\" data-render-owner=\"{owner}\" data-transition-contract-version=\"{transition_contract}\" data-transition-status=\"{transition_status}\" data-transition-kind=\"{transition_kind}\" data-transition-result=\"{transition_result}\" data-blocked-reason=\"{blocked_reason}\" data-move-direction=\"{direction}\" data-target-node-id=\"{target_node_id}\" data-rust-endpoint=\"/world/web/map-move\" data-source-of-truth=\"rust_world_map_move\" data-transition-source-of-truth=\"rust_world_map_transition_rules\" data-web-role=\"input_only\" aria-disabled=\"{disabled}\" data-i18n-aria-label-en=\"{label_en}\" data-i18n-aria-label-zh=\"{label_zh}\"><span>{glyph}</span><small data-i18n-en=\"{label_en}\" data-i18n-zh=\"{label_zh}\">{label_en}</small></button>",
                key = escape_attr(key),
                blocked_class = if disabled { " is-blocked" } else { "" },
                ui_contract = WORLD_RUST_OWNED_UI_SHELL_CONTRACT,
                owner = WORLD_UI_FRAGMENT_RENDERER,
                transition_contract = WORLD_TRANSITION_SEMANTICS_CONTRACT,
                transition_status = escape_attr(transition_status),
                transition_kind = escape_attr(transition_kind),
                transition_result = escape_attr(transition_result),
                blocked_reason = escape_attr(blocked_reason),
                direction = escape_attr(direction),
                target_node_id = escape_attr(target_node_id),
                disabled = disabled,
                label_en = escape_attr(label_en),
                label_zh = escape_attr(label_zh),
                glyph = escape_html(glyph),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn world_keypad_direction_candidates(key: &str) -> &'static [&'static str] {
    match key {
        "8" => &["north", "n"],
        "2" => &["south", "s"],
        "4" => &["west", "w"],
        "6" => &["east", "e"],
        "7" => &["north-west", "northwest", "nw"],
        "9" => &["north-east", "northeast", "ne"],
        "1" => &["south-west", "southwest", "sw"],
        "3" => &["south-east", "southeast", "se"],
        "5" => &["wait", "stay"],
        _ => &[],
    }
}

pub fn world_keypad_direction_label(key: &str) -> (&'static str, &'static str, &'static str) {
    match key {
        "8" => ("8↑", "Move up", "上"),
        "2" => ("2↓", "Move down", "下"),
        "4" => ("4←", "Move left", "左"),
        "6" => ("6→", "Move right", "右"),
        "7" => ("7↖", "Move up-left", "左上"),
        "9" => ("9↗", "Move up-right", "右上"),
        "1" => ("1↙", "Move down-left", "左下"),
        "3" => ("3↘", "Move down-right", "右下"),
        "5" => ("5·", "Wait", "停留"),
        _ => ("?", "Move", "移动"),
    }
}

pub fn escape_world_route_visible_text(value: &str) -> String {
    if let Some(copy) = i18n_span_from_bilingual_slash_copy(value) {
        return copy;
    }
    if contains_cjk_text(value) {
        let english = world_route_english_visible_text(value);
        return i18n_span_from_bilingual_slash_copy(&format!("{} / {}", english, value))
            .unwrap_or_else(|| escape_html(&english));
    }
    escape_html(value)
}

pub fn world_route_english_visible_text(value: &str) -> String {
    if !contains_cjk_text(value) {
        return value.to_string();
    }

    let replacements = [
        ("回访/升级悬赏", "follow-up or upgrade bounty"),
        ("升级悬赏", "upgrade bounty"),
        ("回访", "follow-up"),
        ("评级后升级悬赏", "post-rating bounty upgrade"),
        ("评级通过", "rating passed"),
        ("拒收清算", "rejection settlement"),
        ("取消清算", "cancellation settlement"),
        ("卖家扣回资金", "seller chargeback funds"),
        ("卖家扣回", "seller chargeback"),
        ("买家不二次退款", "buyer avoids double refund"),
        ("买家退款", "buyer refund"),
        ("账本错误", "ledger error"),
        ("账本", "ledger"),
        ("清账", "settle ledger"),
        ("清算", "settlement"),
        ("扣回", "chargeback"),
        ("退款", "refund"),
        ("重试", "retry"),
        ("恢复", "recover"),
        ("重开修订委托", "reopen revision commission"),
        ("修订委托", "revision commission"),
        ("修订成果", "revised result"),
        ("重新校准需求", "recalibrate requirements"),
        ("缺失证据", "missing evidence"),
        ("委托目标", "commission goal"),
        ("里程碑", "milestone"),
        ("第一轮成果", "first result"),
        ("结果摘要整理中", "Outcome summary pending"),
        ("路线摘要整理中", "Route summary pending"),
        ("证据和下一步整理中", "Evidence and next step pending"),
        ("继续推进下一步机会", "continue the next opportunity"),
        ("打开任务牌路线", "Open bounty route"),
        ("打开契约路线", "Open contract route"),
        ("起草任务后续", "Draft task follow-up"),
        ("推进下一条支线", "Advance next branch"),
        ("下一条支线", "next branch"),
        ("下一次协作", "next collaboration"),
        ("下一步", "next step"),
        ("支线", "branch"),
        ("事件", "events"),
        ("委托", "commissions"),
        ("契约", "contracts"),
        ("战报", "battle reports"),
        ("证据", "evidence"),
        ("风险", "risks"),
        ("目标", "goals"),
        ("质量", "quality"),
        ("成果", "result"),
        ("记录", "record"),
        ("确认", "confirm"),
        ("输出", "produce"),
        ("围绕", "around"),
        ("补齐", "fill"),
        ("完成", "complete"),
        ("锁定", "lock"),
        ("复盘", "review"),
        ("和", "and"),
    ];

    let mut translated = value.trim().to_string();
    for (from, to) in replacements {
        translated = translated.replace(from, to);
    }
    let mut normalized_punctuation = String::new();
    for ch in translated.chars() {
        match ch {
            '：' => normalized_punctuation.push_str(": "),
            '，' | '、' => normalized_punctuation.push_str(", "),
            '；' => normalized_punctuation.push_str("; "),
            '。' => normalized_punctuation.push('.'),
            '！' => normalized_punctuation.push('!'),
            '？' => normalized_punctuation.push('?'),
            _ => normalized_punctuation.push(ch),
        }
    }
    translated = normalized_punctuation;

    if contains_cjk_text(&translated) {
        translated = translated
            .chars()
            .filter(|ch| {
                !(('\u{3400}'..='\u{9fff}').contains(ch)
                    || ('\u{f900}'..='\u{faff}').contains(ch)
                    || matches!(ch, '、' | '，' | '。' | '：' | '；' | '！' | '？'))
            })
            .collect::<String>();
    }

    let normalized = translated.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.trim().is_empty() {
        "Route detail pending.".to_string()
    } else {
        normalized
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WorldRouteActionButtonView<'a> {
    pub label: &'a str,
    pub panel_id: &'a str,
    pub input_id: &'a str,
    pub input_value: &'a str,
    pub textarea_id: &'a str,
    pub location_id: &'a str,
    pub target_node_id: &'a str,
    pub task_id: &'a str,
    pub contract_id: &'a str,
    pub listing_id: &'a str,
    pub work_order_id: &'a str,
    pub event_id: &'a str,
    pub event_kind: &'a str,
    pub event_body: &'a str,
    pub event_result: &'a str,
    pub event_task_id: &'a str,
    pub body: &'a str,
}

impl WorldRouteActionButtonView<'_> {
    pub fn target_attrs_html(&self) -> String {
        format!(
            " data-target-panel=\"{}\" data-target-input-id=\"{}\" data-target-value=\"{}\" data-target-textarea-id=\"{}\" data-target-location-id=\"{}\" data-target-node-id=\"{}\" data-target-task-id=\"{}\" data-target-contract-id=\"{}\" data-target-listing-id=\"{}\" data-target-work-order-id=\"{}\" data-target-event-id=\"{}\" data-target-event-kind=\"{}\" data-target-event-body=\"{}\" data-target-event-result=\"{}\" data-target-event-task-id=\"{}\" data-target-body=\"{}\"",
            escape_attr(self.panel_id),
            escape_attr(self.input_id),
            escape_attr(self.input_value),
            escape_attr(self.textarea_id),
            escape_attr(self.location_id),
            escape_attr(self.target_node_id),
            escape_attr(self.task_id),
            escape_attr(self.contract_id),
            escape_attr(self.listing_id),
            escape_attr(self.work_order_id),
            escape_attr(self.event_id),
            escape_attr(self.event_kind),
            escape_attr(self.event_body),
            escape_attr(self.event_result),
            escape_attr(self.event_task_id),
            escape_attr(self.body),
        )
    }

    pub fn render(&self, class_name: &str) -> String {
        format!(
            "<button type=\"button\" class=\"focus-chip {}\"{}>{}</button>",
            escape_attr(class_name),
            self.target_attrs_html(),
            escape_world_route_visible_text(self.label),
        )
    }
}

pub fn render_world_route_preview_item_card(item: &WorldRoutePreviewItem) -> String {
    let detail = if item.detail.is_empty() {
        item.route_bucket.as_str()
    } else {
        item.detail.as_str()
    };
    let summary = if item.summary.is_empty() {
        "waiting"
    } else {
        item.summary.as_str()
    };
    let focus_code = if !item.task_id.is_empty() {
        item.task_id.as_str()
    } else if !item.location_id.is_empty() {
        item.location_id.as_str()
    } else {
        item.route_bucket.as_str()
    };
    format!(
        "<article class=\"module\"><strong>{}</strong><span>{}</span><p>{}</p><div class=\"focus-stack\"><code>{}</code></div></article>",
        escape_world_route_visible_text(&item.title),
        escape_world_route_visible_text(detail),
        escape_world_route_visible_text(summary),
        escape_world_route_visible_text(focus_code),
    )
}

fn suggested_action_button(task: &WorldRouteTaskGraphView, class_name: &str) -> String {
    WorldRouteActionButtonView {
        label: &task.suggested_action_label,
        panel_id: &task.suggested_panel_id,
        input_id: task.resolved_suggested_input_id(),
        input_value: task.resolved_suggested_input_value(),
        textarea_id: task.resolved_suggested_textarea_id(),
        location_id: &task.latest_location_id,
        target_node_id: &task.suggested_node_id,
        task_id: &task.task_id,
        contract_id: &task.latest_contract_id,
        listing_id: "",
        work_order_id: "",
        event_id: "",
        event_kind: "",
        event_body: "",
        event_result: "",
        event_task_id: "",
        body: &task.suggested_body,
    }
    .render(class_name)
}

fn opportunity_action_button(task: &WorldRouteTaskGraphView, class_name: &str) -> String {
    WorldRouteActionButtonView {
        label: &task.next_opportunity_action_label,
        panel_id: &task.next_opportunity_panel_id,
        input_id: &task.next_opportunity_input_id,
        input_value: &task.next_opportunity_input_value,
        textarea_id: &task.next_opportunity_textarea_id,
        location_id: &task.latest_location_id,
        target_node_id: &task.next_opportunity_node_id,
        task_id: &task.task_id,
        contract_id: "",
        listing_id: "",
        work_order_id: "",
        event_id: "",
        event_kind: "",
        event_body: "",
        event_result: "",
        event_task_id: "",
        body: task.opportunity_body(),
    }
    .render(class_name)
}

pub fn render_world_route_task_graph_app_card(task: &WorldRouteTaskGraphView) -> String {
    let suggested_action = suggested_action_button(task, "trillionnium-app-route-flow-action");
    let opportunity_action = opportunity_action_button(task, "trillionnium-app-route-flow-action");
    format!(
        "<article class=\"module app-route-task-graph-item\" data-task-id=\"{}\" data-location-id=\"{}\"><strong>{}</strong><span>{} · {} · branch {}</span><p>{} events · {} commissions · {} battle reports</p><p>{}</p><p><strong>Next branch</strong> · {}</p><p>{}</p><div class=\"focus-stack\"><code>{}</code></div><div class=\"focus-stack\">{}{}</div></article>",
        escape_attr(&task.task_id),
        escape_attr(&task.latest_location_id),
        escape_attr(&task.task_id),
        escape_world_route_visible_text(&task.latest_bucket),
        escape_world_route_visible_text(&task.latest_status),
        escape_world_route_visible_text(&task.next_opportunity_kind),
        task.event_count,
        task.contract_count,
        task.completion_count,
        escape_world_route_visible_text(&task.outcome_summary),
        escape_world_route_visible_text(&task.next_opportunity_hint),
        escape_world_route_visible_text(&task.next_opportunity_playbook),
        escape_world_route_visible_text(&task.next_opportunity_command),
        suggested_action,
        opportunity_action,
    )
}

pub fn render_world_route_task_graph_world_flow_card(task: &WorldRouteTaskGraphView) -> String {
    let suggested_action = suggested_action_button(task, "trillionnium-route-flow-action");
    let opportunity_action = opportunity_action_button(task, "trillionnium-route-flow-action");
    format!(
        "<article class=\"mini task-graph\" data-task-id=\"{}\" data-location-id=\"{}\"><strong>{}</strong><span>{} · {} · branch {}</span><code>{}</code><small>{} events · {} commissions · {} battle reports</small><small>{}</small><small><strong>Next branch</strong> · {}</small><div class=\"focus-stack\"><code>{}</code></div><div class=\"focus-stack\">{}{}</div></article>",
        escape_attr(&task.task_id),
        escape_attr(&task.latest_location_id),
        escape_attr(&task.task_id),
        escape_world_route_visible_text(&task.latest_bucket),
        escape_world_route_visible_text(&task.latest_status),
        escape_world_route_visible_text(&task.next_opportunity_kind),
        escape_attr(&task.task_id),
        task.event_count,
        task.contract_count,
        task.completion_count,
        escape_world_route_visible_text(&task.outcome_summary),
        escape_world_route_visible_text(&task.next_opportunity_hint),
        escape_world_route_visible_text(&task.next_opportunity_command),
        suggested_action,
        opportunity_action,
    )
}

pub fn render_world_route_task_graph_cards(
    tasks: &[WorldRouteTaskGraphView],
    app_surface: bool,
) -> String {
    tasks
        .iter()
        .map(|task| {
            if app_surface {
                render_world_route_task_graph_app_card(task)
            } else {
                render_world_route_task_graph_world_flow_card(task)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn contains_cjk_text(value: &str) -> bool {
    value.chars().any(|ch| {
        ('\u{3400}'..='\u{9fff}').contains(&ch) || ('\u{f900}'..='\u{faff}').contains(&ch)
    })
}

fn i18n_span_from_bilingual_slash_copy(value: &str) -> Option<String> {
    let (english, chinese) = value.split_once(" / ")?;
    let english = english.trim();
    let chinese = chinese.trim();
    if english.is_empty() || chinese.is_empty() {
        return None;
    }
    Some(format!(
        "<span data-i18n-en=\"{}\" data-i18n-zh=\"{}\">{}</span>",
        escape_attr(english),
        escape_attr(chinese),
        escape_html(english),
    ))
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_attr(value: &str) -> String {
    escape_html(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_world_domain::WorldState;
    use trnm_world_projection::project_home;

    #[test]
    fn fragment_is_marked_rust_owned() {
        let html = render_home_fragment(&project_home(&WorldState::fixture(), "local-player"));
        assert!(html.contains("data-render-owner=\"rust_world_ui_renderer\""));
        assert!(html.contains(WORLD_RUST_UI_FRAGMENT_CONTRACT));
    }

    #[test]
    fn keypad_fragment_preserves_cex_transition_contract() {
        let html = render_keypad_buttons_fragment(&WorldState::fixture(), "mirror-city-square");
        assert!(html.contains("id=\"world-keypad-6\""));
        assert!(html.contains(
            "data-transition-contract-version=\"trillionnium_world_transition_semantics_v1\""
        ));
        assert!(
            html.contains("data-transition-source-of-truth=\"rust_world_map_transition_rules\"")
        );
        assert!(html.contains("data-target-node-id=\"league-coliseum\""));
        assert!(html.contains("data-transition-result=\"open_exit\""));
        assert!(html.contains("id=\"world-keypad-8\""));
        assert!(html.contains("data-blocked-reason=\"no_exit_for_direction\""));
    }

    #[test]
    fn route_fragments_preserve_cex_handoff_attrs() {
        let artifacts = trnm_world_projection::world_route_artifacts_from_raw_preview_items(
            vec![serde_json::json!({
                "route_bucket": "rejection",
                "work_order_id": "work-b",
                "route_status": "rejected_chargeback_failed",
                "location_id": "delivery-dock",
                "created_at_epoch": 30,
                "title": "拒收清算",
                "summary": "seller chargeback failed",
                "detail": "settlement retry required"
            })],
            4,
        );
        let task = artifacts.task_views.first().unwrap();
        let app_card = render_world_route_task_graph_app_card(task);
        assert!(app_card.contains("app-route-task-graph-item"));
        assert!(app_card.contains("data-target-panel=\"world-commerce-panel\""));
        assert!(app_card.contains("data-target-input-id=\"world-work-reject-id\""));
        assert!(app_card.contains("data-target-node-id=\"delivery-dock\""));
        assert!(app_card.contains("data-i18n-en="));

        let preview_card = render_world_route_preview_item_card(
            &trnm_world_projection::WorldRoutePreviewItem::from_value(&serde_json::json!({
                "route_bucket": "event",
                "task_id": "task-a",
                "title": "路线摘要整理中",
                "summary": "证据和下一步整理中"
            })),
        );
        assert!(preview_card.contains("<article class=\"module\""));
        assert!(preview_card.contains("data-i18n-zh="));
    }
}
