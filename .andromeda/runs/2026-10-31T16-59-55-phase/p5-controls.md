# P5 controls — 2026-10-31-select-popup

Each entry fired as itself through `gate.py run --entry {n} --project-root {clone}` over a `git clone --shared` of the project at `3af5a9bcf078b70ea37b97f516fcaf444a3da0f0`, in `$TMPDIR/{scratchpad}/`, each input left uncommitted in the clone and the clone removed after. Fired 2026-10-31, the clock reading 18:02–18:04Z.

| Entry | Input | Reading |
|---|---|---|
| 1 preservation guard | unmodified | green, exit 0 |
| 1 | a line in `deny.toml` | red, exit 1 (must fail) |
| 1 | a line in `packages/blitz-shell/src/lib.rs` | red, exit 1 (must fail) |
| 1 | a line in `packages/blitz-dom/src/lib.rs` | green, exit 0 (must pass) |
| 2 silence census | unmodified, three new files absent | red, exit 2 |
| 2 | the three minted | green, exit 1, last line 0 (must pass) |
| 2 | a `println!` in one | red, exit 0 (must fail) |
| 3 `info_span!` count | unmodified | green, last line 2 |
| 3 | a third site | red, last line 3 (must fail) |
| 4 added-line census | unmodified, an empty input | green, exit 1 |
| 4 | a plain added line in `document.rs` | green, exit 1 (must pass) |
| 4 | an added print in `document.rs` | red, exit 0 (must fail) |
| 5 new-file census | unmodified, two new files absent | red, exit 2 |
| 5 | both minted, plain | green, exit 1 (must pass) |
| 5 | an `.unwrap()` in one | red, exit 0 (must fail) |
| 8 motion census | unmodified, two new files absent | red, exit 2 |
| 8 | both minted, plain | green, exit 1 (must pass) |
| 8 | a `transition` in one | red, exit 0 (must fail) |
| 22 stand file count | unmodified | red, last line 50 |
| 22 | `stand_act_popup.rs` minted | green, last line 51 (must pass) |

**A wrong gate the controls caught.** Entry 2's first spelling named `stand_id_persistence.rs`, and its must-pass read red: that check prints its child's lines by design (`:292`, `:332`, `:335`). It left the entry by file, the note saying why, and the three rows of entry 2 above are the re-fired spelling.

**Entry 31's known-positive control**, at 18:04Z, on CI#41208321538, read to a scratch file first and removed: `Test [default features] 214 · 1181 · 0 · 12`, `Test (macos) 195 · 1178 · 0 · 12`, `Coverage report 195 · 1178 · 0 · 12`, `Test (windows) 195 · 1132 · 0 · 9`, `Accessibility (a11y) tests 14 · 112 · 0 · 0`, `Stand checks repeated 219 · 1008 · 0 · 21`, equal to the rows at `.andromeda/test-plan-detail/ci-integration.md:75`.
