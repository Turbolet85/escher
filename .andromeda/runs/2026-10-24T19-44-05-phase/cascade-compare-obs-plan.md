# Cascade compare — obs-plan · phase · 2026-10-24-a11y-re-read
E items 30 · held 4 · lost at classify 4 · lost at strong 1 · beside a placed fact - · cascade only 4 · body 15 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### log-coverage#6.1 · lost at classify · .andromeda/obs-plan-detail/log-coverage.md:55
- (.andromeda/obs-plan-detail/log-coverage.md:55) The Tab-order and activation re-read moves focus through blitz-dom's one focus log site; the chunk must leave it as it stands (an INFO line, behind the `tracing` feature) and add no other.
  > info: focussed node (packages/blitz-dom/src/document.rs:1871-1872)

### pii-scrubbing-compliance#2.1 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) The chunk adds no log field, span or target, so the sink's allowlist and its content-named scrub set must stand as written; any row re-recorded or test added must route nothing through a new field or an unadmitted target.
  > an event whose target starts with `escher_` (`ESCHER_TARGET_PREFIXES` — the underscore is part of the prefix: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted, `escher` alone is not) prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted; an event from any other target is dropped whole before anything is written — no time, no target, no newline — at every level, WARN and ERROR included, and whatever `RUST_LOG` names (packages/escher-telemetry/src/format.rs:144-148);

### pii-scrubbing-compliance#2.1 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20) The scrub sets and the rule must stay unchanged, and `escher_driver`'s emitters stay the two closed-word spans; the re-read must add no third emitter.
  > the five scrub sets and the rule are unchanged, and `escher_driver` now has two emitters, the command span (§4) and, since 2026-10-10-mcp-surface, the `tool_call` span (§4), neither of whose two fields, `tool` and `ending`, is in the content-named set — both print, both closed words —

### pii-scrubbing-compliance#1.1 · lost at strong · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13) The re-read covers the `f9cbb2756` take's engine code; the sink checks that read it must still pass under both builds when the chunk closes (the fast leg's workspace tests).
  > `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build;

### pii-scrubbing-compliance#1.1 · lost at strong · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:10
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:10) The host's needle counts are the standing values a host-level reading after the chunk must match.
  > with the id, name and typed-sentinel needles still 0 at `info` and `trace` under both builds

### span-trace-coverage#3.1 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:36
- (.andromeda/obs-plan-detail/span-trace-coverage.md:36) The `tool_call` span's field count is a measured value, and the chunk must leave it unchanged.
  > no field added, the span keeps its two

### telemetry-strategy#1.1 · lost at classify · .andromeda/obs-plan-detail/telemetry-strategy.md:8
- (.andromeda/obs-plan-detail/telemetry-strategy.md:8) Any code the re-read touches (a new `stand_keyboard` row, a widened `a11y` leg, a defect fix if one is ruled) must add no `tracing` site outside this mechanism: engine and upstream crates emit only behind the `tracing` cargo feature, and escher-driver's spans stay the two named, none added.
  > Telemetry is the `tracing` crate's event macros and, in escher-driver, two span macros (`info_span!`, `command` and, since 2026-10-10-mcp-surface, `tool_call`, §4), in the engine and upstream crates compiled in only with the `tracing` cargo feature

## Held

- ci-integration#1.3
- log-coverage#7.3
- span-trace-coverage#2.1
- telemetry-strategy#2.1

## Cascade only

- ci-integration#1.4
- span-trace-coverage#1.10
- telemetry-strategy#3.1
- telemetry-strategy#3.3
