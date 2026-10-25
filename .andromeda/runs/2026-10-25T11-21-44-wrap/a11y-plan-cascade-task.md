# Task — a11y-plan · 2026-10-25-capability-statement

## The session
The session added a new document, `docs/capabilities.md` («What Escher can do»). It has five sections: what it can do, stated limits, accessibility reach, what an MCP client carries onward, and where each claim is proven.
It added the CI-script pin `.github/scripts/test_capability_statement.py`, which has 4 tests. They check that every verb and every cause is named in the statement, that the five table counts are written in words, and that README.md says «the twelve driver commands as tools». The pin joins the `ci-scripts` leg, which goes from 100 to 104 tests and from seven to eight files.
It replaced one line of README.md: eleven driver commands became twelve, and the line now links `docs/capabilities.md`.
It changed no Rust code, crate, dependency, verb, field, cause, carrier, log field, span or CI leg. The workspace tally is unchanged at 1105 passed.
A measurement disproved the standing claim that the windowed assistive-technology witness is owned by a residuals line «(target: next)». That line reads `re-carried:0.2.0` (`.andromeda/residuals.md:14`), and `escher-0.2.0/requirements.md:42-43` says this version does not reach an active assistive technology.
Its accessibility section restates the tier-0 reach and its limits. Nothing moved there.

## Reached files
- docs/capabilities.md
- .github/scripts/test_capability_statement.py
- README.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/counts.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/pin-controls.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/viola-pointer.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/operator-pass.md
- escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/operator-confirm.md

## Touches
- §1 Dioxus crates bullet (body line 9): the owner of the adapter's windowed assistive-technology witness, stated as a residuals line «(target: next)», is now the residuals line that reads `re-carried:0.2.0`, which this version does not reach.
- §2 tree lifecycle (body line 27): the owed windowed witness at the platform adapter has the same owner correction, from «(target: next)» to `re-carried:0.2.0`.
- §2 role derivation, the three ARIA states (body line 35): only the windowed AT witness half of «that fix and the windowed AT witness owned by … (target: next)» moves to `re-carried:0.2.0`.
- `a11y-scope-summary.md` line 17: the AT witness owner clause has the same correction.
- `a11y-strategy.md` line 9: the owner clause «and is owned by a `.andromeda/residuals.md` line (target: next)» on the AT witness has the same correction.
- `a11y-strategy.md` line 21: the AT witness owner clause has the same correction.
- Other text worded differently that names the platform adapter's witness, a windowed check or an assistive technology with a «(target: next)» owner gets the same correction. The pattern did not reach every wording elsewhere: three such sites were found outside it in other masters. Check the sr-platform-adapter entities in particular.
- Tier 0, SC 2.1.1 · 1.4.3 · 2.4.3 and no other criterion: re-read by the statement, unmoved.
- §9 a11y leg: fourteen files at 14 · 84 · 0 · 0 on CI#40134371519 with no required status check, re-read by the statement, unmoved.
- Contrast record: the accent `#3a5ce5` at 5.47 · 6.83 · 8.27 on the accent controls, and SC 1.4.3 met on every measured non-exempt stand row. Re-read by the statement, unmoved.
- Visual lint: 0 findings at boot per lean task, and `crud-person-14` reads `clipped` inside `crud-list` after twelve Creates. Restated by the statement, unmoved.
- Stated limits re-read unmoved: HTML's focus fixup is not built; a `multiple` select and an `optgroup`'s options are not modelled; `aria-activedescendant` is unread; the tree never states a native checkbox or radio checked.
- `inert` honouring (skipped by Tab, passed through by a hit, left out of the tree), restated by the statement, unmoved.
- The description reads (`aria-description`, `aria-describedby`) leave only to the adapter, and the snapshot and MCP carry no description. Restated by the statement, unmoved.
- The role reaches the tree through the engine's one role-name mapping. Restated by the statement, unmoved.
- What an assistive technology reads is unmeasured, and no stand binary builds a platform accessibility adapter. Restated by the statement, unmoved; only its owner disposition moved.
- Indirect: the `ci-scripts` part of the fast leg moved from 100 to 104 tests and from seven to eight files. This touches §9 only where a11y-plan states a `ci-scripts` count or file count.

## New facts
- F1 · a11y-scope-summary · `docs/capabilities.md` now publishes escher 0.2.0's accessibility reach under `## Accessibility reach`: tier 0, the a11y leg's tally, the AT reading unmeasured, and the named limits. It is the statement that Viola's planning reads, through a pointer naming that path and commit `e7e71221d`. Its pin, `test_capability_statement.py`, checks only the verb, cause and table counts and the README line, not the accessibility claims.

## Untouched
- §5 focus order: the owner of the focus fixup, a residuals line «(target: next)», is not the AT witness, and the report does not move it.
- §7 actionable-key rule: the owner of the missing reacts-to-a-click reading, «(target: next)», is a different residual and is not moved.
- §2 role derivation: the owner of the native checked state fix, the other half of body line 35, is not moved by this session.
- §3 assertion harness test inventory and counts: no Rust test was added, and the workspace tally is unchanged.
- §3 obs log format paragraph: no span, log field or record class was added.
- §4 ARIA mapping, hidden content and role inventory: no engine or builder change.
- §5 keyboard handling (range, select, Enter and Space activation, the CRUD list keys): no engine change. The statement only restates them.
- §6 focus ring, motion, state colour and preferences: there is no stylesheet or token change.
- §8 cognitive accessibility: the flight booker's `aria-invalid` cue is unchanged.
- §9 the a11y leg's fourteen-file list and the `stand-repeat` eight: no CI leg, job, workflow or `ci-leg.sh` line changed.
