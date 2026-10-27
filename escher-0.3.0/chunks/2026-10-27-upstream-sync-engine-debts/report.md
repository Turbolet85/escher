# Report — 2026-10-27-upstream-sync-engine-debts

**Chunk:** Upstream sync ahead of engine debts — upstream/main `ebacd68bd` measured, 27 of 40 past the merge base not yet in; eleven taken by the founder's word, merge-surface owners kept; CI, a11y and audit green, per-OS tallies and sink allowlist re-read (v030-03)
**Date:** 2026-10-27
**Commits:** `e795a088c` chore(2026-10-27-upstream-sync-engine-debts): operator pre-CI commit, for the run this chunk's verdict reads — the one commit of `git log --format='%h %s' de85792f5..HEAD` (Setup 4's base `de85792f5`, the parent of the oldest pre-CI commit); its body names the take on the founder's «Одиннадцать коммитов (Рекомендую)» and carries the eleven `(cherry picked from commit {sha})` trailers (`evidence/ci.md`)

## Changes (structured — detectors read this)
- **Files:** 41 source files, all on research's lists (`gate.py scope`: `clean — changed 41 · listed 41 · recorded 0 · absorbed 0 · excluded 588`, trail `.andromeda/runs/2026-10-27T10-45-32-wrap/gate-2026-10-27-upstream-sync-engine-debts.json`):
  - new (23): `packages/blitz-vibey-script/src/dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs`; `packages/blitz-vibey-script/src/runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs`; `packages/blitz-vibey-script/src/runtime/bootstrap/{css,dom_exception,fetch,fonts,interfaces,node_constants,style,stylesheets,window}.js`; `tests/blitz-tests/tests/pseudo_element_flow_classification.rs`
  - modified (18): `.github/workflows/publish-browser.yml`; `apps/browser/src/{main,tab}.rs`; `wpt/WPT_COMMIT`; `packages/blitz-dom/src/{layout/construct,mutator,node/element,stylo}.rs`; `packages/blitz-shell/src/{lib,window}.rs`; `packages/blitz-vibey-script/src/{document,fetch,lib,dom/element,runtime}.rs`; `packages/blitz-vibey-script/tests/dom.rs`; `packages/dioxus-native-dom/src/dioxus_document.rs`; `tests/blitz-tests/tests/all.rs`
  - beside them (excluded from scope): `escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/{scope,research,plan}.md`, `evidence/{before,take,lock,readings,sink,counts,ci}.md`, `inputs/` (I1–I7); `escher-0.3.0/verification-matrix.json` (`v030-03` → implemented); `.andromeda/master-route.md` (the pending record); `escher-0.3.0/working-route.md`; the phase and implement run dirs
- **Symbols / APIs:** every one upstream's, taken by no-commit cherry-pick of `d63ed2c8c` · `6587e7e79` · `e6f4615ec` · `a7ea9631b` · `b5b163450` · `7fc9ee3af` · `2963b65a2` · `cee6b8731` · `53e9d81c6` · `6fee0db25` · `efdc5472a` in that order (`evidence/take.md`); coordinates of added text from the `## New text, by line` section below:
  - blitz-dom `ElementData::flush_open_state` (`packages/blitz-dom/src/node/element.rs:543-549`, new, pub), called from `ElementData::new` (`:488-489`) and from `DocumentMutator::set_attribute` / `remove_attribute` on an `open` write (`packages/blitz-dom/src/mutator.rs:368-371`, `:520-523`); it sets `ElementState::OPEN` on a `details` or `dialog` carrying `open`; `NonTSPseudoClass::Open` now reads that state in place of `false` (`packages/blitz-dom/src/stylo.rs:510`) — `:open` matches. In `set_attribute` the `open` arm stands after escher's range re-sanitize arm (`min` · `max` · `step` → `resanitize_range_input`) as a second independent `if` (the take's resolution 3)
  - blitz-dom `create_text_editor` (`packages/blitz-dom/src/layout/construct.rs:313-317`): a text input already focused when its editor is constructed calls `shell_provider.set_ime_enabled(true)` (`7fc9ee3af`); `classify_flow_children` (`:860`, `:867-872`): every box-generating `::before`/`::after` votes in flow classification, the `display: contents` filter dropped (`2963b65a2`)
  - blitz-shell (`packages/blitz-shell/src/lib.rs:90`, `:93-98`, `:104-106`; `window.rs:193-194`): the windowed shell's redraw requests travel through its event loop (`6587e7e79`)
  - blitz-vibey-script: `pub fn document_may_load(document_url: Option<&Url>, url: &Url) -> bool` (`packages/blitz-vibey-script/src/fetch.rs:40-46`) — a `file:` URL loads only for a `file:` document — and a `FetchError` variant for the refusal (`fetch.rs:10-12`, `:23`; re-exported `lib.rs:43`), checked by `fetch()` (`runtime/fetch.rs` 40-44), module `import` (`runtime/modules.rs`) and `<script src>` (`document.rs:163-169`, `:210-211`, `:222`) (`cee6b8731`); the runtime split into `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` with nine bootstrap scripts and the element bindings into `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs` (`e6f4615ec`, a move: `runtime.rs` keeps 151 lines, upstream's side whole); CSS properties as accessors on `CSSStyleDeclaration` — `runtime/css.rs` `css_property_names` (8-18) and `bootstrap/style.js` `installStyleProperties` (26-48) (`efdc5472a`); `ScriptRuntime`'s public methods unchanged in name, moved between files; the uncaught-JS-error `tracing::error!` moved whole into `report_js_error` (`runtime/events.rs` 13-32 @19, the line at `:21`, gated `#[cfg(feature = "tracing")]`); escher's select-by-id comment carried to `runtime/events.rs:171-173` (read in `evidence/take.md` §1); blitz-vibey-script's `scroll_into_view` caller moves from `dom/element.rs` into `dom/element/scrolling.rs` (230-268), calling the same engine method
  - apps/browser (upstream-only app): tab lenses survive a webview props replacement (`apps/browser/src/tab.rs:130-136`, `:177-182`, test module `295-343`) (`d63ed2c8c`)
  - dioxus-native-dom: one test added, `input_focused_on_mount_enables_ime` (`packages/dioxus-native-dom/src/dioxus_document.rs:725-759 @729`); the file's diff removes 0 lines (`evidence/take.md` §2); `task_polled_during_render_can_access_document` (`286874cc1`'s) not taken
  - no new port, socket, listener, env read in source, print or log site; one new test-only env read: `TMPDIR` through `std::env::temp_dir()` in `packages/blitz-vibey-script/tests/dom.rs` `temp_dir_with_files` (860-869 @861, the call at `:863`) — test code only, not secret, a second test-only site beside escher-driver's client unit test (`evidence/sink.md`)
  - unchanged and re-read: `hit_inner`, `physical_unrounded_geometry`, `absolute_position`, `scroll_into_view`, `visible_region`, `is_disabled`, `flush_is_focussable`, `RangeInputData`'s seeding, the changed-set inserts, the select arms; `packages/escher-telemetry/` and `packages/escher-driver/` (`git diff --stat de85792f5 -- packages/escher-telemetry packages/escher-driver` empty, `evidence/sink.md`)
- **Crates / modules:** no crate added or removed; blitz-vibey-script gains the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files); blitz-tests gains the auto-discovered target `pseudo_element_flow_classification` (`all.rs:54` gains its `mod` line; `all.rs` stays `test = false`)
- **Dependencies:** none — `Cargo.toml`, `Cargo.lock`, `deny.toml` and every `*/Cargo.toml` byte-identical to `de85792f5` (`git diff --stat de85792f5 -- Cargo.toml Cargo.lock deny.toml '*/Cargo.toml'` empty, `evidence/lock.md`); no package enters, leaves or moves; Stylo 0.22.0, taffy 0.15 (registry), Dioxus 0.7.3, winit `=0.31.0-beta.3`, no git dependency
- **Schema / config:** none — no config key, violation schema, scrub or redaction shape; escher's sink prefix and scrub sets unchanged
- **Spec-master edits:** none (implement and the operator pass write no master)
- **Retired:** none — no surface, stack or capability removed (the runtime and element bindings moved between files, none removed)
- **Counts / qualifiers moved:** each with its pattern id (`.andromeda/runs/2026-10-27T10-45-32-wrap/report-patterns.toml`, swept by `cascade.py sweep --pre-pass`, trail `.andromeda/runs/2026-10-27T10-45-32-wrap/cascade-2026-10-27-upstream-sync-engine-debts-pre-pass.json`); a site count is the pattern's `standing` figure:
  - blitz-tests directory 147 → 148 `.rs` files, 146 → 147 test targets (`pseudo_element_flow_classification`, `2963b65a2`) [`bt-files`: 1 standing · 1 changed (`test-plan-detail/test-strategy.md:54`)]
  - workspace tally 208 · 1117 · 0 · 12 → 209 · 1125 · 0 · 12 (+1 line, the new target; +8 passed: its 3, blitz-vibey-script `tests/dom.rs` 3, dioxus-native-dom 1, `apps/browser/src/tab.rs` 1 — `evidence/counts.md`); per-OS on CI#40487650912 against CI#40434241601: Test [default features] 209 · 1125 · 0 · 12; Test (macos) and Coverage report 190 · 1122 · 0 · 12; Test (windows) 190 · 1076 · 0 · 9; Accessibility (a11y) tests 14 · 85 · 0 · 0 and Stand checks repeated 204 · 864 · 0 · 21 unmoved (`evidence/ci.md`) [`ws-tally`: 2 standing · 2 changed (`test-plan-detail/ci-integration.md:43` the per-OS rows; `:64` the local tally)]
  - `run all` run.end 630 · 0 · 12 → 633 · 0 · 12 (+3, the new target's) [`run-all`: 1 standing · 0 changed · 1 no-change (`test-plan.md:172`, a 0.1.0 reading of 396 as measured then; no master states the current `run all` figure)]; `run stand` 205 · 0 · 7 over 50 files unmoved
  - blitz-vibey-script `tests/dom.rs` 27 → 30 `#[test]` (`cee6b8731` +2, `efdc5472a` +1) [`dom-rs`: 10 standing · 1 changed (`test-plan-detail/test-scope-summary.md:68`, "27 `#[test]`" and its range `:23-824`) · 6 citation rows into the file (`test-plan.md:96`, `:143`; `test-plan-detail/integration-test-strategy.md:8`; `mocking-stubbing-discipline.md:19`; `test-data-fixtures.md:25`, `:40`) — the file gained lines at 455-488 and 859-950, the citation sweep's · 3 no-change (`architecture-detail/existing-scopes.md:61`, `test-plan.md:40` name the file; `design-system-detail/typography.md:73` a match note)]
  - dioxus-native-dom lib 81 → 82 (`input_focused_on_mount_enables_ime`) [`dnd-lib`: 1 standing · 1 changed (`test-plan-detail/test-scope-summary.md:74`, "81 unit tests in nine files")]
  - test-only `TMPDIR` reads 1 → 2 sites (blitz-vibey-script `tests/dom.rs:863`) [`tmpdir`: 10 standing · 3 changed (`architecture.md:216` "in one unit test only"; `architecture-detail/occupied-resources.md:78`; `security-plan.md:302`) · 7 no-change (`CARGO_TARGET_TMPDIR`, another variable sharing the token: `occupied-resources.md:32`, `:46`, `security-plan.md:301`, `test-plan.md:149`, `integration-test-strategy.md:25`, `test-strategy.md:14`, `registries/contracts/test-plan/session-lifecycle.md:8`); leaf `.claude/rules/security.md:9` (the cascade's)]
  - the default script fetcher: `file:` and `data:` only → and a `file:` URL only for a `file:` document (`document_may_load`), a narrowing; same-origin and CORS still absent [`fetcher`: 13 standing · 1 changed (`security-plan-detail/threat-model-summary.md:18`, the trust boundary) · 2 read at P2 for the rule (`architecture.md:173`, `architecture-detail/standard-contracts.md:210`, the ScriptFetcher contract) · 10 no-change (the fetcher's identity named: `architecture.md:254`, `conventions.md:33`, `established-decisions.md:143`, `standard-contracts.md:271`, `security-plan.md:13`, `api-security.md:46`, `threat-model-summary.md:17`, `test-plan.md:255`, `integration-test-strategy.md:8`, `mocking-stubbing-discipline.md:19`); leaves `.claude/rules/security.md:15`, `.claude/docs/security-summary.md:14` (the cascade's)]
  - upstream `main` read `f9cbb2756` → `ebacd68bd`, merge base `23b9b26a9` unmoved, eleven taken, sixteen left out (`evidence/lock.md`) [`upstream`: 17 standing · 7 changed (`architecture.md:287`; `architecture-detail/project-intent.md:9`; `established-decisions.md:67` (@c665), `:69`; `obs-plan.md:98`; `obs-plan-detail/pii-scrubbing-compliance.md:13`; `a11y-plan-detail/a11y-scope-summary.md:18` — each a "take read at" reading) · 1 extended (`security-plan-detail/dependency-security.md:22`, the per-sync lock record: this sync's row) · 9 no-change (the 2026-10-17 merge at `23b9b26a9`: `established-decisions.md:20`, `:75`, `standard-contracts.md:24`, `dependency-security.md:19`, `:20`, `:21`, `test-scope-summary.md:14`, `:98`, `a11y-strategy.md:22`)]
  - `:open` matches a `details` or `dialog` carrying `open` (was: never) [`open`: 2 standing · 2 no-change (`security-plan.md:182` `Connection::open`, `security-plan-detail/api-security.md:58` `webbrowser::open` — other tokens); no master states `:open` — a new fact for its owner]
  - `::before`/`::after` vote in flow classification [`pseudo`: 1 standing · 1 no-change (`design-system-detail/motion.md:58`, todomvc's `::after` animation); no master states the vote — a new fact]
  - a text input focused before its editor exists enables IME at the editor's construction [`ime`: 16 standing · 2 read at P2 (`a11y-plan-detail/keyboard-navigation.md:23`, `screen-reader-support.md:98`, the enable reading) · 14 no-change (IME event plumbing: `architecture.md:80`, `cross-cutting-patterns.md:34`, `established-decisions.md:131`, `standard-contracts.md:35`, `:47`, `:84`, `:96`, `security-plan.md:332`, `mocking-stubbing-discipline.md:11`, `log-coverage.md:74`, `a11y-plan.md:211`, `keyboard-navigation.md:48`, `:61`, `screen-reader-support.md:99`)]
  - the windowed shell routes redraw requests through its event loop [`redraw`: 29 standing · 4 read at P2 (`architecture.md:80`, `cross-cutting-patterns.md:21`, `established-decisions.md:131`, `standard-contracts.md:96` — the shell and its provider) · 25 no-change (the `focus_redraw` target, `requires_redraw`, the mutator's and net's redraw requests, `ci-integration.md:8` @c1328 and `standard-contracts.md:279` @c1181 included)]
  - blitz-vibey-script's runtime and element bindings split into modules [`vibey-mods`: 44 standing · 44 citation rows into `runtime.rs` / `dom/element.rs`, whose cited code moved to the new modules — the citation sweep's, a row whose code left its file re-pointed by hand at P2]
  - the uncaught-JS-error site and the `js_console` bridge: `runtime.rs` → `runtime/events.rs:21` (`report_js_error`) [`jserr`: 10 standing · 3 re-pointed (`architecture.md:269`, `architecture-detail/occupied-resources.md:52`, `obs-plan-detail/log-coverage.md:101`) · 7 no-change (target and print rules: `security-plan.md:354`, `:386`, `logging-monitoring.md:8`, `obs-plan.md:253`, `log-coverage.md:58`, `pii-scrubbing-compliance.md:8`, `:19`)]
  - the WPT pin `9d8aceec0dd…` → `d4b721171c1…` (`6fee0db25`) [`wpt-pin`: 2 standing · 2 no-change (`test-plan.md:314` names the file; `test-plan-detail/test-strategy.md:46` the pin a 2026-10-10 run read, as measured then)]
  - `publish-browser.yml` gains 15 lines at 126 and 153-166 (upstream-only, repository-guarded) [`publish`: 22 standing · 22 citation rows (the citation sweep's); the upstream-only and guard claims no-change]
  - the silence census gains `pseudo_element_flow_classification` (0 env · 0 print · 0 subscriber) [`silence`: 10 standing · 1 extended (obs-plan §3's census sites, `obs-plan.md:90`–`:97`, where P2 places the file) · 9 no-change]
  - the per-sync lock record: this sync's `Cargo.toml` · `Cargo.lock` · `deny.toml` byte-identical [`lockrec`: 24 standing · 4 extended (`security-plan-detail/dependency-security.md:9`, `:20`, `:21`, `:22`, the sync records) · 1 no-change (`architecture.md:308`, the pins standing) · 19 no-change (byte-identical shots, frames and files sharing the token, `registries/contracts/test-plan/session-lifecycle.md:11` @c811 · @c4899 included)]
  - `apps/browser/src/tab.rs` gains 63 lines at 130-136, 177-182 and 294-343 [`browser-tab`: 10 standing · 9 citation rows (the citation sweep's) · 1 no-change (`layout-templates.md:7`)]
  - residual lines 35 (the left-out delta, +6: `a09f46792` · `527394724` · `d3ed4e2c7` · `ccfe15332` · `0980cce5f` · `ebacd68bd`) and 36 (the merge surfaces) — owner moves to Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5's card rows below; P5) [`res-35-36`: 8 standing · 8 read at P2 for an owner a master names (`architecture.md:82`; `established-decisions.md:67` (@c133), `:68`, `:75`, `:76`, `:78`, `:137`; `test-plan-detail/quality-gates-coverage-targets.md:15`)]
  - F2's PROVISIONAL gap markers, now ratified (inputs#I7) [`stand-prov`: 2 standing · 2 changed (`layout-templates.md:20`; `architecture-detail/standard-contracts.md:128`) — each drops]
  - the ten further design-system stand lines (inputs#I7) [`stand-ten`: 39 standing · 10 changed in design-system (`design-system.md:218`, `:247`; `design-system-detail/color-palette.md:44`, `:59`, `:71`; `motion.md:34`; `surface.md:10`, `:38`, `:40`; `typography.md:19`) · 2 fold targets (`layout-templates.md:14`, `:15`) · 27 no-change (`layout-templates.md:10` @c1247 · @c1929, `:107`; 25 rows in architecture, security-plan, test-plan, obs-plan and a11y-plan naming `boot_load` · `LOAD_ANSWER_DELAY` · `type-select` — true claims sharing the token); the listing misses two of the directive's lines worded otherwise, located by `git grep -n -E 'seven_guis|stand|load-page|LoadNet|flight-type' -- .andromeda/design-system.md .andromeda/design-system-detail/`: `typography.md:18`, `spacing.md:20`; leaves 19 (the cascade's)]
  - citations into the other files the take moves [`moved-files`: 209 standing · 209 citation rows into `layout/construct.rs`, `mutator.rs`, `node/element.rs`, `stylo.rs`, blitz-shell `lib.rs` · `window.rs`, `dioxus_document.rs`, `apps/browser/src/{main,tab}.rs`, blitz-vibey-script `{document,fetch,lib,dom/element}.rs` — the citation sweep's; claim text unchanged unless a reading moved]
  - CSS properties as `CSSStyleDeclaration` accessors [`cssdecl`: control unfired — `cssdecl · control UNFIRED over the seven masters (bodies and sets) at e795a088`; no master states it — a new fact]
  - `owner-generic`: 2 standing · 2 no-change (`security-plan-detail/input-validation.md:197`, `test-plan-detail/test-scope-summary.md:139`, other chunks' owners)
  - Sweep reach: 24 patterns · 24 moved facts, 24 with a pattern (this report's count) · owner pattern: control unfired (`title · control UNFIRED over the seven masters (bodies and sets) at e795a088`) · by line 2 lines · 2 offsets
  - by-line dispositions (lines over 2 000 chars, per fact): `established-decisions.md:67` × upstream changed · × res-35-36 read at P2; `ci-integration.md:8` × redraw no-change; `standard-contracts.md:279` × redraw no-change; `layout-templates.md:10` × stand-ten no-change (both offsets); `session-lifecycle.md:11` × lockrec no-change (both offsets)
- **Dev-tool versions:** none
- **Harness / gate surface:** `.github/workflows/publish-browser.yml` (upstream-only, repository-guarded, `a7ea9631b` · `b5b163450`); `tests/blitz-tests/tests/all.rs:54` (an unbuilt `mod` line); no `ci-leg.sh`, `agent-run.sh` or CI-script change — the new target picked up by the workspace leg and `run all` with no script change, not by the `a11y` leg or `run stand`
- **Cross-project / external claims:**
  - upstream DioxusLabs/blitz `main` at `ebacd68bd35552e42ed546faa7fa53dfccc109af` (`git log -1 --format=%H upstream/main`, `evidence/before.md`), merge base `23b9b26a9`; the eleven taken and sixteen left out with their measured reasons in `evidence/lock.md`; nothing sent upstream (the founder's standing hold, 2026-10-07/2026-10-10)
  - fork CI: CI#40466575975 green 17/17 on the base `de85792f5`; CI#40487650912 green, checks 17/17, wall 1365 s, on `e795a088c` — read by `ci.py conclusion --sha HEAD --wait 1800` (`evidence/ci.md`); this wrap's commit adds to that tree
  - `inputs.py verify` (this wrap, before the report): `inputs: 7 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 3 · unparsed 0`
  - I1 · message: the operator, 2026-10-27 07:40Z · copy message · n/a — the take-up directive
  - I2 · `../additional/escher-overseer/relays/upstream-sync-epoch5-founder-2026-10-24.md` · copy no-repo · unchanged — the founder's Epoch 5 take, precedent
  - I3 · message: the operator, the P4 dialog, 2026-10-27 09:27Z · copy message · n/a — «Cherry-pick the eleven (Recommended)», noted as the founder's own word
  - I4 · `../additional/escher-overseer/relays/upstream-sync-epoch2-founder-2026-10-27.md` · copy no-repo · unchanged — the founder's «Одиннадцать коммитов (Рекомендую)»
  - inputs#I5 · message: the operator, 2026-10-27 09:52Z · copy message · n/a — his own word «yes» on the P5 card `.andromeda/runs/2026-10-27T07-42-37-phase/p5-card-1.md` (the operator's own word); its `## Beyond the plan` rows, verbatim:
    - **Residual line 35** (the left-out upstream delta) grows by the six new left-out commits: `a09f46792`, `527394724`, `d3ed4e2c7`, `ccfe15332`, `0980cce5f` and `ebacd68bd`. Its owner moves to the existing entry «Upstream sync ahead of the UI graph — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked» (Epoch 3), with this chunk's measurements (research.md §The trial takes).
    - **Residual line 36** (the merge surfaces of the engine-debts fixes, the clock and the select model) is re-read here, and its owner moves to the same Epoch 3 entry.
    - **For «Engine debts» (v030-04)**: the take leaves the transformed-scrolling-box `visible_region` limit unmoved. `0980cce5f`, which would collide with escher's scrolled-box bounds and hit, is not taken — a reading for that entry, not a route insertion.
  - inputs#I6 · message: the operator, at this wrap invocation, 2026-10-27 10:45Z · copy message · n/a — the wrap directive: P1 only, then stop; snap I7; F2's PROVISIONAL marks off; the ten further stand lines moved as the founder chose; forks to the operator by dialog; transcripts and the run dir kept
  - inputs#I7 · `../additional/escher-overseer/relays/stand-rows-founder-2026-10-27.md` · copy no-repo · unchanged — the founder's own word by phone (answered by 09:00:57Z, 2026-10-27): F2 «Подтвердить (Рекомендую)»; the ten further stand lines «Все десять, общие разделить (Рекомендую)»
- **Reverted / negative API facts:** `task_polled_during_render_can_access_document` stood in `7fc9ee3af`'s conflict hunk and was dropped (it is `286874cc1`'s, not taken); upstream's comment "Blitz only generates `input` events…" in `runtime/events.rs` replaced by escher's select-by-id comment (residual line 36's select model)
- **Insufficient fixes (written, kept, not the remedy):** none
- **Spec claims disproved by measurement:** the plan's Implementation notes and research.md §The trial takes say the trial worktree `target/tmp/trial-wt` is detached at shape B `fb1f87191`; measured detached at shape A's `02b24fccf` (`evidence/before.md`, outcome-2026-10-27-upstream-sync-engine-debts.md) — a chunk-document claim, no master site (no master names the worktree); its removal is the founder's
- **Expected amendments (from plan):**
  1. architecture §Project Intent (`Upstream sync:`) — carried: Cross-project bullet + Counts `upstream`; search `upstream` 17 standing (architecture: `architecture.md:287`, `project-intent.md:9`)
  2. architecture §Established Decisions ([DOM semantics] `:open`; [Unsupported features] the pseudo vote; [JavaScript] the runtime's modules and the `file:` narrowing; [Shell and windowing] the redraw route) and §Existing Scopes — carried: Symbols / APIs + Counts `open` (0 true sites — new), `pseudo` (0 true sites — new), `fetcher` (architecture 5 rows), `redraw` (architecture 4 read at P2), `vibey-mods` (architecture rows among 44), `cssdecl` (control unfired — new)
  3. architecture §Occupied Resources → Environment variables, security-plan §Secret Management (environment values), `.claude/rules/security.md` §Secrets — carried: Symbols / APIs (the `TMPDIR` site) + Counts `tmpdir` (architecture 2 changed, security-plan 1 changed, the rule file a leaf)
  4. security-plan §Threat Model Summary and §Input Validation (script fetch), §Dependency Security's per-sync lock record — carried: Counts `fetcher` (security-plan `threat-model-summary.md:18` changed) and `lockrec` (`dependency-security.md` 4 extended); Dependencies bullet
  5. obs-plan §3 (silence census), §6 · §7 (the JS-error site and `js_console` re-pointed); test-plan §2 (147 → 148, 146 → 147), §2 Script tests (27 → 30), §9 (tallies), §10 (the `keyboard_compat` exclusion standing) — carried: Counts `silence`, `jserr`, `vibey-mods` (obs-plan rows), `bt-files`, `dom-rs`, `ws-tally`, `run-all`; §10 not carried: unchanged — `keyboard_compat.rs` stays and both llvm-cov lines keep its exclusion (gate entry `test -f … && grep -c …` green, last line 2), no master value moves
  6. a11y-plan §5 · §6 (`:open` on `details`, `summary` activation unchanged) and §7 (IME on a focused input's editor construction) — carried: Counts `open` (0 a11y-plan sites — new) and `ime` (a11y-plan 2 read at P2)
  7. every master's citations into a moved file re-pointed by the take's line map — carried: Counts `vibey-mods` (44) and `moved-files` (209), `dom-rs` (6), `publish` (22), `browser-tab` (9) citation rows — the P2 citation sweep's
  - beside the plan's list, the operator directive (inputs#I6, on the founder's word inputs#I7) — carried: Counts `stand-prov` and `stand-ten`; see Decisions & corrections for what its form carries
- **Coverage of new surfaces:**
  - `document_may_load` / the `file:` refusal (script fetch, remote pages included) → validation scheme-and-document check✓ · instrumentation n/a (no log site) · PII n/a · tests integ (`tests/dom.rs` `remote_document_cannot_load_file_urls`, `file_document_can_load_sibling_file_urls`; macOS and Windows CI the `file:` URL witnesses) · a11y n/a · tokens n/a
  - CSS property accessors on `CSSStyleDeclaration` (the registered script-to-DOM crossing) → validation names from Stylo's property list✓ · instrumentation n/a · PII n/a · tests unit (`tests/dom.rs` `style_property_accessors`) · a11y n/a · tokens n/a
  - `:open` on `details` / `dialog` (every document) → validation n/a · instrumentation n/a · PII n/a · tests ✗ (the commit adds no test; `details_element` and the a11y leg green) · a11y `summary` activation unchanged, n/a for focus/kbd · tokens n/a
  - IME enabled at a focused input's editor construction → validation n/a · instrumentation n/a · PII n/a · tests unit (`input_focused_on_mount_enables_ime`) · a11y IME (a11y-plan §7)✓ · tokens n/a
  - pseudos voting in flow classification → tests integ (`pseudo_element_flow_classification` 3, both layout modes not asserted by the file; `incremental_oracle` green) · others n/a
  - the shell's redraw route → tests smoke (windowed `seven_guis_native` boot, `service.name=seven_guis`) + `focus_redraw` green · instrumentation n/a · others n/a
  - the moved uncaught-JS-error site (`runtime/events.rs:21`) → instrumentation `tracing::error!` feature-gated, engine target `blitz_vibey_script`✓ · PII redacted✓ (the sink prints safe fields, message `[redacted]`) · tests `host_log` · `mcp_log` both builds
  - the browser tab lens (upstream-only app) → tests unit (`tab_lens_survives_webview_props_replacement`) · others n/a

## Deviations from intent
- The CI read (operator entry 34) ran `--wait 1800`, the operator's value in place of the plan's 2400 (`evidence/ci.md`, the operator's direction in that session); it read green at 1365 s wall.
- The trial worktree reads shape A (`02b24fccf`), not shape B as the plan's notes say; left as it is, its removal the founder's (outcome file).
- scope record: none — `gate.py scope` clean, 0 recorded.

## Decisions & corrections
- The take is the founder's own word, «Одиннадцать коммитов (Рекомендую)» (inputs#I4, relayed verbatim by the overseer; inputs#I3, the operator's dialog answer and note), ratifying for this take the playbook pattern «An engine behaviour change for every document»: `:open`, the pseudo vote, IME-on-construct, the `file:` refusal, the CSS accessors, the redraw route.
- `cee6b8731` is recorded as a narrowing, not a boundary widening; `efdc5472a`'s accessors a realization on the registered script-to-DOM crossing (plan Provenance, per the 2026-10-06 precedent).
- Every cherry-pick ran `git -c rerere.enabled=false` (rerere is on for this host); `.git/rr-cache` read 16 entries before and after (`evidence/take.md`).
- The three one-hunk resolutions are the plan's own (`evidence/take.md` §1–§3); no further gate-driven edit.
- The operator's P5 «yes» (inputs#I5) on card 1 places residual lines 35 and 36 with Epoch 3's «Upstream sync ahead of the UI graph» and gives «Engine debts» (v030-04) the `visible_region` reading — P5's to land.
- The founder's word on the stand rows (inputs#I7, his own words by phone, relayed for this wrap by the operator, inputs#I6): F2 «Подтвердить (Рекомендую)» — the nine rows' move stands and its PROVISIONAL marks come off; «Все десять, общие разделить (Рекомендую)» — the eight stand-only lines go to layout-templates' `values · desktop-native-screens` block with values unchanged, five folding into what it already states and three new; the two rows shared with todomvc keep todomvc and drop seven_guis; the subjects line drops `desktop-native-stand-load-page`. Passed over: «Откатить» · «Только восемь, общие оставить» · «Ничего не переносить».
- What the wrap's form carries of it, measured at HEAD `e795a088c`:
  - The master-body marks: two OPEN gap markers owned by F2 — `layout-templates.md:20` and `architecture-detail/standard-contracts.md:128` — drop at P2 (classification-contract rules 6–7: the gap marker drops, the sidecar entry carries the ratification).
  - The sidecar marks: six entries carry «PROVISIONAL» in heading or Why — `a11y-plan-amendments.md:500`, `architecture-amendments.md:1289`, `design-system-amendments.md:209`, `:219`, `:234`, `layout-templates-amendments.md:225`. A sidecar is append-only history, rewritten only by the epoch-close consolidation (sidecar-contract §Entry form, §Consolidation), and a superseded provisional record is kept, never reworked (gate-contract §Scope, *Whose word*). The form carries the ratification as a new entry per touched doc, naming F2 ratified by the founder's own word; the six old entries keep their words. Literal removal from them is not carried.
  - `.andromeda/master-route.md:80` reads «(PROVISIONAL, ten further lines left for the founder)»; the wrap's one master-route write is the status flip, so that record keeps its words. Not carried.
  - The relay's coordinates were read at `316c6d25b` (`escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md` (b)), before the nine rows left. At HEAD, by content: `color-palette.md:45` → `:44`, `:60` → `:59`, `:72` → `:71`, `design-system.md:220` → `:218`, the subjects line `:247`; `typography.md:18`, `:19`, `spacing.md:20`, `surface.md:10`, `:38-40`, `motion.md:34` unmoved. On layout-templates, `:13` reads blank at HEAD. By content the load-page facts (`surface.md:38-40`, `motion.md:34`) fold into the served-screens bullet `:14`, and the select facts (`surface.md:10`, `typography.md:19`, `spacing.md:20`) into the mode-region bullet `:15`.
  - For Validate: layout-templates `:15` and `:16` already state `#1a1a1a` (and `:16` the page `#f5f5f5`), so `color-palette.md:59` ("seven_guis text `#1a1a1a`"), which the word names among the three new, restates a value the landing holds. `typography.md:18` (sans-serif, home title 700, base 14px, title 36px) and `color-palette.md:71` (invalid `#e53e3e` / `#fff5f5` / `#c53030`) are stated nowhere in layout-templates (`grep -n -F` over the file: 0 lines each).
  - For Validate: the relay says that afterwards design-system holds no stand fact. Outside the ten, design-system still holds five pointer rows to layout-templates (`border-radius.md:12`, `color-palette.md:27`, `depth-strategy.md:10`, `spacing.md:19`, `typography.md:31`), `surface.md:130-131` (`escher-session` serves six tasks, Cells and the load page among them), `motion.md:8` (no transitions observed in seven_guis styles) and `motion.md:33` · `design-system.md:129` (the driver's load wait). The founder's word names the ten alone.
- Sweep hazards: `:open\b` matches `Connection::open` and `webbrowser::open` (2 false rows); `byte-identical` is shared by 19 unrelated rows; the bare `TMPDIR` alternative matches `CARGO_TARGET_TMPDIR` (7 rows); the design-system stand lines `typography.md:18` and `spacing.md:20` carry neither `seven_guis /`-style labels nor `type-select`, so a token list for the ten misses them.
- Guard refusals: in implement, bash-guard refused a `cd` into the skill's references dir (re-issued with an absolute path) and the stamp-ahead hook caught an estimated time in `evidence/take.md`, corrected from `date -u` (outcome file); at this wrap, bash-guard refused a `cd` into the chunk dir (re-issued with absolute paths).

## Outcome
- Acceptance, re-asserted against the diff:
  - (arch) The take is exact — met: the exact-take entry no output, file identity 0 differing, `runtime/events.rs` `+2 -2`, the comment found once; the three resolutions in `evidence/take.md`; the eleven trailers ride `e795a088c`.
  - (arch, security) The pins stay — met: the preservation guard exit 0; no git dependency; `evidence/lock.md` records the lock unchanged and the sixteen left out with reasons.
  - (security) The audit is green and the script fetcher narrows — met: `advisories ok, licenses ok`, every standing ignore encountered, none added; `tests/dom.rs` 30 passed; `document_may_load` refuses a non-`file:` document's `file:` load by `fetch()`, `import` and `<script src>`, the fetcher still taking only `file:` and `data:`.
  - (tests) Upstream's test lands as a target — met: 3 passed, its own target, no `Running tests/all.rs` line, 148 files, `BlitzTestsTargetsTest` green inside `fast`.
  - (tests) The local gates pass — met: `fast` and `doc` green; tally 209 · 1125 · 0 · 12 (`evidence/counts.md`); dioxus-native-dom lib 82, blitz-dom lib 106.
  - (a11y, design, layouts) The a11y and stand readings hold — met: a11y `lines 14 passed 85 failed 0`; `run stand` 205 · 0 · 7 over 50 files; no stand figure moved.
  - (arch, a11y, security) Residual line 36's merge surfaces keep their owner — met: the integration entry 23 targets 125 passed; `evidence/readings.md` re-reads every row by file with its owner `.andromeda/residuals.md:36` quoted; nothing sent upstream.
  - (obs, security) The sink allowlist re-check holds — met: the census's two named hits; 0 wall-clock reads outside `clock.rs`; `host_log` and `mcp_log` green under both builds; the new test 0 · 0 · 0; `packages/escher-telemetry/` unchanged.
  - (tests, CI) Fork CI is green on the pushed take — met: CI#40487650912 `verdict: green`, 17/17; per-OS rows explained by name (`evidence/ci.md`).
  - `verification-matrix.json#v030-03` — met on the diff: the founder's take, the merge-surface owners, every leg green; status `implemented`, P7.3's flip.
- Gates (implement's run `.andromeda/runs/2026-10-27T09-58-03-implement/`, 35 entries: 30 green · 1 recorded · 0 red, the 4 operator entries then run by the operator pass), by `run`:
  - the exact-take `diff <(git diff … de85792f5 …) <(git show … eleven …)` — green, exit 0, no output
  - the file-identity `{ git show e6f4615ec…:…runtime.rs | cmp … } | grep -c differs` — green, exit 1, last line 0
  - `git diff --numstat efdc5472a… -- …runtime/events.rs` — green, `+2 -2`
  - `grep -c -F 'Blitz generates a `change` event for a `select` alone (dispatched' …runtime/events.rs` — green, 1
  - the preservation guard `git diff --quiet de85792f5… -- Cargo.toml Cargo.lock deny.toml packages …` — green, exit 0
  - `grep -c -E 'DioxusLabs/taffy|DioxusLabs/stylo|git = ' Cargo.toml` — green, exit 1, 0
  - the fixed-sha census `git show --format= {eleven} -- 'packages/*.rs' | grep -c -E …` — green, last line 2 (both named, `evidence/sink.md`)
  - `grep -rlE 'Instant::now\(\)|SystemTime::now\(\)' packages/blitz-dom/src | grep -vc 'clock\.rs$'` — green, exit 1, 0
  - `test -f …keyboard_compat.rs && grep -c -F -e "--ignore-filename-regex 'keyboard_compat\.rs\$'" .github/scripts/ci-leg.sh` — green, 2
  - `cargo test -p blitz-tests --locked --test pseudo_element_flow_classification` — green, 3 passed
  - `cargo test -p blitz-vibey-script --locked --test dom` — green, 30 passed
  - `cargo test -p dioxus-native-dom --locked --lib` — green, 82 passed
  - `cargo test -p blitz-dom --locked --lib` — green, 106 passed
  - the 23-target integration `cargo test -p blitz-tests --locked --test hit_clipped_at_scrolling_box …` — green, 125 passed, 0 failed
  - `bash .github/scripts/ci-leg.sh fast` — green
  - the workspace tally `grep -E '^test result:' target/ci-logs/test.log | awk …` — green, `lines 209 passed 1125 failed 0 ignored 12`
  - every blitz-tests file ran as its own target — green, 0 absent
  - `ls tests/blitz-tests/tests/*.rs | wc -l` — green, 148
  - the CI-scripts tally `grep -E '^Ran [0-9]+ tests|^OK$|^FAILED' target/ci-logs/ci-scripts.log` — recorded: `Ran 108 tests` · `OK` (green by its own verdict inside `fast`)
  - `bash .github/scripts/ci-leg.sh doc` — green
  - `bash .github/scripts/ci-leg.sh audit` — green, `advisories ok`, `licenses ok`
  - `bash .github/scripts/ci-leg.sh a11y` — green
  - the a11y tally — green, `lines 14 passed 85 failed 0`
  - `bash scripts/agent-run.sh boot` — green
  - `bash scripts/agent-run.sh run stand` — green, run.end 205 · 0 · 7
  - `ls tests/blitz-tests/tests/stand_*.rs | wc -l` — green, 50
  - `bash scripts/agent-run.sh run all` — green, run.end 633 · 0 · 12
  - `cargo test -p seven_guis --locked` — green, 12 lines, 41 passed
  - `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` — green, exit 1, 0
  - `cargo build -p seven_guis --bin seven_guis_native --locked` — green
  - smoke `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124` — green, `WAYLAND_DISPLAY` set, `service.name=seven_guis` logged
  - operator `gate.py hygiene` (`leg = 'operator'`) — green: `hygiene: clean — read 579 …` (`evidence/ci.md`)
  - operator `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.3.0` (`leg = 'operator'`) — green, 169.64 s, the push `de85792f5..e795a088c` (`evidence/ci.md`)
  - operator `ci.py conclusion --sha HEAD --wait 2400` (`leg = 'operator'`, run with `--wait 1800`) — green: `e795a088c5b4 verdict: green · checks 17/17 · wall 1365 s · runs CI#40487650912 completed/success` (`evidence/ci.md`)
  - operator per-OS tallies `gh run view <id> …` (`leg = 'operator'`, `expect = []`) — recorded, `--id 40487650912`: the six job rows above, every move the take's by name, no red (`evidence/ci.md`)
- Watches: none folded.
- Outcome basis: the operator pass ran — Setup 4's commit list is `e795a088c` alone, and the final HEAD's CI run CI#40487650912 is recorded in `evidence/ci.md` (green, 17/17); implement's persisted report `.andromeda/runs/2026-10-27T09-58-03-implement/outcome-2026-10-27-upstream-sync-engine-debts.md` (the one implement dir holding this marker's gate trail) is the basis for the take, the gates and the census; no plan revision or operator directive between implement and this report beyond the operator pass's `--wait 1800`.
- Process hygiene: implement P4's census `census: none running` (own tree excluded: 8); re-measured at this wrap, `gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args blitz-tests --args agent-run` → `census: none running` (own tree excluded: 8). The registered trial worktree `target/tmp/trial-wt` stands (a git worktree, no process).
## Addendum — resumed 2026-10-27
- inputs#I8 · message: the operator, the P2 Validate card dialog, 2026-10-27; Q3 the founder by phone, relayed by the operator · copy message · n/a — Q1 «Confirm (Recommended)»: F2's two gap markers drop, one new sidecar entry per doc naming F2 ratified; Q2 «Fold into :16 (Recommended)»: the stand's text `#1a1a1a` (`color-palette.md:59`) lands as no new line, `layout-templates.md:16` already stating it; Q3 the founder's own word «Оставить как есть (Рекомендую)»: the stand facts design-system holds outside the ten (the pointer rows, `surface.md:130-131`, `motion.md:8`, `motion.md:33` · `design-system.md:129`) stay, settled
  - bears on **Decisions & corrections**: adds to it — of the three new lines the founder's word names, two land (`typography.md:18`, `color-palette.md:71`), the third folding into `:16`; the relay's «afterwards, design-system holds no stand fact» is settled by the founder as the ten alone
- I7 · unchanged → drifted — DRIFT: the live relay gained a section «The remaining stand mentions (asked at the upstream-sync wrap's P2, answered by 12:0xZ by phone)» recording the founder's «Оставить как есть (Рекомендую)», passed over «Вынести и их» — the same word inputs#I8 carries; the snapped lines 1–22 unchanged
- inputs.py verify, re-fired: `inputs: 8 entries — unchanged 2 · drifted 1 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 1 · unparsed 0`; moved since the report: drifted 0 → 1 (I7), uncited 3 → 1 (I8, cited by this addendum)
- inputs#I9 · message: the operator, the P5 route-resolve dialog, 2026-10-27 · copy message · n/a — the owner of the three upstream behaviours holding no check of their own (`:open`, the windowed redraw route, `pseudo_element_flow_classification`'s one layout mode) and of the misnamed `remove_attribute` at `architecture-detail/standard-contracts.md:78` is «Engine debts», each a CARRY; escher pauses after this wrap on the founder's word, the CARRYs waiting on that entry
  - bears on **Outcome**: adds to it — the gaps' owner, P5's to pin
## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: de85792f (the parent of the oldest pre-CI commit e795a088) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/publish-browser.yml — added 15 line(s) in 2 range(s)
added: 126 · 153-166
### apps/browser/src/main.rs — added 2 line(s) in 1 range(s)
added: 192-193
### apps/browser/src/tab.rs — added 63 line(s) in 3 range(s)
added: 130-136 · 177-182 · 294-343
  - 178-182 @179 «let tab = tabs»
- 295-343 @296 «mod tests {»
  - 301-342 @302 «fn tab_lens_survives_webview_props_replacement() {»
### packages/blitz-dom/src/layout/construct.rs — added 12 line(s) in 3 range(s)
added: 313-317 · 860 · 867-872
### packages/blitz-dom/src/mutator.rs — added 8 line(s) in 2 range(s)
added: 368-371 · 520-523
### packages/blitz-dom/src/node/element.rs — added 10 line(s) in 2 range(s)
added: 488-489 · 543-550
  - 543-549 «pub fn flush_open_state(&mut self) {»
### packages/blitz-dom/src/stylo.rs — added 1 line(s) in 1 range(s)
added: 510
### packages/blitz-shell/src/lib.rs — added 10 line(s) in 3 range(s)
added: 90 · 93-98 · 104-106
### packages/blitz-shell/src/window.rs — added 2 line(s) in 1 range(s)
added: 193-194
### packages/blitz-vibey-script/src/document.rs — added 11 line(s) in 4 range(s)
added: 16 · 163-169 · 210-211 · 222
### packages/blitz-vibey-script/src/dom/element.rs — added 247 line(s) in 25 range(s)
added: 1 · 3-5 · 7 · 9-17 · 23-29 · 33-41 · 47-48 · 54-76 · 82-83 · 89-97 · 103-111 · 117-118 · 125 · 129 · 133-141
       148 · 153-160 · 197-273 · 277-278 · 284-295 · 299-324 · 329-336 · 350-372 · 377 · 384
  - 23-29 «define_accessor(»
  - 58-64 «define_accessor(»
  - 65-71 «define_accessor(»
  - 197-203 «define_accessor(»
  - 204-210 «define_accessor(»
  - 211-217 «define_accessor(»
  - 218-224 «define_accessor(»
  - 225-231 «define_accessor(»
  - 232-238 «define_accessor(»
  - 239-245 «define_accessor(»
  - 246-252 «define_accessor(»
  - 253-259 «define_accessor(»
  - 260-266 «define_accessor(»
  - 267-273 «define_accessor(»
  - 301-307 «define_method(»
  - 309-315 «define_method(»
  - 316-322 «define_accessor(»
  - 350-356 «define_method(»
  - 357-363 «define_method(»
  - 365-371 «define_method(»
### packages/blitz-vibey-script/src/dom/element/attributes.rs — new file · 382 line(s)
- 10-13 @11 «pub(crate) fn attr_name(local: &str) -> QualName {»
- 17-26 «pub(crate) fn read_attr(ctx: &DomCtx, node_id: NodeId, name: &str) -> Option<String> {»
  - 21-25 «element»
- 28-31 «pub(crate) fn write_attr(ctx: &DomCtx, node_id: NodeId, name: &str, value: &str) {»
- 33-36 «pub(super) fn clear_attr(ctx: &DomCtx, node_id: NodeId, name: &str) {»
- 38-42 «pub(super) fn attr_getter(name: &str, this: &JsValue, context: &mut Context) -> JsResult<JsValue> {»
- 44-55 «pub(super) fn attr_setter(»
- 59-72 «pub(super) fn get_attribute(»
  - 66-67 «let name = to_rust_string(args.first().unwrap_or(&JsValue::undefined()), context)?»
  - 68-71 «match read_attr(&ctx, node_id, &name) {»
- 74-87 @75 «fn read_attrs(ctx: &DomCtx, node_id: NodeId) -> Vec<(String, String)> {»
  - 77-86 «doc.get_node(node_id)»
- 89-101 «pub(super) fn get_attribute_names(»
  - 96-99 «let names: Vec<JsValue> = read_attrs(&ctx, node_id)»
- 103-128 @106 «pub(super) fn attributes(»
  - 113-125 «let items: Vec<JsValue> = read_attrs(&ctx, node_id)»
- 130-142 «pub(super) fn set_attribute(»
  - 137-138 «let name = to_rust_string(args.first().unwrap_or(&JsValue::undefined()), context)?»
- 144-155 «pub(super) fn remove_attribute(»
  - 151-152 «let name = to_rust_string(args.first().unwrap_or(&JsValue::undefined()), context)?»
- 157-167 «pub(super) fn has_attribute(»
  - 164-165 «let name = to_rust_string(args.first().unwrap_or(&JsValue::undefined()), context)?»
- 171-174 «pub(super) fn get_id(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 175-177 «pub(super) fn set_id(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 179-186 «pub(super) fn get_class_name(»
- 187-193 «pub(super) fn set_class_name(»
- 195-200 @198 «fn is_stylesheet_owner_tag(tag: &LocalName) -> bool {»
- 202-215 «pub(super) fn get_disabled(»
  - 209-211 «if element_local_name(&ctx, node_id).is_some_and(|tag| is_stylesheet_owner_tag(&tag)) {»
  - 212-214 «Ok(JsValue::from(»
- 217-241 «pub(super) fn set_disabled(»
  - 225-234 «if let Some(tag) = element_local_name(&ctx, node_id) {»
  - 235-239 «if disabled {»
- 243-251 «pub(super) fn get_hidden(»
- 253-267 «pub(super) fn set_hidden(»
  - 261-265 «if hidden {»
- 275-279 «fn element_local_name(ctx: &DomCtx, node_id: NodeId) -> Option<LocalName> {»
- 281-287 «fn dimension_attr_default(tag: &LocalName, attr: &str) -> Option<u32> {»
  - 282-286 «match &**tag {»
- 289-305 @291 «fn parse_unsigned_long_attr(value: Option<String>, default: u32) -> u32 {»
  - 292-304 «value»
- 307-318 «fn dimension_getter(attr: &str, this: &JsValue, context: &mut Context) -> JsResult<JsValue> {»
  - 310-312 «let Some(tag) = element_local_name(&ctx, node_id) else {»
  - 313-315 «let Some(default) = dimension_attr_default(&tag, attr) else {»
- 320-357 «fn dimension_setter(»
  - 328-330 «let Some(tag) = element_local_name(&ctx, node_id) else {»
  - 331-342 «let Some(default) = dimension_attr_default(&tag, attr) else {»
  - 343-348 @345 «let number = args»
  - 349-354 «let value = if number.is_finite() {»
- 359-361 «pub(super) fn get_width(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 362-368 «pub(super) fn set_width(»
- 369-375 «pub(super) fn get_height(»
- 376-382 «pub(super) fn set_height(»
### packages/blitz-vibey-script/src/dom/element/form_controls.rs — new file · 298 line(s)
- 9-16 «pub(super) fn get_placeholder(»
- 17-23 «pub(super) fn set_placeholder(»
- 25-32 «pub(super) fn get_type(»
- 33-39 «pub(super) fn set_type(»
- 41-51 «pub(super) fn get_autofocus(»
  - 48-50 «Ok(JsValue::from(»
- 52-66 «pub(super) fn set_autofocus(»
  - 60-64 «if value {»
- 68-87 «pub(super) fn get_value(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 71-85 «let value = {»
- 89-95 «pub(super) fn set_value(»
- 97-116 «pub(super) fn get_checked(»
  - 104-114 «let checked = {»
- 118-134 «pub(super) fn set_checked(»
  - 126-132 @127 «write_attr(»
- 143-149 @144 «fn byte_to_utf16(text: &str, byte_offset: usize) -> usize {»
  - 145-148 «text[..byte_offset.min(text.len())]»
- 151-161 @152 «fn utf16_to_byte(text: &str, utf16_offset: usize) -> usize {»
  - 154-159 «for (byte_offset, ch) in text.char_indices() {»
- 163-172 @164 «fn selection_utf16_range(doc: &blitz_dom::BaseDocument, node_id: NodeId) -> Option<(usize, usize)> {»
  - 168-171 «Some((»
- 174-187 @175 «fn set_selection_utf16_range(»
  - 181-186 «doc.with_text_input(node_id, |mut driver| {»
- 189-203 «pub(super) fn get_selection_start(»
  - 199-202 «Ok(match selection_utf16_range(&doc, node_id) {»
- 205-219 «pub(super) fn get_selection_end(»
  - 215-218 «Ok(match selection_utf16_range(&doc, node_id) {»
- 221-239 «pub(super) fn set_selection_start(»
  - 228-232 «let start = args»
  - 235-237 «if let Some((_, end)) = selection_utf16_range(&doc, node_id) {»
- 241-259 «pub(super) fn set_selection_end(»
  - 248-252 «let end = args»
  - 255-257 «if let Some((start, _)) = selection_utf16_range(&doc, node_id) {»
- 261-282 «pub(super) fn set_selection_range(»
  - 268-272 «let start = args»
  - 273-277 «let end = args»
- 286-291 «pub(super) fn focus(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 293-298 «pub(super) fn blur(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
### packages/blitz-vibey-script/src/dom/element/geometry.rs — new file · 251 line(s)
- 13-27 @16 «fn layout_value(»
- 29-44 @33 «fn offset_value(»
- 46-52 «pub(super) fn offset_width(»
- 54-60 «pub(super) fn offset_height(»
- 62-68 «pub(super) fn offset_left(»
- 70-76 «pub(super) fn offset_top(»
- 78-105 @80 «fn client_size(»
  - 90-92 «let Some(node) = doc.get_node(node_id) else {»
  - 93-103 «let value = if node_id == doc.root_element().id {»
- 107-118 «pub(super) fn client_width(»
  - 112-117 «client_size(»
- 120-137 @122 «pub(super) fn offset_parent(»
  - 129-135 «let parent_id = {»
- 142-148 «pub(super) fn client_top(»
- 150-158 «pub(super) fn client_left(»
  - 155-157 «layout_value(this, context, |node| {»
- 160-171 «pub(super) fn client_height(»
  - 165-170 «client_size(»
- 173-179 «pub(super) fn scroll_width(»
- 181-187 «pub(super) fn scroll_height(»
- 189-201 @190 «fn make_rect_object(context: &mut Context, x: f64, y: f64, width: f64, height: f64) -> JsObject {»
  - 191-200 «ObjectInitializer::new(context)»
- 203-207 @205 «fn node_has_boxes(doc: &blitz_dom::BaseDocument, node_id: NodeId) -> bool {»
- 209-223 «pub(super) fn get_bounding_client_rect(»
  - 218-221 «let (x, y, width, height) = match rect {»
- 225-251 «pub(super) fn get_client_rects(»
  - 234-244 @237 «let rects = {»
  - 246-249 «let rect_objects: Vec<JsValue> = rects»
### packages/blitz-vibey-script/src/dom/element/markup.rs — new file · 129 line(s)
- 8-22 «pub(super) fn get_inner_text(»
  - 17-20 «let text = doc»
- 26-32 «pub(super) fn get_style(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 34-48 @36 «pub(super) fn get_sheet(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 38-42 «let sheet = crate::dom::call_js_helper(»
  - 43-47 «Ok(if sheet.is_undefined() {»
- 52-69 «pub(super) fn get_inner_html(»
  - 61-67 «if let Some(node) = doc.get_node(node_id) {»
- 71-89 «pub(super) fn set_inner_html(»
  - 82-86 @84 «for child_id in mutr.child_ids(node_id) {»
- 91-114 @93 «pub(super) fn get_content(»
  - 101-106 «let is_template = ctx»
  - 107-109 «if !is_template {»
- 116-129 «pub(super) fn get_outer_html(»
  - 124-127 «let html = doc»
### packages/blitz-vibey-script/src/dom/element/scrolling.rs — new file · 268 line(s)
- 11-26 @14 «fn current_scroll_offset(doc: &blitz_dom::BaseDocument, node_id: NodeId) -> blitz_dom::Point<f64> {»
  - 15-25 «if doc»
- 28-33 @30 «fn scroll_coord(value: &JsValue, context: &mut Context) -> JsResult<f64> {»
- 35-52 @37 «fn parse_scroll_behavior(value: &JsValue, context: &mut Context) -> JsResult<ScrollBehavior> {»
  - 38-40 «if value.is_undefined() {»
  - 42-51 «match string.as_str() {»
- 54-62 @58 «pub(crate) struct ScrollToArgs {»
- 64-113 @68 «pub(crate) fn parse_scroll_to_args(»
  - 72-78 «if args.len() >= 2 {»
  - 79-112 «match args.first() {»
- 115-124 «pub(super) fn get_scroll_top(»
- 126-135 «pub(super) fn get_scroll_left(»
- 137-150 «pub(super) fn set_scroll_top(»
- 152-165 «pub(super) fn set_scroll_left(»
- 167-185 «pub(super) fn scroll_to_method(»
  - 178-183 «doc.scroll_to(»
- 187-204 «pub(super) fn scroll_by_method(»
  - 197-202 «doc.scroll_by(»
- 206-228 @208 «fn parse_scroll_logical_position(»
  - 213-215 «if value.is_undefined() {»
  - 217-227 «match string.as_str() {»
- 230-268 «pub(super) fn scroll_into_view(»
  - 236-260 @239 «let (behavior, block, inline) = if let Some(options) = arg.as_object() {»
### packages/blitz-vibey-script/src/dom/element/selectors.rs — new file · 133 line(s)
- 9-17 @11 «fn invalid_selector_error(context: &mut Context, selector: &str) -> boa_engine::JsError {»
  - 12-16 «crate::dom::dom_exception(»
- 19-34 «pub(super) fn query_selector(»
  - 29-32 «let result = match result {»
- 36-55 «pub(super) fn query_selector_all(»
  - 46-49 «let matches = match matches {»
  - 50-53 «let wrappers: Vec<JsValue> = matches»
- 57-70 «pub(super) fn matches(»
  - 66-69 «match ctx.doc.borrow().matches_selector(node_id, &selector) {»
- 72-86 «pub(super) fn closest(»
  - 82-85 «match result {»
- 88-108 «pub(super) fn get_elements_by_tag_name(»
  - 95-96 «let tag = to_rust_string(args.first().unwrap_or(&JsValue::undefined()), context)?»
  - 99-101 «let matches = crate::dom::collect_matching_descendants(&ctx.doc.borrow(), node_id, |element| {»
  - 103-106 «let wrappers: Vec<JsValue> = matches»
- 110-133 «pub(super) fn get_elements_by_class_name(»
  - 120-126 «let matches = if class_names.is_empty() {»
  - 128-131 «let wrappers: Vec<JsValue> = matches»
### packages/blitz-vibey-script/src/fetch.rs — added 12 line(s) in 3 range(s)
added: 10-12 · 23 · 40-47
- 40-46 @43 «pub fn document_may_load(document_url: Option<&Url>, url: &Url) -> bool {»
  - 44-45 «url.scheme() != "file"»
### packages/blitz-vibey-script/src/lib.rs — added 1 line(s) in 1 range(s)
added: 43
### packages/blitz-vibey-script/src/runtime.rs — added 23 line(s) in 7 range(s)
added: 1-4 · 9-10 · 16-19 · 21-27 · 65-68 · 112 · 120
### packages/blitz-vibey-script/src/runtime/bootstrap.rs — new file · 209 line(s)
- 14-36 @16 «pub(super) const BOOTSTRAP_JS: &str = concat!(»
- 38-157 «pub(super) fn register_globals(ctx: &DomCtx, base_url: Option<&Url>, context: &mut Context) {»
  - 58-65 @59 «let navigator = ObjectInitializer::new(context)»
  - 73-78 «register_global_fn(»
  - 80-85 «register_global_fn(»
  - 86-91 «register_global_fn(»
  - 99-105 @100 «register_global_fn(»
  - 106-111 «register_global_fn(»
  - 115-120 «register_global_fn(»
  - 143-155 @144 «let css_namespace = ObjectInitializer::new(context)»
- 159-167 «fn register_global(context: &mut Context, name: &str, value: JsValue) {»
  - 160-166 «context»
- 169-194 «fn register_global_accessor(»
  - 177-181 «let getter_fn =»
  - 182-193 «context»
- 196-209 «fn register_global_fn(»
  - 202-208 «context»
### packages/blitz-vibey-script/src/runtime/bootstrap/css.js — new file · 41 line(s)
- 1-41 @4 «if (typeof globalThis.CSS === "object" && typeof CSS.escape !== "function") {»
  - 5-40 «CSS.escape = function (value) {»
### packages/blitz-vibey-script/src/runtime/bootstrap/dom_exception.js — new file · 67 line(s)
- 1-67 «if (typeof globalThis.DOMException !== "function") {»
  - 2-25 «const DOM_EXCEPTION_CODES = {»
  - 26-34 «globalThis.DOMException = class DOMException extends Error {»
  - 35-62 @36 «const LEGACY_CODE_CONSTANTS = {»
  - 63-66 «for (const [name, code] of Object.entries(LEGACY_CODE_CONSTANTS)) {»
### packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js — new file · 98 line(s)
- 1-98 @4 «if (typeof globalThis.fetch !== "function") {»
  - 5-30 «class Headers {»
  - 32-68 «class Response {»
  - 72-97 «globalThis.fetch = function fetch(input, init) {»
### packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js — new file · 15 line(s)
- 1-13 @2 «const fontFaceSet = {»
  - 10-12 «dispatchEvent() {»
### packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js — new file · 133 line(s)
- 1-15 @3 «const makeInterface = (name, proto) => {»
  - 4-6 «const iface = function () {»
  - 9-13 «Object.defineProperty(proto, "constructor", {»
- 23-35 @31 «if (typeof globalThis.Window === "undefined") {»
- 37-48 «for (const name of [»
  - 45-47 «if (typeof globalThis[name] === "undefined") {»
- 49-133 «if (document.documentElement) {»
  - 54-104 @55 «Object.defineProperty(elementProto, "classList", {»
  - 106-132 @107 «Object.defineProperty(elementProto, "dataset", {»
### packages/blitz-vibey-script/src/runtime/bootstrap/node_constants.js — new file · 18 line(s)
- 1-14 «const NODE_CONSTANTS = {»
- 15-18 «for (const [name, value] of Object.entries(NODE_CONSTANTS)) {»
### packages/blitz-vibey-script/src/runtime/bootstrap/style.js — new file · 111 line(s)
- 5-13 «const toKebab = (prop) => {»
- 14-17 @16 «const isCssPropName = (prop) =>»
- 22-24 «const styleIterator = function* () {»
- 26-48 «const installStyleProperties = (proto) => {»
  - 28-46 «for (const name of __blitz_css_property_names()) {»
- 49-111 «globalThis.__blitz_wrap_style = function (native) {»
  - 51-110 «return new Proxy(native, {»
### packages/blitz-vibey-script/src/runtime/bootstrap/stylesheets.js — new file · 545 line(s)
- 1-545 @5 «{»
  - 6-8 «const illegal = () => {»
  - 11-20 «const construct = (cls, init) => {»
  - 22-26 «const data = (obj) => {»
  - 28-62 @30 «const indexedProxy = (target) =>»
  - 63-70 «const defineIterable = (proto) => {»
  - 72-89 «class MediaList {»
  - 91-92 «const makeMediaList = (text) =>»
  - 94-126 «class StyleSheet {»
  - 128-174 @129 «class RuleStyleDeclaration {»
  - 176-181 «const makeRuleStyle = (owner, path, rule) =>»
  - 183-197 «class CSSRuleList {»
  - 199-202 «const makeRuleList = (sheet, owner, path, parentRule) =>»
  - 204-221 «const CSS_RULE_TYPES = {»
  - 223-240 «class CSSRule {»
  - 241-244 «for (const [name, value] of Object.entries(CSS_RULE_TYPES)) {»
  - 246-250 «const attrGetter = (name, convert) =>»
  - 251-259 «const defineAttrs = (cls, attrs) => {»
  - 260-263 «const cssRulesGetter = function () {»
  - 264-280 «const groupingMethods = {»
  - 281-286 «const styleGetter = {»
  - 289-303 «const defineRuleClass = (name, base, mixins, attrs) => {»
  - 306-308 «const CSSConditionRule = defineRuleClass("CSSConditionRule", CSSGroupingRule, [], {»
  - 310-319 «Object.defineProperty(ruleClasses.CSSStyleRule.prototype, "selectorText", {»
  - 321-327 «defineRuleClass("CSSMediaRule", CSSConditionRule, [»
  - 329-332 «defineRuleClass("CSSContainerRule", CSSConditionRule, [], {»
  - 335-337 «defineRuleClass("CSSLayerStatementRule", CSSRule, [], {»
  - 342-348 «defineRuleClass("CSSImportRule", CSSRule, [], {»
  - 352-357 «defineRuleClass("CSSFontPaletteValuesRule", CSSRule, [], {»
  - 359-364 «defineRuleClass("CSSPropertyRule", CSSRule, [], {»
  - 370-400 «defineRuleClass(»
  - 401-416 @403 «const findKeyframeIndex = (keyframesRule, select) => {»
  - 428-429 «const generation = (sheet) =>»
  - 430-432 «const touchSheet = (sheet) => {»
  - 433-436 «const invalidateRules = (sheet) => {»
  - 437-474 «const ruleAt = (sheet, owner, path, parentRule) => {»
  - 476-507 «class CSSStyleSheet extends StyleSheet {»
  - 509-519 «class StyleSheetList {»
  - 525-533 «globalThis.__blitz_sheet_for_node = (node) => {»
  - 534-537 «globalThis.__blitz_style_sheets = () => {»
### packages/blitz-vibey-script/src/runtime/bootstrap/window.js — new file · 45 line(s)
- 1-39 @5 «if (typeof globalThis.history === "undefined") {»
  - 6-19 «const updateLocationFromUrl = (url) => {»
  - 21-38 «globalThis.history = {»
- 41-45 «if (typeof globalThis.queueMicrotask !== "function") {»
  - 42-44 «globalThis.queueMicrotask = function (callback) {»
### packages/blitz-vibey-script/src/runtime/css.rs — new file · 148 line(s)
- 8-18 «pub(super) fn css_property_names(»
  - 13-16 «let names = style::properties::NonCustomPropertyId::iter()»
- 20-27 «pub(super) fn css_property_supported(»
- 29-50 @32 «pub(super) fn parse_transform(»
  - 38-40 «let Some((elements, is_2d)) = blitz_dom::parse_transform_matrix(&value) else {»
  - 41-44 «let elements = boa_engine::object::builtins::JsArray::from_iter(»
  - 45-48 «Ok(boa_engine::object::builtins::JsArray::from_iter(»
- 52-69 @54 «pub(super) fn css_supports(»
  - 60-67 «let supported = if args.len() >= 2 {»
- 71-132 @73 «pub(super) fn css_register_property(»
  - 81-85 «let Some(descriptor) = args.first().and_then(JsValue::as_object) else {»
  - 86-92 «let get = |key: &str, context: &mut Context| -> JsResult<Option<String>> {»
  - 93-97 «let Some(name) = get("name", context)? else {»
  - 99-103 «if inherits.is_undefined() {»
  - 108-113 «let result = ctx.doc.borrow_mut().register_custom_property(»
  - 114-130 «let error = match result {»
- 134-148 «pub(super) fn get_computed_style(»
  - 140-144 «let Some(node_id) = args.first().and_then(node_id_of_value) else {»
### packages/blitz-vibey-script/src/runtime/events.rs — new file · 494 line(s)
- 13-32 @19 «pub(super) fn report_js_error(ctx: &DomCtx, context: &mut Context, what: &str, error: &JsError) {»
  - 22-26 «let already_dispatching = {»
  - 27-29 «if already_dispatching {»
- 34-97 @37 «fn dispatch_error_event(ctx: &DomCtx, context: &mut Context, error: &JsError) {»
  - 38-48 «let listeners: Vec<Listener> = {»
  - 49-53 «let onerror = context»
  - 54-56 «if listeners.is_empty() && onerror.is_none() {»
  - 61-64 «let error_value = error»
  - 65-69 «let message = error_value»
  - 76-83 «for listener in listeners {»
  - 84-96 @85 «if let Some(handler) = onerror {»
- 99-444 «impl ScriptRuntime {»
  - 100-141 @104 «pub fn install_body_onload_attribute(&mut self) {»
  - 143-195 @147 «pub fn dispatch_dom_event(»
  - 197-208 «fn target_is_checkbox_or_radio(&self, node_id: NodeId) -> bool {»
  - 210-374 @212 «fn dispatch_event_inner(»
  - 376-391 @377 «pub fn dispatch_document_event(&mut self, name: &str) -> bool {»
  - 393-443 @394 «pub fn dispatch_window_event(&mut self, name: &str) -> bool {»
- 446-475 «pub(super) fn window_add_event_listener(»
  - 452-453 «let event_type =»
  - 454-457 «let Some(callback) = args»
  - 458-460 «else {»
  - 464-473 «if !listeners»
- 477-494 «pub(super) fn window_remove_event_listener(»
  - 483-484 «let event_type =»
  - 485-487 «let Some(callback) = args.get(1).and_then(|value| value.as_object()) else {»
  - 490-492 «if let Some(listeners) = state.window_listeners.get_mut(&event_type) {»
### packages/blitz-vibey-script/src/runtime/fetch.rs — new file · 66 line(s)
- 9-66 @14 «pub(super) fn fetch_sync(»
  - 22-26 «let input = args»
  - 28-31 «let (base_url, fetcher) = {»
  - 32-35 «let url = match &base_url {»
  - 37-38 «let fetcher =»
  - 40-44 «let result = if document_may_load(base_url.as_ref(), &url) {»
  - 45-55 «let (status, text) = match result {»
  - 57-64 «Ok(JsArray::from_iter(»
### packages/blitz-vibey-script/src/runtime/modules.rs — new file · 132 line(s)
- 16-26 @19 «pub(super) struct BlitzModuleLoader {»
- 28-56 «impl BlitzModuleLoader {»
  - 29-35 «pub(super) fn new(fetcher: Rc<RefCell<Box<dyn ScriptFetcher>>>, base_url: Option<Url>) -> Self {»
  - 37-47 «fn resolve_specifier(&self, referrer: &Referrer, specifier: &str) -> Option<Url> {»
  - 49-51 «fn get(&self, url: &Url) -> Option<Module> {»
  - 53-55 «fn insert(&self, url: Url, module: Module) {»
- 58-92 «impl ModuleLoader for BlitzModuleLoader {»
  - 59-91 «async fn load_imported_module(»
- 94-132 «impl ScriptRuntime {»
  - 95-131 @98 «pub fn eval_module(&mut self, code: &str, url: Option<&Url>) {»
### packages/blitz-vibey-script/src/runtime/timers.rs — new file · 139 line(s)
- 10-38 «impl ScriptRuntime {»
  - 11-14 @12 «pub fn next_timer_deadline(&self) -> Option<Instant> {»
  - 16-37 @17 «pub fn run_due_timers(&mut self) -> bool {»
- 40-66 «fn timer_args(»
  - 44-47 «let Some(callback) = args»
  - 48-50 «else {»
  - 51-54 «let delay_ms = match args.get(1) {»
  - 55-59 «let delay_ms = if delay_ms.is_finite() && delay_ms > 0.0 {»
  - 61-65 «Ok(Some((»
- 68-81 «pub(super) fn set_timeout(»
  - 74-76 «let Some((callback, delay, rest)) = timer_args(args, context)? else {»
- 83-96 «pub(super) fn set_interval(»
  - 89-91 «let Some((callback, delay, rest)) = timer_args(args, context)? else {»
- 98-123 «pub(super) fn request_animation_frame(»
  - 104-107 «let Some(callback) = args»
  - 108-110 «else {»
  - 115-121 «let id = state.timers.add(»
- 125-139 «pub(super) fn clear_timer(»
  - 131-134 «let id = match args.first() {»
  - 135-137 «if id.is_finite() && id >= 0.0 {»
### packages/blitz-vibey-script/src/runtime/window.rs — new file · 223 line(s)
- 12-63 «impl ScriptRuntime {»
  - 13-62 @17 «pub fn sync_named_element_globals(&mut self) {»
- 65-121 «pub(super) fn build_location(base_url: Option<&Url>, context: &mut Context) -> JsValue {»
  - 66-85 «let (href, protocol, host, pathname, search, hash, origin) = match base_url {»
  - 86-120 «ObjectInitializer::new(context)»
- 125-132 «pub(super) fn inner_width(_: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 129-131 «Ok(JsValue::from(»
- 134-141 «pub(super) fn inner_height(_: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
  - 138-140 «Ok(JsValue::from(»
- 143-151 «pub(super) fn device_pixel_ratio(»
- 155-158 «pub(super) fn scroll_x(_: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 160-163 «pub(super) fn scroll_y(_: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {»
- 165-187 @167 «pub(super) fn window_scroll_to(»
  - 176-178 «let Some(root_id) = doc.try_root_element().map(|root| root.id) else {»
  - 180-185 «doc.scroll_to(»
- 189-208 «pub(super) fn window_scroll_by(»
  - 198-200 «let Some(root_id) = doc.try_root_element().map(|root| root.id) else {»
  - 201-206 «doc.scroll_by(»
- 210-223 «pub(super) fn send_message(»
  - 216-220 «let message = args»
### packages/blitz-vibey-script/tests/dom.rs — added 126 line(s) in 2 range(s)
added: 455-488 · 859-950
- 455-487 @456 «fn style_property_accessors() {»
  - 457-485 «let doc = doc_from_html(»
- 860-869 @861 «fn temp_dir_with_files(name: &str, files: &[(&str, &str)]) -> std::path::PathBuf {»
  - 862-863 «let dir =»
  - 865-867 «for (file, contents) in files {»
- 871-892 «const FILE_LOAD_TEST_HTML: &str = r#"»
  - 872-892 «<html><body>»
- 895-902 «const FILE_LOAD_TEST_FILES: &[(&str, &str)] = &[»
  - 896-899 «(»
- 904-928 @905 «fn remote_document_cannot_load_file_urls() {»
  - 910-916 «let mut doc = ScriptDocument::from_html(»
- 930-950 @931 «fn file_document_can_load_sibling_file_urls() {»
  - 936-942 «let mut doc = ScriptDocument::from_html(»
### packages/dioxus-native-dom/src/dioxus_document.rs — added 36 line(s) in 1 range(s)
added: 725-760
  - 725-759 @729 «fn input_focused_on_mount_enables_ime() {»
### tests/blitz-tests/tests/all.rs — added 1 line(s) in 1 range(s)
added: 54
### tests/blitz-tests/tests/pseudo_element_flow_classification.rs — new file · 78 line(s)
- 11-22 «fn layout_doc(html: &str) -> HtmlDocument {»
  - 12-19 «let mut doc = HtmlDocument::from_html(»
- 24-28 «fn rect(doc: &HtmlDocument, selector: &str) -> (f32, f32) {»
- 30-35 «const STYLE: &str = "<style>»
- 37-48 @41 «fn inline_block_pseudo_only_child_exports_line_baseline() {»
  - 42-44 «let doc = layout_doc(&format!(»
- 50-60 @53 «fn inline_block_pseudo_only_child_bottom_aligned() {»
  - 54-56 «let doc = layout_doc(&format!(»
- 62-78 @65 «fn grid_row_grows_by_baseline_shim_of_pseudo_only_items() {»
  - 66-74 «let doc = layout_doc(&format!(»
### wpt/WPT_COMMIT — added 1 line(s) in 1 range(s)
added: 1
