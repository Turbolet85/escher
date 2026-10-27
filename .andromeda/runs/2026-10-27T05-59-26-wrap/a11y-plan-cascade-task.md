# Task — a11y-plan · 2026-10-27-readme-and-records

## The session
A documents-only chunk: no source, test, CI script, manifest, lock file or spec master changed; the CI run on the final HEAD read green, 17 of 17 checks.
README.md now shows 0.1.0 and 0.2.0 done and verified and 0.3.0 planned in six stages, names 0.2.0's additions (paged screens, a select, lint and its six defects, the virtual clock, the bounded load wait, recorded checks that replay), and its Later line names the platform accessibility adapter witnessed with a real assistive technology as outside 0.3.0.
docs/capabilities.md had three passages rewritten: the lint at boot (the flight booker reads one overlap of its flight-type select's two options), the host-sink part read (now measured, 0 id and 0 name needles), and the measured cold-agent run (now the 2026-10-25 run: six apps, 66 calls, 0 wrong).
It enumerated nine seven_guis stand rows in design-system (accent, success, DejaVu Sans, card shadow, spacing, radius, the select's overrides) that P2 moves out of the product's design system, with values unchanged, on F2 (PROVISIONAL). The accent goes to layout-templates §Surface: desktop-native → Primary screens and the font to arch §Standard Contracts → Headless stand. Ten further stand lines are listed but not decided.
It retired the local incubator exclude; .gitignore already ignores the incubator. It re-pointed the closed-version ledger refs v010-17 and v020-05 to three renamed test names, and claimed v030-02 gated on the founder's word on F3.

## Reached files
- README.md
- docs/capabilities.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-27-readme-and-records/scope.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/research.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/inputs/andromeda-inputs.json
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/counts.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/operator-pass.md
- .andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh
- .andromeda/runs/2026-10-27T05-19-41-implement/
- .andromeda/runs/2026-10-27T05-59-26-wrap/
- .git/info/exclude

## Touches
- §6 Visual Design Verification, visual-design-verification.md:12: its citation of design-system §Color Palette for the stand accent is re-pointed to the accent's new home, the values · desktop-native-screens block of layout-templates §Surface: desktop-native → Primary screens (Expected amendment 5, PROVISIONAL under F2).
- visual-design-verification.md:12 and :32 quote the stand accent (#3a5ce5 and its shades): the values are unchanged and only their design-system home moves, so these are no-change except for the citation above.
- visual-design-verification.md:12 quotes the invalid pair (#e53e3e / #fff5f5 / #c53030), and :12 and :16 quote the select's text pair (#1a1a1a on #ffffff, the #f5f5f5 page). These sit among the ten further stand lines whose move the operator decides at P2. The quoted values stay; a design-system citation for them follows only if they move.
- The stand's bundled DejaVu Sans font, named in a11y-plan as a test condition of the stand checks: its design-system Typography row moves to arch §Standard Contracts → Headless stand, beside font_ctx(). The a11y-plan text is no-change, but any citation of design-system §Typography for the stand font is stale.
- a11y-plan rows that name select#flight-type or its #c0c0c0 border as an a11y row: no-change; the select's overrides leave design-system only.
- screen-reader-support.md:39, the load page's 0 lint findings: a true claim that shares the lint-boot token, no-change.
- The visual lint lines (§6 Color contrast, §3 Contrast verification harness): the statement's lint-at-boot correction (the flight booker reads one overlap) matches what the masters already state, no-change.
- The adapter and windowed-AT residual lines that read re-carried:0.2.0 (§1 Dioxus crates, §2 tree lifecycle and role derivation, §7 Platform adapter): the README's 0.3.0 Later line puts the assistive-technology witness outside this version, which matches "an active assistive technology this version does not reach"; re-read, unmoved.
- §5 Test harness pattern, the OPEN load-page line (no Tab-order, keyboard-activation or contrast row for load-fetch): the README's 0.3.0 Engine debts line names keyboard and contrast checks of Cells and the load page as planned, not done. The gap and its residual stand; re-read, unmoved.
- The a11y tier (SC 2.1.1 · 1.4.3 · 2.4.3): the README and the statement claim no criterion beyond it, no assistive-technology reach, and no keyboard or contrast coverage of Cells or the load page; re-read, unmoved.
- §7 Stand states, the stand stating no aria-pressed: the v010-17 ref is re-pointed while its acceptance still describes the retired pressed pair (flight-one-way · flight-return), left as worded under F1. No a11y-plan site.
- The three renamed test names in the v010-17 and v020-05 ledger refs: no a11y-plan site (pattern old-names unfired); a11y-plan citations of test files by name stay as they stand.
- §9 CI Integration counts: the a11y leg's fourteen files and the eight in stand-repeat are unmoved, since no test or CI script changed.
- §3 log-format note (escher's sink, its two record classes and allowlist): the statement's host-sink part read now states the measured reading already held by the obs records; a11y-plan's note is unmoved.
- Any a11y-plan line that names the 2026-10-15 cold-agent run as the measured instance would now be stale (the 2026-10-25 run supersedes it); the cap-run sweep found 0 master sites.

## New facts
- (none)

## Untouched
- §5 Keyboard Navigation's engine rules (focusability, disabled-ness by presence, Enter and Space activation, range and select keys, the double-click window): no engine source changed.
- §2 accessibility tree lifecycle and the changed set's rules: no blitz-dom or blitz-shell change.
- §2 and §4 role derivation and the ARIA state mapping (aria-selected, aria-pressed, aria-checked, aria-invalid, role="switch", upstream's six merged reads): no builder change.
- §7 snapshot model, the masked values, author_id and the actionable-key rule: no dioxus-native-dom or escher-driver change.
- §3 assertion harness inventory and its test citations (accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, inert_subtree, accessibility_active_descendant_unread): no test file changed.
- §6 stand_contrast tables (17 control rows, 14 TEXT_ROWS, 18 STATE_ROWS) and the 5.47 accent reading: no stand colour value changed; only the design-system home of the rows moves.
- §8 Cognitive Accessibility, the flight booker's invalid-date cue and aria-invalid: no stand markup changed.
- §6 Visibility and hit-testing, the inert rules and the inert focus-fixup residual: no engine change.
- The NOT YET MEASURED and Observed absent notes in every section: the session measured no a11y surface.
- §7 Platform adapter note (Adapter creation before the window is shown): accesskit_xplat unchanged.
