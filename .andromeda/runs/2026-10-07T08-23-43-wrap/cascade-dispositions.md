# Cascade dispositions — 2026-10-07-sink-target-allowlist

Written from the tool's final listing (`cascade.py sweep`, the third run of this pass, after the last body and leaf
edit; trail `cascade-2026-10-07-sink-target-allowlist.json`), before any sidecar entry landed. Baseline `ebd7411f`, the
parent of the one pre-CI commit. The listing printed 87 rows: master and key-file text 68 (new 11 · standing 57),
leaf 18, curation 1, judgment base 0.

## The search
- Pattern set: `cascade-patterns.toml`, 22 patterns, each with a known-positive control that fired on the pre-pass
  masters (the tool refused none). They key the retired claim's wording (`as written`, `unredacted`, `engine
  allowlist`, `at any target`, `allowlist scrub|scrubs by allowlist`), its qualifiers (`not measured by level` and two
  variants, `Sink target allowlist`), its mechanism as stated in figures and levels (`(ids?|names?) at … debug|trace`,
  `×12|×20|1165|1501|1.1 MB`, `host-stderr-by-level.md`, `third-party`, `one line per event`), the three moved citation
  paths, and the moved counts and inventories (`572|140 result|7 ignored`, `5 inline unit tests|5 unit`, `four
  telemetry_*`, `one integration-test target`, `hb-refuse`, `re-execute`, `host_binary`).
- Beside the tool, one occurrence-level probe over the masters and key files for phrasings the set does not key:
  `default level`, `3 lines`, `boot smoke`, `windowed stand`, `the scrub`, `scrub ` — it reached nothing the tool's
  rows had not, and confirmed the three standing restatements named below.
- Sections read whole or by offset window: every line a proposal names (38 master lines, 2 key-file lines), and every
  standing row's window before it was left.
- NOT looked for: a restatement of the drop's cost under a wording that names no level, target or figure; the
  `log.file` residual under any wording but `third-party` and `log.file` itself.

## Patterns with zero rows (their controls fired)
`unredacted` · `engine-allowl` · `any-target` · `allowl-scrub` · `by-level` · `old-evidence` · `per-event` ·
`four-files` · `one-target` — the retired wording stands in no master, key file, leaf, curation home or judgment
base. A statement about these patterns, not an absence proof.

## Master and key-file rows
- **as-written** — standing 2, no change: `architecture.md:121` ("dropped since it was written") and `:136`
  ("something was written since the last drain") are another sense of the phrase, both read at their offsets.
- **route-entry** — new 1 (the obs key file's `pii-scrubbing-wire` row: "delivered by the route entry") · standing 2,
  both amended this pass (`security-plan.md:257`, `obs-plan.md:295`: "owed by" → "delivered by").
- **leak-levels** — new 2: `security-plan.md:116` ("the record that carried a name at `trace`", now dropped) and the
  obs key file ("no stable id and no accessible name at `RUST_LOG=debug` or `trace`"). This pass's own text.
- **old-figures** — new 3 (`security-plan.md:376` ×2 and `obs-plan.md:290` ×2, the `1165 → 1` / `1501 → 1`
  readings; `test-plan.md:180`, the red run's "1501 stderr lines") · standing 2: `test-plan.md:49` amended (the 1.1 MB
  figure dated to before the drop), `obs-plan.md:264` no change (`runtime.rs:1165` is a citation).
- **third-party** — new 2 (`test-plan.md:180`, `obs-plan.md:295`) · standing 7: three amended this pass
  (`security-plan.md:116`, `:376` ×2, `obs-plan.md:290`); four no change, another subject — `architecture.md:101`
  (IPC), `:214` (Servo), `security-plan.md:247` (CDN fixtures), `obs-plan.md:86` (no third-party logger).
- **cite-format** 6 rows · **cite-lib** 14 rows · **cite-hostbin** 5 rows — every occurrence re-pointed this pass
  (format.rs 7, lib.rs 16, host_binary.rs 7 of 8) or unmoved (`architecture.md:264`, `host_binary.rs:1-3`, the module
  doc). All 56 citations into the chunk's six files were then resolved against the tree: 0 suspect.
- **count-chain** — standing 3: `test-plan.md:322` amended (a new link appended; the 140 · 572 · 7 link stays as the
  chain's history); `security-plan.md:124` and `:346` no change (`570-572`, `569-572` are citations).
- **unit-count** — standing 1, no change: `test-plan.md:24` is dioxus-native-dom's "55 unit tests".
- **state-dirs** — new 2 (`architecture.md:149`, `obs-plan.md:325`: `hl-trace` added).
- **re-exec** — standing 2, both amended lines: `architecture.md:150` ("three … re-execute", and "two more re-execute
  it as a session host", which stands) and `obs-plan.md:69` (`stand_id_persistence`'s re-executed child, which stands).
- **host-spawn** — new 1 (`architecture.md:149`) · standing 13: ten on lines amended this pass; three no change, each
  read at its offset and true as written and not exclusive — `test-plan.md:21` (the lifecycle edges are covered by
  `stand_session_lifecycle` and `host_binary`), `:185` (`host_binary.rs` is an integration test outside `src`), `:208`
  (`host_binary` launches the binary; none drives the UI through it).

## Folded in by this sweep (beyond the 40 fan-out proposals)
- `security-plan.md:176` (§Data Protection) — "escher-telemetry's allowlist scrub" → the sink redacts and, for an
  outside target, drops. The retired claim's own name for the sink, in a section no detector read.
- `test-plan.md:102` — "one line per event" → "one line per printed event" (an intra-line duplicate the first apply left).
- `a11y-plan.md:87` — "one non-JSON text line per event" → "per printed event" (the same).

## Curation homes and judgment bases
- curation 1, no change: `.claude/docs/session-learnings.md:71` ("left as written") is another sense. No stale entry
  routes to P3 from this sweep.
- base 0: neither `playbook.md` nor `drift-base.md` quotes retired wording.

## Leaves (step 3 — recomputed from the amended masters, not from these rows)
- Re-derived: `CLAUDE.md` (the escher-telemetry module line, the sink warning, the driver-session and telemetry
  pointer rows) · `.claude/rules/observability.md` (the PII bullet; its Session Additions untouched) ·
  `.claude/docs/obs-summary.md` (three bullets) · `security-summary.md` (two rows, one bootstrap line) ·
  `tests-summary.md` (four bullets) · `services/escher-telemetry.md` (read whole and rewritten: two stale lines there
  carried none of the swept wording) · `services/escher-driver.md` · `services/seven_guis.md` · `commands.md`.
- Leaf rows left, 18: all on text re-derived this pass (the delivered-by wording, the new readings, the third-party
  WARN cost, the `host_binary` beside `host_log` mentions, the count chain's earlier link kept as history) except
  `services/dioxus-native-dom.md:19` and `:41`, another sense and another crate's count — no change.
- Read and left: `a11y-summary.md`, `gotchas.md`, `conventions.md`, `stack.md`, `workflow.md`, `design-summary.md`
  and the rules `testing.md`, `security.md`, `a11y.md`, `verification-harness.md` — searched for `sink`, `scrub` and
  `telemetry`; five hits, none stating the scrub's reach, a level, a count or a moved coordinate.
