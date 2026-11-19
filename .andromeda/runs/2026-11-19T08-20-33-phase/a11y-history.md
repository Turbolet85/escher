@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — made ci.yml's `a11y` job ("Accessibility (a11y) tests") run `bash .github/scripts/ci-leg.sh a11y`, which is `cargo test --workspace --locked` with a named `--test` list, in the slow tier, restoring the test job's cache and saving none (rows decision/a11y-ci-leg, decision/a11y-ci-gate). This is the leg invocation this chunk measures. It is a `--workspace` selection that differs from `-p blitz-tests` / `-p seven_guis`, and it shares the test job's cache. A mend that changes its cargo line is a CI-pipeline change pinned by `.github/scripts/test_ci_workflows.py`.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (root-manifest citations re-pointed after the line-61 insert) — an insert into root `Cargo.toml` left a11y-plan citations pointing one line low, and they had to be re-pointed by hand. Watch for the same trap: a `[profile.*]` edit or a new `.cargo/config.toml` that shifts manifest lines cited by rows.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — records the Dioxus feature wiring. The workspace takes dioxus-native-dom / dioxus-native with `default-features = false`, so a crate gets `accessibility` only by naming it. blitz-tests builds dioxus-native-dom with `accessibility`, and the seven_guis stand binary names none (rows decision/dioxus-accessibility-feature-by-name, decision/accessibility-feature-default-on). This is the root of the feature-unification differences between `-p blitz-tests`, `-p seven_guis` and `--workspace`, which the chunk's measurement must name.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — escher-driver first entered the stand graph naming no dioxus-native-dom feature, and the entry that first read the snapshot through a session was to decide the feature. This is the precedent the act-by-id entry replaced (row decision/driver-requires-accessibility).

@history_item n=5 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table, so the feature reaches seven_guis and both its binaries through that edge. The reach was measured by dependency-graph reads before and after, and the lockfile was byte-identical (rows decision/driver-requires-accessibility, decision/stand-builds-no-platform-adapter). Bearing on this chunk: it shapes which feature set each package selection resolves, and so which metadata hashes it gets. No mend may drop this edge or add an adapter (`accesskit_xplat` / `accesskit_winit`) to the stand graph.

@history_item n=6 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand checks' role, name and rendered-id tables were moved line for line into the one shared module `tests/blitz-tests/tests/common/mod.rs`, with no assertion changed (rows decision/role-name-tables-stated-once, decision/stand-expectation-tables). Precedent for this chunk's "one binary per leg" lever: files moved or merged must keep the tables stated once in the shared module and keep every assertion.

@history_item n=7 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met on the stand, three contrast tables, the a11y leg at six files) — widened the existing a11y job to six files on the founder's F4, with no new job, required status check or repository setting (the why of row decision/a11y-ci-leg). A mend that restructures legs keeps that shape: no required status check and no new job.

@history_item n=8 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the stand's a11y files run three times in the `stand-repeat` leg by their `stand_*` prefix, under the zero-failure budget (rows decision/a11y-test-leg, decision/stand-checks-repeated-three-rounds). `cargo test -p blitz-tests` is one of the scope's differing invocations. Merging `stand_*` files into one binary would change how this prefix-based membership is selected.

@history_item n=9 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` run three rounds in `stand-repeat` by its named `--test` list. That membership is held by `stand_requirement_sweep`'s leg check (row decision/stand-checks-repeated-three-rounds and its guard). A test-layout or leg-invocation change has to keep these names reachable, or move them together with that check.

@history_item n=10 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — `ci-leg.sh a11y` runs `cargo test --workspace --locked` over fourteen named files, on one line. Eight of them also run in `stand-repeat`. The pin asserts one `assertIn` per target word, in no order (row decision/a11y-ci-leg, guard LegScriptTest's a11y tuple). If test files are merged into one binary per leg, these `--test` target words and their pin change: that is a CI-pipeline change through `.github/scripts/test_ci_workflows.py`.

@history_item n=11 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` `## Accessibility reach` publishes the `a11y` leg's fourteen files with no required status check, edited by hand (row decision/capability-statement-reach-by-hand). If the mend changes the leg's file layout, this section has to be edited by hand.

@history_item n=12 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — trap for leg edits: `stand_requirement_sweep`'s leg check reads `ci-leg.sh` as one text by `contains`, so it cannot see a name dropped from the `stand-repeat` list alone while the a11y leg still names it. Membership is held only jointly with `test_stand_repeat_leg_runs_three_rounds` (row decision/stand-checks-repeated-three-rounds). A rewritten leg invocation must keep both guards able to see a dropped name.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the leg is fourteen files and 112 tests, measured locally and on CI. `stand-repeat`'s fixed list was raised to fifteen words, pinned by `test_stand_repeat_leg_runs_three_rounds`. The entry keeps the leg's slow tier, `needs`, restore-only `workspace-test` cache and failure-only upload. These are the tallies a merged binary must reproduce test for test: the boundary is no assertion changed.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list; the list's checks and the legs they run in) — `stand-repeat`'s named `--test` words were raised to seventeen and the a11y leg to 14 files · 114 tests. `select_popup` and `select_popup_painted` run by the named list and `stand_act_popup` by the `stand_*` glob (row decision/select-list-coverage-headless). These are the current membership facts that any change to leg invocation or test-binary layout must keep.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`select_popup_painted` also runs as embolden; a key press past a dropped anonymous block) — added the `embolden` leg, last in `FAST_LEGS`, built with `--features blitz-paint/font-embolden`. That is a second feature set over blitz-tests and one of the scope's suspected sources of re-hashes. Separately, `key_press_after_container_lost_its_box` was added with no `stand_` prefix and joins neither the `a11y` nor the `stand-repeat` leg. Both are leg-membership facts a mend has to keep.

@history_item n=16 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg; the stand's graph re-read on pulldown-cmark 0.13.6) — `dioxus_runtime_probe` is an auto-discovered blitz-tests target reached only by the workspace `test` leg, `run all` and `run <name>`. It is an instance of the per-file integration-test binaries this chunk measures, and a merge must keep it reachable. The same entry re-read the adapter probe after a lock-only move: no platform adapter joined the stand graph.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — dioxus-native-dom's `reached` record and `element_listener_ids` are ungated, because the wasm build compiles dioxus-native-dom without `accessibility`. That is a further feature set of the same crate, alongside the gated readers (row decision/agent-readers-behind-accessibility). Also, `stand_handler_coverage` joins `stand-repeat` by its prefix and is in no `a11y` list. Both bear on the feature-unification and prefix-membership measurements.

@history_item n=18 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip, and the stand graph re-read) — set the practice of re-reading the adapter's absence from the stand graph at each lock change (row decision/stand-builds-no-platform-adapter). This chunk's boundary keeps `Cargo.lock`. If a mend touches the lock or the manifests anyway, the probe is re-read.

@history_item n=19 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read on the take at `ebacd68bd`) — states the adapter probe's command: `cargo tree --locked -p seven_guis -e normal --prefix none`, 0 rows for `^accesskit_(xplat|winit) `. This is the check to re-run if a manifest or profile edit in this chunk could move the stand graph.

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the whole a11y master into rows, each verified against the code (decision, contract and absence rows). It is where every picked row's handle and guard comes from. This chunk's changes to leg cargo lines and the test layout are read against those guards (LegScriptTest's a11y tuple, `test_stand_repeat_leg_runs_three_rounds`, `stand_requirement_sweep`).

@history_item n=21 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement-reach-by-hand`: the capability statement's pin now reads the a11y leg's file count stated in `docs/capabilities.md`. Trap for the "one test binary per leg" lever: merging the a11y leg's fourteen files changes the count that pin reads. Both the statement and the pin then move, and the rest of the reach still needs a hand edit.

@history_item n=22 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/stand-shared-tab-order-table` gains its except: a Tab order reached only after a state change is a local table of the keyboard check, not in the shared module. If stand files are moved or merged into one binary, these local tables stay with their check, and the shared tables stay stated once.
