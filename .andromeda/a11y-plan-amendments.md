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

## 2026-10-06-upstream-sync-element-identity — file:line citations re-pointed after the upstream merge
**Section:** every section citing a merged upstream file's lines
**Change:** 16 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`; the merged files' lines moved. The `ci-leg.sh a11y` files stay unnarrowed (6 · 6 · 3).
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-upstream-sync-element-identity — root-manifest citations re-pointed after the line-61 insert
**Section:** §1 A11y Scope Summary (workspace dependencies)
**Change:** 1 root `Cargo.toml` citation re-pointed +1 — it read one line low since the `seven_guis` path entry was inserted at `Cargo.toml:61`; verified against the cited text. No claim text changed.
**Why:** the 2026-10-06-headless-stand wrap did not re-point the root-manifest citations past its insert; the operator chose at this wrap's escalation (2026-10-06) to fix them in this pass rather than carry them.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-stable-element-ids — citations re-pointed after the element-id inserts
**Section:** the sections citing `dioxus_document.rs` and `flight_booker.rs`
**Change:** 6 `file:line` citations into `dioxus_document.rs`, `lib.rs` and the four lean-task files re-pointed by the chunk's measured line maps; no claim text changed.
**Why:** the chunk inserted the element-id methods into `dioxus_document.rs` and author ids into the task files, moving the cited lines.
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/

## 2026-10-06-accessibility-tree-identity — name sources, carried stable id, Dioxus feature wiring
**Section:** §1 A11y Scope Summary (Dioxus crates · blitz-tests · crate docs) · §2 A11y Strategy (tree build) · §7 Screen Reader Support → Accessibility tree output · citations re-pointed
**Change:**
- §1: was "`accessibility` … forwards to blitz-dom (and blitz-shell in dioxus-native)" and "the Dioxus crates only forward the feature to blitz"; now dioxus-native-dom's feature forwards to blitz-dom and enables its own optional `accesskit`, gating its `accessibility_tree` override, and dioxus-native's forwards to blitz-dom, blitz-shell and dioxus-native-dom. The workspace takes both with `default-features = false`, so a crate gets the feature only by naming it — the seven_guis stand binary names none and builds no platform adapter. blitz-tests builds dioxus-native-dom with `accessibility`.
- §2: the shell builds and refreshes the tree through `Document::accessibility_tree`, so a wrapper's override reaches the platform tree.
- §7: an `aria-label` non-empty after trimming becomes the node's `label` (accname-1.2 §2C); a `<label>` names its bound `<input>` (`for`-target, else first nested) through `labelled_by`, before or after it; in a Dioxus document every element node carries its stable id as `author_id`, no `TextRun` / document-root / `Window` node does; the 15 stand controls carry an HTML-AAM role and a non-empty name in both layout modes, the six inputs named by four `<label for>` and two `aria-label` attributes, the Tab order unchanged.
- 33 citations into the changed files re-pointed by the measured line map.
**Why:** intent §Findings 1's accessibility-tree leg; the six stand inputs had no accessible name because the tree read only text children. The operator chose the name sources (`aria-label` plus `<label>` association) at phase P4. Trap: the windowed stand ships no AccessKit at all — the operator directed a working-route note for it at this wrap.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-founder-rulings — falsy-disabled engine fix ratified
**Section:** §5 Keyboard Navigation (the `disabled` reading; the Dioxus falsy `disabled` / `checked` clearing)
**Change:** no body text changes — the body states the clearing (a falsy Dioxus `disabled` or `checked` removes the attribute) as current truth with no provisional clause; its status is now ratified, no longer PROVISIONAL per the 2026-10-06-headless-stand entry. The trap stands: other Dioxus boolean attributes still write `"false"`.
**Why:** the founder's own word at the Epoch 2 boundary (the founder, 2026-10-06), superseding the delegate overseer's provisional widening by rule.
**Ref:** .andromeda/runs/2026-10-06T16-16-18-wrap/

## 2026-10-06-snapshot-model — the snapshot consumes the accessibility tree
**Section:** §2 A11y Strategy (the `accessibility` feature) · §7 Screen Reader Support → Accessibility tree output · → Platform adapter · §8 Cognitive Accessibility (the flight-booker cue)
**Change:**
- §7 Accessibility tree output: adds the snapshot model as a consumer — `DioxusDocument::snapshot` takes each node's role, name and the focused node from the tree `accessibility_tree` returns, with no second role or name mapping; on the stand every snapshot node's role and name equal its accessibility node's, nothing reads focused at boot and only `back-btn` does after one focus move.
- §2: was "gates its own `accessibility_tree` override"; now "and its snapshot model"; citation `lib.rs:7` → `7-8`, plus `17-18`.
- §7 Platform adapter: citation `lib.rs:7` → `7-8`.
- §8: was "no test asserts a Dioxus control's focusability or Tab order"; now names the two stand checks that do — the forward focus sequence in `stand_accessibility_ids` and the snapshot's `focused` after one move in `stand_snapshot`.
**Why:** the snapshot is the second reader of the tree's role, name and focus, so §7 records it and the rule that it never re-derives them. The §8 sentence was already false since the accessibility-tree-identity chunk and is corrected here, where a second check contradicts it.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/

## 2026-10-06-id-stability-across-code-edits — the actionable-key rule and its check
**Section:** §2 Feature exposure · §5 Script and framework exposure · §7 Accessibility tree output · → Platform adapter · §8 Error recovery · → Orientation and status cues
**Change:**
- §7 adds the rule: every element an agent can act on — focusable, or an interactive role, or a listener — reads an author key, so its `author_id` holds under any edit around it. `DioxusDocument::unkeyed_actionable` is the check; it takes the role from the accessibility tree with no second mapping, holds a disabled control by its role and a hidden one by focusability or its listener. On the stand it reads empty on the four lean tasks and Home in every state, and 2 · 3 · 676 on the temp converter, circle drawer and cells. CRUD rows (`crud-person-{person.id}`) and Home's cards (`task-card-{slug}`) were keyed by an `id` attribute alone: no `tabindex`, role, focusability or Tab-order change.
- §2: the `accessibility` feature also gates the actionable-key check.
- Eleven line citations re-pointed, and `lib.rs:14-15` added in §2. Two of the eleven, the seven_guis card citations in §8, had already drifted before this chunk and now point at the `TASKS` table and the card markup.
**Why:** the rule is the founder's (the founder, 2026-10-06). Trap for later chunks: never satisfy the check by adding a `tabindex` or a role — that changes the Tab order §5 pins; key the element.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/
