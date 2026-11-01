# Task — arch · 2026-11-01-upstream-sync-ui-graph

## The chunk
Takes DioxusLabs/blitz `upstream/main`, measured at `e42ee1f25` over merge base `23b9b26a9`, into this fork as its ninth upstream take: 33 commits to take or leave, by the shape the founder chooses at P4 from those P3 measures (cherry-pick onto the registry pins, that plus winit beta.4, the whole merge, or nothing).
Resolves the take's conflicts keeping both sides' intent, and runs upstream's new and edited test files under this fork's blitz-tests rule where the take carries them.
Folds eight carried blocks: re-reads the left-out delta and the engine-debts and select-list merge surfaces row by row, fixes or removes the `matrix_test` free-disk step, joins a font-embolden run to a local gate, settles the orphan comment in blitz-dom's `lib.rs`, runs the key-press panic at the base with its fix the founder's fork, and keeps the watch on the `Stand checks repeated` free-disk step.
Re-asserts every escher proof after the take: the `fast`, `doc`, `a11y` and `audit` legs, the stand checks in both layout modes, the incremental/non-incremental oracle and fork CI green with every job settled.
Re-reads the per-OS test tallies from the take's green run, and re-checks the sink allowlist against the taken delta; it builds nothing of the UI graph.

## Reached files
- Cargo.lock · conflict surface
- examples/seven_guis/src/tasks/cells.rs · conflict surface
- packages/blitz-dom/assets/default.css · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-dom/src/stylo.rs · conflict surface
- packages/blitz-paint/src/text.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · conflict surface
- Cargo.toml
- AGENTS.md
- .github/workflows/ci.yml · edits the free-disk step of the `matrix_test` job, its `if:` (lines 392-394)
- packages/blitz-dom/src/lib.rs · edits the orphan «Re-export taffy» comment (lines 96-97)
- tests/blitz-tests/tests/select_popup.rs · edits the one row that skips the key press
- .andromeda/residuals.md · edits the left-out-delta line and the merge-surfaces line (lines 35 and 36)
- .andromeda/architecture.md · edits the `Upstream sync:` line
- .github/scripts/ci-leg.sh · re-reads the `fast`, `doc`, `a11y` and `audit` legs
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target list pin
- .andromeda/test-plan.md · re-reads §Per-OS test tallies
- packages/escher-telemetry/src/format.rs · re-reads the sink's target allowlist
- packages/blitz-test-harness/src/contrast.rs · re-reads the `None` conditions of `Harness::text_contrast`
- tests/blitz-tests/tests/select_popup_painted.rs · re-reads the font-embolden witness
- tests/blitz-tests/tests/stand_element_ids.rs · re-reads the `stand_element_ids` check
- tests/blitz-tests/tests/stand_id_persistence.rs · re-reads the `stand_id_persistence` check
- tests/blitz-tests/tests/stand_id_edits.rs · re-reads the `stand_id_edits` check
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads the `stand_keyboard` check
- tests/blitz-tests/tests/stand_contrast.rs · re-reads the `stand_contrast` check and its contrast rows
- tests/blitz-tests/tests/inert_subtree.rs · re-reads the `inert_subtree` check

## Needs
- The coupled dependency pins and their values: stylo 0.22.0, taffy 0.15, Dioxus 0.7.3, winit `=0.31.0-beta.3`, parser crates matched to Stylo's web_atoms; a take carries upstream's pins as one set at the chosen sha or none, an unresolved set is a halt, and no git dependency stands in the root manifest unless the founder reverses that rule.
- The recorded sync state: merge base `23b9b26a9`, upstream last read at `ebacd68bd`, and the `Upstream sync:` line moving to the new measured sha and commit at the wrap where the take moves the pin; the sixteen commits left out at the last take and git Stylo, the Taffy-main commits and Dioxus 0.8 waiting for the next version.
- The changed set's contract, which a resolution in `mutator.rs` keeps unchanged: its five kinds of write site, its readers `has_changes` and `take_changed_nodes`, ids read through `get_node`, the shell refreshing the platform tree only on a non-empty set; upstream's narrower restyle invalidation (`fe33b778b`) lands beside it.
- The document clock invariant through taken code: every engine time read goes through `DocumentConfig.clock`, no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs`, the harness's one `VirtualClock`; the clock surface in `scrolling.rs` survives the resolution.
- The select model and its open list, unchanged by any resolution: the one path `apply_select_choice`, `option_disabled`, `reconcile_select_list` in the `resolve` order, `hit_with_scrollbar` answering the list first, `paint_select_list` painted last, and the select and range arms of `mutator.rs`.
- The hit stops and scrolled-box bounds, unchanged by any resolution: `hit_inner`, the padding-box stop, the non-invertible transform's no hit, `Node::absolute_position`, `get_client_bounding_rect`, `visible_region`, `scroll_into_view`, and the bounds in `writing_mode.rs`, against upstream's `0980cce5f` and `ebacd68bd` where taken.
- Our seams in the other conflicted engine files: the `size` hint in `stylo.rs`, a `multiple` select's height reading it, and `stroke_text_as` in `blitz-paint`'s `text.rs`.
- The merge surface of the eight engine-debts changes, re-read row by row against the take: `visible_region` through transforms, a non-invertible transform's no hit, the focus fixup under `inert`, the tree's focus falling back to the window, native `toggled`, the `multiple` select, `optgroup`, `change` on a checkbox and a radio; `0980cce5f` has no `visible_region` hunk and its `hit_inner` hunk conflicts textually; nothing is sent upstream.
- The select list's merge surface, sixteen files, re-read row by row against the take, its record `escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md` in the store's moved form, each row still flagged upstreamable with nothing sent.
- The left-out delta and the merge-surfaces lines of the residuals, both still reading the older take `f9cbb2756` and `absorbed:v030-03`: each left-out commit keeps its measured reason or is taken, and the wrap moves or closes the two lines by the fork's answer.
- The key-press panic at `position_in_containing_block` (`node.rs:1775`), reached when the focused control's anonymous block is dropped because an ancestor gained `hidden`: a controlled run at the base and a read of upstream's `node.rs` are owed; the fix, an engine change for every document, is the founder's P4 fork, and it stays bound by persistent interaction state naming DOM nodes only and by the focus fixup, which reads neither `disabled` nor `hidden`; `select_popup`'s skipped key press ends with the fix.
- A font-embolden run joining a local gate, its witness `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, since CI's macOS leg alone exercises `blitz-paint/font-embolden` today.
- The `matrix_test` free-disk step, skipped on every row because its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` while android runs `ubuntu-latest`, fixed or removed with this entry's CI read as the witness; the watch on the `Stand checks repeated` free-disk step retires on a recurrence or 3 green runs; the CI/CD pattern's job and cache facts and `test_ci_workflows.py` read over the edited file.
- The orphan comment in blitz-dom's `lib.rs`, which goes unless upstream's taffy re-export comes back; the crate root's `pub use clock` under it, re-exporting `Clock`, `WallClock` and `VirtualClock`, stays.
- The blitz-tests rule over upstream's test files under a take: one file and one target per behaviour, upstream's `tests/all.rs` declared and held at `test = false`, each joining target recorded in the list `test_blitz_tests_targets.py` pins.
- The proof gates after the take: `bash .github/scripts/ci-leg.sh fast`, `doc`, `a11y` and `audit` as the one leg runner, the `audit` leg's ignored IDs standing as `.claude/rules/security.md` §Dependencies records them, and fork CI green on the pushed commit with every job settled.
- Numbers a moved pin or taken engine change can shift: a stand check's exact bounds, a snapshot's byte size, a screenshot's pixels, the kept checks `stand_replay` replays under `tests/blitz-tests/checks/`; each moved number is re-measured and recorded, never loosened; both layout modes stay identical through upstream's layout and damage changes (`8b1a0a3ce`, `ae774d7f4`).
- The per-OS tallies: the take's own green run, its test-bearing job logs re-read by the method the test plan records, each moved count explained.
- The telemetry sink's allowlist: engine and `escher_*` targets admitted and the rest dropped whole, `SAFE_FIELDS` and `CONTENT_FIELDS`; the taken delta read for every new `tracing` or `log` call site, target, print and env read, the standing capture checks at 0 needles, and the recorded environment-variable list unchanged by the take.
- The Dioxus bridge, escher's most-modified crate: stable ids in their four tiers, the snapshot, its text, parts and diff read-only with no `NodeId`, `unkeyed_actionable`, the `accessibility_tree` override, `data-dioxus-id` dispatch; a resolution in `dioxus_document.rs` keeps them, `6ef4b532d` edits its mounted-events path, and `286874cc1` replaces upstream's side of the bridge.
- The script engine as a security surface: `91ef65e21` adds `URLSearchParams`, an API a remote page reaches, read by P3 for what it carries out; the `document_may_load` gate on `fetch()`, module `import` and `<script src>`, the bootstrap's globals and `report_js_error` keep their behaviour through conflicts in `bootstrap.rs` and `events.rs`.
- Boundary reads the delta brings to every document: `6fca6308d`, `ac6f9f582` and `e5929f9e2` add paint properties, and `f6edb9701` moves what a failed stylesheet does against the render-blocking `<head>` sheets, the critical-resource gate in `resolve` and `settle`'s report of a load in flight; a contrast row or the `None` conditions of `text_contrast` are re-read where carried, and a password input stays masked through paint changes.
- The a11y target (WCAG SC 2.1.1, 1.4.3, 2.4.3) re-proven by standing checks: focus order, Enter and Space activation, contrast and accessibility-tree identity, through `stand_keyboard`, `stand_contrast`, `accessibility_*`, `inert_subtree` and the a11y leg.
- The dependency and workspace surface: MSRV stays 1.91 in both manifests, the MSRV job only building; a crate that enters is reviewed by hand and passes the audit's licence check; no new crate, port, env var or listener of ours, P3 re-reading the manifests and the lock for a `members` change or new package.
- The additivity record and the take's own rules: no edit to upstream-owned code beyond what a conflict, a gate failure or the panic fork requires, each named, minimal and recorded as non-additive; an upstream change to `README.md` resolves to escher's version and `docs/capabilities.md` stands beside upstream's docs; `target/tmp/trial-wt`'s removal stays the founder's and P3 uses and removes its own worktree under `target/`.

## Standing conventions
- S1 · Each new integration test file covers one behaviour, opens with a `//!` doc naming it, and shares helpers only through modules under `tests/` that are no test target · .andromeda/architecture-detail/conventions.md
- S2 · A new test's scenarios run in both layout modes and first assert that the fixture produces the condition · .andromeda/architecture-detail/conventions.md
- S3 · Shared fixture pages, boot and hold live once in `tests/blitz-tests/tests/fixtures/mod.rs`, and a page with one reader stays in its check's file · .andromeda/architecture-detail/conventions.md
- S4 · New code passes `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and rustdoc `-D warnings`, `clippy::expect_used` and `clippy::unwrap_used` allowed only next to a justification · .andromeda/architecture-detail/conventions.md
- S5 · New engine code opens with `//!` docs, gives public items `///` and cites the spec URL next to a web-semantics implementation · §Conventions and §Design Philosophy of .andromeda/architecture.md
- S6 · A new log line in the engine crates is gated per call site with a `cfg(not(feature = "tracing"))` fallback · .andromeda/architecture-detail/conventions.md
- S7 · A new or changed CI leg runs as `bash .github/scripts/ci-leg.sh {leg}` with `--locked`, the same command CI and the dev host run, its `uses:` pinned to a 40-hex SHA, no job-level permission, and its log uploaded on failure · §Infrastructure Patterns of .andromeda/architecture.md and the CI contracts group of .andromeda/architecture-detail/standard-contracts.md
- S8 · A dependency line a resolution writes is declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`, in-repo crates with `default-features = false` · .andromeda/architecture-detail/conventions.md

## Not needed
- The browser app and rdme: the history store, its `history_entries` table, about pages, URL suggester, favicon cache and polling intervals; the scope names no change in `apps/`.
- The WPT runner's execution, its outputs, buckets, budgets and diff-to-PR script: upstream's WPT conformance numbers are out of scope and those workflows run upstream only.
- Browser bundling and deployment: `publish-browser.yml`, the Signed Builds environment, `dx bundle`, Android activities and the Windows bundle settings, reached only from `DioxusLabs/blitz`.
- The Nix flake's packaging and `Cross.toml`'s pre-build: no part of the scope reads them.
- The driver session's own surface: its twelve verbs, eight causes, `v3` wire, MCP tool schema, recording format and state directory; the chunk changes none, its tests running only inside the legs.
- The command lines of the examples and apps, and the fixture forms of the google and graphite pages: the take's scope names no change to them.
- The cold-agent pipe and agent-run contracts, and the mutation tier with its nextest profile: no leg the scope re-runs calls them.
- The accessibility adapter's platform implementations in `accesskit_xplat`: the a11y target is proven through the document's tree and its standing checks, not through an adapter.
- The UI graph's own components, handlers and state, the capability pool's caps `v030-05` to `v030-15`, and the later entry's Dioxus MountedData hazard: later entries own them.
- The project intent's lists of apps, demonstrators and embedding targets named in code: descriptive, with nothing here for a take to act on.
