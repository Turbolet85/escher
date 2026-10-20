# P5 review — 2026-10-20-large-screen-in-parts · Large screen in parts

Plan: `escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/plan.md` (15 steps · 22 gate entries · 10 acceptance
criteria) · research: `escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/research.md` · controls:
`.andromeda/runs/2026-10-20T09-07-29-phase/p5-controls.md`

## Goal
An agent reads a screen whose snapshot text is over the 10,000-byte budget in parts. On the founder's word (inputs#I2 —
F1 «Аргумент part, части по умолчанию (Рекомендую)», F2 «Пустая часть (Рекомендую)», his own answers relayed by the
overseer):
- `snapshot` gains an optional `part` argument, a new argument kind bounded 1 to 1000.
- Over the budget with no argument, `snapshot` answers part 1, as `{"text","part","parts"}`.
- A part past the last answers the empty text.
- Inside the budget, `snapshot` answers exactly as today.
A part is a run of whole lines of `to_text`, so the parts in order are the whole text, each node's line exactly once.
There is no new verb, cause, exit, carrier or span field.

## Top acceptance criteria
1. On the over-budget fixture, in both layout modes, through `Session::run`:
   - `snapshot` answers part 1 of ≥ 2, and every part is within the budget;
   - the parts concatenate to the whole text, every id on exactly one line;
   - a part past the last is the empty text;
   - an id read only in the last part is scrolled to and clicked (`stand_snapshot_parts`).
2. Every stand screen's `snapshot` is unchanged: `stand_snapshot_text` and `stand_replay` pass unedited, and the
   committed Counter checks re-record byte-equal.
3. `part` 0, 1001, negative, `i64::MAX` and a text value are refused `malformed`, with nothing run and nothing typed
   written back.
4. A password sentinel occurs 0 times across a walk, and exactly one line reads `focused` after a click.
5. A part read's `command` span holds `verb="snapshot"` alone. `host_log` and `mcp_log` read 0 needles at `trace`.

## Touchpoints
- dioxus-native-dom `snapshot_text.rs` / `lib.rs`: `Snapshot::text_parts` → `Vec<TextPart>`.
- escher-driver:
  - `schema/spec.rs`: `ArgKind::Part`;
  - `schema.rs`: `MAX_PART`, the argument, two count fields, help;
  - `command.rs`: `Command::Snapshot { part }`;
  - `execute.rs`: `Outcome::Part`;
  - `json.rs`, `mcp/listing.rs`, `cli.rs` and their tests, `lib.rs`;
  - `mcp.rs` unchanged — a part reaches its generic JSON arm.
- Tests:
  - new `stand_snapshot_parts.rs` (in process and `serve_mcp`);
  - `stand_act_spans`, `session_common`;
  - seven_guis `cli_commands`, `host_log`, `mcp_log`.

## Test commands (fence)
- Unit gates: dioxus-native-dom `--lib`; escher-driver.
- Integration gates: `stand_snapshot_parts` (red baseline: no target yet); `stand_act_spans` + `stand_snapshot_text` +
  `stand_replay` + `stand_mcp_tools`; seven_guis `cli_commands` + `host_log` + `mcp_log` + `recording`.
- Probes: two silence censuses, the `snapshot_text.rs` site count (6), the span sites (2), the manifest/stand guard,
  the write-set guard and the untracked guard. Every guard and census was controlled both sides in a scratch clone.
- Legs: `agent-run boot` / `run stand` (baseline 192 · 0 · 7), `ci-leg.sh a11y` / `fast` / tally / `doc`.
- Operator: hygiene, the push, `ci.py` green and the per-OS tallies (control fired on CI#39387656399).

## Leans the plan states (decisive, each with its artifact)
- **The cut is whole lines.** `to_text` writes each line's indent itself (`snapshot_text.rs:45-47, :54-56`), so
  parts concatenate to the whole text.
- **A part read records the whole screen's ids**, as `snapshot` and `lint` do. `resolve` records the whole screen
  before every acting call (`execute.rs:378-382`), so a part-only record would move no `stale` or `not-found` reading.
- **A paged answer is a new `Outcome::Part`.** The 14 `Outcome::Screen` patterns stand unchanged (graph query 2).
- **The part bound is 1 to 1000**, a screen of about 10 MB of text. A screen past it is a stated limit.

## Concretization preview — verification-matrix.json#v020-09 (method: integration)
- **Current acceptance:** "On a fixture whose screen exceeds the 10,000-byte snapshot budget, an agent reads it part by
  part, each part within the budget, and the parts together hold every node of the whole screen exactly once."
- **Proposed concrete form:** On the `stand_snapshot_parts` fixture page, whose `Snapshot::to_text` passes
  `SNAPSHOT_TEXT_BUDGET`, in both layout modes and through `Session::run` — the path every carrier runs:
  - `snapshot` with no argument answers part 1 with `parts` ≥ 2;
  - `snapshot` with `part` 1..=`parts` answers each part's text, each at most 10,000 bytes;
  - the texts concatenated in order equal the whole `to_text()`, and every node id of the whole snapshot is on exactly
    one line across the parts;
  - `part` = `parts` + 1 answers the empty text;
  - an id read only in the last part is accepted by `scroll` then `click`;
  - the same walk through `serve_mcp` returns the same JSON lines.
- This narrows nothing: "an agent reads" becomes the driver path every carrier runs, and "exactly once" is asserted
  per id.

## Expected amendments (wrap) — for the record
architecture (Dioxus DOM bridge; Driver session) · security-plan §Input Validation · test-plan §3 Session lifecycle ·
obs-plan §4 · design-system §Surface: cli · layout-templates §Surface: cli · a11y-plan §7. The driver-session and
security-plan ones carry playbook «Boundary widening», answered by the founder's own word (inputs#I2), so the wrap halts
on it once, as the answer card.

## Beyond the plan
| # | Proposal the word approves that plan.md and the ledger writes will not hold | Owner / place |
|---|---|---|
| 1 | The stated limit that an acting verb's diff and a lint's findings are not paged: on a large screen either can pass 10,000 bytes, held only by the 1,048,576-byte answer bound | a `CARRY:` on the existing entry «Capability statement», written by the wrap's route-resolve |
