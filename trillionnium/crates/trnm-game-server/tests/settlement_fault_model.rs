//! Executable P0 settlement safety model.
//!
//! These tests cover the failure semantics that the production
//! capture/execute/apply implementation must preserve.  The source-boundary
//! test additionally prevents the legacy regression where a synchronous CEX
//! reconciliation is called before the surrounding PostgreSQL transaction is
//! committed.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
struct CampaignState {
    revision: u64,
    state_hash: String,
    applied_receipts: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CapturedCampaign {
    campaign_id: String,
    revision: u64,
    state_hash: String,
    intent_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SettlementCapture {
    match_id: String,
    terminal_marker: String,
    campaigns: Vec<CapturedCampaign>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExecutedCampaign {
    captured: CapturedCampaign,
    receipt_ids: Vec<String>,
    fully_reconciled: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ApplyOutcome {
    AppliedSettled,
    AppliedPending,
    StaleCapture,
    NotClaimable,
}

#[derive(Default)]
struct IdempotentRemoteLedger {
    committed: BTreeMap<String, String>,
    commit_count: BTreeMap<String, u64>,
    lose_next_response_after_commit: BTreeSet<String>,
}

impl IdempotentRemoteLedger {
    fn lose_next_response_after_commit(&mut self, intent_id: &str) {
        self.lose_next_response_after_commit
            .insert(intent_id.to_owned());
    }

    fn execute(&mut self, intent_id: &str) -> Result<String, &'static str> {
        if let Some(receipt) = self.committed.get(intent_id) {
            return Ok(receipt.clone());
        }
        let receipt = format!("receipt:{intent_id}");
        self.committed.insert(intent_id.to_owned(), receipt.clone());
        *self.commit_count.entry(intent_id.to_owned()).or_default() += 1;
        if self.lose_next_response_after_commit.remove(intent_id) {
            return Err("ambiguous transport outcome");
        }
        Ok(receipt)
    }
}

fn capture(
    match_id: &str,
    terminal_marker: &str,
    campaigns: &BTreeMap<String, CampaignState>,
    intents: &BTreeMap<String, Vec<String>>,
) -> SettlementCapture {
    let campaigns = campaigns
        .iter()
        .map(|(campaign_id, state)| CapturedCampaign {
            campaign_id: campaign_id.clone(),
            revision: state.revision,
            state_hash: state.state_hash.clone(),
            intent_ids: intents.get(campaign_id).cloned().unwrap_or_default(),
        })
        .collect();
    SettlementCapture {
        match_id: match_id.to_owned(),
        terminal_marker: terminal_marker.to_owned(),
        campaigns,
    }
}

fn execute(
    capture: &SettlementCapture,
    remote: &mut IdempotentRemoteLedger,
) -> Result<Vec<ExecutedCampaign>, &'static str> {
    capture
        .campaigns
        .iter()
        .map(|captured| {
            let receipts = captured
                .intent_ids
                .iter()
                .map(|intent_id| remote.execute(intent_id))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ExecutedCampaign {
                captured: captured.clone(),
                receipt_ids: receipts,
                fully_reconciled: true,
            })
        })
        .collect()
}

fn apply(
    capture: &SettlementCapture,
    executed: &[ExecutedCampaign],
    expected_match_id: &str,
    current_terminal_marker: &str,
    campaigns: &mut BTreeMap<String, CampaignState>,
) -> ApplyOutcome {
    if capture.match_id != expected_match_id || capture.terminal_marker != current_terminal_marker {
        return ApplyOutcome::NotClaimable;
    }
    if capture.campaigns.len() != executed.len()
        || capture.campaigns.iter().any(|captured| {
            campaigns.get(&captured.campaign_id).is_none_or(|current| {
                current.revision != captured.revision || current.state_hash != captured.state_hash
            })
        })
    {
        return ApplyOutcome::StaleCapture;
    }

    let all_reconciled = executed.iter().all(|item| item.fully_reconciled);
    for item in executed {
        let current = campaigns
            .get_mut(&item.captured.campaign_id)
            .expect("capture equality was checked before mutation");
        current
            .applied_receipts
            .extend(item.receipt_ids.iter().cloned());
        current.revision = current.revision.saturating_add(1);
        current.state_hash = format!("applied:{}", current.revision);
    }
    if all_reconciled {
        ApplyOutcome::AppliedSettled
    } else {
        ApplyOutcome::AppliedPending
    }
}

fn baseline_campaigns() -> BTreeMap<String, CampaignState> {
    BTreeMap::from([
        (
            "campaign-a".to_owned(),
            CampaignState {
                revision: 7,
                state_hash: "hash-a7".to_owned(),
                applied_receipts: BTreeSet::new(),
            },
        ),
        (
            "campaign-b".to_owned(),
            CampaignState {
                revision: 3,
                state_hash: "hash-b3".to_owned(),
                applied_receipts: BTreeSet::new(),
            },
        ),
    ])
}

fn baseline_intents() -> BTreeMap<String, Vec<String>> {
    BTreeMap::from([
        ("campaign-a".to_owned(), vec!["intent-a".to_owned()]),
        ("campaign-b".to_owned(), vec!["intent-b".to_owned()]),
    ])
}

#[test]
fn ambiguous_remote_commit_retries_the_exact_intent_once() {
    let campaigns = baseline_campaigns();
    let capture = capture("match-a", "terminal-a", &campaigns, &baseline_intents());
    let mut remote = IdempotentRemoteLedger::default();
    remote.lose_next_response_after_commit("intent-a");

    assert_eq!(execute(&capture, &mut remote), Err("ambiguous transport outcome"));
    let executed = execute(&capture, &mut remote).expect("same intent IDs must be replayable");
    assert_eq!(executed[0].receipt_ids, vec!["receipt:intent-a"]);
    assert_eq!(remote.commit_count.get("intent-a"), Some(&1));
    assert_eq!(remote.commit_count.get("intent-b"), Some(&1));
}

#[test]
fn remote_success_followed_by_revision_drift_never_applies_stale_state() {
    let mut campaigns = baseline_campaigns();
    let capture = capture("match-a", "terminal-a", &campaigns, &baseline_intents());
    let mut remote = IdempotentRemoteLedger::default();
    let executed = execute(&capture, &mut remote).unwrap();

    let changed = campaigns.get_mut("campaign-a").unwrap();
    changed.revision += 1;
    changed.state_hash = "concurrent-change".to_owned();
    assert_eq!(
        apply(
            &capture,
            &executed,
            "match-a",
            "terminal-a",
            &mut campaigns,
        ),
        ApplyOutcome::StaleCapture,
    );
    assert!(campaigns
        .values()
        .all(|state| state.applied_receipts.is_empty()));

    let fresh = capture("match-a", "terminal-a", &campaigns, &baseline_intents());
    let replayed = execute(&fresh, &mut remote).unwrap();
    assert_eq!(
        apply(
            &fresh,
            &replayed,
            "match-a",
            "terminal-a",
            &mut campaigns,
        ),
        ApplyOutcome::AppliedSettled,
    );
    assert_eq!(remote.commit_count.get("intent-a"), Some(&1));
    assert_eq!(remote.commit_count.get("intent-b"), Some(&1));
}

#[test]
fn same_revision_with_a_different_persisted_hash_is_stale() {
    let mut campaigns = baseline_campaigns();
    let capture = capture("match-a", "terminal-a", &campaigns, &baseline_intents());
    let mut remote = IdempotentRemoteLedger::default();
    let executed = execute(&capture, &mut remote).unwrap();
    campaigns.get_mut("campaign-b").unwrap().state_hash = "tampered".to_owned();
    assert_eq!(
        apply(
            &capture,
            &executed,
            "match-a",
            "terminal-a",
            &mut campaigns,
        ),
        ApplyOutcome::StaleCapture,
    );
}

#[test]
fn terminal_marker_drift_blocks_every_campaign_write() {
    let mut campaigns = baseline_campaigns();
    let capture = capture("match-a", "terminal-a", &campaigns, &baseline_intents());
    let mut remote = IdempotentRemoteLedger::default();
    let executed = execute(&capture, &mut remote).unwrap();
    assert_eq!(
        apply(
            &capture,
            &executed,
            "match-a",
            "terminal-b",
            &mut campaigns,
        ),
        ApplyOutcome::NotClaimable,
    );
    assert!(campaigns
        .values()
        .all(|state| state.applied_receipts.is_empty()));
}

#[test]
fn two_workers_cannot_apply_one_capture_twice() {
    let mut campaigns = baseline_campaigns();
    let capture = capture("match-a", "terminal-a", &campaigns, &baseline_intents());
    let mut remote = IdempotentRemoteLedger::default();
    let first = execute(&capture, &mut remote).unwrap();
    let second = execute(&capture, &mut remote).unwrap();

    assert_eq!(
        apply(
            &capture,
            &first,
            "match-a",
            "terminal-a",
            &mut campaigns,
        ),
        ApplyOutcome::AppliedSettled,
    );
    assert_eq!(
        apply(
            &capture,
            &second,
            "match-a",
            "terminal-a",
            &mut campaigns,
        ),
        ApplyOutcome::StaleCapture,
    );
    assert_eq!(remote.commit_count.get("intent-a"), Some(&1));
    assert_eq!(remote.commit_count.get("intent-b"), Some(&1));
}

#[test]
fn partial_member_execution_keeps_the_match_pending() {
    let mut campaigns = baseline_campaigns();
    let capture = capture("match-a", "terminal-a", &campaigns, &baseline_intents());
    let mut remote = IdempotentRemoteLedger::default();
    let mut executed = execute(&capture, &mut remote).unwrap();
    executed[1].fully_reconciled = false;
    assert_eq!(
        apply(
            &capture,
            &executed,
            "match-a",
            "terminal-a",
            &mut campaigns,
        ),
        ApplyOutcome::AppliedPending,
    );
}

#[test]
fn production_source_never_reconciles_cex_while_its_function_has_an_open_transaction() {
    let source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let source = fs::read_to_string(source_path).expect("read game-server source");
    let needle = ".reconcile_economy(";
    let mut searched_from = 0;
    let mut calls = 0;

    while let Some(relative) = source[searched_from..].find(needle) {
        let call = searched_from + relative;
        searched_from = call + needle.len();
        let Some(function_start) = source[..call].rfind("async fn ") else {
            continue;
        };
        let Some(body_relative) = source[function_start..call].find('{') else {
            continue;
        };
        let body_start = function_start + body_relative;
        let header = &source[function_start..body_start];
        let prefix = &source[body_start..call];
        let function_name = header
            .strip_prefix("async fn ")
            .and_then(|rest| rest.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_')).next())
            .unwrap_or("unknown");
        let begins = prefix.matches(".begin()").count();
        let commits = prefix.matches(".commit()").count();

        assert!(
            !header.contains("Transaction<")
                && !header.contains("PgConnection")
                && !header.contains("PoolConnection"),
            "{function_name} accepts an open database owner while calling reconcile_economy",
        );
        assert!(
            begins <= commits,
            "{function_name} calls reconcile_economy after {begins} transaction begin(s) but only {commits} commit(s)",
        );
        calls += 1;
    }

    assert!(calls > 0, "settlement source no longer contains an economy reconciliation call; update this conformance test with the replacement boundary");
}
