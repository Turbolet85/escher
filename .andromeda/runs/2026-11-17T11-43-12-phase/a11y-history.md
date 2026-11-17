@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — wired the `a11y` job "Accessibility (a11y) tests" (`bash .github/scripts/ci-leg.sh a11y`) into the slow tier behind the fast jobs, with no required status check. This is the origin of `decision/a11y-ci-leg` and `decision/a11y-ci-gate`. The sync must keep this leg green, and any `ci.yml` edit (the disk-step line) must leave the job's tier and `needs` alone.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (both entries) — the first upstream merge re-pointed 16 `file:line` citations into merged upstream files by the merge's measured line map, with no claim text changed. It also caught a root `Cargo.toml` citation that an earlier insert had left one line low. This is the precedent: the take re-points citations into upstream files it moves, and re-checks the root-manifest cites when the AnyRender lines change.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — set the Dioxus feature wiring: the workspace takes the Dioxus crates with `default-features = false`, so a crate gets `accessibility` only by naming it. It also routed the shell's build and refresh through `Document::accessibility_tree` and established `author_id` stable ids. These back `decision/dioxus-accessibility-feature-by-name`, `decision/accessibility-tree-through-document-method` and `contract/accessibility-tree/author-id`. Upstream's manifest moves (AnyRender 0.15) must not disturb the feature tables.

@history_item n=4 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the engine's changed set truthful: it is marked by mutation, focus/checked and input, and not by node creation, hover, active, scroll or layout. The shell rebuilds the tree on a non-empty drained set. This is the root of `decision/changed-set-marks` and `decision/a11y-tree-refresh-on-change`. Upstream `e5305a4c1` clamps scroll offsets during layout rounding, so it must not start marking the set from layout or scroll.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the shell's refresh on change and the changed-set contract as one item (2026-10-07). It also ratified the bridge's falsy-boolean clear. A merge resolution that bends either clause is an engine change against a ratified row, not a free conflict choice.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on `dioxus-native-dom` unconditionally, with no `[features]` table, so both seven_guis binaries compile the accessibility modules. Neither builds a platform adapter: `accesskit_xplat` and `accesskit_winit` are in neither graph. This is the origin of `decision/driver-requires-accessibility` and the adapter-absence probe that each later take re-reads (`absence/assistive-technology-reach`).

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — restated the §3 obs-format note: a target allowlist prints only safe fields for `blitz*` and `accesskit_xplat`, redacts `text`/`value`/`html`/`attrs` for `escher_*`, and drops any other target whole. The scope re-checks the sink allowlist against the taken delta's new `tracing`/`log` sites. A new upstream target lands in that drop, and the §3 absence note stays true only if the allowlist is unmoved.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — restated the §3 note as two record classes, events and closed spans, both behind the same allowlist (ratified 2026-10-09 per obs plan §6). This is the text the §3 NOT YET MEASURED overview holds. If the sync's sink re-check moves obs-plan §6, this note follows it.

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — the precedent for a take that changes engine a11y behaviour (`autofocus`: `== "true"` became `!= "false"`). The take recorded it as a §5 amendment with the a11y leg's counts re-read. It also re-read the adapter absence on the merged lock when the accesskit family moved. This take's AnyRender move is a lock change of the same kind and needs the same probe.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password is painted as bullets through blitz-paint in every renderer, and the driver's screenshot is a reader of the snapshot model's bounds and changes nothing. These back `decision/password-never-revealed`, `decision/screenshot-changes-nothing` and `contract/driver-screenshot/element-shot`. AnyRender 0.15 and the paint commits must keep the mask, and any moved screenshot reading is re-measured, never loosened.

@history_item n=11 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — took upstream's 28-name `is_bool_attr` predicate, retiring the bridge's own list and leaving `mutation_writer.rs` byte-identical to upstream's. This is the basis of `decision/falsy-boolean-attributes-removed`. It is the precedent that keeping upstream's code as written is preferred, and a take touching the mutation writer re-checks that identity.

@history_item n=12 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast`: the background is composited to the first opaque ancestor, else the root-else-`body` canvas over white, with a painted-pixel witness within 1 per channel. Upstream `c5940f038` paints propagated root/body backgrounds on the canvas only, and `2203de767` paints root SVG backgrounds, so both can move what the reader and its pixel witness read (`decision/stand-contrast-measured`, `decision/text-contrast-required-ratio`).

@history_item n=13 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (both entries) — Enter/Space activation dispatching through `handle_click`, a click focusing buttons, `disabled` read by presence, and `stand_keyboard`. These are the basis of `decision/keyboard-activation-enter-space`, `decision/click-focuses-buttons` and `decision/disabled-by-presence`. An upstream edit to `handle_click`'s arms or key dispatch must keep them; the scope notes such edits are re-applied by hand to the `click_*` functions.

@history_item n=14 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (both entries) — the stand accent `#3a5ce5` (white at 5.47), SC 1.4.3 met on every measured non-exempt row, and the a11y leg widened rather than adding a job, required check or repository setting. This is the basis of `decision/stand-accent-colour`, `decision/stand-contrast-asserted` and `decision/a11y-ci-leg`'s "chosen option". The disk-step line must widen nothing beyond a step.

@history_item n=15 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the stand's a11y files also run three times in the `stand-repeat` leg (`Stand checks repeated`) under the zero-failure budget. That job is one of the two disk reds the operator's directive names. This is the basis of `decision/a11y-test-leg` and `decision/stand-checks-repeated-three-rounds`, so a disk red there is an a11y-leg red to explain, not to waive.

@history_item n=16 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — the §5 click and range rows cite the private `click_button` and `press_range_input` functions after the complexity split. This is the same split the scope's freight `cry-hvl3d4gp` names: an upstream edit to a `handle_click` arm is re-applied by hand to its `click_*` function, and these citations follow it.

@history_item n=17 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — took upstream's six builder reads as upstream wrote them (`aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled`, `aria-level`). The three closed-set state reads and the trimmed `aria-label` stayed, one read per attribute. This is the basis of `decision/one-read-per-aria-attribute`, `decision/aria-labelledby-outranks-label`, `decision/aria-description-to-adapter` and the `contract/aria-property-reads/*` rows. A take touching the accessibility builder must not reintroduce upstream's dropped duplicate reads.

@history_item n=18 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the double click is read on the document's clock, the harness's `VirtualClock` (ratified 2026-10-18). This is the basis of `decision/double-click-on-document-clock` and part of the "clock invariant" the scope's conflict resolutions must keep.

@history_item n=19 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the toggle-per-role, `inert`/`disabled` and ratification entries) — added the toggle read per role and `role="switch"`, `disabled` narrowed to form controls, and `inert` taking focusability, hits and the tree, all ratified 2026-10-18. These are the basis of `decision/toggle-read-by-role`, `decision/focusable-elements`, `decision/inert-subtree-excluded` and `decision/inert-element-takes-no-hit`. These are "the accessibility state reads" and "`inert`" that the scope's resolutions must keep.

@history_item n=20 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — took upstream #1226's `:focus-within` for every document, PROVISIONAL until the epoch boundary. `:focus-visible` still never matches (`decision/focus-visible-never-matches`). This is the precedent: an upstream behaviour change for every document taken in a sync is recorded and marked until ratified.

@history_item n=21 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the previous "polish and ship" take recorded upstream's dropped-node hit skip in §6 hit-testing. It re-read the stand graph on the take (no package entering or leaving the lock, the adapter still absent). This is the direct precedent for this take's §1 adapter re-read and for hit-stop changes.

@history_item n=22 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` joined `stand-repeat`'s named `--test` list, with membership held by `stand_requirement_sweep`'s leg check. This is the basis of `decision/stand-checks-repeated-three-rounds`' "named a11y witnesses". Upstream's new test files must not disturb that list.

@history_item n=23 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — `ci-leg.sh a11y` runs fourteen files on one `cargo test` line, pinned one `assertIn` per target. This is the current shape of `decision/a11y-ci-leg`. The "a11y leg green" acceptance reads these fourteen and their tally.

@history_item n=24 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` `## Accessibility reach` publishes the tier-0 reach, the a11y leg's fourteen files and the named limits. Its pin reads none of these claims, so a reach or limit the take moves must be edited there by hand.

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served-pages entry) — right after a `scroll`, a pointerdown in the shown overlay scrollbar's band is a thumb drag that dispatches nothing. The platform-adapter probe read unchanged with the host serving six tasks. Upstream `e5305a4c1` (scroll offsets clamped at layout rounding) touches the scrolling surface this reading and the driver's `click` rest on.

@history_item n=26 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the trap: the leg check reads `ci-leg.sh` as one text by `contains`, so a name dropped from the stand-repeat list alone stays green. The list's membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`. Any `ci-leg.sh` or workflow edit (the disk step) must keep both holds.

@history_item n=27 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand-graph, citation, `:open`, CSS-accessor and IME entries) — the full shape of a recent take's a11y amendments. It re-read the stand graph on the take (manifests, lock and `deny.toml` unchanged, `accesskit_(xplat|winit)` 0 rows) and re-pointed the citations the take moved. Each upstream engine behaviour change for every document became a row on the founder's word on the take. These are the basis of `decision/open-pseudo-class-on-details-dialog`, `decision/script-style-through-inline-attribute` and `decision/ime-enabled-for-prefocused-input`.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lock re-read, a11y leg tally, `optgroup`/`multiple` select and focus-fixup entries) — re-read the stand graph on the lock's own `pulldown-cmark` move. It set the a11y leg tally to 14 files and 112 tests, the stand-repeat list to fifteen, and added the select model, `option_disabled` and the focus fixup. These are the select model and its readings the scope's resolutions must keep, and the tally the per-OS re-read starts from.

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the scope-entity, changed-set, colour, opacity/clip, long-list, legs and counts entries) — the select list is an engine overlay on every document, painted by `paint_scene` after the root's render, with engine-constant colours. It paints whole past `opacity` and clips, and the list scrolls its own `scroll_y`. A page scroll or a close marks the changed set. The leg reads 14 files and 114 tests, and stand-repeat reads seventeen words. Upstream's canvas/root-background and AnyRender paint changes touch `paint_scene` around the list's paint, and `e5305a4c1` touches the scroll that closes a list (`decision/single-select-engine-overlay-list`, `decision/select-list-colours-engine-constants`, `decision/changed-set-marks`).

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (all entries) — the take before last, and the closest precedent. It re-pointed citations, re-read the stand graph (0 adapter rows), and added `embolden` to the `fast` leg for `select_popup_painted`. It also left the panic fix's focus limit (`decision/focus-fixup-reads-inert-alone`, `deallocate_anonymous_block`, a merge surface the scope re-reads) and made `text_contrast` read `None` past a blend (`decision/contrast-unread-past-non-colour-paint`). Blending paints with no frame test, and the open list's order under a blend is read, not measured. AnyRender 0.15 and the canvas-background commits must keep the `embolden` witness and these paint readings.

@history_item n=31 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand-graph and probe-target entries) — re-read the adapter absence on a lock-only move, and showed that a new blitz-tests target joins neither the `a11y` nor the `stand-repeat` leg unless named. This is the precedent for upstream's new test files under the blitz-tests one-target rule.

@history_item n=32 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the hazard-closed, same-render-limit and owner-clause entries) — the buffered render in `dioxus_document.rs` closed the spawned-task `MountedData` panic (`decision/buffered-dioxus-render`), with an open same-render limit. The adapter-absence owner `res-llkgqygv` reads re-carried at 0.3.0. This is the merge surface the scope's freight `cry-mnnoitsu` keeps until the Dioxus 0.8 move, which is out of scope here.

@history_item n=33 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached-record entry) — the reached record is ungated on `handle_event`'s path, because the wasm build compiles dioxus-native-dom without `accessibility`. This is the basis of `contract/dioxus-document/reached-handlers` and `record-reached`, part of the dioxus-native-dom surfaces the scope keeps at escher's behaviour.

@history_item n=34 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows: 241 rows, each verified against the code. Since then a plan change names rows by handle (`+`/`~`/`−`). This take's amendments, such as a new upstream behaviour row or a moved rule, are written as row deltas in that form.

@history_item n=35 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (all three entries) — the last upstream take changed `~ decision/changed-set-marks` (a scroll that closes a list marks its select) and added `+ decision/empty-alt-image-decorative` and `+ decision/hidden-checkable-label-toggle`. Both new rows are upstream behaviours ratified on the founder's word on the take. This is the direct precedent for how the twelfth take records an upstream behaviour change, such as SVG external images or scroll clamping, as rows.
