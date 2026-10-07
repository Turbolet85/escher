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

## 2026-10-06-snapshot-state-fidelity — a falsy `hidden` no longer drops a node; a key-press focus check exists
**Section:** §3 Keyboard test harness · §5 Script and framework exposure · §5 Test harness pattern · §7 Accessibility tree output
**Change:**
- Script and framework exposure: was "a falsy `disabled` or `checked` removes the attribute; every other boolean attribute is still written as `"false"` — for `hidden` that would drop a `hidden: false` node: recorded, not established"; now a falsy value of one of the bridge's 27 boolean attributes removes it on both attribute paths, so an element rendered `hidden: false` is laid out and stays in the tree and the snapshot, while a truthy `hidden` still excludes it. The drop was established at the chunk's base before the fix.
- Test harness pattern and §3: was "keyboard-event or Tab-order navigation tests observed absent"; now a Tab / Shift+Tab key-press focus check exists on the stand (`crud-name` → `crud-surname` and back, `back-btn` on a fresh boot's first Tab), each time exactly the focused control's node reading `focused` and equal to the tree's focus. The other three observed-absent bullets stand.
- Accessibility tree output: the snapshot reads `enabled` (the presence of `disabled`), `checked` and `value` from the element, since the tree gives an element none of them; a password input reads `MASKED_VALUE`; focus is proven after a click, a Tab press and Shift+Tab.
- 3 line citations re-pointed (`mutation_writer.rs`, `snapshot.rs`).
**Why:** the chunk measured each reading on the stand through real input. The 27-name clear is PROVISIONAL on the direction given at phase's P5 review (2026-10-06), pending the founder's own word at the Epoch 3 boundary. Trap for later chunks: a bare or `"false"` `disabled` control still takes focus by Tab while reading not enabled; and, by phase's code read, not measured, a click on a plain button clears focus instead of focusing it — pointer focus was proven on a text input and a checkbox.
**Ref:** .andromeda/runs/2026-10-06T22-39-16-wrap/

## 2026-10-06-compact-snapshot-serialization — the snapshot's text form reads the model; a file input's value is masked
**Section:** §2 Feature exposure · §7 Accessibility tree output
**Change:**
- Feature exposure: the `accessibility` feature was said to gate the `accessibility_tree` override, the snapshot model and the actionable-key check; now also that model's text form `Snapshot::to_text`, all three named by the crate doc.
- Accessibility tree output: the value mask was a password input's only; now a file input reads the same marker where its `value` attribute is non-empty, its path in no field of the snapshot.
- Accessibility tree output adds that `Snapshot::to_text` is a further reader of the snapshot model, never of the tree or of a control: each line's role and name are the snapshot node's, there is no second role or name mapping, and there is one line per snapshot node; on the stand no line reads `focused` at boot and exactly `back-btn`'s does after a Tab press, in both layout modes.
- 1 line citation re-pointed (`snapshot.rs`); the feature citation now spans the new module's declaration.
**Why:** the chunk built the text form on the snapshot model. The plan listed these as expected amendments and neither a11y detector's invariant covers them, so the wrap raised them itself.
**Ref:** .andromeda/runs/2026-10-06T23-58-14-wrap/

## 2026-10-07-change-tracking-and-diff — the tree is refreshed on change; the diff reads the snapshot model
**Section:** §2 A11y Strategy → Accessibility tree lifecycle · Feature exposure; §7 → Accessibility tree output; `file:line` citations into five edited files
**Change:**
- Lifecycle: the platform tree is built on InitialTreeRequested and refreshed on change — on a poll that reported work `View::poll` takes the document's changed set (outside the `accessibility` cfg) and, under `accessibility`, rebuilds the tree when the set it took was non-empty (was: "refreshed on poll when the document has changes"; that refresh did not run before this chunk, the flag it read answering false, so the tree was built once). No windowed run witnesses it on the dev host.
- Lifecycle: `changed_nodes` is the engine's changed set — marked by a mutation of an in-document node, a focus or checked change and a text control's input; not by node creation, hover, active, scroll or layout; `has_changes()` true while it is non-empty (was: "documented as the set of changed nodes for updating the accessibility tree").
- Feature exposure: the `accessibility` feature of dioxus-native-dom also gates the snapshot's diff `Snapshot::diff`; four items named by the crate doc (was three).
- Accessibility tree output: the diff is one more reader of the snapshot model, with no second role or name mapping — a text change reported on the element whose accessible name it changes, never as a text-run entry; a hidden element leaving and returning with its descendants; focus moves naming exactly the controls whose `focused` reading moved.
- 17 of 28 citations re-pointed by the measured line map.
**Why:** the chunk made the engine's change flag truthful and the shell drain it, so the refresh the plan described now happens. It is PROVISIONAL, pending the founder at the Epoch 3 boundary — one item with the engine flag contract (the operator, 2026-10-07, at the chunk's plan and at this wrap's escalation: record as provisional). Its windowed witness is owed on the working route's "Stand a11y assertions".
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/

## 2026-10-07-audit-corrections — the stand's role and name tables stated once
**Section:** §7 → Accessibility tree output; `file:line` citations in §3 Keyboard test harness · §5 Test harness pattern · §8
**Change:**
- Accessibility tree output: the roles and input names the stand checks expect of the 15 controls are stated once, as the tables `controls` and `INPUT_NAMES` of the stand checks' shared module `tests/blitz-tests/tests/common/mod.rs`, beside `rendered`, the ids rendered per task — stated expectation, never a second role or name mapping (was: each check held its own copy; the body named no table).
- 5 of 16 citations into the edited stand checks re-pointed by the measured line map; 11 keep their numbers.
**Why:** the chunk moved the tables out of `stand_accessibility_ids.rs` and `stand_snapshot.rs` into one module, line for line — 18 control rows naming 15 controls, six input names, 26 rendered rows, none added, dropped or renamed. No criterion, role, name or focus assertion changed.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/

## 2026-10-07-driver-session — the session library names no accessibility feature; founder's rulings: the refresh on change and the falsy clear ratified
**Section:** §1 A11y Scope Summary (Dioxus crates · blitz-tests · citations) · §2 A11y Strategy (the refresh on change · the changed set)
**Change:**
- Dioxus crates: the session library `escher-driver`, which seven_guis now depends on, names no `dioxus-native-dom` feature, so the stand binary's build is unchanged and its `escher-session` host builds no accessibility code; the entry that first reads the snapshot through a session decides the feature.
- The shell's refresh of the platform tree on change, and the changed set it reads: two clauses were PROVISIONAL, pending the founder's word at the Epoch 3 boundary; now ratified by the founder (2026-10-07). The windowed witness is still owed, and the body says so.
- The bridge's falsy clear of its 27 boolean attributes (§5): no body text changes — the body never carried its provisional status; it is now ratified, no longer PROVISIONAL per the 2026-10-06-snapshot-state-fidelity entry.
- Citations: root `Cargo.toml:57` → `:58`, `:141` → `:143`, `:54-55` → `:55-56`; the blitz-tests `accesskit` line → `:33`.
**Why:** the chunk put a crate between the stand and the Dioxus bridge without enabling its feature. The founder ruled on the two items at the Epoch 3 boundary (the founder, 2026-10-07, in the overseer session, relayed verbatim by the overseer), superseding the provisional answers by rule.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/
