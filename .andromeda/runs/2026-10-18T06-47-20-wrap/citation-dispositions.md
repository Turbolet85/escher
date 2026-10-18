sweep: base: fb842671 — the previous sweep (2026-10-18T04-30-47-wrap) · re-pointed 35 · changed 0 · stretched 2

Every row the apply call printed (trail `cites-2026-10-18-upstream-sync-determinism-and-traces.json`, its non-dry-run `apply` record), read at its citing line and, for a master row, at the cited lines in the work tree. The held rows' cited files are byte-identical to `fb842671` (the report's preservation entry: `git diff --quiet fb842671 -- … ':(exclude)packages/blitz-dom/src/document.rs'` exit 0), so each reads as the previous wrap read it.

- held .andromeda/architecture.md:119 Cargo.toml:47-58 — refused at 2026-10-13T15-50-37-wrap, not touched since «blitz-dom =…» → holds — stands
- held .andromeda/architecture.md:126 examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands
- held .andromeda/security-plan.md:230 .github/workflows/wpt-post-results.yml:29-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Fin…» → holds — stands
- held .andromeda/security-plan.md:345 packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…» → holds — stands
- held .andromeda/security-plan.md:379 examples/screenshot.rs:41-42 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands
- held .andromeda/design-system.md:36 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands
- held .andromeda/design-system.md:46 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands
- held .andromeda/design-system.md:134 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands
- stretched .andromeda/design-system.md:316 packages/blitz-dom/src/document.rs:2260-2316 → 2260-2320 — changed inside → holds (`get_cursor` whole, 2260-2320; "Default otherwise" now true over no box as well — the over-no-box reading is the plan's design-system expected amendment, raised at Validate)
- stretched .andromeda/test-plan.md:12 packages/blitz-dom/src/document.rs:3004-3567 → 3008-3581 — changed inside → holds (the `document::` test modules' range, the new `hover_state_tests` test inside it; the count moves as `dom-tests`)
- held .andromeda/test-plan.md:196 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…» → holds — stands
- held .andromeda/obs-plan.md:41 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands
- held .andromeda/a11y-plan.md:12 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands
- held .andromeda/a11y-plan.md:16 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands
- out-of-range .claude/docs/session-learnings.md:140 host.rs:350 — the file held 225 lines at the base → curation home — P3
