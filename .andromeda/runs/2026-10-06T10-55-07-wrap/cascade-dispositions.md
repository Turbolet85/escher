# Cascade dispositions — 2026-10-06-id-persistence

The search: `cascade.py sweep` over `cascade-patterns.toml`, against the baseline 06529554 (the pre-CI parent). It
covers the seven masters, `.andromeda/registries/**`, the three curation homes, playbook/drift-base and the leaf
bodies.

Patterns, each with a known-positive control that fired on the pre-pass masters:
- `key-i` · `key `{i}`` — the CRUD row keyed by its list index (control layout-templates.md:10)
- `index-people` · `index in the (full )?people list` — the same claim in other words (control layout-templates.md:10)
- `no-println` · `no `println!`` — "no println! in the stand or its checks" (control obs-plan.md:68)
- `crud-41-119` · `crud.rs:41-119` — the re-pointed citation (control layout-templates.md:10; 0 rows after the pass)
- `stand-17` · `17 `ok` | run stand 17 | 17 stand` — the superseded `run stand` count (control test-plan.md:111)
- `ws-441` · `441 (passed|·)` — the superseded workspace count (control test-plan.md:314)
- `re-exec` · `current_exe | re-execut | child_emits` — the re-exec spawn claim, its mechanism verbs included (control test-plan.md:314)

Dropped: `row-index-key` (`(rows?|row key)…(list index|by its index|positional)`). Its control never fired on the
pre-pass masters, so the tool refused the call. `key-i` and `index-people` cover the claim's wording at its one master
site.

Sections read beside the sweep: architecture §Standard Contracts → Dioxus DOM bridge (line 134, read whole — 3 950
chars) and §Occupied Resources → Process-wide state and threads (line 148, read whole); security-plan §Input Validation
`id` row (107); layout-templates §Surface: desktop-native (10); obs-plan §3 Logging stack (68, with 83 and 220 for the
same claim — 83 "stand checks install no escher sink" still true; 220 "test diagnostics go to stdout/stderr via
println!" agrees); test-plan §1 (25), §3 Proof (111), §9 (314).

## Rows
- `.claude/docs/services/seven_guis.md:36` key-i · leaf → **re-derived** (row key = person id, set on the `for` item).
- `.claude/docs/services/seven_guis.md:36` index-people · leaf → **re-derived** (same line).
- `.andromeda/obs-plan.md:68` no-println · new → **amended**. The surviving `no `println!`` is the rewritten claim with its one exception (the re-exec child's captured stdout) — true as written.
- `.andromeda/test-plan.md:111` stand-17 · standing → **no change**. It is the dated re-count "at 2026-10-06-stable-element-ids: 17" inside an append-only chain; the pass appended "at 2026-10-06-id-persistence: 20" after it.
- `.andromeda/test-plan.md:314` ws-441 · standing → **no change**. It is the dated re-count "at stable-element-ids: 441 passed"; the pass appended the 444 · 0 · 5 re-count.
- `.claude/docs/tests-summary.md:28` ws-441 · leaf → **re-derived**. The current figure is now 444 · 0 · 5 at id-persistence; 441 kept as dated history.
- `.andromeda/architecture.md:148` re-exec ×3 (@c2217, 2318, 2357) · new → **amended** (this pass's registration; the three matches are `re-execute`, `current_exe()`, `child_emits` in the one new clause).
- `.andromeda/test-plan.md:25` re-exec (@c863) · new → **amended** (this pass's §1 coverage clause).
- `.andromeda/test-plan.md:314` re-exec · standing → **no change** ("+1 ignored `child_emits`" in the dated telemetry-bootstrap re-count; true).
- `.andromeda/obs-plan.md:68` re-exec · new → **amended** (this pass's clause).

Curation homes: 0 rows on every pattern. Judgment bases (playbook, drift-base): 0 rows.

## Leaves re-derived (step 3)
The table floor plus provenance. Grep of leaves for element_id / stable / CRUD / run stand / 441 / println / spawn /
current_exe / persist / remount / stand_element_ids.
- architecture (§Standard Contracts → Dioxus DOM bridge · §Occupied Resources):
  - `CLAUDE.md` `GENERATED:setup:architecture`: persistence added to the stable-element-id clause.
  - `.claude/docs/services/dioxus-native-dom.md`: persistence on the stand; the keyed-list-in-`if` gotcha.
  - `.claude/docs/services/seven_guis.md`: stand file list + row key.
  - `.claude/docs/commands.md`: the stand test command gains `--test stand_id_persistence`.
  - `GENERATED:setup:modules` and `:warnings`, read: unchanged, still true. No docs leaf carries the Process-wide list.
- test-plan:
  - `.claude/docs/tests-summary.md`: the stand coverage clause and the §9 baseline.
  - `.claude/rules/testing.md` and `.claude/rules/verification-harness.md`, read: no count, file list or claim this pass changed.
- obs-plan:
  - `.claude/docs/obs-summary.md` and `.claude/rules/observability.md`, read: no "no println! in the stand checks" claim (observability.md:18 says the stand "and its checks install none" — a subscriber, still true).
- security-plan:
  - `.claude/docs/security-summary.md`, read: no `id`-row restatement.
  - `.claude/rules/security.md`, read: its cold-agent crossing line is unaffected; the re-exec is not a security-plan registration.
- layout-templates:
  - `.claude/docs/design-summary.md:27`, read: no row-key claim; unchanged.

Lateral binds: test-plan §3 ↔ obs-plan §3 — the harness and log format are unchanged on both sides (obs-plan's edit is
§3 Logging stack's stand bullet, which test-plan §3 does not restate). a11y ↔ obs schema — untouched.
