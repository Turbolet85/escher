# Scope — 2026-10-26-codebase-health

**Working entry (verbatim, escher-0.3.0/working-route.md:11):** Codebase health — the lint's findings walk and two
table-asserting tests under the complexity ceiling; test functions counted, engine function untouched (v030-01; Epoch 5
audit M1). No freight block stands on it (`route.py pins`: freight blocks 0).

**Cap:** v030-01 «Codebase health». Requirement: escher's three functions over the cognitive-complexity ceiling (the
lint's findings walk and two table-asserting tests) fall under it as a table walk or named helpers. Test functions stay
in the count, the engine function stays as it is, and every check stays green. Acceptance (as routed, `method:
integration`): on the code audit's own complexity reading, the three escher-written functions the Epoch 5 audit found
over the ceiling of 15 read at or under it. The two tests are still counted and assert the same rows, the engine
function is left as it was, and every check is green.

**The founder's word:** M1 «Код escher, тесты таблицами (Рекомендую)», chosen by question dialog on 2026-10-26
(inputs#I1). `lint.rs::findings` and the two test functions are factored into table walks or helpers. The engine
function `clear_interaction_state_for_removed_node` is left as is. Test functions stay in the count. He passed over
«Всё, включая функцию движка» and «Тесты убрать из счёта». The operator relayed the same label at take-up (inputs#I2).
None of it is a delegate answer.

**The finding:** the Epoch 5 code audit (`.andromeda/runs/2026-10-25T21-43-01-code-audit/proposals.md` M1, evidence
`c-over-ceiling.json`) reads `complexity.over_ceiling` (functions over cognitive 15, rust-code-analysis 0.0.25)
84 → 87 → 88, worse at both of the last two diffs. Three of the real crossings are escher-written.

## What this chunk builds
- **`packages/escher-driver/src/lint.rs::findings`** — the visual lint's per-node walk, cognitive 16 at
  `lint.rs:247` (re-verified at take-up: `fn findings(harness: &Harness<DioxusDocument>, screen: &Snapshot) ->
  Vec<Finding>` stands at line 247). It is brought to at most 15 by moving its branch ladder into named helpers or a
  table walk. Its output does not change: the same `Finding`s in the same order (document order, then `Defect::ALL`
  order), the same six defects, the same `fonts-unavailable` refusal before anything is read, and nothing changed on
  the held instance.
- **`packages/escher-driver/src/mcp_tests.rs::the_listing_is_the_twelve_verbs_of_the_table`** — cognitive 18 at
  `mcp_tests.rs:399` (re-verified at take-up: line 399). It asserts the MCP `tools/list` answer against `VERBS` row by
  row inline. It is brought to at most 15 as a table walk or with a named helper, asserting the same rows: twelve
  tools in `VERBS` order with `lint` after `screenshot` and `help` last, each described by its help and its result
  fields.
- **`tests/blitz-tests/tests/stand_requirement_sweep.rs::every_0_2_0_requirement_names_its_witness_inside_stand_repeat`**
  — cognitive 18 at `stand_requirement_sweep.rs:1016` (re-verified at take-up: line 1016). It is brought to at most 15
  as a table walk or with a named helper, asserting the same rows: each 0.2.0 requirement names its witness inside
  stand-repeat.
- **Boundaries.**
  - Test functions stay in the population and are not excluded from the count.
  - `packages/blitz-dom/src/document.rs::clear_interaction_state_for_removed_node` (17) is not touched.
  - No other over-ceiling function is in scope: the upstream engine's offenders and `mutator.rs`'s `<anonymous>`
    closure stay as they are.
  - No behaviour, verb, field, carrier, log site or span changes.
  - The refactor changes no test's set of assertions.
- Every helper or table walk a factoring introduces itself reads at or under 15 on the same reading. The move does not
  shift the creep into a new function, so `complexity.over_ceiling` on the audit's population reads at most 85 at this
  chunk's HEAD (88 less the three), with no new entrant. [verified at P3: the population re-derived at HEAD reads 88
  over the audit's 429 files. The reading aggregates a function's nested spaces: `findings` 16 over 3 spaces, each
  closure 0. A branch ladder moved into a closure or a nested `fn` stays counted in its parent, so the helpers must be
  module-level items. `overlaps` (13) is the nearest neighbour in `lint.rs`. research.md §Patterns detected, §Sweeps]
- "The code audit's own complexity reading" is rust-code-analysis 0.0.25's per-function cognitive sum, every space of
  kind function (closures included), threshold `> 15`, as the audit fired it
  (`.andromeda/runs/2026-10-25T21-43-01-code-audit/record.json:685`: `rust-code-analysis-cli -m -O json -o {dir} -p …`
  over its `srcfiles.txt`). [verified at P3: `rust-code-analysis-cli 0.0.25` is installed at `~/.cargo/bin`. The recipe
  re-reads 16 · 18 · 18 · 17 for the three functions and the engine function at HEAD `19f9ff91ca`, equal to the
  audit's]
- "Asserting the same rows" is shown by each test still passing and still failing on the row it guarded. A recorded
  control mutates one guarded fact, runs the named test alone, reads it red at an assertion and restores the bytes by
  sha256. Examples: the sweep test with a `--test` name removed from `ci-leg.sh` (the 2026-10-24 chunk's own control,
  `escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/evidence/red-before-green.md`), and the listing test with
  one tool's written description or one argument's bound changed. [verified at P3: the evidence form exists, and each
  test's assertion set is listed in research.md §Files inspected]
- "Every check green": the two touched tests pass in both builds the workspace runs. `stand_lint` and escher-driver's
  lint unit tests pass unchanged on `lint.rs`. `bash .github/scripts/ci-leg.sh fast` and
  `bash .github/scripts/ci-leg.sh doc` pass. `stand_requirement_sweep` is a blitz-tests target held by
  `.github/scripts/test_blitz_tests_targets.py`, and its name and target stay as they are. [verified at P3: the name
  sweep finds the two test names pinned by text in `.andromeda/test-plan-detail/test-scope-summary.md`,
  `.claude/docs/services/escher-driver.md` and `escher-0.3.0/intent.md`, and they stay unchanged. 38 line citations
  into the three files stand in the masters, and re-pointing them is the wrap's citation sweep (research.md §Sweeps).
  `mcp_tests.rs` runs 15 tests on unix (16 `#[test]`, one under `#[cfg(not(unix))]`)]

## CI verdict read at Setup (5a)
Every sha from the last wrap's flip (`5e15b78d64`) through HEAD, read through `ci.py conclusion`
(`.andromeda/runs/2026-10-26T23-36-49-phase/ci-no-marker.json`, read before the marker was minted):
- `19f9ff91ca95` green · checks 17/17 · wall 1719 s · CI#40389488295
- `7914a166e100` green · checks 17/17 · wall 1076 s · CI#40384982560
- `5e15b78d64c2` green · checks 17/17 · wall 1144 s · CI#40232260282

No red and no `not green`, so nothing is dispositioned.
