@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created ci.yml's `a11y` job "Accessibility (a11y) tests" running `bash .github/scripts/ci-leg.sh a11y` in the slow tier behind the fast jobs, restoring the test job's cache and saving none; origin of decision/a11y-ci-leg, decision/a11y-test-leg and decision/a11y-ci-gate. This chunk edits `ci.yml`, `ci-leg.sh` and `test_ci_workflows.py` (and may add a skia fetch step), so the job's tier, `needs`, restore-only cache and leg command must survive the edits.

@history_item n=2 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — wired the Dioxus `accessibility` feature (dioxus-native-dom forwards to blitz-dom, dioxus-native to blitz-dom, blitz-shell and dioxus-native-dom), the workspace taking the Dioxus crates with `default-features = false` so a crate gets the feature only by naming it; recorded that the stand binary builds no platform adapter. Basis of decision/dioxus-accessibility-feature-by-name and decision/stand-builds-no-platform-adapter; a manifest or lock move here must not change that feature reach.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — recorded escher-driver as naming no `dioxus-native-dom` feature (later superseded) and ratified the refresh on change; precedent for re-reading §1's Dioxus crates entry whenever a crate's dependency edge moves, which an advisory-driven manifest change here would do.

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table, so the feature reaches seven_guis' both binaries, yet neither builds a platform adapter (`accesskit_xplat`, `accesskit_winit` in neither graph; lockfile byte-identical, measured by dependency-graph reads before and after). Origin of decision/driver-requires-accessibility and the measurement method this chunk's lock moves must repeat.

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the driver's third dependency `tracing` added one lockfile line and no package, and neither platform adapter joined either seven_guis graph; precedent that a dependency change restates the §1 Dioxus crates lock reading.

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — re-read the stand graph on the merged lock where the accesskit family moved (accesskit 0.25.1, accesskit_unix 0.24.0, accesskit_android 0.9.0), the stand still holding neither `accesskit_xplat` nor `accesskit_winit`. This is the AccessKit 0.25 / accesskit_unix 0.24 pin through which the zbus family (RUSTSEC-2026-0370/0371/0372) reaches the build via `accesskit_xplat` ← `blitz-shell`; a move to an accesskit_unix on zbus 6 moves this pin and must repeat that re-read.

@history_item n=7 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — §1 re-read with escher-driver's `serde_json` (one lock line, no package, `accesskit_(xplat|winit)` 0 rows); one more precedent of the per-lock-change adapter probe.

@history_item n=8 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met; the a11y leg at six files) — widened the existing `a11y` job instead of adding a job, required status check or repository setting, on the founder's F4 «Расширить задание a11y (Рекомендую)»; origin of decision/a11y-ci-leg's "no required status check" and of decision/stand-contrast-asserted. Constrains the cancelled-runner watch's options here: removing or adding a required check touches a ruled shape.

@history_item n=9 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named, three gaps re-homed) — the stand's platform adapter and its windowed AT witness re-homed to a residual on the founder's F3 «Оставить, перенести (Рекомендую)»; every interactive-role node of the four lean tasks named (decision/interactive-nodes-named, decision/adapter-tree-equals-harness-tree); the unread `aria-activedescendant` kept (decision/active-descendant-unread). The adapter deferral is the ruling decision/stand-builds-no-platform-adapter rests on; a zbus/accesskit fix must not pull an adapter into a stand binary.

@history_item n=10 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the stand's a11y files also run three times in the `stand-repeat` leg and its job "Stand checks repeated" under the zero-failure budget, on the founder's «Повторное задание, ноль сбоев (Рекомендую)»; origin of decision/stand-checks-repeated-three-rounds, which a CI edit here must keep.

@history_item n=11 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` run three `stand-repeat` rounds by its named `--test` list, membership held by `stand_requirement_sweep`'s leg check, on the founder's «Список в скрипте (Рекомендую)»; the named list in `ci-leg.sh` is a guard (`stand_requirement_sweep`) this chunk's script edits must not break.

@history_item n=12 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — `ci-leg.sh a11y` runs `cargo test --workspace --locked` over fourteen files on one line with no comment line, pinned in `test_ci_workflows.py` one `assertIn` per target word; the job's tier, `needs`, cache and failure-only upload kept, on the founder's «Плюс проверки дерева (Рекомендую)». The pins and the leg's one-line form are exactly what this chunk's edits to `ci-leg.sh` / `test_ci_workflows.py` must keep (LegScriptTest's a11y tuple).

@history_item n=13 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip, and the stand graph re-read) — states the rule "the adapter's absence from the stand is re-read at each lock change" (re-read on `tokio` 1.54.1); this chunk's `Cargo.lock` moves (rusqlite/libsqlite3-sys, rav1e/ravif/image, memmap2, zip, accesskit_unix/zbus) each trigger that re-read and a §1 amendment.

@history_item n=14 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` `## Accessibility reach` publishes the a11y leg's fourteen files with no required status check and the AT reading unmeasured; the windowed AT witness owned by the residual re-carried at 0.2.0. A CI change to the a11y leg or its required-check status, or an adapter reaching a binary, must be restated there by hand.

@history_item n=15 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — trap: `stand_requirement_sweep`'s leg check reads `ci-leg.sh` as one text by `contains`, so a name dropped from the `stand-repeat` list alone while the a11y leg still names it stays green; the list is held jointly with `test_stand_repeat_leg_runs_three_rounds`. Relevant when this chunk edits `ci-leg.sh`.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read on the take at `ebacd68bd`) — re-read stating `Cargo.toml`, `Cargo.lock`, `deny.toml` and every `*/Cargo.toml` unchanged and the probe `cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows; the exact probe command this chunk's lock and `deny.toml` edits should rerun and cite.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand's graph re-read on the lock's move) — `pulldown-cmark` 0.13.4 → 0.13.5 in the lock alone, RUSTSEC-2026-0414's fix, the operator's call and not put to the founder, with the adapter probe re-read; direct precedent for an advisory-driven lock move and its a11y amendment.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the leg 14 files · 112 tests on CI#41192433140, `stand-repeat`'s list fifteen words pinned by `test_stand_repeat_leg_runs_three_rounds`; kept the job's slow tier, `needs`, restore-only `workspace-test` cache, failure-only log upload and no required status check — the invariants a CI change here must hold.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows) — SC 1.4.3 asserted on the load page and Cells beside the lean four in `stand_contrast`, part of decision/stand-contrast-asserted (the check run in the workspace test leg, per package and in the a11y leg); a leg reshuffle must not drop it from any of those.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed onto the moved lines) — the plan's citations include the CI files (`ci-leg.sh`, `test_ci_workflows.py`); trap: this chunk's edits to them will move cited lines in ci-integration.md and need re-pointing at the wrap.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — leg 14 files · 114 tests on CI#41235503227, `stand-repeat` seventeen named words; the leg stays fourteen files, slow tier, no required status check — the current counts a CI edit must leave unmoved.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup` and `select_popup_painted` run in `stand-repeat` by its named `--test` list, `stand_act_popup` by the `stand_*` glob, none in the a11y leg; leg membership `ci-leg.sh` encodes and this chunk must preserve.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed after the take) — citations into `.github/scripts/ci-leg.sh` and `.github/scripts/test_ci_workflows.py` re-pointed when those files changed; same trap as this chunk's CI edits.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no stand binary builds a platform adapter, on this take) — re-read with `Cargo.toml`, `Cargo.lock`, `deny.toml` unchanged, `^accesskit_(xplat|winit) ` 0 rows; the latest form of decision/stand-builds-no-platform-adapter's re-read before this chunk.

@history_item n=25 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (select_popup_painted also runs as embolden) — added leg `embolden` to `ci-leg.sh` (`LEGS`, last in `FAST_LEGS`), built with `blitz-paint/font-embolden`, with no CI job, macOS exercising the feature; precedent for a leg added to the fast set and for the macOS CI path the cancelled-runner watch and the skia fetch step touch.

@history_item n=26 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — `Cargo.lock` move fixing RUSTSEC-2026-0433 on the operator's own directive, no crate added or removed, adapter probe re-read; the stand adapter and its AT witness owned by the store's residual row re-carried at 0.2.0. Closest precedent for this chunk's advisory fixes and their a11y record.

@history_item n=27 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — a new blitz-tests target with no `stand_` prefix runs in neither the `a11y` nor the `stand-repeat` leg, no leg list moved; the leg-membership rule by prefix and named list that CI edits here rely on.

@history_item n=28 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed from `.andromeda/residuals.md` to residual rows, among them `res-llkgqygv` (the stand's platform adapter and its windowed AT witness); this chunk works residual rows (`res-p73cueyv` zbus et al.), and any closure touching the adapter must name the row, not the retired file.

@history_item n=29 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses re-said to their rows' status) — `res-llkgqygv` owner clauses read «re-carried at 0.3.0 (target: next)»; the deferral decision/stand-builds-no-platform-adapter and absence/assistive-technology-reach stand on, which a zbus/accesskit_unix move must not disturb.

@history_item n=30 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record on handle_event's path) — the reached record is ungated because the wasm build compiles dioxus-native-dom without `accessibility`; the boundary of decision/agent-readers-behind-accessibility, relevant if a dependency or feature change touches dioxus-native-dom's feature set.

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader DioxusDocument::graph) — the graph reader joined the `accessibility`-gated set and `signal_registry` stayed ungated; part of decision/agent-readers-behind-accessibility, which a feature edit must keep.

@history_item n=32 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — `actionable_nodes` added behind the `accessibility` feature; completes decision/agent-readers-behind-accessibility's gated list.

@history_item n=33 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (241 rows) on the founder's approval; every picked row's handle dates from this conversion, so an amendment this chunk raises is a row change (`+`/`~`/`−` a handle).

@history_item n=34 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — ~ decision/capability-statement-reach-by-hand: the capability statement's pin now reads the a11y leg's file count; a change to the leg's file list in `ci-leg.sh` will turn that pin red unless `docs/capabilities.md` moves with it.

@history_item n=35 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy: decision/a11y-tree-refresh-on-change; decision/accessibility-tree-through-document-method) — the refresh moved from the poll to the redraw after style resolve, the set then cleared; the last upstream sync before this chunk, carrying the accesskit/blitz-shell code a zbus/accesskit_unix move touches; guard `stand_accessibility_ids`.

@history_item n=36 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (6. Visual Design Verification: contract/accessibility-tree/hidden-and-inert-nodes; decision/hidden-content-excluded-from-a11y-tree) — `visibility: collapse` added to the exclusions on the W9 ruling; the current wording of two picked rows.

@history_item n=37 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — `aria-describedby` / `aria-description` reach only the platform adapter, which no stand binary builds, ratified as upstream's behaviour; origin of decision/aria-description-to-adapter, tied to the adapter crates (accesskit_xplat → accesskit_unix → zbus) this chunk's advisory work touches.

@history_item n=38 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — a truthy `inert` takes focusability, hits and the tree away; the unread `aria-activedescendant` pinned by `accessibility_active_descendant_unread`; origin of decision/inert-subtree-excluded and decision/active-descendant-unread's guard, both tests in the a11y and `stand-repeat` legs this chunk's CI edits must keep.

@history_item n=39 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — a framework-written `inert: false` removed by upstream's 28-name falsy predicate; the "framework-written inert false is removed" clause of decision/inert-subtree-excluded.
