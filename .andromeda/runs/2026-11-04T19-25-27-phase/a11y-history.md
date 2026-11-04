@history_item n=1 marker=amd-xbggus4n
**amd-xbggus4n** — (2026-10-06-accessibility-tree-identity) made every element node of a Dioxus document carry its stable element id as AccessKit `author_id`, with no `TextRun`, document-root or `Window` node carrying one. It also had the shell build and refresh the tree through `Document::accessibility_tree`, so DioxusDocument's override reaches it. The probe's node → component reading (link 1) must go through that id unchanged, and any probe hook in `packages/dioxus-native-dom/` must not bypass the trait method.

@history_item n=2 marker=amd-6b7at2f2
**amd-6b7at2f2** — (2026-10-06-snapshot-model) made `DioxusDocument::snapshot` a consumer of the tree that takes role, name and focus from `accessibility_tree`, with no second role or name mapping. This is the rule behind the anti-pattern: any role, name or focus the probe reads must come off that model.

@history_item n=3 marker=amd-gm6e2md2
**amd-gm6e2md2** — (2026-10-06-id-stability-across-code-edits) the founder's actionable-key rule: an element that is focusable, has an interactive role or carries a listener reads an author key. `DioxusDocument::unkeyed_actionable` is its check and reads the listener mark `data-dioxus-id`. This is the bridge-held "carries a listener" read that link (2) can name. Trap: never satisfy the check by adding a `tabindex` or a role, because that moves the pinned Tab order; key the element instead.

@history_item n=4 marker=amd-kmnqnl3n
**amd-kmnqnl3n** — (2026-10-14-stand-requirement-sweep) made `unkeyed_actionable` return nothing on every stand task. It also counted and pinned the elements that react to a click and state no role, and handed the "reacts to a click" snapshot field to a `.andromeda/residuals.md` line (target: next) on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)». Link (2)'s `click` verdict is the reading that residual waits on, and this count is its starting reading. No field is added.

@history_item n=5 marker=amd-tfpzv4vr
**amd-tfpzv4vr** — (2026-10-31-engine-debts, Cells by keyboard) made `cells-grid` one Tab stop: its arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, and Enter opens and commits the editor. The reacting count went from 676 to the measured 677 (`cells-grid`'s `onkeydown` with no role), which disproved the plan's forecast. The probe must leave both the scroll and the count unchanged, and a listener-map reading must match 677, not 676.

@history_item n=6 marker=amd-viiy3gd2
**amd-viiy3gd2** — (2026-10-31-engine-debts) recorded the MountedData hazard: a `MountedData` call in a task spawned from a key handler that writes a signal panics `RefCell already borrowed` (`events.rs:162`). The task is first polled inside `render_immediate`. This was measured headlessly; the windowed path was read, not run. The async form was tried, panicked and did not ship, so Cells calls its handles synchronously. P3 re-verifies this mechanism and these coordinates at HEAD, and the fix is the founder's fork.

@history_item n=7 marker=amd-bynk5woo
**amd-bynk5woo** — (2026-11-01-upstream-sync-ui-graph) re-pointed citations after the take into thirteen files, among them dioxus-native-dom's `dioxus_document.rs` and blitz-test-harness's `settle.rs`. The cited lines on the hazard's `poll` path have moved since the hazard was measured, so the probe must re-read the coordinates at HEAD rather than trust the 2026-10-31 digits.

@history_item n=8 marker=amd-lx245pnl
**amd-lx245pnl** — (2026-10-07-change-tracking-and-diff) made the tree refresh on change: `View::poll` takes the changed set and rebuilds when it is non-empty, through the trait method. It defined `changed_nodes`: in-document mutations, focus or checked changes and text input mark it; node creation, hover, active, scroll and layout do not. It made `Snapshot::diff` a reader of the model by stable id. Link (4)'s witness rests on this set and this diff, on the same `poll` where the hazard's task is first polled.

@history_item n=9 marker=amd-pitqbdo5
**amd-pitqbdo5** — (2026-10-07-driver-session) the founder ratified the refresh on change and the changed-set contract (2026-10-07), with the windowed witness still owed. Link (4) leans on a ratified contract, not a provisional one.

@history_item n=10 marker=amd-234wcfpy
**amd-234wcfpy** — (2026-10-07-settle-detection) a harness settle neither reads nor drains the changed set, and the set is non-empty on a freshly booted harness document. So a check that reads `has_changes()` as "this step changed something" drains first. This is the trap behind the drain-first constraint for any state → nodes reading.

@history_item n=11 marker=amd-gu7audla
**amd-gu7audla** — (2026-10-20-select-by-id) added a select's changed choice (but not a choice of the option already chosen) to the sites that mark the changed set. The probe must count this site when it tells handler-written state from other marks.

@history_item n=12 marker=amd-5i7sgsxv
**amd-5i7sgsxv** — (2026-10-31-select-popup) qualified the changed set: a single select's list opening or closing marks it. With a list standing, a page scroll or a move that closes it marks the select through `reconcile_select_list`, and a wheel elsewhere through `close_select_list`. A highlight inside the list marks nothing. The rule "a pointer move, hover or layout marks nothing" holds only with no list open.

@history_item n=13 marker=amd-micbdtwc
**amd-micbdtwc** — (2026-10-10-snapshot-states) made the builder read `aria-selected` / `aria-pressed` / `aria-invalid` (trimmed, compared in lower case) into `selected` / `toggled` / `invalid`, which the snapshot takes from the tree and never from the element. On CRUD, links (3) and (4) must read a row's selection as the tree's `selected`, not as the attribute.

@history_item n=14 marker=amd-ooiv2bsv
**amd-ooiv2bsv** — (2026-10-10-snapshot-states, stand states) made the CRUD list state `role="listbox"` with `role="option"` rows carrying `aria-selected`. No key, id or order moved, and the actionable-key check still returns nothing. This is the stand surface whose handler-written selection the probe follows on CRUD.

@history_item n=15 marker=amd-vkszvnno
**amd-vkszvnno** — (2026-10-13-stand-keyboard-harness) made a pressed Enter, and a released Space, on a focused button dispatch a Click, so the app's onclick runs and then `handle_click`. It also made a pointer click focus a button. A CRUD row click still clears focus. Link (2)'s `click` answer must cover the onclick that Enter, Space or a pointer run. Link (4) must tell these focus marks apart from the handler's state write.

@history_item n=16 marker=amd-wn2krbrb
**amd-wn2krbrb** — (2026-10-13-stand-keyboard-harness) gave the CRUD list `tabindex="0"` (the third Tab stop) and made Arrow/Home/End move the selection as a row click does. It also added `stand_keyboard`, driven through the harness `press` / `press_with`. The probe drives its keys through that same route and must leave the CRUD list keys unchanged.

@history_item n=17 marker=amd-l5tujq64
**amd-l5tujq64** — (2026-10-06-snapshot-state-fidelity) added the Tab / Shift+Tab key-press focus check through `press` / `press_with` (`stand_snapshot_state`) and made a password read `MASKED_VALUE`. The probe uses the same headless key route, and it must print or field no value the plan masks.

@history_item n=18 marker=amd-6s34luva
**amd-6s34luva** — (2026-10-06-compact-snapshot-serialization) masked a file input's value like a password's and made `Snapshot::to_text` a reader of the model under the `accessibility` feature. This adds to the mask rule and to the feature-gated readers the probe reads through.

@history_item n=19 marker=amd-5jjilgrg
**amd-5jjilgrg** — (2026-10-20-large-screen-in-parts) made `Snapshot::text_parts` the fifth feature-gated reader named by the crate doc. The probe gets the override, snapshot and `unkeyed_actionable` under the feature tests/blitz-tests already names, and adds no feature.

@history_item n=20 marker=amd-xjyihgm3
**amd-xjyihgm3** — (2026-10-07-act-by-id) made `escher-driver` name `accessibility` on `dioxus-native-dom` unconditionally, so seven_guis compiles the accessibility modules through that edge while its own manifest names none, and neither binary builds a platform adapter. Probe code in `examples/seven_guis` reaches the bridge's reads only through that edge, and adds no feature, crate or adapter.

@history_item n=21 marker=amd-oeeggqqq
**amd-oeeggqqq** — (2026-11-01-upstream-sync-ui-graph) re-read the stand graph on the latest take: no manifest or lock moved, and `accesskit_(xplat|winit)` reads 0 rows. This is the baseline the probe must keep. It adds no dependency, so the windowed and assistive-technology readings stay read, not run.

@history_item n=22 marker=amd-52q2xqfn
**amd-52q2xqfn** — (2026-10-07-audit-corrections) stated the stand's role and name tables once, as `controls` / `INPUT_NAMES` in `tests/blitz-tests/tests/common/mod.rs`: a stated expectation, never a second role or name mapping. A probe check in tests/blitz-tests reuses that module and adds no mapping of its own.

@history_item n=23 marker=amd-qdbjvf3q
**amd-qdbjvf3q** — (2026-10-17-upstream-sync-engine-debts) made the snapshot's `name` take the computed text of `aria-labelledby`, which outranks `<label>` and `aria-label`. Any name the probe reads must come off the tree, with no reading of its own.

@history_item n=24 marker=amd-5gpfmf6x
**amd-5gpfmf6x** — (2026-10-31-select-popup) recorded that no `expanded` node key exists: `NodeState` holds no such field and `NODE_FIELDS` reads thirteen. This was the operator's call. It is the count the probe's acceptance holds: no field for "reacts to a click" either.

@history_item n=25 marker=amd-6mjvzdfl
**amd-6mjvzdfl** — (2026-10-31-engine-debts) made the tree's focus fall back to the window (`u64::MAX`) when the focused node was not built. A probe reading focus from the tree around a key or click step must expect that fallback.

@history_item n=26 marker=amd-mfvsaxsu
**amd-mfvsaxsu** — (2026-10-25-cold-agent-bar) made the served pages Cells (`boot_cells`) and the load page (`boot_load`), both served by `escher-session` and neither a `LeanTask`. `unkeyed_actionable` reads empty on all six served tasks. `load-done` stays at `display: none` until the stylesheet lands after `LOAD_ANSWER_DELAY` (200 ms). The probe's served task is one of these, and a load-page run must wait out that delay before reading the click-written status.

@history_item n=27 marker=amd-rj476q4o
**amd-rj476q4o** — (2026-10-31-engine-debts) gave the load page and Cells stated Tab sequences (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`), keyboard rows and contrast rows that `stand_keyboard` and `stand_contrast` walk in both layout modes. This is the acceptance the probe must leave holding.

@history_item n=28 marker=amd-57nw4lid
**amd-57nw4lid** — (2026-10-14-stand-a11y-assertions) proved the refresh's two halves headlessly on the stand, made the `&dyn Document` tree equal to the document's (stable ids only), and named the list `People`. The windowed AT witness went to a residuals line. The probe's tree reads match the shell's, and the windowed path stays read, not run.

@history_item n=29 marker=amd-xsx7vpsv
**amd-xsx7vpsv** — (2026-10-05-ci-gate-legs) created the named `a11y` leg (`ci-leg.sh a11y`) on fork CI. A probe check joins it only if the plan says so.

@history_item n=30 marker=amd-jc3rb5c2
**amd-jc3rb5c2** — (2026-10-14-stand-a11y-assertions) widened the `a11y` leg to six files, with `stand_keyboard` and `stand_accessibility_ids` among them, on the founder's F4. These are the files the probe must leave green.

@history_item n=31 marker=amd-w2xcxdkj
**amd-w2xcxdkj** — (2026-10-24-a11y-re-read) made the `a11y` leg fourteen files, eight of them also in `stand-repeat`, on the founder's «Плюс проверки дерева (Рекомендую)», each joining file a witness, a tree reading or a recorded gap. A probe check added to the leg changes this membership and its pin.

@history_item n=32 marker=amd-3usxtcys
**amd-3usxtcys** — (2026-10-14-quality-gates) made every `stand_*.rs` file run three rounds in `stand-repeat`. A probe file with a `stand_` prefix in tests/blitz-tests joins that leg by its glob, so naming the file decides its legs.

@history_item n=33 marker=amd-yjpcnkps
**amd-yjpcnkps** — (2026-10-24-stand-requirement-sweep) made a non-`stand_` witness join `stand-repeat` only by the leg's named `--test` list, with its membership held by `stand_requirement_sweep`'s leg check. A probe check outside the prefix joins no leg unless it is named.

@history_item n=34 marker=amd-syc36kks
**amd-syc36kks** — (2026-10-26-codebase-health) found that the `stand-repeat` list's membership is held jointly by the leg check and `test_stand_repeat_leg_runs_three_rounds`. The leg check reads `ci-leg.sh` as one text, so a name dropped from one site alone stays green. Trap: if the probe touches leg membership, both sites must be read.

@history_item n=35 marker=amd-ugublhgj
**amd-ugublhgj** — (2026-10-31-engine-debts) set the leg's tally at 14 files and 112 tests (`stand_keyboard` 21), and the `stand-repeat` list at fifteen words. This is an intermediate count the later entries move.

@history_item n=36 marker=amd-k5ol2gxq
**amd-k5ol2gxq** — (2026-10-31-select-popup) moved the `a11y` leg to 14 files and 114 tests (`stand_keyboard` 23) and `stand-repeat` to seventeen named words. These are the current counts a probe check would move if it joined either leg.

@history_item n=37 marker=amd-stcx27ze
**amd-stcx27ze** — (2026-10-31-select-popup) is a precedent for checks that run in the workspace test leg and `stand-repeat` without joining the a11y leg's fourteen, with `stand_keyboard` carrying the walk there. It also corrected `stand_keyboard`'s count to 23.

@history_item n=38 marker=amd-tirvs3vi
**amd-tirvs3vi** — (2026-11-01-upstream-sync-ui-graph) kept the leg's tally at 14 · 114 · 0 · 0 after the last chunk. This is the baseline the probe's acceptance ("the fourteen files stay green") reads against.

@history_item n=39 marker=amd-alwkmzpz
**amd-alwkmzpz** — (2026-11-01-upstream-sync-ui-graph) is the latest precedent for a new tests/blitz-tests check (`key_press_after_container_lost_its_box`) that carries no `stand_` prefix and joins neither the `a11y` nor the `stand-repeat` leg, so neither tally moves. This is the default for a probe check unless the plan says otherwise.

@history_item n=40 marker=amd-e4iayplx
**amd-e4iayplx** — (2026-10-25-capability-statement) made `docs/capabilities.md` publish the accessibility reach: tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, the `a11y` leg's fourteen files and the named limits. Its pin reads none of these claims, so a probe check that joins the leg, or any change to the reach, must be edited into that document by hand. The probe brings no new criterion.

@history_item n=41 marker=amd-d24imbve
**amd-d24imbve** — (2026-10-06-telemetry-bootstrap) measured the obs log format: one text line per event on stderr behind the allowlist scrub, with `text`, `value`, `html` and `attrs` redacted. This is the redaction the probe must give nothing to judge.

@history_item n=42 marker=amd-w7awh6wf
**amd-w7awh6wf** — (2026-10-07-sink-target-allowlist) restated the sink as a target allowlist: safe fields for `blitz*` / `accesskit_xplat`, redaction for `escher_*` targets, and a record from any other target dropped whole. The probe adds no target, field or span, and logs no component name, handler or value.

@history_item n=43 marker=amd-ndv4yofc
**amd-ndv4yofc** — (2026-10-07-driver-command-spans) gave the sink a second record class, one line per closed span of an admitted target, behind the same allowlist. This is why the scope's «Nothing crosses» bars a new span along with a new log field.
