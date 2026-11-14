@history_item n=1 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `fonts-unavailable` as a cause name (nine causes, seven from the executor), returned before anything is painted; an accepted shot records `verb` alone, and the image, its byte size and width/height are never fielded. This is where the refusal this chunk must keep comes from (contract/refusal-cause/fonts-unavailable). Bundling fonts must not remove the cause or change its text.

@history_item n=2 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the `command` span's `cause` went from nine names to eight (`time-unavailable` retired), and the span kept its eight fields. `fonts-unavailable` stays one of the eight. The cause set is closed, and this chunk adds no cause (scope boundary).

@history_item n=3 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the eight cause names, `fonts-unavailable` named among them. Any change to that refusal's domain or text reopens a ratified contract and needs a fork.

@history_item n=4 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the cause names became the domain of the span's `cause` field. Standing rule: a cause is fielded by `Cause::name`, and no `Command`, `Call` or `ArgValue` is fielded with `Debug`. This applies if the font-unavailable path is touched (contract/command-span/cause).

@history_item n=5 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — code now returns all eight cause names, and the driver crate stays silent. This is the baseline the refusal-cause rows read from.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built; the closed span's line) — this entry built the `command` span: INFO at target `escher_driver`, eight fields recorded only where they apply, and never fielding the screen text, any argument or any Outcome. It also made the closed-span line a record class. These are the span rows this chunk must leave unchanged (no new field or span).

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line. A new record class or field in the sink is a boundary widening for the founder, so a font-path diagnostic cannot be added as a log line without one.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `tool_call` span (`tool`, `ending`), which the founder ratified. The tool_call rows in this chunk's picks rest on it. The chunk adds no tool or field.

@history_item n=9 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand and its in-process checks install no subscriber, read no env var and print nothing (decision/headless-stand-no-sink). The stand's boot, which this chunk changes to load the bundled DejaVu Sans, must stay silent with no env read.

@history_item n=10 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the first exception for a re-executed child that prints ids its parent reads into assertions only. This is the precedent for any cross-run or cross-OS comparison child the font proof might add.

@history_item n=11 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the silence census was widened to the checks' shared module `tests/blitz-tests/tests/common/mod.rs`. Any shared module this chunk edits (for example the font-context helpers) joins the census.

@history_item n=12 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the third shared module `tests/blitz-tests/tests/fixtures/mod.rs` (fixture boot, pages, `HeldNet`) was recorded silent: no print, tracing, subscriber, env read, file or socket. This chunk touches the shared fixture pages' font context and must keep that silence.

@history_item n=13 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence) — the served tasks (`boot`, `boot_timer`, `boot_cells`, `boot_load`, `options`) were recorded silent: no env read, print, tracing or listener in `stand.rs`. The driver session's served tasks are one path this chunk bundles fonts on. The same entry has replay children spawned without `--record` (decision/replay-children-uninstrumented).

@history_item n=14 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe's two replay children write stderr to `/dev/null`, uninstrumented by design. This is the origin of decision/replay-children-uninstrumented.

@history_item n=15 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` except: a re-run child prints "ids or digests" its parent reads. This is the sibling C3 entry (randomness and fonts depend on the host) right before this chunk, and the latest precedent for a cross-run comparison through a re-run child. A cross-OS readings comparison would reuse this exception and not widen it.

@history_item n=16 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — a second sink installer (`escher-session`). The headless no-sink claim is scoped to boots made in process, and a boot made by the binary runs under its sink (decision/headless-stand-no-sink). Trap: a zero read from a host with no sink says nothing about one that has a sink, so measure font-path changes under the binary too.

@history_item n=17 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink drops every target outside the engine/escher prefixes at every level (decision/scrub-sink-reach). Rule: a target is admitted only together with a scrub rule for it. Font-stack crates' (fontique, parley) log events would be dropped, and admitting them is a scrub decision.

@history_item n=18 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — each command's answer is one line of JSON on stdout, never a log line (contract/escher-session-stdout/command-answer). Standing rule: the next surface adds no span or log field that holds what a call supplied or a screen reads.

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span. The recording directory row holds call content at rest in a local `0600` file. The kept-check rows (arguments, accepted, digest) rest on it, and a cross-OS comparison through a kept check would read this carrier.

@history_item n=20 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check holds each answer whole (`answer`, a shot's without `png`), and replay adds no span. Committed checks `tests/blitz-tests/checks/counter-{cli,mcp}.json` are public stand-Counter sessions, uploaded by no CI leg (decision/committed-checks-hold-no-private-data, contract/recording-file/answer). This is the founder-ratified precedent if the cross-OS proof holds a committed reading that each OS replays. A committed reading must hold no typed text, host path or screenshot step.

@history_item n=21 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (silent and spanless; the gated `lint` verb) — `BaseDocument::text_extent` and the lint were recorded silent and spanless. The `lint` answer `{"findings":[…]}` is never logged or fielded, and its findings are never fielded (contract/command-answer/linted). Text extents and lint findings are among the readings this chunk compares across OSes.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a hosted shot; an open row's bounds; labels as pixels) — a hosted shot was compared equal to the in-process one on one host only. An open row's `bounds` crosses within its closed set (contract/snapshot-diff/open-list-row-bounds). Painting on every document, remote pages included, was an engine change the founder ratified as a boundary widening. This is the precedent for the scope's rule that a change in a remote page's font goes to the founder as an engine-change fork.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (where a `multiple` select's set leaves) — the snapshot's `value` of a multiple select is the first chosen option in tree order (contract/snapshot/multiple-select-value). This picked row is a reading the cross-OS comparison reads.

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's never-logged answer; the snapshot's clickable reading) — fixed the graph answer shape `{"nodes":[…],"states":[…]}` (contract/graph-answer/shape) and the closed `clickable` bool after `focused` (contract/snapshot-text/clickable, contract/snapshot-node/clickable). Both are picked rows that a readings comparison would read but must not change.

@history_item n=25 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler) — standing trap: a trait default can hide a fix, so sweep the implementers. The same applies here: `HarnessOptions::font_ctx` defaults to `None` and becomes the system collection. Every harness-built path that leaves the default must be swept.

@history_item n=26 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — a new upload out of fork CI (the `coverage-report` artifact) is a boundary widening recorded PROVISIONAL until the founder rules (decision/ci-platform-github-workflows). If the cross-OS proof ships a reading as a CI artifact, it falls under this rule.

@history_item n=27 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — kept: no upload step was added, because a new artifact out of fork CI is a boundary widening that waits for the founder. This constrains how the `test` leg's Linux/macOS/Windows readings may be compared.

@history_item n=28 marker=2026-10-12-self-description
**2026-10-12-self-description** — the help and remedy texts are the program's own fixed texts on stdout, never a log line, and they add no span field (contract/command-span/field-count). The scope keeps `fonts-unavailable`'s text unchanged under the same rule.

@history_item n=29 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — every area's master text became rows, verified against the code. Every picked row's handle dates from here, so an edit this chunk makes is a `~`/`+` on these handles.
