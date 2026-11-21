# Outcome — 2026-11-21-engine-debts

**Outcome: green** (one fix iteration; one halt resolved by the operator's and the founder's word, `inputs#I5`, `inputs#I6`)
**Run:** 2026-11-21T10-50-19-implement

## Files changed
New (3): `tests/blitz-tests/tests/focus_fixup_unshown.rs` · `packages/blitz-vibey-script/src/seeded_random.rs` ·
`packages/blitz-vibey-script/tests/seeded_random.rs`.
Modified (16): `packages/blitz-dom/src/{node/node.rs,layout_reason.rs,document.rs,resolve.rs}` ·
`packages/blitz-vibey-script/src/{lib.rs,state.rs,runtime.rs,document.rs}` ·
`packages/dioxus-native-dom/src/{boundary.rs,dioxus_document.rs}` ·
`tests/blitz-tests/tests/{key_press_after_container_lost_its_box.rs,dioxus_runtime_probe.rs}` ·
`.github/scripts/{ci-leg.sh,test_ci_workflows.py,test_capability_statement.py}` · `docs/capabilities.md`.

## Deviations, with their justifications
1. **Step 5 (a)'s wait is a stored-waker future, not the plan's yield-once.** The plan's self-waking yield-once hung in
   both layout modes: dioxus-core 0.7.10 `render_immediate` (`virtual_dom.rs:608-614`) re-polls a task woken during the
   render inside that render, so the task spun there and the handle, made only after the replay, never came. A task
   that leaves its waker for the input's `onmounted` passes: the engine's, the snapshot's and the tree's focus all land
   on the input. The founder's F3 premise («the ordinary pattern works») holds, so the fork did not go back to him. The
   operator's word: «Stored-waker wait (Recommended)» (`inputs#I5`). The busy-yield livelock is stated in
   `docs/capabilities.md` as a limit asserted by no check.
2. **Tab and Shift+Tab skip a node that does not show** — an engine change for every document beyond the plan.
   With the fixup alone, focus given up sat on the root and Tab restarted from the top. `focus_next_node` and
   `focus_prev_node` read `is_focussable` alone, so Tab landed on the hidden control again, the next layout took focus
   away, and `#after` was never reached. That was a Tab trap, and the plan's step 4 «Its next Tab still lands on
   `#after`» did not hold. The founder's word: «Tab пропускает скрытое (Recommended)», relayed by the overseer
   (`inputs#I6`). Built as `Node::shows` (`node.rs:635-644`), read by both walks (`document.rs:2350-2352`,
   `2360-2362`) and by `give_up_unshown_focus`. One stated limit stays: a control whose own `:focus-within` rule hides
   its container still takes Tab and loses it at the next layout (pinned in `key_press_after_container_lost_its_box`).
   Checked by a new test, `tab_and_shift_tab_skip_a_control_that_does_not_show`, and controlled by two more one-shot
   controls (`tab-next-unshown`, `tab-prev-unshown`), beside the plan's four.
3. **Step 4 re-aimed a third test** the step did not name: `a_key_on_a_control_in_no_anonymous_block_reads_the_same`
   also pressed a key on a control hidden after Tab. It takes the same re-focus with `set_focus_to`.
4. **Step 8's pinned registry length is 3, not the predicted 2.** The prune runs before each render, so the boundary the
   last render dropped stands until the next: the root's, the dropped one and the live one. Pinned as measured
   (`REMOUNTED_LEN`).
5. **`BoundaryRegistry`'s entry is a type alias, `RegisteredBoundary`**: clippy's `type_complexity` reddened `fast`.
6. **The scope record** — `gate.py scope`: `scope: clean — changed 19 · listed 18 · recorded 1 (companion 1 · mechanical 0
   · in-intent 0 · widening 0)`:
   - `` `.github/scripts/test_capability_statement.py` · companion · serves step 9 · self `` — it pins the `a11y` leg's
     file count as the literal 18 (`assertEqual(files, 18)` → 19). Research's sweep had called it unedited.

## Gates (final firings, `gate-2026-11-21-engine-debts.json`)
1 lib units green · 2 script tests green · 3 the step-3/4/5 targets green · 4 the companions green · 5
`test_ci_workflows.py` green · 6 `test_capability_statement.py` green (red at firing 1: `AssertionError: 19 != 18`, fixed)
· 7 `ci-leg.sh a11y` green (19 · 177 · 0 · 0) · 8 `ci-leg.sh fast` green (red at firing 1: clippy `type_complexity`
on `boundary.rs:49`, fixed) · 9 `ci-leg.sh doc` green · 10 `ci-leg.sh stand-repeat` green, last line `stand-repeat: 3 of
3 rounds green` · 11–13 `leg operator`, not run (the operator pass). No `gate.py guard` entry. No deferral.

Controls (`{work_dir}/evidence/control-1.txt`): `control: 6 holds · 0 timeout · 0 does not hold · 0 unreproduced ·
clone removed`.

Smoke: skipped. There is no boot-path change, the plan lists no `smoke` or `self-verify` entry, and the project has no
headful self-verify target. The engine change is proven headless by `stand-repeat`.

Matrix: v040-06 planned → implemented.

## Surfaced / stuck
None outstanding. The halt raised two questions in one dialog; both were answered (deviations 1 and 2).

## Process hygiene
| process | started by | final state |
|---|---|---|
| `cargo test` of the gate entries and the control firings | this run (gate.py run, gate.py control) | terminated |
| the hand `cargo test` probes (the hung step-5 (a) run stopped by TaskStop; the trial; the measured literals) | this run | terminated |
`gate.py census --comm cargo --comm seven_guis_nati --comm escher-session --args blitz-tests`: `census: none running`.

## Decisions & corrections
- The operator: pin (a) is written with a stored-waker wait, and the busy-yield hang is stated as a limit — «Stored-waker wait (Recommended)» (`inputs#I5`).
- The founder, relayed by the overseer: Tab and Shift+Tab skip what does not show — «Tab пропускает скрытое (Recommended)» (`inputs#I6`).
- A test-writing hazard: after the post-layout fixup, an assertion that a hidden control is not focused holds in both
  worlds, because the fixup takes focus off it at the next pump. The Shift+Tab check pins the node focus lands on instead.
- A dioxus-core hazard: a task that wakes its own waker while it waits livelocks inside `render_immediate`. A test task
  that waits for a later render leaves its waker for the producer.
- Guard refusals: a `cd` out of the cwd (re-issued with `cd` into the project root) and a `cat` heredoc with a file
  target (re-issued as a scratchpad script). `gate.py control` refused a control id of 17 characters (the limit is 16).
