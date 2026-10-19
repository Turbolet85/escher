group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:32   class: stale
  leaf-text: «with each call's `accepted` and answer digest, no screen reading and no log or event line»
  source: .andromeda/obs-plan.md:341 «with each call's `accepted`, its answer digest and the answer kept whole as `answer` — the line the driver wrote, masked as on every carrier, a shot's as `{width, height}` without `png` — so the screens' ids»
  replace-with: «with each call's `accepted`, its answer digest and the answer kept whole as `answer` — the line the driver wrote, masked as on every carrier, a shot's as `{width, height}` without `png` — so the screens' ids, names, values, snapshot text and diffs at rest, and no log or event line»

- leaf: .claude/docs/obs-summary.md:32   class: omission
  leaf-text: «the checks write theirs under `target/tmp/`) are local only — no CI job uploads any of them.»
  source: .andromeda/obs-plan.md:343 «`tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, two kept checks in git (mode `100644`, a public repository) — the repository's own scripted stand-Counter sessions»
  replace-with: «the checks write theirs under `target/tmp/`) are local only — no CI job uploads any of them. Two kept checks are committed in git, `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` (mode `100644`, a public repository): the repository's own scripted stand-Counter sessions, written by `examples/seven_guis/tests/flows/record_checks.sh` (which prints only `step N`), holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name, secret or `screenshot` step — read by `stand_replay` and seven_guis' freshness check; no log or event line; uploaded by no CI leg.»

- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «the kept check itself holds call content at rest, unmasked typed text included — a file, never a log line, field or print.»
  source: .andromeda/obs-plan.md:76 «writes each answered call's arguments, `accepted`, its answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) into the kept check through the crate's `json` writer»
  replace-with: «the kept check itself holds call content at rest, unmasked typed text included, and each answer itself as `answer` (a shot's without `png`) — the screens' ids, names, values, snapshot text and diffs at rest — a file, never a log line, field or print.»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the session-recording checks `stand_session_recording` (a `session_common` reader) and seven_guis `recording` likewise»
  source: .andromeda/obs-plan.md:76 «`stand_replay` (a `session_common` reader, replaying the two committed checks in process in both layout modes, its failure messages holding the mode, the file and a step index alone)»
  replace-with: «the session-recording checks `stand_session_recording` (a `session_common` reader, replaying through `Check::replay`), `stand_replay` (a `session_common` reader, replaying the two committed checks in process in both layout modes, its failure messages holding the mode, the file and a step index alone) and seven_guis `recording` likewise»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «writing each answered call's arguments, `accepted` and its answer's FNV-1a 64 digest into the kept check through the crate's `json` writer — a file at rest, never a log line, field or print.»
  source: .andromeda/obs-plan.md:76 «and writes each answered call's arguments, `accepted`, its answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) into the kept check through the crate's `json` writer»
  replace-with: «writing each answered call's arguments, `accepted`, its answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) into the kept check through the crate's `json` writer — a file at rest, never a log line, field or print; the sixteenth, `replay`, holds no tracing site, env read or print and calls `serde_json` only to read.»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`Check::from_bytes` and the wire's `recording-failed` reply —,»
  source: .andromeda/obs-plan.md:115 «`Check::from_bytes`, the replay `Check::replay` — each replayed step writing its own `command` span through `Session::run`, the replay adding none — `Replay` and `Change` holding only bools»
  replace-with: «`Check::from_bytes`, the replay `Check::replay` — each replayed step writing its own `command` span through `Session::run`, the replay adding none — and the wire's `recording-failed` reply —,»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «`Check::from_bytes`, the wire's `recording-failed` reply)»
  source: .andromeda/obs-plan.md:115 «`Check::from_bytes`, the replay `Check::replay` — each replayed step writing its own `command` span through `Session::run`, the replay adding none — `Replay` and `Change` holding only bools»
  replace-with: «`Check::from_bytes`, the replay `Check::replay` — each replayed step writing its own `command` span through `Session::run`, the replay adding none —, the wire's `recording-failed` reply)»

- leaf: .claude/rules/observability.md:32   class: stale
  leaf-text: «holds each answered call's arguments — unmasked typed text included —, its `accepted` and its answer's FNV-1a 64 digest at rest»
  source: .andromeda/obs-plan.md:76 «writes each answered call's arguments, `accepted`, its answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) into the kept check through the crate's `json` writer»
  replace-with: «holds each answered call's arguments — unmasked typed text included —, its `accepted`, its answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) at rest»
