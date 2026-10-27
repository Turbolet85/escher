# Codebase Research — 2026-10-26-codebase-health

## Scope
- **Depth:** moderate · **Reads:** 9 (`lint.rs` 200–605, `mcp_tests.rs` 380–493, `stand_requirement_sweep.rs` 960–1091,
  the audit's `proposals.md`, `c-over-ceiling.json`, `over_ceiling.py`, the cookbook, `verification-harness.md`, the
  2026-10-24 sweep chunk's `evidence/red-before-green.md` head) · **Globs/Greps:** 6 (the name sweep, the line-citation
  sweep, two test-attribute counts, the audit's recipe lookup, the function locations)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full, 41 lines; 1 Session Addition,
  about a boot smoke under `timeout`, which does not apply: this chunk drives no live leg, only `cargo test` and the
  `ci-leg.sh` legs) · `.claude/rules/testing.md` (loaded with the touched test paths; its 2026-10-09 rule (restore by
  writing the saved bytes back, compare sha256) and its 2026-10-10 rule (search `tests/` and the owning crate's `src/`
  for a table's readers) apply to the controls below)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the founder's 0.3.0 scope, M1 «Код escher, тесты таблицами (Рекомендую)»: the
  three functions factored, the engine function left as is, test functions kept in the count · `inputs#I2` — the
  operator's take-up directive (snap I1 verbatim, the forced sample at P2, product forks answered by the operator as
  the founder's delegate and marked PROVISIONAL). No fact was read outside this repository at P3.

## Files inspected
- `packages/escher-driver/src/lint.rs` (200–605) — `Session::lint` (221–230) refuses `fonts-unavailable` before
  anything is read, feeds the id record from its own snapshot, then calls `findings` (247–295). `findings` builds the
  id → `NodeId` map, flattens the snapshot (`flatten`, 305–330) and pushes the `overlaps` pairs (334–372). Each entry
  with an element then reads zero-size (or skips on), off-screen or clipped by a `match` with a guard, then truncated
  and contrast, and finally `ordered` (299–302) sorts by (entry index, `Defect::rank`). The per-defect readers
  `holds_content` (469–479), `truncated` (483–504) and `contrast` (507–524) are already private named functions.
  Two inline unit tests: `the_six_defects_are_named_in_order` and `findings_follow_document_order_then_the_defect_order`
  (the latter calls `ordered` directly).
- `packages/escher-driver/src/mcp_tests.rs` (380–493) — `the_listing_is_the_twelve_verbs_of_the_table` (399–479): the
  outer shape (one object, `tools` only, `VERBS.len()` tools, 12, `help` last), then per row zipped with `VERBS`:
  name; description equals `description(verb)` and starts with the help; each result field named; the schema's type,
  `additionalProperties`, property keys sorted, the written order of properties, `required`; and per argument the
  property's type, description, `enum` (key names or app names, else absent) and `minimum`/`maximum` (`MAX_MILLISECONDS`
  or `MAX_PART`, else absent). Every assertion message names `row {row}` only. `listed(verb)` (≈372–396) is an existing
  helper.
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` (960–1091) — `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
  (1016–1090): `WITNESSES_020` holds 9 rows in v020-01…09 order; `ci-leg.sh` (read through `source(CI_LEG)`) holds the
  `stand_*.rs` glob and `--test {name}` for every name of `STAND_REPEAT_NAMED` and `STAND_REPEAT_SEVEN_GUIS`; per row, a
  witness is named, v020-01/02 name `Unit` witnesses alone and every other row names one stand-repeat runs; per witness,
  its file is carried and holds `fn {test}(`, and a non-`Unit` witness's target is inside stand-repeat (a blitz-tests
  `stand_` file or a named target, or a seven_guis named process check). Its sibling `every_requirement_names_its_witness`
  (979–1013) reads 0.1.0's `WITNESSES` and is under the ceiling.
- `.andromeda/runs/2026-10-25T21-43-01-code-audit/{proposals.md, c-over-ceiling.json, over_ceiling.py, record.json:685}`
  — M1's evidence and its recipe: `rust-code-analysis-cli -m -O json -o {dir} -p …` over `srcfiles.txt`, every space of
  kind function (closures included), `cognitive.sum`, over the ceiling when `> 15`.
- `escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/evidence/red-before-green.md` (head) — the house form of a
  row-guard control: one mutation at a time by a scratchpad script, the named test run alone and read red at an
  assertion (no `error[` line), the bytes written back and compared by sha256.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-26T23-36-49-phase/tree-query-2026-10-26-codebase-health.json`, rust plane, rebuilt 8866 nodes)
- **`findings`** — 1 caller: `Session::lint` at `packages/escher-driver/src/lint.rs:229`. The signature
  `fn findings(&Harness<DioxusDocument>, &Snapshot) -> Vec<Finding>` is kept, so nothing threads.
- **`ordered`** — called by `findings` (`lint.rs:294`) and by the inline test at `lint.rs:596`. Kept as it is.
- **`flatten` · `overlaps` · `truncated` · `contrast` · `holds_content`** — each called from `findings` alone (`flatten`
  also by itself, recursively). These are private, so the blast radius is the module.
- The two test functions have no callers (libtest entry points). Their names are pinned by text: see Sweeps.

## Patterns detected
- **The complexity reading aggregates nested spaces** (measured with `$TMPDIR/{scratchpad}/cx_tree.py` on
  rust-code-analysis 0.0.25, re-derived at HEAD): a function's `cognitive.sum` is its whole subtree's.
  - `findings` reads 16 over 3 spaces (average 5.33), the listing test 18 over 2 (9.0) and the sweep test 18 over 3
    (6.0). Each closure under them reads 0.
  - So the 16, 18 and 18 come from each body's own nested control flow.
  - A closure or a nested `fn` holding branches counts toward its parent. Only a sibling item (a module-level function,
    or a `const` table) moves cognitive out of a function.
  - This is the equality the plan's mechanism needs: moving a branch ladder into a module-level helper lowers the
    parent by that ladder's nesting increments, and the helper is counted on its own against 15.
- **Prior complexity splits took private named helpers with no behaviour change**
  (`packages/dioxus-native-dom/src/element_id.rs`'s `record_roots` · `walk_component` · `child_chain`, 2026-10-07; the
  range and button arms, 2026-10-16) — arch-history.
- **Table assertions carry the row index only** (`mcp_tests.rs:413-477`, every message `"row {row}"`;
  `stand_requirement_sweep.rs:1047-1088`, `"row {row}, witness {index}: …"`) — a helper keeps that message form.
- **`overlaps` sits at 13** (`lint.rs:334`) — the nearest function to the ceiling in `lint.rs`. A factoring of
  `findings` that moves code into it would cross.

## Conventions to follow
- **Private helpers, no new public item**: `lint.rs`'s helpers are all private `fn`s with a `///` line
  (`lint.rs:297-302`, `374-524`); escher-driver's root re-exports nothing new (arch §Conventions → Visibility).
- **`#[track_caller]` on assertion helpers** (`.claude/rules/testing.md` §File and naming conventions): a factored row
  assertion reports its caller's line.
- **A helper with one reader stays in its check's file** (arch §Conventions → Tests; `tests/blitz-tests/tests/common/mod.rs`
  is for shared helpers). A new top-level file under `tests/blitz-tests/tests/` would be a new target, which
  `.github/scripts/test_blitz_tests_targets.py` refuses.
- **No print, tracing, env read or `unwrap` on a call's input in `lint.rs`** — security · obs extracts; the silence census
  probe `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`.

## New files to create
- none

## Files to modify
- `packages/escher-driver/src/lint.rs` — `findings`'s per-entry defect readings moved into private module-level helpers (or a table walk), its output unchanged
- `packages/escher-driver/src/mcp_tests.rs` — `the_listing_is_the_twelve_verbs_of_the_table`'s per-row and per-argument assertions moved into private module-level helpers asserting the same rows
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` — `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`'s per-row and per-witness checks moved into private module-level helpers asserting the same rows

## Sweeps
- `the_listing_is_the_twelve_verbs_of_the_table` over tracked files (`git grep -l -F`, `.andromeda/runs` and chunk
  folders aside): 1 hit · 1 changed (`mcp_tests.rs`, the function itself) · 0 no-change; one more in
  `escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md`, a frozen record, no change (the name is kept).
- `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (same command): 4 hits · 1 changed
  (`stand_requirement_sweep.rs`) · 3 no-change (`.andromeda/test-plan-detail/test-scope-summary.md`,
  `.claude/docs/services/escher-driver.md`, `escher-0.3.0/intent.md` — they name the test, whose name is kept).
- `WITNESSES_020` · `STAND_REPEAT_NAMED` (same command): 16 and 3 non-chunk hits · 1 changed (`stand_requirement_sweep.rs`)
  · the rest no-change — the tables are read, never edited, and keep their names and rows.
- Line citations into the three files (`command grep -rnoE '(lint\.rs|mcp_tests\.rs|stand_requirement_sweep\.rs):[0-9]+'`
  over `.andromeda/*.md`, the `-detail/` dirs, `.andromeda/registries`, `.claude/docs`, `docs`, `CLAUDE.md`):
  - 38 hits · 0 changed by this chunk · 38 no-change here. A factoring moves lines, and re-pointing those citations is
    the wrap's citation sweep (masters are read-only to phase and implement).
  - The hits are in `test-plan-detail/{unit-test-strategy,test-scope-summary}.md`, `security-plan.md:295`,
    `obs-plan.md:93`, `layout-templates.md:54`, `architecture-detail/{standard-contracts,existing-scopes}.md`,
    `a11y-plan-detail/visual-design-verification.md` and `a11y-plan-amendments.md:419`.
- The population over the audit's list (`$TMPDIR/{scratchpad}/cx_total.py .andromeda/runs/2026-10-25T21-43-01-code-audit/srcfiles.txt`,
  429 of 429 files read): over 15 = 88 at HEAD `19f9ff91ca`, re-derived, equal to the audit's 88.
  - Escher-written functions on the list: the three of M1 and five that predate the 84 baseline and are not M1's
    crossings: `cells.rs:71 tokenize` 22, `cli.rs:139 read` 20, `execute.rs:184 execute` 22, `mcp.rs:215 run` 18,
    `stand_accessibility_ids.rs:256 every_interactive_role_element_reads_a_name` 16.
  - Those five are out of the founder's M1 and out of this chunk.
- The four functions at HEAD (`$TMPDIR/{scratchpad}/cx.py` over the four files), re-derived: `findings` 16 · the listing
  test 18 · the sweep test 18 · `clear_interaction_state_for_removed_node` 17 (`document.rs:1007`, not touched).

## Open questions
- none
