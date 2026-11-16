# P5 review — 2026-11-16-error-field-in-the-answer · «Error field in the answer»

The plan to open in an editor: `.andromeda/cache/docs/2026-11-16-error-field-in-the-answer/plan.md`, rendered from
`doc:2026-11-16-error-field-in-the-answer/plan` rev 2. No plan stands in the chunk's folder. Writing in that file
changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-16-error-field-in-the-answer` diffs the chunk's records.

## Goal
A `caused` item's field named `error` shows its text in every acting answer. The answer gets its own set of hidden
names, private to `capture.rs`: the sink's twelve without `error`. The other eleven still read `[redacted]`. The
sink's stderr lines, `CONTENT_FIELDS` and the span store do not change. `caused`'s help and the driver's docs say
the same. This is the founder's ruling, inputs#I1, item 2. That fork is not asked again.

## Top acceptance criteria
1. An app button's real parse failure, logged as `error = %e` with `url` and `path` beside it, reads «invalid digit
   found in string» in the answer's `error` field. `url` and `path` read `[redacted]`. This holds in both modes, on
   MCP, the socket and a kept check's replay.
2. In a build with the engine's tracing, a real failed stylesheet fetch:
   - in the answer, its `Resource load failed` item holds the fetch's error text in `error` and `url` reads
     `[redacted]`;
   - on the sink, the same record's line prints `error=[redacted]` and `url=[redacted]`, and the text occurs 0 times
     on the sink's lines.
3. A table of 12 literal rows: eleven names hidden in a kept record, `error` kept. The sink still redacts `error` under
   an `escher_*` target.
4. The sink's checks and both binaries' log checks read as before. `help` no longer lists `error` as hidden.
5. Three red-before-green `gate.py control` runs:
   - `error` back in the answer's set;
   - `error` out of `CONTENT_FIELDS`;
   - the capture reading `CONTENT_FIELDS` again.

## Touchpoints (5, all modified)
`packages/escher-telemetry/src/capture.rs` · `packages/escher-telemetry/src/format.rs` (a test row only) ·
`packages/escher-driver/src/schema.rs` · `packages/escher-driver/src/caused.rs` ·
`tests/blitz-tests/tests/stand_act_error_text.rs`

## Test commands (13 entries)
- Unit: escher-telemetry and escher-driver.
- `stand_act_error_text`, package-alone and with `--features dioxus-native-dom/tracing`.
- A help probe: `grep -c` reads 0.
- Companions: `stand_act_errors`, `stand_replay`, `stand_act_spans`, `stand_mcp_tools`, the three telemetry process
  checks, and seven_guis' `host_log`, `mcp_log`, `mcp_server` and `cli_commands`.
- `ci-leg.sh fast`, `ci-leg.sh stand-repeat`.
- Operator: hygiene, push, CI read with `--wait 2400`.

Baselines on the untouched tree:
- every suite is green;
- the help probe is red, as predicted (`schema.rs:194` still lists `error`); its must-pass control on a clone with the
  sentence reworded reads green, `control-entry-5.md`;
- stand-repeat is 3 of 3 green in 876.8 s.

## What P3 corrected
- The sink drops an app-target record whole (`format.rs:99`). So «the same record's line prints `error=[redacted]`» can
  only be read on an engine record. The plan reads the engine's real `Resource load failed` warn through an in-file
  failing net provider, and pins the `escher_*` rule in `format.rs`'s unit test.
- `schema.rs` (the `caused` help) and `caused.rs` (docs) joined the touchpoints. The row read re-ran whole on the
  corrected scope.

## Capabilities
None claimed: v030-10 is already `verified`. The pool (v030-11, v030-12, v030-15) holds nothing this chunk proves. No
concretization preview.

## Expected amendments (wrap)
These are in the plan's Implementation notes:
- security-plan · decision/caused-text-crossing, a boundary widening on `rul-izni57qv`. The founder's own word,
  2026-11-16, relayed verbatim by the overseer (inputs#I1).
- The «scrubbed by field name» / «same set» wording:
  - security-plan · decision/mask-holds-on-every-carrier, decision/socket-carries-screen-reading,
    decision/escher-telemetry-allowlist-sink;
  - obs-plan · contract/telemetry-scrub-set/error, decision/scrub-by-field-name, decision/answers-echo-nothing-typed;
  - architecture · contract/acting-verb-results/caused, decision/no-echo-of-typed-input.

## Beyond the plan
none
