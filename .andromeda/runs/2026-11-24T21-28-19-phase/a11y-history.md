@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — wired ci.yml's `a11y` job ("Accessibility (a11y) tests", `ci-leg.sh a11y`) into the slow tier with no required status check, the origin of decision/a11y-ci-gate and decision/a11y-ci-leg; a canvas-lint test that joins the a11y leg widens this existing job, never a new job or status check.

@history_item n=2 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the actionable-key rule (decision/actionable-key-rule) and `unkeyed_actionable`; trap: never satisfy it by adding a `tabindex` or role. Matters because a canvas finding names the canvas element by its stable id (Circle Drawer's canvas is keyed), never a command identity this chunk mints.

@history_item n=3 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — made the driver's `screenshot` one more reader of the snapshot model's bounds, changing nothing (no focus move, empty next diff); the canvas's circles and labels leave as shot pixels on this reader, and a canvas lint read must likewise change nothing.

@history_item n=4 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` (WCAG 2.x `contrast_ratio`, `required_ratio` 3.0 large at 24 px / 14 pt bold, else 4.5; background composited to the first opaque ancestor; `None` under opacity < 1, a filter or a background image) and `stand_contrast`; the one formula and threshold a canvas text-contrast candidate (text against the fill painted under it) must reuse (decision/wcag-contrast-thresholds, decision/text-contrast-required-ratio, decision/stand-contrast-measured).

@history_item n=5 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*` file runs three rounds in `stand-repeat` under the zero-failure budget (decision/stand-checks-repeated-three-rounds); a new `stand_` canvas-lint check lands there by its prefix and must be deterministic across three rounds and both layout modes.

@history_item n=6 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (`text_contrast`'s first non-test caller) — `Session::lint`'s `contrast` reading goes through `text_contrast`, reported below `required_ratio()`, exempt exactly where `enabled` reads `Some(false)`, `None` not measured, no second formula or threshold; the stand at boot reads no `contrast` finding. Precedent for any canvas contrast check: same reader, same exempt rule, no second threshold (decision/lint-contrast-single-formula, decision/contrast-exempt-disabled-controls).

@history_item n=7 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the contrast reading through the `lint` verb) — the lint reached every carrier as the `lint` verb on the founder's F1 «Новая команда lint», and a `contrast` finding is written `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only (contract/visual-lint-finding/content, contract/lint-contrast-finding/*). A canvas finding added to the `lint` answer must keep this shape; how it reaches the caller is the chunk's crossing fork.

@history_item n=8 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the visual lint's «PROVISIONAL: in process only» on the founder's word (the lint F1 «Новая команда lint»): precedent that the lint's carriage was ratified as a founder fork, so a new canvas carrier or defect-word set on it is likewise a fork per the 2026-10-10 field rule.

@history_item n=9 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg widened to fourteen files on the founder's P4 fork («Плюс проверки дерева»), each joining file a witness, tree reading or recorded gap; a canvas-lint test joining the leg changes that file count, which the capability statement's pin reads (decision/a11y-test-leg).

@history_item n=10 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes `## Accessibility reach` (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3 only, the AT reading unmeasured) and a change to the reach is written there by hand (decision/capability-statement-reach-by-hand, decision/accessibility-reach-tier-0); the scope names the capability statement if a carrier changes.

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the lint's option overlap) — the stand at boot read one lint finding, an `overlap` of the flight booker's select options (`stand_lint` `[0, 1, 0, 0]`), a driver defect for every document re-pinned as measured and owned by a residual; precedent and trap for "reads clean on the stand": a new geometric check must be run against the stand at boot (decision/stand-lint-clean-at-boot).

@history_item n=12 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat `--test` list's membership is held jointly by `stand_requirement_sweep`'s leg check (which reads `ci-leg.sh` whole by `contains`) and `test_stand_repeat_leg_runs_three_rounds`; trap if the chunk adds a named witness outside the `stand_` prefix to either leg.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — `overlaps` skips a closed select's options in every document via `Entry.in_closed_select`, reading the element and never the role; the stand at boot reads no finding of any kind, `[0, 0, 0, 0]` (decision/stand-lint-clean-at-boot). The canvas lint must keep this at-boot reading clean on the four lean tasks in both modes.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list) — the lint's exclusion widened to `Entry.in_select_box`, and `clip_region` takes an open row's region from `visible_region` alone, no `contain: paint` cut; precedent for how the lint defines a region and a clip, which the canvas overflow check's region (content box and pushed clips) must settle against.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — `text_contrast` reads `None` past a `mix-blend-mode` other than `normal` and the lint reads the same through the one reader (decision/contrast-unread-past-non-colour-paint, guard `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`); a canvas contrast candidate must answer nothing where non-colour paint intervenes, not guess a pair.

@history_item n=16 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — decision/capability-statement-reach-by-hand's why now reads that the capability statement's pin reads only the a11y leg's file count among the section's claims; adding a canvas-lint file to the a11y leg moves a pinned number, the rest edited by hand.

@history_item n=17 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — decision/wcag-contrast-thresholds gained the painted-pair reading of an open list's highlighted row; decision/lint-contrast-single-formula and decision/lint-contrast-shares-one-reader now read the highlight reader first then the text contrast, through the one formula; decision/highlighted-row-contrast-unread retired. Precedent for a painted-pair contrast reading (as a canvas fill under text would be) entering through the one formula, never a second threshold.

@history_item n=18 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — + decision/circle-drawer-canvas-pixels-only: Circle Drawer's drawing area is one keyed, clickable canvas node (role group, named Drawing area) whose circles and diameter labels reach no DOM, snapshot, tree, UI graph or lint; the stand the canvas lint must read clean, and the row a canvas finding would amend (it says «or lint»).

@history_item n=19 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — decision/circle-drawer-canvas-pixels-only ratified by the founder («Подтвердить (Recommended)»), state standing; any change to what leaves the canvas (a lint finding about its labels) amends a founder-ratified row.

@history_item n=20 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — + absence/keyboard-drag-and-drop in §1 A11y Scope Summary, the merge immediately before the canvas work; recorded so the scope's stated absences are read beside the canvas chunk's a11y n/a claims.

@history_item n=21 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (circle-drawer-canvas-pixels-only · readers-of-one-snapshot-model) — the canvas's circles and labels now also leave as the `canvas` verb's draw commands, no label's text among them, and the canvas read is no reader of the snapshot model; the chunk's checks run over these same draw commands, so a canvas finding must carry no glyph, text or font and the row's «reach no … lint» clause is what a canvas lint must amend.

@history_item n=22 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (snapshot-model-single-source) — decision/snapshot-model-single-source gained that the canvas read, reading one canvas element's draw commands, is no reader of the snapshot model; a canvas lint reading draw commands sits outside the snapshot model's readers, unlike the DOM lint.
