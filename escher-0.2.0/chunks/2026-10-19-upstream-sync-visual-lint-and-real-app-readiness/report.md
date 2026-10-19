# Report — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

**Chunk:** Upstream sync ahead of visual lint and real-app readiness. The take is upstream `38ca4e777` (#1226,
`:focus-within`) and `17de049b1` (#1228, the WPT pin), by no-commit cherry-picks. The merge base `23b9b26a9` is
unmoved, and the four Taffy-main commits are left out. CI and the audit are green, the per-OS tallies were re-read,
and the sink allowlist was re-checked.
**Date:** 2026-10-19
**Commits:** `79603d1ef` chore(2026-10-19-upstream-sync-visual-lint-and-real-app-readiness): operator pre-CI commit,
for the run this chunk's verdict reads. Its body carries `(cherry picked from commit 38ca4e777513607c7dcb659748380e7a650b018f)`
and `(cherry picked from commit 17de049b11095808e0d093be2a8f0520e4dcbc0e)`. It is the one commit since
`last_wrap` 2026-10-19T18:30:00Z (`git log --format='%h %s' 67f17b9d6..HEAD`).

## Changes (structured — detectors read this)
- **Files:** taken from upstream (`git diff --name-only 67f17b9d6 HEAD -- packages tests wpt`):
  - `packages/blitz-dom/src/document.rs`, `packages/blitz-dom/src/events/keyboard.rs`,
    `packages/blitz-dom/src/mutator.rs`, `packages/blitz-dom/src/node/node.rs`, `packages/blitz-dom/src/stylo.rs`;
  - `tests/blitz-tests/tests/all.rs` (+2 `mod` lines);
  - new `tests/blitz-tests/tests/focus_within.rs` (192 lines) and `tests/blitz-tests/tests/focus_redraw.rs` (120 lines);
  - `wpt/WPT_COMMIT`.

  `all.rs`, the two new files and `WPT_COMMIT` are byte-identical to upstream's at `17de049b1`
  (`git diff --quiet 17de049b1 -- …` exit 0, `evidence/take.md`). The chunk's evidence is under
  `escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/`: `before.md`, `take.md`,
  `lock.md`, `readings.md`, `sink.md`, `counts.md`, `ci.md`. No other code path moved (the preservation guard, green).
- **Symbols / APIs:** all upstream's text, as written; every signature is kept.
  - **`Node` gains three pub methods** (new-text listing, `node.rs`):
    - `add_focus_within` at 736-743 @738;
    - `remove_focus_within` at 745-751 @746;
    - `contains_focus` at 753-757 @754.

    They sit after `is_hovered` and before `focus`. The first two set or clear `ElementState::FOCUS_WITHIN` and call
    `mark_ancestors_dirty()`; `contains_focus` reads the bit.
  - **`BaseDocument::set_focus_to`** (`document.rs`) refuses a node not in the document: `return false` at the added
    range 1800-1803, after the same-node check and before the `#[cfg(feature = "tracing")]` `Focussed node` INFO line,
    which now stands at 1805-1806. It then moves `FOCUS_WITHIN` from the old focus's ancestor chain to the new one,
    the shared prefix staying (added 1824-1842), and calls `self.shell_provider.request_redraw()` (added 1845-1849).
  - **`clear_focus`** removes `FOCUS_WITHIN` from the cleared node's element ancestors and calls `request_redraw()`
    (added 1780-1784 · 1786).
  - **`clear_interaction_state_for_removed_node`** walks the removed focus's chain, clearing `FOCUS_WITHIN`: through
    `snapshot_node_and` for a node in the document, directly for one outside it (added 959-973).
  - **The Tab arm of `handle_key_or_input_event`** (`events/keyboard.rs`) loses `let old_focus = doc.focus_node_id;`
    and its own `if doc.focus_node_id != old_focus { doc.shell_provider.request_redraw(); }` (lines removed only).
    The redraw now comes from `set_focus_to`.
  - **`DocumentMutator::add_children_to_parent`** (`mutator.rs`, private) records the old `:focus-within` chain of a
    moved child that contains focus (added 734-739, 747-751). After reattachment it transfers the state, removing it
    from the old chain and adding it to the new one, or only removing it when focus left with the subtree (added
    810-836). It also gains `use style_dom::ElementState;` (added 18).
  - **`selectors::Element for BlitzNode::match_non_ts_pseudo_class`** (`stylo.rs`):
    `NonTSPseudoClass::FocusWithin` reads `self.element_state().contains(ElementState::FOCUS_WITHIN)` where it read
    `false` (added 477-479).
  - **Remaining callers:** every caller of `set_focus_to`, `clear_focus`, `clear_interaction_state_for_removed_node`
    and `add_children_to_parent` is kept unedited (research.md §Graph impact). `set_focus_to` has 13 production call
    sites and 8 test sites; none passes a node outside the document on the stand's paths.
  - The `FOCUS_WITHIN` writes go through `snapshot_node_and`, which marks the changed set only for
    `FOCUS | CHECKED`. A focus move therefore still marks exactly the two nodes
    (`changed_set_focus_move_marks_both_nodes_and_hover_marks_nothing ... ok`).
  - No port, socket, env var, crate or log site is added.
- **Crates / modules:** none added, removed or renamed. Two blitz-tests targets are added by auto-discovery:
  `focus_within` and `focus_redraw`.
- **Dependencies:** none. `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical to `67f17b9d6` (the
  preservation guard, exit 0). Taffy stays the registry `"0.15"` and the root manifest has no git dependency (the
  probes `grep -c -E 'DioxusLabs/taffy|git = ' Cargo.toml` → 0 and `grep -c -E '^taffy = \{ version = "0\.15",'
  Cargo.toml` → 1; `evidence/lock.md`).
- **Schema / config:** none. The `wpt/WPT_COMMIT` pin moves `cd1d8aa5bfebc15d1af2af230c7ff95f9e582a4c` →
  `9d8aceec0ddc7ffe46d8f5f3f42d27a0163d6cba` (upstream's own; the WPT workflows are upstream-only).
- **Spec-master edits:** none (the masters are byte-identical to HEAD; the pre-pass sweep's baseline check passed).
- **Retired:** none.
- **Counts / qualifiers moved:**
  - **The blitz-tests directory** (pattern `testfiles`, 1 standing: `test-plan.md:68` at @c2514, "136 `.rs` files,
    135 test targets and the unbuilt `all.rs`" — stale): now 138 `.rs` files and 137 test targets, the unbuilt
    `all.rs` beside them. +2 are `focus_within.rs` and `focus_redraw.rs`. Basis: `ls tests/blitz-tests/tests/*.rs |
    wc -l` → 138, and `absent` count 0 over the workspace log (gate entries `ls …` and the `Running tests/$n.rs` loop).
  - **The workspace tally** (pattern `wstally`, 2 standing: `test-plan.md:330` @c2265 and `test-plan.md:336` ×2
    @c17756,17774):
    - now `lines 199 passed 1045 failed 0 ignored 12`, against `lines 197 passed 1034 failed 0 ignored 12` before the
      take (`evidence/before.md`; the tally entry's awk over this run's `target/ci-logs/test.log`). +2 result lines and
      +11 passed: `focus_within` 7 and `focus_redraw` 4.
    - test-plan:336's last re-count reads `194 result lines, 1001 passed` (2026-10-18-one-virtual-clock). The two
      2026-10-19 chunks' 194 · 1001 → 197 · 1034 was never written there; that is stale by omission, beside this
      chunk's own +2 · +11.
    - test-plan:330 @c2265 is CI tally text: see the next bullet.
  - **Per-OS CI tallies** (pattern `perostally`, 1 standing: `test-plan.md:330` ×2, @c4 the label and @c2199 the last
    reading, CI#39111579925 — stale). Re-read on CI#39312031677 (`79603d1ef`, green 17/17):
    - linux `Test [default features]` 199 · 1045 · 0 · 12; macos and coverage 180 · 1042 · 0 · 12; windows
      180 · 1000 · 0 · 9; the a11y job 6 · 43 · 0 · 0; the stand repeat 156 · 624 · 0 · 21; iOS and Android build
      only.
    - Against the base's green run CI#39302945352 (197 · 1034 · 0 · 12; macos and coverage 178 · 1031; windows
      178 · 989; a11y 6 · 43; stand repeat 156 · 624): +2 · +11 on each of the four test jobs. Read per target,
      exactly `focus_within` (7) and `focus_redraw` (4) on every OS. The a11y job and the stand repeat are unmoved.
    - The 2026-10-19 chunks' moves since CI#39111579925, attributed per target, are in `evidence/ci.md`:
      - session-recording: +2 · +26 on linux, macos and coverage; +2 · +7 on windows; +6 · +24 on the stand repeat;
      - recorded-check-replay: +1 · +7 on linux, macos and coverage; +1 · +8 on windows; +3 · +12 on the stand repeat.

    Basis: entry 34's job totals, and per-target reads of the three runs' job logs, whose sums reconcile.
  - **The `run all` agent-run figure** (pattern `runall`, 4 standing, all no-change: `architecture.md:102`,
    `security-plan.md:130`, `security-plan.md:377`, `a11y-plan.md:147` are `…:558-575` / `:572-575` line-range
    citations matching `\b575\b`, not the figure): 575 → 586 passed, +11 the two upstream files. No master site states
    it.
  - **`:focus-within` matching** (pattern `focuswithin`, 2 standing, both stale):
    - `architecture.md:85` lists `:focus-within` among the pseudo-classes that "always match false";
    - `a11y-plan.md:155` says "`:focus-visible` and `:focus-within` never match".

    Now `:focus-within` matches the focused element and its DOM ancestors, while `:focus-visible` still never matches
    (`stylo.rs:477-479`). Its citation is pattern `stylocite`, 2 standing, both stale:
    - `architecture.md:85` (`stylo.rs:460-506`, the range now 460-508);
    - `a11y-plan.md:155` (`stylo.rs:476-478`, the three arms now at 476 · 477-479 · 480).
  - **A node outside the document is not focused** (pattern `notindoc`, 5 standing, all no-change):
    - `architecture.md:86`, `security-plan.md:132` and `a11y-plan.md:161` name `set_focus_to` as a button click's
      focus path, still true;
    - `architecture.md:113` (Dioxus components outside the document loader) and `test-plan.md:12` (a changed-set
      write outside the document) are other facts sharing the token.

    No master states the new refusal. It is a new [DOM semantics] fact (Expected amendments).
  - **A focus set or cleared requests a redraw** (pattern `focusredraw`, 2 standing):
    - `architecture.md:254` @c360 (a Dioxus channel then `request_redraw`) is another fact sharing the token:
      no-change;
    - `architecture.md:268` (§Service wiring: "redraws are requested through `ShellProvider::request_redraw` after
      mutations, resource loads, hover and device changes") is stale by omission: focus set and cleared now request
      one too, and the Tab arm's own request is gone. Its citation `packages/blitz-dom/src/events/keyboard.rs:62`
      moves −4 with the removed lines (the citation sweep's).
  - **The upstream sync line** (patterns `syncline`, 10 standing, and `taffymain`, 2 standing):
    - `architecture.md:278` (×6 syncline, ×2 taffymain) is stale: "upstream `main` last read at `fb8d6c15e`, 5 commits
      past it, … `fb8d6c15e` … taken alone". Now `main` is read at `17de049b1`, 7 past `23b9b26a9`, and `38ca4e777`
      and `17de049b1` are taken by no-commit cherry-picks onto `67f17b9d6`; the merge base `23b9b26a9` is unmoved.
      The four Taffy-main commits wait again (git Taffy `ea07c4c3` at `17de049b1:Cargo.toml:100`), on F1 «Cherry-pick
      two (Recommended)» — the operator as delegate, PROVISIONAL (inputs#I2).
    - `security-plan.md:250` (×3 syncline, ×3 taffymain) is the 2026-10-18 sync's lock record, true as dated. It
      gains a sibling for this sync (Expected amendments).
    - `security-plan.md:249`, `architecture.md:75`, `architecture.md:90`, `architecture.md:150` @c2427,
      `test-plan.md:10` @c86, `test-plan.md:27` @c12807 and `a11y-plan.md:47` @c126 say "merged at `23b9b26a9`",
      still true as the base is unmoved: no-change.
    - `test-plan.md:330` @c1186 (`275950dc0`) is a dated tally reading: no-change.
  - **The WPT pin** (pattern `wptpin`, 2 standing, both no-change):
    - `test-plan.md:59` names the pin `71b4d5f0…` of its own dated reading at 2026-10-10-scrolling-box-bounds-and-hit,
      true as dated;
    - `test-plan.md:226` names `./wpt/WPT_COMMIT` without a value.
  - **The silence census** (pattern `silence`, 1 standing: `obs-plan.md:70` @c9091) gains two upstream files at
    0 · 0 · 0 (env · print · subscriber): `focus_within.rs`, `focus_redraw.rs` (research.md §Sweeps; `evidence/sink.md`).
  - **The `Focussed node` INFO line's site** (pattern `focusinfo`, 6 standing):
    - `obs-plan.md:197` cites `packages/blitz-dom/src/document.rs:1780-1781`, which is now `:1805-1806` — the
      citation sweep's, same claim;
    - `security-plan.md:407` @c5532, `test-plan.md:50` @c2554, `test-plan.md:105` ×2 @c2321,3972, `test-plan.md:329`
      @c1946 and `obs-plan.md:305` @c3937 state that the line prints twice on a flow. That is unmoved: the new gate
      fires only for a node outside the document, and `host_log` and `mcp_log` are green under both builds.
      No-change.
  - **The CARRY owner** (pattern `owner-title`, 2 standing; `owner-generic`, 2 standing): `architecture.md:90` and
    `architecture.md:91` name «Upstream sync ahead of visual lint and real-app readiness» as the owner of the
    engine-debts merge surface and of the clock's merge surface. Both CARRYs move to «Upstream sync ahead of polish
    and ship» (inputs#I3, card rows 3–4), so both lines are stale on their owner. `security-plan.md:125` @c9272
    ("owned by a route entry", the element id's future command) is another fact: no-change.
  - Sweep reach: 15 patterns · 13 moved facts, 13 with a pattern (the report's own count) · owner pattern 2 standing ·
    by line 4 lines · 4 offsets. The patterns are in `.andromeda/runs/2026-10-19T19-54-24-wrap/report-patterns.toml`;
    the trail is `cascade-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness-pre-pass.json`. `notindoc` was
    re-fired alone with `set_focus_to|clear_focus` added; its figure above is the re-fire's.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none changed. The agent-run contract, CI legs and `test_blitz_tests_targets.py` are
  unchanged: the two new files need no manifest line, `all.rs` stays `test = false`, and `BlitzTestsTargetsTest`
  stays green inside `fast`.
- **Cross-project / external claims:**
  - upstream DioxusLabs/blitz `main` = `17de049b1`, read at take-up (inputs#I1), re-read at implement Setup as
    `upstream/main` `17de049b11095808e0d093be2a8f0520e4dcbc0e` (`evidence/before.md`); crates.io's newest `taffy`
    0.15.0 (inputs#I1, not re-read).
  - CI: CI#39312031677 measured `79603d1ef633`, conclusion success, 17/17 checks, wall 1005 s. Its coverage job reads
    `coverage floor: 3 of 3 crates at or above their floors`. The base's run CI#39302945352 measured `67f17b9d6`,
    green. Both are recorded in `evidence/ci.md`.
  - `inputs.py verify`: `inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 3 · uncited 1 · unparsed 0`. The uncited entry was I3, cited here.
    - `I1 · message: the orchestrator, git and crates.io measurement on the dev host, 2026-10-19T18:51Z · copy · n/a`
      (cited scope.md, research.md, plan.md).
    - `I2 · message: the operator, P4 question dialog in this /andromeda-phase session, 2026-10-19T19:05Z · copy · n/a`
      — F1 answered «Cherry-pick two (Recommended)», the operator as delegate, PROVISIONAL.
    - `I3 · message: the operator, P5 review in this /andromeda-phase session, 2026-10-19T19:16Z · copy · n/a` —
      inputs#I3. The card is `.andromeda/runs/2026-10-19T18-50-26-phase/p5-card-1.md`, approved with «yes», the
      operator's own word (their own word at the review). Its `## Beyond the plan` rows, verbatim:

      | # | Proposal | Owner / place |
      |---|---|---|
      | 1 | CARRY 1 (the four Taffy-main commits, left out again because they put git Taffy back in the root manifest) moves forward. | «Upstream sync ahead of polish and ship» (`escher-0.2.0/working-route.md`, Epoch 5's first entry), by the wrap's route-resolve |
      | 2 | CARRY 2 (`keyboard_compat.rs`, not retired at `17de049b1`, both coverage exclusions standing) moves forward. | «Upstream sync ahead of polish and ship», by the wrap's route-resolve |
      | 3 | CARRY 3 (the engine-debts merge surface, re-read per row at each sync, no upstream pull request) moves forward with this chunk's re-read cited. | «Upstream sync ahead of polish and ship», by the wrap's route-resolve |
      | 4 | CARRY 4 (the document clock's merge surface, re-read at each sync) moves forward with this chunk's re-read cited. | «Upstream sync ahead of polish and ship», by the wrap's route-resolve |
      | 5 | The take's PROVISIONAL mark: upstream's every-document `:focus-within` / not-in-document focus / focus-move redraw, taken on a delegate's answer. It stands until the founder's own word. | Discharged in the Epoch 4 boundary batch (playbook «Provisional discharge»), listed by the wrap that closes Epoch 4 |

      Authority: the operator's own word at P5 review on the plan and these rows. The take itself rests on F1's
      delegate answer (inputs#I2), so it stays PROVISIONAL per row 5.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none in a master. Two plan and research claims are disproved:
  - **mutator.rs line cite.** research.md:11 and plan.md:421 place our changed-set `if` at `mutator.rs:731-733`. The
    base reads it at `HEAD:mutator.rs:730-732` (`git show 67f17b9d6:packages/blitz-dom/src/mutator.rs`, lines
    729-732 read). It now stands at 731-733 (`sed -n 730,742p`). Recorded in `evidence/take.md` and
    `evidence/readings.md`. research.md and plan.md stay as written. No master site — a chunk-folder fact.
  - **research.md §Open questions** ("Do upstream's four `focus_redraw.rs` counts hold on our engine?") is answered:
    yes. All four exact counts held on the first run (`test result: ok. 4 passed`). The open case closes with no edit.
- **Expected amendments (from plan):** each disposition names its search, the pre-pass sweep, with standing hits per
  master. `.andromeda/registries/` was read with the masters and held no hit for any pattern.
  1. architecture §Project Intent `Upstream sync:` — **carried** by Counts › The upstream sync line. Search:
     `syncline` architecture 4 (75, 90, 150, 278) · `taffymain` architecture 1 (278). The site is `architecture.md:278`.
  2. architecture [Unsupported features] (`architecture.md:85`) — **carried** by Counts › `:focus-within` matching.
     Search: `focuswithin` architecture 1 · `stylocite` architecture 1, both at :85.
  3. architecture [DOM semantics] — **carried** by Symbols / APIs (`set_focus_to`'s refusal and redraw, `clear_focus`'s
     redraw) and Counts › A node outside the document / A focus set or cleared requests a redraw. Search: `notindoc`
     architecture 3 (86, 113 no-change), `focusredraw` architecture 2 (254 no-change, 268 stale by omission). The
     entry `architecture.md:98` holds neither fact: new text. PROVISIONAL (inputs#I2; card row 5).
  4. a11y-plan §5 Keyboard Navigation (Focus order) (`a11y-plan.md:155`) — **carried** by Counts › `:focus-within`
     matching. Search: `focuswithin` a11y-plan 1 · `stylocite` a11y-plan 1, both at :155.
  5. test-plan §1 and §2 (`focus_within` 7, `focus_redraw` 4, each its own target; 138 files / 137 targets) and §9
     per-OS tallies — **carried** by Counts › The blitz-tests directory / The workspace tally / Per-OS CI tallies.
     Search: `testfiles` test-plan 1 (:68); `wstally` test-plan 2 (:330, :336); `perostally` test-plan 1 (:330);
     `focuswithin` test-plan 0 (no master names the two files — §1's test scope gains them as new text).
  6. obs-plan §3 Logging stack, the silence census — **carried** by Counts › The silence census. Search: `silence`
     obs-plan 1 (:70).
  7. security-plan §Dependency Security (Pinning), the per-sync lock record — **carried** by Dependencies and Counts ›
     The upstream sync line. Search: `taffymain` security-plan 1 (:250) · `syncline` security-plan 2 (:249, :250).
     A new sibling of :250 for this sync: a partial take, no dependency moved, the Taffy commits out (`evidence/lock.md`).
  8. `file:line` citations into the five touched blitz-dom files, re-pointed in every master that cites them —
     **carried** by the take's line map in `evidence/readings.md` (old → new per file). No master site — P2's citation
     sweep (`cites.py apply`) owns the re-pointing; `obs-plan.md:197` (`document.rs:1780-1781`) and `architecture.md:268`
     (`keyboard.rs:62`) are two such sites the sweep above met.
- **Coverage of new surfaces** (each is upstream's engine behaviour in every document; no surface of escher's own is
  added):
  - `:focus-within` matching (`stylo.rs:477-479`, the `FOCUS_WITHIN` chain in `document.rs`/`mutator.rs`/`node.rs`) →
    validation n/a · instrumentation n/a (no log site; the fixed-sha census reads 0) · PII n/a · tests integ
    (`focus_within` 6 of 7) · a11y n/a (no stand style uses it; `stand_keyboard`, `stand_contrast` and the a11y leg
    are unmoved at 6 · 43) · tokens n/a.
  - The not-in-document focus refusal (`document.rs:1800-1803`) → validation n/a · instrumentation n/a (the INFO line is
    unmoved, after the gate) · PII n/a · tests integ (`focus_within::a_node_outside_the_document_cannot_be_focused`) ·
    a11y: focus order unmoved (`stand_keyboard` 14, `inert_subtree` 4 green) · tokens n/a.
  - The focus-move redraw (`set_focus_to`, `clear_focus`) → validation n/a · instrumentation n/a · PII n/a · tests integ
    (`focus_redraw` 4, on a counting `RecordingShell`) · a11y: unrunnable-here for the windowed half (the smoke cannot
    see a focus ring; headless documents set no shell provider, so the redraw is the default no-op) · tokens n/a.

## Deviations from intent
- **Code: none.** The `mutator.rs` resolution is the plan's line for line: our changed-set `if` first, then upstream's
  blank line, three comment lines, `focus_within_old_chain` declaration and blank line. No gate-driven edit was
  made: the block was green on its first firing.
- **Citation off by one:** research and the plan cite our `if` at `mutator.rs:731-733`; it was `730-732` at the base
  and is `731-733` now (Spec claims disproved). Recorded, not edited.
- **The pre-CI commit carried the whole tree** (`git add -A`: the take, evidence, the phase and implement run dirs,
  the phase's route and master edits, the friction log). This matches the precedent pre-CI commit `d2f010a2a`'s shape
  and entry 32's clean-tree guard (`git diff --quiet && git diff --cached --quiet`). `evidence/ci.md` was written
  after it and is uncommitted.
- **The per-target CI reads** behind `evidence/ci.md`'s attribution ran outside the gate tool, through a scratchpad
  script over `gh run view {id} --log`. Entry 34 sums per job, not per target, and the 2026-10-19 chunks recorded
  no per-job tallies. Their sums reconcile with entry 34's totals on all three runs.
- Scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 9 · listed 9 · recorded 0 (companion
  0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 59`).

## Decisions & corrections
- **The operator's word for the operator pass** (this session, their own word): «Run the operator pass for
  2026-10-19-upstream-sync-visual-lint-and-real-app-readiness now, each entry in plan order: 31 hygiene; the commit
  with the subject exactly "chore(…): operator pre-CI commit, for the run this chunk's verdict reads" and both
  (cherry picked from commit …) trailers in its body; 32 the push; 33 ci.py conclusion; 34 the per-OS tallies. Record
  it in evidence/ci.md. Stop and tell me if any entry is red.» Every entry ran in order and none read red
  (`evidence/ci.md`).
- **The wrap's invocation word** (this session, the operator): «run P1 only and stop». This wrap is in the two-window
  form; P2–P7 resume from `.andromeda/runs/2026-10-19T19-54-24-wrap/`.
- **Guard refusal, recurring despite a standing learning:** the Bash guard refused a `cd` twice this chunk. At
  implement Setup it was a `cd` into the skill's references dir; at wrap P1 a `cd` into the chunk's `inputs/` dir.
  The refusal reads "moves the session cwd for every later call". Each was re-issued once with absolute paths and
  ran. The standing rule is host-linux.md §Paths ("`cd` only inside a subshell `( cd X && … )` (setup's Bash guard
  denies a `cd` that would move the cwd)").
- **Sweep hazard:** a count pattern `\b575\b` for an agent-run figure matched four line-range citations
  (`node.rs:558-575`, `net.rs:572-575`) and none of the figure's sites. A bare number is read inside a citation's
  digits; the rows had to be read to disposition all four as no-change.
- **Sweep hazard:** the name token for the not-in-document refusal is `set_focus_to`. A wording-only pattern ("not
  in the document | outside the document") found 2 unrelated rows and missed the three `set_focus_to` sites;
  re-fired with the token, it read 5.
- **Observation — tallies behind by two chunks:** test-plan.md:330 (per-OS) and test-plan.md:336 (local baseline)
  stopped at 2026-10-18-one-virtual-clock (CI#39111579925; 194 · 1001). The two 2026-10-19 chunks' operator passes
  read CI green without re-reading per-job tallies, and no wrap wrote their counts. This chunk's `evidence/ci.md`
  reconstructs both chunks' moves per target from the three runs' job logs.
- **Research's open question closed by measurement:** upstream's exact redraw counts in `focus_redraw.rs` hold on our
  fork (`tabbing_to_the_next_node_requests_a_redraw ... ok`). No other redraw site of ours on the Tab path moves
  the count.
- **outcome file read** (`.andromeda/runs/2026-10-19T19-16-25-implement/outcome-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.md`):
  it holds the same guard refusal (implement Setup) and "No operator correction or ruling was given in this run".

## Outcome
**Acceptance criteria**, each re-asserted against the diff:
- (arch) **Exact take — met.**
  - The exact-take probe (`diff <(git diff … 67f17b9d6 -- packages tests wpt Cargo.toml Cargo.lock | grep -E
    '^[+-][^+-]') <(git show … 38ca4e777 17de049b1 | grep -E '^[+-][^+-]')`) is green, no output.
  - The `mutator.rs` resolution is named in `evidence/take.md` as the one non-additive edit; no further edit was made.
- (arch, security) **No git dependency, Taffy registry 0.15, manifest/lock/deny byte-identical — met.** This rests on
  the preservation guard and the two Taffy probes, all green; `evidence/lock.md` names the four Taffy commits and
  cites inputs#I2.
- (security) **Audit — met.** `bash .github/scripts/ci-leg.sh audit` reads `advisories ok, licenses ok`, with
  `deny.toml` unchanged; CI's Dependency audit job is green.
- (tests) **Upstream targets — met.**
  - `focus_within` 7 and `focus_redraw` 4 run as their own targets.
  - There are 0 `Running tests/all.rs` lines, and `BlitzTestsTargetsTest` is green inside `fast`.
  - The directory holds 138 files.
- (tests) **Fast and doc — met.** Both legs are green. The tally reads `failed 0`; its +2 lines and +11 passed (as
  predicted) are in `evidence/counts.md`.
- (a11y, layouts) **Focus and a11y checks — met.**
  - `stand_keyboard` 14, `inert_subtree` 4, `stand_accessibility_ids` 8, `interaction_state_teardown` 4 and
    `stale_interaction_state` 5 all pass.
  - The a11y leg reads `lines 6 passed 43 failed 0`.
  - `run stand` reads 178 · 0 · 7 over 44 files, unmoved, `stand_contrast` included.
- (arch, a11y) **Changed set — met.** `changed_set_focus_move_marks_both_nodes_and_hover_marks_nothing ... ok`.
- (obs, security) **Telemetry and sink — met.**
  - The fixed-sha census reads 0, and there is no wall clock outside `clock.rs` (0).
  - `host_log` 2 and `mcp_log` 2 pass under the per-package build (the seven_guis entry) and the workspace build
    (`fast`'s `test.log`).
  - `packages/escher-telemetry/` is unchanged.
  - All of it is in `evidence/sink.md`.
- (arch) **CARRY re-reads — met.** CARRY 1–4 are re-read per row in `evidence/readings.md`. F1–F8 each read untouched,
  or name the upstream hunk beside them. Nothing was sent upstream.
- (tests, CI) **Fork CI and per-OS tallies — met.** Fork CI is green on the pushed take, CI#39312031677
  (`verdict: green · checks 17/17`). The per-OS tallies were re-read job by job and explained by name against
  CI#39302945352, with the 2026-10-19 chunks' moves attributed beside (`evidence/ci.md`).
- **No 0.2.0 capability claimed — met.** `matrix.py show --chunk …` reads `claimed … 0`.

**Gates** (implement run `.andromeda/runs/2026-10-19T19-16-25-implement/`, one block firing: `entries 34 · green 29 ·
red 0 · recorded 1 · timeout 0 · not-run 4`; trail `gate-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`):

| entry (`run`) | verdict |
|---|---|
| the exact-take `diff <(git diff … ) <(git show … )` | green (exit 0, no output) |
| `git diff --quiet 67f17b9d6 -- Cargo.toml Cargo.lock deny.toml packages examples tests apps wpt .github scripts ':(exclude)…'` | green (exit 0) |
| `grep -c -E 'DioxusLabs/taffy\|git = ' Cargo.toml` | green (exit 1, 0) |
| `grep -c -E '^taffy = \{ version = "0\.15",' Cargo.toml` | green (exit 0, 1) |
| the `git show --format= 38ca4e777 -- '*.rs' \| grep -c …` census | green (exit 1, 0) |
| `grep -rlE 'Instant::now\(\)\|SystemTime::now\(\)' packages/blitz-dom/src \| grep -vc 'clock\.rs$'` | green (exit 1, 0) |
| `git ls-tree -r --name-only 17de049b1 -- …/keyboard_compat.rs \| wc -l` | green (exit 0, 1) |
| `grep -c -F -e "--ignore-filename-regex 'keyboard_compat\.rs\$'" .github/scripts/ci-leg.sh` | green (exit 0, 2) |
| `grep -c -F 'NonTSPseudoClass::FocusWithin => false' packages/blitz-dom/src/stylo.rs` | green (exit 1, 0) |
| `cargo test -p blitz-tests --locked --test focus_within 2>&1` | green (`7 passed; 0 failed`) |
| `cargo test -p blitz-tests --locked --test focus_redraw 2>&1` | green (`4 passed; 0 failed`) |
| `cargo test -p blitz-dom --locked --lib changed_set_tests 2>&1` | green |
| `cargo test -p blitz-tests --locked --test interaction_state_teardown --test stale_interaction_state --test inert_subtree --test stand_keyboard --test stand_accessibility_ids 2>&1` | green |
| `bash .github/scripts/ci-leg.sh fast` | green (artifact `test.log` fresh) |
| the workspace tally awk | green (`lines 199 passed 1045 failed 0 ignored 12`) |
| the `Running tests/$n.rs` loop | green (exit 1, 0 absent) |
| `ls tests/blitz-tests/tests/*.rs \| wc -l` | green (138) |
| `grep -E '^Ran [0-9]+ tests\|^OK$\|^FAILED' target/ci-logs/ci-scripts.log` | recorded (`Ran 100 tests in 8.679s` · `OK`, unmoved) |
| `bash .github/scripts/ci-leg.sh doc` | green |
| `bash .github/scripts/ci-leg.sh audit` | green (`advisories ok, licenses ok`) |
| `bash .github/scripts/ci-leg.sh a11y` | green |
| the a11y tally awk | green (`lines 6 passed 43 failed 0`) |
| `bash scripts/agent-run.sh boot` | green |
| `bash scripts/agent-run.sh run stand` | green (run.end 178 · 0 · 7) |
| `ls tests/blitz-tests/tests/stand_*.rs \| wc -l` | green (44) |
| `bash scripts/agent-run.sh run all` | green (run.end 586 · 0 · 12) |
| `cargo test -p seven_guis --locked 2>&1` | green (12 result lines, 33 passed) |
| `cargo tree --locked -p seven_guis -e normal --prefix none \| grep -c -E '^accesskit_(xplat\|winit) '` | green (exit 1, 0) |
| `cargo build -p seven_guis --bin seven_guis_native --locked` | green |
| `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124` | green (smoke; `WAYLAND_DISPLAY` set, `service.name=seven_guis` logged) |
| `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) | fired as written in the operator pass: `hygiene: clean — read 51 …` (`evidence/ci.md`) |
| `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.2.0` (leg operator) | green (exit 0, 152.67 s), `history moved: refs/remotes/origin/build/escher-0.2.0 67f17b9d→79603d1e` — the intended push (`evidence/ci.md`) |
| `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (leg operator) | fired as written: `79603d1ef633 verdict: green · checks 17/17 · wall 1005 s · runs CI#39312031677 completed/success` (`evidence/ci.md`) |
| `gh run view <id> -R Turbolet85/escher --log \| …` (leg operator, `--id 39312031677`) | recorded (exit 0): the six job tallies, each `failed 0` — linux 199 · 1045 · 0 · 12, macos and coverage 180 · 1042 · 0 · 12, windows 180 · 1000 · 0 · 9, a11y 6 · 43 · 0 · 0, stand repeat 156 · 624 · 0 · 21. Dispositioned: green, every move explained (`evidence/ci.md`) |

No `defer` entry and no deferral: Rust changed and every gate ran. Smoke: the plan's smoke entry ran as a P2 gate,
green; P3 did not re-drive it.

**Watches:** none — no `watch:` was folded (scope.md: no WATCH on the entry).

**Outcome basis:**
- The operator pass ran (`79603d1ef`, the one commit from `67f17b9d6`), and its final state is the basis.
  - `fast` was re-run green inside the push entry on the committed tree.
  - The final HEAD's CI run CI#39312031677 was green (17/17), recorded in `evidence/ci.md`.
  - `evidence/ci.md` was written after the commit and is the only uncommitted change.
- implement's P4 report is held in this window and persisted at
  `.andromeda/runs/2026-10-19T19-16-25-implement/outcome-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.md`,
  outcome green + smoke. The earlier implement dirs of the day belong to other markers.

**Process hygiene:** implement P4's census reads `census: none running` (`seven_guis_native` · `escher-session` ·
`cargo` · `agent-run.sh`). Re-measured at this wrap's P1 by `gate.py census --comm seven_guis_native --comm
escher-session --comm cargo --args agent-run.sh --run-dir .andromeda/runs/2026-10-19T19-54-24-wrap …`, it again reads
`census: none running`. Every process the runs started was terminated: the smoke's `seven_guis_native` by its
`timeout 10` bound, and the gate entries' cargo, test binaries, session hosts and agent-run runs on exit.

**For the later phases** (this wrap stops after P1):
- **P2:** the stale master sites above.
  - `architecture.md:85`, `:90`, `:91`, `:98` (new [DOM semantics] text), `:268` and `:278`;
  - `a11y-plan.md:155`;
  - `test-plan.md:68`, `:330` and `:336`, plus §1's test scope;
  - `obs-plan.md:70`;
  - `security-plan.md` (the new sibling of `:250`).

  Every behaviour fact is PROVISIONAL (inputs#I2).
- **P5:** carry the card rows 1–4 to «Upstream sync ahead of polish and ship», each citing this chunk's
  `evidence/readings.md`. List row 5 (the take's PROVISIONAL mark) for the Epoch 4 boundary batch.
- **P6:** no failed command is outstanding.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 67f17b9d (the parent of the oldest pre-CI commit 79603d1e) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/src/document.rs — added 49 line(s) in 6 range(s)
added: 959-973 · 1780-1784 · 1786 · 1800-1803 · 1824-1842 · 1845-1849
### packages/blitz-dom/src/events/keyboard.rs — nothing added (lines removed only)
### packages/blitz-dom/src/mutator.rs — added 39 line(s) in 4 range(s)
added: 18 · 734-739 · 747-751 · 810-836
### packages/blitz-dom/src/node/node.rs — added 23 line(s) in 1 range(s)
added: 736-758
  - 736-743 @738 «pub fn add_focus_within(&mut self) {»
  - 745-751 @746 «pub fn remove_focus_within(&mut self) {»
  - 753-757 @754 «pub fn contains_focus(&self) -> bool {»
### packages/blitz-dom/src/stylo.rs — added 3 line(s) in 1 range(s)
added: 477-479
### tests/blitz-tests/tests/all.rs — added 2 line(s) in 1 range(s)
added: 26-27
### tests/blitz-tests/tests/focus_redraw.rs — new file · 120 line(s)
- 9-12 «use blitz_traits::{»
- 14-17 «use std::sync::{»
- 19-22 @20 «struct RecordingShell {»
- 23-27 «impl ShellProvider for RecordingShell {»
  - 24-26 «fn request_redraw(&self) {»
- 28-32 «impl RecordingShell {»
  - 29-31 «fn take_redraws(&self) -> usize {»
- 34-38 «const HTML: &str = r#"»
  - 35-38 «<html><body>»
- 41-54 «fn make_doc() -> (HtmlDocument, Arc<RecordingShell>) {»
  - 43-51 «let mut doc = HtmlDocument::from_html(»
- 56-58 «fn node_id(doc: &HtmlDocument, selector: &str) -> NodeId {»
- 60-71 «fn tab_key_event() -> BlitzKeyEvent {»
  - 61-70 «BlitzKeyEvent {»
- 73-81 @74 «fn focusing_a_node_requests_a_redraw() {»
- 83-92 @84 «fn refocusing_the_same_node_requests_nothing() {»
- 94-107 @95 «fn blurring_requests_a_redraw() {»
- 109-120 @110 «fn tabbing_to_the_next_node_requests_a_redraw() {»
### tests/blitz-tests/tests/focus_within.rs — new file · 192 line(s)
- 16-25 «fn make_doc(html: &str) -> HtmlDocument {»
  - 17-24 «HtmlDocument::from_html(»
- 27-29 «fn node_id(doc: &HtmlDocument, selector: &str) -> NodeId {»
- 31-33 «fn matches(doc: &HtmlDocument, selector: &str) -> bool {»
- 35-66 @36 «fn focus_within_matches_the_ancestors_and_clears_on_blur() {»
  - 37-40 «let mut doc = make_doc(»
  - 46-52 «for selector in [»
- 68-83 @69 «fn focus_within_matches_a_display_contents_ancestor() {»
  - 70-73 «let mut doc = make_doc(»
  - 79-82 «assert!(»
- 85-97 @86 «fn focus_within_matches_when_focus_is_set_before_the_first_layout_pass() {»
  - 93-96 «assert!(»
- 99-117 @100 «fn removing_the_subtree_containing_the_focus_clears_the_ancestors() {»
  - 101-102 «let mut doc =»
- 119-146 @120 «fn moving_the_focused_subtree_takes_the_state_along() {»
  - 121-124 «let mut doc = make_doc(»
  - 138-141 «assert!(»
  - 142-145 «assert!(»
- 148-171 @149 «fn moving_the_focused_subtree_out_of_the_document_clears_the_state() {»
  - 150-151 «let mut doc =»
  - 161-164 «let detached = doc.mutate().create_element(»
- 173-192 @174 «fn a_node_outside_the_document_cannot_be_focused() {»
  - 175-178 «let mut doc = make_doc(»
### wpt/WPT_COMMIT — added 1 line(s) in 1 range(s)
added: 1
