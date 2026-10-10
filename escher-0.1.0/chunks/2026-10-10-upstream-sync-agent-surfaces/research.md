# Codebase Research — 2026-10-10-upstream-sync-agent-surfaces

## Scope
- **Depth:** deep · **Reads:** 16 (the 7 extracts and 7 history files are counted at P2; here: the three earlier syncs' scope / research / report, `.github/scripts/test_ci_workflows.py` 20-241, `.github/scripts/ci-leg.sh` 1-70, both `deny.toml`s, `packages/blitz-dom/src/scrolling.rs` 741-806, upstream's `layout/writing_mode.rs` 355-400, upstream's `tests/all.rs`, the code-graph cookbook, the harness rule file, the last chunk's `## Test Commands`) · **Globs/Greps:** 44 (git diff / grep / show probes over the merge base `23354585`, HEAD `09f479b8`, upstream `7832c177` and the in-memory trial merge tree `31853906`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (full read, 1 Session Addition: the gate tool reads an entry's exit 124 or 137 as its own bound — no entry here returns one) · `.claude/rules/testing.md` (full, 5 Session Additions; two apply: result lines of a red `cargo test` run are counted only under `--no-fail-fast`, so the count entry reads a green run's log and a red run is never compared by count; a helper in a `.github/scripts` unittest file takes no `test_` prefix, which binds any edit of `test_ci_workflows.py`)
- **Platform issues consulted:** the signature of the two folded runner-only bullets — jobs concluded `cancelled` seconds after a newer push to the same ref, under `concurrency … cancel-in-progress: true` → GitHub Docs, "Control the concurrency of workflows and jobs" (fetched 2026-10-10) states: "To also cancel any currently running job or workflow in the same concurrency group, specify `cancel-in-progress: true`." The page states no conclusion word for a cancelled run; the word `cancelled` is read from the two runs' own job records (below). No instrumentation is planned for these bullets, and the plan's CI-reading entries are the operator leg's
- **External inputs:** `inputs#I1` — the orchestrator's measurement of `upstream/main` at take-up: `7832c177`, 61 commits past the merge base `23354585`, ours 65 past, 72 files +5672 −876, 16 files changed by both sides, 4 conflicted in the trial merge. Re-derived at P3 from this checkout: `git ls-remote upstream refs/heads/main` → `7832c177…` (2026-10-10T00:58Z), `git merge-base HEAD upstream/main` → `23354585…`, `git rev-list --count` → 61 / 65, `git merge-tree --write-tree --name-only --no-messages HEAD 7832c177…` → exit 1, tree `318539062b9f124fd4aaf4dc7f70ec3b73a4b9dd`, the same 4 files

## Files inspected
- **In-memory trial merge** (`git merge-tree`, tree `31853906`; the working tree was not touched). Conflict regions, read from the tree's blobs:
  - `Cargo.toml` — 1 region. Ours ends the workspace dependency block with `[profile.dev]` / `debug = "line-tables-only"` (HEAD `:201-202`); upstream appends five ICU4X entries (`icu_casemap`, `icu_locale_core`, `icu_properties`, `icu_segmenter`, `writeable`) at the same place. Both stay: the five entries, then our stanza. Upstream's manifest carries no `[profile.dev]` of its own (`git show 7832c177:Cargo.toml | grep -n '^\[profile'`: `profile`, `production`, `p2`, `small`, `small-panic`, `tiny` only), so one statement of the debuginfo level stands in the manifest.
  - `tests/blitz-tests/Cargo.toml` — 1 region, the `blitz-dom` dev-dependency line: ours adds `escher-telemetry` above it, upstream adds the features `autofocus` and `text-transform-icu`. Both stay. Outside the region the merged manifest takes `autotests = false` and `[[test]] name = "all"`, `path = "tests/all.rs"` with no conflict.
  - `deny.toml` — 1 region, the whole file (add/add). Ours: a six-target `[graph]` with `all-features = true`, `[advisories]` with one per-ID ignore (`RUSTSEC-2026-0192`). Upstream: `[graph]` with `all-features = true` and no targets, `[licenses]` with a 13-entry allow list, `include-dev`, `include-build`, `confidence-threshold = 0.8`. The two do not overlap in a key but `[graph]`: a resolved file holds our `[graph]`, our `[advisories]` and upstream's `[licenses]`.
  - `.github/workflows/ci.yml` — 6 regions: the MSRV job's steps, the block from the default build job through "Test [default features]" and the counter job, the clippy steps, the new `linux` matrix row, the "Free Disk Space" step, the matrix job's package install.
- **What the merged `ci.yml` takes with no conflict**, each against a fork invariant (`.github/scripts/test_ci_workflows.py`):
  - a job `licenses` ("Dependency licenses", `EmbarkStudios/cargo-deny-action@v2`, `actions/checkout@v4`) — fails `test_j_every_action_is_pinned_to_a_commit_sha` (`:184`), `test_c_slow_jobs_need_every_fast_job` (`:135`, it carries no `needs`) and `test_d_linux_jobs_run_their_leg_through_the_script` (`:146`, the linux job set must equal `LINUX_JOB_LEGS`)
  - `env: CARGO_PROFILE_DEV_DEBUG: "line-tables-only"` — a second statement of the level the manifest stanza already sets; architecture's history records the stanza as chosen over this exact env form
  - 4 `uses:` spellings at a mutable ref and 8 bare `apt-get` lines over the whole merged text, the regions' upstream sides included (`test_o_package_installs_are_bounded`, `:216`, allows none), and the runner label `warp-windows-2025-x64-8x`
  - upstream's `linux` matrix row — `test_i_matrix_keeps_every_platform_but_linux` (`:180`) pins the matrix to windows, macos, ios, android
- **The three upstream-only workflows in the trial merge**, read as YAML: `publish-browser.yml` 1 job, `wpt.yml` 2, `wpt-post-results.yml` 1 — every job still carries `github.repository == 'DioxusLabs/blitz'`, 0 conflict markers, the `always()` and `secrets.` counts equal HEAD's (2 / 7, 0 / 1, 0 / 3). Upstream's delta there replaces the apt-cache action with a bare `apt-get` line; those files are outside `CiWorkflowTest`, which reads `ci.yml` alone.
- **`Cargo.lock` in the trial merge** — 0 conflict markers. Package set: HEAD 890 names / 992 entries, upstream 892 / 990, merged 894 / 992; the merged `name`+`version`+`source` set equals upstream's set plus the 3 entries ours added since the base, exactly. Against HEAD 6 names enter (`core_detect` 1.0.0, `fearless_simd_macros` 0.1.0, `icu_casemap` 2.3.0, `icu_casemap_data` 2.3.0, `memchr-n` 0.1.9, `multiversion_no_op` 1.0.0) and 2 leave (`jetscii`, `tinyvec_macros`). Moved: `parley` and `fontique` 0.11.0 (git) → 0.12.0 (registry), `accesskit` 0.25.0 → 0.25.1, `accesskit_unix` 0.23.0 → 0.24.0, `accesskit_android` 0.8.0 → 0.9.0, `comrak` 0.55.0 → 0.56.0. Unmoved: `skrifa` 0.44.0 (one version), `vello` 0.11.0, `vello_cpu` 0.3.0, `harfrust` 0.12.0, `taffy` 0.14.0 (git, new rev), `winit` 0.31.0-beta.3, `stylo` 0.22.0, the html5ever family, `svgtypes` / `usvg`, `tracing` 0.1.44, `tracing-subscriber` 0.3.23, `tracing-log` 0.2.0, `ttf-parser` 0.25.1 (the ignored advisory is still reached), `rustls` 0.23.45 (the recorded fix is kept), `libc` 0.2.189. No OTel, Sentry, Prometheus or metrics-exporter name. Not measured: `--locked` resolution — the tree's manifests carry conflict markers.
- **The engine sources both sides changed**, ours against upstream's hunks (`git diff -U0` of each side from the base):
  - `packages/blitz-dom/src/mutator.rs` — ours 15 three-line marks, upstream 6 hunks (a custom-widget early return in the attribute path, the tag post-processing, the `autofocus` reading). The merged blob holds 15 `changed_nodes.insert` lines, as HEAD does; upstream's new early return (merged `:386-390`) sits after the mark of that method (`:324`). Upstream adds no mutation method.
  - `packages/blitz-dom/src/document.rs` — ours 7 hunks (the trait method, the set, the drain, the unit tests), upstream 8 (a style pref, the image arm, the geometry readers `:2226-2358`). Changed-set lines 37 at HEAD and 37 merged.
  - `packages/blitz-dom/src/node/node.rs` — ours 2 rustdoc lines, upstream 4 hunks, one of them the removal of `unrounded_absolute_position`.
  - `packages/blitz-shell/src/window.rs` — ours 4 hunks in `View::poll` (`:376-381`, `:523`), upstream 1 at `:683` (`print_taffy_tree` through `inner_mut`). Changed-set lines 3 and 3.
  - `packages/blitz-dom/src/layout/replaced.rs` — ours 1 line at `:20`, upstream 2 hunks at `:203-216`.
- **`packages/blitz-dom/src/scrolling.rs` 741-806** — `visible_region`, ours: per clipping box it reads `holder.unrounded_absolute_position(own_scroll.x, own_scroll.y)` (`:776-777`) and `holder.unrounded_layout()` for border, size and scrollbar. Upstream's replacement, `BaseDocument::physical_unrounded_geometry(node_id) -> (Point<f32>, taffy::Layout)`, is `pub(crate)` and has two bodies: `document.rs` under `not(feature = "writing-mode")` and `layout/writing_mode.rs:375` under the feature. Both sum `location − scroll_offset` over the containing-block chain, the node's own scroll offset included, which is what our call cancelled by passing the offset in.
- **`tests/blitz-tests/tests/all.rs` at `7832c177`** — 62 `mod` lines for upstream's 62 other files, and the test `all_test_files_are_included`, which reads `CARGO_MANIFEST_DIR/tests`, skips entries without an `.rs` extension (so a subdirectory) and asserts each other stem appears as `\nmod {name};` in the file's own text.
- **Our test files** — 41 paths added under `tests/blitz-tests/tests/` since the base: 39 top-level `.rs` files and the two shared modules (`common/mod.rs`, `session_common/mod.rs`); 98 `.rs` files at HEAD against upstream's 63. In ours: 15 files read `mod common;`, 15 `mod session_common;`; 5 sites re-run `std::env::current_exe()` with `--ignored --exact {child} --nocapture` (`telemetry_stdout_silent.rs:10`, `telemetry_drop.rs:18`, `stand_id_persistence.rs:291`, `stand_act_spans.rs:484`, `session_common/mod.rs:244`), two of them with the same child name `child_emits`.
- **Upstream's test files stand alone**: no file under `tests/blitz-tests/tests/` at `7832c177` but `all.rs` uses a `crate::` or `super::` path (`git grep -n -E '\b(super|crate)::' 7832c177 -- 'tests/blitz-tests/tests/*.rs'`: 0 hits outside `all.rs`), so each still compiles as a crate root of its own.
- **Target-named contracts** — `.github/scripts/ci-leg.sh:34` (`a11y`: three `--test` names, all three upstream files, all three `mod` lines of `all.rs`); `scripts/agent-run.sh:10` (`TESTS_DIR`), `:172` (`boot`: `cargo test -p blitz-tests --locked --no-run`), `:181-204` (`stand` globs `stand_*.rs` and passes one `--test` per file; `{name}` likewise).
- **Upstream's CI-script tests** — `.github/scripts/test_wpt_diff_to_pr.py` goes from 4 unittest methods to 7 and `test_wpt_area_changes.py` arrives with 4 (`grep -c '^    def test_'` on each blob). The `ci-scripts` leg discovers every `test*.py` there, so its tally is expected to read 77 where it reads 70 today (25 + 14 + 27 + 4): a prediction from method counts, not a run.
- **Features** (`git grep` over `*.toml` at `7832c177`): `writing-mode` is turned on for blitz-dom by `wpt/runner/Cargo.toml:15` alone among the default build's manifests (`apps/browser` and the façade crates only forward it); `autofocus` by blitz-vibey-script and by the merged blitz-tests line; `text-transform-icu` is a default of blitz, dioxus-native and dioxus-native-dom. `wpt/runner` is a workspace member (`Cargo.toml:23`).
- **Census of upstream's delta** (`git diff 23354585 7832c177 -- packages apps examples wpt tests`): log or print sites added 1 (`println!("{}", test.url)` in the WPT runner, an upstream-only binary the sink is not installed in); no `tracing::` site added; `unsafe` 1 (a stylo calc pointer read in stylo_taffy); bind or listen sites 0; env reads 1 (`env!("CARGO_MANIFEST_DIR")` in `all.rs`, compile-time, a name already registered); workspace members unchanged. `packages/blitz-dom/assets/default.css`: the link rule's selector `a` → `a[href]`, colour and decoration unchanged. No stand source holds an `a` element (`grep -rn -E '\ba \{|<a |"a"' examples/seven_guis/src`: 0 hits) and none holds `autofocus`.
- **Upstream's script additions** — `dom/selection.rs` writes through `BaseDocument::set_text_selection` (4 `inner_mut` sites, 0 `mutate()` calls); `inner_text.rs` reads only. Neither adds a path that mutates nodes outside `DocumentMutator`.
- **Licence fields** — `escher-telemetry`, `escher-driver`, `seven_guis` and `blitz-tests` all read `license.workspace = true`, and the workspace's is `MIT OR Apache-2.0` (`Cargo.toml:37`), inside upstream's allow list. Whether the whole six-target graph passes that list is not measured.
- **CI verdicts** — `09f479b8` green, 16/16, wall 641 s (CI#38010081458, re-read 2026-10-10T00:58Z); the two cancelled runs unchanged. Their job records (`gh run view` by run id): in CI#37993529819 the cancelled jobs ended 21:34:34–21:34:43Z, 35–44 s after the next commit; in CI#38009711913 at 00:41:01–00:41:07Z, 19–25 s after it; every other job success. `ci.yml:11-13` sets `cancel-in-progress: true` per ref.
- **No code moved since the last recorded baseline**: `git diff --stat b03a5fc7 HEAD -- packages examples apps tests wpt Cargo.toml Cargo.lock .github deny.toml scripts` lists `scripts/code-graph.py` alone, so test-plan §9's last figures (154 result lines, 657 passed · 0 failed · 10 ignored; `run stand` 110 passed · 0 failed · 5 ignored over 30 files; `Ran 70 tests`; the a11y leg 6 + 6 + 3) describe HEAD.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-10T00-42-37-phase/tree-query-2026-10-10-upstream-sync-agent-surfaces.json`, rust plane regenerated: 6722 nodes / 37402 edges)
- **unrounded_absolute_position** — 5 call sites: `BaseDocument::get_client_bounding_rect` @ `packages/blitz-dom/src/document.rs:2257`, `offset_rect` @ `:2307`, `inline_fragment_rects` @ `:2347` (all three rewritten by upstream in the same delta), the method's own recursion @ `packages/blitz-dom/src/node/node.rs:1588` (removed with it), and `BaseDocument::visible_region` @ `packages/blitz-dom/src/scrolling.rs:777` — ours, in a file upstream does not touch: the one caller the merge leaves dangling.
- **visible_region** — 2 call sites: `escher_driver` `execute::in_view` @ `packages/escher-driver/src/execute.rs:316` (the `off-screen` reading) and `reader_holds_the_target` @ `tests/blitz-tests/tests/scroll_into_view_nested.rs:120`. Its signature does not change; the edit is inside its body, and `scroll_into_view_nested` (7 tests) with the `stand_act_scroll` / `stand_act_obstructed` checks are its standing proof.
- **get_client_bounding_rect** — 12 call sites, among them `snapshot::Builder::visit` @ `packages/dioxus-native-dom/src/snapshot.rs:127` and `controls_lie_inside_the_viewport` @ `tests/blitz-tests/tests/stand_snapshot.rs:200`. Signature unchanged; upstream rewrites its body onto the new reader, so every snapshot `bounds` is read through code this merge changes.
- A name-level census of the delta (`fn` names on a removed line and on no added line, over `git diff 23354585 7832c177 -- packages`) finds one name: `unrounded_absolute_position`. It cannot see a signature whose first line is unchanged; the compile is the proof for those.

## Patterns detected
- **A clean textual merge that is wrong** occurs twice in this delta and is the class to look for: a file neither side conflicts in that calls something the other side removed (`scrolling.rs:777`), and a workflow that takes a whole new job between our jobs (`ci.yml`, `licenses`).
- **The fork's CI shape is stated in tests, not prose** (`.github/scripts/test_ci_workflows.py:23-64`: `FAST_JOBS`, `LINUX_JOB_LEGS`, `INSTALL_JOBS`, `APT_ACTION_JOBS`). Each upstream job change of this delta is refused by one of them as written, so the resolution of `ci.yml` is ours on every region, with nothing of upstream's job set taken, and the test file is not edited unless the operator takes the license job.
- **Upstream's guard makes the test-shape break loud**: `all_test_files_are_included` turns an unregistered test file into a red run. The silent case is elsewhere — a child selected by `--exact {name}` that matches nothing still exits 0 — and exists only if our files join upstream's binary.
- **The fork's standing position on one binary for blitz-tests is "deferred until measured"** (architecture's history, the dev-profile entry of 2026-10-05: consolidating the then-59 binaries was left until measured after cache and debuginfo; the debuginfo change alone took the cold local baseline from 2239.59 s to 161.36 s).

## Conventions to follow
- **Gate through the leg runner**: `bash .github/scripts/ci-leg.sh {fast|doc|audit|a11y}`, logs under `target/ci-logs/{leg}.log` (`.github/scripts/ci-leg.sh:8-36`).
- **Agent-run proof**: `bash scripts/agent-run.sh boot`, then `run stand` and `run all`; the outcome is read from `run.end`, and an empty run is exit 1 (`.claude/rules/verification-harness.md` §The 5-command contract).
- **Counts are re-read and attributed**, never copied: each delta against the last recorded baseline is traced to a named upstream commit or file (the first sync's precedent, test-plan §9 Local baseline).
- **A stand number the pins move is re-measured and restated in the shared tables** (`tests/blitz-tests/tests/common/mod.rs`), never loosened per test.
- **Evidence as dated readings** under `chunks/{marker}/evidence/` — shas, verdicts and counts only; no host path and no raw log.
- **The merge's verdict is read on a run left to settle**: nothing is pushed on top of the merge commit until its run concludes.

## New files to create
- `escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/` — the pre- and post-merge readings: the resolved conflicts, the lock and feature reads, the counts with their attribution, the moved stand numbers, the CI rows
- `.github/scripts/test_blitz_tests_targets.py` — ours: the standing check of the blitz-tests manifest's target shape, run by the `ci-scripts` leg (the P4 decision)
- `.github/scripts/test_wpt_area_changes.py` — upstream, by merge
- `.github/scripts/wpt_area_changes.py` — upstream, by merge
- `docs/licensing.md` — upstream, by merge
- `packages/blitz-dom/src/layout/text_transform.rs` — upstream, by merge
- `packages/blitz-dom/src/layout/writing_mode.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/selection.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/inner_text.rs` — upstream, by merge
- `packages/blitz-vibey-script/tests/selection.rs` — upstream, by merge
- `packages/stylo_taffy/src/writing_mode.rs` — upstream, by merge
- `tests/blitz-tests/tests/all.rs` — upstream, by merge
- `tests/blitz-tests/tests/anonymous_block_percentage_height.rs` — upstream, by merge
- `tests/blitz-tests/tests/autofocus_attribute.rs` — upstream, by merge
- `tests/blitz-tests/tests/text_transform.rs` — upstream, by merge
- `wpt/runner/src/test_variants.rs` — upstream, by merge

## Files to modify
- `.github/workflows/ci.yml` — conflict: every region resolved to the fork's side; of the four changes the merge takes outside the regions (the `licenses` job, the debuginfo env line, the WarpBuild Windows runner label, the matrix toolchain step's `components: rustfmt` line dropped) only the last is kept
- `Cargo.toml` — conflict: upstream's ICU4X entries, Taffy rev and Parley version, our profile stanza and escher entries
- `deny.toml` — conflict: our graph and advisories, upstream's license table
- `tests/blitz-tests/Cargo.toml` — conflict: both sides of the dev-dependency line; the test-target lines as the operator decides
- `Cargo.lock` — by merge; re-resolved only if `--locked` refuses the merged file
- `packages/blitz-dom/src/scrolling.rs` — ours: the one call in `visible_region` moved to upstream's replacement reader
- `tests/blitz-tests/tests/common/mod.rs` — only if a pinned stand number moves under the new pins: the restated row
- `.github/scripts/test_wpt_diff_to_pr.py` — upstream, by merge
- `.github/scripts/wpt_diff_to_pr.py` — upstream, by merge
- `.github/workflows/publish-browser.yml` — upstream, by merge (the guard kept)
- `.github/workflows/wpt-post-results.yml` — upstream, by merge (the guard kept)
- `.github/workflows/wpt.yml` — upstream, by merge (the guards kept)
- `CONTRIBUTING.MD` — upstream, by merge
- `apps/browser/Cargo.toml` — upstream, by merge
- `apps/readme/Cargo.toml` — upstream, by merge
- `examples/screenshot.rs` — upstream, by merge
- `examples/wasm_hello/Cargo.toml` — upstream, by merge
- `packages/accesskit_xplat/Cargo.toml` — upstream, by merge
- `packages/blitz-dom/Cargo.toml` — upstream, by merge
- `packages/blitz-dom/assets/default.css` — upstream, by merge
- `packages/blitz-dom/src/debug.rs` — upstream, by merge
- `packages/blitz-dom/src/document.rs` — upstream, by merge (our changed-set hunks kept)
- `packages/blitz-dom/src/layout/construct.rs` — upstream, by merge
- `packages/blitz-dom/src/layout/inline.rs` — upstream, by merge
- `packages/blitz-dom/src/layout/mod.rs` — upstream, by merge
- `packages/blitz-dom/src/layout/replaced.rs` — upstream, by merge (our line kept)
- `packages/blitz-dom/src/layout/table.rs` — upstream, by merge
- `packages/blitz-dom/src/lib.rs` — upstream, by merge
- `packages/blitz-dom/src/mutator.rs` — upstream, by merge (our 15 marks kept)
- `packages/blitz-dom/src/net.rs` — upstream, by merge
- `packages/blitz-dom/src/node/element.rs` — upstream, by merge
- `packages/blitz-dom/src/node/node.rs` — upstream, by merge (our two rustdoc lines kept)
- `packages/blitz-dom/src/node/text.rs` — upstream, by merge
- `packages/blitz-dom/src/resolve.rs` — upstream, by merge
- `packages/blitz-dom/src/resolved_style.rs` — upstream, by merge
- `packages/blitz-net/src/lib.rs` — upstream, by merge
- `packages/blitz-paint/src/text.rs` — upstream, by merge
- `packages/blitz-shell/src/window.rs` — upstream, by merge (our poll hunks kept)
- `packages/blitz-vibey-script/Cargo.toml` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/document.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/element.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/mod.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/lib.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/runtime.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/state.rs` — upstream, by merge
- `packages/blitz-vibey-script/tests/dom.rs` — upstream, by merge
- `packages/blitz/Cargo.toml` — upstream, by merge
- `packages/dioxus-native-dom/Cargo.toml` — upstream, by merge
- `packages/dioxus-native/Cargo.toml` — upstream, by merge
- `packages/stylo_taffy/Cargo.toml` — upstream, by merge
- `packages/stylo_taffy/src/convert.rs` — upstream, by merge
- `packages/stylo_taffy/src/lib.rs` — upstream, by merge
- `packages/stylo_taffy/src/wrapper.rs` — upstream, by merge
- `tests/blitz-tests/tests/inline_fragment_rects.rs` — upstream, by merge
- `wpt/WPT_COMMIT` — upstream, by merge
- `wpt/runner/Cargo.toml` — upstream, by merge
- `wpt/runner/src/main.rs` — upstream, by merge
- `wpt/runner/src/test_runners/js_wrapper.rs` — upstream, by merge
- `wpt/runner/src/test_runners/mod.rs` — upstream, by merge
- `wpt/runner/src/test_runners/ref_test.rs` — upstream, by merge

## Open questions
- How are the blitz-tests files built after the merge? → blocks: plan-decision. **Decided at P4** (the operator, 2026-10-10, by question dialog): every file stays its own target — upstream's `autotests = false` is not taken and its `all` target stays in the manifest with `test = false` — with two directions: the plan says how the standing two-line difference is kept visible for the next sync, and the reading that `test = false` keeps `all.rs` out of every leg is a gate entry. So `scripts/agent-run.sh`, `.github/scripts/ci-leg.sh`, `.github/scripts/test_agent_run.py` and the files under `tests/blitz-tests/tests/` are not written by this decision, and one new file carries the standing check (listed above). Still not measured: that `test = false` does what the decision needs — the plan's gate entries read it.
- Does upstream's license check run in the fork's CI? → blocks: plan-decision. **Decided at P4** (the operator, 2026-10-10, by question dialog): no gate yet — the resolved `deny.toml` keeps upstream's `[licenses]` table, upstream's CI job is not taken, and /implement takes one `cargo deny check licenses` reading into evidence; the gate question goes to the route's "Quality gates" entry at the wrap. `.github/scripts/test_ci_workflows.py` is therefore not written.
- If a gate goes red on the merged tree beyond the one known break — fmt or clippy on upstream's new code under `-D warnings` and toolchain 1.99.0, a signature the name census could not see, a stand number the pins move — the fix's file is not knowable until the gate runs. → blocks: implementation-scope. The list above is provisional by that class: each such edit is named, minimal and recorded as non-additive.
