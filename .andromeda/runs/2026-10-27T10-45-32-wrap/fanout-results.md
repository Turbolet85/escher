# Fan-out results — 2026-10-27-upstream-sync-engine-debts (wrap .andromeda/runs/2026-10-27T10-45-32-wrap)

Batches: cites (10, re-spawn cites-r1 2: `[EVERY_ROW]`) · stage1 (13, re-spawn stage1-r1 3: `[REACHED]` · `[UNTOUCHED_REACHED]`) · classify (75, yes 230 of 1030) · strong (19, re-spawn strong-r1 2: `[PROPOSED_ARE_PICKED]` · `[NO_REPOINT]`) · newdetail (6) · newgroup (13) — every check `[PATH_CHECK] PASS`, every agent passing after at most one re-spawn. Not a sampled run (`cascade-sample.txt`: `sample: no`), so no `(sampler)` item.

## Verdicts
- architecture: 4 proposals · fanout-architecture.md · 26 cascade proposals · fanout-architecture--cascade-g1.md (0) · -g2.md (10) · -g3.md (2) · -g4.md (2) · -g5.md (4) · -g6.md (6) · -g7.md (2) · 9 facts · 9 placed · 0 no home
- security-plan: 2 proposals · fanout-security-plan.md · 5 cascade proposals · fanout-security-plan--cascade-g1.md (1) · -g2.md (3) · -g3.md (1) · fanout-security-plan--cascade-new-error-handling.md (0) · fanout-security-plan--cascade-new-secret-management.md (0) · 6 facts · 4 placed · 2 no home
- design-system: 0 proposals · fanout-design-system.md · 10 cascade proposals · fanout-design-system--cascade-g1.md (7) · -g2.md (3) · 0 facts · 0 placed · 0 no home
- layout-templates: 0 proposals · fanout-layout-templates.md (whole, no detail)
- test-plan: 4 proposals · fanout-test-plan.md · 22 cascade proposals · fanout-test-plan--cascade-g1.md (7) · -g2.md (3) · -g3.md (9) · -g4.md (1) · fanout-test-plan--cascade-new-integration-test-strategy.md (1) · fanout-test-plan--cascade-new-test-scope-summary.md (1) · 11 facts · 11 placed (2 by pass) · 0 no home
- obs-plan: 1 proposal · fanout-obs-plan.md · 9 cascade proposals · fanout-obs-plan--cascade.md (6) · fanout-obs-plan--cascade-new-log-coverage.md (3) · fanout-obs-plan--cascade-new-error-capture-reporting.md (0) · 4 facts · 3 placed (3 by pass) · 1 no home
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 5 cascade proposals · fanout-a11y-plan--cascade-g1.md (1) · -g2.md (4) · 4 facts · 4 placed · 0 no home

Notes the files carry (substance only): design-system and a11y-plan detectors hold; layout-templates' agent leaves the directive's landings to the orchestrator; security-plan's agent names §Error Handling's `FetchError` line and the Threat Model broad line (`security-plan.md:13`, sweep no-change) as the orchestrator's call; obs-plan's agent offers a replace of `obs-plan.md:98` in place of a sibling; the three no-home facts (security F2 `FetchError` variant, F5 `TMPDIR`; obs F3 the `file:` refusal error class) each name a body line as their home (security-plan.md:336, :302; obs-plan.md §7).

## Dispositions

Checks 1–7 per amendment-flow §Validate. Check 3: the scope record holds 0 lines; the directive's stand rows (inputs#I6 on the founder's word inputs#I7) are outside the chunk's intent and justified by that word. Check 4: every absence the proposals rest on ("no master states `:open`", the pseudo vote, the accessors) carries the report's sweep rows (`open`, `pseudo`, `cssdecl`) as its search. Check 6: the report's one disproved claim (the trial worktree's shape) is a chunk-document claim with no master site — disposed to the implement outcome file that records it, its removal the founder's (handoff). Check 7: 2 contradiction rows, both cleared as «rewritten by …» under F359; 2 twins rejected.

fanout-architecture.md
1. apply — check 1: Accurate this-chunk addition (report Symbols / APIs, Counts `tmpdir` `architecture.md:216` changed)
2. apply — check 1: Accurate this-chunk addition; an engine behaviour change for every document on the founder's own word (inputs#I4), a narrowing, not a widening (report Decisions)
3. apply-merged → fanout-architecture--cascade-g4.md 1 — the same `flush_open_state` row, the detail file's
4. apply — check 1: Accurate this-chunk addition (Counts `upstream` `architecture.md:287` changed)
5. apply — check 5 (citation-dispositions.md rows 3, 4, 7, 8, 9: claim false, the code moved by the runtime split) · §Established Decisions / §Infrastructure Patterns, .andromeda/architecture.md:205, :254, :269, :298 — re-point each listed citation · as: each citation of `packages/blitz-vibey-script/src/runtime.rs` those rows list takes the path and lines its row names (`runtime/bootstrap.rs:59-65`, `runtime/window.rs:76-84`, `runtime/modules.rs:16-92`, `runtime/events.rs:13-32`, `runtime/bootstrap/window.js:1-4`); the claim text unchanged, except :254's module loader, which adds that its fetch is gated by `document_may_load`
6. apply — check 5 (rows 11, 12, 14, 15) · §Conventions, .andromeda/architecture-detail/conventions.md:27, :33, :46 — re-point · as: :27's two delimiter citations read `runtime/window.rs:123` and `dom/element/attributes.rs:15` (the `// === … ===` convention stands in the moved files); :33's `runtime.rs:1386-1408` reads `runtime/bootstrap.rs:93-121`; :46's let-chain `runtime/modules.rs:115-117`; claim text unchanged
7. apply — check 5 (rows 16, 17, 18, 19) · §Cross-cutting Patterns, .andromeda/architecture-detail/cross-cutting-patterns.md:15, :27 — re-point · as: `runtime/events.rs:20-21`, `runtime/timers.rs:35`, `runtime/events.rs:190-192`, `dom/element/geometry.rs:13-27`; claim text unchanged
8. apply — check 5 (rows 21–26) · §Established Decisions, .andromeda/architecture-detail/established-decisions.md:39, :143 — re-point · as: :39 `dom/element/geometry.rs:78-79`; :143 `dom/element/markup.rs:82-86`, `runtime/bootstrap/interfaces.js:23-48`, `runtime/bootstrap/window.js:1-39`, `runtime/bootstrap.rs:48-52`, `runtime/events.rs:171-188`; claim text unchanged
9. apply — check 5 (expected amendment 2's owners; Counts `res-35-36` read at P2; inputs#I5's card rows, the operator's own word) · §Established Decisions, .andromeda/architecture-detail/established-decisions.md:76 — the clock's merge-surface owner · as: «owned by the same `.andromeda/residuals.md` line (target: next)» reads «owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5)»
10. apply — check 5 (row 27: claim false) · §Stack and Technologies, .andromeda/architecture-detail/stack-and-technologies.md:73 — publish's dioxus-cli · as: «publish with Rust "1.96.1" and dioxus-cli "0.7.8"» reads «publish with Rust "1.96.1" and dioxus-cli `0.8.0-alpha.3` (or, on a platform flagged `use_git_cli`, a git install at a pinned rev) — upstream's, taken at 2026-10-27-upstream-sync-engine-debts (`a7ea9631b` · `b5b163450`), the workflow upstream-only and repository-guarded»; its three citations as they now stand
11. apply — check 5 (rows 30, 32–37: claim false) and check 7's companion of fanout-architecture--cascade-g5.md 3 · §Standard Contracts → JavaScript, .andromeda/architecture-detail/standard-contracts.md:210, :211 — the `FetchError` variants and the moved citations · as: line 210's «with `UnsupportedScheme`, `Io`, `InvalidData`» reads «with `UnsupportedScheme`, `Io`, `InvalidData` and, since 2026-10-27-upstream-sync-engine-debts, `Forbidden` (a `file:` load refused for a non-`file:` document)»; «a JS bootstrap string» reads «a JS bootstrap, `BOOTSTRAP_JS`, a `concat!` of nine `include_str!` scripts under `runtime/bootstrap/`»; each `runtime.rs` / `dom/element.rs` citation of rows 32–37 takes its row's path and lines (`runtime/window.rs:210-223`, `runtime/fetch.rs:9-66`, `runtime/events.rs:362-371`, `runtime/bootstrap.rs:12`, `runtime/bootstrap.rs:14-35`, `dom/element/markup.rs:8-22`); row 33's `fetch_sync` adds that a forbidden load is also a TypeError; the rest unchanged

fanout-architecture--cascade-g1.md
(no proposals)

fanout-architecture--cascade-g2.md
1. apply — check 1: Accurate this-chunk addition (Counts `upstream` `established-decisions.md:67` @c665 changed)
2. apply — check 1: Accurate this-chunk addition (Counts `upstream` `:69` changed)
3. apply — check 5 (inputs#I5's card rows, the operator's own word; Counts `res-35-36`) · as: line 75's closing clause reads «is owned by a `.andromeda/residuals.md` line, re-read row by row with its owner quoted at the 2026-10-27 upstream sync (escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/readings.md), its owner Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5)»
4. apply — check 1: Accurate this-chunk addition · as: as the change reads, and in the same line «is owned by the same `.andromeda/residuals.md` line (target: next)» reads «is owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5)»
5. apply — check 1: An engine behaviour change for every document, on the founder's own word (inputs#I4) · check 7: architecture-newgroup-established-decisions.md · distinct
6. apply — check 7: architecture-newgroup-established-decisions.md · distinct
7. apply — check 7: architecture-newgroup-established-decisions.md · distinct
8. apply — check 7: architecture-newgroup-established-decisions.md · distinct
9. apply — check 7: architecture-newgroup-established-decisions.md · distinct
10. apply — check 7: architecture-newgroup-established-decisions.md · distinct

fanout-architecture--cascade-g3.md
1. apply — check 1: Accurate this-chunk addition; it settles citation-dispositions.md row 13
2. apply — check 1: Accurate this-chunk addition (report Coverage: both layout modes not asserted by the file)

fanout-architecture--cascade-g4.md
1. apply — check 1: Accurate this-chunk addition (a new public API, report Symbols / APIs)
2. apply — check 1: Accurate this-chunk addition (dependent of 1, applied with it)

fanout-architecture--cascade-g5.md
1. apply — check 1: Accurate this-chunk addition (Counts `redraw` `standard-contracts.md:96` read at P2)
2. escalate → Q1 — check 1: Provisional discharge (playbook, verdict escalate): the F2 gap marker drops on the founder's own word (inputs#I7) · resolved: apply (Q1)
3. apply — check 7: architecture-newgroup-standard-contracts.md · contradiction · lines 210–210 · rewritten by fanout-architecture.md 11
4. apply — check 7: architecture-newgroup-standard-contracts.md · distinct

fanout-architecture--cascade-g6.md
1. apply — check 1: Accurate this-chunk addition (Counts `tmpdir` `occupied-resources.md:78` changed)
2. apply-merged → fanout-architecture.md 1 — the same `architecture.md:216` clause
3. apply — check 1: Accurate this-chunk addition (a test filesystem write, registered as `eab-{pid}` is) · as: as the change reads, without «local only and uploaded by no CI leg», which the report does not carry, and stating nothing of the directory's removal
4. apply — check 1: Accurate this-chunk addition (Counts `redraw` `cross-cutting-patterns.md:21` read at P2)
5. apply — check 1: Accurate this-chunk addition (Counts `upstream` `project-intent.md:9` changed)
6. apply-merged → fanout-architecture.md 4 — the same `architecture.md:287` line

fanout-architecture--cascade-g7.md
1. apply — check 7: architecture-newgroup-existing-scopes.md · distinct
2. apply — check 1: Accurate this-chunk addition

fanout-security-plan.md
1. apply — check 1: graded `escalate` by its detector, which reports the invariant holding (the new input surface carries its validation), the plan's reviewed list naming §Input Validation (script fetch) (expected amendment 4); an engine behaviour change for every document on the founder's own word (inputs#I4); a narrowing, not a boundary widening (report Decisions)
2. apply — check 1: Accurate this-chunk addition (Counts `tmpdir` `security-plan.md:302` changed); the no-home fact F5's home
3. apply — check 5 (row 40: claim false) · §Dependency Security, .andromeda/security-plan.md:247 — the CI tooling's dioxus-cli · as: «dioxus-cli is pinned to 0.7.8» reads «the upstream-only publish workflow installs dioxus-cli `0.8.0-alpha.3`, or on a platform flagged `use_git_cli` from git at a pinned rev (upstream's, taken at 2026-10-27-upstream-sync-engine-debts, behind its `DioxusLabs/blitz` repository guard)»; the citation `publish-browser.yml:126` reads `:122-130`
4. apply — check 5 (rows 42, 46, 47) and the no-home fact F2's home · §Error Handling / §Logging, .andromeda/security-plan.md:328, :336, :386 — re-point and the new message · as: :328's `runtime.rs:1458-1514` reads `runtime.rs:127-138; runtime/modules.rs:94-131`; :336 reads «with scheme, IO, invalid-data and, since 2026-10-27-upstream-sync-engine-debts, forbidden-load messages» and its `runtime.rs:1225-1240` reads `runtime/modules.rs:65-86`; :386's `runtime.rs:1104-1105` reads `runtime/events.rs:20-21`; claim text otherwise unchanged
5. apply — check 5 (row 49) · .andromeda/security-plan-detail/api-security.md:46 — re-point · as: `runtime.rs:155-180` reads `runtime/bootstrap/fetch.js:81-83; runtime/fetch.rs:14-66`; claim text unchanged
6. apply — check 5 (rows 53–63) · .andromeda/security-plan-detail/input-validation.md:296–314 — re-point · as: each citation takes its row's path and lines (:296 `runtime/bootstrap/fetch.js:38-39`; :299 `runtime/bootstrap/fetch.js:81-83`; :302 `dom/element/selectors.rs:9-17`; :305 `dom/element/scrolling.rs:35-52`, `:206-228`, `:90-95`; :308 `dom/element/attributes.rs:289-305`, `:343-354`; :311 `runtime/timers.rs:51-59`, `:135-137`; :314 `runtime/css.rs:81-131`); claim text unchanged
7. apply — check 5 (rows 65, 66) · .andromeda/security-plan-detail/threat-model-summary.md:17 — re-point · as: `runtime/events.rs:100-141`; `runtime/fetch.rs:32-36`; claim text unchanged

fanout-security-plan--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition (Counts `fetcher` `threat-model-summary.md:18` changed); a narrowing

fanout-security-plan--cascade-g2.md
1. apply — check 7: security-plan-newgroup-input-validation.md · distinct
2. apply — check 7: security-plan-newgroup-input-validation.md · distinct
3. apply — check 7: security-plan-newgroup-input-validation.md · distinct

fanout-security-plan--cascade-g3.md
1. apply — check 7: security-plan-newgroup-dependency-security.md · distinct

fanout-design-system.md
1. apply — check 5 (the directive, inputs#I6 on the founder's word inputs#I7: «the two rows shared with todomvc keep todomvc and drop seven_guis»; Counts `stand-ten` `design-system.md:218` changed) · §Surface: web-spa, .andromeda/design-system.md:218 · as: «seven_guis and todomvc fill the window with a canvas on `#f5f5f5` (examples/seven_guis/index.html:7-10; examples/todomvc/index.html:7-10)» reads «todomvc fills the window with a canvas on `#f5f5f5` (examples/todomvc/index.html:7-10)»
2. apply — check 5 (rows 67, 68) · .andromeda/design-system-detail/motion.md:10, :35 — re-point · as: `dom/element/scrolling.rs:37-52`; `runtime/timers.rs:98-123`; claim text unchanged
3. apply — check 5 (rows 69, 70) · .andromeda/design-system-detail/surface.md:13 — re-point · as: `runtime/bootstrap.rs:127-132`; `runtime/window.rs:125-151`; claim text unchanged
4. apply — check 5 (row 71) · .andromeda/design-system-detail/typography.md:36 — re-point · as: `runtime/bootstrap/fonts.js:1-15`; claim text unchanged

fanout-design-system--cascade-g1.md
1. apply — check 5: the directive on the founder's word (inputs#I7)
2. apply — check 5: the directive on the founder's word (inputs#I7); its value already held at `layout-templates.md:16` (Q2, fanout-layout-templates.md 6 folded)
3. apply — check 5: the directive on the founder's word (inputs#I7); its landing fanout-layout-templates.md 5
4. apply — check 5: the directive on the founder's word (inputs#I7); its landing fanout-layout-templates.md 4
5. apply — check 5: the directive on the founder's word (inputs#I7); its fold fanout-layout-templates.md 3
6. apply — check 5: the directive on the founder's word (inputs#I7); its fold fanout-layout-templates.md 3
7. apply — check 5: the directive on the founder's word (inputs#I7); its fold fanout-layout-templates.md 2

fanout-design-system--cascade-g2.md
1. apply — check 5: the directive on the founder's word (inputs#I7); its fold fanout-layout-templates.md 3
2. apply — check 5: the directive on the founder's word (inputs#I7); its fold fanout-layout-templates.md 2
3. apply — check 5: the directive on the founder's word (inputs#I7)

fanout-layout-templates.md
1. escalate → Q1 — check 1: Provisional discharge · §Surface: desktop-native (values · desktop-native-screens), layout-templates.md:20 — the F2 gap marker drops on the founder's own word (inputs#I7) · resolved: apply (Q1) · as: line 20 is removed whole; the `<!-- /detail -->` closer and the blank before it stay
2. apply — check 5 (the directive: the load-page facts fold into the served-screens bullet) · layout-templates.md:14 · as: the served-screens bullet adds, from `motion.md:34` and `surface.md:40`, that `LoadNet` answers from a thread of its own, so a driver `click` on `load-fetch` waits it out and returns with the message shown, and that the load page holds two keyed elements, `load-fetch` and `load-done`, citing `tests/blitz-tests/tests/stand_load_page.rs:34-68` — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md; values unchanged
3. apply — check 5 (the directive: the select facts fold into the mode-region bullet) · layout-templates.md:15 · as: the mode-region bullet adds weight 400 to its «styled 15px …» clause (from `typography.md:19`) and that its two options show the chosen one alone by the engine's select rule (from `surface.md:10`); padding 8/10 already stated; values unchanged
4. apply — check 5 (the directive: a new line) · the `values · desktop-native-screens` block, after :19 · as: a new bullet «seven_guis stand type — sans-serif, the home title 700 at 36px over a 14px base (examples/seven_guis/src/app.rs:179-205)», from `typography.md:18`, values unchanged
5. apply — check 5 (the directive: a new line) · the same block · as: a new bullet «seven_guis stand invalid state — `#e53e3e` / `#fff5f5` / `#c53030` (examples/seven_guis/src/tasks/flight_booker.rs:194-198; examples/seven_guis/src/tasks/flight_booker.rs:229-237)», from `color-palette.md:71`, values unchanged
6. escalate → Q2 — check 5 (the directive names `color-palette.md:59`, the stand's text `#1a1a1a`, among the three new lines; layout-templates.md:15 and :16 already state that value with the same three citations) · the same block — a new bullet for the stand's text colour · resolved: reject (Q2) — folded into `layout-templates.md:16`, which already holds the value
7. apply — check 5 (rows 73, 74) · layout-templates.md:54, :55 — re-point · as: `runtime/window.rs:165-208`; `dom/element/geometry.rs:78-171`; claim text unchanged

fanout-test-plan.md
1. apply — check 1: Accurate this-chunk addition (report Symbols / APIs, Coverage)
2. apply — check 1: Accurate this-chunk addition (dependent of 1)
3. apply — check 1: Accurate this-chunk addition (report Coverage `:open` tests ✗)
4. apply — check 1: Accurate this-chunk addition (Counts `run-all` 630 → 633)

fanout-test-plan--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition (Symbols / APIs, tab.rs's test module)
2. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct
3. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct
4. apply — check 1: Accurate this-chunk addition (Counts `dom-rs` `test-scope-summary.md:68` changed); it settles citation-dispositions.md row 77 · as: as the change reads, and the citation `tests/dom.rs:23-824` reads `tests/dom.rs:23-950`
5. apply — check 1: Accurate this-chunk addition (Counts `dnd-lib` `:74` changed)
6. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct
7. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct

fanout-test-plan--cascade-g2.md
1. reject — check 7: test-plan-newgroup-test-scope-summary.md · twin · fanout-test-plan--cascade-g1.md 2
2. reject — check 7: test-plan-newgroup-test-scope-summary.md · twin · fanout-test-plan--cascade-g1.md 3
3. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct

fanout-test-plan--cascade-g3.md
1. apply — check 1: Accurate this-chunk addition (Counts `bt-files` `test-strategy.md:54` changed)
2. apply — check 1: Accurate this-chunk addition (Crates / modules, the 13 modules)
3. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
4. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
5. apply — check 7: test-plan-newgroup-test-data-fixtures.md · distinct
6. apply — check 1: Accurate this-chunk addition (New text, by line: `FILE_LOAD_TEST_HTML` 871-892, `FILE_LOAD_TEST_FILES` 895-902)
7. apply — check 7: test-plan-newgroup-ci-integration.md · distinct
8. apply — check 1: Accurate this-chunk addition (Counts `ws-tally` `ci-integration.md:43` changed)
9. apply — check 1: Accurate this-chunk addition (Counts `ws-tally` `ci-integration.md:64` changed)

fanout-test-plan--cascade-g4.md
1. apply — check 5 (inputs#I5's card rows, the operator's own word; Counts `res-35-36` `quality-gates-coverage-targets.md:15` read at P2)

fanout-test-plan--cascade-new-integration-test-strategy.md
1. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct

fanout-test-plan--cascade-new-test-scope-summary.md
1. apply — check 7: test-plan-newgroup-test-scope-summary.md · contradiction · lines 14–14 · rewritten by fanout-test-plan--cascade-g1.md 1

fanout-obs-plan.md
1. apply — check 1: Accurate this-chunk addition (Counts `upstream` `obs-plan.md:98` changed, `silence` extended) · as: the take reading of line 98 is replaced by this take's (read at `ebacd68bd`, as the change states), not added as a sibling bullet — a body holds current truth, the earlier reading's history in the sidecar
2. apply — check 5 (the no-home fact F3's home, obs-plan §7) · §7 Script (JS) error surface, after obs-plan.md:229 · as: a new bullet «A `file:` load refused for a non-`file:` document (`document_may_load`, since 2026-10-27-upstream-sync-engine-debts) is a typed `FetchError` with no log site: `fetch()` rejects with a TypeError, a module `import` fails, a `<script src>` is skipped (packages/blitz-vibey-script/src/fetch.rs:40-46; packages/blitz-vibey-script/src/runtime/fetch.rs:40-44; packages/blitz-vibey-script/src/document.rs:163-169)»
3. apply — check 5 (rows 80–87) · .andromeda/obs-plan.md:226–229 — re-point · as: :226 `runtime/events.rs:58-74`; :227 `runtime/events.rs:84-96`; :228 `runtime/events.rs:34-36`; :229 `runtime/timers.rs:32`, `runtime/timers.rs:35`, `runtime/events.rs:191`, `runtime/events.rs:310`, `runtime/events.rs:81`; claim text unchanged
4. apply — check 5 (row 89) · .andromeda/obs-plan-detail/log-coverage.md:101 — re-point · as: `runtime/events.rs:20-21`; claim text unchanged

fanout-obs-plan--cascade.md
1. apply — check 1: Accurate this-chunk addition (Counts `upstream` `pii-scrubbing-compliance.md:13` changed) · as: as the change reads, without its closing «— as the take read at `f9cbb2756` held before (…)», the earlier reading's history in the sidecar
2. apply — check 1: Accurate this-chunk addition
3. apply — check 1: Accurate this-chunk addition
4. apply — check 1: Accurate this-chunk addition
5. apply — check 1: Accurate this-chunk addition
6. apply — check 1: Accurate this-chunk addition

fanout-obs-plan--cascade-new-log-coverage.md
1. reject — check 2: the same fact lands in obs-plan §3's silence census, its home (fanout-obs-plan.md 1); the file's own note asks for the drop
2. reject — check 2: the same fact lands in obs-plan §3's silence census (fanout-obs-plan.md 1); the file's own note asks for the drop
3. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct

fanout-a11y-plan.md
1. apply — check 5 (rows 93, 94) · .andromeda/a11y-plan-detail/keyboard-navigation.md:54, :55 — re-point · as: `dom/element/form_controls.rs:286-298`; `dom/element/form_controls.rs:41-66`; claim text unchanged

fanout-a11y-plan--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition (Counts `upstream` `a11y-scope-summary.md:18` changed)

fanout-a11y-plan--cascade-g2.md
1. apply — check 7: a11y-plan-newgroup-keyboard-navigation.md · distinct
2. apply — check 7: a11y-plan-newgroup-keyboard-navigation.md · distinct
3. apply — check 7: a11y-plan-newgroup-visual-design-verification.md · distinct
4. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
