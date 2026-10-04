---
status: current-candidate
owner: world-product-and-accessibility
applies_to_gap: WORLD-V5-018
last_reviewed: 2026-09-09
review_due: 2026-10-09
production_authorization: not_granted
---

# Trillionnium World human and accessibility evidence protocol v1

## Scope

This protocol qualifies player comprehension, unguided task completion and accessibility for one exact World binary/package. It cannot be satisfied by developer self-review, screenshots, scripted UI automation, synthetic personas, language-model judgement or a record that omits participant consent and independent review.

The qualified object is the exact package digest, platform build, content/rules revision, configuration and launch scope recorded in the evidence file. Any material UI, controls, tutorial, text, audio, online failure behavior or binary change invalidates the affected evidence.

## Roles and independence

The session executor may prepare equipment and observe sessions but may not coach participants after a task begins. The independent reviewer must be different from the executor, binary author and sole product approver. Conflicts are disclosed in the evidence record.

Minimum participants:

- three independent five-second comprehension observers;
- at least one non-developer participant completing the vertical slice without guidance;
- enough participants or assistive profiles to cover every declared accessibility mode rather than assigning untested modes a pass.

Participants use anonymized IDs. Consent, withdrawal and recording scope are captured before execution. Raw evidence containing voice, video or personal data is stored under the declared privacy/retention policy and is not committed to Git.

## Exact-object preflight

Before the first session, record and independently verify:

1. repository commit and tree;
2. package/binary SHA-256 and SBOM/provenance where available;
3. operating system, hardware, display size/scaling, input devices, audio path and locale;
4. content/rules/protocol/build identities;
5. settings and accessibility profile digest;
6. public-online mode, dependency availability and any mocked/offline scope;
7. task script and pass/fail thresholds frozen before observation;
8. consent and retention policy version.

No participant may be moved to a different binary after a failed task without creating a new evidence tuple.

## Five-second comprehension sessions

Show the intended first meaningful screen for five seconds, remove it and ask each observer—without leading prompts—to state:

- what product or activity they believe this is;
- what the next primary action is;
- whether the state is local, online, pending, failed or complete where shown;
- whether any economic or account consequence is implied;
- what blocked or unsafe condition is visible.

Record the verbatim response, task outcome and confusion codes. A majority guess is not enough: all three required observers must satisfy the predeclared essential concepts, or the criterion fails and the UI is revised for a new tuple.

## Unguided vertical slice

The non-developer participant receives only the declared starting instruction and must complete the selected 10–15 minute path without coaching. The path must include, where applicable:

- start or load a campaign;
- navigate authored RPG content;
- enter, understand and complete an RTS encounter;
- interpret the debrief and return to campaign state;
- save, exit and reload;
- understand an offline, reconnect, rejected-command or dependency-failure state;
- distinguish local progress, pending settlement and verified external effects.

Record completion, abandonment, misclicks, time to first correct action, recovery attempts, comprehension errors and participant comments. Any emergency intervention is recorded and fails the unguided criterion for that session.

## Accessibility matrix

Execute the relevant path under each declared profile:

- keyboard-only;
- mouse-only;
- high-contrast or equivalent visual profile;
- subtitles/captions for all required information-bearing audio;
- low-motion/reduced-animation profile;
- compact and wide viewport or window sizes;
- supported scaling/text-size settings;
- supported assistive input or screen-reader path when included in launch scope.

Each profile records focus visibility/order, control reachability, text clipping, contrast findings, timing dependencies, motion behavior, subtitle completeness, input remapping and recovery from errors. A profile excluded from launch scope must be explicitly excluded by the accountable product/accessibility authority; silence is not an exclusion.

## Online and failure comprehension

After the canonical Nakama component exists, at least one session must inject reconnect, stale generation, command rejection, full resynchronization and dependency-unavailable states. The participant must not interpret local speculation as canonical success or pending/ambiguous settlement as wallet completion.

A local compatibility server cannot satisfy the canonical-online portion. Until that portion exists, the human evidence record states the limitation and cannot authorize public online launch.

## Raw artifacts and privacy

Retain, as permitted by consent:

- session script and frozen thresholds;
- anonymized participant metadata relevant to accessibility only;
- exact binary/platform/configuration identities;
- timestamped observer notes and task events;
- screen/video/audio or equivalent raw record hashes;
- issue codes and remediation decisions;
- independent review decision;
- retention location, deletion date and access policy.

Do not store names, contact details, authentication tokens, wallet details or unnecessary biometric material in the World repository or evidence manifest.

## Acceptance

A PASS requires all applicable predeclared tasks and accessibility profiles to pass, no unresolved severity-one comprehension/accessibility defect, exact-object binding, an independent reviewer and an unexpired strict evidence record accepted by:

```bash
python3 scripts/check-trnm-world-external-evidence.py path/to/human-record.json
```

Automation may validate record structure but cannot create participant observations or the independent decision. A failed session remains evidence of a gap; it must not be deleted or rewritten as a pass.

## Change and invalidation

Requalification is required for material changes to navigation, controls, text, localization, onboarding, economy presentation, accessibility behavior, binary/package, platform integration or online failure semantics. Cosmetic changes may be dispositioned only by the independent reviewer with a recorded rationale and exact diff.

This protocol does not grant legal, privacy, support, custody, commercial or production authorization. Those remain separate evidence classes.
