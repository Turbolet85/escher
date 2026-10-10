# Fan-out results — 2026-10-10-upstream-sync-agent-surfaces

Seven doc-agents, one parallel batch. Each return was taken from its agent's hand-back whole and probed for HTML entities (0 in every one, before and after the decode).
A coordinate check ran over every proposal: each `file:line` it writes is one the report's bullets state or its **New text, by line** section lists, or a citation the body already held — 0 proposals rejected for a source the report does not carry.

## architecture

**Verdict:** 23 proposals (22 warning · 1 escalate) · nothing stripped: the return opened at `proposals:`

```yaml
proposals:
- detector: D-arch-decisions
  severity: warning
  section: §Established Decisions → [Dependency pinning]
  change: >-
    Replace "taffy and parley are git dependencies pinned by `rev` (Cargo.toml:106; Cargo.toml:116)" with: taffy is a git dependency pinned by `rev` (Cargo.toml:106); parley is a registry version, `parley = { version = "0.12", … }` (Cargo.toml:116), no longer a git `rev`.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — [Dependency pinning]: parley moved from a git rev to registry version "0.12"; taffy stays a git rev.
  rationale: >-
    Report Dependencies, "Pin form changed": `parley` is a registry version (`Cargo.toml` 116), no longer a git `rev`; `taffy` stays a git dependency (`Cargo.toml` 106, rev 4142c9d8 → 7d33901c…). The report's Expected amendments quote architecture line 73 as the sentence made false.
  basis: Cargo.toml:116
- detector: D-arch-decisions
  severity: warning
  section: §Stack and Technologies → Text and fonts row
  change: >-
    Replace "parley from git `https://github.com/linebender/parley` at a pinned rev" with: parley "0.12" from the registry (Cargo.toml:116); the rest of the row (skrifa "0.44", `wuff`) unchanged.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Stack, Text and fonts: parley is registry "0.12", not a git rev.
  rationale: >-
    Same retired claim as [Dependency pinning], restated in the Stack table. Report Dependencies: `parley = { version = "0.12", … }` (`Cargo.toml` 116); with it fontique, parley_data, parley_emoji, parley_engine 0.11.0 (git) → 0.12.0 (registry). skrifa 0.44.0 is listed Unmoved.
  basis: Cargo.toml:116
  dependent-of: D-arch-decisions
- detector: D-arch-decisions
  severity: warning
  section: §Inherited Defaults → Framework
  change: >-
    Replace "taffy and parley at pinned git revs (Cargo.toml:106; Cargo.toml:116)" with: taffy at a pinned git rev (Cargo.toml:106), parley registry "0.12" (Cargo.toml:116).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Inherited Defaults, Framework: parley is registry "0.12"; only taffy is a pinned git rev.
  rationale: >-
    Third occurrence of the retired claim; the report's Expected amendments quote architecture line 219 ("taffy and parley at pinned git revs"). Report Dependencies: parley is a registry version at `Cargo.toml` 116.
  basis: Cargo.toml:116
  dependent-of: D-arch-decisions
- detector: D-arch-decisions
  severity: warning
  section: §Stack and Technologies → Accessibility row
  change: >-
    accesskit_unix reads 0.24.0 (was 0.23.0) and accesskit_android 0.9.0 (was 0.8.0); accesskit "0.25" and the accesskit_windows / accesskit_macos manifest strings stay as written.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Stack, Accessibility: accesskit_unix 0.23.0 → 0.24.0, accesskit_android 0.8.0 → 0.9.0.
  rationale: >-
    Report Dependencies, "Bumped": `accesskit_unix` 0.23.0 → 0.24.0 and `accesskit_android` 0.8.0 → 0.9.0 (`packages/accesskit_xplat/Cargo.toml`, added 27 · 30); the manifest still reads accesskit "0.25". Expected amendments name architecture line 53 as carrying both old versions. accesskit_windows / accesskit_macos moved "by a patch or minor in the lock" only — their manifest lines are not among the file's added lines (3 · 27 · 30), so the row's manifest strings for them are not shown changed.
  basis: packages/accesskit_xplat/Cargo.toml:27
- detector: D-arch-decisions
  severity: warning
  section: §Stack and Technologies → Markdown app (rdme) row
  change: >-
    Replace "comrak 0.55" with "comrak 0.56".
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Stack, rdme: comrak 0.55 → 0.56.
  rationale: >-
    Report Dependencies, "Bumped": `comrak` 0.55.0 → 0.56.0; `apps/readme/Cargo.toml` has one added line, 60. The exact manifest string was not quoted by the report — the orchestrator's one read of that line settles "0.56" vs another spelling.
  basis: apps/readme/Cargo.toml:60
- detector: D-arch-decisions
  severity: warning
  section: §Stack and Technologies (new row — Unicode / text transform)
  change: >-
    Add a row: ICU4X — icu_casemap, icu_locale_core, icu_properties, icu_segmenter "2.3" and writeable "0.6" (Cargo.toml:202-206); role: CSS `text-transform` casing and segmentation in blitz-dom behind its default-on `text-transform-icu` feature (`dep:icu_casemap`, `dep:writeable`; packages/blitz-dom/src/layout/text_transform.rs) and `icu_properties` in blitz-vibey-script's Selection (packages/blitz-vibey-script/src/dom/selection.rs:9-12); upstream's, arrived with the 2026-10-10 sync.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Stack: ICU4X (icu_casemap, icu_locale_core, icu_properties, icu_segmenter 2.3; writeable 0.6) registered as a new library family.
  rationale: >-
    A new library family the Stack table does not list (the report's own search: `ICU4X|icu_` 0 hits in any master — "stated nowhere yet"). Report Dependencies: "Added to the root manifest (202-206, under the comment at 201)"; six names enter the lock (core_detect, multiversion_no_op, icu_casemap, icu_casemap_data, memchr-n, fearless_simd_macros), each reviewed by hand in evidence/lock.md. Schema / config: `text-transform-icu = ["dep:icu_casemap", "dep:writeable"]` in blitz-dom's default list.
  basis: Cargo.toml:202-206
- detector: D-arch-decisions
  severity: warning
  section: §Stack and Technologies → CI/CD row
  change: >-
    The Python 3 helper scripts now also include upstream's `wpt_area_changes.py` with `test_wpt_area_changes.py` and escher's `test_blitz_tests_targets.py`; the latter imports stdlib `tomllib`, so the `ci-scripts` leg needs Python 3.11 or later (no package added; ran green on `ubuntu-latest`, the runner's Python version not read; dev host Python 3.14.7).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Stack, CI/CD: three new CI scripts; the ci-scripts leg gains a Python ≥ 3.11 floor through `tomllib`.
  rationale: >-
    Report Dev-tool versions: "The new CI-script test imports `tomllib` (Python 3.11 or later)… the runner's Python version was not read". Harness / gate surface: the leg "now discovers six test files", among them the new `test_blitz_tests_targets.py` and upstream's new `test_wpt_area_changes.py`; the last section lists `.github/scripts/wpt_area_changes.py` as a new file (92 lines). The row names only `wpt_diff_to_pr.py` / `test_wpt_diff_to_pr.py` as helper scripts and states no Python floor.
  basis: .github/scripts/test_blitz_tests_targets.py:17-26
- detector: D-arch-decisions
  severity: warning
  section: §Established Decisions → [Default features]
  change: >-
    blitz-dom's default list is svg, woff, accessibility, system-fonts, file-input, custom-widget, text-transform-icu (`text-transform-icu = ["dep:icu_casemap", "dep:writeable"]`); blitz, dioxus-native and dioxus-native-dom each gain a forwarding `text-transform-icu` in their defaults; a new `writing-mode` feature (blitz-dom: `["stylo_taffy/writing-mode"]`, forwarded by blitz, dioxus-native, dioxus-native-dom) is in no default list — vertical writing modes compile only under it; by runner, the workspace build has `writing-mode` on and the per-package blitz-tests build has it off.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — [Default features]: `text-transform-icu` added to four default lists; opt-in `writing-mode` feature added in four crates.
  rationale: >-
    Report Schema / config: blitz-dom features (`packages/blitz-dom/Cargo.toml`, added 20-21 · 32-34 · 37-38 · 78-84) — new `text-transform-icu` "in blitz-dom's own default list" and new `writing-mode` "in no default list"; "`blitz`, `dioxus-native`, `dioxus-native-dom` gain forwarding `text-transform-icu` (in their defaults) and `writing-mode` (not in them)". Counts: blitz-dom features by runner 14 → 16 (workspace) and 8 → 10 (per-package). The bullet's four default lists omit the new member. stylo_taffy also gains a `writing-mode` feature (implied by `stylo_taffy/writing-mode`; `packages/stylo_taffy/Cargo.toml` added 27-29) — its default membership is not stated by the report.
  basis: packages/blitz-dom/Cargo.toml:20-21
- detector: D-arch-decisions
  severity: warning
  section: §Established Decisions → [Document defaults]
  change: >-
    Add to the "Stylo prefs enabled" list: under the `writing-mode` feature the document also sets `layout.writing-mode.enabled` (packages/blitz-dom/src/document.rs:416-417).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — [Document defaults]: feature-gated Stylo pref `layout.writing-mode.enabled` added to the prefs list.
  rationale: >-
    Report Schema / config: "Under `writing-mode` the document sets the stylo pref `layout.writing-mode.enabled` (`document.rs` added 416-417)". The bullet enumerates the prefs the document enables and does not hold this one.
  basis: packages/blitz-dom/src/document.rs:416-417
- detector: D-arch-decisions
  severity: escalate
  section: §Established Decisions → [Unsupported features]
  change: >-
    Retire or re-measure "`text-indent` `hanging`/`each-line` do not work because their parsing is cfg'd out in Stylo": the inline layout now forwards Stylo's computed `each_line` and `hanging` into the text-indent options it hands Parley (packages/blitz-dom/src/layout/inline.rs:473-477); whether Stylo parses the two keywords in this build is not measured.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — [Unsupported features]: the text-indent hanging/each-line sentence no longer matches the code; needs a measured reading before it is restated.
  rationale: >-
    Report Expected amendments (inside Changes) names architecture line 86 and says "what upstream's text-indent commit changed about that sentence was not read by this chunk — the detector reads `packages/blitz-dom/src/layout/inline.rs` (added ranges in the last section)". Read at that direction: the added block 473-477 builds indent options from `text_indent.each_line` / `text_indent.hanging`, and a grep of the file for `hanging|each.line` finds no comment saying they are unsupported. Escalated because the report measured nothing here — the sentence's cause ("parsing cfg'd out in Stylo") is neither confirmed nor refuted by the chunk.
  basis: packages/blitz-dom/src/layout/inline.rs:473-477
- detector: D-arch-decisions
  severity: warning
  section: §Project Intent → the `Upstream sync:` line
  change: >-
    Upstream sync: DioxusLabs/blitz `main` last merged at 7832c177ff272128154bac58afe56c2b9164b417 (merge commit 9462a7e47923ee512ef71a8b0eb86613315aa69d, 2026-10-10) — the next sync's merge base.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Upstream sync line: merge base 23354585 → 7832c177, merge commit f00b0216 → 9462a7e4, 2026-10-10.
  rationale: >-
    Report Counts / qualifiers moved: "The upstream sync line: merge base `23354585` → `7832c177`, merge commit `f00b0216` → `9462a7e47923ee512ef71a8b0eb86613315aa69d`. Stated in architecture (`Upstream sync:`: line 205)"; Expected amendments list it first. Neither detector's invariant names this line exactly — filed here as the stack's upstream baseline; one occurrence only (the report's search: architecture 1 hit, no other file).
- detector: D-arch-decisions
  severity: warning
  section: §Conventions → Tests
  change: >-
    Add: blitz-tests keeps one test target per file under `tests/` — `[package]` sets no `autotests`; the one file that is not a built target is upstream's single-binary `tests/all.rs`, kept in the manifest as a `[[test]]` table `all` with `test = false` (tests/blitz-tests/Cargo.toml:54-57, under the comment 45-53) — the operator's decision, 2026-10-10; a standing two-line difference from upstream, pinned by `BlitzTestsTargetsTest`.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Conventions, Tests: one test target per blitz-tests file; upstream's `all` target kept unbuilt (`test = false`).
  rationale: >-
    Report Schema / config: a `[[test]]` table `name = "all"`, `path = "tests/all.rs"`, `test = false` (54-57) under the comment 45-53; `[package]` sets no `autotests`. Harness / gate surface: "blitz-tests keeps one test target per file; upstream's single-binary target `all` stands in the manifest unbuilt". Decisions & corrections: the operator, 2026-10-10. Expected amendments name §Conventions → Tests (`autotests|tests/all\.rs|single.binary|one binary`: 0 hits) — the convention is stated nowhere, and `tests/all.rs` is now a file under the integration-test directory that is not a test target.
  basis: tests/blitz-tests/Cargo.toml:54-57
- detector: D-arch-resources
  severity: warning
  section: §Standard Contracts → Layout
  change: >-
    Replace "`BaseDocument` implements Taffy's `TraversePartialTree`, … `RoundTree`, `PrintTree`" with: the crate-private `LayoutPassState<'doc>` (packages/blitz-dom/src/layout/mod.rs:82-103; `new` 123-141; `Deref`/`DerefMut` to `BaseDocument` 144, 151) implements Taffy's `TraversePartialTree`, `TraverseTree`, `LayoutPartialTree`, `LayoutContainingBlock`, `CacheTree`, `LayoutBlockContainer`, `LayoutFlexboxContainer`, `LayoutGridContainer`, `RoundTree` — `BaseDocument` implements none of them — and `PrintTree` is implemented for `TaffyDebugTree` (layout/mod.rs:911).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Layout contract: Taffy's tree traits moved off `BaseDocument` onto `LayoutPassState`; `PrintTree` onto `TaffyDebugTree` (upstream #1099).
  rationale: >-
    Report Symbols / APIs: "Taffy's tree traits moved off `BaseDocument` (upstream #1099)" — `impl … for LayoutPassState<'_>` at lines 608, 640, 642, 686, 732, 763, 795, 815, 846, plus `Deref` 144 and `DerefMut` 151; "the chunk start held 10 `impl … for BaseDocument` there and the merged file 0"; `LayoutPassState<'doc>` is `pub(crate)` (82-103, head 84; `new` 123-141); `PrintTree` for `TaffyDebugTree` (line 911). Expected amendments quote architecture line 125 as the one hit; a sweep for the claim in other wording (tree traits, layout tree, `for BaseDocument`) finds no second occurrence.
  basis: packages/blitz-dom/src/layout/mod.rs:608
- detector: D-arch-resources
  severity: warning
  section: §Standard Contracts → Layout
  change: >-
    Add the new public layout surface: `stylo_taffy` also exports `WritingModeExt` (packages/stylo_taffy/src/writing_mode.rs:26-84; lib.rs:12-17) and `convert::inline_containing_block_claims` (convert.rs:373-394); `TaffyStyloStyle` gains `new_in` and `set_percent_basis` (wrapper.rs:76-90; 414-423); blitz-dom gains `Node::layout_style_in` (node/node.rs:1140-1150) and `full_width` / `full_size_kana` (layout/text_transform.rs:247-289; 291-321).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Layout contract: upstream's new public writing-mode and text-transform API registered.
  rationale: >-
    Report Symbols / APIs, "Added by upstream, public": `Node::layout_style_in` (1140-1150, head 1143); `stylo_taffy::WritingModeExt` (26-84, head 27); `stylo_taffy::convert::inline_containing_block_claims` (373-394, head 375); `TaffyStyloStyle::new_in` and `set_percent_basis` (76-90 and 414-423); `full_width` and `full_size_kana` (247-289, 291-321). The contract's export list for `stylo_taffy` reads `to_taffy_style`, `TaffyStyloStyle`, `StyleFlags`, `Atom` only.
  basis: packages/stylo_taffy/src/writing_mode.rs:26-84
- detector: D-arch-resources
  severity: warning
  section: §Standard Contracts → Dioxus DOM bridge
  change: >-
    In the `bounds` clause, replace "the reader positions a node through `unrounded_absolute_position(0, 0)`, which subtracts the node's own scroll offset with its ancestors'" with: the reader positions a node through the crate-private `BaseDocument::physical_unrounded_geometry`, which sums `location − scroll_offset` over the containing-block chain, the node's own scroll offset included (two bodies: packages/blitz-dom/src/document.rs:2254-2270 without `writing-mode`, packages/blitz-dom/src/layout/writing_mode.rs:371-401 with it); `unrounded_absolute_position` no longer exists; the shifted-`bounds` consequence stands.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Dioxus DOM bridge, `bounds`: reader named `physical_unrounded_geometry` (upstream removed `unrounded_absolute_position`); behaviour unchanged.
  rationale: >-
    Report Symbols / APIs: "Removed by upstream: `Node::unrounded_absolute_position`… no definition and no caller left"; "Added by upstream, crate-private: `BaseDocument::physical_unrounded_geometry`… two bodies — `document.rs` 2254-2270 (head 2257)… `writing_mode.rs` 371-401 (head 375)… Both sum `location − scroll_offset` over the containing-block chain, the node's own scroll offset included"; `get_client_bounding_rect` rewritten onto the new reader. Open findings: it "replaces `unrounded_absolute_position` and still subtracts a node's own scroll offset; that entry's cited lines move". Single occurrence of the symbol in architecture; the Driver session and Scrolling contracts restate only the consequence (own-offset shift / cancel), which still holds.
  basis: packages/blitz-dom/src/document.rs:2254-2270
- detector: D-arch-resources
  severity: warning
  section: §Standard Contracts → CI contracts
  change: >-
    Add: the blitz-tests target contract — `[package]` does not set `autotests` to false and the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false` — pinned by `.github/scripts/test_blitz_tests_targets.py` (`BlitzTestsTargetsTest`, one test, 17-26), run by the existing `ci-scripts` leg, which now discovers six test files.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — CI contracts: `test_blitz_tests_targets.py` registered as the pin of the one-target-per-file blitz-tests manifest.
  rationale: >-
    Report Files: `.github/scripts/test_blitz_tests_targets.py` — new, ours, 30 lines. Harness / gate surface: `BlitzTestsTargetsTest`, one test, lines 17-26 — "`[package]` does not set `autotests` to false; exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`"; red on three control manifests. The contract lists each pinning test of the leg (`test_ci_workflows.py`, `test_agent_run.py`, `test_cold_agent.py`) and not this one (the report's search for the name in architecture is not among its hits).
  basis: .github/scripts/test_blitz_tests_targets.py:17-26
- detector: D-arch-resources
  severity: warning
  section: §Standard Contracts → CI contracts
  change: >-
    Add upstream's WPT area reporting: a new script `.github/scripts/wpt_area_changes.py` (`load_units`, `areas_of`, `compare`, `main`) with its test `test_wpt_area_changes.py`, and `wpt_diff_to_pr.py` reading an optional areas input (`args.areas`, 287-289) rendered by `format_area_lines` (154-189) — upstream-only WPT surfaces, no escher leg but `ci-scripts` runs their tests.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — CI contracts: upstream's `wpt_area_changes.py` and the areas input of `wpt_diff_to_pr.py` registered.
  rationale: >-
    Report Counts: CI-scripts tally 70 → 78 tests, "4 new in `test_wpt_area_changes.py`, 4 → 7 in `test_wpt_diff_to_pr.py`". The last section lists `.github/scripts/wpt_area_changes.py` as a new file (92 lines; `load_units` 21-34, `areas_of` 37-39, `compare` 42-76, `main` 79-88) and, in `wpt_diff_to_pr.py`, `format_area_lines` 154-189 and `if args.areas and os.path.exists(args.areas):` 287-289. The contract's CLI list for `wpt_diff_to_pr.py` (`diff_file`, `--repo`, `--pr`, `--run-url`, `--dry-run`) predates it; the flag's spelling is not stated by the report.
  basis: .github/scripts/wpt_diff_to_pr.py:287-289
- detector: D-arch-resources
  severity: warning
  section: §Standard Contracts → JavaScript (blitz-vibey-script)
  change: >-
    Add the new script-to-DOM surface: `document.getSelection` returning a Selection object with `setBaseAndExtent`, `removeAllRanges` and `toString` (packages/blitz-vibey-script/src/dom/document.rs:75-81; dom/selection.rs:29-57, 69-119, 121-128, 130-145) — it writes the document's text selection through `BaseDocument::set_text_selection`, not through `mutate()` — and read-only `innerText` / `outerText` getters (dom/element.rs:764-774; inner_text.rs:12-48); escher uses none of them.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — JavaScript contract: script Selection API and `innerText`/`outerText` getters registered (upstream's; unused by escher).
  rationale: >-
    Report Symbols / APIs: "Added by upstream on the script-to-DOM crossing (blitz-vibey-script): `document.getSelection` (`src/dom/document.rs` 75-81; `src/dom/selection.rs`, new, `get_selection` 29-57, `set_base_and_extent` 69-119, `remove_all_ranges` 121-128, `to_string` 130-145) and the `innerText` / `outerText` getters (`src/dom/element.rs` 764-774; `src/inner_text.rs`, new, `inner_text` 12-48). Research read the Selection API as writing through `BaseDocument::set_text_selection` with 0 `mutate()` calls and `inner_text.rs` as read-only." A new crossing that writes document state and is named in no contract (architecture has 0 hits for `getSelection|innerText|outerText`).
  basis: packages/blitz-vibey-script/src/dom/selection.rs:69-119
- detector: D-arch-resources
  severity: warning
  section: §Occupied Resources → Names
  change: >-
    In the features list, blitz reads `net`, `accessibility`, `tracing`, `scrollbars`, `text-transform-icu`, `writing-mode`; add the new feature names blitz-dom `text-transform-icu` and `writing-mode`, stylo_taffy `writing-mode`, and the forwarding `text-transform-icu` / `writing-mode` of dioxus-native and dioxus-native-dom.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Names: feature names `text-transform-icu` and `writing-mode` registered (blitz, blitz-dom, stylo_taffy, dioxus-native, dioxus-native-dom).
  rationale: >-
    Report Schema / config: new blitz-dom features `text-transform-icu = ["dep:icu_casemap", "dep:writeable"]` and `writing-mode = ["stylo_taffy/writing-mode"]`; "`blitz`, `dioxus-native`, `dioxus-native-dom` gain forwarding `text-transform-icu`… and `writing-mode`… features" (`packages/blitz/Cargo.toml` added 14 · 17 · 20). The Names entry enumerates blitz's features as four names. Crates / modules: no workspace member added or removed, so the crate names themselves do not move.
  basis: packages/blitz/Cargo.toml:14
- detector: D-arch-resources
  severity: warning
  section: §Existing Scopes → blitz-vibey-script row
  change: >-
    Modules gain `inner_text` (packages/blitz-vibey-script/src/lib.rs:37); dom submodules gain `selection` (dom/mod.rs:14); integration tests gain `tests/selection.rs`.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Existing Scopes, blitz-vibey-script: modules `inner_text` and `dom::selection`, test file `tests/selection.rs`.
  rationale: >-
    Report Crates / modules: new modules, all upstream's — `blitz-vibey-script` `dom/selection.rs` (417) and `inner_text.rs` (280); Coverage: `packages/blitz-vibey-script/tests/selection.rs`, 15 passed. The last section lists `lib.rs` added 37 and `dom/mod.rs` added 14. The row's module, submodule and test-file lists predate all three.
  basis: packages/blitz-vibey-script/src/lib.rs:37
- detector: D-arch-resources
  severity: warning
  section: §Existing Scopes → stylo_taffy row
  change: >-
    "Modules wrapper and convert" becomes: modules wrapper, convert and writing_mode (packages/stylo_taffy/src/lib.rs:12-17).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Existing Scopes, stylo_taffy: module `writing_mode` added.
  rationale: >-
    Report Crates / modules: `stylo_taffy` `writing_mode.rs` (283), new, upstream's; the last section lists `packages/stylo_taffy/src/lib.rs` added 12-17 and the new file's `pub trait WritingModeExt` at 26-84.
  basis: packages/stylo_taffy/src/lib.rs:12-17
- detector: D-arch-resources
  severity: warning
  section: §Existing Scopes → wpt row
  change: >-
    Modules gain `test_variants` (wpt/runner/src/main.rs:43) beside `test_runners`, `net_provider`, `panic_backtrace`, `report`.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Existing Scopes, wpt: module `test_variants` added.
  rationale: >-
    Report Crates / modules: "the WPT runner's `test_variants.rs` (291)", new, upstream's; the last section lists `wpt/runner/src/main.rs` added 43 · 46 and the new file (291 lines).
  basis: wpt/runner/src/main.rs:43
- detector: D-arch-resources
  severity: warning
  section: §Existing Scopes → blitz-dom: layout row
  change: >-
    Add the two new layout modules: `text_transform` (CSS text-transform; packages/blitz-dom/src/layout/text_transform.rs) and `writing_mode` (vertical writing modes and the feature-gated body of `physical_unrounded_geometry`, compiled under `writing-mode`; packages/blitz-dom/src/layout/writing_mode.rs:371-401).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — Existing Scopes, blitz-dom layout: modules `text_transform` and `writing_mode` added.
  rationale: >-
    Report Crates / modules: new modules, all upstream's — `blitz-dom` `layout/text_transform.rs` (979 lines) and `layout/writing_mode.rs` (435). Symbols / APIs: the second body of `physical_unrounded_geometry` is `writing_mode.rs` 371-401 "under the feature". The row enumerates the layout submodules (construct, damage, inline, list, paint_tree, replaced, table) and holds neither.
  basis: packages/blitz-dom/src/layout/writing_mode.rs:371-401
```

**Dispositions** (proposal number, in list order):

1. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the citing row read `claim false` at the sweep
2. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 1)
3. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 1)
4. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the two versions read at the sweep
5. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the manifest string read at the sweep, "0.56"
6. apply in part · check 1 — routine; the row is re-derived from the report's Dependencies and Schema bullets, and the clause placing `icu_properties` in the script Selection is not applied (no bullet of the report states it)
7. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
8. apply in part · check 1 — routine: the four default lists and the opt-in feature; the by-runner clause is test-plan §9's
9. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
10. escalate · check 1 — graded `escalate` by the detector, no rule governs (the plan's entry leaves the judgment to the wrap, and the report measured nothing of the sentence's cause) → resolved: the sentence is kept, re-cited, its cause labelled `recorded, not established` — word: "Keep, marked not established (Recommended)" — the operator, 2026-10-10, through the question dialog of this wrap (`escalations.md`) → apply
11. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
12. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
13. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the citing row read `claim false` at the sweep
14. apply in part · check 1 — routine for `stylo_taffy`'s crate-root exports, which the contract enumerates; the methods of an already-registered type and the functions inside an already-registered module are rejected as registry over-reach (playbook rule 3)
15. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the citing rows read `claim false` at the sweep
16. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
17. apply in part · check 1 — routine: the `--areas` argument (its spelling read at the sweep) and the script that feeds it; the script's function names are not registered (playbook rule 3)
18. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list — judged by its subject: scripts already write the document through this crossing, so no read-only channel gains a write and no new input class crosses; recorded as the first sync's accessors were
19. apply in part · check 1 — routine for blitz's own list, which the entry enumerates and the sweep read short; other crates' feature names, which the entry never listed, are rejected as registry over-reach (playbook rule 3)
20. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
21. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
22. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
23. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list

## security-plan

**Verdict:** 11 proposals (10 warning · 1 escalate) · stripped: five comment lines — each detector's verdict (D-security-input: no unvalidated boundary confirmed, one escalate; D-security-auth and D-security-deps: invariant holds) and what it left to the citation sweep

```yaml
proposals:
- detector: D-security-input
  severity: warning
  section: "§Input Validation → row `file:` URLs (net provider) | Path"
  change: >-
    Read through `Url::to_file_path` off wasm — a `file:` URL that is not a path returns a typed `InvalidInput` error, no panic — replacing "Read with `std::fs::read(request.url.path())`"; still no path restriction (packages/blitz-net/src/lib.rs:160-169) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Input Validation `file:` URLs (net provider): the read goes through `Url::to_file_path` off wasm, a non-path URL is a typed `InvalidInput` error; no path restriction, as before (upstream #1110).
  rationale: >-
    Report Changes → Symbols / APIs (engine behaviour changed by upstream, #1110) and Coverage of new surfaces ("validation ✓ for the path form (a typed `InvalidInput` error, no panic); still no path restriction … tests ✗"). The row's stated mechanism is retired; the standing gap it records is unchanged, so warning. Swept for `url.path` / `to_file_path` / "path restriction": this row only — the Threat Model's "reads `file:` URLs from local disk" names no mechanism and stands; the script fetcher's `file:` read (blitz-vibey-script fetch.rs) is another surface, not in the report.
  basis: "packages/blitz-net/src/lib.rs:160-169"
- detector: D-security-input
  severity: escalate
  section: "§Input Validation → row `@font-face` source | URL"
  change: >-
    Add to the row: a source with no format hint whose URL has no extension (a `data:` URL) is no longer skipped — it is fetched and its format sniffed from the bytes, the Fetched fonts | Format row's mechanism (packages/blitz-dom/src/net.rs:458-462); no test is named for it — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Input Validation `@font-face` source: a hint-less, extension-less source (a `data:` URL) is now fetched and byte-sniffed instead of skipped (upstream #1109); an input widening arriving by merge, untested in the delta.
  rationale: >-
    Report Changes → Symbols / APIs: "an `@font-face` source with no format hint whose URL has no extension (a `data:` URL) is no longer skipped — it is fetched and its format sniffed from the bytes (net.rs 458-462, #1109)". This widens which document-supplied font sources reach the fetch and decode path. The report's Coverage of new surfaces has NO row for it, so "validation present" rests on that one sentence about upstream's diff, not on a measured reading or a named test — the detector cannot confirm the boundary validated, hence escalate. Security-plan held no "skipped" claim for this case (the row's own claim, an unresolvable URL is skipped, is a different case and stands), so this is an addition, no duplicate occurrence.
  basis: "packages/blitz-dom/src/net.rs:458-462"
- detector: D-security-input
  severity: warning
  section: "§Input Validation → JS API arguments (new row: Selection)"
  change: >-
    New row — JS API arguments | Selection (`document.getSelection`, `setBaseAndExtent`, `removeAllRanges`, `toString`) | `setBaseAndExtent` checks its argument count and its offsets and throws a JS error on a bad call, leaving the selection unchanged (upstream's test `invalid_offsets_throw_without_changing_selection`; `packages/blitz-vibey-script/tests/selection.rs`, 15 passed); the API writes through `BaseDocument::set_text_selection`; the `innerText` / `outerText` getters added beside it are read-only; escher uses none of them (packages/blitz-vibey-script/src/dom/document.rs:75-81; packages/blitz-vibey-script/src/dom/selection.rs:69-119) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Input Validation gains a JS API arguments row for upstream's Selection API (argument count and offsets checked, JS error thrown; 15 crate tests).
  rationale: >-
    Report Changes → Symbols / APIs ("Added by upstream on the script-to-DOM crossing") and Coverage of new surfaces ("script Selection API … validation ✓ (argument count and offsets checked, a JS error thrown …) · tests integ … 15 passed"). A new script-to-DOM input surface, validated in the manner the table's JS API arguments rows record, with no row of its own — an inventory gap, not an unvalidated boundary, so warning. The write path ("`BaseDocument::set_text_selection` with 0 `mutate()` calls") is research's reading as the report relays it, not a measurement of this chunk.
  basis: "packages/blitz-vibey-script/src/dom/selection.rs:69-119"
- detector: D-security-input
  severity: warning
  section: "§Input Validation → Markup attributes (new row: `autofocus`)"
  change: >-
    New row — Markup attributes | `autofocus` | On a focusable element, enables autofocus when present with any value but `"false"`: a bare, an empty and a `"true"` attribute focus the element, `"false"` is ignored, and no attribute leaves focus unset (packages/blitz-dom/src/mutator.rs:1024-1026; tests/blitz-tests/tests/autofocus_attribute.rs:27-50) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Input Validation gains a Markup attributes row for `autofocus`: read by presence, any value but "false" (upstream #1085; 5 tests).
  rationale: >-
    Report Changes → Symbols / APIs: "a focusable element's `autofocus` attribute enables autofocus when present with any value but `"false"` (mutator.rs 1024-1026 · 1028, #1085; … `if value == "true"` → `if value != "false"`)"; Coverage: "tests integ (`autofocus_attribute.rs`, 5 passed)". The report's Expected amendments name "the `autofocus` reading" for §Input Validation. No security-plan claim is retired: its only `autofocus` mention is the Dioxus boolean-attributes row (line 135, a falsy value is removed by the bridge), which stands — the bridge sources are byte-identical per the preservation gate entry. A changed reading of document-supplied markup, validation n/a, so warning.
  basis: "packages/blitz-dom/src/mutator.rs:1024-1026"
- detector: D-security-auth
  severity: warning
  section: "§Secret Management → Environment values read (none secret-bearing)"
  change: >-
    The `env!("CARGO_MANIFEST_DIR")` bullet also names upstream's unbuilt `tests/blitz-tests/tests/all.rs` (a compile-time read in a target kept at `test = false`); upstream's delta adds no runtime `env::var` read.
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Secret Management env inventory: one compile-time `env!("CARGO_MANIFEST_DIR")` arrives in the unbuilt `tests/blitz-tests/tests/all.rs`; 0 runtime env reads added.
  rationale: >-
    Report Changes → Symbols / APIs: "No listener, bind site, runtime `env::var` read or `tracing::` call site is added by upstream's delta (… 0). Added by it: … one compile-time `env!("CARGO_MANIFEST_DIR")` in the unbuilt `tests/all.rs`." The invariant itself holds — no identity, session, token, key or secret source is touched (rustls unmoved, every upstream-only job keeps its repository guard) — this only completes the section's enumerated list; not secret-bearing. The report gives no line for the read, so the file is named without one.
- detector: D-security-deps
  severity: warning
  section: "§Dependency Security → Pinning (first bullet, git dependencies)"
  change: >-
    Git dependencies are pinned by commit rev — taffy (Cargo.toml:106); parley is no longer one: it is a registry version, `parley = { version = "0.12", … }` (Cargo.toml:116) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Dependency Security Pinning: parley moved from a git `rev` to registry 0.12 (Cargo.toml:116); taffy stays a git dependency at a new rev (Cargo.toml:106).
  rationale: >-
    Report Changes → Dependencies: "Pin form changed: `parley` is a registry version, `parley = { version = "0.12", … }` (`Cargo.toml` 116), no longer a git `rev` … `taffy` stays a git dependency, rev `4142c9d8` → `7d33901c…` (`Cargo.toml` 106)". Expected amendments: "security-plan line 229 ("Git dependencies are pinned by commit rev (Cargo.toml:106; Cargo.toml:116)" — line 116 is now a registry version)". Swept for `pinned by` / `git rev` / `parley` / `taffy`: the other hits are other subjects (a pinning test, `cross` from a pinned git rev, `git rev-parse`, stylo_taffy source citations) — no second occurrence.
  basis: "Cargo.toml:116"
- detector: D-security-deps
  severity: warning
  section: "§Dependency Security → Pinning (new bullet: dependencies entering with the 2026-10-10 upstream sync)"
  change: >-
    New bullet — the 2026-10-10 upstream sync took upstream's dependency set whole, no `cargo update` run: the root manifest gains `icu_casemap`, `icu_locale_core`, `icu_properties`, `icu_segmenter` at "2.3" and `writeable` "0.6" (Cargo.toml:202-206); six names enter the lock — `core_detect` 1.0.0, `multiversion_no_op` 1.0.0, `icu_casemap` 2.3.0, `icu_casemap_data` 2.3.0, `memchr-n` 0.1.9, `fearless_simd_macros` 0.1.0 — all crates.io with a checksum, each reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent, their source beyond the manifest not read; `jetscii` and `tinyvec_macros` leave; the audit leg reads `advisories ok` on the merged lock with no ignore added or dropped — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/lock.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Dependency Security records the sync's entering dependencies (five root-manifest ICU4X-family entries, six new lock names, two leaving) and their by-hand review; advisories ok.
  rationale: >-
    Report Changes → Dependencies ("Added to the root manifest (202-206 …) … Names entering the lock, 6 … each reviewed by hand in `evidence/lock.md` … all crates.io with a checksum. Leaving, 2") and Cross-project claims ("each crate's source beyond its manifest was not read"); Outcome 3 ("`audit` reads `advisories ok` … the merge added and dropped no ignore"). No entering name is banned — security-plan records no ban list and Security Anti-Patterns is "NO RECORDED INTENT" — so the invariant holds; the section simply carries no record of the new names (`ICU4X|icu_`: 0 hits in any master per the report). Warning.
  basis: "Cargo.toml:202-206"
- detector: D-security-deps
  severity: warning
  section: "§Dependency Security → Supply chain integrity (the NOT YET MEASURED note)"
  change: >-
    Narrow the note to "SBOM generation and base image scanning", and add a bullet — License compliance: the root `deny.toml` carries upstream's `[licenses]` table (deny.toml:22-42; `allow` 27-42, 13 expressions, `include-dev`, `include-build`, `confidence-threshold = 0.8`), byte-identical to upstream's; no CI leg runs it — no licence gate yet, the operator's decision of 2026-10-10 (upstream's `licenses` CI job was reverted); one reading, `cargo deny --locked check licenses`: exit 0, `licenses ok`, two `license-not-encountered` warnings (`NCSA`, `Unicode-DFS-2016`), over the six-target graph — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/licenses.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Dependency Security: licence compliance has one reading (`licenses ok`, exit 0) against upstream's `[licenses]` table now in deny.toml; no gate; the NOT YET MEASURED note narrows to SBOM and base image scanning.
  rationale: >-
    Report Changes → Schema / config ("`deny.toml`: a `[licenses]` table (22-42 …) byte-identical to upstream's"), Harness / gate surface ("`deny.toml` carries a licence table no leg runs"), Reverted facts (upstream's `licenses` CI job reverted: "the operator's decision of 2026-10-10 (no licence gate yet)"), the gate list ("`cargo deny --locked check licenses` — recorded: exit 0, `licenses ok`, two `license-not-encountered` warnings … a reading, no gate"). Expected amendments cite security-plan line 252 ("NOT YET MEASURED — … license compliance"), which the reading retires in part.
  basis: "deny.toml:22-42"
- detector: D-security-deps
  severity: warning
  section: "§Dependency Security → Audit tool"
  change: >-
    Add after the `deny.toml` description: the file also carries a `[licenses]` table (deny.toml:22-42) that the audit leg does not read — `bash .github/scripts/ci-leg.sh audit` still runs `check advisories` only; `[graph]` and `[advisories]` are unchanged (six targets, `all-features = true`, the one per-ID ignore RUSTSEC-2026-0192).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Dependency Security Audit tool: deny.toml now also holds a `[licenses]` table the audit leg does not run; advisories configuration unchanged.
  rationale: >-
    Second occurrence of the claim the licence proposal retires: the Audit tool paragraph (security-plan line 226) describes the root `deny.toml` as `[graph]` and `[advisories]` only. Report Schema / config: "`[graph]` and `[advisories]` are the chunk start's … No ignore added or dropped"; Harness / gate surface: "`ci-leg.sh audit` still runs `check advisories` only". The Bootstrap phases mention of `deny.toml` (line 259) says only that it is the audit tool's config and stands.
  basis: "deny.toml:22-42"
  dependent-of: D-security-deps
- detector: D-security-deps
  severity: warning
  section: "§Dependency Security → Pinning (CI tooling bullet)"
  change: >-
    Replace "the upstream-only publish-browser and wpt workflows still reference it at `@latest`" with: the upstream-only workflows no longer use `awalsh128/cache-apt-pkgs-action` — since the 2026-10-10 upstream sync bare `apt-get` installs replace it there, each job still behind its `github.repository == 'DioxusLabs/blitz'` guard (.github/workflows/publish-browser.yml; .github/workflows/wpt.yml).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Dependency Security CI tooling: the upstream-only workflows dropped the `@latest` apt-cache action for bare `apt-get` installs (by merge); repository guards kept.
  rationale: >-
    Report Changes → Harness / gate surface: "The three upstream-only workflows change by merge (bare `apt-get` installs replace the apt-cache action); every job keeps its `github.repository == 'DioxusLabs/blitz'` guard — 1, 2 and 1 — with 0 conflict markers." That retires the `@latest` claim (security-plan line 234). The report does not say which added line of either file is the install, so the files are named without one. Swept for `cache-apt` / `@latest`: this bullet only. The same bullet's ci.yml half is not this chunk's change (ci.yml: every conflict region ours, one `rustfmt` line removed).
- detector: D-security-deps
  severity: warning
  section: "§Dependency Security → Unsafe code"
  change: >-
    Add: the 2026-10-10 upstream sync adds one more `unsafe` in `stylo_taffy` — a stylo calc pointer read — beside the `LengthPercentage` block the line already cites; the count and the new site's citation are for the orchestrator's read (the report names the crate, no file or line).
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §Dependency Security Unsafe code: upstream's delta adds one `unsafe` (a stylo calc pointer read in stylo_taffy).
  rationale: >-
    Report Changes → Symbols / APIs: "Added by it: one `println!` (the WPT runner, upstream-only), one `unsafe` (a stylo calc pointer read in `stylo_taffy`)". The section's line states "one `unsafe` block" for stylo_taffy; the census reads one added. Outside the detector's invariant proper (no dependency is concerned) — raised because the claim sits in §Dependency Security and the report's Changes make it incomplete. Whether the added line is a new site or a rewritten one is not stated by the report.
```

**Dispositions** (proposal number, in list order):

1. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list — a narrowing: a typed error where there was none
2. escalate · check 1 — a boundary widening, the playbook's never-routine class: a document-supplied font source that was skipped is now fetched and decoded → resolved: the row records the widened case under a PROVISIONAL mark for the founder's ruling — word: "Record as PROVISIONAL" — the operator, 2026-10-10, through the question dialog of this wrap (`escalations.md`) → apply
3. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list — an inventory row; judged by its subject as architecture 18
4. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
5. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
6. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the citing row read `claim false` at the sweep
7. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
8. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; re-derived: whether a gate is added is an open question owned by the route entry "Quality gates", for the founder — not this wrap's to state as decided
9. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 8)
10. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the two citing rows read `claim false` at the sweep
11. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; re-derived from the read: the new site is the public `resolve_calc_value`

## design-system

**Verdict:** proposals: [] · stripped: two comment lines — D-design-tokens holds (every Coverage row reads `tokens n/a`); a note that the plan's `a[href]` expected amendment is outside the detector and that line 25 states no selector · raw twin kept

## layout-templates

**Verdict:** proposals: [] · stripped: six comment lines — D-layout-surface holds (no surface or region added); a borderline note on the WPT runner's one added `println!` under §Surface: cli, which the detector could not read · raw twin kept

## test-plan

**Verdict:** 18 proposals (18 warning) · stripped: seven comment lines — all three detectors' invariants hold (tier 0, no off-spec runner, harness untouched); the proposals are the doc's stale counts and inventories; a note that some test counts are read off the new-text listing rows

```yaml
proposals:
- detector: D-tests-coverage
  severity: warning
  section: "§4 Unit Test Strategy → What unit tests cover → CI workflows and leg script (the closing count clause)"
  change: >-
    Replace "the CI-scripts leg runs 70 tests — the 29 of the files above (test_ci_workflows.py's 25 and test_wpt_diff_to_pr.py's 4), test_agent_run.py's 14 and test_cold_agent.py's 27, as measured on the dev host (`ci-leg.sh fast`: `Ran 70 tests`, `OK`) and at CI run 37985678276" with: the CI-scripts leg runs 78 tests over six test files — test_ci_workflows.py's 25, test_wpt_diff_to_pr.py's 7 (was 4), test_agent_run.py's 14, test_cold_agent.py's 27, upstream's new test_wpt_area_changes.py's 4 and ours, test_blitz_tests_targets.py's 1 — as measured on the dev host (`ci-leg.sh fast`: `Ran 78 tests`, `OK`) and at CI#38013740580 on `9462a7e4` ("Test CI scripts"; escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md).
  sidecar: "§4 CI workflows and leg script: the leg count was 70 tests over four files (25 + 4, 14, 27); now 78 over six (25, 7, 14, 27, 4, 1), on the dev host and at CI#38013740580."
  rationale: "Report Counts / qualifiers moved: `Ran 70 tests` → `Ran 78 tests` (4 new in test_wpt_area_changes.py, 4 → 7 in test_wpt_diff_to_pr.py, 1 ours), the leg's test files 4 → 6; stated in test-plan at line 144. Harness / gate surface names the six files; test_ci_workflows.py and test_agent_run.py are unedited. Named by the plan's expected amendment (test-plan §1, §2 and §4)."
  basis: "target/ci-logs/ci-scripts.log (`Ran 78 tests`, `OK`)"
- detector: D-tests-coverage
  severity: warning
  section: "§1 Test Scope Summary → Coverage scope → CI Python scripts"
  change: >-
    "— 70 tests (§4), among them the five contract tests of the CI install script …" becomes "— 78 tests in six test files (§4), among them the five contract tests of the CI install script …"; and after the workflow-invariant clause add: and the one test of `BlitzTestsTargetsTest`, which pins tests/blitz-tests' manifest to one test target per file with upstream's `all` target unbuilt (.github/scripts/test_blitz_tests_targets.py:17-26).
  sidecar: "§1 CI Python scripts: 70 tests → 78 tests in six test files; the row names `BlitzTestsTargetsTest`."
  rationale: "Same retired count (70) restated in §1 — the report's search hit 2 lines, 8 and 144. Report Harness / gate surface: the leg discovers six test files, the new one ours."
  basis: ".github/scripts/test_blitz_tests_targets.py:17-26"
  dependent-of: D-tests-coverage
- detector: D-tests-coverage
  severity: warning
  section: "§4 Unit Test Strategy → What unit tests cover → CI Python script"
  change: >-
    The bullet keeps its three named coverages of test_wpt_diff_to_pr.py and gains upstream's: the file now holds 7 tests (was 4) — added `test_subtests_changing_in_both_directions`, `test_headline_counts_each_direction` and `test_area_lines`, over a second inline fixture `VERBOSE_ENTRIES` (.github/scripts/test_wpt_diff_to_pr.py:117-160; .github/scripts/test_wpt_diff_to_pr.py:50-101) — and a second upstream test file, test_wpt_area_changes.py, whose `CompareTest` holds 4 tests of `wpt_area_changes.py`'s `compare`: areas use only the URL path, variants stay distinct in area totals, changed areas are counted, and no change yields none (.github/scripts/test_wpt_area_changes.py:26-62); both arrived with the upstream merge of 7832c177 and run in the fork's `ci-scripts` leg, though the WPT workflows they serve run on upstream only.
  sidecar: "§4 CI Python script: test_wpt_diff_to_pr.py 4 → 7 tests (three named, the `VERBOSE_ENTRIES` fixture); new upstream file test_wpt_area_changes.py, `CompareTest`, 4 tests."
  rationale: "Report Counts / qualifiers moved (4 new in test_wpt_area_changes.py, 4 → 7 in test_wpt_diff_to_pr.py) and Harness / gate surface (upstream's test_wpt_diff_to_pr.py and the new test_wpt_area_changes.py are discovered by the leg); the bullet is test-plan's one statement of what the upstream Python tests cover and names neither the new tests nor the new file."
  basis: ".github/scripts/test_wpt_area_changes.py:26-62"
- detector: D-tests-coverage
  severity: warning
  section: "§4 Unit Test Strategy → What unit tests cover (new bullet beside \"CI workflows and leg script\")"
  change: >-
    Add a bullet, "blitz-tests targets": test_blitz_tests_targets.py — `BlitzTestsTargetsTest`, one test, reads `tests/blitz-tests/Cargo.toml` with the stdlib `tomllib` and asserts that `[package]` does not set `autotests` to false and that the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false` — the standing check on the fork's two-line difference from upstream's manifest; controlled: it read FAILED on three scratch manifests (upstream's shape; `autotests = false` alone; the table without `test = false`) and OK on the resolved one. `tomllib` needs Python 3.11 or later — 3.14.7 on the dev host; the fork's `Test CI scripts` job ran it green on `ubuntu-latest`, whose Python version was not read (.github/scripts/test_blitz_tests_targets.py:17-26) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md.
  sidecar: "§4: new bullet \"blitz-tests targets\" — `BlitzTestsTargetsTest` (one test, three red controls), and the leg's Python 3.11 floor from `tomllib`."
  rationale: "Report Harness / gate surface and Coverage of new surfaces: the one new code path that is ours alone, `tests unit (itself: 1 test in the ci-scripts leg, red on three control manifests)`; Dev-tool versions: the test imports `tomllib` (Python 3.11 or later), the runner's version not read. test-plan states neither the check nor the interpreter floor (`tomllib`, `Python 3`: 0 hits in test-plan)."
  basis: ".github/scripts/test_blitz_tests_targets.py:17-26"
- detector: D-tests-framework
  severity: warning
  section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
  change: >-
    After "`tests/blitz-tests` holds integration tests, one test-target file per behavior under `tests/`, and two shared modules that are no target …" add a third thing that is no built target: `tests/all.rs`, upstream's single-binary test target — the manifest's one `[[test]]` table, `name = "all"`, `path = "tests/all.rs"`, held with `test = false` under a comment, so cargo builds and runs no `all` binary; `[package]` sets no `autotests`, so every other `.rs` file directly under `tests/` stays its own auto-discovered target — 102 `.rs` files, 101 test targets and the unbuilt `all.rs` (was 98 files). Upstream's own layout is `autotests = false` with `all` as the crate's one test target; the fork keeps one target per file by the operator's decision of 2026-10-10, the two-line difference guarded by `BlitzTestsTargetsTest` (§4) and read at each run as 0 `Running tests/all.rs` lines and `all-target 0` over 107 executables (tests/blitz-tests/Cargo.toml:44-57; tests/blitz-tests/tests/all.rs:67-84) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md.
  sidecar: "§2 Directory pattern: `tests/all.rs`, upstream's single-binary target, stands in the manifest with `test = false` and is not built; 102 files, 101 targets; the fork's one-target-per-file layout is guarded by `BlitzTestsTargetsTest`."
  rationale: "Report Schema / config (the `[[test]]` table at 54-57 under the comment 45-53; `[package]` sets no `autotests`), Reverted / negative API facts (`autotests = false` removed), Counts (98 → 102 files, 101 targets and the unbuilt all.rs) and Decisions (the operator, 2026-10-10). The runner layout matches test-plan — upstream's was not taken — but the bullet's list of what under `tests/` is no target is now incomplete (`all.rs`, `autotests`: 0 hits in test-plan). Named by the plan's expected amendment (the unbuilt `all` target)."
  basis: "tests/blitz-tests/Cargo.toml:54-57"
- detector: D-tests-framework
  severity: warning
  section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests (the opening clause)"
  change: >-
    After "one test-target file per behavior under `tests/`, and two shared modules that are no target — `tests/common/mod.rs` … and `tests/session_common/mod.rs` …" add: and one file that is a declared target but never built, `tests/all.rs` — upstream's single-binary target `all`, kept in the manifest with `test = false` (§2 Directory pattern) (tests/blitz-tests/Cargo.toml:54-57).
  sidecar: "§1 tests/blitz-tests: the opening clause names `tests/all.rs`, declared with `test = false` and never built."
  rationale: "The same claim — everything under `tests/` is a per-behavior target bar the two shared modules — is restated in §1's tests/blitz-tests row; a single-site apply at §2 would leave it standing here."
  basis: "tests/blitz-tests/Cargo.toml:54-57"
  dependent-of: D-tests-framework
- detector: D-tests-coverage
  severity: warning
  section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests (the coverage list)"
  change: >-
    To "coverage spans accessibility, layout, text, paint, hit testing, input events, …" add the three test-target files and the one test the upstream merge of 7832c177 brought: `autofocus_attribute`, 5 — a bare, an empty and a `"true"` `autofocus` each focus the element, `"false"` is ignored and no attribute leaves focus unset, the engine now reading the attribute by presence (tests/blitz-tests/tests/autofocus_attribute.rs:27-50); `text_transform`, 3 — `math-auto` inherited and overridable, counted before whitespace collapsing, and applied to adjacent single-character text nodes (tests/blitz-tests/tests/text_transform.rs:18-47); `anonymous_block_percentage_height` — an anonymous block keeps the percentage-height containing block, for a `200px` and an `auto` container (tests/blitz-tests/tests/anonymous_block_percentage_height.rs:6-39); and in `inline_fragment_rects`, `offset_sizes_use_unrounded_border_box` over `horizontal-tb`, `vertical-lr` and `vertical-rl` (tests/blitz-tests/tests/inline_fragment_rects.rs:125-150) — the package's dev-dependency on blitz-dom gaining the `autofocus` and `text-transform-icu` features for them (tests/blitz-tests/Cargo.toml:17).
  sidecar: "§1 tests/blitz-tests: coverage gains upstream's `autofocus_attribute` 5, `text_transform` 3, `anonymous_block_percentage_height` and one `inline_fragment_rects` test over three writing modes; the dev-dependency gains two engine features."
  rationale: "Report Coverage of new surfaces: `autofocus` by presence → tests integ (autofocus_attribute.rs, 5 passed); writing modes and `text-transform` → tests unit + integ (tests/text_transform.rs 3, inline_fragment_rects +1 over three writing modes); Counts: blitz-tests 98 → 102 files; Schema / config: the dev-dependency line gains `autofocus` and `text-transform-icu` (17). The row is test-plan's inventory of what blitz-tests covers and holds none of them (`autofocus`, `text_transform`: 0 hits)."
  basis: "tests/blitz-tests/tests/autofocus_attribute.rs:27-50"
- detector: D-tests-coverage
  severity: warning
  section: "§1 Test Scope Summary → Coverage scope → blitz-vibey-script"
  change: >-
    "tests/dom.rs holds 26 `#[test]` functions covering the JS DOM APIs" becomes 27 — upstream's `computed_style_used_values` added at the file's end (packages/blitz-vibey-script/tests/dom.rs:788-824); "tests/preact.rs holds 2" stays; and add the crate's third test file: tests/selection.rs, 15 tests of the script Selection API (`document.getSelection`, `setBaseAndExtent`, `removeAllRanges`, `toString`) through one helper `select(html, script)` — identity and the empty state, UTF-16 offsets in both directions, offsets across text nodes, element children, inline roots and anonymous blocks, collapsed and preserved whitespace, case expansions and `math-auto`, recomputation after a style change, and `invalid_offsets_throw_without_changing_selection` (packages/blitz-vibey-script/tests/selection.rs:11-266). Observed absent: a test of the `innerText` / `outerText` getters that arrived in the same merge · searched: `innerText|outerText` over packages/blitz-vibey-script/tests (0 hits; whether a WPT case covers them was not read) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md.
  sidecar: "§1 blitz-vibey-script: dom.rs 26 → 27 `#[test]` functions; new tests/selection.rs, 15 tests of the script Selection API; the `innerText` / `outerText` getters observed with no test in the crate."
  rationale: "Report Coverage of new surfaces: script Selection API → tests integ (tests/selection.rs, 15 passed); `innerText` / `outerText` getters → tests ✗ in the crate's own tests (grep 0 hits). The row enumerates the crate's test files as dom.rs (26) and preact.rs (2); the merge adds one function past the cited dom.rs range and a third file. The getters are upstream's, escher uses none, and tier 0 with no critical-path list mandates no tier for them — recorded as an absence, not as a failed gate."
  basis: "packages/blitz-vibey-script/tests/selection.rs:11-266"
- detector: D-tests-coverage
  severity: warning
  section: "§5 Integration Test Strategy → Boundaries covered → Script ↔ DOM (blitz-vibey-script)"
  change: >-
    After the dom.rs coverage add: the script Selection API, a script-to-DOM crossing new with the upstream merge of 7832c177, is covered by tests/selection.rs (15) — a selection set from script by `setBaseAndExtent` is read back through `toString`, invalid offsets throw and leave the selection unchanged, and a native selection change replaces the script's (packages/blitz-vibey-script/tests/selection.rs:11-266); dom.rs gains `computed_style_used_values` (packages/blitz-vibey-script/tests/dom.rs:788-824); the `innerText` / `outerText` getters, read-only, carry no test in the crate (§1).
  sidecar: "§5 Script ↔ DOM: the Selection API covered by tests/selection.rs (15); dom.rs +1; `innerText` / `outerText` untested in the crate."
  rationale: "The claim that the crate's script-to-DOM coverage is dom.rs (and preact.rs) is restated here as the boundary's coverage statement; report Symbols / APIs lists `document.getSelection` and the `innerText` / `outerText` getters as added on the script-to-DOM crossing."
  basis: "packages/blitz-vibey-script/tests/selection.rs:11-266"
  dependent-of: D-tests-coverage
- detector: D-tests-coverage
  severity: warning
  section: "§1 Test Scope Summary → Coverage scope → blitz-dom (layout)"
  change: >-
    "one inline unit-test module, in list.rs, covering `marker_for_style`" becomes two inline unit-test modules: list.rs, covering `marker_for_style`, and upstream's new `layout/text_transform.rs`, 22 unit tests of the `text-transform` values — capitalize, uppercase and lowercase (language-sensitive included), `math-auto`, `full-width`, `full-size-kana`, their combinations, and unchanged text staying borrowed (packages/blitz-dom/src/layout/text_transform.rs:539-979); blitz-dom's lib result line reads 87 passed with them — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md. The other new layout module, `layout/writing_mode.rs`, holds no unit test; vertical writing modes are read by `inline_fragment_rects` in blitz-tests (§1 tests/blitz-tests).
  sidecar: "§1 blitz-dom (layout): one inline unit-test module → two — list.rs and upstream's `layout/text_transform.rs` (22 tests); the lib line reads 87 passed."
  rationale: "Report Crates / modules (new module `layout/text_transform.rs`, 979 lines), Coverage of new surfaces (`layout/text_transform.rs` 22 unit) and Outcome 11 (blitz-dom's unit tests 87 passed, 0 failed). The row's \"one inline unit-test module, in list.rs\" is retired by the merge."
  basis: "packages/blitz-dom/src/layout/text_transform.rs:539-979"
- detector: D-tests-coverage
  severity: warning
  section: "§4 Unit Test Strategy → What unit tests cover → blitz-dom (layout)"
  change: >-
    After "four unit tests check list markers …" add: 22 unit tests in `layout/text_transform.rs` check the `text-transform` values on text runs — capitalize by word (leading punctuation skipped, titlecase used, across text nodes and after a word break or collapsed whitespace, language-sensitive), uppercase and lowercase, the `math-auto` italic mappings per text node, `full-width` (preserved spaces kept, collapsible whitespace left) and `full-size-kana` mappings, combined transforms, the ASCII fast path matching ICU, and unchanged text returned borrowed (packages/blitz-dom/src/layout/text_transform.rs:539-979).
  sidecar: "§4 blitz-dom (layout): gains the 22 `layout/text_transform.rs` unit tests beside the four list-marker tests."
  rationale: "§4's blitz-dom (layout) bullet restates §1's layout unit coverage as list markers alone; a single-site apply at §1 would leave the two sections disagreeing."
  basis: "packages/blitz-dom/src/layout/text_transform.rs:539-979"
  dependent-of: D-tests-coverage
- detector: D-tests-coverage
  severity: warning
  section: "§1 Test Scope Summary → Coverage scope → wpt/runner"
  change: >-
    "its unit tests: 8 in fuzzy.rs, 3 in js_wrapper.rs, 2 in harness_test.rs, 1 in attr_test.rs, 1 in mod.rs" gains three modules the upstream merge of 7832c177 brought: a `tests` module of 10 functions in the new `test_variants.rs` (wpt/runner/src/test_variants.rs:159-291), one of 2 in ref_test.rs (wpt/runner/src/test_runners/ref_test.rs:356-389) and one of 1 in main.rs, `discovers_variants_and_accepts_explicit_urls` (wpt/runner/src/main.rs:854-914) — the runner now expanding a test into its declared variants.
  sidecar: "§1 wpt/runner: the unit-test list gains `test_variants.rs` (10), ref_test.rs (2) and main.rs (1), all upstream's."
  rationale: "Report Crates / modules (the WPT runner's new `test_variants.rs`, 291 lines) and Outcome 6 (all eight differing rows of the workspace tally attributed to named upstream files; 62 = 62); the report's New text section lists the three `mod tests` blocks. The row's per-file enumeration of the runner's unit tests is retired as a complete list. Counts are the listing's functions — see the note at the head."
  basis: "wpt/runner/src/test_variants.rs:159-291"
- detector: D-tests-coverage
  severity: warning
  section: "§4 Unit Test Strategy → What unit tests cover → wpt/runner"
  change: >-
    Add after the mod.rs clause: `test_variants.rs` tests variant discovery — HTML variants replacing the default with entities decoded, metadata in comments and scripts ignored, XML variants requiring the HTML namespace, XHTML with a doctype and prefixed elements, JS variants read from the initial metadata block only, an undeclared test running once, wrapper URLs keeping their suffix, the script location using the wrapper URL and variant, an artifact suffix unable to change directories, and an empty query rejected (wpt/runner/src/test_variants.rs:159-291); ref_test.rs tests that references receive the test's query and fragment variants and that an explicit reference query is preserved (wpt/runner/src/test_runners/ref_test.rs:356-389); main.rs tests that variants are discovered and explicit URLs accepted (wpt/runner/src/main.rs:854-914).
  sidecar: "§4 wpt/runner: gains what the `test_variants.rs`, ref_test.rs and main.rs unit tests cover."
  rationale: "§4's wpt/runner bullet restates the same per-file list of the runner's unit tests by what each covers; it must agree with §1 after the apply."
  basis: "wpt/runner/src/test_variants.rs:159-291"
  dependent-of: D-tests-coverage
- detector: D-tests-coverage
  severity: warning
  section: "§1 Test Scope Summary → Coverage scope → blitz-net"
  change: >-
    Keep "no test — observed absent" and add what now stands untested there: since the upstream merge of 7832c177 a `file:` URL is read through `Url::to_file_path` off wasm, a non-path URL returning an `InvalidInput` error, and the delta names no test for it (packages/blitz-net/src/lib.rs:160-169) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md; the crate's one indirect exercise in test-plan is the Preact TodoMVC fixture's `file:` base URL (§6).
  sidecar: "§1 blitz-net: the absence stands; the row names the `file:` read through `Url::to_file_path` as arrived with no test."
  rationale: "Report Coverage of new surfaces: `file:` URL read through `Url::to_file_path` (upstream) → tests ✗ (none named for it in the delta). A new path with no test; tier 0 with no critical-path list mandates none, so the row records the gap beside its existing absence rather than a failed gate."
  basis: "packages/blitz-net/src/lib.rs:160-169"
- detector: D-tests-coverage
  severity: warning
  section: "§3 Test Harness Contract → Agent-run contract → Proof"
  change: >-
    Append a clause after the 2026-10-09-audit-corrections-agent-surfaces one: re-read at 2026-10-10-upstream-sync-agent-surfaces, on the upstream merge of 7832c177 with `scripts/agent-run.sh` byte-identical: `run stand`'s `run.end` unmoved at passed 110 · failed 0 · ignored 5 over the same 30 files, and `run all`'s `run.end` reads passed 396 · failed 0 · ignored 10 — the whole package's first stated total (the 15 `ok` above is the three a11y-leg files' at 2026-10-06-stand-test-contract) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md. The earlier clauses stay as their chunks' readings.
  sidecar: "§3 Agent-run contract → Proof: re-read on the merged tree — `run stand` unmoved at 110 · 0 · 5 over 30 files; `run all` 396 · 0 · 10, its first stated total."
  rationale: "Report Counts / qualifiers moved: `run all` 396 passed, 0 failed, 10 ignored; `run stand` unmoved at 110 / 0 / 5 over 30 files; \"the `run all` total is stated in no master\" (test-plan line 115 holds a different reading). A count only, in a body line of §3 — no keyed contract's file, and the §3 ↔ obs-plan §3 bind is untouched (no event, status or verdict shape changed)."
  basis: "scripts/agent-run.sh (`run all` and `run stand`, each `run.end`)"
- detector: D-tests-coverage
  severity: warning
  section: "§9 CI Integration → Pipeline facts → Local baseline → `cargo test --workspace`"
  change: >-
    Append a re-count clause after the 2026-10-09-audit-corrections-agent-surfaces one: re-counted at 2026-10-10-upstream-sync-agent-surfaces (the upstream merge of 7832c177, 61 commits): 158 result lines, 719 passed · 0 failed · 10 ignored (+62 passed and +4 result lines, every one upstream's — four new integration-test executables, the workspace's 103 → 107: blitz-vibey-script's `selection` 15 and blitz-tests' `autofocus_attribute` 5, `text_transform` 3 and `anonymous_block_percentage_height`; the rest in existing result lines, blitz-dom's lib line reading 87 with `layout/text_transform.rs`'s 22 and `inline_fragment_rects` +1 among them; eight rows differ in all, each attributed to a named upstream file and PR; no cargo test of ours added, removed or edited, and the unbuilt `all` target is no executable), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs on the uncommitted merge and on its merge commit 9462a7e4 (escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md; the per-target tables at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/counts.md).
  sidecar: "§9 Local baseline: re-counted — `cargo test --workspace --locked` 158 result lines, 719 passed · 0 failed · 10 ignored (was 154 / 657; +62 and +4 lines, all upstream's)."
  rationale: "Report Counts / qualifiers moved: 154 → 158 result lines, 657 → 719 passed, 0 failed, 10 ignored unmoved, stated in test-plan at line 326; integration-test executables 103 → 107; Outcome 6: both per-target tables in evidence/counts.md, all eight differing rows attributed (62 = 62). Named by the plan's expected amendment (test-plan §9)."
  basis: "target/ci-logs/test.log (`lines 158 passed 719 failed 0 ignored 10`)"
- detector: D-tests-coverage
  severity: warning
  section: "§9 CI Integration → Pipeline facts → Engine features by runner"
  change: >-
    Add after the `tracing` split: since the upstream merge of 7832c177 the two runners differ in a second engine feature. The workspace build resolves blitz-dom with 16 features (was 14; `writing-mode` and `text-transform-icu` new) and the per-package build with 10 (was 8; `autofocus` and `text-transform-icu` new) — `tracing` on in the first and off in the second as before, and `writing-mode` on in the first only. `writing-mode` selects which of two bodies of the engine's bounds reader `BaseDocument::physical_unrounded_geometry` is compiled — `layout/writing_mode.rs` under the feature, `document.rs` without it (packages/blitz-dom/src/layout/writing_mode.rs:371-401; packages/blitz-dom/src/document.rs:2254-2270) — and `visible_region` and every snapshot `bounds` read through it, so the same stand check runs over a different reader under each runner: `scroll_into_view_nested` 7, `stand_act_scroll` 4 and `stand_act_obstructed` 3 pass under both, and a green per-package run is still not the workspace leg's green (as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/features.md, from `cargo tree --locked … -e features -i blitz-dom --depth 1`).
  sidecar: "§9 Engine features by runner: blitz-dom resolves 16 features under the workspace build and 10 per package; `writing-mode` is on in the first only, so the two runners compile different bodies of the bounds reader — the stand's scroll and obstruction checks pass under both."
  rationale: "Report Counts / qualifiers moved: blitz-dom features by runner 14 → 16 and 8 → 10, `tracing` as before, stated in test-plan at line 320; Symbols / APIs: `physical_unrounded_geometry` has two bodies by `writing-mode`, and `visible_region` (our one edit, scrolling.rs 776-780) reads through it; Coverage of new surfaces: tests integ \"under both bodies of the reader — the workspace build with `writing-mode`, the per-package build without\". Named by the plan's expected amendment (features 16 and 10). obs-plan restates only the `tracing` half, which did not move."
  basis: "packages/blitz-dom/src/layout/writing_mode.rs:371-401"
- detector: D-tests-coverage
  severity: warning
  section: "§9 CI Integration → Pipeline facts → Cache"
  change: >-
    Append to the wall-clock readings: the upstream merge's run, CI#38013740580 on `9462a7e4` (16 of 16 jobs green at its first attempt), took 1100 s wall, a cold run, against 641 s for CI#38010081458 on the chunk start `09f479b8` — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/ci.md.
  sidecar: "§9 Cache: adds the merge run's wall-clock — CI#38013740580, 1100 s cold, against 641 s on the chunk start."
  rationale: "Report Counts / qualifiers moved: fork CI wall-clock on the merge commit 1100 s, a cold run, against 641 s on the chunk start, stated in test-plan at line 313 (the bullet that holds the fork's cold / warm / wall readings); Cross-project / external claims carries both run ids and shas. Named by the plan's expected amendment (wall 1100 s). The report does not say why the run was cold, so the clause states no cause."
  basis: "escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/ci.md"
```

**Dispositions** (proposal number, in list order):

1. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
2. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 1)
3. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
4. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
5. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
6. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 5)
7. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
8. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
9. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 8)
10. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
11. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 10)
12. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the three counts re-read by the orchestrator before they are written
13. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 12)
14. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
15. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
16. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
17. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list
18. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list

## obs-plan

**Verdict:** proposals: [] · stripped: four comment lines — D-obs-instrumentation, D-obs-stack and D-obs-pii each hold, with the report's sentences; a note that its citations move with the sweep · raw twin kept

## a11y-plan

**Verdict:** 2 proposals (2 warning) · stripped: four comment lines — D-a11y-obs-schema holds; no interactive element added, the two proposals filed under D-a11y-surface on a loose reading for the plan's §5 expected amendment; §1 needs no proposal by the detector's reading

```yaml
proposals:
- detector: D-a11y-surface
  severity: warning
  section: "§5 Keyboard Navigation → Focus order per layout (engine) — the `autofocus` bullet (a11y-plan line 147)"
  change: >-
    Under the `autofocus` feature, the latest mounted focussable node whose `autofocus` attribute is present with any value but "false" — bare, empty or "true" — is focused on flush, and `autofocus="false"` is ignored (packages/blitz-dom/src/mutator.rs:1024-1028; tests/blitz-tests/tests/autofocus_attribute.rs:27-50) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §5 Focus order: autofocus is keyed on the attribute's presence with any value but "false" (upstream #1085), no longer on the value "true"; new check autofocus_attribute.rs, 5 passed.
  rationale: >-
    The body says the node "with `autofocus="true"`" is focused on flush. Report, Changes → Symbols / APIs, "Engine behaviour changed by upstream": "a focusable element's `autofocus` attribute enables autofocus when present with any value but "false" (`packages/blitz-dom/src/mutator.rs` 1024-1026 · 1028, #1085; upstream's diff of the line: `if value == "true"` → `if value != "false"`)". Coverage of new surfaces: "`autofocus` by presence (upstream) → tests integ (`tests/blitz-tests/tests/autofocus_attribute.rs`, 5 passed) · a11y focus✓ for the engine (no stand source carries `autofocus`, so no stand boot focus moves)". Expected amendments names "a11y-plan §5 … carried: Symbols / APIs (`autofocus`)". This is not a new interactive UI element; it is the §5 focus claim the merge made false. The bullet's second citation (mutator.rs:941-946) and the 1021-1033 range are pre-merge lines — left to the citation sweep.
  basis: "packages/blitz-dom/src/mutator.rs:1024-1028 (report: added 1024-1026 · 1028); tests/blitz-tests/tests/autofocus_attribute.rs:27-50 (bare / empty / true focus, false ignored, none leaves focus unset)"
- detector: D-a11y-surface
  severity: warning
  section: "§5 Keyboard Navigation → Script and framework exposure — the `autofocus` reflection bullet (a11y-plan line 172)"
  change: >-
    `autofocus` reflection writes the value "true", one of the values blitz-dom's autofocus handling accepts — it takes the attribute present with any value but "false" (§5 Focus order); blitz-dom is used with feature `autofocus` (packages/blitz-vibey-script/src/dom/element.rs:487; packages/blitz-vibey-script/Cargo.toml:19)
  sidecar: >-
    2026-10-10-upstream-sync-agent-surfaces — §5 Script exposure: the reason clause "because blitz-dom's autofocus handling expects it" is retired; the reflection still writes "true", and the handling now accepts any present value but "false".
  rationale: >-
    Second occurrence of the retired claim that blitz-dom's autofocus handling requires the value "true". The report's Expected amendments quotes this line ("172 reads "`autofocus` reflection writes the value "true" because blitz-dom's autofocus handling expects it"; the reflection still writes `"true"`, `packages/blitz-vibey-script/src/dom/element.rs` 487"). The written value is unchanged; the stated reason is no longer true after #1085. Sweep of the other four `autofocus` hits found nothing else to amend: 177 (the feature forwards to blitz-dom), 179 (a falsy Dioxus value removes the attribute), 184 and 189 (app inputs autofocus) do not assert the "true"-only mechanism, and the report changes none of them.
  basis: "packages/blitz-vibey-script/src/dom/element.rs:487 (stated in the report's Expected amendments bullet for a11y-plan)"
  dependent-of: D-a11y-surface
```

**Dispositions** (proposal number, in list order):

1. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list; the citing row read `claim false` at the sweep
2. apply · check 1 — routine: an accurate this-chunk addition, named by the plan's reviewed list (dependent of 1); the citing row read `claim false` at the sweep

## Raised by the orchestrator (check 5)

- design-system §Color Palette → Core Colors, the "blitz-dom default link" row — the rule selects `a[href]` (the plan's expected amendment; no detector proposed it; the report's Symbols bullet and the sweep's read of default.css 42-46 carry it) → apply · check 5, routine
- a11y-plan §1 → Dioxus crates, the adapter-absence and lockfile sentences — re-read on the merged lock (the plan's expected amendment; the report's gate entry reads 0 `accesskit_xplat` / `accesskit_winit` rows in the stand's graph, and its Dependencies bullet carries the family's bumps) → apply · check 5, routine
- architecture §Conventions → Licensing exceptions — `wasm_hello` now inherits the workspace licence (`citation-dispositions.md`, the row at architecture.md:107; the report's listing carries the file's one added line) → apply · check 5, routine
- architecture §Established Decisions → [Default features], §Occupied Resources → Names, §Standard Contracts → CI contracts and Layout, §Existing Scopes; security-plan §Input Validation and §Dependency Security; a11y-plan §5 — every other `claim false` row of `citation-dispositions.md` is met by a proposal above and settled with it

## Disproved claims (check 6)

- research.md, the `Cargo.lock` bullet ("the 3 entries ours added") — the count is 2: a chunk artifact, frozen; no master states the 3 (searched: `3 entries|entries ours` over the seven masters and the registries, 0 hits) → DISPOSED: the report is its record
- plan.md, the last `[[gate]]` entry and the acceptance that names its count — the pattern cannot match a CI log → DISPOSED: the operator's word (inputs#I3) accepts the second reading; the standing hazard goes to curation (P3) and its owner to route-resolve (P5)
- the scope's hypothesis that Parley 0.12 and the Taffy bumps would move a stand bound or a snapshot's size — measured, none moved → DISPOSED: no master states the hypothesis; the reading stands in `evidence/moved-readings.md`

## Checks 2 · 3 · 4

- Cross-contradiction: none — no two proposals edit one sentence in opposing directions; architecture 8 and 19, and security-plan 8 and 9, edit different sentences of one fact and agree.
- Intent-consistency: the report's six deviations each rest on the plan or on the operator's recorded word (inputs#I2, inputs#I3); the scope record is absent and `gate.py scope` reads clean, so no line is outside the intent.
- Absence needs evidence: the report's absence claims each cite their search; this pass's own caught-all claim is the cascade sweep's, written to `cascade-dispositions.md` after it runs.
