# Cascade compare — obs-plan · phase · 2026-10-26-codebase-health
E items 28 · held 4 · lost at classify 3 · lost at strong 3 · beside a placed fact - · cascade only 0 · listed 2 · body 11 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.4 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:13
- (.andromeda/obs-plan-detail/ci-integration.md:13) The helpers or table walk the chunk introduces in `packages/escher-driver/src/lint.rs` are workspace public source counted in the `coverage` leg's lcov report, so the CI run the chunk's verdict reads must still produce and upload that report on a passing coverage leg.
  > | Coverage report | the `coverage` leg's lcov file `target/coverage/lcov.info` — line counts of the workspace's public source, no user data (§8), `keyboard_compat.rs` excluded by filename since 2026-10-17-upstream-sync-engine-debts (test-plan §10 Coverage floors); uploaded only on success as artifact `coverage-report` from `target/coverage/`, kept 7 days — the one fork-CI artifact outside `target/ci-logs/` |

### ci-integration#1.10 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:19
- (.andromeda/obs-plan-detail/ci-integration.md:19) "No behaviour changes" includes the two committed kept checks: they stay as committed and still replay under `stand_replay` with every kept answer equal, the chunk writing no check of its own into git.
  > | Committed checks | `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, two kept checks in git (mode `100644`, a public repository) — the repository's own scripted stand-Counter sessions, written by `examples/seven_guis/tests/flows/record_checks.sh` (which prints only `step N`), holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name, secret or `screenshot` step — read by `stand_replay` and seven_guis' freshness check; no log or event line; uploaded by no CI leg |

### pii-scrubbing-compliance#1.1 · lost at strong · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:9
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:9) The refactored `lint.rs::findings` must leave the `lint` verb's accepted and refused command-span lines exactly as `stand_act_spans` reads them, each line holding `verb` alone with no id or name needle captured, in both layout modes.
  > the verb table held seven since 2026-10-12-headless-screenshot, when `stand_act_spans` also began checking a refused shot's `fonts-unavailable` cause on the span, and holds eight with the `lint` verb, whose accepted line and refused no-font line `stand_act_spans` also reads (36 calls per mode, eight verbs read back, two part reads among them — part 1, and part 2 of the shared `many_items` page held as `paged`, read as `Outcome::Part` with at least 2 parts, its 400 `item-NNN` ids and its buttons' text joining the id and name needles (`Needles::read`), none on a captured line — each line `verb` alone).

### pii-scrubbing-compliance#1.1 · lost at strong · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11) A hosted `lint` is one of the client commands `host_log`'s third test reads at `RUST_LOG=trace`; after the lint's factoring that check must still read 0 id, name, sentinel, recording-path and `recorded_needles` occurrences under both builds.
  > A recording session's clients are read by a standing check: `host_log`'s third test (unix only) runs each client command through `common::run` at `RUST_LOG=trace` — `start crud --record`, `type` with a sentinel, `snapshot`, `snapshot --part 1`, `lint`, `stop` — in the state directories `hl-client` and `hl-client-rec`, guarded by `common::Started`, and over all the clients' own stderr reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences under both builds (`host_log` 3, green as `-p seven_guis` and as `--workspace`);

### pii-scrubbing-compliance#2.1 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) Any helper or table the chunk adds under `escher-driver` sits under the `escher_` target rule: it must add no field, and certainly none outside the content-named redaction set, to what the sink prints.
  > an event whose target starts with `escher_` (`ESCHER_TARGET_PREFIXES` — the underscore is part of the prefix: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted, `escher` alone is not) prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted;

### pii-scrubbing-compliance#2.1 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20) `escher_driver` must keep exactly its two emitters, the `command` and `tool_call` spans with their closed-word fields; the factoring of `findings` and of the MCP listing test must add no third emitter and change no scrub set.
  > the five scrub sets and the rule are unchanged, and `escher_driver` now has two emitters, the command span (§4) and, since 2026-10-10-mcp-surface, the `tool_call` span (§4), neither of whose two fields, `tool` and `ending`, is in the content-named set — both print, both closed words — its closed-span line ratified by the founder (2026-10-09), as §6 → Log format states

### span-trace-coverage#3.1 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:37
- (.andromeda/obs-plan-detail/span-trace-coverage.md:37) Under "every check green", the lint tool-call span check must still pass after `lint.rs` is refactored, in both layout modes. That means the `{"findings":[…]}` reply and one `command` child whose `verb` is `lint`.
  > a `lint` tool call ends `accepted` (its reply the JSON line `{"findings":[…]}`, one text block) or `refused` and parents one `command` child whose `verb` is `lint`, measured in process in both layout modes (`stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command`, tests/blitz-tests/tests/stand_mcp_tools.rs:706-766); no field added, the span keeps its two;

### telemetry-strategy#1.1 · lost at strong · .andromeda/obs-plan-detail/telemetry-strategy.md:8
- (.andromeda/obs-plan-detail/telemetry-strategy.md:8) The factoring of `lint.rs::findings` into helpers or a table walk must leave escher-driver's telemetry at exactly its two span macros, `command` and `tool_call`: no new `tracing` event, span or `info_span!` site in any helper it introduces.
  > Telemetry is the `tracing` crate's event macros and, in escher-driver, two span macros (`info_span!`, `command` and, since 2026-10-10-mcp-surface, `tool_call`, §4)

### telemetry-strategy#1.1 · lost at strong · .andromeda/obs-plan-detail/telemetry-strategy.md:8
- (.andromeda/obs-plan-detail/telemetry-strategy.md:8) escher-driver's `tracing` dependency shape must stay as stated (ungated, no `[features]` table, the same workspace requirement and lock): the chunk adds no feature, dependency or manifest change to reach its complexity target.
  > escher-driver is the second escher crate to take `tracing` ungated (`{ workspace = true }`, no `[features]` table; the workspace requirement is 0.1.40, the lock holds 0.1.44): its command span is compiled into both binaries through `seven_guis → escher-driver`

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### log-coverage#9.3 · listed (lost at strong) · .andromeda/obs-plan-detail/log-coverage.md:123
- (.andromeda/obs-plan-detail/log-coverage.md:123) Refactoring `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` must not add a log site, span, field, env read or listener to `stand_requirement_sweep`'s `WITNESSES_020` table or its leg check, so the silence census over the file must still read 0.
  > observed absent — a log site, span, field, env read or listener in the 0.2.0 requirement record's test additions (`stand_requirement_sweep`'s `WITNESSES_020` and its leg check reading `.github/scripts/ci-leg.sh` through `include_str!`, `stand_lint`'s `reading_page` and its seventh test, `text_extent`'s `extent` helper and three limit tests, `stand_act_spans`' paged part read, the `fixtures` module's `many_items` and `ITEMS`, seven_guis `host_log`'s third test) · searched: the silence census over those files, reading 0;

### telemetry-strategy#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/telemetry-strategy.md:22
- (.andromeda/obs-plan-detail/telemetry-strategy.md:22) The refactored `stand_requirement_sweep` test (a test-crate file) must add no telemetry or logging crate use, keeping the test crate slice's recorded absence true.
  > observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)

## Held

- ci-integration#1.3
- log-coverage#6.1
- span-trace-coverage#2.1
- telemetry-strategy#2.1

## Cascade only
