# Codebase Research — 2026-10-27-upstream-sync-engine-debts

## Scope
- **Depth:** deep (two trial takes built and gated in a throwaway worktree, a git-Taffy probe, the whole-merge blockers re-read) · **Reads:** 24 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the `timeout` exit-124 rule; no boot smoke here) · `.claude/rules/testing.md` — read in full, 9 Session Additions applied (result lines counted only under `--no-fail-fast`; search `tests/`, `examples/*/tests/` and the crate's own `src/` before calling a check unaffected; the off-unix check after a `#[cfg(unix)]` move — none moves here; the windows cross-check is owed only by such a move)
- **Platform issues consulted:** none — no runner-only bullet: Setup 5a read one green run (CI#40466575975), and no CI-reading entry lies outside the operator leg
- **External inputs:** `inputs#I1` — the operator's take-up directive (one Workflow per batch, transcripts kept, forks to the operator as dialogs answered as the founder's delegate PROVISIONAL, stop on a hard cascade failure) · `inputs#I2` — the founder's word at the last sync, «Десять коммитов (Рекомендую)»: ten cherry-picks plus a tokio update, pins unchanged

## The trial takes (worktree `target/tmp/trial-wt`, target dir `target/tmp/trial-target`, base HEAD `de85792f5`, upstream `ebacd68bd`)
Each commit cherry-picked with `-x` and `-c rerere.enabled=false` (this host has `rerere.enabled true`; nothing was recorded in `.git/rr-cache` this time), then `cargo check --workspace --all-targets --locked`. The gates are scratch script `gates.sh`: check · `cargo fmt --all --check` · `cargo clippy --workspace --locked -- -D warnings` · `cargo deny --locked --workspace --all-features check advisories licenses` · `cargo test --workspace --locked --no-fail-fast`, result lines summed from `^test result:`.
- **Already in, 13 of the 40:** a per-commit read of `23b9b26a9..upstream/main` against `(cherry picked from commit {sha})` trailers on our branch — `fb8d6c15e`, `38ca4e777`, `17de049b1` and the Epoch 5 ten (`git cherry` marks all 40 `+`, since a conflict-resolved pick changes its patch id). 27 remain.
- **Shape A — the cherry-pick take, pins unchanged: eleven commits.** In upstream order: `d63ed2c8c` · `6587e7e79` · `e6f4615ec` · `a7ea9631b` · `b5b163450` · `7fc9ee3af` · `2963b65a2` · `cee6b8731` · `53e9d81c6` · `6fee0db25` · `efdc5472a`. Eight pick clean (`cee6b8731` and `efdc5472a` only on top of `e6f4615ec`, whose new `runtime/` modules they edit); three conflict in one hunk each:
  - `e6f4615ec` (`runtime.rs` split into `runtime/` and `dom/element/` modules) — our side of `runtime.rs` since `23b9b26a9` is one two-line comment (select-by-id, `14818317c`: "Blitz generates a `change` event for a `select` alone …"). Resolution: upstream's `runtime.rs`, the comment carried to its new home `runtime/events.rs:171-173`. Against `e6f4615ec` itself the take's tree differs only by our three earlier rustdoc link fixes in `document.rs`.
  - `7fc9ee3af` (IME enabled when a focused text input's editor is created) — its engine hunk (`construct.rs` `create_text_editor`, 7 lines) picks clean; the conflict is `dioxus_document.rs`'s test module, where upstream's context holds `task_polled_during_render_can_access_document` from `286874cc1` (not taken). Resolution: ours, plus upstream's new test `input_focused_on_mount_enables_ime` alone; it passes (`cargo test -p dioxus-native-dom --lib input_focused_on_mount_enables_ime`: 1 passed).
  - `53e9d81c6` (`<details>`/`<dialog>` `:open`) — `mutator.rs` `set_attribute`: our range re-sanitize arm (`min`/`max`/`step` → `resanitize_range_input`) and upstream's `open` arm (`flush_open_state`) at one spot. Resolution: both, ours first, each an independent `if`; `remove_attribute`'s `open` arm picks clean.
  No manifest, `Cargo.lock` or `deny.toml` line moves (`git diff --stat HEAD 02b24fccf -- Cargo.toml Cargo.lock deny.toml '*/Cargo.toml'`: empty). Gates on the take (`02b24fccf`): check 0 · fmt 0 · clippy 0 · audit 0, `advisories ok, licenses ok` · tests 0, **209 result lines · 1125 passed · 0 failed · 12 ignored**, 0 `Running tests/all.rs` lines, 169 s. Against the recorded 208 · 1117 · 0 · 12 (test-plan, 2026-10-26-codebase-health, CI#40434241601 linux): +1 line, upstream's new target `pseudo_element_flow_classification`, and +8 passed, exactly the take's added `#[test]`s by `git diff HEAD 02b24fccf` — 3 in `pseudo_element_flow_classification.rs`, 3 in blitz-vibey-script `tests/dom.rs` (`cee6b8731` 2, `efdc5472a` 1), 1 in `dioxus_document.rs` (the IME test), 1 in `apps/browser/src/tab.rs`. The audit keeps its two standing warnings (`license-not-encountered`, `yanked` clipboard-win 5.4.1).
- **Shape B — A plus winit beta.4: thirteen commits.** `a09f46792` (winit `=0.31.0-beta.3` → `=0.31.0-beta.4`) picks clean with its own lock hunk and checks green under `--locked`; `527394724` (winit's clipboard API in place of `arboard`) conflicts in `Cargo.lock` alone (escher's own `escher-*` entries beside upstream's removals), resolved as ours and re-pruned by `cargo metadata`. The lock against shape A: `arboard` 3.6.1, `clipboard-win` 5.4.1 and `error-code` 3.4.0 leave; eleven winit-family entries move beta.3 → beta.4; `x11rb` 0.14.0 and `x11rb-protocol` 0.14.0 enter (`MIT OR Apache-2.0`, psychon/x11rb, `build = false`, no proc-macro; direct dependent `winit-x11` 0.31.0-beta.4, beside the standing `x11rb` 0.13.2 of softbuffer). Gates on the take (`fb1f87191`): check 0 · fmt 0 · clippy 0 · audit 0, `advisories ok, licenses ok`, the `yanked` clipboard-win warning gone, RUSTSEC-2026-0192 still encountered (ttf-parser 0.25.1 · owned_ttf_parser · ab_glyph 0.2.32 · sctk-adwaita 0.12.0 all in the beta.4 lock) · tests 0, **210 · 1128 · 0 · 12**, 0 `Running tests/all.rs`, 186 s — +1 line, upstream's `clipboard` target (1 test), and +3 passed, that one and `node/text.rs`'s two new IME tests · CI scripts `python3 -m unittest discover -s .github/scripts`: 108 tests OK · rustdoc `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked`: exit 0. (The doc and CI-scripts legs were read on shape B, a superset of A's files.)
- **The git-Taffy probe** (on shape B): upstream's `taffy` line (git `c2560562…`) and `cargo update -p taffy` alone fail `cargo check` with 10 errors in blitz-dom — `ScrollableOverflow` replaced `scrollable_overflow_rect`'s Rect fields (`node/node.rs:1487`, `:1489`, `:1985`, `:1992`, `debug.rs:97-100`), `place_floated_box` takes a sixth argument (`layout/inline/layout.rs:752`), a Rect mismatch (`layout/inline/layout.rs:1144`): the Taffy pin and `0980cce5f` are one unit. `0980cce5f` on top conflicts in four hunks — `document.rs` (its context is `06ecec886`'s Stylo-0.23 pref names: the commit was authored on git Stylo), `layout/writing_mode.rs:554` (upstream subtracts the node's own scroll offset and passes `is_scroll_container` where escher's scrolled-box bounds fix excludes the own offset — 2026-10-10-scrolling-box-bounds-and-hit) and `node/node.rs` ×2 (an import, and `matches_self` — upstream's scroll-offset test against escher's `own_x`/`own_y` hit at a scrolled box). `ebacd68bd` conflicts in `node/node.rs` alone because it edits the line `0980cce5f` introduces (`layout_style()` direction in `hit_inner`); its own message says it fixes a panic "Since #1294". Unmeasured past the conflicts: resolving them is design work on two of escher's engine changes.
- **Shape C — the whole merge of `ebacd68bd`**, not re-built at P3: the in-memory `git merge-tree` exits 1 with nine files conflicted (scope.md). Its blockers measured at the last sync still stand at HEAD — our `contain: paint` stop's `style.get_box().clone_contain()` (`node/node.rs:1544`), absent under Stylo 0.23 (escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/research.md §The trial takes, shape C); Dioxus `0.8.0-alpha.3` against escher's `element_id.rs`, still on 0.7's vdom (`node_id_mapping` / `dynamic_nodes` / `mounted_root`: 2 hits; 10 + 15 errors then, shape B there); upstream's manifest names git Stylo `d6502e00…`, git Taffy `c2560562…`, Dioxus `0.8.0-alpha.3` and winit `=0.31.0-beta.4`.
- **Shape D — take nothing**, the 27 carried to Epoch 3's «Upstream sync ahead of the UI graph».
- **Left standing:** the worktree `target/tmp/trial-wt` stays registered (`git worktree list`), detached at shape B `fb1f87191`; its trial commits are detached objects (shape A `02b24fccf`, shape B `fb1f87191`, the Taffy probe `f79c01fa0`). The operator removes it; nothing was written to `.git/rr-cache`.

## Files inspected
- `packages/blitz-vibey-script/src/runtime.rs` at HEAD (1100-1110, 1669-1676) and `runtime/events.rs` at `e6f4615ec` (15-26, 171-174) — `report_js_error`'s `#[cfg(feature = "tracing")] tracing::error!("Uncaught JS error in {what}: {error}")` moves whole; our one comment lands at `events.rs:171-173`.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (423-480, 722-760) — the test module's helpers `rerender` (470) and `poll_until_idle` (475) that upstream's IME test calls exist on our side; the bridge's `accessibility_tree` override and `#[path]` children untouched by the take.
- `packages/blitz-dom/src/layout/construct.rs` (`create_text_editor`, 860-885 at `7fc9ee3af`; the flow-classification vote at `2963b65a2`) — the IME enable reads `is_focussed()` before the editor's creation and calls `shell_provider.set_ime_enabled(true)`; `RangeInputData`'s seeding untouched.
- `packages/blitz-dom/src/mutator.rs` (358-372, 513-524 on the take) — the `open` arm sits after the range arm in `set_attribute` and after the link flush in `remove_attribute`; the changed-set inserts, the range re-seed on `value` and the select arms untouched.
- `packages/blitz-dom/src/node/element.rs` (485-550 on the take) — `flush_open_state` sets `ElementState::OPEN` on a `details` or `dialog` carrying `open`, called from `ElementData::new`; `is_disabled` and `flush_is_focussable` untouched.
- `packages/blitz-dom/src/stylo.rs` (507-512) — `NonTSPseudoClass::Open` now reads `ElementState::OPEN`; the `:focus-within` matching (`38ca4e777`) untouched.
- `packages/blitz-dom/src/events/driver.rs` (224-300 on shape B) — `UiEvent::Paste(text)` targets the focused node and is applied as an IME `Commit` through `ime::handle_ime_event`, then the queue is processed.
- `packages/blitz-traits/src/events.rs` (71-75) and `shell.rs` (25-33) on shape B — `UiEvent::Paste(String)`; `ShellProvider::get_clipboard_text` becomes `request_clipboard_text() -> Result<(), ClipboardError>`, its text delivered later as a `UiEvent`. No escher crate implements or calls either (`git grep` outside blitz-shell, blitz-traits and blitz-dom: 0 source hits).
- `packages/blitz-shell/src/application.rs` at `upstream/main` (55-62) — `event_loop.set_clipboard(data)`'s error logged by `#[cfg(feature = "tracing")] tracing::error!("Failed to set clipboard: {_error}")`, target `blitz_shell::application`.
- `packages/blitz-shell/src/convert_events.rs` and `packages/blitz-dom/src/node/text.rs` at `a09f46792` — the `_ =>` arm logging "unsupported Ime variant" leaves (winit beta.4's `Ime` is matched exhaustively), and an IME `Commit` now calls `driver.clear_compose()` before inserting.
- `packages/blitz-vibey-script/src/fetch.rs` and `document.rs` at `cee6b8731` — `FetchError::Forbidden(Url)` and `document_may_load(base_url, url)`; `ScriptDocument` refuses a disallowed `<script src>` with a recorded error and omits it from the prefetch list; upstream's message: "Remote pages could read any local file via fetch('file:///...'), import 'file:///...' or <script src="file:///...">".
- `deny.toml` (19-25) — RUSTSEC-2026-0192's reason names winit 0.31.0-beta.3 explicitly ("it reaches the graph only through winit 0.31.0-beta.3 -> winit-wayland -> sctk-adwaita -> ab_glyph -> owned_ttf_parser"); under shape B the chain stands at beta.4, so the ID stays and its reason's version text moves.
- `packages/blitz-shell/Cargo.toml` (14, 20), `packages/dioxus-native/Cargo.toml` (13, 30), `apps/browser/Cargo.toml` (51) at HEAD — the `clipboard` feature and `dep:arboard`, which `527394724` removes; seven_guis and the escher crates name neither (`git grep`).
- `tests/blitz-tests/tests/{display_contents,inline_box_scrollable_overflow,pre_overflow_scroll}.rs` — standing targets, edited by `0980cce5f` only.
- `packages/blitz-dom/src/node/node.rs` (1214, 1544 at HEAD) — the two `clone_contain` reads (the paint-clip read and the hit stop).
- `.claude/rules/security.md` (9, 32) — the env-read list holds `TMPDIR` through `std::env::temp_dir()` for escher-driver's client unit test only; the 2026-10-15 rule registers a test-only env read before it lands.

## Graph impact (trace `.andromeda/runs/2026-10-27T07-42-37-phase/tree-query-2026-10-27-upstream-sync-engine-debts.json`, plane rust; editor lines)
- **`visible_region`** — 7 callers: escher-driver `in_view` (`execute.rs:454`), `crop` (`execute.rs:465`), the lint's `clip_region` (`lint.rs:394`), and four checks — a take reaching `scrolling.rs` reaches the driver's `off-screen` refusal, the shot crop and the lint. Shapes A and B do not reach `scrolling.rs`.
- **`physical_unrounded_geometry`** — 9 call sites in `document.rs` (2518, 2567, 2570, 2679, 2724), `resolved_style.rs` (509, 521), `scrolling.rs` (769) and `text_extent.rs` (32) — the bounds every snapshot, lint and `getBoundingClientRect` reads; `0980cce5f` alone of the delta reaches its `writing_mode.rs` body.
- **`hit_inner`** — `hit_with_scrollbar` (`document.rs:2041`), `hit` (`node/node.rs:1421`) and its own recursion (1578, 1612, 1627); `0980cce5f` and `ebacd68bd` reach it, shapes A and B do not.
- **`absolute_position`** — 12 call sites (`events/mod.rs:110`, `node/node.rs:778`, `:1770`, `:2015`, `scrolling.rs:715`, the harness's `inspect.rs:60`, `:142`, five in checks); not reached by A or B.
- **`scroll_into_view`** — 9 call sites (`scrolling.rs:832`, the harness's `input.rs:238`, blitz-vibey-script `dom/element.rs:1384`, dioxus-native-dom `events.rs:262`, five in checks); `e6f4615ec` moves blitz-vibey-script's caller into `dom/element/scrolling.rs`.

## Patterns detected
- **The partial take by no-commit cherry-picks** (`.andromeda/architecture.md` §Project Intent → Upstream sync; the last three syncs): `-x` trailers name each upstream sha, the merge base stays `23b9b26a9`, and the left-out delta is owned by a residuals line (`.andromeda/residuals.md:35`).
- **A conflict resolved to upstream's text plus escher's addition** (escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/research.md: `construct.rs`'s import list) — the three shape-A resolutions above follow it.
- **Upstream's script additions as realizations on the registered script-to-DOM crossing** (2026-10-06 and 2026-10-10 syncs, the security history) — `efdc5472a`'s `CSSStyleDeclaration` accessors.

## Conventions to follow
- **Commits carry the upstream sha** — `git cherry-pick -x`; a resolution is the pick's own content, and our side's intent is named in the chunk evidence (`.andromeda/architecture.md` §Project Intent → Upstream sync).
- **Upstream test files enter as their own targets** (`tests/blitz-tests/Cargo.toml`'s `test = false` on `all`, no `autotests`; `.github/scripts/test_blitz_tests_targets.py`) — `pseudo_element_flow_classification.rs` and `clipboard.rs` need no manifest line; `all.rs` takes their `mod` lines from the picks.
- **rerere off for the takes** — `git -c rerere.enabled=false cherry-pick -x {sha}`, so no resolution lands in the shared `.git/rr-cache` (the last sync's trial left seven there).

## New files to create
- `packages/blitz-vibey-script/src/dom/element/attributes.rs` — upstream's split (`e6f4615ec`), both shapes
- `packages/blitz-vibey-script/src/dom/element/form_controls.rs` — the same split
- `packages/blitz-vibey-script/src/dom/element/geometry.rs` — the same split
- `packages/blitz-vibey-script/src/dom/element/markup.rs` — the same split
- `packages/blitz-vibey-script/src/dom/element/scrolling.rs` — the same split
- `packages/blitz-vibey-script/src/dom/element/selectors.rs` — the same split
- `packages/blitz-vibey-script/src/runtime/bootstrap.rs` — the same split (`efdc5472a` edits it)
- `packages/blitz-vibey-script/src/runtime/bootstrap/css.js` — the split's bootstrap scripts, nine
- `packages/blitz-vibey-script/src/runtime/bootstrap/dom_exception.js` — the same
- `packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js` — the same
- `packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js` — the same
- `packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js` — the same
- `packages/blitz-vibey-script/src/runtime/bootstrap/node_constants.js` — the same
- `packages/blitz-vibey-script/src/runtime/bootstrap/style.js` — the same (`efdc5472a` edits it)
- `packages/blitz-vibey-script/src/runtime/bootstrap/stylesheets.js` — the same
- `packages/blitz-vibey-script/src/runtime/bootstrap/window.js` — the same
- `packages/blitz-vibey-script/src/runtime/css.rs` — the same split (`efdc5472a` edits it)
- `packages/blitz-vibey-script/src/runtime/events.rs` — the same split; our select-by-id comment lands here
- `packages/blitz-vibey-script/src/runtime/fetch.rs` — the same split (`cee6b8731` edits it)
- `packages/blitz-vibey-script/src/runtime/modules.rs` — the same split (`cee6b8731` edits it)
- `packages/blitz-vibey-script/src/runtime/timers.rs` — the same split
- `packages/blitz-vibey-script/src/runtime/window.rs` — the same split
- `tests/blitz-tests/tests/pseudo_element_flow_classification.rs` — upstream's target (`2963b65a2`)

## Files to modify
- `.github/workflows/publish-browser.yml` — `a7ea9631b`, `b5b163450` (upstream-only, repository-guarded)
- `apps/browser/src/main.rs` — `d63ed2c8c`
- `apps/browser/src/tab.rs` — `d63ed2c8c`
- `wpt/WPT_COMMIT` — `6fee0db25`
- `packages/blitz-dom/src/layout/construct.rs` — `7fc9ee3af`, `2963b65a2`
- `packages/blitz-dom/src/mutator.rs` — `53e9d81c6`, its one-hunk resolution
- `packages/blitz-dom/src/node/element.rs` — `53e9d81c6`
- `packages/blitz-dom/src/stylo.rs` — `53e9d81c6`
- `packages/blitz-shell/src/lib.rs` — `6587e7e79`
- `packages/blitz-shell/src/window.rs` — `6587e7e79`
- `packages/blitz-vibey-script/src/document.rs` — `cee6b8731`
- `packages/blitz-vibey-script/src/dom/element.rs` — `e6f4615ec`
- `packages/blitz-vibey-script/src/fetch.rs` — `cee6b8731`
- `packages/blitz-vibey-script/src/lib.rs` — `cee6b8731`
- `packages/blitz-vibey-script/src/runtime.rs` — `e6f4615ec`, its one-hunk resolution
- `packages/blitz-vibey-script/tests/dom.rs` — `cee6b8731`, `efdc5472a`
- `packages/dioxus-native-dom/src/dioxus_document.rs` — `7fc9ee3af`, its test-module resolution
- `tests/blitz-tests/tests/all.rs` — `2963b65a2`'s `mod pseudo_element_flow_classification;`

## Sweeps
- Tracing / log / print / env / panic sites over shape B's added Rust lines (`git diff HEAD fb1f87191 -- '*.rs'`, 3,671 `+` lines, pattern `tracing::|log::|println!|eprintln!|dbg!|env::var|env!\(|std::env|trace!|debug!|info!|warn!|error!|set_global_default|tracing_subscriber|Instant::now|SystemTime::now|unwrap\(\)|panic!|todo!|unimplemented!`): 21 hits · 0 changed · 21 no-change — 2 `tracing::error!` sites, both `#[cfg(feature = "tracing")]`-gated under `blitz*` engine targets (blitz-shell's clipboard error, new; the JS error site, moved from `runtime.rs:1105`); 1 test-only `std::env::temp_dir()` (blitz-vibey-script `tests/dom.rs:863`, `cee6b8731`); 18 `unwrap()` lines in test code (`apps/browser/src/tab.rs`, `tests/dom.rs`, `pseudo_element_flow_classification.rs`, `dioxus_document.rs`'s IME test); 0 `println!`, 0 `std::env::var`, 0 subscriber install, 0 wall-clock read.
- Wall-clock reads in blitz-dom outside `clock.rs` on shape B (`git grep -n -E 'Instant::now\(\)|SystemTime::now\(\)' fb1f87191 -- packages/blitz-dom/src`, `clock.rs` set aside): 0 hits.
- `clipboard` feature and `arboard` names over manifests, `examples/`, escher crates and `.github/` (`git grep -n -E '"clipboard"|clipboard\b'`): 6 hits · 5 changed under shape B (`blitz-shell/Cargo.toml:14`, `:20`, `dioxus-native/Cargo.toml:13`, `:30`, `apps/browser/Cargo.toml:51`) · 1 no-change (an `examples/assets/` HTML string).
- `get_clipboard_text` / `request_clipboard_text` outside blitz-shell, blitz-traits and blitz-dom (`git grep`): 0 source hits — the trait method's rename reaches no escher code (hits only in old run-dir graph traces).
- `temp_dir` in blitz-vibey-script (`git grep -n temp_dir {rev} -- packages/blitz-vibey-script/`): HEAD 0 hits, shape B 3 hits, all in `tests/dom.rs` — the new test-only env read.

## Scope premise closure
Every `[inferred]` bullet of `scope.md` was closed in place: five VERIFIED, three corrected (`[premise-corrected: …]`) — the likeliest-to-need-more list (eleven build), the JS error site (a moved line) and the next entry's ground (A and B leave the scroll files untouched). No extract leaned on a falsified premise beyond those readings: the a11y extract's "84 passed" acceptance count is stale by its own history file (85 since 2026-10-25-cold-agent-bar), carried at P4 as 85.

## The fork's answer (P4)
- The take: shape A, the eleven, on the founder's own word «Одиннадцать коммитов (Рекомендую)» (2026-10-27, relayed verbatim by the overseer, inputs#I4; the operator's dialog answer and note, inputs#I3). The lists above are shape A's alone. Shape B's own files — `Cargo.toml`, `Cargo.lock`, `deny.toml`, `apps/browser/Cargo.toml`, blitz-dom `events/driver.rs` and `node/text.rs`, blitz-shell's manifest, `application.rs`, `convert_events.rs` and `event.rs`, blitz-traits `events.rs` and `shell.rs`, dioxus-native's manifest and upstream's `tests/clipboard.rs` — are the rejected branch and leave the lists.

## Open questions
- none — the one plan-decision question, the take, was answered at P4 (above)
