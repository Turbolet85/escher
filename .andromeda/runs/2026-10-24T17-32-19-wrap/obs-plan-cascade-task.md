# Task — obs-plan · 2026-10-24-stand-requirement-sweep

## The session
The chunk changed test code and one CI leg only: 9 files changed, no new file, no library surface, no crate, `[[test]]` target, dependency, env var or listener added, and nothing under `packages/` changed.
`stand_requirement_sweep` goes from 2 to 3 tests. It adds a second record table, `WITNESSES_020` (rows v020-01…09, 46 witnesses), a `Subject::Unit` kind, the consts `STAND_REPEAT_NAMED` (10) and `STAND_REPEAT_SEVEN_GUIS` (8), and 28 new `source()` arms, among them `.github/scripts/ci-leg.sh` read through `include_str!` (`CI_LEG`). Its new test checks that every 0.2.0 witness runs inside `stand-repeat`.
The `stand-repeat` leg now names ten more blitz-tests targets as literal `--test` words after the `stand_*.rs` glob: `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction` and `text_extent`. This is the founder's word of 2026-10-24, and `test_ci_workflows.py` pins the ten. CI's `Stand checks repeated` reads 198 result lines · 837 passed · 0 failed · 21 ignored.
`stand_lint` goes from 6 to 7 tests. It asserts that a clipped or off-screen node holding text also reads `truncated`. `text_extent` goes from 3 to 6 tests and asserts the reader's three stated limits: RTL text, vertical text and a transformed box.
`stand_act_spans`' sink capture now holds a paged part read, part 2 of a screen that pages, which takes it from 35 to 36 calls per mode. The fixture page `many_items` and its const `ITEMS` (400) moved unchanged into the shared `fixtures` module.
`host_log` goes from 2 to 3 tests. The new test reads the `start --record` clients' own stderr at `RUST_LOG=trace` in the new state directories `hl-client` and `hl-client-rec`, and finds 0 of every needle class. The chunk's silence census reads 0 (its gate entry 8). A paged part read through the `escher-session` host's own sink is stated as unmeasured, and a residuals line owns that limit.

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- tests/blitz-tests/tests/stand_act_spans.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- examples/seven_guis/tests/host_log.rs
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- `obs-plan.md:77`: `stand_requirement_sweep` reads "its `WITNESSES` table" through `include_str!`. It now also reads a second table, `WITNESSES_020`, and the leg script `.github/scripts/ci-leg.sh` through a `source()` arm (`CI_LEG`), with 3 tests where it had 2. It still installs no subscriber, reads no env var and prints nothing.
- `obs-plan-detail/pii-scrubbing-compliance.md:9`: "35 calls per mode … a part read among them" becomes 36 calls per mode. The capture now also holds part 2 of a screen that pages.
- `obs-plan-detail/pii-scrubbing-compliance.md:10`: the 2026-10-14 one-off reading of a client's own stderr is now joined by a standing check. `host_log`'s third test reads that stderr at `trace` under `--record`.
- `obs-plan-detail/ci-integration.md:15`: the session checks' state-directory list ("`hb-*`, `hl-trace`, `cc-*`, `cf-*` and `ht-timer`") gains `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`. `hl-rec` and `hl-rec-kept` are missing from that list too, a gap that predates this chunk.
- `obs-plan.md:91`: the shared `fixtures` module is described as holding "three pages". It now also holds `many_items` and `ITEMS`, moved in from `stand_snapshot_parts`. The census still reads it silent, and its reader count of 24 is not restated by the report.
- `obs-plan.md:91` cites `tests/blitz-tests/tests/fixtures/mod.rs:87-130`. The 23 lines added at 64-86 shift that span, so the citation moves.
- `obs-plan.md:95`: `stand_snapshot_parts` now reads `fixtures::many_items` in place of its own in-file page. It stays a `fixtures` and `session_common` reader and stays silent.
- `obs-plan.md:72`: on the re-run children of `stand_act_spans`, they now also hold a `fixtures` page (`paged`). `Needles::read` takes the page's button text as name needles and its 400 `item-NNN` ids as id needles. The count of 4 print or sink sites is unchanged.
- `obs-plan.md:80` cites `tests/blitz-tests/tests/stand_act_spans.rs:102` and `:569-577`. Lines added at 6-8 and 243 shift both citations.
- `obs-plan.md:79`: `host_log` "spawns the same binary on `crud` at `RUST_LOG=trace` … over three hosted calls". It now has 3 tests. The third runs each client command (`start crud --record`, `type` with a sentinel, `snapshot`, `snapshot --part 1`, `lint`, `stop`) through `common::run` at `trace`, guarded by `common::Started`.
- `obs-plan.md:80` cites `examples/seven_guis/tests/host_log.rs:156-175`. The 97 lines added at 11-15 and 40-131 shift that citation.
- `obs-plan.md:88`: `host_log` already reads a recording host at `trace`. It now also reads the recording clients' own stderr, which holds 0 recording-path needles and 0 `recorded_needles`.
- `obs-plan.md:90`: the seven_guis shared module's readers `recorded_needles` and `Started` are now also used by `host_log`'s third test. The module file itself did not change.
- `obs-plan.md:92`: the visual-lint silence covers the `stand_lint` and `text_extent` check files, and both grew. `stand_lint` has a new in-file page, `reading_page`, and one more test; `text_extent` has three new tests and the helper `extent`. The chunk's census reads 0, so the claim is re-read and unmoved.
- `obs-plan.md:250`: "the sink's lines read 0 id, name and typed-text needles … in the `escher-session` host". A standing check now adds the same zero for a recording session's clients' own stderr at `trace`.
- The body's absence · logs block (`obs-plan.md:84-98`) and log-coverage's absence · logs group hold one "checks are silent" line per chunk. This chunk's census reading is a candidate for such a line.
- `obs-plan-detail/ci-integration.md`, if it names the `stand-repeat` leg or its selection: the leg now runs the `stand_*` files, the ten named targets and seven_guis' eight. Its CI job reads 198 · 837 · 0 · 21, where it read 168 · 684 · 0 · 21.
- Any detail entity that counts `stand_act_spans` calls, `host_log` tests, the `fixtures` module's pages or the session state directories moves with the counts above.
- `obs-plan-detail/log-coverage.md:41` holds the part-read line's shape. That shape is unchanged, so the line is re-read and unmoved.
- `obs-plan-detail/span-trace-coverage.md:28` and `obs-plan.md:92` name the `text_extent` reader. The reader did not change, so these are re-read and unmoved.

## New facts
- F1 · pii-scrubbing-compliance · `host_log`'s third test is `a_recording_sessions_clients_log_nothing_of_it_at_trace`, built only on unix. It runs a recording session's client commands at `RUST_LOG=trace`: `start crud --record`, `type` with a sentinel, `snapshot`, `snapshot --part 1`, `lint`, `stop`. Over all the clients' own stderr it reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles`, and it is a standing check under both builds.
- F2 · pii-scrubbing-compliance · A paged `part` read through the `escher-session` host's own sink stays unmeasured, because the host boots only the four lean tasks and none of them pages. This is a stated limit owned by a residuals line (target: next).
- F3 · pii-scrubbing-compliance · `stand_act_spans`' sink-installing children read part 2 of a screen that pages (`fixtures::many_items` held as `paged`, `Outcome::Part` part 2 of at least 2). That page's 400 `item-NNN` ids and its buttons' text join the needles, and no id or name of the page appears on a captured line.
- F4 · log-coverage · The stand requirement sweep's test changes are silent. They add no log site, span, field, log field, env read or listener, and the chunk's silence census reads 0 (gate entry 8). `packages/` is unchanged, so escher-telemetry and escher-driver keep their sites. `RUST_LOG` on `host_log`'s client command is the registered `RUST_LOG` read.

## Untouched
- escher-telemetry's subscriber, line format, target allowlist, scrub sets and panic hook (§3, §6 log format, §7, §8 scrubbing): nothing under `packages/` changed.
- escher-driver's two spans, `command` (eight fields) and `tool_call` (`tool` · `ending`), and the driver's 2 `info_span!` sites with 0 events: no driver source changed.
- Telemetry strategy and feature wiring (§2): no manifest, lock or feature changed, and the by-invocation rule for `blitz-dom/tracing` stands.
- Metric coverage (§5) and error capture (§7): no timing, counter, panic hook or error path was added or changed.
- The agent-run harness log and its events (§3, §6): `run stand` keeps its prefix selection. The ten newly named targets reach the agent harness only through `run all` or `run {name}`, and its event shapes are unchanged.
- The cold-agent pipe log, its `server.log` span counting and its verdict fields (§3, §6, §8): the pipe was not touched.
- `mcp_log` and its readings at `trace`: still 2 tests, unchanged, passing under both builds.
- The `stand_mcp_tools` recorder, the `stand_id_persistence` and `stand_screenshot_repeat` re-executed children, and the `session_common` module and its reader counts: none was reached.
- The upstream apps' stdout subscribers, the WPT runner's `env_logger` and the publish builds' logging (§3, §8, §9): not reached.
- The CI artifact handling in §9 (a leg's merged log on failure, the coverage report on success) and the two committed kept checks: unchanged. The coverage floors are unmoved.
