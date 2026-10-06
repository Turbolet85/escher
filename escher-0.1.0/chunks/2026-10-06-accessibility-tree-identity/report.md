# Report — 2026-10-06-accessibility-tree-identity

**Chunk:** Accessibility-tree identity — stable id on every accessibility node, stand controls carrying role and name (v010-03)
**Date:** 2026-10-06T12:58Z
**Commits:** `9285fe75` chore(2026-10-06-accessibility-tree-identity): operator pre-CI commit, for the run this chunk's verdict reads (the only commit since `last_wrap` 2026-10-06T11:50:05Z / `076d74cb`; basis `git log --format='%h %s' 076d74cb..HEAD`)

## Changes (structured — detectors read this)
- **Files** (basis `git diff --name-only 076d74cb` filtered to non-audit paths; `gate.py scope` reads changed 14 · listed 12 · recorded 2):
  - engine: `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/accessibility.rs` · `packages/dioxus-native-dom/src/dioxus_document.rs` · `packages/blitz-shell/src/accessibility.rs` · `packages/blitz-shell/src/window.rs`
  - manifests: `packages/dioxus-native-dom/Cargo.toml` · `packages/dioxus-native/Cargo.toml` (recorded companion) · `tests/blitz-tests/Cargo.toml` (recorded widening) · `Cargo.lock`
  - stand markup: `examples/seven_guis/src/tasks/{timer,crud,flight_booker}.rs`
  - new tests: `tests/blitz-tests/tests/accessibility_names.rs` · `tests/blitz-tests/tests/stand_accessibility_ids.rs`
  - chunk folder: `plan.md` (gate 6's guarded path list amended on the operator's direction, see Deviations), `scope-record.md`, `evidence/{tab-order-base,smoke,gate6-amended,operator-pass}.md`, this report
- **Symbols / APIs:**
  - NEW `blitz_dom::Document::accessibility_tree(&self) -> accesskit::TreeUpdate` — a defaulted trait method, `#[cfg(feature = "accessibility")]` (document.rs:152-160). Its default returns `self.inner().build_accessibility_tree()`. Its rustdoc says a wrapper may override it to enrich nodes.
  - NEW override `impl Document for DioxusDocument { fn accessibility_tree }`, `#[cfg(feature = "accessibility")]` (dioxus_document.rs:288-304). It builds the base tree through `self.inner.borrow().build_accessibility_tree()`, maps `element_ids()` by `NodeId::as_u64()`, and calls `set_author_id(id)` on each tree node whose raw id is in the map. No other node is touched (TextRuns, the document root, the synthetic `Window` `u64::MAX`). The method never indexes the slab with a tree id. It is the only override among the six `Document` implementors.
  - CHANGED `BaseDocument::build_accessibility_tree` (inherent, signature unchanged; its 10 callers are unchanged, per research's code-graph query). Two new name sources:
    - an element whose `aria-label`, trimmed, is non-empty gets `set_label(value)`, citing accname-1.2 §2C (accessibility.rs:88-94);
    - after the visit, every built `<label>` node is resolved through the existing `label_bound_input_element` (a `for`-target `<input>` by id, else the first nested `<input>`). When the bound input has a built node, the label's AccessKit id is pushed onto that input's `labelled_by`, citing HTML-AAM's accessible-name computations (accessibility.rs:29-48).
    - Roles, `hidden`, `TextRun` and keying are unchanged.
  - CHANGED `blitz_shell::AccessibilityState::update_tree(&mut self, doc: &dyn Document)` — it was `&BaseDocument`. It builds with `doc.accessibility_tree()` (blitz-shell/src/accessibility.rs:44-46). Its two callers, both re-threaded, are `View::poll` (window.rs:376-382: `has_changes()` is read through `self.doc.inner()`, the guard is dropped, then `update_tree(&*self.doc)`) and `View::build_accessibility_tree` (window.rs:521-524). application.rs is unchanged. So the windowed platform tree is now built through the document trait, and a `DioxusDocument` window carries the ids.
  - No IPC method, endpoint, port, socket, env var or listener is added (the census gate reads 0 sites).
- **Crates / modules:** no crate added or removed. Changed: blitz-dom, dioxus-native-dom, blitz-shell, dioxus-native (manifest only), seven_guis (markup only), blitz-tests (two new files plus a manifest feature).
- **Dependencies:**
  - dioxus-native-dom gains `accesskit = { workspace = true, optional = true }` (Cargo.toml:39), enabled by its `accessibility` feature, which now reads `["blitz-dom/accessibility", "dep:accesskit"]` (Cargo.toml:18).
  - The only `Cargo.lock` change is `+ "accesskit",` in dioxus-native-dom's dependency list (basis: `git diff -U0 076d74cb -- Cargo.lock | grep -E '^[+-][^+-]'` → that one line). accesskit stays 0.25.0, already in the graph. No version moves.
  - FEATURE wiring:
    - dioxus-native's `accessibility` feature now also forwards to `dioxus-native-dom/accessibility` (packages/dioxus-native/Cargo.toml:30 → `["blitz-shell/accessibility", "blitz-dom/accessibility", "dioxus-native-dom/accessibility"]`).
    - tests/blitz-tests/Cargo.toml:21 now reads `dioxus-native-dom = { workspace = true, features = ["accessibility"] }`.
- **Schema / config:** no config key. No log or scrub shape changed (no log site was added; `run stand` events carry 0 content-named keys, per gate `agent-run.sh logs | … bad=…` → 0).
- **Spec-master edits:** none (no master was touched before this wrap).
- **Counts / qualifiers moved:**
  - workspace tests 444 · 0 · 5 → **454 · 0 · 5**. Basis: `grep -E '^test result' target/ci-logs/test.log`, summed after entry 19's `ci-leg.sh fast` at `9285fe75`; the same figure was read after implement's entry 14.
  - stand checks via `agent-run.sh run stand`: 20 → **25** ok stand events (gate `agent-run.sh logs | …startswith("stand_")` printed 25).
  - The a11y CI leg is unchanged: its three files `accessibility_hidden` · `accessibility_roles` · `focusability_updates` (6 + 6 + 3); the new files ride the workspace test leg.
  - stand_* files: 6 → 7.
  - Accessible-name sources in the tree build: 1 (direct text children) → 3 (+ `aria-label`, + `<label>` association).
  - Stand inputs carrying a name: 0 of 6 → 6 of 6. The 15 stand controls with a role and a name: 9 → 15.
- **Dev-tool versions:** none — no host tool was installed or upgraded.
- **Harness / gate surface:** none. agent-run.sh is unchanged; `run stand` picks up `stand_accessibility_ids.rs` by its `stand_` prefix. CI is unchanged.
- **Cross-project / external claims:**
  - The operator pass's CI run: CI#37465287000 on `9285fe75350a`, `verdict: green · checks 16/16 · wall 692 s` (`ci.py conclusion --sha HEAD --wait 1800`, recorded in `evidence/operator-pass.md`). The verdict was taken on `9285fe75`; this wrap's commit adds the report, specs and audit trail on top.
  - accesskit-0.25.0 `Node::author_id` / `set_author_id` ("A way for application authors to identify this node for automated testing purposes"), read in the cargo registry. dioxus-html-0.7.10: `label { r#for: Id "for" }` (elements.rs:1494-1497) and `aria_label: "aria-label"` (attribute_groups.rs:1614), read in the cargo registry.
  - `inputs.py verify`: `inputs: absent — no external input snapshotted`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **plan.md step 3 (and research §Patterns "Optional accesskit per crate"):** following blitz-shell's optional-accesskit pattern makes the `DioxusDocument` override reach "the windowed app, the harness and any test". Measured false: the workspace pins `dioxus-native-dom` and `dioxus-native` with `default-features = false` (root Cargo.toml:54-55). Before the two manifest edits, nothing in blitz-tests' or seven_guis' graph enabled `dioxus-native-dom/accessibility` (`cargo tree -p blitz-tests -e features -i dioxus-native-dom` → no `feature "accessibility"` line). So `cargo test -p blitz-tests --test stand_accessibility_ids` would have compiled the override out; only `cargo test --workspace` unifies the package's own defaults in. **Disposition:** fixed in-impl by the two recorded manifest edits (the operator's word for the widening); afterwards `cargo tree` shows `dioxus-native-dom feature "accessibility"`.
  2. **a11y-plan.md:55** "the Dioxus crates only forward the feature to blitz". Now false: dioxus-native-dom's `accessibility` also enables `dep:accesskit` and gates its own `accessibility_tree` override, and dioxus-native's `accessibility` forwards to dioxus-native-dom. **a11y-plan.md:14** "forwards to blitz-dom (and blitz-shell in dioxus-native)" is incomplete the same way. Disposition: P2 amendment (a11y-plan §1).
  3. **The seven_guis stand app ships with no AccessKit at all.** seven_guis takes dioxus-native with `default-features = false` (workspace) and never names `accessibility`. Basis: `cargo tree -p seven_guis -e features -i dioxus-native` lists only `prelude` · `system-fonts` · `vello-hybrid` · `woff`, and `cargo tree -p seven_guis -e features -i blitz-shell | grep accessib` → 0. So `just seven_guis` builds no platform adapter, even though CLAUDE.md:44 says "`accessibility` is a default feature" (true per crate, not for the stand binary). Disposition: out of this chunk's scope. On the operator's direction (2026-10-06) it goes on the working route at P5 as a note, not a spec amendment here.
  4. **plan.md gate 6 (the diff guard)** listed `tests/blitz-tests/Cargo.toml` as must-not-change, which (1) makes impossible. Disposition: the guard was amended on the operator's direction (see Deviations); `evidence/gate6-amended.md`.
- **Expected amendments (from plan)** (site basis: `grep -cF` per token over the seven masters plus every `.andromeda/registries/**` file, run at this P1):
  - **arch §Standard Contracts → Dioxus DOM bridge** (the stable id is carried as `author_id` by `DioxusDocument`'s `Document::accessibility_tree`) — carried: Symbols bullet 2. Sites: `Dioxus DOM bridge` arch:1; `author_id` 0 hits in any master.
  - **arch §Standard Contracts → Mutation, query and CSSOM** (the defaulted, gated `Document::accessibility_tree`; the two name sources; the shell builds through the document) — carried: Symbols bullets 1, 3 and 4. Sites: `build_accessibility_tree` arch:1 · a11y-plan:2; `update_tree` 0.
  - **arch re-points** (citations into the changed files, by the measured line map below) — carried: the line map. Sites: `blitz-dom/src/accessibility.rs` arch:4 · security:1 · a11y:15; `blitz-dom/src/document.rs` arch:22 · security:7 · design:7 · test:9 · obs:8 · a11y:4; `dioxus_document.rs` arch:7 · security:1 · design:1 · layouts:3 · test:8 · a11y:3; `blitz-shell/src/window.rs` arch:7 · security:2 · design:4 · layouts:2 · obs:2 · a11y:7; `blitz-shell/src/accessibility.rs` a11y:4; `dioxus-native-dom/Cargo.toml` arch:8 · security:1 · test:1 · obs:1 · a11y:2. Only citations at or past a shift point move. Detectors re-point those, not the in-place hunks.
  - **a11y-plan §2 → Role derivation · §7 → Accessibility tree output** (the name sources and the carried id) — carried: Symbols bullets 2-3, Counts. Sites: `aria-label` a11y:6; `build_accessibility_tree` a11y:2; `blitz-dom/src/accessibility.rs` a11y:15.
  - **test-plan §1 · §3 Proof · §9 Local baseline** (two new files, 25 stand events, the re-counted workspace) — carried: Counts. Sites: `444` arch:2 · test:1 · a11y:1; `stand_id_persistence` arch:2 · security:1 · test:3 · obs:1 (the stand-file list sites).
  - **layout-templates §Surface: desktop-native → Primary screens** (`for` / `aria-label` attributes added, structure unchanged; task-file citations re-pointed) — carried: Files, plus the line map (timer.rs and crud.rs edits are in place, with no shift; flight_booker.rs shifts +1 from old 82 and +2 from old 88). Sites: `tasks/timer.rs` layouts:1; `tasks/crud.rs` layouts:1; `tasks/flight_booker.rs` layouts:1.
  - **design-system** (task-file citations re-pointed if lines move) — carried: line map. Sites: `tasks/flight_booker.rs` design:3; `tasks/timer.rs` design:1. Only flight_booker lines ≥ 82 move.
  - **security-plan §Input Validation `id` row** (the tree build is a new in-process reader of the id, which is still written nowhere; `aria-label` and `<label for>` are read into the tree from parsed content, the browser's remote HTML included, as an in-process reader, not a boundary widening) — carried: Symbols bullets 2-3. Sites: `dioxus_document.rs` security:1; `blitz-dom/src/accessibility.rs` security:1. Note: the id now LEAVES the process through the platform accessibility API (AccessKit `author_id` is delivered to the OS adapter when an AT is active), exactly as text runs and labels already do. It is written to no log, DOM or vdom.
  - **obs-plan** (re-points only, if the cited dioxus-native-dom lines move) — carried: line map. Sites: `dioxus-native-dom/Cargo.toml` obs:1 (obs-plan:19 cites `:19`, the `tracing` feature line, which is unchanged — 0 shift before line 39); `dioxus_document.rs` obs:0; `dioxus-native/Cargo.toml` obs:4 (`:30` changed in place, no shift).
  - **NEW from Deviations — the feature wiring** (not in the plan's list):
    - dioxus-native → dioxus-native-dom `accessibility` forwarding: sites `dioxus-native/Cargo.toml` arch:10 · a11y:3, at the a11y-plan:14 · :55 claims;
    - blitz-tests' dioxus-native-dom `accessibility` feature: sites `blitz-tests/Cargo.toml` arch:5 · test:3 · a11y:1.
- **Line map** (basis `git diff -U0 076d74cb HEAD -- {file} | grep '^@@'`; old line → new line):
  - `packages/blitz-dom/src/accessibility.rs`: 1-9 same · 10-27 +1 · 28-30 +7 · 31-69 +18 · 70-255 +26 (now 281 lines)
  - `packages/blitz-dom/src/document.rs`: ≤150 same · ≥151 +10
  - `packages/dioxus-native-dom/src/dioxus_document.rs`: ≤17 same · 18-284 +2 · ≥285 +20
  - `packages/dioxus-native-dom/Cargo.toml`: ≤38 same (18 edited in place) · ≥39 +1
  - `packages/dioxus-native/Cargo.toml`: no shift (30 edited in place)
  - `packages/blitz-shell/src/accessibility.rs`: ≤43 same (4 edited in place) · old 44-47 → 44-45 · ≥48 −2
  - `packages/blitz-shell/src/window.rs`: ≤522 same (378-380 edited in place) · old 523-524 → 523 · ≥525 −1
  - `examples/seven_guis/src/tasks/{timer,crud}.rs`: no shift (timer 78; crud 44, 85, 91 edited in place)
  - `examples/seven_guis/src/tasks/flight_booker.rs`: ≤81 same · 82-87 +1 · ≥88 +2
  - `tests/blitz-tests/Cargo.toml`: no shift (21 edited in place)
- **Coverage of new surfaces:**
  - `Document::accessibility_tree` (trait method + DioxusDocument override) → validation n/a (no external input; tree ids are mapped back through `element_ids()` pairs, never used to index the slab) · instrumentation n/a (no log site, by obs-plan §8) · PII n/a (the id and names are logged nowhere; they reach the platform AT like text runs already do) · tests integ (`stand_accessibility_ids` ×5, `accessibility_names::plain_html_tree_carries_no_author_id`) · a11y ✓ (the tree is the a11y surface) · tokens n/a
  - `aria-label` / `<label>` name sources → validation ✓ (a whitespace-only `aria-label` is ignored; only built, non-hidden labels and inputs associate) · instrumentation n/a · PII n/a (not logged) · tests integ (`accessibility_names` ×4) · a11y ✓ · tokens n/a
  - stand markup `for` ×4 / `aria-label` ×2 (no new element) → validation n/a · instrumentation n/a · PII n/a · tests integ (`controls_carry_role_and_name`, `tab_order_is_unchanged`, the markup probe) · a11y ✓ (names; Tab order pinned unchanged) · tokens n/a (no style change)

## Deviations from intent
- **Feature wiring** (plan step 3 assumed the optional-feature pattern reaches the tests and the windowed app; measured false, Spec claims disproved 1). The operator was asked at /implement P1 (AskUserQuestion, three options) and chose "Test manifest (Recommended)". Implement added the companion forwarding in dioxus-native on its own authority.
- **Gate 6 amended in plan.md** — on the operator's direction (2026-10-06, after implement P4): "Amend gate 6 in plan.md so its guarded list no longer covers tests/blitz-tests/Cargo.toml (the approved one-line feature change)". `tests/blitz-tests/Cargo.toml` left the guarded path list; the entry's `note` records why. Re-run alone: `green · exit 0` (`evidence/gate6-amended.md`). /implement normally leaves plan.md read-only; the operator's direction overrode that for this one edit.
- **P3 smoke form**: the plan named `just seven_guis` as the windowed boot. Because of Spec claims disproved 3, implement built `seven_guis_native` with `--features dioxus-native/accessibility` (dev profile) and ran it under `timeout -s TERM -k 5 15`. Result: exit 124 (stayed up), 0 panic lines, no survivor. The host's AT-SPI `IsEnabled` reads false, so the adapter never built a platform tree. The smoke proves the boot, not the windowed tree build (`evidence/smoke.md`).
- **Stand check, one extra assertion**: `assert_carried` also checks that each id-carrying node's `html_tag` equals its DOM element's local name, a keying cross-check beyond the plan's text. `label_for_names_its_input` also covers a label placed after its input.
- **Operator pass driven by the agent** on the operator's direction (entries 18 · 19 · 20, recorded in `evidence/operator-pass.md`).
- Scope record (`gate.py scope` at this P1: `scope: clean — changed 14 · listed 12 · recorded 2 (companion 1 · mechanical 0 · in-intent 0 · widening 1)`):
  - companion: `packages/dioxus-native/Cargo.toml` · serves packages/dioxus-native-dom/Cargo.toml · self
  - widening: `tests/blitz-tests/Cargo.toml` · serves step 8 · word: "Test manifest (Recommended)" — the operator, 2026-10-06

## Decisions & corrections
- Operator (implement P1): enable dioxus-native-dom's `accessibility` in the test manifest rather than enabling dioxus-native accessibility in seven_guis (which would give the windowed stand a live AccessKit adapter) or re-planning.
- Operator (post-implement): amend gate 6 in plan.md and re-run it alone; the agent runs the operator pass; at this wrap the "seven_guis ships without AccessKit" note goes on the route.
- Sweep hazard: a feature-gated item can pass under `cargo test --workspace`, where every selected member's own defaults unify in, and still be compiled out under `cargo test -p blitz-tests --test X` when the workspace pins `default-features = false`. Before trusting a feature gate, check the per-package graph with `cargo tree -p {pkg} -e features -i {crate}`.
- Hazard: the project's PreToolUse guard refuses a whole Bash call that carries a `cat > file <<EOF` heredoc, edits bundled in the same call included. None of the call runs.

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - (arch) each element node carries `author_id == element_id`; pairwise distinct; keyed by `as_u64` — **met** (`stand_accessibility_ids::element_nodes_carry_their_stable_id`, both modes).
  - (arch) blitz-dom gains no dioxus dependency; a plain tree through the trait equals `build_accessibility_tree()` node for node with no author id; no new crate, env var or port — **met** (blitz-dom/Cargo.toml is unchanged; `plain_html_tree_carries_no_author_id`; the root-manifest guard, amended, is green). The test manifest changed by the operator's widening: a feature line, not a crate or dependency.
  - (arch) the no-default-features builds and `ci-leg.sh doc` pass — **met**.
  - (a11y) 15 controls with an HTML-AAM role and a non-empty name, both modes — **met** (`controls_carry_role_and_name` asserts 15).
  - (a11y) no TextRun, document root or Window node carries an id — **met** (`non_elements_carry_no_id`).
  - (a11y) Tab order equals the base measurement; the a11y leg stays green — **met** (`evidence/tab-order-base.md`; `tab_order_is_unchanged`; `ci-leg.sh a11y` green).
  - (a11y) `aria-label` names; a whitespace-only one names nothing; `<label for>` and a nested label name through `labelled_by` — **met** (`accessibility_names` 5/5).
  - (layouts) after CRUD Create the created row carries `TaskShell/Crud/div:0/div:1/div:0/div[3]`; after Delete no dropped id appears and nothing panics — **met** (`ids_hold_after_a_rerender`).
  - (layouts · design) the markup probe prints `markup-unchanged-but-names` — **met**.
  - (security · obs) census 0; 0 content-named event keys — **met**.
  - (security) the lockfile diff is exactly one line; audit green — **met**.
  - (tests) `run stand` gives 25 ok — **met** (25).
  - (tests) `ci-leg.sh fast` exits 0, workspace 454 · 0 · 5 — **met**.
  - (tests) CI green on the pushed sha — **met** (CI#37465287000 on `9285fe75`, 16/16).
  - No matrix capability claimed — **held** (`matrix.py show --chunk` → claimed 0).
- Gates (the implement run in `.andromeda/runs/2026-10-06T12-22-52-implement`, then the operator pass at `9285fe75`):
  - `cargo test -p blitz-tests --locked --test accessibility_names` — green · exit 0 · `contains 5 passed; 0 failed` ✓
  - `cargo test -p blitz-tests --locked --test stand_accessibility_ids` — green · exit 0 · `5 passed; 0 failed` ✓. Red-before-green: on the base markup `controls_carry_role_and_name` failed at "FlightBooker: \"flight-start\" is named"
  - `cargo test -p blitz-tests --locked --test accessibility_hidden … --test stand_id_persistence` — green · exit 0 · `lacks FAILED` ✓
  - `cargo build -p blitz-dom --no-default-features --locked && cargo build -p dioxus-native-dom --no-default-features --locked` — green · exit 0
  - markup probe (`python3 -c "… markup-unchanged-but-names …"`) — green · `last line markup-unchanged-but-names` ✓
  - `git diff --quiet 076d74cb… -- Cargo.toml examples/seven_guis/Cargo.toml …` (the diff guard) — first run **red · exit 1** (sole hit `tests/blitz-tests/Cargo.toml`, the operator's widening). The guard was amended on the operator's direction and re-run alone: **green · exit 0**.
  - `test "$(git diff -U0 … Cargo.lock | grep …)" = '+ "accesskit",' && echo lock-one-edge` — green · `last line lock-one-edge` ✓
  - `test -f … && cat … | grep -c -E 'tracing|println!|…'` (census) — green · exit 1 · `last line 0` ✓
  - `bash scripts/agent-run.sh boot` — green · `contains "ready"` ✓
  - `bash scripts/agent-run.sh run stand` — green · `contains "run.end"` ✓
  - `agent-run.sh logs | … startswith("stand_")` — green · `last line 25` ✓
  - `agent-run.sh logs | … bad=…` — green · `last line 0` ✓
  - `bash scripts/agent-run.sh cleanup` — green
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (454 · 0 · 5)
  - `bash .github/scripts/ci-leg.sh doc` — green
  - `bash .github/scripts/ci-leg.sh a11y` — green
  - `bash .github/scripts/ci-leg.sh audit` — green
  - `gate.py hygiene` (leg operator) — by hand, exit 0 · `hygiene: clean` ✓ (re-fired over the final evidence set: read 37, 0 host paths kept)
  - `ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` (leg operator) — by hand: fast exit 0, tree clean, pushed `076d74cb..9285fe75`
  - `ci.py conclusion --sha HEAD --wait 1800` (leg operator) — by hand, exit 0 · `verdict: green` ✓ (CI#37465287000, 16/16, 692 s)
  - smoke: windowed `seven_guis_native` (built with accessibility forced) stayed up for 15 s, 0 panics; the platform tree build was not exercised (AT-SPI off).
- Watches: none folded on this entry.
- Outcome basis: the operator pass ran (pre-CI commit `9285fe75`, the only commit in `076d74cb..HEAD`). The gate verdicts rest on implement's run (this conversation) plus the operator pass's final HEAD CI run, recorded in `evidence/operator-pass.md`. Between implement P4 and the pass, the operator's directive changed plan.md gate 6 only.
- Process hygiene: implement P4's census (this conversation):
  - the gate tool's entries (cargo, agent-run): terminated with their entries;
  - `target/agent-run`: absent after `cleanup`;
  - the smoke's `seven_guis_native`: started by this run, terminated by `timeout` (TERM).
  - Re-measured at implement P4 (`ps -eo … | grep -E 'seven_guis_native|agent-run|cargo (test|build)|gate.py'` → none matching). The operator pass started cargo (fast) and `ci.py` (both exited); these were not re-measured after the pass.
