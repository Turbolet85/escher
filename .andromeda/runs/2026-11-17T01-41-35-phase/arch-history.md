@history_item n=1 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (Standard Contracts: 9 rows) added `contract/base-document/layout-reason`, the closed `PlacedBy` · `ShiftBy` · `SizeCause` · `Length` sets, the crate-root and dioxus-native-dom re-exports, `contract/driver-session/layout` (`Session::layout`, in process, refusing stale / not-found, no step, no span) and `+ decision/layout-read-in-process-only`, PROVISIONAL on the delegate's answer pending the founder's word. This chunk is that word's consequence: the row is retired or rewritten at its wrap, the `LayoutExplanation` crosses as one line, and the unexplained element (mapped to `PlacedBy::Block` today) needs its own closed word before any carrier holds it — a new member of the closed `PlacedBy` set.

@history_item n=2 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (Established Decisions: 1 row) `~ decision/driver-spans-only`: one span per call "its run path runs", an in-process read outside the run path opening none, written for `Session::layout`. Once `layout` runs through `Session::run` it opens a `command` span (verb, cause alone when refused) like every verb, so this row's carve-out and `contract/driver-session/layout`'s "opens no span" must be re-read for the routed call.

@history_item n=3 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the closest precedent: `actions` became the fourteenth verb and tenth instance verb (`VERBS` 13 → 14, `FieldKind` 12 → 13, `describe()` 14, `tools/list` after `graph`, the recording's instance verbs 9 → 10, the CLI, the cold-agent pipe's tool count and README's count all moved), and the crossing was ratified as a boundary widening with no new carrier, exit, cause, session error, span field or dependency. It is also where the per-verb `target_verbs` table (`click` · `type` · `scroll` · `screenshot`, every instance verb that takes an `id`, no verb name in its code) and the shared `check_target` were built: `layout` takes an `id`, so it enters that table and every node of the actions answer gains the word, moving the Cells answer (106,088 bytes, unpaged, under the 1,048,576 bound; limit «An unpaged actions answer» owned by `res-7cc5bi4k`).

@history_item n=4 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the `graph` verb's landing: a read verb with an `Outcome` variant of its own, a hand-written JSON form, kept whole in a check, waiting on nothing in the load wait, recording `verb` alone on its span, the count of verbs moved at every surface that states it (README, recording, lint row, served CLI, cold-agent pipe). Template for which sites move from fourteen to fifteen and for "a read that dispatches nothing and settles nothing" through `Session::run`.

@history_item n=5 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — two entries: the lint built in process only (PROVISIONAL), then «the gated part: `lint` the twelfth verb, on every carrier and recorded» (`Command::Lint`, `Outcome::Linted`, the arm calling `Session::lint` so nothing is dispatched, `Check::from_bytes` taking the eighth instance verb, `describe` counts). The same in-process-first then carrier path this chunk follows for `Session::layout`, and the form the founder ratified for `layout`.

@history_item n=6 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — discharged the lint's in-process-only PROVISIONAL mark on the founder's «Новая команда lint», replacing the delegate's «In process only» and naming the owed verb: the exact precedent for retiring `decision/layout-read-in-process-only` on his `layout --id` answer.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` joined as an instance verb with an optional `id` resolved as `click` does, a new `Outcome` variant, a new `FieldKind`, a cause from that verb alone, and the MCP result form; it set the pattern of a verb whose `id` resolution reuses the run path's and whose span never fields its result.

@history_item n=8 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the one table rule: a later entry adds its verbs to `VERBS`, never a second table; refusal causes closed (now eight); `Command`/`Call` print ids under `Debug`, so nothing fields them. `layout` must be a `VERBS` row and add no cause.

@history_item n=9 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` is the one entry that runs a call, `validate` first; nothing public runs a `Command` not from validation; nothing resolved outlives a call. `layout` must reach `Session::layout` only through this path.

@history_item n=10 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — id resolution on the screen's snapshot reads `stale` where the record of seen ids holds the id, else `not-found`; a refused call runs nothing; a verb joins `VERBS` as a ratified widening. `layout`'s refusals reuse exactly these causes.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the `command` span's closed eight fields and its never-fielded list (any argument, any `Outcome`); the scope fixes that a `layout` call records `verb` and, refused, `cause` alone — no id, no reading, no count.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the socket widened to carry a call and its answer (ids, names, values, text, diffs; no `NodeId`), and the verb levels, `command_line`, hand-written JSON keyed by the schema's words, and no ending echoing caller input; the crossing rows `decision/socket-carries-driver-calls` and `decision/session-socket-carries-calls` that the layout answer must fit inside.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP crossing (`tools/list` in table order, results `{content, isError}`, a 1,048,576-byte bound) and `serde_json` as a reader only with the `tool_call` span holding `tool` and `ending`; standing rule that no span field holds what a call supplied or a screen reads.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — `describe()` and `help` written from the tables alone; the fixed `'static` text of the verb table rides the ratified carriers (`decision/fixed-program-text-on-carriers`). `layout`'s help and result fields come from its `VERBS` row, not separate text.

@history_item n=15 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — `mcp` split into the private children framing, listing and result, and `schema` holds its shape types in the child `spec` with tests in `schema_tests.rs` / `mcp_tests.rs`: where the new verb row, tool listing and result mapping land.

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check: owner-only, a record per answered instance-verb call, the strict `Check::from_bytes` over the instance verbs; a fifteenth verb must be admitted to its instance-verb list to be kept and read.

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each answer kept whole beside its digest and replayed through `Session::run` alone, `Check::replay` comparing the live kept line; a `layout` step is replayed and compared the same way, and the cross-OS byte checks hold no `screenshot` step.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting verbs wait on every carrier, while `snapshot`, `screenshot` (later `lint`, `graph`, `actions`) wait on nothing; `layout` joins the no-wait list, as the scope's "no load wait" requires.

@history_item n=19 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` and README's «the {word} driver commands as tools» held to `VERBS`, `Cause`, `SESSION_ERRORS`, `NODE_FIELDS` and `EXIT_STATUSES` by `CapabilityStatementTest` (`.github/scripts/test_capability_statement.py`); the fifteenth verb moves its counts in words, or the `ci-scripts` leg goes red.

@history_item n=20 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `+ decision/stand-readings-same-on-every-ci-os`: the six served tasks' snapshot, lint and actions digests equal on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes (guard `stand_cross_os_readings`). The actions digests move with the new verb word and must be re-read on all three OSes.

@history_item n=21 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `contract/driver-explore/explored-verbs`: the explored verbs are the actions table's target verbs less any whose result holds an image-kind field, selected by kind, never name. A `layout` verb that takes an `id` enters `target_verbs` and so the explored set unless excluded; a trap for exploration's step budget and path expansion. Also `contract/kept-check/opening-record` with `format` 1.

@history_item n=22 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `~ decision/no-node-id-across-session`: the closed list of what crosses a driver session gained "a step's caused text"; the layout answer's stable ids, closed words and numbers must fit this list or extend it with the widening.

@history_item n=23 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a box's client rect never subtracts its own scroll offset; the shared bounds reader subtracts only the containing-block chain's (`decision/client-rect-own-box`, `decision/client-rect-ignores-own-scroll`). The layout answer's offset and shift readings rest on these readers.

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the geometry readers) `Node::absolute_position` subtracts only its containing blocks' scroll (`decision/absolute-position-ancestor-scroll-only`, guard `scrolled_box_absolute_position`) and `get_client_bounding_rect` reads where transforms move a box: the readings beneath the layout reason's `Scroll` and `Transform` shifts.

@history_item n=25 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — `~ decision/client-rect-own-box` (why: differs from upstream's scroll-shifted geometry), `+ decision/reversed-scroll-origins` (reversed boxes scroll to negative offsets) and `+ contract/nodes-widgets/try-layout-data`: a layout answer's scroll shift can read negative, and its numbers must be written as the answer's own numbers on every carrier.

@history_item n=26 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the `stand-repeat` leg (three rounds, zero failures, plus seven_guis' CLI, host, MCP and recording targets) and the coverage floors (escher-driver 97 %); the new CLI, MCP and recording tests for `layout` run under that budget and the new executor arm must keep escher-driver above its floor.
