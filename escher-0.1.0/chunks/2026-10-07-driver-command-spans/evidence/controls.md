# Red-before-green controls — 2026-10-07-driver-command-spans (plan step 6)

Measured 2026-10-07T22:04Z on the chunk's working tree, against the check as it stands. Each
mutation was applied alone, the three suites that hold the chunk's new tests were run, and the
sources were restored: the four files read the same `sha256sum` before the first mutation and after
the last (`execute.rs`, `format.rs`, the sink's `lib.rs`, `stand_act_spans.rs`).

This is the second run. The first (21:58Z) ran against the check before its per-call count was
narrowed from "the capture grew by one line" to "one driver line, every other line an engine
record" (the workspace build's engine call sites log beside a command — the report's deviation); the
same tests turned red under the same mutations in both runs, and only the stand messages of
mutations 6 and 8 read differently.

Suites run under every mutation:
- **driver-unit** — `cargo test -p escher-driver --locked --lib`
- **sink-unit** — `cargo test -p escher-telemetry --locked --lib`
- **stand** — `cargo test -p blitz-tests --locked --test stand_act_spans` (two parents, each
  re-running the binary on one ignored child)

A message is recorded by its kind and its counts. The sink unit tests print the line they read,
which holds only this crate's synthetic sentinel; the stand check's messages hold a layout mode, a
task and a call index, or kinds and counts.

| # | Mutation | Turned red | Message (kind · count) | Stayed green |
|---|---|---|---|---|
| 1 | the driver records `added` from the `changed` list (`execute.rs`, `record_result`) | stand: `each_driver_command_leaves_one_line_holding_nothing_it_handled`, through `child_records_each_command_at_info` | per-call: `incremental=false: Crud: call 1` — the line's three counts are not the lengths of the returned diff's lists · 1 test | driver-unit, sink-unit |
| 2 | the driver records the cause of an executor refusal (`stale`) as the word of another cause (`not-found`) | stand: the records parent, through its child | per-call: `incremental=false: Crud: call 20` — the line's cause is not the name of the cause the call returned · 1 test | driver-unit, sink-unit |
| 3 | the driver fields the typed text under a ninth name, `typed`, added to its table | stand: the records parent, through its child · driver-unit: `execute::tests::the_span_has_eight_fields_and_none_is_an_argument_name` | whole-capture, both kinds in one message: `1 field names outside the stated set, 0 redacted values, 0 of 36 id needles, 0 of 8 name needles and 1 occurrences of a text a call supplied, in 27 lines` · unit: the table's rows differ from the eight stated (`left == right`) · 2 tests | sink-unit |
| 4 | the busy-class function answers `Busy::Layout` with the word of `Busy::Loads` | driver-unit: `execute::tests::each_busy_class_reads_as_its_schema_word` | `row 1` · 1 test | sink-unit, stand (no stand step reads not settled — the arm is not driven there) |
| 5 | the span's level is raised to WARN | stand: `a_driver_command_writes_no_line_at_the_default_level`, through `child_drives_at_the_default_level` · and the records parent | default level: `1 probe lines and 4 driver lines at the default level` · records: `incremental=false: Crud: call 0` — the line is not at INFO · 2 tests | driver-unit, sink-unit |
| 6 | the sink also prints a line when a span is created (`FmtSpan::NEW | FmtSpan::CLOSE`) | sink-unit: (a) `escher_span_closes_into_one_line_and_writes_nothing_before` · also (b), (d), (e) · stand: the records parent | (a): `written at creation` · (b), (d), (e): line count `left == right` · stand: `call 0: the call left 2 driver lines and 0 lines under no engine target` · 5 tests | driver-unit |
| 7 | the sink prints a span's stored fields without judging them | sink-unit: (b) `content_named_span_field_is_redacted_and_a_value_stays_one_pair` · (d) `engine_target_span_prints_only_safe_fields` | (b): the content-named pairs read an empty value, not the marker (the value itself was never stored) · (d): the pair `attribute=[redacted]` is counted 0 times, the engine span's unsafe fields print · 2 tests | driver-unit, stand (the driver's eight fields all print under an escher target either way) |
| 8 | the sink stops dropping an outside-target span | sink-unit: (c) `outside_target_span_writes_no_byte` · stand: the records parent | (c): one line written for the outside target, its head alone · stand: `call 0: the call left 1 driver lines and 6 lines under no engine target` · 2 tests | driver-unit |
| 9 | the sink adds the enclosing span's name to an event's line | sink-unit: (e) `event_inside_a_span_reads_as_it_does_outside_one` | the event inside the span no longer equals the one outside (`left == right`) · 1 test | driver-unit, stand |

Every new test was seen red at least once: the records parent and its child (1, 2, 3, 5, 6, 8), the
default-level parent and its child (5), the five sink unit tests — (a) under 6, (b) under 6 and 7,
(c) under 8, (d) under 6 and 7, (e) under 6 and 9 — and the two driver unit tests (3, 4).

Readings beside the plan's forecast:
- Mutation 3's stand red is one message carrying both kinds (field names, content), as the plan
  asks ("red twice over"): the whole-capture assertion counts every kind before it fails.
- Mutation 6 turns four sink unit tests red, not (a) alone, and the stand check too.
- Mutation 8 also turns the stand check red: at `RUST_LOG=info` six spans of outside targets close
  inside the first `snapshot` call, so the outside-target drop is what keeps a command at one line.
- Mutation 7 leaves (b)'s content-named values absent even while unjudged — they read empty, never
  the sentinel — because the field formatter does not store them; (b) is red on the missing marker.
