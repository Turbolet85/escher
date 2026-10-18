# Report — 2026-10-18-engine-debts (gated wrap)

**Chunk:** Engine debts — the nine recorded engine deferrals each fixed, pinned or kept on the founder's fork
**Date:** 2026-10-18
**Commits:** none of the gated part. The take-up is `48680dfb9` (operator pre-CI commit) + `fb8426711` (its wrap commit,
the gated flip, `git log -1 -S'2026-10-18-engine-debts · gated' -- .andromeda/master-route.md`); its report stands at
`report.md`. The four commits since the flip, `275950dc0` · `f45bb570a` · `87f120794` · `1c9bc58b5`
(`git log fb842671..HEAD`), are the two later chunks' (2026-10-18-upstream-sync-determinism-and-traces,
2026-10-18-one-virtual-clock) — none of this chunk's, by the operator's ruling (inputs#I10, Deviations). Since
`state.yaml` `last_wrap` (2026-10-18T09:25:30Z): `1c9bc58b5`, the previous wrap's own commit.
**Wrap mode:** GATED WRAP, two-window, on the operator's word at invocation «run P1 only and stop» (inputs#I10): this
window writes P1 alone; P2–P7 resume in run dir `.andromeda/runs/2026-10-18T20-46-02-wrap` (Setup 2a).

## Changes (structured — detectors read this)
- **Files:** the gated part touches no source, test, script, workflow, manifest, spec master, registry or leaf
  (`cites.py added --base HEAD`: `new text: 0 file(s)` — every changed path is a pipeline artifact). It changed:
  - `escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/founder-word.md` — new, 9 rows `| F{n} | {answer} | ratified
    by the founder | 2026-10-18 |` (F1 A · F2 A · F3 B · F4 A · F5 A · F6 A · F7 A · F8 A · F9 A), the `switch` mapping
    a line under F2 (plan step 13; gated implement, `.andromeda/runs/2026-10-18T20-41-41-implement/outcome-2026-10-18-engine-debts.md`);
  - `escher-0.2.0/chunks/2026-10-18-engine-debts/inputs/` — I9 (the founder's word, copy) and I10 (this wrap's
    directive, message) added, `andromeda-inputs.json` with them;
  - `escher-0.2.0/verification-matrix.json` — v020-03 `planned → implemented`, its ref and the clearance note (+960
    chars; `matrix-2026-10-18-engine-debts.json` in the implement run dir);
  - `.andromeda/residuals.md:33` — the native-checkedness line, the founder's word appended at this wrap on the
    operator's word (inputs#I10 item 1): «the founder's «Исправить позже (Рекомендую)» (2026-10-18, by dialog, relayed
    by the overseer, inputs#I9): it stays open, to be fixed in a later chunk or with the nearest accessibility work»;
    its status `open` and target `next` unchanged;
  - bookkeeping: `.andromeda/friction-log.ndjson`, the run dirs `.andromeda/runs/2026-10-18T20-41-41-implement/` and
    `…/2026-10-18T20-46-02-wrap/`, and the previous wrap's transient `evolve-…`/`health-…` trails and handoff
    (`.andromeda/runs/2026-10-18T08-45-11-wrap/`, `.claude/session-handoff.md`).
- **Symbols / APIs:** none moved by the gated part. What moves is the STANDING of the take-up's seven engine
  behaviour changes for every document (F1 `inert` honoured · F2 the toggle read per role with the `role="switch"` →
  `Role::Switch` mapping · F4 `Node::absolute_position` · F5 the hit's stop at a `contain: paint` box · F6 the password
  click caret by the bullets · F7 `disabled` on a form control alone · F8 the client rect under transforms) and its two
  pins (F3 `aria-activedescendant` unread · F9 `covered` on a box with no area), all recorded at `report.md` as
  PROVISIONAL on the operator's word as the founder's delegate (inputs#I2, inputs#I3): **ratified by the founder,
  2026-10-18** — «Утвердить всё (Рекомендую)», by question dialog in the overseer session, his chosen labels relayed
  verbatim by the overseer (inputs#I9 item 1), every fork answer as given, none reworked. The same relay carries the
  founder's word on two batches of the later chunks, recorded PROVISIONAL at their wraps:
  - 2026-10-18-upstream-sync-determinism-and-traces F1 B — the cherry-pick of `fb8d6c15e` alone, Taffy kept on the
    registry 0.15 — «Утвердить (Рекомендую)», ratified; his no-git-dependency rule of 2026-10-13 stands (inputs#I9 item 2);
  - 2026-10-18-one-virtual-clock (v020-06) F1 B · F2 A · F2b A · F3 A — an embedder-supplied clock, wall by default ·
    `advance` moves the session clock by exactly N · `time-unavailable` retired (eight causes) · the double-click window
    on the session clock — «Утвердить всё (Рекомендую)», ratified (inputs#I9 item 3).
- **Crates / modules:** none.
- **Dependencies:** none.
- **Schema / config:** none.
- **Spec-master edits:** none (the seven masters and every registry file equal HEAD — `cascade.py sweep --pre-pass`
  ran, which refuses otherwise).
- **Counts / qualifiers moved:** the PROVISIONAL mark on the three ratified batches — each fact with its pattern
  (`{run_dir}/report-patterns.toml`, trail `cascade-2026-10-18-engine-debts-pre-pass.json`):
  - **the engine-debts batch ratified** — `prov` (`PROVISIONAL`) with `ed-batch`
    (`2026-10-18-engine-debts|v020-03|delegate operator|…|role="switch"`). `prov`: 57 standing rows, 71 marks over six
    masters and one key file; each mark read at its row or its `by line` offset and assigned to the batch it records
    (no mark belongs to any other batch):

    | class | rows / marks | sites (`file:line`, ×marks) |
    |---|---|---|
    | engine-debts (F1–F9, `switch`) | 28 / 34 | architecture 86 ×4 · 90 · 98 · 102 · 143 · 145 ×2 · 149 · 164 · 178 ×2; security-plan 119 · 127 · 128 · 130 · 131; design-system 216; test-plan 19 · 93; a11y-plan 46 · 105 · 147 · 151 · 161 · 170 · 190 · 200 · 268 · 285 ×2 · 287 |
    | one-virtual-clock (F1 B · F2 A · F2b A · F3 A) | 26 / 34 | architecture 91 · 139 · 169 · 173 · 175 ×5 · 180; security-plan 81 ×2 · 217 · 357 · 403; design-system 291; test-plan 10 · 22 · 27 ×3 · 88 · 89 · 144 · 183 · 287 · 327; obs-plan 114 ×2 · 115 · 188; a11y-plan 162; registries `contracts/test-plan/session-lifecycle.md` 5 · 11 |
    | cursor fix (F1 B) | 3 / 3 | architecture 248 · security-plan 246 · design-system 316 |

    Windows read for the classing: architecture:86 @c9528 (F7, «on a form control alone (PROVISIONAL)»),
    security-plan:131 @c6084 (F6), security-plan:217 @c1461 (the no-clock document fading by wall time), test-plan:27
    @c21281 (`advance`'s step) and @c35881 (the causes served through MCP), a11y-plan:46 @c1246 (the residual).
    `ed-batch`: 9 standing — architecture:90 · architecture:143 · security-plan:357 · obs-plan:74 cite
    (a citation of the chunk's report or evidence, no change); test-plan:328 · test-plan:334 true (a measured reading at
    that chunk, no change); security-plan:127 · a11y-plan:46 · a11y-plan:105 true (the `switch` mapping, its mark in the
    engine-debts class).
  - **the cursor fix ratified** — `prov` (class above) with `cf-batch` (`fb8d6c15e|cursor fix|cherry-pick`): 2 standing,
    architecture:248 ×3 · security-plan:246 ×2 — true (the take as measured), the line's mark in the cursor-fix class.
  - **the one-virtual-clock batch ratified** — `prov` (class above) with `vc-batch`
    (`2026-10-18-one-virtual-clock|v020-06|founder's epoch batch|F2b`): 7 standing — architecture:173 · design-system:291 ·
    test-plan:10 · test-plan:27 · test-plan:287 cite; test-plan:328 · test-plan:334 ×2 true (re-reads at that chunk).
  - **the native-checkedness residual carries the founder's word** — `chk-resid`
    (`native checkedness|native checkbox or radio reading neither|checkbox_input_checked|natively checked|own checked state`):
    5 standing — architecture:143 @c4821 · architecture:178 @c15652 · test-plan:27 @c11543 · a11y-plan:105 true (F2's
    native exclusion; the snapshot's native reader); a11y-plan:46 @c1246 owner («a measured absence, its fix owned by a
    `.andromeda/residuals.md` l[ine]») — the one master site stating the residual's ownership; the word is in
    `residuals.md:33` now.
  - **the block cleared, its evidence present** — `fw-block` (`founder-word\.md|founder's own word|Provisional
    discharge`): 0 — `control: fw-block, own-title never fired over the seven masters' pre-pass text (1c9bc58b)`; no
    master names the block's evidence.
  - **Epoch 2 closes at this wrap** — `ep2` (`Epoch 2`): 1 standing, test-plan:25 @c6820 true — «the Epoch 2 code audit»
    is escher-0.1.0's epoch, no change (epoch labels repeat across versions).
  - **v020-03 implemented** — no master site: a ledger entry (`verification-matrix.json`).
  - Owner patterns: `own-title` (`Engine debts`) — owner pattern: control unfired (the refusal line above);
    `own-generic`: 2 standing — architecture:90 true (the merge surface's CARRY on «Upstream sync ahead of visual lint
    and real-app readiness», as placed at 2026-10-18-one-virtual-clock), security-plan:123 @c8547 true (another
    subject).
  - Lines over 2 000 chars, one word per (line, fact) pair at every offset rows and the `by line` block print:
    architecture:86 ED×4 · :98 ED · :143 ED, cite, true · :145 ED×2 · :164 ED · :173 VC, cite · :175 VC×5 · :178 ED×2,
    true · :180 VC; security-plan:81 VC×2 · :119 ED · :123 true · :127 ED, true · :130 ED · :131 ED · :217 VC · :357 VC,
    cite · :403 VC; test-plan:10 VC, cite · :22 VC · :25 true · :27 VC×3, cite, true · :89 VC · :144 VC · :183 VC · :327
    VC · :328 true, true · :334 true, true×2; obs-plan:114 VC×2 · :115 VC; a11y-plan:46 ED, true, owner · :170 ED · :285
    ED×2; session-lifecycle.md:5 VC · :11 VC (ED · VC · CF: the mark the founder's word ratifies, its batch).
  - Sweep reach: 7 patterns · 7 moved facts, 6 with a pattern (the report's own count) · owner pattern control unfired ·
    by line 9 lines · 15 offsets.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none.
- **Cross-project / external claims:** the inputs read (`inputs.py verify`, trail `inputs-2026-10-18-engine-debts.json`):
  - I1 · message: the operator, at the /andromeda-phase invocation · copy message · n/a — cited scope · research · plan;
  - I2 · message: the operator, in the founder's stead as delegate · copy message · n/a — cited plan · report.md;
  - I3 · message: the operator, at the P5 review of p5-card-1.md · copy message · n/a — cited plan · report.md;
  - I4 · message: the operator, at the P5 review of p5-card-2.md · copy message · n/a — cited report.md;
  - I5 · I6 · message: the operator, in the implementing session · copy message · n/a — cited report.md;
  - I7 · message: the operator, at the take-up wrap's resume · copy message · n/a — cited report.md's addendum;
  - inputs#I8 · message: the operator, by question dialog in the take-up wrap, 2026-10-18 05:12Z · copy message · n/a —
    «Stamp BLOCKED-ON, then gate (Recommended)», «Accept, continue (Recommended)»: the BLOCKED-ON this gated wrap
    clears was stamped on it (escher-0.2.0/working-route.md:20);
  - inputs#I9 · `../additional/escher-overseer/relays/provisional-batch-founder-word-2026-10-18.md` · copy no-repo ·
    unchanged — the founder's word on the PROVISIONAL batch, 2026-10-18 about 20:40Z, by question dialog in the escher
    overseer session, relayed by the overseer, his chosen labels verbatim; four items (Symbols / APIs above; item 4
    the residual, Files above). Snapped at the gated implement (`--step implement`);
  - inputs#I10 · `.andromeda/runs/2026-10-18T20-46-02-wrap/relay-1.md` · copy message · n/a — this wrap's invocation
    directive: P1 only; the founder's word appended to `residuals.md:33`; the scope ruling (Deviations). Snapped at
    this wrap (`--step wrap`).
  - `inputs: 10 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 9 · uncited 3 · unparsed 0`,
    fired before this report existed; I8 · I9 · I10 are cited here. No DRIFT.
  - CI: no run measured the gated part (it changed no source). The take-up's own verdict: CI#39096326308 on
    `48680dfb98f3`, green 17/17 (`evidence/operator-pass.md`). HEAD `1c9bc58b5` carries the later chunks' source; its
    source tree equals `87f12079`'s (`git diff --stat 87f12079 1c9bc58b -- packages examples tests apps` empty), and
    CI#39111579925 read `87f1207940e1 verdict: green · checks 17/17` — recorded in
    `escher-0.2.0/chunks/2026-10-18-one-virtual-clock/evidence/operator-pass.md`, not in this chunk's evidence.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none.
- **Expected amendments (from plan):** the plan's seven entries (plan.md, «Expected amendments (wrap)») were carried
  and applied at the take-up's wrap (report.md; run `.andromeda/runs/2026-10-18T04-30-47-wrap`), each engine change
  recorded PROVISIONAL there — superseded here but for one remainder: the entry on architecture §Established Decisions
  («each PROVISIONAL on `delegate operator` (inputs#I2) … under «Provisional discharge» (playbook:48)») — carried:
  Counts / qualifiers moved, the engine-debts class (`prov` 28 rows / 34 marks · `ed-batch` 9 standing). The
  ledger-note entry: none listed.
- **Coverage of new surfaces:** none — the gated part adds no surface.

## Deviations from intent
1. **The scope read and the operator's ruling.** `gate.py scope` (P1, trail `gate-2026-10-18-engine-debts.json`):
   `scope: unrecorded 21 files · changed 43 · listed 22 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 ·
   widening 0) · absorbed 0 · excluded 438`, base `34bcf71c` (the parent of the oldest pre-CI commit `48680dfb`). The
   21 are exactly the source files changed after the gated flip and in neither the take-up nor the gated tree
   (`git diff --name-only fb842671 HEAD` less `git diff --name-only 34bcf71c fb842671`, the excluded paths dropped, 21 =
   21): `examples/seven_guis/src/stand.rs` · `examples/seven_guis/tests/{cli_commands,host_timer}.rs` ·
   `packages/blitz-dom/src/{clock,config,iframe,lib,resolve}.rs` · `packages/blitz-test-harness/src/harness.rs` ·
   `packages/escher-driver/src/{describe,execute,schema_tests,session}.rs` · `tests/blitz-tests/tests/session_common/mod.rs` ·
   `tests/blitz-tests/tests/{stand_act_spans,stand_act_timer,stand_mcp_tools,stand_non_lean_tasks,stand_screenshot_repeat,stand_virtual_clock,virtual_clock_engine}.rs`.
   The operator's ruling, word: "the base is 34bcf71c, but this chunk is only 48680dfb + fb842671 + this gated tree.
   Files changed only in 275950dc, f45bb570, 87f12079 and 1c9bc58b belong to the two later chunks: write no
   scope-record lines for them, state this ruling (operator word) in the report, and treat a scope stop on exactly that
   set as answered" — the operator, 2026-10-18 (inputs#I10). So `scope-record.md` gets no line and the stop on those 21
   is answered; P7.3c's re-read halts only on a row outside that set. Seven files changed in both the take-up and the
   later commits (`packages/blitz-dom/src/{document,scrolling}.rs` · `events/pointer.rs` ·
   `packages/escher-driver/src/{json,lib,refusal,schema}.rs`) read `listed`; their later edits are the later chunks'
   by the same ruling.
   scope record: none — 0 recorded; the scope read's one stop answered by the operator's ruling above.
2. **The new-text listing's base.** The GATED WRAP's letter passes its basis commit (`--base fb842671`). Fired without
   `--out` first, that base listed `28 file(s) · 3 new · 128 added range(s) · 843 line(s) · 59 block(s)`, every one
   the later chunks' text (`packages/blitz-dom/src/clock.rs`, `tests/blitz-tests/tests/virtual_clock_engine.rs` …;
   the cites trail holds the call). Pasting that as this report's new text would attribute it to this chunk against
   the ruling, so the listing was written with `--base HEAD` (the gated part's own tree): `new text: 0 file(s) · 0 new ·
   0 added range(s) · 0 line(s) · 0 block(s)`. P7.0 re-fires the same call (`--base HEAD`, the same `--out`).
3. **The gated implement fired entry 26 alone** (implement's outcome file, Deviations): on the operator's invocation;
   the block is re-run by P7's light gate.

## Decisions & corrections
- The founder's word on the PROVISIONAL batch (inputs#I9; the founder, 2026-10-18 about 20:40Z, by question dialog in
  the escher overseer session, relayed by the overseer, his labels verbatim): item 1 «Утвердить всё (Рекомендую)» — the
  nine engine-debts answers F1 A · F2 A · F3 B · F4 A · F5 A · F6 A · F7 A · F8 A · F9 A, the seven fixes and the
  `switch` mapping; item 2 «Утвердить (Рекомендую)» — the cursor fix F1 B; item 3 «Утвердить всё (Рекомендую)» —
  one-virtual-clock F1 B · F2 A · F2b A · F3 A; item 4 «Исправить позже (Рекомендую)» — `residuals.md:33` stays open,
  its fix an engine change for every document, by his word, when taken up. **For P2 (Validate → the Provisional
  discharge escalation, playbook:48):** the epoch-closing listing is Counts / qualifiers moved's `prov` table, 57
  master and key-file rows, with their leaves (`prov` leaf 50 rows); the engine-debts class is this chunk's and Epoch
  2's; the cursor-fix and one-virtual-clock classes are Epoch 3 chunks' marks, which the rule discharges «in one batch
  at each epoch boundary» while the founder has already ruled on them — whether this pass ratifies those two classes
  too is the card's question for the operator, never the wrap's own call.
- The operator's invocation directive (inputs#I10): P1 only; the founder's word appended to `residuals.md:33` before
  `cites.py added` (done, Files); the scope ruling (Deviations 1).
- **For P5:** the BLOCKED-ON on escher-0.2.0/working-route.md:20 should read CLEARED —
  `evidence/founder-word.md` present, 9 rows, entry 26 green at the gated implement. A STANDING reading halts this
  wrap before P7.
- **For P7:** (a) the light gate runs the block with `--void 26:block cleared — …` (the gated implement's reason). Two
  forecasts of entries whose basis `34bcf71c` now reads the later chunks' edits, measured by a scratch script at this
  P1: the source-list probe (`git diff --name-only 34bcf71c… -- packages examples apps wpt scripts .github | grep -c -v -x -E …`,
  expect `exit 1` · `last line 0`) reads **13** on the work tree and 0 at `fb842671` — a red whose basis is that
  0 at the chunk's own tree, its owner for the operator to name at P7 (the ruling gives the 13 to two complete chunks;
  P7.1's arm wants a pin or a `--skip` with its reason); the census probe (`git diff -U0 34bcf71c… -- packages/blitz-dom/src
  packages/dioxus-native-dom/src/snapshot.rs | … grep -c …`) reads 0 on both; the manifests probe exits 0 on both. The
  static `#[test]` counts of escher-driver (74) and dioxus-native-dom (72) are unchanged from `fb842671` to HEAD
  (`git grep -c '#\[test\]'`), a forecast for their `passed` atoms. (b) The operator legs 29–31 are recorded for
  `48680dfb` (`evidence/operator-pass.md`); the final HEAD's run is the later chunk's CI#39111579925 on `87f12079`
  (Cross-project) — whether it re-verifies them here is P7's. (c) The coverage flip is plain (`gated → complete`),
  then the flip-compaction; Epoch 2 completes at this wrap (both its entries markered), so step 3b's consolidation
  fires.
- Pipeline gap (candidate, the evolve checkpoint's too): a gated take-up with later chunks committed on top of its
  flip — `gate.py scope`'s base rule reads those chunks' files unrecorded, `cites.py added --base {basis}` lists their
  text, and a plan's base-anchored probes read their edits; the letter has no rule for it, the operator ruled.
- Sweep hazards: `Epoch 2` matched escher-0.1.0's «Epoch 2 code audit» (test-plan:25) — an epoch label repeats across
  versions; the chunk title «Engine debts» fired on no master (they name the facts, not the entry); `PROVISIONAL` alone
  spans three batches, each mark classed by its clause.
- Standing learning recurred (host-linux «Paths & the working directory»): this window led its compound Bash calls with
  `cd /home/…/escher && …`; no guard refused one, the cwd already the root. The gated implement recorded the same.
- From the gated implement's outcome (`.andromeda/runs/2026-10-18T20-41-41-implement/outcome-2026-10-18-engine-debts.md`):
  entry 26 alone on the operator's directive; an evolve record citing a non-existent `scope-{marker}.json` trail,
  retracted by clause (gate.py writes every verb's record into `gate-{marker}.json`); the code checkpoint appended
  after P2's entry fired.

## Outcome
- Acceptance criteria, against the diff (the gated part: the evidence file, the inputs, the ledger, the residual line):
  - (v020-03) **met, pending P7's light gate** — the founder's own word discharges the batch: `evidence/founder-word.md`
    holds 9 rows, one per fork, each `ratified by the founder | 2026-10-18` (inputs#I9); every outcome matches its
    answer (report.md Outcome: seven fixed, each proven in both layout modes and red with its fix reverted; F3 and F9
    pinned; none kept); the gated criterion (entry 26) green at the gated implement.
  - every other criterion: met at the take-up (report.md Outcome), unchanged by the gated part, which touched no
    source; P7's light gate re-asserts them on HEAD's tree, the forecasts above beside it.
- Gates (the gated part; implement's trail `.andromeda/runs/2026-10-18T20-41-41-implement/gate-2026-10-18-engine-debts.json`):
  - `grep -c -E '^\| F[1-9] \| [AB] \| ratified by the founder \| 2026-[0-9]{2}-[0-9]{2} \|$' escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/founder-word.md`
    — green · exit 0 · `last line '9'` — `void: block cleared — the founder's own word «Утвердить всё (Рекомендую)» …
    (inputs#I9) recorded in evidence/founder-word.md`; summary `entries 31 · green 1 · red 0 · recorded 0 · timeout 0 ·
    not-run 30 · voided 1 (26)`.
  - the other 30 entries: not fired in the gated part; their last verdicts are the take-up's (report.md Outcome); P7.1
    re-runs the block.
  - smoke: skipped — no boot path or UI surface changed.
- Watches: none folded.
- Outcome basis: no operator pass in the gated part (no `chore(2026-10-18-engine-debts): operator pre-CI commit` in
  `fb842671..HEAD`); the gated implement's outcome file, `.andromeda/runs/2026-10-18T20-41-41-implement/`, read whole —
  of the three implement run dirs `fb842671` does not hold (`git ls-tree -d --name-only fb842671 .andromeda/runs/`), the
  one holding this marker's `outcome-`/`gate-` files; `2026-10-18T06-16-11-implement` and `2026-10-18T07-55-10-implement`
  hold only the two later chunks' (`outcome-2026-10-18-upstream-sync-determinism-and-traces.md`,
  `outcome-2026-10-18-one-virtual-clock.md`).
- Process hygiene: the gated implement's census — `census: none running` (escher-session · seven_guis_native · cargo);
  re-measured at this P1 by `gate.py census --comm escher-session --comm seven_guis_native --comm cargo`: `census: none
  running`. This wrap's own code-graph refresh (Setup 7) runs in the background.
## Addendum — resumed 2026-10-18
- inputs#I11 · `.andromeda/runs/2026-10-18T20-46-02-wrap/relay-2.md` · copy message · n/a — the operator's resume
  directive, 2026-10-18 21:12Z: (a) the new-text listing from HEAD stands, P7.0 re-runs it with `--base HEAD`; (b) P2
  discharges all 57 PROVISIONAL lines now, on the founder's word inputs#I9, all three batches; (c) P7 `--skip`s the
  plan.md:377 entry, reason «its 13 are the later chunks' (I10); 0 at fb842671»; legs 29–31 stand on `48680dfb`, with
  CI#39116069520 green on HEAD `1c9bc58b`; commit and stop; `target/` cargo-cleaned at 21:11Z, the first build cold.
  - bears on **Decisions & corrections**: supersedes «whether this pass ratifies those two classes too is the card's
    question for the operator, never the wrap's own call» — the operator's word: all three classes, the 57 `prov` rows
    (71 marks), discharged at this P2 on inputs#I9.
  - bears on **Cross-project / external claims**: adds to «CI: no run measured the gated part» — CI#39116069520, green,
    on HEAD `1c9bc58b` (the operator's reading, inputs#I11).
  - bears on **Deviations from intent**: adds to item 2 — the `--base HEAD` listing stands by the operator's word; P7.0
    re-fires it so.
  - bears on **Outcome**: adds to the forecast of the source-list probe (Decisions & corrections, For P7 (a)) — its
    treatment is `--skip` with the reason above, by the operator's word; legs 29–31 re-verify on `48680dfb`'s recorded
    run (`evidence/operator-pass.md`), CI#39116069520 the final HEAD's.
- inputs.py verify, re-fired: `inputs: 11 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 10 · uncited 1 · unparsed 0`; moved since the report: entries 10 → 11 (I11, snapped at this resume, cited here); I8 · I9 · I10 uncited → cited.
- gate.py scope, re-fired: `scope: unrecorded 21 files · changed 43 · listed 22 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 451` — the same 21 files, answered by the ruling (Deviations 1); excluded 438 → 451, this wrap's own run-dir artifacts.
- cites.py added `--base HEAD`, re-fired: `new text: 0 file(s) · 0 new · 0 added range(s) · 0 line(s) · 0 block(s)` · unchanged.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 1c9bc58b (given) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

(no file outside the pipeline's own artifacts changed)
