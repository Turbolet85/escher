sweep: base: 494abd25 — the previous sweep (2026-10-16T17-27-46-wrap) · re-pointed 9 · changed 2 · stretched 4

Each master row read at its citing line (by its offset on a line over 2 000 chars, `cascade.py window`) and at the
cited lines in the work tree. Of the files the held rows cite, none moved since the base (`git diff --name-only
494abd25`: the chunk's code edits are the three test files, `deny.toml`, `.gitignore` and `.config/nextest.toml`).

held .andromeda/architecture.md:106 Cargo.toml:47-58 — refused at 2026-10-13T15-50-37-wrap, not touched since «blitz-dom =…» → holds — stands (Cargo.toml untouched since the base; stays held)
held .andromeda/architecture.md:113 examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (untouched; stays held)
stretched .andromeda/architecture.md:159 deny.toml:1-26 — changed inside → holds — «a cargo-deny advisory gate configured by the root `deny.toml`»: deny.toml:1-26 is the header, `[graph]` and `[advisories]` through the ignore list's closing `]` at 26; the change inside is the three RUSTSEC-2026-0367/0368/0369 ignores at 23–25
held .andromeda/security-plan.md:124@c14037 tests/blitz-tests/tests/stand_actionable_keys.rs:1-4 — refused at 2026-10-14T22-00-51-wrap, not touched since «//! Every e…» → holds — stands (untouched; stays held)
held .andromeda/security-plan.md:228 .github/workflows/wpt-post-results.yml:29-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Fin…» → holds — stands (untouched; stays held)
stretched .andromeda/security-plan.md:237@c828 deny.toml:1-26 — changed inside → holds — «`[advisories]` per-ID ignores only, each with a written reason, no blanket allow, `unmaintained` and `unsound` at their defaults»: 1–26 hold `[advisories]` and its five per-ID ignores, each with a reason; the change inside is the three ignores at 23–25 (the clause's «Two ignores stand» is the `zip-ign` fact, amended at P2)
held .andromeda/security-plan.md:342 packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…» → holds — stands (untouched; stays held)
held .andromeda/security-plan.md:376 examples/screenshot.rs:41-42 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (untouched; stays held)
held .andromeda/design-system.md:36 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (untouched; stays held)
held .andromeda/design-system.md:46 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (as at :36)
held .andromeda/design-system.md:134 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (untouched; stays held)
stretched .andromeda/test-plan.md:21@c7396 packages/escher-driver/src/mcp_tests.rs:1-655 — changed inside → holds — the whole file (655 lines, `wc -l`); the change inside is the new test at 167–196 (New text, by line)
stretched .andromeda/test-plan.md:24@c5942 packages/dioxus-native-dom/src/dioxus_document_tests.rs:48-240 — changed inside → holds — line 48 is the first `#[test]`, 240 the file's last line; the change inside is the new test at 147–166 (New text, by line); the count the sentence states is the `dnd-unit` / `bridge-six` facts, P2's
held .andromeda/test-plan.md:99@c1592 packages/dioxus-native-dom/src/snapshot.rs:530-577 — refused at 2026-10-13T23-23-45-wrap, not touched since «fn a_presen…» → holds — stands (untouched; stays held)
changed .andromeda/test-plan.md:99@c2299 packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-5 → ? — the end rewritten → holds — re-pointed by hand to 1-6: the module's `//!` doc now runs 1–6, its last line «A child module of `dioxus_document`, because the event handler's fields are private to it.» at 6 (the claim: no constructor, `pub` or shim gained — the child-module form)
changed .andromeda/test-plan.md:137 packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-5 → ? — the end rewritten → holds — re-pointed by hand to 1-6 (the same `//!` doc, its child-module reason at line 6)
held .andromeda/test-plan.md:196 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…» → holds — stands (untouched; stays held)
held .andromeda/obs-plan.md:41 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (untouched; stays held)
held .andromeda/a11y-plan.md:12 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (untouched; stays held)
held .andromeda/a11y-plan.md:16 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (untouched; stays held)
out-of-range .claude/docs/session-learnings.md:122 host.rs:350 — the file held 225 lines at the base → curation home — P3 (a bare `host.rs:350` inside a learning's prose, a preserve-verbatim home)
