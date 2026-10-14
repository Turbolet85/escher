# P5 controls — 2026-10-14-stand-a11y-assertions

Each control fired AS THE ENTRY through `gate.py run --plan escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/plan.md
--entry {n} --project-root {clone}` over a `git clone --shared` of the project at HEAD `ccfcda48`, in the session
scratchpad (`$TMPDIR/{scratchpad}/clone-controls`, `$TMPDIR/{scratchpad}/clone-controls-5`); the inputs lived in the
clones only.

| entry | clone input | reading | wanted |
|---|---|---|---|
| 5 | unmodified | red · exit 0 ✗ (exit 1) · 17 hits | red, as the real tree |
| 5 | re-colour as first written in step 2 (temp converter's rgba only) | red · exit 0 · 1 hit | green — the control read RED: circle drawer's two `rgba(74, 108, 247, …)` fills were outside step 2; step 2 and research.md §Sweeps corrected |
| 5 | re-colour as the corrected step 2 writes it (every rgba form) | green · exit 1 · `0` | green (must pass) |
| 5 | the corrected re-colour plus `/* #4A6CF7 */` appended to cells.rs | red · exit 0 | red (must fail, case-insensitive) |
| 6 | unmodified | green · exit 1 · `0` | green, as the real tree |
| 6 | `fn control() { eprintln!("x"); }` appended to stand_contrast.rs | red · exit 0 | red (must fail) |
| 6 | `pub const REGION_NAMES …` appended to common/mod.rs | green · exit 1 | green (must pass) |
| 7 | unmodified | green · exit 0 | green, as the real tree |
| 7 | `// control` appended to packages/blitz-dom/src/accessibility.rs | red · exit 1 | red (must fail) |
| 7 | `// control` appended to crud.rs and `# control` to .github/scripts/ci-leg.sh (write set) | green · exit 0 | green (must pass) |
