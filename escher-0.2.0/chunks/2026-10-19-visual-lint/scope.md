# Scope — 2026-10-19-visual-lint · Visual lint

Working entry (escher-0.2.0/working-route.md, Epoch 4 — Visual lint and real-app readiness), verbatim:
«Visual lint — one command: overlap, clipping, truncated text, off-screen, zero size, text contrast as `stand_contrast`
reads it; stable id, measured reason; fixture-proven (v020-05)»

Capability: v020-05 (requirement: one command reports, for the current screen, each element that overlaps another, is
clipped, has truncated text, lies off-screen, has zero size or fails text contrast, naming its stable id and measured
reason — intent §Findings 4, E1, chosen by the founder on 2026-10-16 as intent records). Matrix acceptance as
routed: on a fixture that plants each of the six defects, one lint command names every planted element by its stable id
with its measured reason, and on the stand at boot it reports no defect the stand's recorded contrast and layout readings
do not hold.

Freight on the entry: none (no CARRY / PREREQ / CONTEXT / WATCH / BLOCKED-ON at an annotation position).

## What it builds
- One command that lints the current screen of a held instance and returns its findings: each element that overlaps
  another, is clipped, has truncated text, lies off-screen, has zero size, or fails text contrast.
- Each finding names the element's stable element id and its measured reason (the defect's class and the reading that
  shows it).
- Text contrast is read as `stand_contrast` reads it — `Harness::text_contrast`, the WCAG 2.x ratio against
  `TextContrast::required_ratio` (3.0 large text, else 4.5).
- Proof on a fixture that plants each of the six defects (fixture-proven), and on the stand at boot (no defect the
  stand's recorded readings do not hold — `stand_contrast`'s rows, the stand's layout).

## Closed at P3 (research.md)
- Whether "one command" is a new driver verb is the founder's fork, not a fact research can settle. The table admits
  `lint` as an eighth instance verb (`schema.rs:432-434`), and every surface reads it from `VERBS`: `Session::run`, the
  command line, the MCP tools and `help`. A new verb, and what its answer carries out, widens the boundary. It goes
  into the run dir's `founder-forks.md` and is asked by dialog (inputs#I1).
- The command can be read-only like `screenshot`: no step, no settle, nothing dispatched, no time moved
  (`execute.rs:226-250`, `record_result`'s early return at `execute.rs:263`).
- The elements linted are the snapshot's nodes, each carrying its stable element id (`snapshot.rs:96-189`). Each
  finding's id is one a `snapshot` of the same screen lists.
- [premise-corrected: `visible_region` reads `overflow` alone over the containing-block chain, never the element's own
  clip or `contain: paint` (scrolling.rs:618-633, 744-780); no text-extent reader exists] `zero size`, `off-screen`
  and `overlap` build on the snapshot's `bounds`. `off-screen` and `clipped` build on `visible_region` as built,
  which sees `overflow` clips only. A `contain: paint` clip is outside it, a technical fork. `contrast` builds on
  `Harness::text_contrast`, already a dependency (`execute.rs:5-7`). `truncated text` needs a new read-only blitz-dom
  reader of an inline root's laid-out text extent against its content box (`inline.rs:870-905`), which changes no
  engine behaviour.
- Overlap and zero size are measured on the stand at boot (research.md §Measured). Zero pairs of elements, neither
  holding the other, intersect with positive area, so "boxes intersect, neither holds the other" reads no false
  overlap. Five nodes have no area: `task-header-spacer` ×4 (0 high) and `timer-progress` (0 wide at 0 % elapsed).
  What `zero size` and `truncated text` report on such cases is a technical fork with a stand consequence.
- The command's span (`escher_driver` / `command`) records `verb` and, when refused, `cause`. A count of findings
  would be a ninth field, a new log field and the founder's fork (inputs#I1).
- [premise-corrected: the recorded set is read from the table by `Level::Instance` (recording.rs:92, 425), not a
  list of seven] A new instance verb joins recording and `Check::from_bytes` with no code change. Leaving it
  unrecorded would need code. Its findings at rest in a check would be ids, closed words and numbers.
- [premise-corrected: no dioxus-native-dom or blitz-test-harness change is needed (bounds and `text_contrast` are
  reused); the cold-agent pipe hard-codes the eleven verb names (scripts/cold-agent.sh:48,
  .github/scripts/test_cold_agent.py:23)] Touchpoints are listed in research.md §New files / §Files to modify:
  escher-driver, one new blitz-dom reader, blitz-tests' fixtures, session helpers and a new check, seven_guis' CLI,
  MCP and log checks, and the cold-agent pipe's verb names. The plans' driver sections are wrap's amendments.

## Decided at P4 (inputs#I2)
- F1, the command's carriage, answered by the operator as delegate «In process only (Most conservative)», PROVISIONAL
  until the founder's own word. This chunk builds the lint in process (`Session::lint`, its types, a read-only
  blitz-dom text extent reader) with its fixture and stand proofs. Nothing crosses a carrier. v020-05's "one command"
  is claimed gated on the founder's F1 word, its carriage the gated criterion (plan step 12).
- F2: no new span field. T1: `zero-size` only where the element holds content. T2: `truncated` is text cut from view,
  a text control's value never measured. T3: the lint itself walks `contain: paint` clips.

## Boundaries
- No defect is fixed by this chunk; the lint reports. No engine behaviour changes for any document.
- `covered` and `off-screen` as per-action refusals stay as they are; the lint is a whole-screen reading beside them.
- Load wait, `select` and the large screen in parts are later entries of this epoch, not this chunk.

## CI read at take-up (Setup 5a)
- e97a7f41b10a (2026-10-19-shared-fixtures, the last wrap's flip and HEAD): CI#39343307796 in progress — verdict not
  yet available (4/4 checks, the oldest `Test [default features]` running 242 s at 2026-10-19T23:47Z). Not folded,
  not read as green.
