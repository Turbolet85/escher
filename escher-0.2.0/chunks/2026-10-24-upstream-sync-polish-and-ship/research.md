# Codebase Research — 2026-10-24-upstream-sync-polish-and-ship

## Scope
- **Depth:** deep (four trial takes built and gated in a throwaway worktree) · **Reads:** 19 · **Globs/Greps:** 21
- **Resumed on overseer1's ruling:** this P3 runs on P2's validated products in `.andromeda/runs/2026-10-24T09-31-30-phase/` (Setup 3 (c) would have re-run P2 whole; the operator's invocation relays overseer1's ruling for this one case). The 7 extracts, the 7 history files, the cascade extracts and every `## Lost` item of the six `cascade-compare-*.md` were read in this window.
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the `timeout` exit-124 rule; no boot smoke here) · `.claude/rules/testing.md` — read in full, 9 Session Additions applied (result lines counted only under `--no-fail-fast`; `cargo test` results attributed on one merged stream; search `tests/`, `examples/*/tests/` and the crate's own `src/` before calling a check unaffected; the off-unix check after a `#[cfg(unix)]` move — none moves here)
- **Platform issues consulted:** none — the one folded CI red (Dependency audit) is no runner-only failure: the advisory reproduces in cargo-deny's own reading and its fix is read locally on the take's tree (inputs#I3 item 2)
- **External inputs:** `inputs#I1` — the run mode (forced sample) · `inputs#I2` — upstream/main `f9cbb2756`, 16 past `17de049b1`, the four predicted merge conflicts, the CI red by RUSTSEC-2026-0386 · `inputs#I3` — P3's reads: tokio 1.54.1 (or `--precise 1.53.3`) clears the advisory with 12 lock edges re-pointed (I3 item 1 says "twelve … from `windows-sys`" plus one `syn` edge; its own list names eleven `windows-sys` dependents — re-derived at P4 on the take: `git diff --numstat a356f9086 -- Cargo.lock` reads `15 14`, i.e. `9eeebb302`'s one added line plus tokio's version, checksum, eleven `windows-sys` edges and one `syn` edge), the audit reads ok on the take, Dioxus 0.8.0-alpha.3 and the whole merge each fail to build in escher code, HEAD's CI#40081725681 settled red by the audit alone

## The trial takes (worktree `target/tmp/trial-wt`, target dir `target/tmp/trial-target`, base `a356f9086`)
Each commit cherry-picked with `-x`, then `cargo check --workspace --all-targets --locked` (base: exit 0, 38.8 s warm).
- **Shape A — the cherry-pick take.** In upstream order: `28f848d3b` (one conflict, below), `9eeebb302`, `e4ca5b9b5`, `46d7cf993`, `fb297a38c`, `50bf2f235`, `f8723f495`, `21edc92d9`, `f102663b8`, `f9cbb2756` — every one picks clean but the first and checks exit 0 after each; then `cargo update -p tokio`. On that tree: `cargo fmt --all --check` 0 · `cargo clippy --workspace --locked -- -D warnings` 0 · `cargo deny --locked --workspace --all-features check advisories licenses` 0, `advisories ok, licenses ok` · `cargo test --workspace --locked --no-fail-fast` 0: **206 result lines · 1099 passed · 0 failed · 12 ignored** against the base's 205 · 1098 · 0 · 12 (test-plan-detail/ci-integration.md:59) — +1 line and +1 passed, exactly upstream's new target `hit_test_dropped_nodes` (1 test); `stale_node_mapping` 4 unmoved; 0 `Running tests/all.rs` lines (`tally.py` over the merged stream). No stand number moved: every stand check, `stand_replay` and seven_guis `recording` (the committed Counter checks) green.
- **Not takeable on Stylo 0.22.0** — `3954ecb85` sets `layout.css.text-indent-keywords.enabled` and `layout.css.word-break.break-word.enabled`, neither in stylo_static_prefs 0.22.0's 57-line `preferences.toml` (its pick conflicts in `document.rs` on 0.23's renamed prefs); `1c5265a8b` implements `TElement::set_may_have_animations` and calls `has_active_animations_or_transitions`, neither in stylo 0.22.0 (`dom.rs:765` holds `may_have_animations` alone; `servo/animation.rs:1660-1668` holds `has_active_animations` and `has_active_transitions`) — its pick conflicts in `stylo.rs`. Both need `06ecec886` (Stylo 0.23-pre, git).
- **Shape B — A plus `286874cc1`** (Dioxus `0.8.0-alpha.3`, a crates.io pre-release, no git source): picks with `Cargo.lock` alone conflicting, every dioxus-native(-dom) file auto-merging; re-locked by cargo; `cargo check` exits 101 — dioxus-native-dom lib 10 errors, lib test 15, every site in escher's own `element_id.rs` (26-27, 78-127) and `dioxus_document_tests.rs` (12, 150-156): dioxus-core 0.8 reshaped `VNode` (`template`, `key`, `dynamic_nodes`, `mounted_root` now methods or gone), `DynamicNode::Placeholder`, `TemplateNode`, `DioxusState` (`node_id_mapping` gone, `stack` private), plus a `rustc_hash` import whose dependency the commit drops (inputs#I3 item 3). A port of the stable-id derivation onto Dioxus 0.8 — unmeasured past the compile errors.
- **Shape C — the whole merge of `f9cbb2756`.** `git merge` conflicts in `Cargo.lock`, `mutator.rs`, `scrolling.rs` and `construct.rs` (the last auto-resolved by rerere from shape A's pick — below); resolved as the take's resolutions plus: `scrolling.rs` upstream's `use stylo_taffy::CopyStyleGetters;` without its `web_time` line; `mutator.rs` ours (the conflict is the last sync's cherry-picked `38ca4e777` meeting upstream's own copy); the lock upstream's, re-locked. `cargo check` exits 101 on its first escher-owned line: blitz-dom 1 error, our `contain: paint` hit stop's `style.get_box().clone_contain()` (`node/node.rs:1531` at HEAD), absent under Stylo 0.23; no crate past blitz-dom reached, so shape B's dioxus-native-dom port is owed on top (inputs#I3 item 4). No `Instant::now()`/`SystemTime::now()` outside `clock.rs` on that tree.
- **Shape D — the audit fix alone.** The tokio lock move of shape A on HEAD; nothing of upstream taken.
- **Side effect, left standing:** git rerere is on (`~/.config/git/config`, `rerere.enabled true`) and recorded seven resolutions of these trials in the shared `.git/rr-cache` (`d3e70713…` `document.rs`, `46a43bbc…` `stylo.rs`, `4f8055e9…` `construct.rs`, `58a28736…` and `4439e693…` `Cargo.lock`, `ce4a55b6…` `mutator.rs`, `d2055d86…` `scrolling.rs`, 12:54–13:03 local); this host's guard refuses a recursive `rm`, so they stand. A pick of `28f848d3b` under /implement will apply `4f8055e9…`'s resolution — the intended one (upstream's import line plus `RangeInputData`) — and git says so ("Resolved … using previous resolution"); the trial worktree stays registered (`git worktree list`) until the operator removes it.

## Files inspected
- `packages/blitz-dom/src/node/node.rs` (1415-1560, 1734-1760) — `hit_inner`'s `inert` stop first (1426-1433), visibility (1435-1443), `pointer-events: none` (1445-1449), the clip stop at a box paint clips, `contain: paint` included (1524-1540); `f102663b8`'s four hunks replace `self.with(id)` by `self.tree().get(id)` in the positive-z (1558), paint-child (1573), negative-z (1605) loops and the text-run lookup only — every CARRY 3 gate above them untouched; `absolute_position` / `position_in_containing_block` (1743-1760) untouched by both node.rs commits.
- `packages/blitz-dom/src/layout/construct.rs` (20-34) — the one conflict of `28f848d3b`: our `node::{…}` list holds `RangeInputData` (2026-10-13-range-input-interaction); upstream's moves `ListItemLayout`, `ListItemLayoutPosition`, `Marker`, `TextBrush` out with the inline build code. Resolution: upstream's line plus `RangeInputData`; the range seeding stays in `construct.rs` (our 25-line diff since `23b9b26a9`, none of it inside the moved functions).
- `packages/blitz-dom/src/config.rs` (10-30, 50-55) — `fb297a38c` edits doc comments alone: `StyleThreading` now "serialised process-wide", the `style_threading` field doc corrected to `Sequential` (the enum's `#[default]` already was — the mismatch arch [Style threading] records); the `clock` field untouched.
- `packages/blitz-dom/src/stylo.rs` (26-30, 147-160) — `fb297a38c` adds `static PARALLEL_TRAVERSAL_LOCK: Mutex<()>` taken around a `Parallel` traversal, poison recovered (`unwrap_or_else(|e| e.into_inner())`), no panic path; `e4ca5b9b5` matches classes from `ElementData::classes`; `46d7cf993` records out-of-flow-ness in `RecalcStyle` and `sync_pseudo_element_styles`.
- `packages/dioxus-native/src/lib.rs` (218-222) — `launch_cfg_with_props` now builds its document with `StyleThreading::Parallel`: the windowed path (`seven_guis_native` through `dioxus_native::launch`) resolves styles on Stylo's pool; no headless document — harness, stand, driver — sets `style_threading` (`git grep style_threading` outside blitz-dom: 0 hits).
- `packages/blitz-dom/src/layout/replaced.rs` (141-212) — `50bf2f235` keeps `is_usable_ratio` (finite and > 0) on the style's ratio and divides by it only through `Some(ratio)`, `.max(0.0)` after: the degenerate-ratio guard (security-plan input-validation) holds.
- `packages/blitz-dom/src/node/element.rs`, `mutator.rs` (348-360, 453-465) — `e4ca5b9b5` adds `ElementData::classes` (a `ThinVec<Atom>`) set by `ElementData::new`, cloned, and kept by `set_attribute` / `remove_attribute` arms placed after the `id` arm, ahead of the `value` arm our range re-seed reads — the range and select arms untouched.
- `examples/multi_doc.rs` at `upstream/main` (116 lines) — two `std::env::var` call sites reading three variables: `MULTI_DOC_N` (55) and `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` through `env_flag(name)` (17-19, 22, 36); `std::env::args` (48-54); one `println!` (111); `panic!` on an unknown mode (108); `Instant::now()` twice (61, 64), an example, outside blitz-dom; no `tracing`/`log` site, subscriber or listener.
- `.andromeda/architecture-detail/occupied-resources.md:55` and `.andromeda/security-plan.md:294-302` — the registries list every source env read, upstream's own included (`WPT_DIR` in the WPT runner, the CI script's `GITHUB_REPOSITORY` · `PR_NUMBER` · `RUN_URL` · `GITHUB_STEP_SUMMARY`, upstream's never-built `tests/all.rs`'s `env!`): the precedent that registers the example's three.
- `Cargo.lock` at HEAD (4839-4845, 5152-5158) — `mimalloc` 0.1.52 and `libmimalloc-sys` 0.1.49 already in the lock (the browser's optional `mimalloc`); `9eeebb302` adds one edge (`"mimalloc"` under the WPT runner's dependencies) — no package enters; the runner declares `mimalloc = "0.1.48"` directly, not through `[workspace.dependencies]`, as upstream wrote it.
- `.github/scripts/test_blitz_tests_targets.py` (full) — pins `autotests` not false and the one `[[test]]` table alone; a new file under `tests/` needs no line there. `tests/blitz-tests/tests/all.rs` takes `f102663b8`'s `mod hit_test_dropped_nodes;`.
- `tests/blitz-tests/tests/hit_test_dropped_nodes.rs` at `upstream/main` — one `#[test]`; 0 env · 0 print · 0 subscriber.
- `tests/blitz-tests/tests/stale_node_mapping.rs` — on our branch since `db12391ab` (upstream #586); `286874cc1` edits it (+19 −35 net, its `assert_no_stale_mappings` over `try_element_to_node_id` removed), it does not add it.
- `packages/dioxus-native-dom/src/lib.rs` at `upstream/main` (34-46) — the crate-local `trace!` stays `#[cfg(feature = "tracing")] tracing::debug!`; under `286874cc1` `mutation_writer.rs` holds 11 `trace!` lines (13 at HEAD; `grep -c 'trace!'`), 10 of them among its added lines, `mutation_buffer.rs` none.
- `stylo_static_prefs-0.22.0/preferences.toml` and `stylo-0.22.0/{dom.rs,servo/animation.rs}` in the cargo registry — the two Stylo-0.23-only commits' measurement above.

## Graph impact (trace `.andromeda/runs/2026-10-24T09-31-30-phase/tree-query-2026-10-24-upstream-sync-polish-and-ship.json`, plane rust, regenerated then fresh; editor lines)
- **hit_inner** — `hit` @ `packages/blitz-dom/src/node/node.rs:1408`, `hit_with_scrollbar` @ `packages/blitz-dom/src/document.rs:2041`, itself @ `node.rs:1564`, `:1597`, `:1611`. `f102663b8` keeps its signature; no caller is edited.
- **compute_replaced_layout** — `layout` @ `packages/blitz-dom/src/layout/mod.rs:37`, `dispatch_child_layout` @ `:497`. `50bf2f235` keeps the signature; the natural height a replaced element with an `aspect-ratio` reads moves for every document.
- **is_out_of_flow** — `dispatch_child_layout` @ `packages/blitz-dom/src/layout/mod.rs:511`, `LayoutPassState::is_out_of_flow` @ `:856` (now reading `Node::out_of_flow`), `compute_orthogonal_child_layout` @ `packages/blitz-dom/src/layout/writing_mode.rs:122`, `physicalise_and_round_inner` @ `:283`, `resolve_transforms` @ `packages/blitz-dom/src/resolve.rs:209`, `:228`. `Node::is_out_of_flow` keeps its signature and style reading; only the `RoundTree` method moves to the recorded flag.
- **ElementData** — referenced outside blitz-dom by blitz-paint `render.rs:26`, `:708`, blitz-vibey-script `dom/mod.rs:390`, `:413`, dioxus-native-dom `snapshot.rs:7`, `:234`, `:256`, `:678` and the WPT runner `test_runners/mod.rs:47`, all by type, none by struct literal — the new `classes` field breaks none (the take checks green). `StyleThreading` / `style_threading`: no reference outside blitz-dom.

## Patterns detected
- **Partial take by cherry-pick with its trailer** (`.andromeda/architecture-detail/project-intent.md:9`): the last two syncs took upstream's text by no-commit cherry-pick, each `(cherry picked from commit …)`, the merge base `23b9b26a9` unmoved, the left-out commits named.
- **A conflict resolved by keeping both sides' intent, named** (`.andromeda/architecture-detail/established-decisions.md`, the 2026-10-10 sync's one recorded upstream-code edit): the `construct.rs` import line is this take's one resolution and is recorded as such.
- **Upstream test file as its own target** (`tests/blitz-tests/Cargo.toml:46-59`; `.github/scripts/test_blitz_tests_targets.py:17-26`): `hit_test_dropped_nodes.rs` lands with no manifest line, `all.rs` takes its `mod` line and stays `test = false`.
- **External decay closed by a lock update** (`.claude/rules/security.md` §Dependencies; the 2026-10-05 rustls 0.23.43 → 0.23.45 precedent): tokio moves in `Cargo.lock` alone, the manifest's `tokio = "1.42"` unchanged.

## Conventions to follow
- **Coupled pins move as one set** (CLAUDE.md Critical Warnings): shape A moves none — Stylo 0.22.0, taffy 0.15, Dioxus 0.7.3 stand; the six left-out commits (two Taffy bumps, Stylo 0.23, its two dependants, Dioxus 0.8) stay out together.
- **Moved counts attributed by name** (`.andromeda/test-plan-detail/ci-integration.md:39-40`): the take's +1 result line and +1 passed on each test job are `hit_test_dropped_nodes`'s, the a11y job (6 · 43) and the stand repeat (168 · 684) forecast unmoved.
- **An upstream example's env read is registered** (`.andromeda/architecture-detail/occupied-resources.md:55`): `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT`, non-secret, an upstream example's knobs, no crossing — registered at the wrap where the take carries `fb297a38c`.

## New files to create
- `packages/blitz-dom/src/layout/inline/build.rs` — upstream's file from `28f848d3b`, as written
- `packages/blitz-dom/src/layout/inline/mod.rs` — upstream's file from `28f848d3b`, as written
- `packages/blitz-dom/src/layout/inline/layout.rs` — `layout/inline.rs` renamed by `28f848d3b`, then `21edc92d9`'s hunk
- `examples/multi_doc.rs` — upstream's file from `fb297a38c`, as written
- `tests/blitz-tests/tests/hit_test_dropped_nodes.rs` — upstream's file from `f102663b8`, as written
- `escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/` — the take's readings (take, lock, counts, sink, merge-surface rows, CI tallies)

## Files to modify
- `packages/blitz-dom/src/layout/inline.rs` — removed by the rename to `inline/layout.rs`
- `packages/blitz-dom/src/layout/construct.rs` — `28f848d3b` (the import line resolved) and `46d7cf993`
- `packages/blitz-dom/src/resolve.rs` — `28f848d3b`
- `packages/blitz-dom/src/layout/mod.rs` — `46d7cf993`
- `packages/blitz-dom/src/layout/paint_tree.rs` — `f102663b8`
- `packages/blitz-dom/src/layout/replaced.rs` — `50bf2f235`
- `packages/blitz-dom/src/node/node.rs` — `46d7cf993`, `f102663b8`
- `packages/blitz-dom/src/node/element.rs` — `e4ca5b9b5`
- `packages/blitz-dom/src/node/mod.rs` — `e4ca5b9b5`
- `packages/blitz-dom/src/mutator.rs` — `e4ca5b9b5`
- `packages/blitz-dom/src/stylo.rs` — `e4ca5b9b5`, `46d7cf993`, `fb297a38c`
- `packages/blitz-dom/src/config.rs` — `fb297a38c`
- `packages/dioxus-native/src/lib.rs` — `fb297a38c`
- `apps/browser/src/document_loader.rs` — `fb297a38c`
- `apps/browser/src/toolbar.rs` — `fb297a38c`
- `apps/browser/Cargo.toml` — `9eeebb302`
- `wpt/runner/Cargo.toml` — `9eeebb302`
- `wpt/runner/src/main.rs` — `9eeebb302`
- `wpt/runner/src/test_runners/attr_test.rs` — `f8723f495`
- `tests/blitz-tests/tests/all.rs` — `f102663b8`'s one `mod` line
- `flake.nix` — `f9cbb2756`
- `Cargo.lock` — `9eeebb302`'s one edge and the tokio update
- `Cargo.toml` — only under shape B (the Dioxus lines) or C (upstream's root manifest, git Taffy and git Stylo)
- derived `packages/**` by `git diff --name-only a356f9086 upstream/main -- packages` — only under shape C, the whole merge: upstream's delta there, the four conflicts resolved
- derived `apps/**` by `git diff --name-only a356f9086 upstream/main -- apps` — only under shapes B and C
- derived `examples/**` by `git diff --name-only a356f9086 upstream/main -- examples` — only under shapes B and C (`cells.rs`, `todomvc`, `transforms.rs`)
- derived `tests/blitz-tests/**` by `git diff --name-only a356f9086 upstream/main -- tests/blitz-tests` — only under shapes B and C (`stale_node_mapping.rs` and Stylo 0.23's test edits)
- derived `wpt/**` by `git diff --name-only a356f9086 upstream/main -- wpt` — only under shape C
- derived `packages/dioxus-native-dom/**` by `git show --name-only 286874cc1` — only under shapes B and C: the Dioxus 0.8 sync, with escher's `element_id.rs` and `dioxus_document_tests.rs` ported

## Sweeps
- Upstream's Rust delta past `17de049b1`, added lines (`git diff 17de049b1 upstream/main -- '*.rs'`, 1652) matched against `println!|eprintln!|print!|dbg!|std::env|env::var|env!\(|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(|tracing::|debug!|trace!|info!|warn!|error!|log::|Instant::now|SystemTime::now|set_hook|catch_unwind|span!|instrument|unimplemented!|todo!|panic!|unwrap\(\)`: 30 hits · 0 changed · 30 no-change (re-counted per file) — `examples/multi_doc.rs` 8 (its env reads, args, `Instant::now`, `panic!`, `println!`), `inline/build.rs` 1 (a commented `dbg!`), `mutation_writer.rs` 11 (10 `trace!` and 1 `panic!("renderer asked for unknown ElementId …")`), `events.rs` 1 `unimplemented!` (`convert_before_input_data`), `dioxus_document.rs` 5 `unwrap`s, `history.rs` 1, `render.rs` 1, `attr_test.rs` 1, `hit_test_dropped_nodes.rs` 1 — the `mutation_writer`/`events`/`dioxus_document` hits are `286874cc1`'s, outside shape A.
- Shape A's own added lines (`git diff a356f9086 bebf048f7 -- '*.rs'`, 674) against that pattern plus `Mutex|static |\.expect\(`: 14 hits · 0 changed · 14 no-change — `multi_doc.rs` 9, the commented `dbg!` 1, `PARALLEL_TRAVERSAL_LOCK` 1 (process-wide state), the WPT runner's `#[global_allocator]` 1 (process-wide state, an upstream-only binary), `hit_test_dropped_nodes.rs` 1 `unwrap().expect`, `attr_test.rs` 1 `unwrap()` (the WPT runner, upstream-only) — no `tracing`/`log` site, print, subscriber, listener or engine env read in an engine crate.
- `Instant::now()|SystemTime::now()` in `packages/blitz-dom/src` outside `clock.rs` (`git grep` at `bebf048f7`, and on shape C's tree): 0 · 0.
- `style_threading|StyleThreading` over `packages tests examples apps` outside `packages/blitz-dom/src/config.rs` (`git grep`): 7 hits, all in blitz-dom (`document.rs:21`, `:258`, `:504`, `iframe.rs:71`, `lib.rs:99`, `stylo.rs:9`, `:149-150`) · 0 changed — no harness, stand or driver document sets it.
- `layout/inline\.rs|layout::inline::` over tracked files outside `.andromeda`, `escher-0.*` and `*.md` (`git grep`): 0 hits — the split moves no code reference; the masters' citations (`.andromeda/architecture.md`, `architecture-detail/{conventions,established-decisions,existing-scopes}.md`, `security-plan.md`, `security-plan-detail/input-validation.md`) are the wrap's re-points.
- `keyboard_compat` over `upstream/main`'s tree (`git ls-tree -r --name-only upstream/main`): 0 — retired at `286874cc1`.

## Scope premise closure
Every `[inferred]` bullet of `scope.md` re-read against the reads above and amended in place (0 left; 7 `[VERIFIED`, 4 `[premise-corrected`, 1 `[closed`):
- The pin fork — VERIFIED and corrected: Dioxus 0.8.0-alpha.3 is a registry pre-release, no git source; the four shapes measured.
- The four predicted conflicts — VERIFIED, with the conflict set per shape.
- Upstream's new test files — corrected: `stale_node_mapping.rs` is a standing target `286874cc1` edits; `hit_test_dropped_nodes.rs` alone is new. The 286874cc1 group line in «What it builds» is corrected with it (a derived section).
- The audit fold — VERIFIED against the recorded run (CI#40081725681 settled red by the audit alone), its two P3 questions answered.
- The sink re-check — corrected (three env variables through two call sites; the Dioxus rewrite's 10 `trace!` lines) and its registration question closed.
- CARRY 2, 3, 4, 5 — VERIFIED, each with the cherry-pick take's reach.
- dioxus-native-dom's caveat — VERIFIED and measured (the clean textual merge fails to build).
No extract leaned on a falsified premise past these corrections: the extracts' `stale_node_mapping` lines ("where new") and their "two env reads" read as corrected here, and P4 takes this file as overriding them.

## Open questions
- The take's shape — A (the cherry-pick take, no pin moved), B (A plus Dioxus 0.8.0-alpha.3, a pre-release, with escher's stable-id derivation ported), C (the whole merge, git Stylo and git Taffy in the root manifest against the founder's rule, plus B's port and our `contain: paint` getter) or D (the audit fix alone) → blocks: plan-decision (P4, the founder's fork: every shape but D changes engine behaviour for every document, C reverses a recorded rule of his)
- The tokio form — `cargo update -p tokio` (1.54.1, the advisory's own remedy) or `--precise 1.53.3` (the advisory's floor); both re-point the same 12 edges → blocks: plan-decision (a decisive lean at P4)
