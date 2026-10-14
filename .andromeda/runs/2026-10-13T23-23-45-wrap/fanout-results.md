# Fan-out results — 2026-10-13-stand-keyboard-harness (resume of the 2026-10-13T23-23-45 wrap)

Batch: one parallel batch of seven `general-purpose` doc-agents. Snapshot before: `tree f6f2e6a52ba5e5c09c63ada49b50eed1626f9267
index c94b6a43657523a0a641e0965fd701cb943f59d5 head 71af33d06f0a782469c7589c5ff88245af780bbe`. Snapshot after: `tree
4c65c0be2c4c3c0a0acf41a0a91a4958d97b3488`, with index and head unchanged. `git diff-tree` printed exactly the seven `A
.andromeda/runs/2026-10-13T23-23-45-wrap/fanout-{doc}.md` lines. Every file opens `proposals:`; entity probe 0 in all
seven; no fix, so there are no twins; every receipt count equals its file's.

Validate: no proposal is rejected for a source the report does not carry. Every coordinate a proposal writes is the
report's or a row, an `@` line or an added range of its `## New text, by line` (spans from one row's first number to
another row's last included). Check 2 (cross-contradiction): no pair edits one section in opposing directions. Check 3
(intent): the scope record is clean (0 lines); the F4 deferral and the F5 gap are the plan's own, so nothing diverges.
Check 5: every `Expected amendments (wrap)` entry of plan.md:366-387 is matched below, and every `claim false` row of
citation-dispositions.md is matched or raised. Check 6: the report's four `Spec claims disproved` entries are all
DISPOSED — the `disabled` parse (architecture 2, 3; security-plan 2, 3; a11y-plan 1, 2, 3; test-plan 9), the click
leaving no focus (a11y-plan 4; test-plan 3, 4), no key activating a stand button (a11y-plan 6; scope.md is a chunk
artifact, no master states it), and the inert claim (architecture 4, 5, 6; a11y-plan 11).

## architecture: 7 proposals · fanout-architecture.md
note: the agent flags §Existing Scopes (architecture.md:263) as stale on the `common` counts and the unlisted `stand_keyboard` — raised as item 8.
1. card — playbook :64 (an engine change for every document, on the founder's fork answers) holds for F2 and F3, but F1 widens what crosses a boundary (playbook :36, judged by its subject), so the proposal escalates; it lies wholly inside the founder's F1–F3 answers (inputs#I3) → `answer-card.md`
2. apply — playbook :64 (F3, «По наличию (Рекомендую)», inputs#I3; narrows, no widening) and check 6; the `<div disabled tabindex=0>` limit's owner is card 2 row 5 (inputs#I6)
3. apply — dependent of 2 (check 6 groups it); playbook :28
4. apply — playbook :56 (a disproved claim about code the chunk did not edit — the engine never honoured `inert` — amended to the measured limit, the fix a CARRY: card 2 row 3, inputs#I6)
5. apply — dependent of 4; playbook :56
6. apply — playbook :28 (report Counts: `dioxus_falsy_boolean_attrs` 27 → 28); the applied text also restates "the 27 names" (citation row architecture.md:263@c4882, claim false)
7. card — §Standard Contracts → Events: the Enter/Space → Click default actions are F1's subject (playbook :36), and KeyUp's default action is a new route into the same widening; inside the founder's F1 and F2 answers (inputs#I3) → `answer-card.md`
8. apply (raised by main) — §Existing Scopes: the stand checks' shared module states `tab_order` and `focused` as well, read by 19 stand checks, `stand_keyboard` the nineteenth (was 18, `stand_contrast` the eighteenth), and the new target `stand_keyboard` is listed — playbook :28; report Counts (common readers 18 → 19) and Symbols (tests' shared module); citation row architecture.md:263@c5409, claim false

## security-plan: 3 proposals · fanout-security-plan.md
note: D-security-auth and D-security-deps hold (no identity handling; Cargo.lock +[[package]] 0). The keyboard.rs citations below the added ranges were the citation sweep's (moved and written).
1. card — Boundary widening (playbook :36): Enter and Space now reach the click default actions (form submission, link navigation, `summary` toggling) in every document, remote pages included; answered by the founder himself at the P4 fork F1 «Клавиши нажимают (Рекомендую)», with F2 «Клик даёт фокус (Рекомендую)» for the click's focus, relayed verbatim (inputs#I3) → `answer-card.md`
2. card — the `disabled` row: F3 is playbook :64 and :52 routine on its own (it narrows), but the row it restates now gates F1's activation arms, so it rides the card with the widening it gates; inside F3 (and F1) of inputs#I3 → `answer-card.md`
3. card — dependent of 2 (the `attr_parsed` row, the same retired boolean parse); check 6 groups it with 2 → `answer-card.md`

## design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds — every Coverage row reads `tokens n/a`; F5 adds no class or style.

## layout-templates: 1 proposal · fanout-layout-templates.md
note: the five "adding no … order" hits at :10 speak of DOM order for earlier chunks' ids and stay true; :45's geometry is no-change.
1. apply — playbook :28 (the operator's F5, "List takes focus", inputs#I4); plan.md:385-387 names it

## test-plan: 15 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness hold. The agent left `stand_act_diff` and `stand_act_spans` as written; read by main at test-plan.md:26 @c15321 and @c21166, both still true (a click "names exactly the nodes stated for it"; the span check's call counts unmoved). The root-fallback reading of `Harness::focused()` is carried by a11y-plan 12. test-plan.md:26 @c4516 and :99 state no focusable reading (read by main), so nothing is raised there.
1. apply — playbook :28 (the new target; Coverage of new surfaces)
2. apply — playbook :28 (report Counts `stand_act_keys` +1)
3. apply — check 6 (click-focus disproved; restated `stand_act_obstructed`); playbook :64 (F2) — the reading of a check, no widening
4. apply — dependent of 3
5. apply — playbook :28 (report Counts: 9 → 15 control rows; Decisions: rows over `controls(task)`, not distinct controls)
6. apply — playbook :28; citation row test-plan.md:26@c29262, claim false
7. apply — playbook :28 and :56 (27 → 28 names; the engine honours no `inert`)
8. apply — playbook :28 (CRUD Tab sequence 5 → 6)
9. apply — check 6 (the snapshot pin restated); playbook :64 (F3)
10. apply — playbook :28 (`focusability_updates` +1)
11. apply — playbook :28; citation row test-plan.md:66@c1261, claim false
12. apply — playbook :28; citation rows test-plan.md:100 ×2, claim false
13. apply — playbook :28 (report Outcome: CI#38482525151 `Test (windows)` and `Test (macos)` green)
14. apply — playbook :28 (report Counts: `run stand` 141 → 153, 37 → 38 files)
15. apply — playbook :28 (report Counts: workspace 181 · 915 → 182 · 928)

## obs-plan: 2 proposals · fanout-obs-plan.md
note (the agent's three unmeasured readings, kept for P5 and none of them a proposal, since none is in the report): (1) the silence probe read `stand_keyboard.rs` alone, not the shared module's added `tab_order` / `focused`, which §3's "no env read in … their shared modules" also claims; (2) under F2 a click now focuses a button, so the engine's `focussed node` event may fire on more steps than the dated host/MCP line counts recorded — `host_log` / `mcp_log` stayed green, but no count was re-measured; (3) F1's Enter on an `a href` gives the existing pointer.rs href log site a keyboard path — no new site, engine targets scrubbed in escher's sink.
1. apply — playbook :28; citation row obs-plan.md:70@c10243, claim false
2. apply — playbook :28 (Coverage: `stand_keyboard` silent, probe 0)

## a11y-plan: 13 proposals · fanout-a11y-plan.md
note: D-a11y-obs-schema holds. The agent flags §9's a11y-leg count — raised as item 14.
1. apply — playbook :64 (F3, inputs#I3); the known limit is the operator's condition (inputs#I5); citation row a11y-plan.md:144, claim false
2. apply — dependent of 1; citation rows a11y-plan.md:148 ×2, claim false
3. apply — dependent of 1
4. apply — playbook :64 (F2, «Клик даёт фокус (Рекомендую)», inputs#I3; no widening — a click is already an input, the focus it moves crosses nothing) and check 6
5. apply — dependent of 4; citation row a11y-plan.md:279@c10017, claim false
6. card — §5 Keyboard handling: Enter and Space activation in every document is F1's subject (playbook :36); inside the founder's F1 answer (inputs#I3) → `answer-card.md`
7. apply — playbook :28 (the operator's F5, inputs#I4; the gap stated plainly per its condition, owner card 2 row 1)
8. apply — dependent of 7
9. apply — dependent of 7
10. apply — playbook :28 (CRUD forward sequence 5 → 6)
11. apply — playbook :56 (inert; owner card 2 row 3); citation row a11y-plan.md:185, claim false
12. apply — playbook :28 (the new target; the root-fallback engine reading the report measured, document.rs:647-650)
13. apply — dependent of 12
14. apply (raised by main) — §9 CI Integration: the a11y leg runs `accessibility_hidden`, `accessibility_roles` and `focusability_updates`, 6 + 6 + 4 tests (was 6 + 6 + 3; report Counts `focusability_updates` +1) — playbook :28

## Resumed 2026-10-14 — the card answered, the apply
The founder answered the card «Подтвердить (Рекомендую)» (inputs#I7, `answer-card.md` §The answer): the six card
proposals apply as his ratification, beside the 37 routine ones.

Raised by main at the apply, each a routine amendment (playbook :28):
- architecture 9 — §Existing Scopes: the twelve driver-action files hold 41 tests and two ignored children (was 40;
  `#[test]` counted 43 over the twelve files, 2 of them ignored children), `stand_act_keys` adding Enter and Space through
  the driver.
- architecture 10 — §Standard Contracts → Driver session, the `covered` reading: after the accepted click the clicked
  button reads `focused` (was "no element reads `focused`"). Found by the cascade sweep (`cascade-dispositions.md`).
- a11y-plan 15 — §7: "with the Tab order unchanged" bound to its 2026-10-06 chunk, CRUD's restatement named. Found by
  the cascade sweep.
- test-plan 16 — registry Session lifecycle: `stand_act_keys` 3, the thirteen driver-action files 47 (was 2 and 46).
  Found by the cascade sweep; `registry.py check` 0 defects.

obs-plan note (1), measured at the apply: the silence probe over `tests/blitz-tests/tests/common/mod.rs` reads 0 (`grep -c
-E 'println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\('`,
exit 1, `0`), so §3's "no env read in … their shared modules" holds; written into obs-plan 2. Notes (2) and (3) stay
unmeasured and go to P5.

Applied: 47 amendments — architecture 10, security-plan 3, layout-templates 1, test-plan 16, obs-plan 2, a11y-plan 15 —
in 8 sidecar entries. Rejected 0.

Totals: 41 proposals read + 2 raised by main = 43 — apply 37 · card 6 (architecture 1, 7; security-plan 1, 2, 3; a11y-plan 6) · reject 0. No proposal was rejected for a source the report does not carry. No playbook rule is proposed at this card: the widening class is never-routine and stays so.
