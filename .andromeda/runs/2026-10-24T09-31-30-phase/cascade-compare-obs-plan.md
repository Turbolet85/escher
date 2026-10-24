# Cascade compare — obs-plan · phase · 2026-10-24-upstream-sync-polish-and-ship
E items 101 · held 17 · lost at classify 11 · lost at strong 10 · cascade only 1 · body 50 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.4 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:13
- (.andromeda/obs-plan-detail/ci-integration.md:13) The coverage artifact's `keyboard_compat.rs` filename exclusion must drop exactly when the take carries upstream's `286874cc1` (which deletes the file), and must stand unchanged when it does not (CARRY 2); either way the coverage report stays the one fork-CI artifact outside `target/ci-logs/`, uploaded on success only, kept 7 days, holding no user data.
  > | Coverage report | the `coverage` leg's lcov file `target/coverage/lcov.info` — line counts of the workspace's public source, no user data (§8), `keyboard_compat.rs` excluded by filename since 2026-10-17-upstream-sync-engine-debts (test-plan §10 Coverage floors); uploaded only on success as artifact `coverage-report` from `target/coverage/`, kept 7 days — the one fork-CI artifact outside `target/ci-logs/` | (.github/scripts/ci-leg.sh:40-47; .github/workflows/ci.yml:286-291) |

### ci-integration#1.3 · lost at strong · .andromeda/obs-plan-detail/ci-integration.md:12
- (.andromeda/obs-plan-detail/ci-integration.md:12) The per-leg CI log is the record the chunk's "CI and audit green" and per-OS tally re-read pass through (the audit leg's failed log names RUSTSEC-2026-0386); a take that touches `ci-leg.sh` or `ci.yml` must keep each leg's log path, its truncation at the leg's start, and its failure-only upload as `ci-log-{job id}` from `target/ci-logs/` alone, kept 7 days.
  > | Per-leg CI log | each ci.yml leg's merged stdout+stderr, written by `ci-leg.sh` to `target/ci-logs/{leg}.log` (matrix: `target/ci-logs/matrix-{platform}.log`), truncated at the leg's start; uploaded only on failure as artifact `ci-log-{job id}` (`if-no-files-found: ignore`), kept 7 days, from `target/ci-logs/` alone; unscrubbed build output (no user data — §8) | (.github/scripts/ci-leg.sh:83-84; .github/workflows/ci.yml:48-54; .github/workflows/ci.yml:416-425) |

### ci-integration#1.1 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:10
- (.andromeda/obs-plan-detail/ci-integration.md:10) Upstream's WPT runner commit (`f8723f495`) and any upstream workflow change a take carries must leave the WPT report's publication upstream-only, the `wpt` and `trigger-archive` jobs repository-guarded so neither runs on the fork.
  > | WPT report and scores | archived to GitHub Pages and dispatched to `DioxusLabs/blitz-wpt-results` on main, on upstream `DioxusLabs/blitz` only (`wpt` and `trigger-archive` are repository-guarded; neither runs on the fork) | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:89-113) |

### ci-integration#1.2 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:11
- (.andromeda/obs-plan-detail/ci-integration.md:11) `wptscores.json` must stay computed and published on upstream only after the take.
  > | `wptscores.json` | computed from the WPT run and published to Pages, upstream only | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:64-67) |

### ci-integration#1.9 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:18
- (.andromeda/obs-plan-detail/ci-integration.md:18) The two committed Counter checks are replayed by `stand_replay` and held fresh by seven_guis' freshness check; where the take's Taffy, Stylo or Dioxus move shifts a kept answer (bounds, snapshot text, diff), the checks must be re-written by `record_checks.sh` and re-measured, never loosened, keeping mode `100644` and holding no typed text, host path, name, secret or `screenshot` step.
  > | Committed checks | `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, two kept checks in git (mode `100644`, a public repository) — the repository's own scripted stand-Counter sessions, written by `examples/seven_guis/tests/flows/record_checks.sh` (which prints only `step N`), holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name, secret or `screenshot` step — read by `stand_replay` and seven_guis' freshness check; no log or event line; uploaded by no CI leg | (tests/blitz-tests/checks/counter-cli.json:1-8; tests/blitz-tests/checks/counter-mcp.json:1-11; examples/seven_guis/tests/flows/record_checks.sh:1-65) |

### error-capture-reporting#1.1 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:8
- (.andromeda/obs-plan-detail/error-capture-reporting.md:8) The take must add no error-reporting client or service. Upstream's delta brings a new dependency (`mimalloc`, an allocator) and WPT-runner changes, so the take's manifests and added lines must still match no error-reporting search term.
  > - observed absent — an error-reporting client · searched: `sentry|analytics|gtag` over the 21 s02 files

### error-capture-reporting#1.4 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:11
- (.andromeda/obs-plan-detail/error-capture-reporting.md:11) No panic hook or `catch_unwind` may arrive in the engine crates with the take. escher-telemetry's chaining hook stays the only escher-side hook.
  > - observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files

### log-coverage#7.2 · lost at strong · .andromeda/obs-plan-detail/log-coverage.md:107
- (.andromeda/obs-plan-detail/log-coverage.md:107) The absence of structured logging over the examples slice stays true only if the new example adds no `tracing::`/`log::` site — the scope reads none, and the take re-reads it.
  > - observed absent — structured logging · searched: `tracing::|log::|env_logger` over the 32 slice files (examples slice); `seven_guis_native` has since gained escher-telemetry's subscriber (§3)

### log-coverage#6.11 · lost at strong · .andromeda/obs-plan-detail/log-coverage.md:97
- (.andromeda/obs-plan-detail/log-coverage.md:97) The WPT runner is an upstream-only surface the take reaches (`f8723f495`, and `9eeebb302`'s `mimalloc` allocator in its `main.rs`); its log sites stay unscrubbed by design, and moved cites are re-pointed.
  > - Log calls cover WPT_DIR setup, glob failures, net load errors, pending requests, skips, JS errors and missing harness results (wpt/runner/src/main.rs:470-475; wpt/runner/src/main.rs:295; wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220; wpt/runner/src/test_runners/harness_test.rs:151-153; wpt/runner/src/test_runners/harness_test.rs:179)

### metric-coverage#1.1 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:10
- (.andromeda/obs-plan-detail/metric-coverage.md:10) The frame and phase timing gates must stay behind their `log-times` features only. Their cited manifests are the root `Cargo.toml` and `packages/dioxus-native/Cargo.toml`, which a take of upstream's pin moves (`06ecec886`, `286874cc1`) would edit, so those cites are re-read on the take's tree.
  > | Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:265-268); (packages/dioxus-native/Cargo.toml:60-68) |

### metric-coverage#1.2 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:11
- (.andromeda/obs-plan-detail/metric-coverage.md:11) blitz-dom's resolve phase times must stay a printed, prefixed figure. The take carries engine code into blitz-dom's style and layout passes (`fb297a38c` serialises Parallel style traversals; `28f848d3b` splits inline layout), so the `resolve.rs` cite is re-read wherever the take moves that file.
  > | blitz-dom `resolve` | Phase times style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs | printed, prefixed `Resolve({id}): ` | (packages/blitz-dom/src/resolve.rs:77-169) |

### metric-coverage#1.3 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:12
- (.andromeda/obs-plan-detail/metric-coverage.md:12) `debug_timer` must stay opt-in printed timing. The phase timings above reach it through blitz-dom's `log-phase-times`, and the take must add no other emitter.
  > | debug_timer | Labelled instants; total and per-step durations in ns/us/ms/s | printed | (packages/debug_timer/src/lib.rs:14-24; packages/debug_timer/src/lib.rs:33-66) |

### metric-coverage#1.10 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:19
- (.andromeda/obs-plan-detail/metric-coverage.md:19) The WPT runner's figures must stay printed. Upstream's `f8723f495` (#1247) and `9eeebb302` (#1239, `mimalloc` in the runner) edit `wpt/runner`, so these three rows' `main.rs` cites are re-read on the take's tree when it carries them.
  > | WPT runner, per test | Duration in ms | printed | (wpt/runner/src/main.rs:624; wpt/runner/src/main.rs:694; wpt/runner/src/main.rs:375) |
  > | WPT runner, per run | Pass, fail, timeout, skip, crash, subtests, fractional pass, failure buckets and total duration | printed | (wpt/runner/src/main.rs:498-518; wpt/runner/src/main.rs:783-829) |
  > | WPT runner, report | Report generation and write times in ms | printed | (wpt/runner/src/main.rs:837-851) |

### metric-coverage#1.13 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:22
- (.andromeda/obs-plan-detail/metric-coverage.md:22) WPT scores must stay published on upstream `DioxusLabs/blitz` only. Any merge of upstream's workflow text must keep the `wpt` job's repository guard. The chunk leaves upstream's WPT conformance numbers out of scope.
  > | WPT scores (CI) | Scores computed into `wptscores.json` | published to Pages, on upstream `DioxusLabs/blitz` only (the `wpt` job is repository-guarded) | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:64-67) |

### metric-coverage#2.1 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:28
- (.andromeda/obs-plan-detail/metric-coverage.md:28) The recorded absences must hold on the merged tree. The re-check of upstream's added `.rs` lines (one `println!` and two env reads in `examples/multi_doc.rs`) must find no metrics exporter and no `counter!|histogram!|gauge!` site.
  > - observed absent — metrics exporters · searched: `opentelemetry|metrics|prometheus|sentry` over the 86 slice files (apps slice)
  > - observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files

### span-trace-coverage#1.1 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:8
- (.andromeda/obs-plan-detail/span-trace-coverage.md:8) The take adds upstream's `examples/multi_doc.rs` (#1243) and edits `examples/seven_guis/src/tasks/cells.rs` (#1260), so the examples slice must still hold no span on the merged tree.
  > - observed absent — spans · searched: `span!|instrument` over the 32 slice files (examples slice)

### span-trace-coverage#1.2 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:9
- (.andromeda/obs-plan-detail/span-trace-coverage.md:9) The take carries `mimalloc` into `apps/browser` (#1239), so the apps slice must still hold no span or instrumented function after the take.
  > - observed absent — spans or instrumented functions · searched: `span!|#\[instrument|info_span|debug_span` over the 86 slice files (apps slice)

### span-trace-coverage#1.11 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:18
- (.andromeda/obs-plan-detail/span-trace-coverage.md:18) The take carries WPT runner changes (#1239, #1247), so that slice must still hold no span or trace after the take.
  > - observed absent — spans or traces · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files (wpt runner slice)

### span-trace-coverage#3.1 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:35
- (.andromeda/obs-plan-detail/span-trace-coverage.md:35) The `tool_call` span's construction and closed field set must stay unchanged by the take.
  > - `packages/escher-driver` — a second span, `tool_call`, since 2026-10-10-mcp-surface (answered by the founder, 2026-10-10, «Span с tool и ending», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's): built by hand with `tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)`, opened and entered once per MCP `tools/call` request only — before its params are read — and held until its reply is written; two fields and no other, each a closed word:

### span-trace-coverage#3.1 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:36
- (.andromeda/obs-plan-detail/span-trace-coverage.md:36) `stand_mcp_tools` is the standing check of the `tool_call` → `command` nesting. It must pass in both layout modes on the merged engine.
  > measured in process in both layout modes, one `tool_call` per call the parent of each `command` (`stand_mcp_tools`, its hand-written recorder; a mutation that leaves the span unentered turns it red)

### telemetry-strategy#3.1 · lost at classify · .andromeda/obs-plan-detail/telemetry-strategy.md:20
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) Upstream's delta edits `flake.nix` (`f9cbb2756`) and the root and package manifests (the Stylo, Taffy and Dioxus pins, `mimalloc`), so this absence must still hold over the take's tree, or the new telemetry backend is recorded.
  > - observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

### telemetry-strategy#3.4 · lost at strong · .andromeda/obs-plan-detail/telemetry-strategy.md:23
- (.andromeda/obs-plan-detail/telemetry-strategy.md:23) `f8723f495` (WPT runner) and `9eeebb302` (`mimalloc` in the WPT runner) reach the wpt runner slice; its tracing/metrics absence is re-read on the take.
  > - observed absent — tracing or metrics libraries · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files (wpt runner slice)

## Held

- log-coverage#1.1
- log-coverage#4.1
- log-coverage#5.1
- log-coverage#6.1
- log-coverage#6.10
- log-coverage#6.3
- log-coverage#6.4
- log-coverage#6.9
- pii-scrubbing-compliance#1.1
- pii-scrubbing-compliance#2.1
- pii-scrubbing-compliance#3.1
- span-trace-coverage#1.5
- span-trace-coverage#2.1
- telemetry-strategy#1.1
- telemetry-strategy#2.1
- telemetry-strategy#3.2
- telemetry-strategy#3.3

## Cascade only

- log-coverage#6.2
