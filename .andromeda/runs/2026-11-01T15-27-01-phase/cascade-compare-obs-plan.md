# Cascade compare — obs-plan · phase · 2026-11-01-upstream-sync-ui-graph
E items 122 · held 43 · lost at classify 3 · lost at strong 2 · beside a placed fact - · cascade only 20 · listed 4 · body 44 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.1 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:10
- (.andromeda/obs-plan-detail/ci-integration.md:10) A take that carries upstream's workflow files must leave `wpt` and `trigger-archive` repository-guarded, so that neither runs on the fork and nothing is dispatched from it; upstream's WPT numbers stay out of this chunk.
  > | WPT report and scores | archived to GitHub Pages and dispatched to `DioxusLabs/blitz-wpt-results` on main, on upstream `DioxusLabs/blitz` only (`wpt` and `trigger-archive` are repository-guarded; neither runs on the fork) | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:89-113) |

### ci-integration#1.2 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:11
- (.andromeda/obs-plan-detail/ci-integration.md:11) Under the same take, `wptscores.json` must stay an upstream-only publication that the fork's CI does not compute or publish.
  > | `wptscores.json` | computed from the WPT run and published to Pages, upstream only | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:64-67) |

### log-coverage#9.8 · lost at strong · .andromeda/obs-plan-detail/log-coverage.md:135
- (.andromeda/obs-plan-detail/log-coverage.md:135) The driver's site count is a value the take must match.
  > escher-driver holding 2 `info_span!` sites, no event and no `#[instrument]`

### metric-coverage#2.1 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:29
- (.andromeda/obs-plan-detail/metric-coverage.md:29) The taken delta must add no metrics exporter, metric emission or counter, gauge or histogram macro; each search is re-run over what the take brings, and in the layout slice (`layout/construct.rs`, `layout/inline/build.rs`, `layout/writing_mode.rs` among the predicted conflicts) a text-layout `metrics()` call does not count as a metric.
  > - observed absent — metrics exporters · searched: `opentelemetry|metrics|prometheus|sentry` over the 86 slice files (apps slice)
  > - observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files
  > - observed absent — metric emission · searched: `counter!|histogram!|gauge!` over the 17 listed s06 files
  > - observed absent — metrics · searched: `metric|counter!|histogram` over the 8 slice files (matches are only text-layout `metrics()` calls and comments)
  > - observed absent — metrics · searched: `metric|counter|histogram|gauge` over the 16 listed files (no match)

### metric-coverage#1.14 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:23
- (.andromeda/obs-plan-detail/metric-coverage.md:23) WPT scores must stay outside this chunk's proof and publish only on upstream: a shape that brings `.github/workflows/wpt.yml` edits must keep the job's repository guard, and no WPT number becomes a measure of this take.
  > | WPT scores (CI) | Scores computed into `wptscores.json` | published to Pages, on upstream `DioxusLabs/blitz` only (the `wpt` job is repository-guarded) | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:64-67) |

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#1.4 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:13
- (.andromeda/obs-plan-detail/ci-integration.md:13) The take's green CI run must keep the coverage artifact as stated, and the `keyboard_compat.rs` filename exclusion must survive whatever the take brings to the workspace's source and legs.
  > | Coverage report | the `coverage` leg's lcov file `target/coverage/lcov.info` — line counts of the workspace's public source, no user data (§8), `keyboard_compat.rs` excluded by filename since 2026-10-17-upstream-sync-engine-debts (test-plan §10 Coverage floors); uploaded only on success as artifact `coverage-report` from `target/coverage/`, kept 7 days — the one fork-CI artifact outside `target/ci-logs/` | (.github/scripts/ci-leg.sh:40-47; .github/workflows/ci.yml:286-291) |

### log-coverage#9.7 · listed (lost at strong) · .andromeda/obs-plan-detail/log-coverage.md:134
- (.andromeda/obs-plan-detail/log-coverage.md:134) The a11y leg's checks, re-proven at this take, must keep their silence census at 0, and a take's edit to them or their shared modules must re-run it.
  > a print, an env read, a subscriber or a listener in the eight checks the `a11y` leg runs beside its six earlier files (`accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`) and the two shared modules they read (`tests/common/mod.rs`, `tests/fixtures/mod.rs`) · searched: the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over those ten files, reading 0

### log-coverage#9.10 · listed (lost at classify) · .andromeda/obs-plan-detail/log-coverage.md:137
- (.andromeda/obs-plan-detail/log-coverage.md:137) The engine-debts checks must keep the silence census at 0 through the merge surface's re-read.
  > a print, an env read, a subscriber or a listener in the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` · searched: the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over those files, exit 1, reading 0

### log-coverage#9.11 · listed (lost at strong) · .andromeda/obs-plan-detail/log-coverage.md:138
- (.andromeda/obs-plan-detail/log-coverage.md:138) The popup checks must stay silent, including `select_popup` should the panic fix restore its skipped key-press row.
  > a print, an env read, a subscriber or a listener in the popup checks `select_popup`, `select_popup_painted` and `stand_act_popup` (a `session_common` reader) · searched: the silence census over those files, exit 1, reading 0

## Held

- ci-integration#1.10
- ci-integration#1.3
- error-capture-reporting#1.1
- error-capture-reporting#1.4
- log-coverage#1.1
- log-coverage#6.1
- log-coverage#7.1
- log-coverage#8.1
- log-coverage#8.3
- log-coverage#8.4
- log-coverage#8.7
- log-coverage#8.8
- log-coverage#8.9
- log-coverage#9.4
- log-coverage#9.5
- log-coverage#9.9
- metric-coverage#1.1
- metric-coverage#1.2
- metric-coverage#1.5
- metric-coverage#2.6
- metric-coverage#2.7
- pii-scrubbing-compliance#1.1
- pii-scrubbing-compliance#1.2
- pii-scrubbing-compliance#1.3
- pii-scrubbing-compliance#1.4
- pii-scrubbing-compliance#1.5
- pii-scrubbing-compliance#1.6
- pii-scrubbing-compliance#1.7
- pii-scrubbing-compliance#2.1
- pii-scrubbing-compliance#3.1
- pii-scrubbing-compliance#5.7
- span-trace-coverage#1.1
- span-trace-coverage#1.10
- span-trace-coverage#1.5
- span-trace-coverage#1.8
- span-trace-coverage#2.1
- span-trace-coverage#2.2
- span-trace-coverage#2.3
- span-trace-coverage#3.1
- telemetry-strategy#1.1
- telemetry-strategy#2.1
- telemetry-strategy#3.1
- telemetry-strategy#3.3

## Cascade only

- error-capture-reporting#1.2
- error-capture-reporting#1.3
- error-capture-reporting#1.5
- error-capture-reporting#1.6
- log-coverage#5.1
- log-coverage#9.2
- log-coverage#9.6
- metric-coverage#1.3
- metric-coverage#2.2
- metric-coverage#2.3
- metric-coverage#2.4
- metric-coverage#2.5
- pii-scrubbing-compliance#5.3
- pii-scrubbing-compliance#5.4
- span-trace-coverage#1.3
- span-trace-coverage#1.4
- span-trace-coverage#1.6
- span-trace-coverage#1.7
- span-trace-coverage#1.9
- telemetry-strategy#3.2
