# Report — 2026-10-20-large-screen-in-parts

**Chunk:** Large screen in parts — a screen over the snapshot text budget read in parts an agent can walk, on a fixture
**Date:** 2026-10-20T11:17:18Z
**Commits:** `191e66fdb chore(2026-10-20-large-screen-in-parts): operator pre-CI commit, for the run this chunk's verdict reads`
(the one commit since `state.yaml.last_wrap` 2026-10-20T08:56:50Z; `git log --format='%h %s' c1a2e5b85..HEAD`)

## Changes (structured — detectors read this)
- **Files:** 19 code and test files, from the pre-CI commit's parent `c1a2e5b85` to the tree (`gate.py scope`: `changed 19 · listed 19 ·
  recorded 0`). 18 modified · 1 new.
  - dioxus-native-dom: `packages/dioxus-native-dom/src/snapshot_text.rs` · `packages/dioxus-native-dom/src/lib.rs`.
  - escher-driver: `packages/escher-driver/src/{schema/spec,schema,schema_tests,command,lib,execute,json,mcp/listing,mcp_tests,cli,cli_tests}.rs`.
  - seven_guis tests: `examples/seven_guis/tests/{cli_commands,host_log,mcp_log}.rs`.
  - blitz-tests: `tests/blitz-tests/tests/{session_common/mod,stand_act_spans}.rs`, and the new `tests/blitz-tests/tests/stand_snapshot_parts.rs`.
  - Pipeline files: `escher-0.2.0/verification-matrix.json` (v020-09 → implemented), the chunk folder and its `evidence/ci.md`, the
    phase, implement and wrap run dirs, and `.andromeda/friction-log.ndjson`.
  - `packages/escher-driver/src/mcp.rs` is unchanged: a part reaches its generic arm (`mcp.rs:234`).
- **Symbols / APIs:** coordinates are from `## New text, by line` below.
  - **dioxus-native-dom** (`packages/dioxus-native-dom/src/snapshot_text.rs`):
    - `pub struct TextPart { pub text: String, pub ids: Vec<String> }` (`Debug, Clone, Default, PartialEq, Eq`; 19-28). It is
      re-exported from the crate root under `accessibility` as `pub use snapshot_text::{SNAPSHOT_TEXT_BUDGET, TextPart}`
      (`lib.rs:39`), and the crate doc's feature list names `Snapshot::text_parts` (`lib.rs:8-9`).
    - `pub fn Snapshot::text_parts(&self, budget: usize) -> Vec<TextPart>` (59-81). It takes consecutive pre-order lines
      of `to_text` while a part stays within `budget` bytes, and never splits a line: a line longer than `budget` is a part
      alone. An empty snapshot is one empty part. It has no document access, failure path, log or print.
    - The parts concatenated are `to_text()`. `to_text` and `text_parts` share a private `each_line` pre-order walk with
      depth, and `to_text`'s output is unchanged.
    - `SNAPSHOT_TEXT_BUDGET`'s doc now names `text_parts` as the reader that cuts the text into parts within it (15-16),
      while `to_text` "never truncates or pages" stands. Six unit tests: 493-609.
  - **escher-driver**:
    - `ArgKind::Part` (`schema/spec.rs:19-21`): name `part`, bound "a whole number from 1 to 1000"; the seventh argument kind.
    - `pub const MAX_PART: u32 = 1000` (`schema.rs:17-19`), re-exported from the crate root (`lib.rs:156-157`).
    - `SNAPSHOT`:
      - one optional argument, `part` (`ArgKind::Part`, `required: false`), help "the part of the screen to read, counted
        from 1; a screen over the budget is read in parts without it from part 1";
      - its fields are now `text`, then `part` and `parts` (`FieldKind::Count`, `always: false`; helps "the part this answer holds, present when the
        screen is read in parts" and "how many parts the screen reads as, present with `part`");
      - `SNAPSHOT.help` and `text`'s help are rewritten (`schema.rs:178-216`) to state the five paging rules.
    - `Command::Snapshot` is now `Snapshot { part: Option<u32> }`.
      - `validate` admits `snapshot` with no argument or with `Admitted::Part(n)`, `n` in `1..=MAX_PART` through `u32::try_from`.
      - Anything else of kind Number is `OutOfBound("part")`; a text or flag value is `WrongKind("part")`.
      - `Command::spec` is updated (`command.rs:139-144`, `:252-253`, `:362-366`).
    - `Outcome::Part { text: String, part: u32, parts: u32 }` (`execute.rs:32-41`).
      - The `Command::Snapshot { part }` arm reads the screen once through `read_screen`, which records every id of the whole
        screen (`execute.rs:188-204`).
      - With `part` `None` and `to_text()` at most `SNAPSHOT_TEXT_BUDGET` bytes, it answers `Outcome::Screen { text }` as before.
      - Otherwise it cuts `text_parts(SNAPSHOT_TEXT_BUDGET)`, takes `n` = `part` or 1 and `parts` = the count
        (`u32::try_from`, saturating), and answers `Outcome::Part` with part `n`'s text, or the empty text when `n` > `parts`.
      - `record_result` puts `Outcome::Part` in the arm that records `verb` alone (`execute.rs:308-313`).
      - `Session::run`'s doc gains the part read (`execute.rs:133-138`).
    - `Outcome::to_json` writes `Outcome::Part` as `{"text":…,"part":n,"parts":m}` (`json.rs:330-336`).
    - The MCP listing's `tool` gives `ArgKind::Part` `{"type":"integer","minimum":1,"maximum":1000}` plus its description
      (`mcp/listing.rs:85-89`).
    - The command line reads a `Part` flag's value as a number, as `Milliseconds` (`cli.rs:253-257`). The usage line now
      spells `snapshot [--part <part>]`, built from the table.
    - Callers of the changed surfaces:
      - `Outcome` is a public enum matched exhaustively at `execute.rs`'s `record_result` and `json.rs`'s `to_json` (both
        updated), and outside the crate by `session_common::act` and `stand_act_spans::acting` (both updated).
      - Every other match is a `{ text }` or a wildcard pattern, which stands: the 14 `Outcome::Screen` patterns
        (research.md §Sweeps) are unchanged.
      - `Command::Snapshot` is matched only inside escher-driver.
  - **Test helpers** (`tests/blitz-tests/tests/session_common/mod.rs`):
    - `pub fn snapshot_part(n: i64) -> Call` (68-71);
    - `pub fn part(outcome: Outcome) -> (String, u32, u32)` (73-80), which panics with the fixed message "the outcome is a part"
      on any other outcome;
    - `act`'s outcome match gains `Outcome::Part` (213-216).
  - **No new verb, cause, exit, carrier, wire form or span field.** `VERBS` 12, `CAUSES` 8, `NODE_FIELDS` 13, wire `v3`,
    and describe's count row `[12, 13, 12, 3, 8, 4, 15]` are unmoved (escher-driver unit tests green).
- **Crates / modules:** changed — dioxus-native-dom (`snapshot_text`), escher-driver (`schema`, `schema::spec`, `command`,
  `execute`, `json`, `mcp::listing`, `cli`). No crate added or removed.
- **Dependencies:** none. Gate entry 10 checks that no manifest, lock, stand source, shared fixture, stand table or
  committed check changed: `git diff --name-only c1a2e5b85… -- Cargo.toml Cargo.lock … | wc -l` → 0.
- **Schema / config:** the driver's command schema gains:
  - the argument kind `part`, a whole number 1 to 1000 (`MAX_PART`);
  - `snapshot`'s optional `part` argument;
  - two `always: false` result fields, `part` and `parts` (`Count`).

  Paged answer line: `{"text":…,"part":n,"parts":m}`. A whole screen's line is still `{"text":…}` alone. No config key, env var,
  scrub or redaction shape changed.

  The `command` span records `verb` alone for a part read (`stand_act_spans`), with no field for a part's number,
  count, size or text, so escher-driver keeps 2 `info_span!` sites (gate entry 9 → 2).
- **Spec-master edits:** none (the masters equal HEAD; the pre-pass sweep's baseline check passed).
- **Retired:** none.
- **Counts / qualifiers moved:** each fact names its pattern (`{run_dir}/report-patterns.toml`), with its sites from
  `cascade.py sweep --pre-pass` (trail `.andromeda/runs/2026-10-20T11-11-53-wrap/cascade-2026-10-20-large-screen-in-parts-pre-pass.json`),
  fired in two calls.
  - **Argument kinds: six → seven** (`ArgKind::Part`, 1 to `MAX_PART` = 1000) — `kinds`: 5 standing.
    - Changed:
      - architecture.md:196 @c11617, the `ArgKind` list;
      - security-plan.md:83 @c2129 «six argument kinds, each with its bound»;
      - test-plan.md:146 @c3029 «the six argument kinds with their bounds» and @c3079 «the sixth, `ArgKind::Name`».
    - No-change:
      - architecture.md:196 @c9718 (`screenshot`'s `ArgKind::Id`);
      - architecture.md:196 @c12615 (a dated history, «no … argument kind … added»);
      - architecture.md:354 («six shape types `ArgKind`, `ArgSpec`, …» counts types, not kinds);
      - security-plan.md:78 @c3768 (a dated chunk's «no … argument kind … added»).
  - **`snapshot` takes an optional `part`.** It took no argument; the result fields go `text` → `text` · `part` · `parts` —
    `snaparg`: 1 standing, architecture.md:196 @c9344 «`snapshot` (no argument; result `text`)», changed.
  - **The snapshot text gains a parts reader** (`Snapshot::text_parts`; `to_text` still never pages) — `budget`: 5 standing.
    - Changed:
      - architecture.md:209 @c21207 («no truncation or paging» holds of `to_text`, and the parts reader is to be added);
      - architecture.md:351 (dioxus-native-dom's module row, «its `SNAPSHOT_TEXT_BUDGET` constant re-exported», gains `TextPart`).
    - No-change:
      - architecture.md:359 (`stand_requirement_sweep`'s screens inside the budget);
      - test-plan.md:27 @c7849 and @c46110 (stand screens inside the budget, still true);
      - test-plan.md:204 @c1070 (the four tasks' snapshot lines inside the budget, still true).
    - `parts` (text_parts · TextPart · Outcome::Part · «never truncates or pages» · «read in parts»): 0 — its control's
      refusal: `control: title, parts, unitcount, limit never fired over the seven masters' pre-pass text (191e66fd)`.
  - **The MCP `snapshot` result is now two-formed.** A whole screen is still the bare `to_text`; a paged one is the JSON line
    `{"text","part","parts"}` (mcp.rs's generic arm) — `mcpsnap`: 1 standing, security-plan.md:129 @c4031 «a `snapshot`
    result is `to_text`'s output», changed (a paged result is its JSON line, holding the same text).
  - **The usage line's first segment** `snapshot` → `snapshot [--part <part>]` — `usage`: 15 standing.
    - Changed:
      - layout-templates.md:74 (the cli surface's usage line and answers);
      - registries/contracts/test-plan/session-lifecycle.md:9 (the usage line the session-binary contract states).
    - No-change, 13:
      - generic «usage line» mentions: architecture.md:196 @c31562, :222; security-plan.md:76, :84 ×2, :411;
        layout-templates.md:67, :71 (`paint_bench`); test-plan.md:10, :22, :146 @c8694, :204 @c3405; obs-plan.md:70 @c2143;
      - obs-plan.md:190 ×2 (the `help` and `mcp` segments, which stand).
    - `ltsnap` (verb lists naming `snapshot`, `click`): 9 standing, all no-change — the verb set and the flows are unchanged.
  - **`run stand`: 47 files · 192 passed · 0 · 7 → 48 files · 195 passed · 0 failed · 7 ignored** (implement gate entry 13,
    `run.end` 2026-10-20T10:42:23Z; files counted from its `run.start` list) — `standrun`: 4 standing, all changed:
    test-plan.md:119 @c10904/@c11286/@c11325, test-plan.md:132, test-plan.md:324 @c1299 («47 files»), test-plan.md:353 («the 47 `stand_*.rs` files»).
  - **`stand_act_spans` calls per mode: 34 → 35** (one `snapshot` with `part` 1 per mode, asserted `run.calls == 35`) —
    `spancalls`: 2 standing, both changed: test-plan.md:27 @c31178, obs-plan.md:311 @c2334.
  - **seven_guis `cli_commands`: 13 → 14** (`snapshot_reads_a_part_by_number_and_a_part_outside_its_bound_is_refused`;
    gate entry 5's log) — `clicount`: 5 standing.
    - Changed: test-plan.md:146 @c9655 «`cli_commands` (13)», test-plan.md:204 @c193 «`cli_commands` (13)»,
      registries/contracts/test-plan/session-lifecycle.md:11 @c2511.
    - No-change: test-plan.md:334 @c926 and :340 @c15417/@c16765 (dated «+13 passed» deltas).
  - **Workspace tally: lines 204 · passed 1087 · 0 · 12 → lines 205 · passed 1098 · failed 0 · ignored 12** (gate entry 16
    over `target/ci-logs/test.log`, fresh from entry 15's `fast`; +11 = dioxus-native-dom +6, escher-driver +1,
    `stand_snapshot_parts` +3 and +1 line, `cli_commands` +1) — `tally`: 2 standing, both changed: test-plan.md:334 @c5524, test-plan.md:340 @c19974.
  - **Per-OS CI tallies, CI#39387656399 → CI#39410865604** (`evidence/ci.md`) — `peros`: 2 standing, both changed:
    test-plan.md:334 @c5564/@c5593/@c5668, test-plan.md:353.
    - Test [default features] 204 · 1087 → 205 · 1098; macos and coverage 185 · 1084 → 186 · 1095; windows 185 · 1041 → 186 · 1051
      (+10: `cli_commands` is unix-only).
    - The a11y job is unmoved at 6 · 43; the stand repeat goes 165 · 672 → 168 · 684 · 0 · 21 (`stand_snapshot_parts` 0 → 9
      and `cli_commands` 39 → 42, by a per-target diff of the two job logs).
  - **Shared-module readers.** `tests/session_common/mod.rs` readers go 28 → 29, and `tests/fixtures/mod.rs` readers 23 → 24
    (`stand_*` 21 → 22), since `stand_snapshot_parts` declares both — `readers`: 4 standing, all changed:
    architecture.md:368, test-plan.md:68 @c2549, :103 @c3021, :104 ×2.
    - The call builders' list gains `snapshot_part` and the part reader `part`.
  - **`host_log` and `mcp_log` each drive one part read.** `host_log` sends seven calls, not six, and reads 7 command-span
    lines; `mcp_log` sends ten tool calls, not nine, with 8 `command` lines. Each file's test count is unchanged at 2 —
    `logflows`: 30 standing.
    - Changed, 2: test-plan.md:187 @c2326 («each also drives one `lint`») and security-plan.md:410 @c7450/@c7465 («each driving one `lint` with 0 needles»), to add the part read.
    - No-change, 28: file citations, «under both builds» claims and per-file test counts, which still hold.
  - **dioxus-native-dom lib tests 75 → 81; escher-driver unit tests 99 → 100** (gate entries 1 and 2) — `unitcount`: 0, its
    control's refusal (the line above).
  - **escher-driver's span unit test pins the table's argument count: 9 → 10** — no master site claimed. An inline unit literal
    (`execute.rs:600`), not swept.
  - **a11y-plan §7 gains one more reader of the snapshot model, the parts**, each line equal to the whole text's —
    `a11y7`: 1 standing, a11y-plan.md:282 (`## 7. Screen Reader Support`), changed (the section to add it).
  - **The cli surface's answers gain the paged line** `{"text","part","parts"}` — `dsjson`: 14 standing.
    - Changed: design-system.md:398 (the cli component patterns, @c137/@c1025/@c1369/@c2525, the list of one-line answers)
      and layout-templates.md:74 (@c718/@c1008/@c1715/@c2151/@c3756, the cli surface's answer list).
    - No-change, 12: architecture.md:196 ×3, :209, :219, :307; security-plan.md:84 ×2, :125, :411; test-plan.md:106;
      obs-plan.md:69, :70 ×2, :190 ×2; session-lifecycle.md:9. Each states the one-line rule, which holds.
  - **The stated limit: an acting verb's diff and a lint's findings are not paged**, held only by the 1,048,576-byte answer bound —
    `limit`: 0, its control's refusal (the line above). No master states it yet; its owner is the P5 card's row 1 (Cross-project
    bullet), a `CARRY:` on «Capability statement».
  - **Owner patterns:**
    - `title` («Large screen in parts»): the owner pattern's control is unfired (the refusal line above).
    - `owner` («owned by a (route entry|CARRY)»): 2 standing, both no-change. architecture.md:90 is the upstream-sync CARRY and
      security-plan.md:125 @c9272 a dated id-exit route entry; neither is this chunk's.
  - **Long lines** (over 2 000 chars): one disposition per (line, fact) pair, at the offsets the rows and the `by line` blocks
    print.
    - architecture.md:196: kinds changed @c11617 · no-change @c9718/@c12615; snaparg changed; usage no-change; dsjson no-change @c3173/@c14863/@c41763.
    - architecture.md:222: usage no-change · logflows no-change @c3649/@c3689.
    - architecture.md:230: logflows no-change @c3975/@c4454.
    - architecture.md:239: logflows no-change @c2019/@c2465/@c2646/@c7363/@c8008/@c9042/@c9084/@c9223.
    - security-plan.md:84: usage no-change @c4118/@c6116 · dsjson no-change @c1717/@c3280.
    - security-plan.md:126: logflows no-change @c3217/@c5978/@c6014.
    - security-plan.md:410: logflows changed @c7450/@c7465 · no-change @c4801/@c4971/@c6539/@c6575/@c7786/@c7798/@c7834; ltsnap no-change @c5038.
    - layout-templates.md:74: usage changed @c3060 · no-change @c3364; dsjson changed (the answer list).
    - test-plan.md:10: usage no-change; logflows no-change, all 13 offsets; ltsnap no-change @c2752/@c4837.
    - test-plan.md:27: budget no-change @c7849/@c46110 · spancalls changed @c31178.
    - test-plan.md:50: logflows no-change @c726/@c3301.
    - test-plan.md:68: readers changed @c2549 · logflows no-change @c8508/@c8654.
    - test-plan.md:106: logflows no-change, all 9 offsets; dsjson no-change; ltsnap no-change.
    - test-plan.md:119: standrun changed @c10904/@c11286/@c11325.
    - test-plan.md:146: kinds changed @c3029/@c3079 · usage no-change @c8694 · clicount changed @c9655 · logflows no-change @c9713.
    - test-plan.md:187: logflows changed @c2326 · no-change at its other 12 offsets.
    - test-plan.md:205: logflows no-change @c942/@c2162 · ltsnap no-change @c288.
    - test-plan.md:324: standrun changed @c1299 · logflows no-change @c1746/@c1786.
    - test-plan.md:333: logflows no-change @c1743/@c2092/@c2107 · ltsnap no-change @c1520.
    - test-plan.md:334: clicount no-change @c926 · tally changed @c5524 · peros changed @c5564/@c5593/@c5668.
    - test-plan.md:340: clicount no-change @c15417/@c16765 · tally changed @c19974 · logflows no-change @c5945/@c12326.
    - obs-plan.md:70: usage no-change · logflows no-change, all 9 offsets · dsjson no-change.
    - obs-plan.md:190: usage no-change · dsjson no-change.
    - obs-plan.md:311: spancalls changed @c2334 · logflows no-change @c2591/@c2603/@c2959 · ltsnap no-change @c3032.
    - session-lifecycle.md:9: usage changed @c1859 · no-change @c2362; dsjson no-change.
    - session-lifecycle.md:11: clicount changed @c2511 · logflows no-change @c2205/@c2620/@c7746/@c8503/@c10201.
    - design-system.md:398: dsjson changed (@c137/@c1025/@c1369/@c2525, one list).
  - **Sweep reach:** 16 patterns (two `total` lines: 14 + 2) · 17 moved facts, 16 with a pattern (the report's own count) ·
    owner pattern control unfired · by line 26 lines · 85 offsets, and 7 lines · 13 offsets on the second call.
- **Dev-tool versions:** none.
- **Harness / gate surface:**
  - A new blitz-tests target, `stand_snapshot_parts`. It joins `run stand` and `ci-leg.sh stand-repeat` by its `stand_`
    prefix, with no `[[test]]` table.
  - No agent-run script, CI step, leg or status shape changed.
- **Cross-project / external claims:**
  - **CI#39410865604** on `191e66fdbe7f` (the pre-CI commit): `verdict: green · checks 17/17 · wall 1105 s`
    (`ci.py conclusion`, `evidence/ci.md`). The tree this wrap commits adds this report, the wrap run dir and `evidence/ci.md`
    to that sha, and no code.
  - **Inputs** (`inputs.py verify`: `inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited 1 · unparsed 0`):
    - I1 · message: the operator, with /andromeda-phase, 2026-10-20T09:07Z · copy · n/a. The fork routing: a boundary or direction
      fork goes to the operator by dialog, marked as the founder's. Cited scope.md:15, :29 · research.md:9, :130 · plan.md:390.
    - I2 · message: the founder, by question dialog in the overseer session, relayed by the overseer, 2026-10-20 · copy · n/a.
      F1 «Аргумент part, части по умолчанию (Рекомендую)» · F2 «Пустая часть (Рекомендую)». Cited scope.md:54 · plan.md:12, :389, :399, :407, :408, :438.
    - I3 · message: the operator, in this session, at P5, 2026-10-20 · copy · n/a — `UNCITED` by `verify`. Cited here:
      **inputs#I3** is the operator's word «yes» approving the review card
      `.andromeda/runs/2026-10-20T09-07-29-phase/p5-card-2.md`, which governs. No other entry names a card; card 1
      (`p5-card-1.md`) is the earlier round the card-2 heading names as folded. Its `## Beyond the plan` row, verbatim:

      | # | Proposal the word approves that plan.md and the ledger writes will not hold | Owner / place |
      |---|---|---|
      | 1 | The stated limit that an acting verb's diff and a lint's findings are not paged: on a large screen either can pass 10,000 bytes, held only by the 1,048,576-byte answer bound | a `CARRY:` on the existing entry «Capability statement», written by the wrap's route-resolve |

      Authority: the operator's own word (inputs#I3), on a row inside the chunk's ruled scope. It is P5's to land.
- **Reverted / negative API facts:** none.
  - The MCP check first walked two calls, a no-argument `snapshot` and part 2 (plan step 12.7). It was extended in place to
    the full walk; nothing shipped and was removed (see Deviations).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none.
  - architecture.md:209's «no truncation or paging» stays true of `to_text`. The reader beside it is new (an amendment above),
    not a disproof.
- **Expected amendments (from plan):** sites from the two pre-pass sweeps above.
  - **architecture §Standard Contracts → Dioxus DOM bridge** (`Snapshot::text_parts`, `TextPart`): carried (Symbols / APIs). Search:
    `budget` 3 architecture rows (:209 changed · :351 changed · :359 no-change) · `parts` 0 (control unfired).
  - **architecture §Standard Contracts → Driver session** (the optional `part`, `ArgKind::Part` the seventh kind, `MAX_PART` 1000,
    `Outcome::Part`, default paging over the budget, the empty part past the last, a part read feeding the record of ids as
    `snapshot` does): carried (Symbols / APIs · Schema / config). Search: `kinds` 3 architecture rows (:196 · :354) ·
    `snaparg` 1 (:196).
    - Playbook «Boundary widening», answered by the founder's own word (inputs#I2), so P2 halts on it once, as the answer card.
  - **security-plan §Input Validation, Driver command schema row** (the new bounded argument kind on `snapshot` and the paged
    answer on every carrier): carried. Search: `kinds` 2 security-plan rows (:78 no-change · :83 changed) · `mcpsnap` 1 (:129
    changed) · `logflows` :410 changed. The same «Boundary widening» pattern and word (inputs#I2).
  - **test-plan §3 → Session lifecycle** (`session-binary`, `session-proof`: `snapshot [--part <part>]`, the new check, the
    counts): carried (Counts / qualifiers moved). Search:
    - test-plan rows: `usage` 4 · `clicount` 4 · `standrun` 4 · `spancalls` 1 · `tally` 2 · `peros` 2 · `readers` 3;
    - key file `registries/contracts/test-plan/session-lifecycle.md`: `usage` 2 rows on :9 · `clicount` 1 (:11).
  - **obs-plan §4** (a part read records `verb` alone): carried (Schema / config). Search: `spancalls` 1 obs-plan row (:311) ·
    `logflows` obs-plan rows :70, :76, :77, :78, :121, :195, :311, :346, all no-change for this fact. The §4 statement of
    the `command` span's fields per verb stands at :311's span list; the amendment adds the part read beside `snapshot`.
  - **design-system §Surface: cli §Component Patterns** (a paged `snapshot` answers one line `{"text","part","parts"}`): carried.
    Search: `dsjson` 1 design-system row (:398).
  - **layout-templates §Surface: cli → Primary screens** (the usage line's `snapshot [--part <part>]`): carried. Search: `usage`
    3 layout-templates rows (:67 · :71 `paint_bench` no-change · :74 changed) · `dsjson` :74.
  - **a11y-plan §7 Screen Reader Support** (the parts are one more reader of the snapshot model, each line equal to the whole
    text's): carried. Search: `a11y7` 1 a11y-plan row (:282).
- **Coverage of new surfaces:**
  - `snapshot`'s `part` argument (socket `call`, command line `--part`, MCP `{"part":n}`, a kept check's `arguments`) →
    - validation ✓ (`validate`: `ArgKind::Part`, `1..=MAX_PART` via `u32::try_from`; 0, 1001, −1, `i64::MAX`, text and flag
      refused `malformed` with nothing run; the command line's refusal JSON is the library's own `Refusal::malformed(fault)`,
      holding nothing typed);
    - instrumentation ✓ (`command` span `verb` alone; `tool_call` `tool` · `ending`);
    - PII n/a (a number) · tests unit + integ ✓ · a11y n/a · tokens n/a.
  - `Outcome::Part`, the paged answer `{"text","part","parts"}` on every carrier →
    - validation n/a (an answer) · instrumentation ✓ (no field added; `host_log` and `mcp_log` 0 id and 0 name needles at
      `trace` with a part read driven);
    - PII redacted ✓ (a password's value reads `MASKED_VALUE` in every part, the typed sentinel 0 times across a walk —
      `stand_snapshot_parts`);
    - tests integ ✓ (both layout modes, in process and `serve_mcp`) · a11y ✓ (each part's lines equal the whole text's;
      one `focused` line after a click) · tokens n/a.
  - `Snapshot::text_parts` / `TextPart` (an in-process reader) → validation n/a (pure, infallible) · instrumentation n/a ·
    PII n/a (it carries what `to_text` carries) · tests unit ✓ (6) · a11y n/a · tokens n/a.

## Deviations from intent
- **The MCP check walks the whole screen** (plan step 12.7 named two calls). Step 12.7 asked for a `tools/call` `snapshot`
  with no argument and with `{"part":2}`. `verification-matrix.json#v020-09`'s acceptance, as concretized at P5 (the card's
  preview, approved by inputs#I3), reads «the same walk through serve_mcp returns the same JSON lines».
  - `over_mcp_the_walk_answers_the_json_lines_of_the_same_calls` therefore sends `snapshot` with no argument, then parts 1 to
    `parts`, then `parts + 1`. Each text block is equal to the JSON line `Session::run` writes for the same call on a session
    of the same fixture.
  - Justification: the acceptance is the ledger's contract, and the step was its subset. It cost one more full gate run,
    green 17/17.
- **The fixture check's shape.**
  - Plan step 12's eight assertions are grouped into three tests:
    - `a_screen_over_the_budget_is_walked_in_parts`: condition, default paging, walk, end, addressable, masked;
    - `over_mcp_…`: MCP;
    - `a_screen_inside_the_budget_reads_whole_as_before`: inside the budget.
  - Every assertion message carries a mode and an index only.
  - The page uses `display: block` buttons, so `item-400` lies below the 800 × 600 viewport and is read only in the last
    part (asserted). The click count is an `output` `clicks`.
  - The secret input is scrolled to before `type`, since it also lies below the viewport.
- **The plan's touchpoint line and step 7 disagreed.** The Codebase touchpoints line for execute.rs says «the record fed from
  the part»; step 7 and Provenance say the record takes the whole screen's ids. Step 7 was followed (`read_screen`).
- **The span unit test's argument-count pin moved 9 → 10** (`execute.rs:600`), a listed file. The new `part` is the tenth
  argument name in `VERBS`.
- **The plan named `part 1000` as admitted.** It is admitted at `validate` (command.rs unit rows), not on a screen. A flag
  value is refused `WrongKind` beside the text value the plan named.
- **Scope record:** none — `gate.py scope`: `scope: clean — changed 19 · listed 19 · recorded 0 (companion 0 · mechanical 0 ·
  in-intent 0 · widening 0) · absorbed 0 · excluded 59`, 0 recorded.

## Decisions & corrections
- **The founder's rulings on the fork, by dialog** (inputs#I2): F1 «Аргумент part, части по умолчанию (Рекомендую)» —
  an optional `part`, default paging over the budget, the paged MCP answer as its JSON line; F2 «Пустая часть (Рекомендую)» —
  a part past the last answers the empty text, no new cause. Both are relayed by the overseer as his own answers.
- **The operator's word «yes» on the P5 review card 2** (inputs#I3) approved:
  - its concretized v020-09 acceptance, which put «the same walk through `serve_mcp`» in the ledger;
  - its Beyond-the-plan row 1, a CARRY on «Capability statement» for the unpaged diff and lint findings (P5 lands it).
- **A matrix acceptance wider than its plan step was followed, and cost one re-run.**
  - Measured: plan step 12.7 named two MCP calls, while v020-09's acceptance names «the same walk». The MCP test was extended
    before `matrix.py implement`, and the whole block re-ran green.
  - Implement's evolve record: `2026-10-20T10:45:45Z-b`, untyped.
  - Candidate rule: read the ledger's acceptance against the plan's steps before writing the satisfying test, not after
    the first green.
- **A plan's touchpoint row contradicted its step.**
  - Measured: plan.md's Codebase touchpoints line says «the record fed from the part»; step 7 and Provenance say the whole
    screen. Implement followed the step.
  - Implement's evolve record: `2026-10-20T10:32:33Z-b`, `input.plan-step-ambiguous`.
- **A guard refusal that cost its one re-issue.** bash-guard refused `cd /home/…/packages/escher-driver/src && wc -l …`
  («cd … moves the session cwd for every later call»), re-issued once as a subshell. This is host-linux.md's standing rule
  «`cd` only inside a subshell», met again. Implement's code-step record carries `guard-refused:1`.
- **The pre-pass sweep's unfired controls** (this wrap). Four patterns never fired over the masters: `parts`, `unitcount`,
  `limit`, and the title «Large screen in parts». Each zero carries the refusal line; the title's is the owner pattern's
  `control unfired`.
- **The operator's directive for the operator pass**, in the implementing session, after implement's report: «Run the operator
  pass for 2026-10-20-large-screen-in-parts now, each entry in plan order: 18 hygiene; the commit with the subject exactly
  "chore(2026-10-20-large-screen-in-parts): operator pre-CI commit, for the run this chunk's verdict reads"; 19 the fast leg
  with the clean-tree guard and the push; 20 ci.py conclusion; 21 the per-OS tallies. Record it in evidence/ci.md. Stop and
  tell me if any entry is red.» Every entry was fired in order and none was red (Outcome).
- **The stand-repeat job counts more than the `stand_*` files** (a measured fact for the tally readers). Its +12 passes
  split as `stand_snapshot_parts` 0 → 9 and seven_guis `cli_commands` 39 → 42, read by a per-target diff of the two CI runs'
  job logs. The leg runs seven_guis' binary checks too, so a new `cli_commands` test moves the stand-repeat tally by three.
- **The operator's word at this wrap:** «run P1 only and stop» (the two-window form). P2 resumes in this run dir.

## Outcome
- **Acceptance criteria, re-asserted against the diff:**
  - **(tests)** On the over-budget fixture, both layout modes, through `Session::run` (`stand_snapshot_parts`, 3 passed):
    met.
    - `snapshot` with no argument answers part 1 of `parts` ≥ 2;
    - each part 1..=`parts` is within the budget;
    - the parts concatenate to the whole `to_text()`, every node id on exactly one line;
    - `parts + 1` answers the empty text with the same `parts`, the screen unchanged;
    - `item-400`, read only in the last part, takes `scroll` (in view) then `click` (accepted, diff names `clicks`).
  - **(arch)** Every stand screen answers as before: met. `stand_snapshot_text` 6 and `stand_replay` 3 pass unedited (entry 4),
    the committed Counter checks re-record byte-equal (`recording` 4, entry 5), and on the Counter `snapshot` answers
    `Outcome::Screen` equal to `to_text()`.
  - **(arch)** The walk is reached only through `Session::run` after `validate`. `part`, `part` and `parts` are in `VERBS` with
    their help, in `help` (describe) and MCP `tools/list` (`mcp_tests` pin). `VERBS` 12, `CAUSES` 8, describe's count row
    unmoved: met (escher-driver 100 passed).
  - **(security)** `part` 0, 1001, −1, `i64::MAX`, a text and a flag are refused `malformed` with nothing run (command.rs rows).
    On the command line 0, 1001 and `x` exit 1 with a line equal to `Refusal::malformed(fault).to_json()`, nothing typed:
    met.
  - **(security)** The sentinel occurs 0 times across a walk, and the `secret` line reads `MASKED_VALUE`: met.
  - **(a11y)** Each part's lines equal their lines in the whole text, and exactly one line reads `focused` after the click: met.
  - **(obs)** A part read's `command` line holds `verb="snapshot"` and no other field (`stand_act_spans`, 35 calls per mode).
    escher-driver keeps 2 `info_span!` sites. `host_log` and `mcp_log` drive one part read at `trace` with 0 id and 0 name
    needles under the `-p seven_guis` build (entry 5) and the workspace build (entry 15's workspace tests, and every CI test
    job): met.
  - **(design)** `snapshot --part` through `escher-session` writes one uncoloured JSON line and exits 0 accepted or 1 refused
    (`cli_commands`): met.
  - **(layouts)** No stand source, shared fixture, stand table or committed check changed (entry 10 → 0), and the four lean
    tasks mount as before (`run stand` green): met.
  - **(tests)** `fast` and `doc` pass, the a11y leg's six files keep 6 · 6 · 4 · 8 · 5 · 14, and CI on the pushed sha is green
    with the coverage floors and stand-repeat held (CI#39410865604, 17/17): met.
- **Gates.** The block was fired twice by /implement, both green 17/17. The second firing's lines are below; the summary
  read `entries 21 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 4`.
  - `cargo test -p dioxus-native-dom --locked --lib 2>&1` — green, exit 0 · lacks FAILED (81 passed).
  - `cargo test -p escher-driver --locked 2>&1` — green, exit 0 · lacks FAILED (100 passed, 0 doc tests).
  - `cargo test -p blitz-tests --locked --test stand_snapshot_parts 2>&1` — green, exit 0 · contains `0 failed` (3 passed).
  - `cargo test -p blitz-tests --locked --test stand_act_spans --test stand_snapshot_text --test stand_replay --test stand_mcp_tools 2>&1`
    — green, exit 0 · lacks FAILED.
  - `cargo test -p seven_guis --locked --test cli_commands --test host_log --test mcp_log --test recording 2>&1` — green, exit 0
    · lacks FAILED (14 · 2 · 2 · 4).
  - The silence census on `stand_snapshot_parts.rs` — green, exit 1 · last line 0.
  - The print/log census on `session_common/mod.rs` — green, exit 1 · last line 0.
  - The `snapshot_text.rs` site census — green, exit 0 · last line 6.
  - `grep -rc 'info_span!' packages/escher-driver/src | awk …` — green, exit 0 · last line 2.
  - The manifest / stand / shared-module `git diff … | wc -l` — green, exit 0 · last line 0.
  - `gate.py guard … --base c1a2e5b85…` — green, exit 0 · last line `guard: clean`. Its pass line:
    `guard: allowed 19 — examples/seven_guis/tests/cli_commands.rs, examples/seven_guis/tests/host_log.rs, examples/seven_guis/tests/mcp_log.rs, packages/dioxus-native-dom/src/lib.rs, packages/dioxus-native-dom/src/snapshot_text.rs, packages/escher-driver/src/cli.rs, packages/escher-driver/src/cli_tests.rs, packages/escher-driver/src/command.rs, packages/escher-driver/src/execute.rs, packages/escher-driver/src/json.rs, packages/escher-driver/src/lib.rs, packages/escher-driver/src/mcp/listing.rs, packages/escher-driver/src/mcp_tests.rs, packages/escher-driver/src/schema.rs, packages/escher-driver/src/schema/spec.rs, packages/escher-driver/src/schema_tests.rs, tests/blitz-tests/tests/session_common/mod.rs, tests/blitz-tests/tests/stand_act_spans.rs, tests/blitz-tests/tests/stand_snapshot_parts.rs · companions excluded 0 · changed 19 · excluded 0`.
  - `bash scripts/agent-run.sh boot` — green, exit 0 · contains `"outcome": "ready"`.
  - `bash scripts/agent-run.sh run stand` — green, exit 0 · contains `"failed": 0` (48 files · 195 · 0 · 7).
  - `bash .github/scripts/ci-leg.sh a11y` — green, exit 0 (6 · 6 · 4 · 8 · 5 · 14 = 43).
  - `bash .github/scripts/ci-leg.sh fast` — green, exit 0, artifact `target/ci-logs/test.log` fresh.
  - The `test.log` tally — green, exit 0 · contains `failed 0` (`lines 205 passed 1098 failed 0 ignored 12`).
  - `bash .github/scripts/ci-leg.sh doc` — green, exit 0.
  - Operator entries, fired in the operator pass, their results in `evidence/ci.md`:
    - `python -X utf8 ~/…/gate.py hygiene` — `hygiene: clean — read 50 …`.
    - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.2.0`
      (`--operator 19`) — green, exit 0, `history moved: refs/remotes/origin/build/escher-0.2.0 c1a2e5b8→191e66fd`, the
      intended push.
    - `python -X utf8 ~/…/ci.py conclusion --sha HEAD --wait 1800` — `191e66fdbe7f verdict: green · checks 17/17`,
      CI#39410865604.
    - The `gh run view <id> … --log | … awk …` tallies (`--operator 21 --id 39410865604`) — recorded, exit 0, history
      unmoved. The tallies and their move, each explained by name, are in `evidence/ci.md`. The CI outcome it read is
      green, this chunk's.
  - No `defer` key in the block, no deferral, no `leg = 'round'` or `live` entry. Smoke: the plan lists none. The one
    binary touchpoint (`cli.rs`, the argv reader) was driven as the `escher-session` process by the seven_guis entry.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran, so the verdicts rest on its final state.
  - Its one commit is `191e66fdb` (Setup 4: `git log --format='%h %s' c1a2e5b85..HEAD`). The final HEAD's CI is
    CI#39410865604 on `191e66fdbe7f`, green 17/17, recorded in `chunks/2026-10-20-large-screen-in-parts/evidence/ci.md`.
  - This window holds the implement run whole: `.andromeda/runs/2026-10-20T10-25-09-implement/`. Its
    `outcome-2026-10-20-large-screen-in-parts.md` reads green, superseded here only by the operator pass's results.
  - No other implement run dir holds a `gate-2026-10-20-large-screen-in-parts.json`.
- **Process hygiene:**
  - implement P4's census: the `escher-session` hosts and mcp children the seven_guis checks started, and the agent-run
    test runs, all terminated (`census: none running`).
  - Re-measured at this wrap: `gate.py census --comm escher-session --comm seven_guis_native --args escher-session --args agent-run`
    → `census: none running`.
## Addendum — resumed 2026-10-20
- inputs#I4 · message: the operator, in this session, at the wrap resume, 2026-10-20T11:30Z · copy message · n/a — the
  operator's word at the resume: the wrap resumes from this report in its run dir; the founder is reachable, and the
  answer card on his F1/F2 (inputs#I2) goes to the operator by dialog, who brings it to him; at curation, the learning
  `.claude/docs/session-learnings.md:59-66` is weighed for correction under classify v1.3 (provenance counted by class
  and span, not by the neighbouring words; a row never cleared by deleting standing text).
  - bears on **The founder's rulings on the fork, by dialog**: adds to it — the card P2's Validate stops on is carried
    to the founder by the operator.
  - bears on **The operator's word at this wrap**: supersedes «run P1 only and stop» — the wrap resumes at P2.
- inputs.py verify, re-fired: `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 4 · uncited 1 · unparsed 0` (the one uncited entry I4, cited by this addendum); moved since the
  report: I3 uncited → cited (report.md:214), I4 added.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: c1a2e5b8 (the parent of the oldest pre-CI commit 191e66fd) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### examples/seven_guis/tests/cli_commands.rs — added 52 line(s) in 3 range(s)
added: 6-9 · 30 · 294-340
- 294-339 @295 «fn snapshot_reads_a_part_by_number_and_a_part_outside_its_bound_is_refused() {»
  - 298-301 «assert!(»
  - 303-306 «assert!(»
  - 307-310 «assert!(»
  - 311-314 «assert!(»
  - 315-318 «assert_eq!(»
  - 319-323 «let rows: [(&str, Fault); 3] = [»
  - 324-333 «for (row, (part, fault)) in rows.into_iter().enumerate() {»
  - 334-337 «assert!(»
### examples/seven_guis/tests/host_log.rs — added 11 line(s) in 4 range(s)
added: 4 · 97 · 128-131 · 173-177
  - 173-177 «assert!(»
### examples/seven_guis/tests/mcp_log.rs — added 10 line(s) in 4 range(s)
added: 4-5 · 109 · 112 · 159-164
  - 159-164 «assert!(»
### packages/dioxus-native-dom/src/lib.rs — added 3 line(s) in 2 range(s)
added: 8-9 · 39
### packages/dioxus-native-dom/src/snapshot_text.rs — added 162 line(s) in 5 range(s)
added: 15-16 · 19-29 · 55-84 · 88 · 492-609
- 19-28 @22 «pub struct TextPart {»
  - 59-81 @66 «pub fn text_parts(&self, budget: usize) -> Vec<TextPart> {»
  - 493-514 «fn sample() -> Snapshot {»
  - 516-532 @517 «fn parts_concatenate_to_the_whole_text_each_within_the_budget() {»
  - 534-555 @535 «fn each_node_id_is_in_exactly_one_part_in_pre_order() {»
  - 557-573 @558 «fn a_line_longer_than_the_budget_is_a_part_alone() {»
  - 575-580 @576 «fn an_empty_snapshot_is_one_empty_part() {»
  - 582-596 @583 «fn a_budget_that_holds_the_whole_text_is_one_part() {»
  - 598-609 @599 «fn a_part_keeps_each_lines_absolute_indent() {»
### packages/escher-driver/src/cli.rs — added 5 line(s) in 1 range(s)
added: 253-257
### packages/escher-driver/src/cli_tests.rs — added 19 line(s) in 4 range(s)
added: 135-138 · 377-388 · 436 · 550-551
### packages/escher-driver/src/command.rs — added 42 line(s) in 12 range(s)
added: 7-8 · 139-144 · 187 · 235 · 252-253 · 362-366 · 404-412 · 547 · 643 · 695 · 795-806 · 820
  - 139-144 @140 «Snapshot {»
### packages/escher-driver/src/execute.rs — added 44 line(s) in 7 range(s)
added: 8-10 · 27 · 32-41 · 133-138 · 188-204 · 308-313 · 600
- 8-10 «use dioxus_native_dom::{»
  - 32-41 @34 «Part {»
### packages/escher-driver/src/json.rs — added 29 line(s) in 4 range(s)
added: 330-336 · 717 · 720-739 · 817
### packages/escher-driver/src/lib.rs — added 2 line(s) in 1 range(s)
added: 156-157
### packages/escher-driver/src/mcp/listing.rs — added 6 line(s) in 2 range(s)
added: 8 · 85-89
### packages/escher-driver/src/mcp_tests.rs — added 28 line(s) in 5 range(s)
added: 15 · 392 · 466-471 · 474 · 481-499
- 481-498 @482 «fn the_snapshot_tool_takes_an_optional_part_and_names_its_two_counts() {»
  - 484-489 «let input_schema = concat!(»
  - 493-496 «assert!(described.contains(»
### packages/escher-driver/src/schema.rs — added 41 line(s) in 3 range(s)
added: 17-19 · 178-190 · 192-216
  - 178-184 «help: "reads the screen: every element with its id, role, name, state and bounds; a screen \»
  - 192-216 «fields: &[»
### packages/escher-driver/src/schema/spec.rs — added 5 line(s) in 3 range(s)
added: 19-21 · 34 · 47
### packages/escher-driver/src/schema_tests.rs — added 39 line(s) in 9 range(s)
added: 62-86 · 124 · 184-188 · 253-254 · 260-261 · 286 · 321 · 333 · 335
### tests/blitz-tests/tests/session_common/mod.rs — added 18 line(s) in 2 range(s)
added: 68-81 · 213-216
- 68-71 @69 «pub fn snapshot_part(n: i64) -> Call {»
- 73-80 @75 «pub fn part(outcome: Outcome) -> (String, u32, u32) {»
  - 76-79 «match outcome {»
### tests/blitz-tests/tests/stand_act_spans.rs — added 17 line(s) in 7 range(s)
added: 16 · 41-42 · 183 · 195-198 · 356-362 · 611 · 717
### tests/blitz-tests/tests/stand_snapshot_parts.rs — new file · 333 line(s)
- 35-53 @36 «fn many_items() -> Element {»
  - 38-52 «rsx! {»
- 55-71 @57 «fn walk(session: &mut Session, mode: &str) -> (Vec<String>, u32) {»
  - 61-69 «for n in 2..=parts {»
- 73-80 @74 «fn lines_holding(texts: &[String], needle: &str) -> usize {»
  - 75-79 «texts»
- 82-204 @83 «fn a_screen_over_the_budget_is_walked_in_parts() {»
  - 84-203 «for incremental in [false, true] {»
- 206-210 «fn tool_call(id: usize, name: &str, arguments: &str) -> String {»
  - 207-209 «format!(»
- 212-241 @215 «fn result(reply: &str) -> (bool, String) {»
  - 216-218 «let (_, rest) = reply»
  - 221-239 «loop {»
- 243-304 @244 «fn over_mcp_the_walk_answers_the_json_lines_of_the_same_calls() {»
  - 245-303 «for incremental in [false, true] {»
- 306-333 @307 «fn a_screen_inside_the_budget_reads_whole_as_before() {»
  - 308-332 «for incremental in [false, true] {»
