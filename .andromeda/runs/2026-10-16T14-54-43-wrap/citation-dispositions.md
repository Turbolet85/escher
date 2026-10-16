sweep: base: af1a00ad — the previous sweep (2026-10-15T09-44-57-wrap) · re-pointed 73 · changed 7 · stretched 8

Each master row read at its citing line (`cascade.py window` at its offset for a line over 2 000 chars) and at the cited
lines in the work tree. The cited files of the 14 held rows are untouched since the base: `git diff --name-only af1a00ad
-- {the nine cited files}` prints nothing.

changed .andromeda/architecture.md:87@c4530 packages/blitz-dom/src/events/keyboard.rs:103-117 → ? — the end rewritten → holds — re-pointed by hand to 141-158 (the claim is the key mapping — the arrows step, `Home` · `End` go to the minimum and maximum, every other key does nothing — now `press_range_input_key`'s body, keyboard.rs:141-158; its focus and `disabled` gate, keyboard.rs:103-108, is cited by the [Scrolling and input] amendment)
changed .andromeda/architecture.md:87@c4866 packages/blitz-dom/src/events/pointer.rs:463-474 → ? — both ends rewritten → holds — re-pointed by hand to 566-577 (the fraction of the content box, clamped to [0, 1], 0 for no width — now `range_track_fraction`, pointer.rs:566-577)
stretched .andromeda/architecture.md:87@c4916 packages/blitz-dom/src/events/pointer.rs:517-525 → 508-510 — changed inside → holds (pointer.rs:508-510 is the `ClickTarget::RangeInput` arm, now calling `press_range_input`, whose main-button write is pointer.rs:579-594 — cited by the [Scrolling and input] amendment)
stretched .andromeda/architecture.md:87@c8912 packages/blitz-dom/src/events/pointer.rs:800-821 → 815-827 — changed inside → holds (pointer.rs:815-827 is `handle_click`'s button arm — its element and type match, `submits`, the call to `click_button` and the matched end of the walk; the focus and submit body is `click_button`, pointer.rs:890-907, cited by the amendment)
held .andromeda/architecture.md:106 Cargo.toml:47-58 — refused at 2026-10-13T15-50-37-wrap, not touched since «blitz-dom =…» → holds — stands (the cited file untouched since the base; the row stays held)
held .andromeda/architecture.md:113 examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (untouched; stays held)
stretched .andromeda/architecture.md:123@c2487 packages/blitz-dom/src/events/pointer.rs:800-821 → 815-827 — changed inside → holds (the button arm, as at :87@c8912; `generate_focus_events` before the submit now sits in `click_button`, pointer.rs:890-907, which the Events amendment cites)
changed .andromeda/security-plan.md:120@c1750 packages/blitz-dom/src/events/keyboard.rs:103-117 → ? — the end rewritten → holds — re-pointed by hand to 103-108 (the claim is the gate: the keys act only on a focused range input with no `disabled` attribute — the range arm's condition, keyboard.rs:103-108)
held .andromeda/security-plan.md:124@c14037 tests/blitz-tests/tests/stand_actionable_keys.rs:1-4 — refused at 2026-10-14T22-00-51-wrap, not touched since «//! Every e…» → holds — stands (untouched; stays held)
stretched .andromeda/security-plan.md:127@c632 packages/blitz-dom/src/node/range.rs:230-294 → 232-270 — changed inside → holds (range.rs:232-270 is `parse_float` with its rustdoc: an error, NaN or an infinity read as `None` — the `is_finite` filter at :268 — and `-0` as `0` at :269; its steps `take_sign` · `take_digits` · `take_exponent` at :272-301)
changed .andromeda/security-plan.md:127@c1955 packages/blitz-dom/src/events/keyboard.rs:103-117 → ? — the end rewritten → holds — re-pointed by hand to 141-158 (the key mapping, as at arch :87@c4530)
changed .andromeda/security-plan.md:127@c2235 packages/blitz-dom/src/events/pointer.rs:463-474 → ? — both ends rewritten → holds — re-pointed by hand to 566-577 (the fraction, as at arch :87@c4866)
stretched .andromeda/security-plan.md:127@c2285 packages/blitz-dom/src/events/pointer.rs:517-525 → 508-510 — changed inside → holds (the range arm, as at arch :87@c4916)
stretched .andromeda/security-plan.md:128@c1756 packages/blitz-dom/src/events/pointer.rs:800-821 → 815-827 — changed inside → holds (the button arm, as at arch :87@c8912)
held .andromeda/security-plan.md:228 .github/workflows/wpt-post-results.yml:29-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Fin…» → holds — stands (untouched; stays held)
held .andromeda/security-plan.md:342 packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…» → holds — stands (untouched; stays held)
held .andromeda/security-plan.md:376 examples/screenshot.rs:41-42 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (untouched; stays held)
held .andromeda/design-system.md:36 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (untouched; stays held)
held .andromeda/design-system.md:46 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (as at :36)
held .andromeda/design-system.md:134 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (untouched; stays held)
held .andromeda/test-plan.md:99@c1592 packages/dioxus-native-dom/src/snapshot.rs:530-577 — refused at 2026-10-13T23-23-45-wrap, not touched since «fn a_presen…» → holds — stands (untouched; stays held)
held .andromeda/test-plan.md:196 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…» → holds — stands (untouched; stays held)
held .andromeda/obs-plan.md:41 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (untouched; stays held)
held .andromeda/a11y-plan.md:12 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (untouched; stays held)
held .andromeda/a11y-plan.md:16 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (untouched; stays held)
stretched .andromeda/a11y-plan.md:158 packages/blitz-dom/src/events/pointer.rs:800-821 → 815-827 — changed inside → holds (the button arm, as at arch :87@c8912)
changed .andromeda/a11y-plan.md:161 packages/blitz-dom/src/events/pointer.rs:463-474 → ? — both ends rewritten → holds — re-pointed by hand to 566-577 (the fraction, as at arch :87@c4866)
stretched .andromeda/a11y-plan.md:161 packages/blitz-dom/src/events/pointer.rs:517-525 → 508-510 — changed inside → holds (the range arm, as at arch :87@c4916)
changed .andromeda/a11y-plan.md:165 packages/blitz-dom/src/events/keyboard.rs:103-117 → ? — the end rewritten → holds — re-pointed by hand to 141-158 (the key mapping, as at arch :87@c4530)
out-of-range .claude/docs/session-learnings.md:107 host.rs:350 — the file held 225 lines at the base → curation home — P3 (a bare `host.rs:350` inside a learning's prose, a preserve-verbatim home)
unresolved 2 — `map`'s; no disposition owed.
