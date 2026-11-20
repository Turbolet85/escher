@history_item n=1 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — introduced `test_capability_statement.py` (`CapabilityStatementTest`, run in the `ci-scripts` leg) pinning `docs/capabilities.md` and README's «the {word} driver commands as tools» phrase by path, whitespace folded, each phrase an `assertIn` message; the claim section names each claim's check. Matters: the README is held by this pin, so the 0.3.0 heading edit must leave that phrase and its count word untouched; the edit is a version heading the pin does not read.

@history_item n=2 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — widened decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-check-names-held-to-schema and decision/capability-statement-from-schema: the pin also holds the a11y leg's file count and stand-repeat's named-target count against `.github/scripts/ci-leg.sh`, and the README's MCP-tool count to the schema. Matters: these are the picked rows' current wording; a README or capabilities edit that touches any counted phrase reddens `ci-scripts`, so P3's re-read of `docs/capabilities.md` must change nothing pinned.

@history_item n=3 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — moved contract/driver-verbs/verb-table, contract/mcp-tools-list/order, contract/mcp-server/tools-list and contract/escher-session-command-line/command-form from fourteen to fifteen verbs, `layout` after `actions`, `help` last. Matters: the README's «driver commands as tools» count word the capability pin reads is fifteen; the heading edit must not disturb it, and the rows this chunk is bound by state fifteen.

@history_item n=4 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the command line and MCP answer the graph) `mcp_server`'s `tools/list` gained `graph` after `lint`; the verb-table pins moved to thirteen. Matters: origin of the `graph`-after-`lint` order in contract/mcp-tools-list/order, a row this chunk is bound by; a precedent that a verb-count move ripples to README and capabilities count words.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions verb through the command line and MCP) `tools/list` offers `actions` after `graph`; `VERBS` 14. Matters: origin of the `actions`-after-`graph` clause of contract/mcp-tools-list/order; same count-word ripple precedent.

@history_item n=6 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (CI on the build branch) CI runs on PRs and pushes to main, v0.* and build/**; the fast/slow split (fmt, clippy, test-features-default, ci-scripts with no `needs`); the matrix tests windows and macos and builds ios and android, linux the `test` leg's; WPT only on upstream, the fork running WPT on the host. Matters: the origin of decision/ci-trigger-branches, decision/ci-fast-slow-split, decision/fast-jobs-gate-the-rest, decision/native-platform-ci-matrix, decision/workspace-tests-in-fast-ci-leg and decision/wpt-run-on-host-on-fork; this chunk's pushed README commit on `build/escher-0.4.0` runs the full 17-check CI by those rows, no test moving.

@history_item n=7 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — the action-pin assertions moved from `CiWorkflowTest` into the new `WorkflowPinTest`, the pin rule widened to every workflow. Matters: `CiWorkflowTest` is the guard of decision/fast-jobs-gate-the-rest; the CI-workflow tests are split across those two classes.

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the CI scripts' count; the `embolden` step of `fast`) the `ci-scripts` leg reads 110 tests in eight files, `fast` ends fmt → clippy → test → ci-scripts → embolden. Matters: decision/ci-scripts-unit-tested's leg and the local pre-push gate a README-only chunk still runs; the last recorded ci-scripts tally.

@history_item n=9 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — decision/local-pre-push-gate: a green `fast` leg on a dev host with CI unset ends with the target sweep, which never reddens the leg. Matters: the gate this chunk runs before push behaves this way; a sweep refusal is not a red.

@history_item n=10 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure flakiness budget for stand checks on the founder's word. Matters: origin of decision/per-crate-coverage-floors and decision/stand-checks-zero-failure-budget / decision/zero-failure-stand-repeat; the CI run on this chunk's commit reads both, no test moving.

@history_item n=11 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the coverage leg excludes `keyboard_compat.rs` by filename) both llvm-cov lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`, floors unchanged. Matters: origin of decision/keyboard-compat-coverage-exclusion, a picked row.

@history_item n=12 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (residual owners re-said to their rows' status) the `keyboard_compat.rs` exclusion's owner reads residual `res-6ssktw6x`, absorbed into `v030-03`, re-read owned by CARRY `cry-mjwr45xr`. Matters: the latest owner statement of a picked row's exception; a precedent that master sites naming a residual follow its row's status, relevant as this chunk closes `res-xe7pyput` by word and writes no residual row (the wrap reconciles).

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the stand-repeat leg, seventeen named words and 51 files) `stand-repeat` names seventeen `--test` words beside the `stand_*.rs` glob, pinned by `test_stand_repeat_leg_runs_three_rounds`. Matters: the named-target count the capability statement pins against `ci-leg.sh` (decision/capability-statement-pinned); the README chunk must not restate it.

@history_item n=14 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg names fourteen files, pinned one `assertIn` per word. Matters: the a11y leg file count the capability pin reads from `ci-leg.sh`.

@history_item n=15 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — + decision/requirement-witness-inside-stand-repeat: every 0.3.0 requirement names a witness run inside stand-repeat, pinned by function name. Matters: the test evidence behind stating 0.3.0 «done and verified» in the README; the heading rests on it, no new witness owed.

@history_item n=16 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — ~ overview/session-lifecycle: the session is no verb of the five-command agent-run contract, its `run session` selection running `cli_flow` flows. Matters: the current text of the picked overview row; unchanged by a README edit.

@history_item n=17 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** — §10 rulings: upstream's oversize files, escher's seven oversize integration-test targets and upstream's clone pairs kept, the complexity ceiling ratified for upstream functions. Matters: the base this chunk's CI reads (`a34ed3086`, green 17/17) landed these §10 decisions; the README statement of 0.3.0 closed sits beside them and adds no audit sentence, per the founder's F1.
