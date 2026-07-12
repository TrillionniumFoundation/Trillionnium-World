# TRNM First Contact Human Validation — 2026-07-11

Status: executable validation packet; real human results remain pending.

This runbook closes the procedural part of the outstanding commercial
single-player usability gate (Gate B in
`trnm-native-game-release-gates-v1.md`). It does not block the already-green
native software-alpha gate. It
does not claim that an observer or player session happened. Automated tests,
screenshots, the facilitator and the developer cannot substitute for the
required independent people.

## Pre-flight

1. Run `scripts/check_trnm_human_validation_packet.sh`.
2. Start the release client with `scripts/run_trnm_first_contact.sh`.
3. Confirm the title shows the three save slots and create a disposable slot.
4. Keep `playtests/first_contact-visual-review.yaml` and
   `playtests/first-contact-human-play-session.yaml` open for direct recording.
5. Do not coach once either timed exercise starts.

## Five-second understanding gate

Use three independent observers. Show each observer the same live RTS screen
for at most five seconds, hide it, then ask exactly:

1. Who am I?
2. What is selected?
3. Where is the objective?
4. What command should I press next?

Record the answer and elapsed milliseconds immediately. An observer passes only
when all four answers are recognizable and the elapsed time is at most 5,000
ms. All three rows must contain real names or stable private participant IDs;
leave the packet pending if any row is absent or fails.

## 10–15 minute play session

Use a human who did not implement the feature. Before timing, explain only the
title controls and that `F4` opens the journal. During timing, do not direct the
player to a room, target or command.

The player should:

- create and confirm a character;
- use the guide/journal to meet the mentor, train, equip and reach the gate;
- choose a difficulty and preparation;
- deploy to RTS, issue meaningful orders and reach a terminal outcome;
- return to town and identify what changed.

Stop between 10 and 15 minutes. Record duration, whether RTS and town return
were reached, the first confusion, input/readability/progression issues and the
strongest moment. A failed mission is valid evidence; an unfinished round trip
is useful feedback but does not pass the closed-loop gate.

## Truth boundary

The software gate is green independently. Human P0 becomes green only when the
three observer rows and one play-session record contain real observations and
the product gate accepts them. Until then report: “validation packet ready;
three observers and one 10–15 minute human session pending.”
