# a11y-plan — amendments

One entry per amendment to `a11y-plan.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-ci-gate-legs — a named a11y leg on fork CI
**Section:** §3 A11y Assertion Harness Contract (CI integration · `a11y-ci-gate-wire`) · §9 CI Integration
**Change:** was "accessibility checks in CI are observed absent" (§3, §9) and `a11y-ci-gate-wire` recorded absent; now ci.yml's `a11y` job, "Accessibility (a11y) tests", runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates`, 6 + 6 + 3 tests — in the slow tier behind the four fast jobs, restoring the test job's cache and saving none; it runs existing tests and adds no stand assertion; the §9 search `a11y|accessib|axe` over the workflows finds it; the key reads discharged.
**Why:** the CI gate legs chunk wired the existing accessibility tests into fork CI as their own leg; SC assertions on stand controls gating merges stay with the "Stand a11y assertions" work.
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-06-telemetry-bootstrap — obs log format no longer unmeasured
**Section:** §3 A11y Assertion Harness Contract (NOT YET MEASURED marker) · every section citing `Cargo.toml` or `tests/blitz-tests/Cargo.toml` lines
**Change:**
- Was "the test plan (§3) and obs plan (§3, §6) leave the log format unmeasured"; now the obs log format is measured for escher's own sink: one non-JSON text line per event on stderr behind the allowlist scrub, which prints only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` for `blitz*` and `accesskit_xplat` targets and redacts `text`, `value`, `html`, `attrs` at any target.
- The structured a11y violation schema, the WCAG mapping and a screen-reader test pattern stay unmeasured; no violation schema is yet defined against that line.
- 4 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk fixed the format the a11y↔obs schema bind reads.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/

## 2026-10-06-headless-stand — disabled keyed two ways; Dioxus falsy disabled cleared; flight-booker cue checked
**Section:** §5 Keyboard Navigation → Focus order per layout (engine) · Script and framework exposure · §8 Cognitive Accessibility → Error recovery
**Change:**
- Focus order: was "focusable if it is not disabled"; now "if its `disabled` attribute does not parse as `true`", and a new bullet: the DISABLED/ENABLED state (`:disabled`) and the click target key on presence, focusability on the parsed bool — `disabled="false"` matches `:disabled` yet stays focusable, a bare `disabled=""` is focusable too.
- Framework exposure: dioxus-native-dom removes a falsy `disabled` / `checked`; its other boolean attributes are still written as `"false"` — for `hidden` that would drop a `hidden: false` node from the tree: recorded, not established.
- Error recovery: the stand check asserts the invalid-date cue without colour (`invalid` class, `disabled` Book it did not carry before); no test asserts a Dioxus control's focusability or Tab order.
**Why:** the headless stand chunk measured `disabled="false"` matching `:disabled` on enabled stand buttons and fixed the Dioxus write (widening on the delegate overseer's word, 2026-10-06, PROVISIONAL). Trap: the focusability path never had the defect — only state, styling and click targeting did.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/
