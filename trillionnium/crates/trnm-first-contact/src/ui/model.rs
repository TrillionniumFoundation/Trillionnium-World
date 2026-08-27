use super::super::{
    campaign_flow::{CampaignFlow, CampaignMode, ShellMode},
    campaign_ui::campaign_action_specs,
};
use super::theme::UiTone;
use trnm_campaign_core::{CampaignRoom, EconomyMode, InputMode};

pub(super) const OFFLINE_AUTHORITY_PROFILE: &str = "offline_world_v1";
pub(super) const COMPATIBILITY_AUTHORITY_PROFILE: &str =
    "world_legacy_local_alpha_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WorldUiPage {
    Now,
    System,
    Help,
}

impl WorldUiPage {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Now => "NOW",
            Self::System => "SYSTEM",
            Self::Help => "HELP",
        }
    }

    pub(super) fn next(self) -> Self {
        match self {
            Self::Now => Self::System,
            Self::System => Self::Help,
            Self::Help => Self::Now,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct UiSnapshotInput {
    pub phase_label: String,
    pub location_label: String,
    pub objective: String,
    pub next_action: String,
    pub online_attached: bool,
    pub economy_mode: EconomyMode,
    pub ordinary_outbox: usize,
    pub compensation_outbox: usize,
    pub dead_letters: usize,
    pub save_label: String,
    pub progress_label: String,
    pub input_label: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct WorldUiSnapshot {
    pub phase_label: String,
    pub location_label: String,
    pub objective: String,
    pub next_action: String,
    pub authority_label: String,
    pub authority_detail: String,
    pub authority_tone: UiTone,
    pub economy_label: String,
    pub economy_detail: String,
    pub economy_tone: UiTone,
    pub save_label: String,
    pub progress_label: String,
    pub input_label: String,
    pub status: String,
    pub status_tone: UiTone,
    pub now_body: String,
    pub system_body: String,
    pub help_body: String,
}

impl WorldUiSnapshot {
    pub(super) fn from_input(input: UiSnapshotInput) -> Self {
        let (authority_label, authority_detail, authority_tone) = if input.online_attached {
            (
                "COMPATIBILITY LAB".to_string(),
                format!(
                    "{COMPATIBILITY_AUTHORITY_PROFILE} · migration evidence only · not Nakama canonical"
                ),
                UiTone::Warning,
            )
        } else {
            (
                "OFFLINE WORLD".to_string(),
                format!("{OFFLINE_AUTHORITY_PROFILE} · deterministic local campaign"),
                UiTone::Neutral,
            )
        };

        let pending = input
            .ordinary_outbox
            .saturating_add(input.compensation_outbox);
        let (economy_label, economy_detail, economy_tone) = match input.economy_mode {
            EconomyMode::OfflineLocal => (
                "LOCAL ECONOMY".to_string(),
                "bound local credits · public player market disabled".to_string(),
                UiTone::Neutral,
            ),
            EconomyMode::CexConnected if input.dead_letters > 0 => (
                "CEX ATTENTION".to_string(),
                format!(
                    "pending {pending} · dead letters {} · operator review required",
                    input.dead_letters
                ),
                UiTone::Critical,
            ),
            EconomyMode::CexConnected if pending > 0 => (
                "CEX SYNCING".to_string(),
                format!(
                    "ordinary {} · compensation {} · public player market disabled",
                    input.ordinary_outbox, input.compensation_outbox
                ),
                UiTone::Warning,
            ),
            EconomyMode::CexConnected => (
                "CEX CONNECTED".to_string(),
                "wallet read model current · public player market disabled".to_string(),
                UiTone::Positive,
            ),
        };

        let status_tone = status_tone(&input.status);
        let now_body = format!(
            "NEXT ACTION\n{}\n\nCURRENT OBJECTIVE\n{}\n\nPROGRESS\n{}",
            input.next_action, input.objective, input.progress_label
        );
        let system_body = format!(
            "AUTHORITY\n{}\n{}\n\nSAVE\n{}\n\nECONOMY\n{}\n{}\n\nSTATUS\n{}",
            authority_label,
            authority_detail,
            input.save_label,
            economy_label,
            economy_detail,
            input.status,
        );
        let help_body = format!(
            "F6  SHOW / HIDE GUIDE\nF7  CYCLE GUIDE PAGE\n\nINPUT PROFILE\n{}\n\nREADING ORDER\n1. Header: where and mode\n2. NOW: next action and objective\n3. Primary campaign actions\n4. Status line\n\nACCESSIBILITY\nHigh contrast, subtitles, keyboard-only and mouse-only modes remain profile-scoped and never change campaign authority.",
            input.input_label,
        );

        Self {
            phase_label: input.phase_label,
            location_label: input.location_label,
            objective: input.objective,
            next_action: input.next_action,
            authority_label,
            authority_detail,
            authority_tone,
            economy_label,
            economy_detail,
            economy_tone,
            save_label: input.save_label,
            progress_label: input.progress_label,
            input_label: input.input_label,
            status: input.status,
            status_tone,
            now_body,
            system_body,
            help_body,
        }
    }

    pub(super) fn from_flow(
        flow: &CampaignFlow,
        online_attached: bool,
        battle_objective: &str,
    ) -> Self {
        let phase_label = phase_label(flow).to_string();
        let location_label = room_label(flow.save.room).to_string();
        let objective = objective(flow, battle_objective);
        let next_action = next_action(flow);
        let input_label = format!(
            "{:?} · {:?} · subtitles {} · contrast {} · audio {}%",
            flow.settings.input_mode,
            flow.settings.control_scheme,
            on_off(flow.settings.subtitles),
            on_off(flow.settings.high_contrast),
            flow.settings.master_volume_percent,
        );
        Self::from_input(UiSnapshotInput {
            phase_label,
            location_label,
            objective,
            next_action,
            online_attached,
            economy_mode: flow.save.economy_mode,
            ordinary_outbox: flow.save.pending_economic_intents.len(),
            compensation_outbox: flow.save.pending_economic_compensations.len(),
            dead_letters: flow.save.economic_dead_letters.len(),
            save_label: format!(
                "SLOT {} · REV {} · {:?}",
                flow.active_slot.label(),
                flow.save.revision,
                flow.save.phase,
            ),
            progress_label: format!(
                "LEVEL {} · {:?} · {}",
                flow.save.progression.level,
                flow.save.story.current_step,
                flow.save.active_mission.display_name(),
            ),
            input_label,
            status: flow.status.clone(),
        })
    }

    pub(super) fn body_for(&self, page: WorldUiPage) -> &str {
        match page {
            WorldUiPage::Now => &self.now_body,
            WorldUiPage::System => &self.system_body,
            WorldUiPage::Help => &self.help_body,
        }
    }

    pub(super) fn context_line(&self) -> String {
        format!(
            "{}  ·  {}  ·  {}",
            self.location_label, self.phase_label, self.save_label
        )
    }

    pub(super) fn battle_badge(&self) -> String {
        format!("{} · {}", self.authority_label, self.phase_label)
    }
}

fn phase_label(flow: &CampaignFlow) -> &'static str {
    match flow.shell_mode {
        ShellMode::Title => "TITLE",
        ShellMode::CharacterCreate => "CHARACTER CREATE",
        ShellMode::SkirmishSetup => "SKIRMISH SETUP",
        ShellMode::Journal => "JOURNAL",
        ShellMode::ResumeGuard => "RESUME CHECK",
        ShellMode::Paused => "PAUSED",
        ShellMode::ReplayBrowser => "REPLAY BROWSER",
        ShellMode::Playing => match flow.mode {
            CampaignMode::Town => "OPEN WORLD",
            CampaignMode::Battle => "RTS BATTLE",
            CampaignMode::Debrief => "DEBRIEF",
        },
    }
}

fn objective(flow: &CampaignFlow, battle_objective: &str) -> String {
    match flow.shell_mode {
        ShellMode::Title => "Choose a valid save slot or create a new campaign.".to_string(),
        ShellMode::CharacterCreate => "Confirm one persistent character identity.".to_string(),
        ShellMode::SkirmishSetup => {
            "Choose deterministic skirmish rules, then deploy.".to_string()
        }
        ShellMode::Journal => "Review active, complete and locked goals.".to_string(),
        ShellMode::ResumeGuard => "Verify the recovered save state, then resume.".to_string(),
        ShellMode::Paused => "Resume play or save and return to title.".to_string(),
        ShellMode::ReplayBrowser => {
            "Verify replay hashes without changing campaign state.".to_string()
        }
        ShellMode::Playing if flow.mode == CampaignMode::Battle => battle_objective.to_string(),
        ShellMode::Playing if flow.mode == CampaignMode::Debrief => {
            "Review the one-time settlement and return to Mirror Square.".to_string()
        }
        ShellMode::Playing if flow.save.active_encounter.is_some() => {
            "Read the enemy intent and choose the next RPG combat action.".to_string()
        }
        ShellMode::Playing => flow
            .save
            .active_regional_quest_objective()
            .unwrap_or_else(|| flow.save.current_guide_step().prompt().to_string()),
    }
}

fn next_action(flow: &CampaignFlow) -> String {
    if let Some(spec) = campaign_action_specs(flow)
        .into_iter()
        .find(|spec| spec.enabled)
    {
        return spec.label;
    }
    if flow.mode == CampaignMode::Battle && flow.shell_mode == ShellMode::Playing {
        return "Select units, read the recommended command, then issue one deterministic order."
            .to_string();
    }
    if flow.settings.input_mode == InputMode::KeyboardOnly {
        return "Use the keyboard hint shown below the campaign panel.".to_string();
    }
    "No immediate action is available; review SYSTEM status for the blocker.".to_string()
}

fn room_label(room: CampaignRoom) -> &'static str {
    match room {
        CampaignRoom::MirrorSquare => "MIRROR SQUARE",
        CampaignRoom::MentorHall => "MENTOR HALL",
        CampaignRoom::ExpeditionGate => "EXPEDITION GATE",
        CampaignRoom::RelayQuarter => "RELAY QUARTER",
        CampaignRoom::CisternWard => "CISTERN WARD",
        CampaignRoom::NightWatchPost => "NIGHT WATCH POST",
        CampaignRoom::WorkshopGate => "WORKSHOP GATE",
        CampaignRoom::MarketWindPavilion => "MARKET WIND PAVILION",
        CampaignRoom::LanternInfirmary => "LANTERN INFIRMARY",
        CampaignRoom::ArchiveSteps => "ARCHIVE STEPS",
        CampaignRoom::CaravanYard => "CARAVAN YARD",
        CampaignRoom::OuterSignalRoad => "OUTER SIGNAL ROAD",
        CampaignRoom::GlassBasinWayhouse => "GLASS BASIN WAYHOUSE",
        CampaignRoom::DeepRelay => "DEEP RELAY",
        CampaignRoom::GlassReedMarsh => "GLASS REED MARSH",
        CampaignRoom::BasinObservatory => "BASIN OBSERVATORY",
        CampaignRoom::MoonBridge => "MOON BRIDGE",
        CampaignRoom::EmberOrchardEdge => "EMBER ORCHARD EDGE",
        CampaignRoom::AshBeaconField => "ASH BEACON FIELD",
        CampaignRoom::CinderRefuge => "CINDER REFUGE",
    }
}

fn status_tone(status: &str) -> UiTone {
    let status = status.to_ascii_lowercase();
    if ["failed", "error", "corrupt", "dead letter", "rejected"]
        .into_iter()
        .any(|needle| status.contains(needle))
    {
        UiTone::Critical
    } else if ["pending", "retry", "blocked", "warning", "in progress"]
        .into_iter()
        .any(|needle| status.contains(needle))
    {
        UiTone::Warning
    } else if ["ready", "saved", "complete", "recovered", "settled"]
        .into_iter()
        .any(|needle| status.contains(needle))
    {
        UiTone::Positive
    } else {
        UiTone::Neutral
    }
}

fn on_off(value: bool) -> &'static str {
    if value {
        "on"
    } else {
        "off"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> UiSnapshotInput {
        UiSnapshotInput {
            phase_label: "OPEN WORLD".to_string(),
            location_label: "MIRROR SQUARE".to_string(),
            objective: "Meet the mentor".to_string(),
            next_action: "TRAVEL TO MENTOR HALL".to_string(),
            online_attached: false,
            economy_mode: EconomyMode::OfflineLocal,
            ordinary_outbox: 0,
            compensation_outbox: 0,
            dead_letters: 0,
            save_label: "SLOT A · REV 4".to_string(),
            progress_label: "LEVEL 1 · MeetMentor · First Contact".to_string(),
            input_label: "Hybrid · Classic".to_string(),
            status: "Campaign ready".to_string(),
        }
    }

    #[test]
    fn offline_snapshot_never_claims_nakama_authority() {
        let snapshot = WorldUiSnapshot::from_input(input());
        assert_eq!(snapshot.authority_label, "OFFLINE WORLD");
        assert!(snapshot.authority_detail.contains(OFFLINE_AUTHORITY_PROFILE));
        assert!(!snapshot.authority_detail.contains("Nakama canonical"));
    }

    #[test]
    fn compatibility_snapshot_is_explicitly_noncanonical() {
        let mut input = input();
        input.online_attached = true;
        let snapshot = WorldUiSnapshot::from_input(input);
        assert_eq!(snapshot.authority_label, "COMPATIBILITY LAB");
        assert!(snapshot
            .authority_detail
            .contains(COMPATIBILITY_AUTHORITY_PROFILE));
        assert!(snapshot.authority_detail.contains("not Nakama canonical"));
        assert_eq!(snapshot.authority_tone, UiTone::Warning);
    }

    #[test]
    fn connected_economy_surfaces_pending_and_dead_letter_state() {
        let mut input = input();
        input.economy_mode = EconomyMode::CexConnected;
        input.ordinary_outbox = 2;
        input.compensation_outbox = 1;
        input.dead_letters = 1;
        let snapshot = WorldUiSnapshot::from_input(input);
        assert_eq!(snapshot.economy_label, "CEX ATTENTION");
        assert!(snapshot.economy_detail.contains("dead letters 1"));
        assert_eq!(snapshot.economy_tone, UiTone::Critical);
    }

    #[test]
    fn every_page_has_a_distinct_five_second_read() {
        let snapshot = WorldUiSnapshot::from_input(input());
        assert!(snapshot.body_for(WorldUiPage::Now).contains("NEXT ACTION"));
        assert!(snapshot.body_for(WorldUiPage::System).contains("AUTHORITY"));
        assert!(snapshot.body_for(WorldUiPage::Help).contains("READING ORDER"));
        assert_ne!(
            snapshot.body_for(WorldUiPage::Now),
            snapshot.body_for(WorldUiPage::System)
        );
    }

    #[test]
    fn status_tone_fails_closed_for_corruption_and_dead_letters() {
        assert_eq!(status_tone("slot corrupt"), UiTone::Critical);
        assert_eq!(status_tone("dead letter detected"), UiTone::Critical);
        assert_eq!(status_tone("wallet sync in progress"), UiTone::Warning);
        assert_eq!(status_tone("campaign ready"), UiTone::Positive);
    }
}
