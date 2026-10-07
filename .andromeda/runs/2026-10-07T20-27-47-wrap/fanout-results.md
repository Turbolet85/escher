# Fan-out results — 2026-10-07-refusal-detection

Seven detector returns, collected from their transcripts by script (entity-decoded; the probe
`entities=0` held on every return before and after decoding). Proposal ids are this file's: a
letter per doc and the proposal's position in its return (A architecture · S security-plan ·
L layout-templates · T test-plan · O obs-plan · Y a11y-plan; X is raised by the orchestrator).
This wrap resumed from `report.md` in a fresh window; the fan-out ran in the resuming window.
One edit was made to the returns as collected: two of them (layout-templates, a11y-plan) spelled
`basis` paths from the host's root, and the repository root prefix is dropped here so that every
path reads repo-relative.

## Verdicts

- **architecture** — 27 proposals (D-arch-resources 26 · D-arch-decisions 1; graded `warning` 26 · `escalate` 1). Stripped beside the list: 13 comment lines; decoding changed nothing.
- **security-plan** — 8 proposals (D-security-input 8; graded `escalate` 8). Stripped beside the list: 8 comment lines; decoding changed nothing.
- **design-system** — `proposals: []`. Stripping removed 14 comment lines; the raw return is `.raw-fanout-design-system.md`; decoding changed nothing.
- **layout-templates** — 1 proposal (D-layout-surface 1; graded `warning`). Stripped beside the list: 6 comment lines; decoding changed nothing.
- **test-plan** — 14 proposals (D-tests-coverage 8 · D-tests-obs-harness 6; graded `warning` 14). Stripped beside the list: 32 comment lines; decoding changed nothing.
- **obs-plan** — 8 proposals (D-obs-instrumentation 3 · D-obs-stack 5; graded `warning` 8). Stripped beside the list: 6 comment lines; decoding changed nothing.
- **a11y-plan** — 1 proposal (D-a11y-surface 1; graded `warning`). Stripped beside the list: 21 comment lines; decoding changed nothing.

Total: 59 proposals over 6 docs; 1 doc returned none.

## Validate — dispositions

**Before the checks — the re-derivation tell.** Two facts rest on a detector's own read of the tree and
are in no bullet of the report:

- **O5** — "the module fourteen of those fifteen read": the count is the detector's own grep. **REJECTED
  as proposed**, its fact re-raised as **X1** from the orchestrator's measurement (below). The same count
  inside T9, T10, T11 and A22 is taken from X1, never from the returns' greps; the rest of those four is
  the report's.
- **The `lib.rs` citations in A12 and A24** — "lib.rs gained 8 lines" is the architecture return's own diff
  read ("One figure not in the report"). That half of A12 and A24 is **REJECTED as proposed** and
  re-raised as **X2**; the rest of both is the report's.

Every other coordinate a proposal carries (a moved line range, a symbol's line) is a check of a move the
report states — the file, the direction and the size — and each one is re-derived by the orchestrator's
own remap before it is written (`git show 9b758f6c:{path}` lines located in the working tree: 90
citations over the seven masters and the registry files, 25 moved mechanically, 35 read by hand, 30
unchanged). No coordinate is pasted from a `change` line.

1. **Playbook.**
   - **Escalated — two widenings, judged by subject** ("Boundary widening": never routine, whatever a
     plan or a direction says):
     - **W1 — the verb table five → six (`scroll`)**: a validated surface admits a new input. Primary A1
       with A2–A5 and A13; S1 with S2, S7, S8; and the same claim's other wordings in T5, T6 and O7.
       The plan names it this class itself (`plan.md:447`, `:451`). On the record: the founder's
       choice at the P4 forks (`inputs#I2`).
     - **W2 — `BaseDocument::scroll_into_view` scrolls nested boxes for every document** (script
       `scrollIntoView`, Dioxus `MountedData::scroll_to`, fragment navigation — the reference browser's
       remote pages included), with the locked decision "programmatic scrolls clamp to one scroller"
       qualified for it: A17 and A18 (D-arch-decisions — a locked decision qualified is ratified once,
       never applied under "Accurate this-chunk addition"), with X4's motion half and L1's first clause.
       On the record: the founder's choice at the P5 review (`inputs#I4`).
     Both are put to the operator at this wrap (§Escalations, below).
   - **Routine, "Accurate this-chunk addition"**: every other A, T, O, L and Y proposal names a fact the
     report's Changes carry as this chunk's work — the executor's detection and its order, the session's
     record, the click point, the reader, the harness helper, the counts and the moved citations — landing
     in an existing section with the detector's invariant holding.
   - **The eight `escalate` grades on security-plan** are the detector's severity for a VIOLATION of
     D-security-input; the return reports the invariant HOLDING ("the one new input … IS validated … no
     unvalidated boundary") and the plan's reviewed list names the change (entry 5). The rule appended
     2026-10-07 applies S3–S6 without a halt; it never covers a boundary widening, so S1, S2, S7 and S8
     ride W1.
   - **A15** (graded `escalate`: the scrolled box's own `bounds`) and **S5's** open question (nothing bounds
     a recorded id's length) are settled by the operator's direction at this wrap's invocation — each open
     finding of the report gets a route owner at route-resolve, the upstreamable flag kept in the handoff.
     The body states what was measured; the owner is P5's CARRY. No halt. "Not this chunk's drift" does
     not govern A15: the reading is in the report's Changes (Spec claims disproved), the claim it
     qualifies stands at one site (`architecture.md:136@c11555`), and the amendment brings that site to
     the measured truth rather than leaving a mismatch.
   - **"Registry over-reach" does not govern A1–A5, A16 or A19**: §Standard Contracts states contract
     shapes, and the four driver chunks before this one registered theirs there.
2. **Cross-contradiction.** None in direction. Two returns disagree on a coordinate and neither is used:
   A9 moves `session.rs:11-99` to `:10-157`, the security return's note reads the label check at
   `:10-14` — the file reads `valid_label` at `:13-18` and `impl Session` ending `:155`. Where two docs
   state one fact they agree: the reader counts (A22 · T9 · T10 · T11 · X1), the nine files and 27 tests
   (A20 · A21 · T1–T5 · O4), eight causes returned (A6 · S3 · T5 · O1), the record (A8 · S5 · S6 · O7).
3. **Intent-consistency.** The entry's intent is the five screen-level causes named per action; the
   report goes past it in two places, both on a recorded word: the sixth verb (`inputs#I2`, the entry's
   own CARRY posed the fork) and the engine change (`inputs#I3`, `inputs#I4`). Justified; intent was
   incomplete, and the bodies now state both. No scope-record line (`gate.py scope` clean at implement,
   at P1 and at this resume: changed 15 · listed 15). The five deviations are each justified in the
   report; deviation 3 (the byte figure is not a property of the code) is written into the bodies as
   built and owned on the route.
4. **Absence needs evidence.**
   - New names, 0 hits before this pass in all seven masters and the registry files, by fixed-string
     count: `visible_region` · `in_view` · `Scrolled` · `SeenIds` · `MAX_SEEN_IDS` · `six verbs`. The old
     `off-screen` texts are quoted nowhere: `outside the viewport` 0 · `bring it into view` 0.
   - "The new check files install no sink": `grep -cE 'println!|eprintln!|print!|dbg!|escher_telemetry|tracing::|env::var|log::'`
     reads 0 in each of the nine `stand_act_*` files, in `scroll_into_view_nested.rs` and in
     `session_common/mod.rs`; `execute.rs` and `session.rs` read 0 with `Instant|SystemTime` added.
   - Every master site was read whole by line or by offset window (architecture holds 27 lines over
     2 000 chars, security-plan 5, test-plan 8, obs-plan 2, a11y-plan 2, layout-templates 1).
   - **Sites the report's searches missed, found by the detectors or the remap, all applied:**
     `architecture.md:87` (the locked decision "programmatic scrolls clamp to one scroller", and two
     `scrolling.rs` citations past the edited range) · `architecture.md:133@c5378` and `:136@c18311`
     ("four acting") · `architecture.md:133`, `:257` (three `lib.rs` citations — the report's citation
     list omits `lib.rs`) · `architecture.md:149`, `:150` · `security-plan.md:75`, `:82`, `:295` ·
     `security-plan.md:116` ("four acting commands"; "`click` and `type` … take a caller-supplied id") ·
     `design-system.md:283` (`scrolling.rs:724-747`, which the report read as unmoved) ·
     `test-plan.md:49`, `:65` · `obs-plan.md:325`. The report's expected-amendment 3 cites `Outcome::`
     at architecture `:132`; the hits are `:133` and `:136`. A marked note at the report's end says so.
5. **Expected amendments.** Nine entries. 1 → A1–A12 · 2 → A16–A18 · 3 → A2, A4, A5, A11 · 4 → A19–A25 ·
   5 → S1–S8 for §Input Validation; its §Error Handling half drew no proposal (the return: "every
   statement still holds … only its citations moved") and is raised as **X3** · 6 → T1–T8 · 7 → O1–O7 ·
   8 → Y1 · 9 → L1 for layout-templates; design-system returned none (no detector invariant reaches a
   motion bullet) and its half is raised as **X4**. Citation-only moves no detector's invariant reached
   are raised as **X5**. With them every moved coordinate of the remap has an owner.
6. **Disproved claims.** (1) `bounds` for a box that is itself scrolled → A15 in the body, as measured,
   and a CARRY at P5 with the upstreamable flag (the operator's direction). (2) The plan's forecast of
   three refused nodes → a forecast, not a spec claim; the bodies state the measured 70 and 7 (A11, T4).
   (3) "six of the eight causes" → the bodies state eight of eight, six by the executor (A6, S3, O1).
   (4) The hit walk reaches a scrolled-out row, as predicted → stated as measured beside the reader
   (A16) and a CARRY at P5 (the operator's direction). All four DISPOSED.

**Dispositions.** A1–A5, A13, A17, A18 · S1, S2, S7, S8 — escalated as W1 and W2, then applied as the
operator resolves them (§Escalations) · A6–A12 (the `lib.rs` half of A12 by X2), A14–A16, A19–A27 (the
`lib.rs` half of A24 by X2) apply · S3–S6 apply · L1 apply · T1–T14 apply · O1–O4, O6–O8 apply · O5 reject
(re-raised as X1) · Y1 apply · X1–X5 raised and applied. Applied text is re-derived from the report's
facts and the code it names, never pasted from a `change` line.

**X1 — raised (the reader counts).** Measured: `grep -l '^mod session_common;'` over
`tests/blitz-tests/tests/*.rs` lists 14 files — four `stand_session_*` (`fresh`, `ids`, `lifecycle`,
`quiet`), `stand_settle` and the nine `stand_act_*`; `grep -l '^mod common;'` lists 14, as before — of the
nine `stand_act_*` only `ids`, `diff` and `keys`. Sites: architecture §Existing Scopes → blitz-tests ·
test-plan §2 Directory pattern · §3 blitz-tests (stand checks) · (session checks) · obs-plan §3 Logging
stack.

**X2 — raised (`lib.rs` citations).** `packages/escher-driver/src/lib.rs:33-57` → `:41-65` and `:35-57` →
`:43-65` (`architecture.md:133` twice, `:257` once), from the remap: the old block found once in the
working tree.

**X3 — raised (security-plan §Error Handling, the `Refusal` bullet).** The set stays eight with no
`String` field; two texts of `off-screen` are reworded — its remedy names the verb `scroll` (the
founder's ruling, `inputs#I2`) and its meaning covers a scrolling box (the operator's directive,
`inputs#I3`); the executor now returns six of the eight; `refusal.rs:115-126` → `:119-130`,
`:144-147` → `:148-151`.

**X4 — raised (design-system §Motion → Animation runtime).** A programmatic into-view scroll never
animates a nested box, and the driver's `scroll` is instant in every box and in the viewport;
`scrolling.rs:724-747` → `:875-898` at `design-system.md:283`. Its first half rides W2.

**X5 — raised (citation-only).** `security-plan.md:75` `session.rs:11-52` → `:13-108` ·
`security-plan.md:82` `scrolling.rs:663-666` → `:814-817` · `security-plan.md:295`
`session_common/mod.rs:195` → `:233`.

## Escalations

Two, both resolved WITH the operator at this wrap, before any body was edited
(`operator-rulings.md`, Message 2).

- **W1 — the sixth verb.** Resolved: **ratified**. The bodies state six verbs as ratified by the
  founder (2026-10-07) — word: "Команда scroll сейчас" — the founder, 2026-10-07, relayed verbatim by
  the overseer (`inputs#I2`); the option's wording is the overseer's rendering, the choice his own.
  Confirmed at the escalation: the operator, 2026-10-07. Applied: A1–A5, A13, S1, S2, S7, S8, and the
  verb lists of T5, T6 and O7.
- **W2 — the engine's into-view scroll, for every document.** Resolved: **ratified**. The bodies
  state the widened method and the qualified locked decision as ratified by the founder
  (2026-10-07) — word: "Исправить саму функцию" — the founder, 2026-10-07, relayed verbatim by the
  overseer (`inputs#I4`); the option's wording is the overseer's rendering, the choice his own.
  Confirmed at the escalation: the operator, 2026-10-07. Applied: A17, A18, X4, L1.

No PROVISIONAL mark is written, and none stood before this pass (`grep -ci PROVISIONAL` reads 0 in
each of the seven masters and the four key files). No rule is proposed: the class is never-routine
and keeps reaching the operator.

**Escalations: 2 raised · 2 resolved · 0 open.**

## architecture — the return (27; ids A1–A27 by position)

The return as collected, its comment lines kept in place.

```yaml
proposals:
  # ---- Driver session: the sixth verb (primary) and every site that restates the five-verb shape ----
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`VERBS` lists six verbs in order — `snapshot`, `click` (`id`), `type` (`id`, `text`), `press` (`key`, optional `shift`), `advance` (`ms`) and `scroll` (one required `id`, kind `Id`, 1 to 1024 bytes) — replacing 'lists five verbs'; the sixth verb is a boundary widening on the founder's own word, relayed by the overseer (inputs#I2), not provisional (packages/escher-driver/src/schema.rs:290)."
    sidecar: "Driver session: VERBS five -> six, `scroll` (`id`) registered (2026-10-07-refusal-detection)."
    rationale: "Report Changes > Symbols / APIs: `VERBS` (schema.rs:290) lists six, `scroll` takes one required `id`; Counts: driver verbs 5 -> 6, `five verbs` architecture 1 (:133); Expected amendments 1. The new verb is unregistered in the contract."
    basis: "packages/escher-driver/src/schema.rs:290"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "'The four acting verbs' result fields' becomes five acting verbs — `click`, `type`, `press`, `advance`, `scroll` share `settled`, `busy`, `added`, `changed`, `removed`; `advance` adds `advanced_ms` and `scroll` adds `in_view` (`Flag`, always present), the seventh result-field name; `FieldKind` is unchanged."
    sidecar: "Driver session: acting verbs four -> five; result field `in_view` registered."
    rationale: "Report Changes > Symbols / APIs: `scroll` 'returns the five acting fields and a sixth, `in_view` (`Flag`, always present)'; Schema / config: 'a seventh result-field name, `in_view`'. `in_view` has 0 hits in architecture."
    basis: "packages/escher-driver/src/schema.rs:193"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`Command` is exactly `Snapshot` · `Click { id }` · `Type { id, text }` · `Press { key, shift }` · `Advance { ms: u32 }` · `Scroll { id }`, with `spec()`; `validate`'s order, its five argument kinds and their bounds are unchanged."
    sidecar: "Driver session: `Command::Scroll { id }` added to the closed `Command` list."
    rationale: "Report Changes > Symbols / APIs: `Command::Scroll { id }` (new variant, command.rs:131) with its `spec` and `validate` arms. The contract's 'exactly' list of five is retired."
    basis: "packages/escher-driver/src/command.rs:131"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`Outcome` is exactly `Screen { text: String }` · `Acted { settled, busy, diff }` · `Advanced { settled, busy, diff, advanced_ms: u32 }` · `Scrolled { settled: bool, busy: Option<Busy>, diff: SnapshotDiff, in_view: bool }` (`Debug`, `Clone`, `PartialEq`); `Acted` is what `click`, `type` and `press` return."
    sidecar: "Driver session: `Outcome::Scrolled` added to the closed `Outcome` list."
    rationale: "Report Changes > Symbols / APIs: `Outcome::Scrolled { settled, busy, diff, in_view }` (new variant, execute.rs:44). The contract's 'exactly' list of three is retired."
    basis: "packages/escher-driver/src/execute.rs:15-57"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Add to the `Session::run` paragraph, after `advance`: `scroll` resolves its id as `click` does, applies none of the `disabled` / `off-screen` / `covered` checks, calls `Harness::scroll_into_view` (instant, nearest on both axes — every scrolling box that holds the element, then the viewport) as one settled step, and answers `in_view` by making the `off-screen` reading again on the snapshot taken after the step — false when scrolling could not bring the element into view or the step removed it; `scroll` is never refused for failing to reach its target, its result says so."
    sidecar: "Driver session: `scroll`'s run behaviour and the meaning of `in_view` stated."
    rationale: "Report Changes > Symbols / APIs > `Session::run`: '`scroll` applies none of the three checks; it calls the harness helper as one settled step and answers `in_view` ...'; Reverted / negative API facts: no refusal from `scroll` when it cannot reach its target. Unregistered behaviour of a new verb."
    basis: "packages/escher-driver/src/execute.rs:126-139"
    dependent-of: D-arch-resources
  # ---- Driver session: refusal detection (primary) and every site that restates the old detection ----
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace 'the executor returns two more, `not-found` and `time-unavailable` (below), and nothing yet returns `stale`, `disabled`, `covered` or `off-screen`' with: all eight causes are returned by code — `unknown-verb` and `malformed` by `validate`, and `not-found`, `stale`, `disabled`, `covered`, `off-screen` and `time-unavailable` by the executor."
    sidecar: "Driver session: causes returned by code 4 of 8 -> 8 of 8 (two by `validate`, six by the executor)."
    rationale: "Report Changes > Symbols / APIs: 'Which causes code returns now: eight of eight ... (was four of eight)'; Counts: `yet returns` architecture 1 (:133); Spec claims disproved: 'six' is the executor's share, the total is eight."
    basis: "packages/escher-driver/src/execute.rs:189-223"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace '`click` and `type` resolve their id first — the first pair of `element_ids()` ... refused `not-found` with nothing run' with: `click`, `type` and `scroll` read the screen's snapshot and resolve their id on it — the target is on the screen when the snapshot lists the id and the first `DioxusDocument::element_ids()` pair with the id resolves through `get_node`; otherwise the call is refused with nothing run, `stale` when the session's record holds the id and `not-found` when it does not."
    sidecar: "Driver session: id resolution reads the snapshot; a missing id is `stale` or `not-found` by the session's record."
    rationale: "Report Changes > Symbols / APIs > `Session::run`, first sub-bullet ('For `click`, `type`, `scroll`: the screen's snapshot is read ... `stale` when the session's record holds the id, `not-found` when not'). The contract's resolution-by-`element_ids`-only and its sole `not-found` answer are retired."
    basis: "packages/escher-driver/src/execute.rs:167-203"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace 'the session keeps no id and no `NodeId` between calls' with: the session keeps no `NodeId` between calls and keeps a record of id text only — a crate-private `SeenIds` bounded by `MAX_SEEN_IDS = 4096` ids (a count of ids, not of bytes, and unrelated to `MAX_TEXT_BYTES` = 4096; nothing bounds a recorded id's length), empty at `Session::start`, living as long as the session, reading no clock, with no public reader and never printed or logged; re-reading an id makes it the most recently read and past the bound the id read longest ago is forgotten; it is fed from every snapshot a call takes after `validate` admits it (the screen read for a resolution — recorded after the `stale`/`not-found` answer is decided, also on a refusal — the snapshot after a settled step, the screen `snapshot` returns, both snapshots of a `press` or an `advance`); a call `validate` refuses and an `advance` refused `time-unavailable` record nothing."
    sidecar: "Driver session: 'keeps no id' retired — the bounded record of id text (`SeenIds`, 4096 ids) registered."
    rationale: "Report Changes > Symbols / APIs: 'The session's record of ids' (session.rs:25, :32, :81); Counts: 'A new bound: ... at most 4096 ids. It is a count of ids, not of bytes'; Deviations 3 (the byte figure is not a property); Sweep hazards: 4096 is already `MAX_TEXT_BYTES` at :133. Site: `keeps no id` architecture 1 (:133)."
    basis: "packages/escher-driver/src/session.rs:25-81"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace 'the session holds a label, the harness and that optional step only' with: the session holds a label, the harness, that optional step and one private field, the bounded record of id text (below) — still no `NodeId`, no viewport, font or scheme setting, no subscriber; and move the paragraph's citation `session.rs:11-99` to `session.rs:10-157` (`Session` :81, `start` :95, `with_time` :116, `act` :146)."
    sidecar: "Driver session: the session's fields now include the id record; session.rs citation moved."
    rationale: "Report Changes > Symbols / APIs: '`Session` gains one private field (session.rs:81)'; Citation moves: `Session` :27 -> :81, `start` :40 -> :95, `with_time` :60 -> :116, `act` :90 -> :146. The 'only' list restates the claim the record retires."
    basis: "packages/escher-driver/src/session.rs:81-157"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace '`click` clicks the centre of the node's border box (`Harness::click_at`)' with: `click` clicks the centre of the snapshot's `bounds` for the node plus the viewport's scroll — a page point — through `Harness::click_at`, and `type` clicks the same point; for a box that is itself scrolled, `bounds` is shifted by the box's own scroll offset, so the point and the `off-screen` reading are off by that offset (measured, see the Dioxus DOM bridge `bounds` clause)."
    sidecar: "Driver session: click point moved from the layout border-box centre to the snapshot-bounds centre plus viewport scroll."
    rationale: "Report Changes > Symbols / APIs > `Session::run`: 'The click point moved: the centre of the snapshot's `bounds` plus the viewport's scroll (was the centre of the layout border box, `centre_of`, removed)'; Spec claims disproved, first bullet (consequences on the driver). Site: `border box` architecture 1 (:133)."
    basis: "packages/escher-driver/src/execute.rs:207-223"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace 'A click on a disabled or a covered target runs, and its diff says what changed.' with: a `click` or a `type` on a target that cannot take it is refused with nothing run — the snapshot and both focus readings equal before and after — with one answer, the first that holds in this order: `disabled` (the snapshot node's `enabled` is `Some(false)`), `off-screen` (the centre of the node's `bounds` is outside `BaseDocument::visible_region` of the node, or that reads `None` — outside the viewport or outside the visible part of a scrolling box that holds it; its remedy names `scroll`), `covered` (the raw hit at the page point, resolved through `nearest_non_anonymous_ancestor` and then up the `parent` chain, never reaches the target; no hit reads the root element); `press` and `advance` are refused for no screen-level cause; a zero-height box can read `covered` (the stand's `task-header-spacer`: 70 of 77 nodes accept a click per layout mode)."
    sidecar: "Driver session: a click or type on a disabled, out-of-view or covered target is refused, in that fixed order; `off-screen` meaning and remedy reworded."
    rationale: "Report Changes > Symbols / APIs > `Session::run` second sub-bullet and `Cause::OffScreen` (two texts reworded); Counts: stand nodes accepted 77 -> 70 of 77; Deviations 1. Site: `A click on a disabled` architecture 1 (:133) — the contract asserts the opposite of what ships."
    basis: "packages/escher-driver/src/execute.rs:59-82"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Move the entry's citations into the edited files: `execute.rs:15-43; :60-96; :100-177` -> `execute.rs:15-57` (`Outcome`), `:59-142` (`Session::run`, at :82), `:144-281` (the resolution, detection and settled-step helpers); `command.rs:161` -> `command.rs:167` (`validate`); `lib.rs:33-57` -> `lib.rs:41-65` and `lib.rs:35-57` -> `lib.rs:43-65`; and append to the entry's 'as measured at' tail: the refusal detection and the `scroll` verb at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md."
    sidecar: "Driver session: citations into execute.rs, command.rs and lib.rs moved; provenance for the refusal-detection chunk appended."
    rationale: "Report Changes > Citation moves: execute.rs rewritten below its imports (346 lines, was 209; `run` at :82, `Outcome` at :15, `centre_of` gone), `validate` and below shifted by 5 to 22 lines; lib.rs is in the Modified list. Every amended claim in this entry would otherwise cite lines that no longer hold it."
    basis: "packages/escher-driver/src/lib.rs:43-65"
    dependent-of: D-arch-resources
  # ---- Dioxus DOM bridge: the same driver claims restated outside the Driver session entry ----
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: "Replace 'escher-driver's executor, whose four acting commands return it in `Outcome::Acted` and `Outcome::Advanced`' with: whose five acting commands return it in `Outcome::Acted`, `Outcome::Advanced` and `Outcome::Scrolled`; move its citation `execute.rs:108` to `execute.rs:257-267` (`before.diff(&after)` at :266)."
    sidecar: "Dioxus DOM bridge: the diff's driver caller restated for five acting commands and `Outcome::Scrolled`."
    rationale: "Duplicate of the five-verb / three-outcome claim retired in the Driver session entry. Report Changes: `Outcome::Scrolled` carries `diff`; Expected amendments 3 names `Outcome::` architecture 4 (:132 [sic], :136); Citation moves: execute.rs architecture 5."
    basis: "packages/escher-driver/src/execute.rs:257-267"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: "Move the citation of `to_text`'s one caller outside the tests (the executor's `Outcome::Screen` arm) from `execute.rs:63` to `execute.rs:86-88`; the claim itself stands."
    sidecar: "Dioxus DOM bridge: citation of the `Outcome::Screen` arm moved to execute.rs:86-88."
    rationale: "Report Changes > Citation moves: execute.rs is rewritten below its imports; old :63 is now doc text of `run`. Citation-only, in the same clause family as the diff-caller amendment."
    basis: "packages/escher-driver/src/execute.rs:86-88"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: "Qualify '`bounds` `BaseDocument::get_client_bounding_rect` (viewport-relative, unclamped)': the reading is viewport-relative for every element except a box that is itself scrolled — the reader positions a node through `unrounded_absolute_position(0, 0)`, which subtracts the node's OWN scroll offset as well as its ancestors', so a scrolled box's `bounds` is shifted by its own offset (measured: the stand's `crud-list` did not move and read y 112.796875 before a driver `scroll` and 94.796875 after, smaller by 18, and the scroll's diff names it among the changed nodes); predates 2026-10-07-refusal-detection, unfixed, no owner named, upstreamable."
    sidecar: "Dioxus DOM bridge: `bounds` is not viewport-relative for a box that is itself scrolled (measured; engine misreading, no owner)."
    rationale: "Report Changes > Spec claims disproved by measurement, first bullet, which names architecture :136 as the stating site; Deviations 2; Cross-project: flagged upstreamable (inputs#I4). Escalated because the misreading has no owner and the driver's click point and `off-screen` reading now depend on this field — whether to record it as a known gap or route a fix is the operator's call."
    basis: "escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/stand-census.md"
  # ---- Scrolling, selection, tree ----
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Scrolling, selection, tree"
    change: "Register `BaseDocument::visible_region(&self, node_id) -> Option<BoundingRect>` (packages/blitz-dom/src/scrolling.rs:749): read-only — the viewport's rect narrowed, per axis, to the padding box (less the scrollbar size) of every box on the element's containing-block chain whose `overflow` on that axis is not `visible`; viewport-relative CSS pixels, edges snapped to the 1/64 px grid `get_client_bounding_rect` uses; `None` for a node that does not resolve and when nothing is left; the element's own box is not consulted; visibility is read from this geometry, never from a hit — the raw hit reaches a row scrolled out of its scrolling box (measured)."
    sidecar: "Scrolling, selection, tree: new public reader `BaseDocument::visible_region` registered."
    rationale: "Report Changes > Symbols / APIs: `visible_region` (new, public, scrolling.rs:749); Spec claims disproved, last bullet (the hit walk reaches a clipped row, measured); Expected amendments 2: `visible_region` 0 hits in all seven masters. A new public engine API absent from the contract."
    basis: "packages/blitz-dom/src/scrolling.rs:749"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Scrolling, selection, tree"
    change: "State `scroll_into_view`'s widened behaviour (signature unchanged, for every document and all callers — script `scrollIntoView`, Dioxus `MountedData::scroll_to`, fragment navigation, the harness helper): it scrolls every scrolling box on the target's containing-block chain, innermost first, then the viewport; a box is scrolled when a programmatic scroll has range in it (`overflow` `scroll`, `auto` or `hidden` on the axis with overflowing content), the root element is left to the viewport; a nested box is written through `scroll_to(.., ScrollBehavior::Instant)` whatever behaviour was asked and dispatches no `scroll` event, and the requested behaviour applies to the viewport, whose offset is computed after the nested writes; and move the entry's citations `scrolling.rs:620-626` -> `scrolling.rs:666-672` and `scrolling.rs:712-722` -> `scrolling.rs:863-873`."
    sidecar: "Scrolling, selection, tree: `scroll_into_view` now scrolls nested scrolling boxes then the viewport; two scrolling.rs citations moved."
    rationale: "Report Changes > Symbols / APIs: `BaseDocument::scroll_into_view` (scrolling.rs:666, was :620) 'signature unchanged, behaviour widened for every document' — the founder's own word, not provisional (inputs#I4); Citation moves: `scroll_into_view` :620-654 -> :666-739, everything after old :654 sits 151 lines further down. Both architecture hits for `scroll_into_view` are at :124."
    basis: "packages/blitz-dom/src/scrolling.rs:666"
    dependent-of: D-arch-resources
  # ---- Established decision the widened scroll qualifies ----
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Scrolling and input]"
    change: "Qualify 'programmatic scrolls clamp to one scroller': `scroll_to` and `scroll_by` clamp to one scroller and chain nothing, while `scroll_into_view` writes every scrolling box on the target's containing-block chain, innermost first, then the viewport — each nested write an instant `scroll_to` clamped to its own box, only the viewport taking the requested behaviour, so a smooth into-view scroll never animates a nested box (the founder's word, 2026-10-07, relayed by the overseer: the method itself is widened for every document, not a second method); and move this entry's two citations below the edited range: `scrolling.rs:674-678` -> `scrolling.rs:825-829` and `scrolling.rs:732-746` -> `scrolling.rs:883-897`."
    sidecar: "[Scrolling and input]: 'programmatic scrolls clamp to one scroller' qualified for the widened `scroll_into_view`; two citations moved +151."
    rationale: "Report Changes > Symbols / APIs: `scroll_into_view` 'now scrolls every scrolling box on the target's containing-block chain, innermost first, then the viewport', nested writes Instant whatever behaviour was asked; Expected amendments 2 cites inputs#I4 as the authority. The locked decision's wording reads as one scroller per programmatic scroll; the chunk ships a programmatic scroll that moves several. No dependency, runtime or stack change: Dependencies 'none'."
    basis: "packages/blitz-dom/src/scrolling.rs:666-739"
  # ---- Test harness ----
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Test harness"
    change: "Register the input helper `Harness::scroll_into_view(&mut self, node_id: NodeId)` (packages/blitz-test-harness/src/input.rs:235-245): it calls `BaseDocument::scroll_into_view` with `ScrollBehavior::Instant` and `ScrollLogicalPosition::Nearest` on both axes, then pumps as every input helper does; it does not settle, and the crate's export list is unchanged — the name now denotes both the engine method and this helper; and move the neighbouring `apple_keybinding` citation `input.rs:227-232` -> `input.rs:228-233`."
    sidecar: "Test harness: input helper `Harness::scroll_into_view` registered; `apple_keybinding` citation moved by one line."
    rationale: "Report Changes > Symbols / APIs: `Harness::scroll_into_view(&mut self, node_id)` (new, input.rs:237); Harness / gate surface; Citation moves: every line of input.rs from :14 sits one line further down (`apple_keybinding` :229 -> :230); Sweep hazards: the name is now shared. Expected amendments 4. A new public harness API absent from the contract."
    basis: "packages/blitz-test-harness/src/input.rs:237"
  # ---- The six-check-files count, restated in Conventions and Existing Scopes ----
  - detector: D-arch-resources
    severity: warning
    section: "§Conventions → Tests"
    change: "Replace 'the six driver-action checks `stand_act_*`' with 'the nine driver-action checks `stand_act_*`'."
    sidecar: "Conventions/Tests: driver-action check files six -> nine."
    rationale: "Report Changes > Counts: driver-action check files `stand_act_*` 6 -> 9; `six driver-action` architecture 1 (:115). Sweep hazards: this 'six' is a file count, distinct from the verb count and the executor's cause count."
    basis: "tests/blitz-tests/tests/stand_act_scroll.rs:1-12"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: "Replace 'the six files `stand_act_ids` ... `stand_act_range`, 17 tests' with the nine files — adding `stand_act_disabled`, `stand_act_obstructed` and `stand_act_scroll` — 27 tests (ids 2 · diff, timer, keys, range unchanged · refused 3 · disabled 2 · obstructed 3 · scroll 4): add to the behaviours a not-enabled control refused and acted on once enabled, a covered target refused on an in-file fixture, and an out-of-view target refused `off-screen` then brought into view by `scroll`; add citations `stand_act_disabled.rs:1-8`, `stand_act_obstructed.rs:1-13`, `stand_act_scroll.rs:1-12` and move `stand_act_ids.rs:1-10` -> `:1-14`, `stand_act_refused.rs:1-7` -> `:1-9`."
    sidecar: "Existing Scopes/blitz-tests: `stand_act_*` six files / 17 tests -> nine files / 27 tests."
    rationale: "Report Changes > Files (New 4) and Counts: check files 6 -> 9, tests 17 -> 27; Expected amendments 4: `stand_act_` architecture 14 (:115, :263). Three new check files are unregistered in the scope row."
    basis: "tests/blitz-tests/tests/stand_act_obstructed.rs:1-13"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: "In the `session_common` clause replace 'by the six `stand_act_*`' with 'by the nine `stand_act_*`', and extend what it holds since 2026-10-07-refusal-detection: the `scroll(id)` call builder, `refused(session, call) -> Option<Cause>`, `refused_unchanged(session, call) -> (Option<Cause>, bool)` and `Acted.in_view: Option<bool>` beside the call builders, the outcome reader and the focus reader."
    sidecar: "Existing Scopes/blitz-tests: `session_common` read by nine `stand_act_*`; its new helpers listed."
    rationale: "Report Changes > Symbols / APIs > Test helpers (session_common/mod.rs :94, :107, :150, :193, :200); Counts: check files 6 -> 9. Second occurrence of the six-files claim in the same row."
    basis: "tests/blitz-tests/tests/session_common/mod.rs:94-207"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: "Extend 'scrolling and fragment navigation (tests/blitz-tests/tests/fragment_navigation.rs:1-2)' with the nested into-view check: `scroll_into_view` scrolling every scrolling box that holds its target, and the `visible_region` reader, 7 tests in both layout modes (tests/blitz-tests/tests/scroll_into_view_nested.rs:1-10)."
    sidecar: "Existing Scopes/blitz-tests: `scroll_into_view_nested` (7 tests) registered."
    rationale: "Report Changes > Files: new `tests/blitz-tests/tests/scroll_into_view_nested.rs`; Counts: 'blitz-tests integration files +4; `scroll_into_view_nested` holds 7 tests'. The file is unregistered in the scope row."
    basis: "tests/blitz-tests/tests/scroll_into_view_nested.rs:1-10"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → escher-driver"
    change: "Keep 'nine private modules' and extend two descriptions: session (`Session`, the label rule, the settled step `act`, the caller's time step `with_time`, and the crate-private bounded record of id text `SeenIds`), execute (`Session::run` and `Outcome`, the executor — it resolves a call's id on the screen and detects the five screen-level refusals before anything runs); the summary's 'the one in-process definition of the driver's verbs' now covers six verbs; move the citation `lib.rs:35-57` -> `lib.rs:43-65`."
    sidecar: "Existing Scopes/escher-driver: the id record and the executor's detection named; lib.rs citation moved."
    rationale: "Report Changes > Crates / modules: 'escher-driver (six files, no new module — still nine private modules)'; Symbols / APIs: `SeenIds` (session.rs:32), the detection in `Session::run`. Expected amendments 4 names this scope row."
    basis: "packages/escher-driver/src/lib.rs:43-65"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-dom: scrolling and selection"
    change: "Extend 'User and programmatic scrolling with animations' with: into-view scrolling through nested scrolling boxes, and the visible-region reader `visible_region` (packages/blitz-dom/src/scrolling.rs:666; packages/blitz-dom/src/scrolling.rs:749)."
    sidecar: "Existing Scopes/blitz-dom scrolling: nested into-view scrolling and `visible_region` named."
    rationale: "Report Changes > Crates / modules: `blitz-dom` (one file, scrolling.rs); Symbols / APIs: `visible_region` is a read-only reader, not a scroll — the row's description does not cover it. Expected amendments 4 names blitz-dom in Existing Scopes."
    basis: "packages/blitz-dom/src/scrolling.rs:749"
    dependent-of: D-arch-resources
  # ---- Occupied Resources: no new resource; two citations into an edited file moved ----
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem"
    change: "Move the citation `tests/blitz-tests/tests/session_common/mod.rs:194-202` -> `tests/blitz-tests/tests/session_common/mod.rs:232-240` (`state_dir`, `clear`); no resource claim changes — the chunk adds no file, directory or socket."
    sidecar: "Occupied Resources/Filesystem: session_common citation moved +38 lines; no resource added."
    rationale: "Report Changes > Citation moves: session_common/mod.rs architecture 4, 307 lines (was 269); Symbols / APIs 'no listener, port, env var, crate, binary or feature is added'. Citation-only: the registry entry is otherwise current."
    basis: "tests/blitz-tests/tests/session_common/mod.rs:232-240"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads"
    change: "Move the citation `tests/blitz-tests/tests/session_common/mod.rs:205-269` -> `tests/blitz-tests/tests/session_common/mod.rs:243-307` (`host_command`, `Host`, its `Drop`); no claim changes — the chunk starts no thread or process and the session's id record reads no clock."
    sidecar: "Occupied Resources/Process-wide state: session_common citation moved +38 lines; no state added."
    rationale: "Report Changes > Citation moves: session_common/mod.rs (`act` :142 -> :150; 307 lines, was 269). Citation-only: the registry entry is otherwise current."
    basis: "tests/blitz-tests/tests/session_common/mod.rs:243-307"

# Detector verdicts (for the orchestrator's read):
# D-arch-resources — VIOLATED. Unregistered: the `scroll` verb with `Command::Scroll`, `Outcome::Scrolled` and `in_view`; the executor's refusal detection and the session's id record; `BaseDocument::visible_region`; `Harness::scroll_into_view`; four new blitz-tests files. No new socket, port, env var, crate, binary or feature (report: Symbols / APIs 'Unchanged surfaces'; Crates / modules 'none added'), so §Occupied Resources and §Inherited Defaults need no new entry — only the two citation moves above.
# D-arch-decisions — one qualified decision ([Scrolling and input], founder-authorised via inputs#I4). No new library or runtime: report Dependencies 'none — no manifest and no lockfile moved'; §Stack is untouched. The [Driver session] decision holds: nothing of a call, an outcome or a refusal crosses the socket.
# Line numbers in `change` / `basis` were measured against the working tree at this pass (grep of the cited symbols); what changed is taken from the report's Changes section only. One figure not in the report: lib.rs gained 8 lines after old :29 (git diff hunk @@ -29,3 +29,11 @@), which moves architecture's lib.rs:33-57 / :35-57 citations to :41-65 / :43-65; lib.rs:13-19 in §Established Decisions → [Driver session] is unmoved.
# The report's Expected amendments 3 cites `Outcome::` at architecture ':132'; the hits are at :133 and :136 (:132 is Telemetry bootstrap).
```

## security-plan — the return (8; ids S1–S8 by position)

The return as collected, its comment lines kept in place.

```yaml
# security-plan drift — chunk 2026-10-07-refusal-detection
# D-security-auth: no drift — the report touches no identity / token / key / secret; "no listener, port, env var, crate, binary or feature is added" (report.md:32).
# D-security-deps: no drift — "Dependencies: none — no manifest and no lockfile moved" (report.md:37).
# D-security-input: the one new input (the `scroll` verb's `id`) IS validated (report.md:88, `validate`, kind Id, 1 to 1024 bytes) and stays in process (report.md:32) — no unvalidated boundary. The drift is §Input Validation's own text, which states a closed five-verb table, a not-found-only lookup and a session that keeps no id. Severity below is the detector's own; every item is a text reconcile the plan expected (report.md:80, expected amendment 5).
proposals:
  - detector: D-security-input
    severity: escalate
    section: >-
      §Input Validation → Driver command schema (escher-driver)
    change: >-
      Replace "a closed `'static` table of five verbs (`snapshot`, `click`, `type`, `press`, `advance`)" with a closed table of six verbs — `snapshot`, `click`, `type`, `press`, `advance`, `scroll` — where `scroll` takes one required `id` (kind `id`, 1 to 1024 bytes) checked by the same `validate` before anything runs; the five argument kinds, their bounds and `validate`'s order are unchanged; re-cite `validate` (command.rs:167, was :161), `VERBS` (schema.rs:290) and `Session::run` (execute.rs:82, was execute.rs:60-96).
    sidecar: >-
      2026-10-07-refusal-detection — Driver command schema row: verb table five → six (`scroll`, one required `id`, 1 to 1024 bytes, through `validate`); citations refreshed.
    rationale: >-
      report.md:20 (`Command::Scroll { id }` with its `spec` and `validate` arms; order, five argument kinds and bounds unchanged), report.md:22 (`VERBS` lists six; `scroll` takes one required `id`, kind `Id`, 1 to 1024 bytes), report.md:44 (verbs 5 → 6, `five verbs` security-plan :76 and :203), report.md:88 (validation mechanism present). The sixth verb is a boundary widening by the founder's own word (report.md:76, inputs#I2). Validation is present; the row's closed-set count and list are what is stale.
    basis: >-
      .andromeda/security-plan.md:76; packages/escher-driver/src/schema.rs:290; packages/escher-driver/src/command.rs:131, :167
  - detector: D-security-input
    severity: escalate
    section: >-
      §API Security → Settle and loads in flight
    change: >-
      Replace "none of its five verbs has an argument or a result field of a wait" with "none of its six verbs has an argument or a result field of a wait" (the sixth, `scroll`, takes one `id` and returns the five acting fields plus the flag `in_view`; a step that did not go quiet is still a result, not a refusal).
    sidecar: >-
      2026-10-07-refusal-detection — Settle and loads in flight row: driver verb count five → six.
    rationale: >-
      Second occurrence of the five-verb claim: report.md:44 names security-plan :203; report.md:21-22 (`Outcome::Scrolled { settled, busy, diff, in_view }`; `scroll` returns the five acting fields and `in_view`, no wait argument or field).
    basis: >-
      .andromeda/security-plan.md:203
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: >-
      §Input Validation → Driver command schema (escher-driver)
    change: >-
      Replace "an `id` is then looked up among the ids the screen reads now (the `id` row below) and an id that names no element is refused `not-found` with nothing run" with: for `click`, `type` and `scroll` the screen's snapshot is read and the target is on the screen when the snapshot lists the id and the first `element_ids()` pair with that id resolves through `get_node`; otherwise the call is refused with nothing run — `stale` when the session's record holds the id, `not-found` when not; a `click` or `type` on a target that is on the screen is then refused, one answer in this fixed order, `disabled` (the snapshot node's `enabled` is `Some(false)`), `off-screen` (the centre of its `bounds` lies outside `visible_region`, or that reads `None`), `covered` (the hit at that point never reaches the target); `scroll` applies none of the three and reports `in_view` instead of refusing; `press` and `advance` are refused for no screen-level cause; all eight causes are now returned by code (two by `validate`, six by the executor) and a refused `click` or `type` leaves the instance unchanged for each of the five detected causes (snapshot and both focus readings equal before and after).
    sidecar: >-
      2026-10-07-refusal-detection — Driver command schema row: executor refusals after `validate` are `stale` / `not-found`, then `disabled` → `off-screen` → `covered` for `click` and `type`; eight of eight causes returned by code.
    rationale: >-
      report.md:25-29 (the resolution and the fixed order), report.md:31 (eight of eight causes returned, was four), report.md:89 (runs after `validate`; a refused call dispatches nothing), report.md:134 (security criterion met for all five causes, both verbs). The row states `not-found` as the only id-level refusal.
    basis: >-
      .andromeda/security-plan.md:76; packages/escher-driver/src/execute.rs:82, :189-221 (grep-located — re-derive the range)
  - detector: D-security-input
    severity: escalate
    section: >-
      §Input Validation → Markup attributes · `id` (stable element id)
    change: >-
      In the act-by-id clause, replace "an id no pair carries, or a node that does not resolve, is refused `not-found` with nothing run and no panic" with: the target is on the screen when the screen's snapshot lists the id and the first `element_ids()` pair with that id resolves through `get_node` (never by index); otherwise the call is refused with nothing run and no panic — `stale` when the session's record of ids holds the id, `not-found` when not; re-cite the executor (execute.rs:82 for `run`; the old execute.rs:63, :108, :125-134 are positions in a file rewritten below its imports).
    sidecar: >-
      2026-10-07-refusal-detection — `id` row: an unresolved caller id is refused `stale` or `not-found` by the session's record; executor citations refreshed.
    rationale: >-
      Second occurrence of the not-found-only claim: report.md:25 (the `stale` / `not-found` split and the snapshot-lists-the-id condition), report.md:80 (sites: security-plan :116), report.md:85 (execute.rs rewritten below its imports, 346 lines, `run` at :82).
    basis: >-
      .andromeda/security-plan.md:116; packages/escher-driver/src/execute.rs:189-201 (grep-located)
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: >-
      §Input Validation → Driver command schema (escher-driver)
    change: >-
      Replace "reads no clock and keeps no id and no `NodeId` between calls" with: reads no clock and keeps no `NodeId` between calls, but the session now keeps a record of id text — a crate-private `SeenIds` in one private `Session` field, bounded at `MAX_SEEN_IDS = 4096` ids, least-recently-read forgotten first, empty at `Session::start`, living as long as the session, with no public reader and never printed or logged; it is fed only from snapshots a call takes after `validate` admits it (a call `validate` refuses, and an `advance` refused `time-unavailable`, record nothing), never from a call's own argument. State the bound as a count of ids, not bytes: the recorded ids are the app's own element ids, which `validate` never sees, so an id longer than 1024 bytes is recorded whole and nothing bounds the record's size in bytes (no test exercises it); this 4096 is unrelated to the `text` bound of 0 to 4096 bytes in the same row.
    sidecar: >-
      2026-10-07-refusal-detection — Driver command schema row: the session keeps a bounded record of id text (4096 ids, a count not a byte bound); "keeps no id" retired, "no `NodeId`" kept.
    rationale: >-
      report.md:30 (the record: `SeenIds`, `MAX_SEEN_IDS = 4096`, what feeds it, no reader, never printed), report.md:52 and report.md:100 (deviation 3: a count of ids, nothing bounds an id's length, the plan's "about 4 MiB" is not a property of the code), report.md:80 (sites `keeps no id` security-plan :76 and :116; the 4096 hazard), report.md:136 (no field holds a `NodeId`). The record is the operator's decision at the P4 forks (inputs#I2). The open byte bound is the part a human should rule on.
    basis: >-
      .andromeda/security-plan.md:76; packages/escher-driver/src/session.rs:25, :32, :81
  - detector: D-security-input
    severity: escalate
    section: >-
      §Input Validation → Markup attributes · `id` (stable element id)
    change: >-
      Replace "the lookup is made anew on every call — the session keeps no id and no `NodeId` between calls" with: the lookup is made anew on every call against the screen read then; the session keeps no `NodeId` between calls but does keep a bounded record of id text (at most 4096 ids, a count not a byte bound; private, no public reader, never printed or logged), read only to tell `stale` from `not-found`. Keep "no field of `Session`, `Outcome` or `Refusal` holds a `NodeId`" and "in process only, so it adds no crossing".
    sidecar: >-
      2026-10-07-refusal-detection — `id` row: "the session keeps no id" retired for the bounded record of id text; the no-`NodeId` and no-crossing statements stand.
    rationale: >-
      Second occurrence of the keeps-no-id claim: report.md:80 names `keeps no id` security-plan :116 and `anew on every call` :116; report.md:30 and :90 (the record holds text ids in process, no reader, never printed or logged — the silence probe reads 0); report.md:136.
    basis: >-
      .andromeda/security-plan.md:116; packages/escher-driver/src/session.rs:25-81
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: >-
      §Input Validation → Markup attributes · `id` (stable element id)
    change: >-
      Replace "the driver's `click` and `type`, through `Session::run`, take a caller-supplied id" with "the driver's `click`, `type` and `scroll`, through `Session::run`, take a caller-supplied id" (each already bounded at 1 to 1024 bytes by `validate`).
    sidecar: >-
      2026-10-07-refusal-detection — `id` row: `scroll` joins `click` and `type` as a verb that takes a caller-supplied id.
    rationale: >-
      The row names the id-taking verbs as two; report.md:22 (`scroll` takes one required `id`) and report.md:25 (the resolution applies to `click`, `type`, `scroll`). Same closed-set claim as the five-verb table, restated without its tokens.
    basis: >-
      .andromeda/security-plan.md:116
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: >-
      §Input Validation → Markup attributes · `id` (stable element id)
    change: >-
      Replace "the driver's four acting commands return the diff of their one settled step through `Session::run`" with "the driver's five acting commands (`click`, `type`, `press`, `advance`, `scroll`) return the diff of their one settled step through `Session::run`" — still in process, to the code that called `run`, nothing on the socket.
    sidecar: >-
      2026-10-07-refusal-detection — `id` row: diff-returning acting commands four → five (`scroll`).
    rationale: >-
      Verb count restated as "four acting": report.md:21 (`Outcome::Scrolled` holds `diff`), report.md:22 (`scroll` returns the five acting fields), report.md:28 (`scroll` runs as one settled step), report.md:32 (nothing of a call, an outcome or a refusal crosses the socket).
    basis: >-
      .andromeda/security-plan.md:116
    dependent-of: D-security-input
# Not proposed — no claim retired under these three detectors, listed so they are not lost:
# - §Error Handling, the `Refusal` bullet (security-plan.md:333; named by expected amendment 5, report.md:80): every statement still holds (eight causes, no `String` field, no path in a text, in process only — report.md:23, :135). Only its citations moved: refusal.rs grew 5 lines (report.md:85); `Refusal` now reads at refusal.rs:147-148, `Fault`'s derive at :118 (was cited :115-126, :144-147). The two reworded `off-screen` texts are quoted nowhere in security-plan.
# - Citation-only moves from report.md:85, claims unchanged: :75 `session.rs:11-52` (label check now session.rs:10-14 with `start` at :95-100); :82 `scrolling.rs:663-666` (fragment percent-decode now about :814-817, +151); :295 `session_common/mod.rs:195` (`env!("CARGO_TARGET_TMPDIR")` now :233). Unmoved: scrolling.rs:165-168, :570-572, :515-518 (all at or below :618).
# - `BaseDocument::scroll_into_view` widened for every caller, script `scrollIntoView` included (report.md:16): no new input class; nested writes go through `scroll_to`, whose clamp the Scrolling row (:125) already states. No security-plan claim is retired by it.
```

## layout-templates — the return (1; ids L1–L1 by position)

The return as collected, its comment lines kept in place.

```yaml
proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: desktop-native → IA notes"
    change: "Add one bullet: a programmatic into-view scroll (`BaseDocument::scroll_into_view`) scrolls every scrolling box on the target's containing-block chain, innermost first, then the viewport, and `BaseDocument::visible_region` reads the viewport narrowed to the padding box of each non-`visible`-overflow box on that chain; on the headless stand the nested scroller is the CRUD list — `crud-list` (24, 112.796875, 369 × 419.203125 at boot) is the box that clips its rows (after 12 Creates `crud-person-14`'s centre lies below the list's box with the viewport unscrolled), not `task-body` (0, 46, 800 × 554) and not the viewport; no stand markup, id, class or style changed (packages/blitz-dom/src/scrolling.rs:666; packages/blitz-dom/src/scrolling.rs:749; tests/blitz-tests/tests/stand_act_scroll.rs)."
    sidecar: "2026-10-07-refusal-detection: IA notes gain the stand's nested scroller (`crud-list` clips its rows; into-view scroll walks every scrolling box then the viewport; `visible_region` reader) — region was undocumented."
    rationale: "The report makes a scroll region inside the stand user-facing for the first time: `off-screen` is now read against `visible_region` (the viewport AND every scrolling box holding the target, report.md:17, :26) and the new `scroll` verb moves nested boxes (report.md:16, :28). layout-templates describes no such region — its only scroll facts are 'TaskShell is a header over a scrolling body' (:10) and the root-element scroller (:41); `crud-list` appears at :10 only as an id. The report's Expected amendment 9 names exactly this section and marks it carried (report.md:84). Hazard the wording must respect: the report says `#task-body` was NOT measured as a clipping box (report.md:84) — the bullet may state `task-body`'s bounds and that it is not the box clipping the CRUD row, never that `task-body` scrolls or clips; the baseline phrase 'over a scrolling body' at :10 is neither confirmed nor retired by this chunk, so no dependent proposal is raised against it. Also do not cite the scrolled list's own after-scroll `bounds` (shifted by its own scroll offset, report.md:70) — the boot reading is the one given."
    basis: "escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md:84 (expected amendment 9); report.md:16-17 (the two engine symbols, scrolling.rs:666 / :749); escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/stand-census.md:38-46 (the CRUD list measurement); .andromeda/layout-templates.md:10, :41 (the doc's only scroll claims)"

# Notes for the orchestrator (not proposals)
# - No other drift under D-layout-surface: the report adds no stand markup, no screen, no CLI binary/verb/usage line (report.md:32 "no listener, port, env var, crate, binary or feature is added"; :56 `scripts/agent-run.sh` unedited; :94 the three fixtures are in-file test fixtures, "not a UI surface of the stand"; :145 no file under `examples/seven_guis` touched). The driver's sixth verb `scroll` is an in-process call whose verb table layout-templates does not carry (the doc's only "verbs" are those of `agent-run.sh` and `cold-agent.sh`, both unchanged), so §Surface: cli needs nothing.
# - Sweep for a retired claim: none found. The chunk retires no statement of layout-templates; the single proposal is an addition, hence no `dependent-of` entries.
# - layout-templates carries no line citations into the files whose lines moved (`scrolling.rs`, `execute.rs`, `session.rs`, `input.rs`, `session_common/mod.rs`) — its one harness citation is `harness.rs:30-43` / `:57-62`, a file the report lists as untouched (report.md:13). No citation-move amendment is needed in this doc.
# - The plan's wording for this amendment (plan.md:455) also says the pinned viewport is scrolled "by one on a fixture"; the report's Changes do not state a measured viewport scroll on a fixture in so many words, so the proposed bullet omits it.
# - `stand_act_scroll.rs` is cited without line numbers: the report gives none for it.
```

## test-plan — the return (14; ids T1–T14 by position)

The return as collected, its comment lines kept in place.

```yaml
# drift-detector: test-plan · chunk 2026-10-07-refusal-detection
#
# Verdicts
#   D-tests-coverage     — no new path lacks a test (report "Coverage of new surfaces": every new symbol reads
#                          unit and/or integ, and the Outcome ran them green). The drift is test-plan's own
#                          coverage record, which still states the pre-chunk values; proposals 1-8 restate it.
#                          If the orchestrator reads this invariant strictly (missing tests only), 1-8 are
#                          record-keeping amendments, not a coverage gap.
#   D-tests-framework    — NO DRIFT. Runner unchanged: `cargo test --locked`, `scripts/agent-run.sh`,
#                          `.github/scripts/ci-leg.sh`, libtest; report Dependencies "none", Dev-tool versions "none".
#   D-tests-obs-harness  — the 5-command contract, status shape and log format are untouched (`agent-run.sh`
#                          unedited, no CI file moved). What moved is the in-process harness (one input helper)
#                          and the silent check set that test-plan §3 and obs-plan §3 both enumerate; proposals
#                          9-14 are test-plan's side, to be applied with obs-plan.md:69 so it is not one-sided.
#
# Measured in the tree for `basis` only (not in the report; re-derive before applying):
#   - `mod session_common;` readers: 14 (was 11) · `mod common;` readers: still 14 — grep `^mod (common|session_common);`
#     over tests/blitz-tests/tests/stand_*.rs
#   - escher-driver `#[test]` per file: error 3 · session 3 · wire 8 · refusal 3 · schema 4 · command 5 · execute 1 = 27
#   - test-module ranges: session.rs:157-203 · refusal.rs:191-319 · schema.rs:297-470 · command.rs:243-594 ·
#     execute.rs:316-346 (error.rs and wire.rs unedited)
#   - input.rs: `impl Harness` :96, `scroll_into_view` :237, `ime` :248, file 252 lines
#   - session_common/mod.rs: 307 lines; host half starts :222 (`READY`), `SOCKET_FILE` :228, `state_dir` :232, `clear` :237
#
# Not proposed, flagged for the orchestrator:
#   - "eight causes" (test-plan.md:21, :142) is a sweep hit in the report but the count stays eight — no change.
#   - test-plan.md:115 and :324 say "the six `stand_act_*` files" inside dated "re-counted at 2026-10-07-act-by-id"
#     entries: true of that chunk, left as history; proposals 7 and 8 append a new re-count instead.
#   - test-plan.md:26's `stand_act_ids` clause "not empty for the 8 controls a click changes at boot, empty for
#     the other 7" is NOT restated by the report (a click on a disabled control is now refused, not an empty
#     diff). Proposal 4 does not assert a new split; it needs a reading of the file before that clause is kept.
#   - The CI finding (apt-get step, no timeout, no retry) is a finding on an unedited file, not a change: no §9 proposal.

proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: >-
      State 27 inline unit tests in seven files (was 26): session.rs 3 (adds a table test that the session's record holds the ids read and forgets the one read longest ago, bound 4096 ids — a count of ids; no test bounds an id's byte length), schema.rs 4 with "the six verbs are named in order" (was five; `scroll` the sixth, result field `in_view`), command.rs 5 with admitted rows 28 (was 26), malformed rows 35 (was 34) and a six-row verb table; the executor's verbs and its refusals are covered by the nine `stand_act_*` integration files (was six); citations move to session.rs:157-203 · refusal.rs:191-319 · schema.rs:297-470 · command.rs:243-594 · execute.rs:316-346; add "the count of 27 at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md".
    sidecar: "§1 escher-driver: 26 → 27 unit tests (session.rs 2 → 3, the id record), five → six verbs, command rows 26/34 → 28/35, six → nine stand_act_* files, citations moved (2026-10-07-refusal-detection)."
    rationale: >-
      Report Counts: "escher-driver unit tests 26 → 27", "admitted rows 26 → 28, malformed rows 34 → 35, table rows 5 → 6", "Driver verbs 5 → 6", "stand_act_* 6 → 9"; Coverage of new surfaces: the record "tests unit (session.rs, one table test)"; Deviation 3: nothing bounds an id's length and "No test exercises it"; Citation moves list all five files. The tests exist — the body's record of them is stale.
    basis: ".andromeda/test-plan.md:21 · packages/escher-driver/src/session.rs:181 · report.md:44-46"

  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: >-
      schema.rs pins the verb table as six verbs in a stated order (was five); session.rs also pins the session's record of ids — read ids held, the one read longest ago forgotten past 4096, a count of ids with no byte bound tested; command.rs's admitted and malformed tables read 28 and 35 rows; "the nine `stand_act_*` files drive it through a `Session` (§5)" (was six), where nine mutation controls of this chunk each turned a named check red then green (escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/controls.md), beside the five of act-by-id; same citation moves as §1 (session.rs:157-203 · refusal.rs:191-319 · schema.rs:297-470 · command.rs:243-594 · execute.rs:316-346).
    sidecar: "§4 escher-driver: verb table five → six, session.rs record test added, six → nine stand_act_* files, nine mutation controls cited, citations moved (2026-10-07-refusal-detection)."
    rationale: >-
      Same retired claim as §1's ("five verbs", "the six `stand_act_*` files", the old test-module line ranges) restated in §4; report sweep names test-plan `five verbs` at :21 and :142, and Outcome: "nine mutations recorded red then green — met (evidence/controls.md)".
    basis: ".andromeda/test-plan.md:142 · report.md:44, :148"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-proof)"
    change: >-
      Read "the nine driver-action files, `Session::run` in each, both layout modes — `stand_act_ids` 2 · `stand_act_diff` 6 · `stand_act_timer` 4 · `stand_act_refused` 3 · `stand_act_keys` 2 · `stand_act_range` 1 · `stand_act_disabled` 2 · `stand_act_obstructed` 3 · `stand_act_scroll` 4 (27)" (was six files, 17); "of the crate's other 14, 12 pin the command and refusal schema, 1 the executor's key table and 1 the session's record of ids" (was other 13); add "the nine driver-action files and the crate's 27 at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md"; add the witness: the fork's CI run 37673662374 on `983d8973` (16 of 16 jobs green at its third attempt; in the macOS and windows job logs `scroll_into_view_nested` 7 · `stand_act_disabled` 2 · `stand_act_obstructed` 3 · `stand_act_scroll` 4 · `stand_act_ids` 2 · `stand_act_refused` 3 · `stand_act_keys` 2 · `fragment_navigation` 19 read as run, 0 failed; MSRV, iOS and android read by conclusion only).
    sidecar: "session-proof: six → nine driver-action files (17 → 27 tests), crate's other 13 → 14, CI run 37673662374 on 983d8973 added as the cross-platform witness (2026-10-07-refusal-detection)."
    rationale: >-
      Report Counts: "stand_act_* 6 → 9; their tests 17 → 27 (ids 2 · diff, timer, keys, range unchanged · refused 2 → 3 · disabled 2 · obstructed 3 · scroll 4)", site named "key file session-lifecycle.md:11"; Cross-project: CI#37673662374, the windows and macOS job logs read at this report.
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11 · report.md:47, :59-60"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: >-
      In the driver-calls clause: `stand_act_ids` 2 no longer reads "a driver `click` naming it is accepted" for each of the 77 nodes — 70 of 77 are accepted and 7 refused per layout mode, both row counts asserted, the four `task-header-spacer` nodes reading `covered`; `stand_act_refused` is 3 (was 2: `stale` for an id the session's record holds, `not-found` for one it does not, each leaving the snapshot and both focus readings equal); add `stand_act_disabled` 2 (`disabled` on `crud-delete` and `flight-return-date`, acted on once enabled), `stand_act_obstructed` 3 (`covered` on an in-file fixture, the cover above by an explicit `z-index`, a transparent overlay giving no `covered`, the click landing once the cover is dismissed) and `stand_act_scroll` 4 (`off-screen` on a CRUD row past the list's box, then `scroll` returning `in_view` true and a second `scroll` an empty diff, on the stand and on tall and unreachable fixtures; the list's box read from its `bounds` before the scroll); and add the engine check `scroll_into_view_nested` 7 (every scrolling box on the target's chain scrolled innermost first then the viewport, and `visible_region`, both layout modes); cite the four new files and add "as measured at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md".
    sidecar: "§1 tests/blitz-tests: stand_act_ids 77 accepted → 70 accepted · 7 refused, stand_act_refused 2 → 3, +stand_act_disabled 2 · stand_act_obstructed 3 · stand_act_scroll 4 · scroll_into_view_nested 7 (2026-10-07-refusal-detection)."
    rationale: >-
      Report Files "New (4)"; Counts: "Stand nodes a driver click is accepted on 77 → 70 of 77 per layout mode", "scroll_into_view_nested holds 7 tests"; Deviation 1 (seven refused rows, `task-header-spacer` covered) and Deviation 2 (the pre-scroll bounds reading); Outcome v010-11 and the design criteria. The body's "is accepted" for all 77 is now false and the four new test files are absent from §1.
    basis: ".andromeda/test-plan.md:26 · report.md:11, :47, :50-51, :98-99, :131"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session ↔ held instance"
    change: >-
      List the acting verbs as `click` · `type` · `press` · `advance` · `scroll`, the first two and `scroll` addressed by stable id alone; a refused call reads any of the eight causes — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `off-screen`, `covered`, `time-unavailable` — and for each of the five the executor detects in front of `click` and `type`, a refused `click` and a refused `type` leave the snapshot and both focus readings equal; `scroll` is refused for none of the three screen checks and answers `in_view`; attribute to "the nine `stand_act_*` files, §1" (was six) and add this chunk's report as the measure, with the limit that `stand_act_scroll` compares against the list's pre-scroll `bounds` because a scrolled box's own `bounds` shift by its scroll offset.
    sidecar: "§5 Session ↔ held instance: verb list gains `scroll`, refused causes four → eight, refused click and type per detected cause, six → nine stand_act_* files (2026-10-07-refusal-detection)."
    rationale: >-
      Report sweep: the verb list written out sits at "test-plan.md:179, key file session-lifecycle.md:5"; Symbols: "Which causes code returns now: eight of eight", `VERBS` lists six; Outcome (security): "a refused click and a refused type leave the instance unchanged for each detected cause — met (all five causes, both verbs)"; Spec claims disproved (the bounds bullet) and Deviation 2.
    basis: ".andromeda/test-plan.md:179 · report.md:22, :31, :44, :70, :134"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-start)"
    change: >-
      Restate the `run` clause: for `click`, `type` and `scroll` the target is on the screen when the snapshot lists the id and it resolves to an element, otherwise the call is refused with nothing run — `stale` when the session's record of ids holds it, `not-found` when not (was "one that names no element is refused `not-found`"); in front of `click` and `type`, one answer in the order `disabled` · `off-screen` · `covered`; each acting verb (`click` · `type` · `press` · `advance` · `scroll`) is one settled step returning `Outcome::Acted`, `Outcome::Advanced` or `Outcome::Scrolled` (the last with `in_view`); citations move — `Session` session.rs:81, `start` :95, `with_time` :116, `act` :146; `Outcome` execute.rs:15, `run` :82.
    sidecar: "session-start: lookup answers `stale` or `not-found` from the session's id record, the disabled · off-screen · covered order, sixth verb `scroll` with `Outcome::Scrolled`, citations moved (2026-10-07-refusal-detection)."
    rationale: >-
      Report Symbols (`Session::run`): the screen read, `stale` vs `not-found`, the fixed order, `scroll` applying none of the three checks; `Outcome::Scrolled { settled, busy, diff, in_view }`; Citation moves: session.rs `:27 → :81`, `:40 → :95`, `:60 → :116`, `:90 → :146`, execute.rs "`run` is at `:82`, `Outcome` at `:15`". The key file's verb list and its `not-found` rule restate the claim §5 retires.
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:5 · report.md:21, :24-28, :85"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: >-
      Append: re-counted at 2026-10-07-refusal-detection: `run stand`'s `run.end` reads passed 107 · failed 0 · ignored 3 (+10: `stand_act_refused` +1, `stand_act_disabled` 2, `stand_act_obstructed` 3, `stand_act_scroll` 4 — the three new files picked up by their `stand_` prefix with no script change; the selection lists 29 files) — as measured at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md.
    sidecar: "Agent-run Proof: run stand re-counted 97 → 107 passed, 26 → 29 files (2026-10-07-refusal-detection)."
    rationale: >-
      Report Counts: "Stand check files 26 → 29; agent-run.sh run stand 97 → 107 passed, 0 failed, 3 ignored (basis: the gate log's run.end). `26 files`: test-plan 1 (:115)"; Harness / gate surface: "`scripts/agent-run.sh` is unedited — the three new `stand_*` files are selected by their prefix". The prior entries stay as dated history.
    basis: ".andromeda/test-plan.md:115 · report.md:48, :56, :164"
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Local baseline (`cargo test --workspace`)"
    change: >-
      Append: re-counted at 2026-10-07-refusal-detection: 153 result lines, 647 passed · 0 failed · 8 ignored (+10 in the `stand_act_*` files — three new result lines, `stand_act_disabled` 2 · `stand_act_obstructed` 3 · `stand_act_scroll` 4, and +1 in `stand_act_refused`; +7 `scroll_into_view_nested` in one new result line; +1 `escher-driver` unit test, the session's record of ids, in the crate's existing lib result line), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs at /implement and in its operator pass on the tree its pre-CI commit 983d8973 carries (escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md).
    sidecar: "§9 Local baseline: workspace re-counted 149 → 153 result lines, 629 → 647 passed, 8 ignored (2026-10-07-refusal-detection)."
    rationale: >-
      Report Counts: "Workspace 149 → 153 result lines, 629 → 647 passed, 0 failed, 8 ignored (basis: target/ci-logs/test.log of the fast leg, counted by script, at implement and again at the operator pass). `149 result` and `629 passed`: test-plan 1 each (:324)"; the +18 decomposes as 10 + 7 + 1 from the same Counts bullets.
    basis: ".andromeda/test-plan.md:324 · report.md:46-50, :165"
    dependent-of: D-tests-coverage

  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: >-
      The driver's call builders gain `scroll(id)`; `act(session, call)` returns `Acted { settled, busy, diff, advanced_ms, in_view }` (`in_view: Option<bool>`, set by the `Outcome::Scrolled` arm); add `refused(session, call) -> Option<Cause>` and `refused_unchanged(session, call) -> (Option<Cause>, bool)` (the cause, and whether the snapshot and both focus readings are equal before and after), the latter shared by four files; "so do the nine driver-action checks `stand_act_*` (14 readers)" (was six, 11 readers); cite session_common/mod.rs:1-307 (was :1-269) and add this chunk's report for the refusal helpers and the fourteen readers. Apply together with obs-plan §3 (obs-plan.md:69: "six driver-action checks", "eleven of those twelve") so the two harness contracts still name the same silent check set.
    sidecar: "§3 session checks: +`scroll`, `refused`, `refused_unchanged`, `Acted.in_view`; six → nine driver-action checks, 11 → 14 readers; session_common/mod.rs 269 → 307 lines (2026-10-07-refusal-detection)."
    rationale: >-
      Report Symbols (Test helpers): "`scroll(id)` (:94), `refused` (:193), `refused_unchanged` (:200), `Acted.in_view: Option<bool>` (:107), the `Outcome::Scrolled` arm of `act` (:150)"; Counts: "`six driver-action`: architecture 1 · test-plan 5 (test-plan.md:99, :100, key file) · obs-plan 1 (:69)"; Citation moves: "session_common/mod.rs … 307 lines, was 269". test-plan §3 and obs-plan §3 both enumerate this set; a one-sided update is the detector's drift.
    basis: ".andromeda/test-plan.md:100 · .andromeda/obs-plan.md:69 · tests/blitz-tests/tests/session_common/mod.rs:94-206 · report.md:33, :47, :85"

  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: >-
      Read "three of the nine driver-action checks — `stand_act_ids`, `stand_act_diff` and `stand_act_keys` — do, 14 in all" (was "three of the six"); the count 14 stands, since none of the three new `stand_act_*` files declares `mod common;`.
    sidecar: "§3 stand checks: three of the six → three of the nine driver-action checks read common/; 14 readers unchanged (2026-10-07-refusal-detection)."
    rationale: >-
      Report Counts: "Driver-action check files `stand_act_*` 6 → 9", site "test-plan.md:99". The 14 is also obs-plan §3's figure (obs-plan.md:69, "14 of which read the shared module") and stays in agreement.
    basis: ".andromeda/test-plan.md:99 · report.md:47"
    dependent-of: D-tests-obs-harness

  - detector: D-tests-obs-harness
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      Read "`tests/session_common/mod.rs`, read through `mod session_common;` by 14 checks — four `stand_session_*.rs`, `stand_settle.rs`, which declares it and not `mod common;`, and the nine `stand_act_*.rs`, three of which (`stand_act_ids`, `stand_act_diff`, `stand_act_keys`) declare both" (was 11 checks, six files); "read through `mod common;` by 14 `stand_*.rs` checks" stands.
    sidecar: "§2 Directory pattern: session_common readers 11 → 14, six → nine stand_act_* files (2026-10-07-refusal-detection)."
    rationale: >-
      Second statement of the reader count §3 retires; report Counts: "stand_act_* 6 → 9" and Files "New (4)" under tests/blitz-tests/tests.
    basis: ".andromeda/test-plan.md:65 · report.md:11, :47"
    dependent-of: D-tests-obs-harness

  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (`Harness`) → Input helpers"
    change: >-
      Add `scroll_into_view` to the helper list and state: `scroll_into_view(node_id)` calls `BaseDocument::scroll_into_view` with `ScrollBehavior::Instant` and `ScrollLogicalPosition::Nearest` on both axes, then pumps like every input helper — none settles — and has no check of its own: it is exercised only through the driver's `scroll` (`stand_act_scroll`); the crate's export list is unchanged; citations move to input.rs:96-252 (was :95-239), :19-28 (was :18-27) and :76-94 (was :75-93).
    sidecar: "§3 Input helpers: +`Harness::scroll_into_view` (pumps, does not settle; exercised through the driver's `scroll` only); input.rs citations moved (2026-10-07-refusal-detection)."
    rationale: >-
      Report Symbols: "`Harness::scroll_into_view(&mut self, node_id)` (new, input.rs:237) — an input helper … then pumps. The crate's export list is unchanged"; Harness / gate surface: "it pumps after its call, like every input helper; no helper settles"; Coverage: "tests integ (through the driver's `scroll` only; no check of its own)"; Citation moves: "input.rs … every line from :14 sits one line further down". The body's helper list is closed and omits it.
    basis: ".andromeda/test-plan.md:90 · packages/blitz-test-harness/src/input.rs:237 · report.md:19, :56, :93"
    dependent-of: D-tests-obs-harness

  - detector: D-tests-obs-harness
    severity: warning
    section: "§2 Test Strategy → Test levels observed → Process-lifecycle checks"
    change: >-
      Move the citation `tests/blitz-tests/tests/session_common/mod.rs:184-269` to `:222-307` (the host half — the bounds, `SOCKET_FILE`, `state_dir`, `clear`, `host_command`, the `Host` guard — sits 38 lines further down); no claim of the bullet changes.
    sidecar: "§2 Process-lifecycle checks: session_common/mod.rs citation :184-269 → :222-307 (2026-10-07-refusal-detection)."
    rationale: >-
      Report Citation moves: "session_common/mod.rs — … test-plan 4 (`act` :142 → :150; 307 lines, was 269)". Same file-extent claim the session-checks bullet retires, cited at a second site.
    basis: ".andromeda/test-plan.md:49 · tests/blitz-tests/tests/session_common/mod.rs:222 · report.md:85"
    dependent-of: D-tests-obs-harness

  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Session lifecycle (session-state)"
    change: >-
      Move the citation `tests/blitz-tests/tests/session_common/mod.rs:190-202` to `:228-240` (`SOCKET_FILE`, `state_dir`, `clear`); no claim of the row changes.
    sidecar: "session-state: session_common/mod.rs citation :190-202 → :228-240 (2026-10-07-refusal-detection)."
    rationale: >-
      Report Citation moves: session_common/mod.rs grew 269 → 307 lines with the refusal helpers inserted above the host half; third site of the same moved extent, in the keyed contract.
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:8 · tests/blitz-tests/tests/session_common/mod.rs:228-240 · report.md:85"
    dependent-of: D-tests-obs-harness
```

## obs-plan — the return (8; ids O1–O8 by position)

The return as collected, its comment lines kept in place.

```yaml
# obs-plan drift pass, chunk 2026-10-07-refusal-detection
# No detector's invariant is broken in the code; all 8 proposals correct obs-plan text the chunk made stale (the report's expected amendment 7, plus two moved citations).
# D-obs-instrumentation: sections 4-6 require no span, metric or log of the new operations. The driver crate stays silent by contract and its per-command span is still owed by the route entry "Driver command spans" (report.md:88-90). The widened BaseDocument::scroll_into_view and the new visible_region add no tracing call (report.md:91-92), and section 4's blitz-dom "observed absent" rows still hold.
# D-obs-stack: no dependency, subscriber or logger was added (report.md:37, :32); packages/escher-telemetry is untouched (report.md:13). The contract key bootstrap-phases-derive-for-route-setup-project (otel-sdk-install, pii-scrubbing-wire) was read and is unaffected.
# D-obs-pii: holds, no escalation. Nothing is printed or logged, the silence probe reads 0 (report.md:139-140, :160), a Refusal is built from a Cause alone (report.md:89), and no scrub or redaction shape changed (report.md:39). The section 8 line-290 statement that no command reaches a sink-installing host stands (report.md:32).
# I read the code only to locate moved lines and count readers for the `basis` fields; every change is taken from the report's Changes section.
proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage (the escher-driver bullet, obs-plan.md:105)"
    change: >-
      Replace "four of them returned by code today (`unknown-verb` and `malformed` by `validate`, `not-found` and `time-unavailable` by the executor) and four by nothing yet" with "all eight returned by code today (`unknown-verb` and `malformed` by `validate`; `not-found`, `stale`, `disabled`, `covered`, `off-screen` and `time-unavailable` by the executor)". The rest of the bullet stands: no span, the span owed by the route entry "Driver command spans".
    sidecar: >-
      2026-10-07-refusal-detection: §4 — the refusal-cause value domain is now returned by code eight of eight (was four of eight); two by `validate`, six by the executor.
    rationale: >-
      report.md:31 ("Which causes code returns now: eight of eight ... was four of eight") and report.md:45 name obs-plan :105 as the one site of "four of them returned". report.md:72 corrects the plan's "six of the eight" wording: six is the executor's share, all eight are returned.
    basis: ".andromeda/obs-plan.md:105 · escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md:31"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage (the escher-driver bullet, obs-plan.md:105 — its provenance tail)"
    change: >-
      Keep "the executor and the two causes it returns at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md" (true of that chunk) and append ", the six causes the executor returns and the `scroll` verb at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md".
    sidecar: >-
      2026-10-07-refusal-detection: §4 — provenance added for the executor's six causes and the sixth verb.
    rationale: >-
      The tail restates the retired claim (the executor returns two causes) as the bullet's measurement source. Left alone, it reads as the basis of the corrected "eight of eight" (report.md:31).
    basis: ".andromeda/obs-plan.md:105"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage (the escher-driver bullet, obs-plan.md:105 — its source citation)"
    change: >-
      Citation `packages/escher-driver/src/command.rs:161` becomes `packages/escher-driver/src/command.rs:167` (`pub fn validate`).
    sidecar: >-
      2026-10-07-refusal-detection: §4 — citation moved, command.rs:161 → :167 (`validate`), after the `Command::Scroll` variant and its arms.
    rationale: >-
      report.md:85 lists the citation move: "`command.rs` — ... obs-plan 1 (`validate` and below shifted by 5 to 22 lines)". Line 161 was `pub fn validate` at the base 9b758f6c; it is at :167 now.
    basis: "packages/escher-driver/src/command.rs:167"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (the headless-stand bullet, obs-plan.md:69)"
    change: >-
      Replace "for the six driver-action checks `stand_act_*`" with "for the nine driver-action checks `stand_act_*`". The three added — `stand_act_disabled`, `stand_act_obstructed`, `stand_act_scroll` — install no subscriber, read no env var and print nothing, and the same holds for the new engine check `scroll_into_view_nested`.
    sidecar: >-
      2026-10-07-refusal-detection: §3 — the silent set's driver-action checks 6 → 9 (`stand_act_disabled`, `stand_act_obstructed`, `stand_act_scroll`); `scroll_into_view_nested` joins the silent check files.
    rationale: >-
      report.md:47 ("Driver-action check files `stand_act_*` 6 → 9 ... `six driver-action`: ... obs-plan 1 (`:69`)"). report.md:140 says every new or edited check file installs no telemetry, reads no env var and prints nothing. report.md:82 is the expected amendment for the new check files in the silent set.
    basis: ".andromeda/obs-plan.md:69 · escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md:47"
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (the headless-stand bullet, obs-plan.md:69 — the shared-module clause)"
    change: >-
      Replace "the module eleven of those twelve read — four of the five `stand_session_*`, `stand_settle` and the six `stand_act_*`" with "the module fourteen of those fifteen read — four of the five `stand_session_*`, `stand_settle` and the nine `stand_act_*`".
    sidecar: >-
      2026-10-07-refusal-detection: §3 — readers of `session_common/mod.rs` 11 of 12 → 14 of 15 (the three new `stand_act_*` files read it).
    rationale: >-
      This is the second occurrence of the "six `stand_act_*`" claim on the same line (report.md:82 counts `stand_act_` 3 at obs-plan :69). The three new files use the shared helpers (report.md:33, :101 — `refused_unchanged` shared by four files). Measured: 14 files under tests/blitz-tests/tests declare `session_common` — 4 `stand_session_*`, `stand_settle`, 9 `stand_act_*`.
    basis: "grep -l session_common tests/blitz-tests/tests/*.rs → 14 files; .andromeda/obs-plan.md:69"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (the headless-stand bullet, obs-plan.md:69 — its provenance tail)"
    change: >-
      Keep "the executor and the six `stand_act_*` checks at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md" (true of that chunk) and append ", the refusal detection, the `scroll` verb, the session's record of ids and the three further `stand_act_*` checks at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md".
    sidecar: >-
      2026-10-07-refusal-detection: §3 — provenance added for the three further `stand_act_*` checks, the sixth verb and the record of ids.
    rationale: >-
      This is the third occurrence of "six `stand_act_*`" on line 69. It is a historical measurement source, so it is kept and extended, not rewritten. Without the addition the nine-file claim has no cited measurement (report.md:47, :140).
    basis: ".andromeda/obs-plan.md:69"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (the headless-stand bullet, obs-plan.md:69 — the escher-driver parenthesis)"
    change: >-
      After "and nothing in the crate prints, logs or fields any of the four" add: "the session's record of ids — the crate-private `SeenIds` behind one private `Session` field, at most `MAX_SEEN_IDS` = 4096 id texts (a count of ids, not of bytes; an id's length is unbounded there), fed only from the session's own snapshots, with no public reader and no `Debug` — is likewise never printed, logged or fielded". Name `scroll` among the verbs the silent executor runs, with `Outcome::Scrolled` (its diff and the flag `in_view`) under the existing `Outcome` `Debug` statement.
    sidecar: >-
      2026-10-07-refusal-detection: §3 — the session's bounded record of id text (`SeenIds`, 4096 ids) added to the values the driver crate holds and nothing prints or logs; the sixth verb `scroll` named in the silent executor.
    rationale: >-
      report.md:30 ("is never printed or logged ... has no public reader"), report.md:90 (PII n/a, the silence probe reads 0) and report.md:82 (expected amendment 7: "the record among the values nothing prints"). The hazard at report.md:100 and :120: this 4096 is a count of ids, unrelated to `MAX_TEXT_BYTES`, and no byte bound exists. D-obs-pii holds, so this is a completeness amendment at warning, not an escalation.
    basis: "packages/escher-driver/src/session.rs:25 (MAX_SEEN_IDS) · :32 (SeenIds, no derive) · :85 (the field); report.md:30"
  - detector: D-obs-stack
    severity: warning
    section: "§9 CI Integration → Telemetry artifact handling (the Session state directory row, obs-plan.md:325)"
    change: >-
      Citation `tests/blitz-tests/tests/session_common/mod.rs:194-202` becomes `tests/blitz-tests/tests/session_common/mod.rs:232-240` (`state_dir` and `clear`). The §3 line-69 citation `session_common/mod.rs:1-6` is unmoved and stays.
    sidecar: >-
      2026-10-07-refusal-detection: §9 — citation moved, session_common/mod.rs:194-202 → :232-240 (the file grew 269 → 307 lines).
    rationale: >-
      report.md:85 lists the citation move: "`session_common/mod.rs` — ... obs-plan 2 (`act` `:142 → :150`; 307 lines, was 269)". Of obs-plan's two citations into that file, the module-doc range `:1-6` is unmoved. The state-directory range sat at :194-202 at the base 9b758f6c and now sits 38 lines lower.
    basis: "tests/blitz-tests/tests/session_common/mod.rs:232-240"
    dependent-of: D-obs-stack
```

## a11y-plan — the return (1; ids Y1–Y1 by position)

The return as collected, its comment lines kept in place.

```yaml
# a11y-plan drift pass — chunk 2026-10-07-refusal-detection
# Detector verdicts, read against the report's Changes section only:
#   D-a11y-surface    — literal invariant NOT violated: the chunk adds no interactive UI element. No stand markup,
#                       id, class or style was touched (report.md:94, :145); the three in-file fixtures are test
#                       fixtures, each acted-on element carries an author id and `unkeyed_actionable` is asserted
#                       empty (report.md:94, :142).
#   D-a11y-obs-schema — NOT violated: no a11y violation schema exists yet (a11y-plan.md:87) and the report moves no
#                       obs log schema ("No config key, no migration, no scrub or redaction shape", report.md:39).
# One stale claim stands in a11y-plan anyway, and the report's own Expected amendment 8 names it (report.md:83).
# It falls outside both detectors' literal invariants; it is carried below under D-a11y-surface as the nearest
# one (the §7 statement of the driver's acting surface). The orchestrator should decide whether that attribution
# is acceptable or whether the amendment belongs to the expected-amendments channel instead.
# Sweep for other occurrences of the retired claim ("nothing yet detects `disabled`"): one site only.
#   grep over a11y-plan.md for nothing yet / named and worded / refus* / off-screen / five verbs /
#   scroll_into_view / visible_region / not-found / `stale` / `covered` / click point / border box
#   -> hits on line 270 only. The one key file (bootstrap-phases-derive-for-route-setup-project.md) states
#   nothing about the driver or refusals; no Change touches it.
# Checked and left alone (still true per the report): §1 :14 (the executor reads the snapshot; no manifest moved),
#   §2 :37 (settle neither reads nor drains the changed set), §7 :270 "the driver's schema defines no role or name
#   set of its own" and the quoted `disabled` meaning text (only `off-screen` texts were reworded, report.md:23),
#   §9 (the a11y leg keeps its three files, green, report.md:167).

proposals:
  - detector: D-a11y-surface
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output (the snapshot-model bullet, a11y-plan.md:270)"
    change: "Replace 'named and worded only, with nothing yet detecting it' with: the driver detects it on this same reading — for `click` and `type`, `Session::run` refuses `disabled` when the snapshot node's `enabled` is `Some(false)`, first of the three screen-level checks (`disabled`, then `off-screen`, then `covered`), with nothing dispatched and the snapshot and both focus readings equal before and after; on the stand `crud-delete` and `flight-return-date` are refused `disabled` and the same control is acted on once it reads enabled (packages/escher-driver/src/execute.rs:82; tests/blitz-tests/tests/stand_act_disabled.rs) — as measured at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md. Keep the quoted meaning text and its command-and-refusal-schema provenance, and keep the following clause on the schema defining no role or name set of its own."
    sidecar: "2026-10-07-refusal-detection: §7 — the driver's `disabled` refusal cause is now detected (snapshot `enabled` = `Some(false)`, for `click` and `type`), no longer 'named and worded only'."
    rationale: "Not a literal violation of either detector's invariant (no interactive UI element added, no schema moved); it is a stale §7 claim the chunk retired. The report's Changes state `Session::run` now answers `disabled` from the snapshot node's `enabled` (report.md:26), that code returns eight of eight causes where it returned four (report.md:31), and that the a11y coverage of the new detection is '`disabled` read from the snapshot's `enabled`; focus restoration asserted after every refusal' (report.md:89). Expected amendment 8 names exactly this site: 'a11y-plan §7 (the `disabled` cause is detected from the snapshot's `enabled`) — carried … Sites: `named and worded only` a11y-plan 1 (`:270`)' (report.md:83). The acceptance lines confirm it met (report.md:131, :141)."
    basis: ".andromeda/a11y-plan.md:270 (the stale claim, one occurrence); escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md:26, :83, :89"
```
