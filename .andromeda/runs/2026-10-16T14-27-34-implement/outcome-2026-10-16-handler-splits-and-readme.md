# Implement outcome — 2026-10-16-handler-splits-and-readme

**Outcome: green** (smoke skipped: no boot-path change). The gate block is 15/15 green on its first full firing, 1
iteration. The three `leg = 'operator'` entries (16, 17, 18) were not fired.

## Files changed
- `packages/blitz-dom/src/events/pointer.rs` — `handle_pointerdown`:
  - the range arm's content-box fraction moved out to `range_track_fraction(&taffy::Layout, f32) -> f32`;
  - its `ClickTarget::RangeInput` match-arm body moved out to `press_range_input(doc, target, button, fraction,
    dispatch_event)`.
- `packages/blitz-dom/src/events/pointer.rs` — `handle_click`: the button arm's body moved out to
  `click_button(doc, node_id, submits, dispatch_event)`. The arm's guard, its `submits` read and its
  `break 'matched true` stay in place, and so does the range-focus block.
- `packages/blitz-dom/src/events/keyboard.rs` — `handle_key_or_input_event`: the range-key arm's body moved out to
  `press_range_input_key(doc, node_id, &Key, dispatch_event)`, with its WHATWG citation, beside `activates_on_enter`.
  The `else if` chain's order and conditions are unchanged.
- `packages/blitz-dom/src/node/range.rs` — `parse_float` is split into `take_sign`, `take_digits` (used for the
  integer, the fraction and the exponent digits) and `take_exponent`. Its signature, rustdoc and readings are
  unchanged, and the 20-row table is untouched.
- `README.md` — the current state:
  - `# Escher`, with «Escher» in all prose;
  - `## Status`: 0.1.0 done and verified (its six stages and three facts), then 0.2.0 planned in five stages;
  - a `Later:` line.
- New, under `escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/evidence/`: `complexity.py`,
  `complexity-before.md`, `complexity-after.md` and `readme.md`.

Every new function is private (`fn`) and documented with `///`.

## Gate readings (the full block, `gate-run-1.txt` in this run dir)
- 1 `cargo test -p blitz-dom --locked`: green, 97 passed (baseline 97).
- 2 the seven blitz-tests files: green, 43 passed (4 · 8 · 8 · 3 · 1 · 5 · 14, as the baseline).
- 3 `host_log` and `mcp_log` built alone: green, 1 and 1.
- 4 complexity: green. The over-ceiling set is exactly the five names, and the parents read 71 · 35 · 53, against 74 ·
  40 · 56 before. `parse_float` reads 10 (19 before), and every new function reads 2 or less
  (`evidence/complexity-after.md`).
- 5–8 the diff probes: green (0 files outside the three sources, 0 untracked code files, 0 banned added lines, 0
  added `pub` lines).
- 9–12 the README probes: green.
- 13 a11y leg: green, six files (6 · 6 · 4 · 8 · 5 · 14).
- 14 `ci-leg.sh fast`: green, 158 s.
- 15 `ci-leg.sh doc`: green.
- 16–18: `not run — leg operator`.

Step 1's known-verdict controls were taken before any source edit (`evidence/complexity-before.md`):
- **Must fail:** the untouched tree read exit 1, with `parse_float` in the set and the parents at 74 · 40 · 56.
- **Must pass:** the merge-base copies read exit 0, with the parents at 69 · 35 · 50 and the entry's exact last line.

Matrix: `matrix.py show --chunk` reads 0 claimed. The plan claims no capability (v020-01 has legs in "File splits"),
so no `ref` was written. Smoke: skipped. No binary, entry point or boot path moved, and the plan lists no `smoke` or
`self-verify` entry. The split arms ran through the real dispatch pipeline in `stand_keyboard`, `stand_act_range`,
`stand_act_keys` and `range_input_interaction` at P2.

No deferral: the Rust delta is 3 files, and every Rust gate ran.

## Deviations from plan (with justifications)
- **The parents stop above their upstream floors.** `handle_click` reads 71 (floor 69) and
  `handle_key_or_input_event` 53 (floor 50); `handle_pointerdown` reaches 35. What remains is what the plan keeps in
  place: the range-focus `if` in `handle_click`'s walk (step 3), and the range-key and Enter conditions in the
  `else if` chain (step 4). The plan predicted "a few points toward" the floor, and the acceptance asks only "below 74 ·
  40 · 56".
- **The range-focus block was not routed through `click_button`'s focus half.** Step 3 made that optional. Leaving it
  keeps the event order trivially identical.
- **`take_sign` reads the exponent's sign as well as the number's.** The two match the same closed set, `-` or `+`,
  so the readings are unchanged (the 20-row table passes).
- **`range.rs` gains a `use std::{iter::Peekable, str::Chars};` line** for the step functions' signatures. It is in a
  listed file and changes no visibility.
- **The README keeps the 0.1.0 epochs as a list under `### 0.1.0 — done and verified`**, because architecture
  §Project Intent → Front page lists "the 0.1.0 epochs" as a kept element. The 0.2.0 epoch lines follow the route's
  entries in plain words: "the nine engine debts", and "the six mutants that only timed out".
- **The complexity entry's `mktemp -d` leaves its JSON directories under the OS temp dir**, as the plan's entry is
  written. They are files, not processes.

Scope record: none written. `gate.py scope` reads `clean — changed 4 · listed 4 · recorded 0`.

## Surfaced / stuck
None.

## Process hygiene

The census was measured with `ps -eo pid,comm,args` after the block, matching `escher-session`, `seven_guis`,
`cargo`, `rustc`, `rust-code-analysis` and the test-binary names. The only hit was the census's own shell.

| process | started by | final state |
|---|---|---|
| `cargo test` / `cargo build` / ci-leg children (entries 1–3, 13–15) | a leg (the gate tool, this run) | terminated |
| `rust-code-analysis-cli` + `complexity.py` (entry 4, both firings; the must-pass control; one listing) | this run | terminated |
| `cargo build -p blitz-dom` and `cargo fmt --check` (the P1 compile check) | this run | terminated |

## Decisions & corrections
none

## Operator pass (run on the operator's directive, 2026-10-16)
- Entry 16, hygiene, fired as written: `hygiene: clean`, green.
- The pre-CI commit is `c71f7ed14d5ba8c0e9ffa3c66cb91747688861e2`.
- Entry 17, fast, clean-tree guard and push: green. `history moved: refs/remotes/origin/build/escher-0.2.0
  d236e918→c71f7ed1`, the intended push.
- Entry 18, `ci.py conclusion`: `verdict: green · checks 17/17 · wall 897 s`, CI#38917429198, with `Stand checks
  repeated` among the checks.
- Record: `escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/evidence/ci.md`.

## Next
`/andromeda-wrap-session`. The pass stopped before the wrap, on the operator's directive.
