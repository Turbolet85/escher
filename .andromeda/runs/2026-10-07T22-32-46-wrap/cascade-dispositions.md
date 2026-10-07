# Cascade dispositions — 2026-10-07-driver-command-spans

Written from the sweep's listing after it ran (`cascade.py sweep`, `cascade v1.1`, baseline `b75ed30b`,
the parent of the one pre-CI commit; trail `cascade-2026-10-07-driver-command-spans.json`) and before any
sidecar entry of this pass. The sweep ran three times: once refused (below), once after the bodies were
amended, once after the leaves were re-derived. The master rows of the last two listings are the same
set; one offset moved (security-plan `:116`, the edit the sweep itself caused).

## What was searched

19 patterns in `cascade-patterns.toml`, each with a control that fired on the pre-pass masters, over
the seven masters, the 7 files under `.andromeda/registries/`, the three curation homes, the two
judgment bases and the leaf bodies:

`per-event` · `one-line-per` (the sink stated as one line per printed event) · `no-tracing-dep` ·
`two-deps-only` (the driver's dependency list) · `prints-logs` (the crate logs nothing) · `no-span`
(spans observed absent, the route entry owed) · `no-sink` (no subscriber or sink installed) ·
`typed-text` (typed text not measured) · `nine-act` · `fourteen` · `old-counts` (every count that
moved) · `reexec` (the re-executing test binaries) · `lock` (the lockfile byte-identical) · `ungated` ·
`features-off` (the engine `tracing` features stay off) · `diff-clause` (the returned-value-only
clauses) · `fmt-layer` (the layer and the event formatter) · `in-process` (typed into an instance where
no sink is installed) · `old-cites` (every pre-chunk citation range into the five edited files).

## What was not searched

- `PROVISIONAL` — the tool refused the pattern (exit 3): no pre-pass master holds the word, so its
  control cannot fire. Read by hand instead; the listing is the last section.
- Citations that name one of the five files without its full path.
- The amendment sidecars and their archives (never part of the cascade).
- Phrasings of the claims above that use none of the words in the patterns. The detectors read each
  doc for the claims however worded; the sweep is keyed on words.

## Rows

`amended` = this pass's own text or a line this pass edited where the hit is now true as it stands.
Rows on a line over 2 000 chars were read at their offset (`cascade.py window`), not from the row.

**old-cites** — 0 rows after the pass (control fired at architecture `:101`): no pre-chunk range into
`execute.rs`, the driver's `lib.rs` and `Cargo.toml`, `format.rs` or the telemetry `lib.rs` stands in a
master or a key file. 36 distinct ranges were re-pointed: 22 by locating the formerly cited block in the
current file (a script over `git show b75ed30b:{file}`), 14 by hand where the cited block was itself
edited (`impl Session` `59-142` → `81-179`; `Target`…`keyboard_key` `144-314` → `228-407`;
`settled_step` `257-267` → `342-359`; the driver's unit tests `316-346` → `409-472`; the scrub sets and
`decide` `21-107` → `22-111`; the formatter impl `123-155` → `134-178` and with its visitors `123-198` →
`134-286`; the sink's unit tests `200-376` → `288-598`; the crate doc `1-18` → `1-22`; the public items
`34-140` → `40-163`; `init`…`init_with_writer` `96-140` → `102-163`, with `sink_layer` now its first
part; `init_with_writer` `105-140` → `130-163`; the filter to the install event `119-138` → `144-161`;
the dependency table `13-15` → `13-16`). `sink_layer` is cited `102-116`: the report gives `102-117`,
line 117 is blank. Every citation now standing was printed with its first and last cited line and read.

**per-event** · **one-line-per**
- architecture `:132` ×2, test-plan `:102` ×2, obs-plan `:146`, a11y-plan `:87` ×2 — amended: each now states two record classes; the hit is the first class.
- architecture `:133`, security-plan `:76`, obs-plan `:106` ("one line per call"), security-plan `:379`, obs-plan `:11` — amended: this pass's own text.
- architecture `:136`, test-plan `:24`, `:26`, a11y-plan `:270` — no change: the snapshot text's one line per node.
- architecture `:140` (WPT expectations), layout-templates `:78` (the WPT runner's terminal mode), test-plan `:146` and obs-plan `:159` (the cold-agent stub's call log) — no change: other subjects.
- leaves — CLAUDE.md `:36`, `rules/observability.md:18`, `docs/tests-summary.md:17`, `docs/services/escher-telemetry.md:18-19`, `docs/obs-summary.md:49`, `docs/services/escher-driver.md:45`: re-derived. CLAUDE.md `:105`, `docs/a11y-summary.md:22`, `docs/services/dioxus-native-dom.md:18`, `docs/tests-summary.md:21`: no change, the snapshot text.

**no-tracing-dep**
- obs-plan `:70` @c2452 — no change: blitz-test-harness's settle loop, still true (the crate is unedited). The driver's two occurrences on that line were amended and no longer match.

**two-deps-only**
- a11y-plan `:14` — amended ("the library's third dependency, `tracing`"). Leaf `docs/services/escher-driver.md:11` — re-derived.

**prints-logs**
- architecture `:130` (the harness's settle), `:136` @c20685 and security-plan `:116` @c4500 (`unkeyed_actionable`), test-plan `:88` (the harness) — no change: other subjects, each "logs nothing" still true.
- obs-plan `:70` ×2 — no change, read at both offsets: "nothing in the crate prints, logs or fields any of the four" (the four `Debug` types) and "nothing prints, logs or fields it either" (the record of ids) both hold — the span fields neither.
- leaves `docs/services/blitz-test-harness.md:20`, `docs/services/dioxus-native-dom.md:16` — no change: the same other subjects.

**no-span**
- architecture `:111`, `:133`, security-plan `:76` — amended: this pass's own "no `#[instrument]`".
- obs-plan `:95`-`:105` (ten rows) — no change: slice-scoped searches recorded at adoption ("over the 15 s05 files", "the 61 slice files", "the 12 listed files"). Each states what a search over a named file set found then; none names `packages/escher-driver` or `packages/escher-telemetry`, and the span this chunk built is stated on `:106`.
- obs-plan `:106` ×2 — amended: the bullet is now the span as built; "still observed absent" is kept for `Harness::settle` and the lifecycle.
- leaves `rules/observability.md:24`, `docs/conventions.md:26`, `docs/obs-summary.md:49`, `docs/services/escher-driver.md:45` — re-derived.

**no-sink**
- architecture `:101`, `:133`, `:193`, security-plan `:76`, obs-plan `:106` — amended: "the crate installs no subscriber", this pass's own text, true.
- architecture `:263` @c3770, test-plan `:26` @c7782, `:181` @c362, key file `session-lifecycle.md:10` — no change: `stand_session_quiet`'s host installs no log sink; the check is unedited.
- obs-plan `:70` ×5 — read at each offset: the stand's census (amended, with its exception), the private modules (amended), `stand_session_quiet`'s child @c3187 and seven_guis' shared test module @c3799 (no change, true).
- leaves `rules/observability.md:18`, `:24`, `docs/obs-summary.md:12`, `:49`, `docs/services/escher-driver.md:45`, `docs/services/escher-telemetry.md:32`, CLAUDE.md `:37` — re-derived. `docs/services/escher-driver.md:55`, `docs/tests-summary.md:24` @c2532 — no change: `stand_session_quiet`.

**typed-text** · **in-process**
- security-plan `:117`, test-plan `:102`, `:181` — amended: "not measured" now reads of a host's log only, beside the in-process reading.
- `in-process`: 0 master rows after the pass; one leaf row, `docs/obs-summary.md:49`, re-derived ("held in process only, where a sink capture of it reads 0 occurrences").
- leaves CLAUDE.md `:47`, `docs/security-summary.md:29` — re-derived. `rules/observability.md`, `docs/obs-summary.md:43`, `docs/services/escher-telemetry.md` — re-derived and no longer matching.

**nine-act** · **fourteen** · **old-counts**
- test-plan `:115`, `:325`, obs-plan `:70` @c4787 ("the six `stand_act_*`"), key file `session-lifecycle.md:11` @c1503 ("the nine driver-action files and the crate's 27 at …refusal-detection"), test-plan `:100` ("the fourteen readers at …refusal-detection"), test-plan `:115` ×2 and `:325` ×2 (the earlier re-counts) — no change: history, each tied to the chunk that measured it; this chunk's re-count is appended beside it.
- test-plan `:146` — no change: the cold-agent contract tests' 27.
- leaves `docs/tests-summary.md:30` ×2 — no change: the same history. `rules/testing.md:15`, `docs/obs-summary.md:49`, `docs/services/escher-driver.md:26`, `:53`, `docs/services/escher-telemetry.md:39`, `docs/tests-summary.md:22`, `:24` — re-derived and no longer matching.

**reexec**
- architecture `:150` ×2 — amended (four binaries). obs-plan `:70` @c879 — no change: `stand_id_persistence`'s child.
- leaf `docs/services/escher-telemetry.md:41` — no change: the two `telemetry_*` binaries re-execute once each.

**lock**
- architecture `:100` — no change: a byte-identical match in the reftest harness. The driver's "`Cargo.lock` is byte-identical" (architecture `:101`) and "the lockfile is byte-identical" (a11y-plan `:14`) were amended and no longer match.
- curation `docs/session-learnings.md:55`, base `playbook.md:34` — no change: other subjects (an expression; a probe signature).

**ungated**
- architecture `:111` ×3, `:193`, `:226`, obs-plan `:30` ×2, `:70`, a11y-plan `:14` — amended.
- architecture `:154` ×2 — no change: "a library with no features", true of both escher crates.
- leaves CLAUDE.md `:46`, `docs/conventions.md:26`, `docs/stack.md:36` — re-derived. `docs/services/escher-telemetry.md:11`, `:24` — no change, true.

**features-off**
- obs-plan `:30`, `:299` — amended: narrowed to a package-alone build. Leaf `docs/services/seven_guis.md:13` — re-derived.

**diff-clause**
- architecture `:136` @c16058 and @c16092, security-plan `:116` @c3645, `:118` @c1423 — no change: the snapshot TEXT's clause; no text and no length of it is fielded.
- architecture `:136` @c18834 — amended (the escalated qualification, PROVISIONAL).
- security-plan `:116` @c5569 — **found by this sweep**: the same founder-ratified diff clause, stated a second time. Amended as architecture's, on the same word.
- security-plan `:116` @c4934 (`unkeyed_actionable` adds no crossing), @c8686 (the record of ids, written to no log) — no change, true.
- security-plan `:118` — no change beyond the above: its "the diff adds no input class and no crossing" speaks of a diff entry's values and points at the `id` row, which now carries the qualification.
- leaves `docs/security-summary.md:29`, `docs/services/dioxus-native-dom.md:19` — re-derived with the hedge. `docs/services/dioxus-native-dom.md:18` — no change: the text's clause.

**fmt-layer**
- architecture `:193`, `:256` — amended.

## Leaves re-derived (step 3)

By provenance and by the table: CLAUDE.md (modules: escher-telemetry and escher-driver; warnings: the
`tracing` gate and the sink; the pointer table's telemetry and executor rows; the architecture
paragraph) · `rules/observability.md` · `rules/testing.md` · `rules/security.md` (one sentence, the
span's fields) · `docs/obs-summary.md` · `docs/tests-summary.md` · `docs/security-summary.md` ·
`docs/conventions.md` · `docs/stack.md` · `docs/services/{escher-driver,escher-telemetry,seven_guis,dioxus-native-dom}.md`.
Read and left: `docs/a11y-summary.md` (it does not state the sink's line format), `docs/commands.md`,
`docs/gotchas.md`, `docs/design-summary.md`, `rules/a11y.md`, `rules/verification-harness.md`,
`docs/services/blitz-test-harness.md` (no statement of the sink, the span or a moved count).
No `USER:*` block and no `## Session Additions` section was edited.

## Binds

- test-plan §3 ↔ obs-plan §3 (the log format): both state the two record classes with the same line
  shape; obs-plan §6 owns the PROVISIONAL mark and test-plan §3 points at it.
- a11y-plan schema ↔ obs-plan schema: no a11y violation schema exists; a11y-plan §3's note restates the
  format as obs-plan §6 does and points at the mark.

## PROVISIONAL marks standing after this pass (read by hand: `grep -n -o PROVISIONAL`)

Before the pass: none in any master, key file, leaf or route line (the sidecars hold the word in entries
whose marks were ratified at earlier boundaries). After it, two items:

1. **The sink prints a closed span — a second record class** (a boundary widening on the operator's
   answer at the chunk's plan forks). Bodies: architecture `:132`, `:193` · security-plan `:379` ·
   obs-plan `:147` (the owning statement), `:298` · test-plan `:102` · a11y-plan `:87` (the last two by
   pointer). Leaves: CLAUDE.md (the escher-telemetry module bullet; the architecture paragraph) ·
   `rules/observability.md` · `docs/obs-summary.md` · `docs/tests-summary.md` ·
   `docs/services/escher-telemetry.md`.
2. **The diff's three list lengths reach a log, stated beside the founder-ratified returned-value-only
   sentence.** Bodies: architecture `:136` · security-plan `:116`. Leaves: `docs/security-summary.md` ·
   `docs/services/dioxus-native-dom.md`.

Neither is discharged here (the operator, 2026-10-07, at this wrap's invocation and at its P2 halt):
both wait for the founder's batch at the Epoch 4 boundary and are named in the handoff.
