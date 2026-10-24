# Task — tests · 2026-10-24-upstream-sync-polish-and-ship

## The chunk
Takes upstream/main (DioxusLabs/blitz, measured at f9cbb2756) into the fork, either as a whole merge or as cherry-picks of the engine commits onto the registry pins. Which one is a P4 fork over git Taffy, git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3.
The engine commits touch blitz-dom's inline layout, the class atoms on ElementData, out-of-flow flags on Node, serialised style traversal in config.rs, aspect-ratio natural height, animation-set lookups, hit testing that skips dropped nodes, and Stylo text-indent and word-break keywords. The Dioxus move re-syncs packages/dioxus-native-dom and deletes keyboard_compat.rs.
Upstream's new test files tests/blitz-tests/tests/hit_test_dropped_nodes.rs and stale_node_mapping.rs come in with their commits. Four textual conflicts are predicted: Cargo.lock, layout/construct.rs, mutator.rs and scrolling.rs.
Turns the Dependency audit job green with a semver-compatible tokio update for RUSTSEC-2026-0386. Re-reads the per-OS test tallies on the take's green CI run, and re-checks the sink allowlist against upstream's new print and env-read sites.
Adds no new driver capability, verb, crate, port or env var of ours, and links no matrix row.

## Needs
- The blitz-tests target discipline: one target per file, upstream's tests/all.rs declared and never built, and BlitzTestsTargetsTest pinning the manifest. Upstream's new files become targets by auto-discovery, and all.rs takes upstream's mod lines.
- The randomized stress check over stale_node_mapping.rs (the LCG template swaps behind the slab and ElementId reuse) and upstream's hit-testing check for dropped nodes, both run as fork targets.
- The local gates: ci-leg.sh fast (fmt, clippy, workspace test, ci-scripts) and the doc leg, all with --locked. The workspace test leg is run against the new lock.
- The audit leg (cargo-deny, advisories and licences over the whole workspace graph), which has to read green after the tokio update, with the ignored IDs left as they stand. mimalloc, a new dependency in apps/browser and wpt/runner, goes through the licence check.
- Per-OS test tallies (CI integration): every test-bearing job's log is re-read by the recorded method (linux over macOS by the doc-test lines, macOS over windows by the unix-gated binaries, ios and android build only), and each moved count is explained by upstream's new targets.
- The engine-features-by-runner rule (tracing and writing-mode are on only in the workspace build): the workspace leg's green and a per-package green are different readings, and sink captures count lines by target.
- The coverage leg and its per-crate floors (dioxus-native-dom 83 %, escher-driver 97 %, blitz-test-harness 90 %). The keyboard_compat.rs exclusion in both llvm-cov lines drops only if the take carries 286874cc1, and a re-synced dioxus-native-dom can move its floor reading.
- The incremental/non-incremental differential oracle and every pipeline test that runs both layout modes. Upstream's layout and style changes reach them directly.
- The stand checks in both layout modes, and the agent-run tallies: run stand and run all re-counted against the last recorded totals (run stand 195 · 0 · 7 over 48 files), with upstream's new non-stand files read by run all.
- The stand-repeat leg's zero-failure budget: three rounds, no retry.
- dioxus-native-dom's unit tests, which the Dioxus 0.8 re-sync of dioxus_document.rs, events.rs, lib.rs and mutation_writer.rs reaches. That means the element_id and snapshot tests, the bridge_tests child module (it relies on DioxusEventHandler's private fields), keyed_nodes_do_not_crash, the touch-event tests and the mutation writer tests.
- The stable-id and snapshot checks (stand_element_ids, stand_id_persistence, stand_id_edits, stand_snapshot*, stand_diff), and stand_non_lean_tasks and stand_actionable_keys for upstream's edit to the Cells task.
- Exact-value readings that a Taffy or Stylo move can shift: layout bounds, a snapshot's text bytes, pixel tests and shot rasters (byte-identical on one host). A moved number is re-measured and recorded, never loosened.
- Recorded-check replay: stand_replay and seven_guis recording compare the committed Counter checks under tests/blitz-tests/checks/, whose kept answers hold snapshot text and bounds, byte for byte. A layout move turns them red until they are re-recorded.
- The a11y checks: stand_keyboard (Tab order tables, Enter and Space), stand_contrast's rows, the accessibility_* files and inert_subtree. Upstream touches accessibility.rs and adds CSS keywords.
- The clock surface: clock.rs unit tests, virtual_clock_engine and stand_virtual_clock, and the invariant that no wall-clock read in blitz-dom sits outside clock.rs. These are re-read on the take's tree because config.rs, document.rs, events/pointer.rs and scrolling.rs are reached.
- The select model's tests, reached through the mutator.rs conflict: node/select.rs unit tests, select_interaction and stand_act_select. Also the mutator's own unit tests (the disabled-state toggle).
- The hit and bounds checks that upstream's hit-testing change reaches through hit_inner: hit_clipped_at_scrolling_box, scrolled_box_client_rect, scrolled_box_absolute_position, transformed_client_rect and scroll_into_view_nested. These are the engine-debts rows re-read per sync.
- The telemetry capture checks, which must stay at 0 needles after the delta: host_log, mcp_log, stand_act_spans and the telemetry_* files, with escher-telemetry's format.rs unit tests standing.
- The harness's re-exports of keyboard-types' Key, Modifiers and NamedKey, and the test-that dev-dependency. A Dioxus or upstream move can shift what the key-pressing checks and upstream's verify_that checks compile against.
- Font-dependent tests, under system-fonts in the workspace build and the bundled DejaVu Sans with no skip path on the stand. Stylo's text keywords and inline layout split reach their measurements.
- The MSRV job, which is a build only on 1.91. It decides whether a carried Dioxus alpha or Stylo pre-release builds.
- The workspace leg also compiles and runs wpt/runner's and apps/browser's unit tests, which upstream's WPT-runner change and the mimalloc allocator reach.
- Fork CI green on the pushed commit with every job settled. That includes the windows job, which alone runs escher-driver's off-unix arms.

## Not needed
- The cold-agent run pipe, its contract tests and its live run. These belong to the later Cold-agent bar entry and are not re-fired by a sync.
- WPT conformance as a CI workflow: reftest scoring, the area diff, results posted to a PR and the Thai-font reftests. These are upstream-only, and the fork's conformance numbers are out of scope.
- The mutation tier (cargo-mutants under the nextest mutants profile). It runs on the dev host only and is not owed by a sync.
- Driver verb behaviour, refusal causes, the paged snapshot, lint findings and the MCP protocol bounds as specified behaviour. The chunk changes none of them; their checks bind only as part of the legs that must stay green.
- The agent-run.sh and cold-agent.sh contract tests' content. Neither script changes, and only the run tallies are re-read.
- The shared fixture module tests/blitz-tests/tests/fixtures/mod.rs, its pages and HeldNet, and the common and session_common tables. They are not edited unless a moved reading forces it.
- The recording and kept-check rules on owner-only files and stand-ins. Only the committed checks' replay freshness is reached.
- The browser app's history, suggestion, favicon and persistence test content, and blitz-vibey-script's Preact and real-sleep timer tests. These are compiled and run, but no change targets them.
- The human-read example fixtures under examples/assets, the commented-out tests/stylo_usage.rs and the ignored paint_tree_bench benchmark.
- CI caching, apt-install.sh's bounded package installs and the failure-log upload rules. The sync does not touch them.
