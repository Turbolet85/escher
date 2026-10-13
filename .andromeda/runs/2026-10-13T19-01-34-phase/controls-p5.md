# P5 controls — 2026-10-13-range-input-interaction

Check 4 (9)'s known-positive controls for the five probe entries the chunk's subject feeds (entries 7–11). Each was fired
AS THE ENTRY through `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan
escher-0.1.0/chunks/2026-10-13-range-input-interaction/plan.md --entry {n} --project-root {clone}`, with no `--run-dir`.
The clone was `git clone --shared` of the project at HEAD `97ce760ddfb4db2996162551501b809e51b46367`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0c1bcdf6-d393-4ac0-a2a6-b13a48a0a2b7/scratchpad/clone`.

Each input was left uncommitted in the clone (every entry reads the work tree), and the clone was reset between inputs.
The script is `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0c1bcdf6-d393-4ac0-a2a6-b13a48a0a2b7/scratchpad/controls.sh`;
its lines are copied verbatim below.

| Entry | Clone role | Input | Reading |
|---|---|---|---|
| 7 | unmodified | — | green · exit 1 (count 0), as on the real tree |
| 7 | must-fail | a `[[package]]` block appended to `Cargo.lock` | red · exit 0 (count 1) |
| 7 | must-pass | a dependency line added inside an existing package entry of `Cargo.lock` | green · exit 1 (count 0) |
| 8 | unmodified | — (no `range.rs`) | green · exit 1, vacuous: the file is absent, as on the real tree |
| 8 | must-fail | `range.rs` holding a `pub fn` that calls `println!` | red · exit 0 |
| 8 | must-pass | `range.rs` holding `unwrap()` and `println!` inside its `#[cfg(test)]` module only | green · exit 1 |
| 9 | unmodified | — | green · exit 1 (count 0), as on the real tree |
| 9 | must-fail | a `fn` calling `println!` appended to `packages/blitz-dom/src/form.rs` | red · exit 0 |
| 9 | must-pass | a comment and a plain `fn` appended to `packages/blitz-dom/src/events/keyboard.rs` | green · exit 1 |
| 10 | unmodified | — (neither test file present) | green · exit 1, vacuous for `range_input_interaction.rs` |
| 10 | must-fail | `range_input_interaction.rs` calling `eprintln!` | red · exit 0 |
| 10 | must-pass | `range_input_interaction.rs` with `mod common;` and one assertion | green · exit 1 |
| 11 | unmodified | — | green · exit 0, as on the real tree |
| 11 | must-fail | a line appended to `packages/escher-driver/src/schema.rs` | red · exit 1 |
| 11 | must-pass | a line appended to `packages/blitz-dom/src/form.rs` (the write set) | green · exit 0 |

The lines as the tool printed them:

```
entry 7 · unmodified clone ·   7 probe       green · exit 1 · 0.01s · 2 B → 7.log
entry 8 · unmodified clone ·   8 probe       green · exit 1 · 0.0s · 82 B → 8.log
entry 9 · unmodified clone ·   9 probe       green · exit 1 · 0.01s · 2 B → 9.log
entry 10 · unmodified clone ·  10 probe       green · exit 1 · 0.0s · 85 B → 10.log
entry 11 · unmodified clone ·  11 probe       green · exit 0 · 0.01s · 0 B → 11.log
entry 7 · must-fail ·   7 probe       red · exit 1 ✗ (exit 0) · exit 0 · 0.01s · 2 B → 7.log
entry 7 · must-pass ·   7 probe       green · exit 1 · 0.01s · 2 B → 7.2.log
entry 8 · must-fail ·   8 probe       red · exit 1 ✗ (exit 0) · exit 0 · 0.0s · 2 B → 8.log
entry 8 · must-pass ·   8 probe       green · exit 1 · 0.0s · 2 B → 8.log
entry 9 · must-fail ·   9 probe       red · exit 1 ✗ (exit 0) · exit 0 · 0.01s · 2 B → 9.log
entry 9 · must-pass ·   9 probe       green · exit 1 · 0.01s · 2 B → 9.2.log
entry 10 · must-fail ·  10 probe       red · exit 1 ✗ (exit 0) · exit 0 · 0.0s · 2 B → 10.log
entry 10 · must-pass ·  10 probe       green · exit 1 · 0.0s · 2 B → 10.log
entry 11 · must-fail ·  11 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 11.log
entry 11 · must-pass ·  11 probe       green · exit 0 · 0.01s · 0 B → 11.2.log
```

These are probe controls, not mutation controls: no source of this project was mutated, and the mutation tier stays
unstarted (inputs#I1).
