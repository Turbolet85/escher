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

## 2026-10-07-sink-target-allowlist — the obs sink described with its drop
**Section:** §3 A11y Assertion Harness Contract (the closing NOT YET MEASURED note)
**Change:** was "behind an allowlist scrub" that "redacts `text`, `value`, `html` and `attrs` at any target"; now "behind a target allowlist" that prints only the safe fields for `blitz*` and `accesskit_xplat` targets, redacts those four fields for escher's own targets (`escher_*`) and drops a record from any other target whole, at every level; the line is "per printed event". The note still says no a11y violation schema is defined against that format.
**Why:** the sink this note describes gained a third outcome in this chunk; a future a11y violation record reaches the log only under an engine or an `escher_` target.
**Ref:** .andromeda/runs/2026-10-07T08-23-43-wrap/

## 2026-10-07-settle-detection — settle neither reads nor drains the changed set
**Section:** §2 A11y Strategy → Accessibility tree lifecycle (the `changed_nodes` bullet)
**Change:** the bullet now states that a harness settle (`Harness::settle`, the driver session's `act`) neither reads nor drains the changed set, so a step settled through it leaves the tree refresh, and a later diff, exactly what the step marked; and that the set is non-empty on a freshly booted harness document — the boot's own in-document mutations mark it and nothing has drained it — so a check that reads `has_changes()` as "this step changed something" drains first. The ratified refresh-on-change and the changed-set contract are unchanged.
**Why:** a settle that drained the set would starve the shell's refresh and the later diff; the boot reading was measured when the chunk's idle check failed its plan's letter.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/

## 2026-10-07-command-and-refusal-schema — the driver's `disabled` cause is stated on the presence reading
**Section:** §7 Screen Reader Support → Accessibility tree output (the snapshot's `enabled` clause)
**Change:** beside "`enabled` follows the presence of `disabled`", the clause now records that the driver's `disabled` refusal cause is stated on the same presence reading — its meaning reads "the element carries the `disabled` attribute, which the snapshot reads as not enabled" — named and worded only, with nothing yet detecting it; and that the driver's schema defines no role or name set of its own: `role` and `name` are field names of its result nodes.
**Why:** the chunk worded the cause, and a cause worded on the attribute's value instead of its presence would disagree with what the snapshot reads for `disabled="false"`. Rule for later chunks: the entry that detects `disabled` reads the snapshot's `enabled`, never the attribute's value.
**Kept:** no detector of this plan covers the addition; it was raised from the plan's reviewed list.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/

## 2026-10-07-act-by-id — the session library names `accessibility`; both stand binaries compile the tree override, neither an adapter
**Section:** §1 A11y Scope Summary → Dioxus crates
**Change:** the row was "the session library `escher-driver` … names no feature of `dioxus-native-dom` either — it reads no snapshot — so the stand binary's build is unchanged and its `escher-session` host builds no accessibility code; the entry that first reads the snapshot through a session decides the feature then". Now: `escher-driver` names `accessibility` on `dioxus-native-dom`, unconditionally and with no `[features]` table of its own, because its executor reads the snapshot; the feature reaches seven_guis through that edge although seven_guis' own manifest names none, so both its binaries — the windowed `seven_guis_native` and the `escher-session` host — compile dioxus-native-dom's accessibility modules (the `accessibility_tree` override that carries `author_id`, the snapshot model, its text and its diff). Neither builds a platform adapter: `accesskit_xplat` and `accesskit_winit` are in neither graph, `accesskit` was already in both through blitz-dom, and the lockfile is byte-identical. A crate gets the feature by naming it or by depending on a crate that names it (was "only by naming it").
**Why:** the entry the row deferred the decision to has run: a driver action returns a diff, and the diff does not exist without the snapshot model. The reach was measured on the dev host's target by five dependency-graph reads before and after the one manifest line. Standing consequence: the stand binary still hands no assistive technology a tree, so a windowed run still proves the boot only; the windowed witness of the tree refresh stays owed where the route carries it.
**Kept:** "the seven_guis stand binary names none … and builds no platform adapter" — true of its own manifest and of both graphs; the statement that the stand's windowed binary builds blitz-shell without `accessibility`.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/

## 2026-10-07-refusal-detection — the `disabled` cause is detected
**Section:** §7 Screen Reader Support → Accessibility tree output
**Change:** the driver's `disabled` refusal is detected on the snapshot's presence reading and no other — `Session::run` refuses a `click` or a `type` when the node's `enabled` is `Some(false)`, the first of its three screen-level checks, with nothing dispatched and focus as it was; on the stand `crud-delete` and `flight-return-date` are refused and acted on once they read enabled (was "named and worded only, with nothing yet detecting it").
**Why:** the chunk built the detection. Rule kept: `disabled` is read from the snapshot's `enabled`, never from the attribute's value, from focusability or from a role list of the driver's own.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/

## 2026-10-07-driver-command-spans — the obs log format restated as two record classes; the driver's third dependency
**Section:** §1 → Dioxus crates · §3 (the closing NOT YET MEASURED note) · §7 (one citation)
**Change:**
- §3: the sink's format reads as two record classes — one line per printed event and one per closed span of an admitted target — both behind the same allowlist; the second is PROVISIONAL in obs-plan §6, which owns the mark. No a11y violation schema is defined against it (unchanged).
- §1: "the lockfile is byte-identical" now reads as the act-by-id chunk's measurement; the session library's third dependency, `tracing`, taken with no feature, adds one lockfile line and no package, and neither platform adapter joins either seven_guis graph.
- Citations re-pointed (`Cargo.toml:13-15` → `:13-16`; `execute.rs:205-223` → `:289-307`).
**Why:** the note restates obs-plan §6, which changed; the dependency line restates the driver's manifest.
**Kept:** "no `[features]` table of its own" — still true.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-09-founder-rulings — the sink's second record class reads ratified
**Section:** §3 (the closing NOT YET MEASURED note)
**Change:** the parenthesis on the closed span's line read "the second class is PROVISIONAL in obs plan §6, which owns the mark"; now "the second class was ratified by the founder, 2026-10-09, as obs plan §6 states".
**Why:** the note restates obs-plan §6, which the founder's ruling changed (the founder, 2026-10-09, relayed verbatim by the overseer as his chosen option label; the English wording of the ruling is the overseer's).
**Ref:** .andromeda/runs/2026-10-09T19-17-00-wrap/

## 2026-10-09-audit-corrections-agent-surfaces — the value reading is proven on a textarea
**Section:** §7 Screen Reader Support → Accessibility tree output (the snapshot-model bullet)
**Change:** the clause "`value` is the editor's text, else the `value` attribute" now says the reading holds for a textarea as for a text input: on the in-file fixture of `stand_snapshot_state` a textarea named by `aria-label` reads role `MultilineTextInput`, a non-empty name, its HTML `id` as its stable id and a box wider and taller than 0, reads `""` untyped and its typed text back as its `value`, unmasked, and after the click exactly it reads `focused` and is the tree's focus, in both layout modes; it is a test-only element of that fixture. Nothing was retired: §7 held no textarea value reading before.
**Why:** the snapshot's `value` reader already took a textarea, and no check read one — the one mutant the Epoch 4 code audit left alive. The textarea is named by `aria-label` because a `label for` binds an `input` element only.
**Kept:** the password and file-input mask clauses are unchanged; the textarea's text is synthetic and reads back unmasked by design.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — autofocus is read by the attribute's presence; the stand's adapter absence re-read on the merged lock
**Section:** §5 Keyboard Navigation → Focus order per layout (engine), Script and framework exposure · §1 → Dioxus crates
**Change:**
- Focus order: was "the latest mounted focussable node with `autofocus="true"` is focused on flush"; now the node whose `autofocus` attribute is present with any value but "false" — bare, empty or "true" — and `autofocus="false"` is ignored.
- Script exposure: was "`autofocus` reflection writes the value "true" because blitz-dom's autofocus handling expects it"; now it writes "true", one of the values the handling takes.
- Dioxus crates: a re-read clause — on the merged lock, where the accesskit family moved (accesskit 0.25.1, accesskit_unix 0.24.0, accesskit_android 0.9.0), the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit`.
**Why:** upstream changed the engine's reading of the attribute (`== "true"` became `!= "false"`), with a five-test check of its own. No stand source carries `autofocus`, so no stand boot's focus moves; the a11y leg is unmoved at 6 + 6 + 3.
**Kept:** the bridge still removes a falsy Dioxus `autofocus`, so that bullet stands; the two app bullets (the browser's new-tab input, the Preact TodoMVC input) state no value and stand.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — a click on a plain button leaves nothing focused, measured
**Section:** §5 → Focus restoration
**Change:** gains one line: after an accepted pointer click on a plain `button` no node reads `focused` in the snapshot and the accessibility tree's focus carries no author id, in both layout modes and under both builds. The section held one click line before — a click on a non-interactive area clears focus — and no statement on a button.
**Why:** the chunk's plan took this reading to be recorded here already, as read from code; this section held no such line, and the chunk measured it on a restated stand check. Whether a click should focus the button is not decided here: it is owned by the route entry that works keyboard focus.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-snapshot-states — the tree states selected, toggled and invalid, and the snapshot takes them from it
**Section:** §2 A11y Strategy → Role derivation · §3 → A11y testing tool pick · §4 → Per-component pattern catalog · §7 → Accessibility tree output (the snapshot-model bullet)
**Change:**
- §2: gains the builder's three state reads, on every element whatever its role, each value trimmed and compared exactly in lower case — `aria-selected` `true` / `false` → `selected`; `aria-pressed` `true` / `false` / `mixed` → `toggled`; `aria-invalid` `true` / `grammar` / `spelling` → `invalid`, an absent, empty or `false` value nothing, any other value `Invalid::True`. `TRUE` reads unrecognized. `aria-checked` is not read, so a node's `toggled` is exactly its `aria-pressed` reading. A role that does not support an attribute reads it as one that does.
- §3: gains the state check — one stated table per attribute, its row count asserted, both layout modes — run by the workspace test leg and per package; the `a11y` leg stays three files.
- §4: the catalog gains "ARIA state attributes"; `aria-checked` sets none of the three, so an ARIA checkbox or switch states no checked reading through it.
- §7: was "the tree gives an element node no value, checked or disabled property, so the snapshot reads those three from the element"; that stands, and beside it the tree now gives `selected`, `toggled` and `invalid`, which the snapshot takes from the tree and never from the element — `NodeState` at seven fields, a mixed toggle reading `pressed` false and a grammar or spelling error reading invalid, three text tokens after `value=` and before `focused`, each stand reading equal to its accessibility node's.
**Why:** ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed by the overseer; confirmed by him at this wrap's card, relayed the same way): the builder writes the three states on the node. State is what the tree exists to carry, and one parser of each attribute keeps the snapshot and the tree from disagreeing.
**Kept:** what an assistive technology reads of the three is not measured; the windowed witness and the ARIA checkbox reading are each owned by a route entry.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-snapshot-states — the stand states a listbox, two toggle buttons and two invalid inputs; two gaps recorded
**Section:** §5 → Per-surface keyboard shortcuts · §7 → Accessibility tree output (the stable-id bullet, the actionable-key bullet, a new stand bullet) · §8 → Error recovery
**Change:**
- §7, new bullet: by attribute alone, with no element, class, style, key, id or order moving, the CRUD list states `role="listbox"` and each row `role="option"` with `aria-selected` (reading `ListBox` / `ListBoxOption`), the flight booker's two mode buttons state `aria-pressed` (role Button and the focus order unchanged) and its two date inputs state `aria-invalid`. The actionable-key check still returns nothing.
- §7, the stable-id bullet: "the 15 interactive controls carry … a non-empty accessible name" stands for those 15; an element outside them now carries an interactive role — the list reads `ListBox` with the accessible name `""` — so not every interactive-role element on the stand is named.
- §7, the actionable-key bullet: was "keyed by an `id` attribute alone with no `tabindex`, role, focusability or Tab-order change"; that holds of the keying chunk, and each CRUD row now also reads `ListBoxOption`, still with no `tabindex`, no focusability and no Tab-order change.
- §5: gains the CRUD rows as options of a listbox that are not focusable — no keyboard reaches an option; recorded, not closed.
- §8: the invalid-date bullet gains `aria-invalid` on `flight-start` and `flight-return-date`, so the invalid state is in the tree and the snapshot and not carried by the class alone; the class and its styling stay.
**Why:** the stand's tasks state the three states so a snapshot can read them; the roles and attributes are the operator's answers at the chunk's plan forks.
**Kept:** no `tabindex` on a row and no name on the list in this chunk — a focusability change would move the pinned Tab order. Each gap is owned by a route entry.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-mcp-surface — the stand graph re-read; MCP results read the snapshot model
**Section:** §1 (the Dioxus crates bullet) · §7 (the snapshot model's readers)
**Change:**
- §1: re-read with escher-driver's fourth dependency, `serde_json` — one lock line, no package, neither platform adapter in the stand graph (`accesskit_(xplat|winit)` 0 rows).
- §7: an MCP tool result is a further reader of the snapshot model and of nothing else — a `snapshot` result is `to_text`'s output, an acting call's its JSON line with the diff, no second role or name mapping; on the stand a `snapshot` block equals `Session::run`'s screen text byte for byte in both layout modes.
**Why:** the snapshot's text reaches an MCP client by the crossing the founder ratified (2026-10-10, relayed by the overseer; his confirm at this wrap's Validate). No interactive element and no assistive-technology reading entered.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-12-self-description — the password over MCP measured; the node keys' help and `differs` as readers
**Section:** §7 Screen Reader Support → Accessibility tree output (the snapshot-model bullet, its MCP-reader clause)
**Change:** a password input typed through an MCP `type` reads `MASKED_VALUE` in the tool result, with role `PasswordInput` and a non-empty name, and the typed sentinel occurs 0 times, in both layout modes. The input is a test-only element of `stand_mcp_masked`'s fixture; no stand task or app gained one. The snapshot model gains two readers, closed words only: the node keys' help, which `help` serves (each reading stated, the boolean model's two limits among them), and `DiffNode::differs`, which names the readings that differ in node-key order — the Flight Booker's Book names eight nodes that only moved, each `[bounds]`, in both modes.
**Why:** the chunk added an interactive test fixture whose role, name and masked value through the MCP reader it measured, and two readers of the model that add no role or name mapping of their own.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-headless-screenshot — a password painted masked; the shot one more reader of bounds
**Section:** §6 Visual Design Verification → Focus ring · §7 Screen Reader Support → Accessibility tree output (the snapshot model's readers)
**Change:**
- §7: was a password's typed text in no field of the snapshot or tree; now in no painted frame either — bullets laid out by `BaseDocument::layout_as_text_input`, in every document and renderer through blitz-paint, windowed included; equal-length values paint identical frames, focused or not, both layout modes; the snapshot still reads `MASKED_VALUE`, the tree unchanged; a click inside a filled field places the caret by the real glyph widths — not measured, owned by a CARRY on "Stand requirement sweep"; the windowed paint not run here.
- §7: the driver's `screenshot` is one more reader of the model's bounds and of nothing else — an element shot by stable id is its bounds rounded outward and cropped to what can be seen — and a shot changes nothing: no focus moves, `has_changes` stays false, the next diff is empty, both layout modes.
- §6: a focused password input paints its caret at the bullet of its index in the input's caret colour (`caret_color`), its bullets in the text colour, the field scrolling on the bullets.
**Why:** painted colours, the caret and focus are a11y surface; the mask is the founder's fork 2, an engine change for every document ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer).
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the 28-name falsy predicate, a falsy `inert` cleared; the founder's word respelled
**Section:** §5 Keyboard Navigation → Script and framework exposure · §2 A11y Strategy (the three states written on the node) · §7 Screen Reader Support (the MCP crossing)
**Change:**
- §5: was one of the bridge's 27 boolean attributes, the `isBoolAttr` list of Dioxus 0.7.10's web interpreter; now one of 28 — upstream's private `is_bool_attr`, a closed `matches!` set, the bridge's own `BOOLEAN_ATTRIBUTES` retired and `mutation_writer.rs` byte-identical to upstream's at `434e0661`. A framework-written `inert: false` is now removed, so it no longer makes content inert. Both attribute paths and the `hidden` consequence are unchanged.
- §2 and §7: the two rulings («Ратифицировать», «Ратифицировать как есть») now read "relayed verbatim by the overseer as his chosen option label …; the English wording is the overseer's", the form of the entries they copy.
**Why:** a falsy `inert` no longer hides content from focus and the tree — upstream's predicate, merged, focus order and reach otherwise unchanged (the a11y leg reads 15 passed). The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's); the respelling was folded at the cascade sweep under playbook rule A, added at the same card.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-stand-contrast-harness — the contrast verification harness built; the stand's 18 control rows measured
**Section:** §3 A11y Assertion Harness Contract (Contrast verification harness) · §3 → contrast-verification-harness-setup · §6 Visual Design Verification (Color contrast · the closing NOT YET MEASURED)
**Change:**
- §3: was "contrast checks are observed absent"; now built — `Harness::text_contrast` reads an element's text/background pair and font metrics from computed styles as a `TextContrast` (`ratio` by `contrast_ratio`, WCAG 2.x; `required_ratio` 3.0 large, else 4.5), and `stand_contrast` states the 18 control rows as a table asserted row for row in both layout modes, beside a painted-pixel witness; in the workspace `test` leg, not the `a11y` leg.
- §3 → contrast-verification-harness-setup: discharged (was "contrast checks are observed absent").
- §6 Color contrast: the stand's reading — the reader's rules (the text-bearing element; the background composited to the first opaque ancestor, else the root-else-`body` canvas over white; `None` under opacity < 1, a filter or a background image; large text at 24 px, or 14 pt at weight 700), and the record at boot, 800 × 600, identical in both modes: pass 9 · fail 5 (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, white on `#4a6cf7` at 4.39 against 4.5) · exempt-inactive 3 (exactly `enabled == Some(false)`) · no-text 1 (`timer-duration`); no text large; no colour differs from its declared value; the painted witness within 1 per channel. SC 1.4.3 is not met on the five accent buttons.
- §6 NOT YET MEASURED: narrowed — the stand controls' pairs are measured; focus-ring, target-size and typography tokens, the CRUD list rows, the text outside the controls and the colour-changing states stay unmeasured.
**Why:** the chunk built the instrument and took the reading. It gates nothing: the re-colouring, the states not measured, the CRUD rows and the text outside the controls are owned by "Stand a11y assertions" (CARRYs on the operator's word at the P5 review).
**Kept:** the key's label `contrast-verification-harness-setup`, as `a11y-ci-gate-wire` keeps its own after discharge; §6's s05 "Observed absent — contrast checks" search record, true of its slice.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/

## 2026-10-13-range-input-interaction — the range input becomes keyboard-operable and clicks focus it
**Section:** §5 Keyboard Navigation (Keyboard handling · Focus restoration · Test harness pattern) · §7 Screen Reader Support (Accessibility tree output)
**Change:**
- §5 Keyboard handling: in every document, a focused range input with no `disabled` attribute (presence) steps up on ArrowRight/ArrowUp and down on ArrowLeft/ArrowDown, and goes to its minimum on Home and its maximum on End. Other keys do nothing. Each change marks the changed set and fires one `input`.
- §5 Focus restoration: a click focuses a range input and moves no value, so a label's click focuses it too; a main-button pointerdown sets the value by position. Was: no click focused or moved a range input.
- §5 Test harness pattern: `range_input_interaction` (a 7-row keys table and a 5-row clicks table, both modes), and `stand_act_range` driving the Timer slider. After two Tabs an arrow step leaves it focused, with role `Slider`, its name and the forward focus sequence unchanged.
- §7: the snapshot's `value` for a range input is its current value (`range_input_value`), was its `value` attribute. The file-input clause stands.
**Why:** SC 2.1.1 keyboard reach on the stand's slider. The keys and the click focus were ratified by the founder (2026-10-13, his fork answers «Стрелки + Home/End (Рекомендую)» and «Клик даёт фокус (Рекомендую)», confirmed on this wrap's answer card «Подтвердить (Рекомендую)», each by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label).
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/

## 2026-10-13-stand-keyboard-harness — Enter and Space activate a focused control, and a click focuses a button
**Section:** §5 Keyboard Navigation → Keyboard handling (engine and shell) · Focus restoration · §7 Accessibility tree output (the diff)
**Change:**
- Keyboard handling: a pressed Enter on the focused node, after the text-input and range arms, dispatches a Click on a `button`, a `summary`, an `input` of type `submit`/`reset`/`button` or an `a` with `href` with no `disabled` attribute. A released Space (`handle_key_up`, KeyUp's first default action) does the same for a `button`, a `summary` or an `input` of type `submit`/`reset`/`button`/`checkbox`/`radio`. A link takes no Space; a text or range input takes neither key there. The app's handlers run, then `handle_click`. In every document. SC 2.1.1 on the stand's buttons is met by measurement: 20 rows, and the driver's `press enter`/`press space`.
- Focus restoration: a pointer click on a `button` or a `submit`/`reset`/`button` `input` focuses it, then submits a submit button's form owner, in every document. A `disabled` attribute still stops the walk; any other click still clears focus. On the stand a click focuses each button and Tab continues. Was: a click on a plain button left no element focused, whether it should focus owned by a route entry.
- §7 diff: the Book names seven bounds-only nodes and `flight-book` `[focused, bounds]`. Was eight `[bounds]`.
**Why:** engine changes for every document; keyboard activation is a boundary widening. The founder answered himself by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label: «Клавиши нажимают (Рекомендую)» and «Клик даёт фокус (Рекомендую)», 2026-10-13. He confirmed on this wrap's answer card, 2026-10-14, «Подтвердить (Рекомендую)», that the keyboard-handling proposal lies inside them.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/

## 2026-10-13-stand-keyboard-harness — `disabled` by presence, the CRUD list's keys and the keyboard check
**Section:** §5 Focus order · Per-surface keyboard shortcuts · Script and framework exposure · Test harness pattern · §3 Keyboard test harness · §7 Accessibility tree output · §8 Error recovery · §9 CI Integration
**Change:**
- §5 Focus order: an element is focusable only with no `disabled` attribute, any value, on any element. Disabled-ness is keyed one way, by presence. Was: focusability parsed the value as a bool and `disabled="false"` or a bare `disabled` stayed focusable. Known limit: a `<div disabled tabindex=0>` is not focusable, unlike in browsers, its narrowing owned by a CARRY on "Stand requirement sweep".
- §5 Per-surface: the CRUD list takes `tabindex="0"` (the third Tab stop), and Arrow/Home/End move the selection. The rows stay unfocusable options; the tree's focus is the list; no `aria-activedescendant`, owned by a CARRY on "Stand a11y assertions". Was: no keyboard reaches an option.
- §5 Script exposure: `inert` the 28th name the check reads; the engine honours no `inert`, owned by a CARRY on "Stand a11y assertions". Was: "it no longer makes content inert".
- §5 Test harness pattern and §3: + `stand_keyboard`, and the driver's Enter and Space in `stand_act_keys`. Its focus reading drops the engine's root fallback.
- §7: the snapshot's `enabled`, a present `disabled` not focusable either (was "both stay focusable"); the CRUD list focusable and a Tab stop in the actionable-key rule; the open gap restated as tree focus on the list; the 2026-10-06 Tab order bound to its chunk.
- §8: the forward sequence read against `common::tab_order`, CRUD six stops.
- §9: the a11y leg runs 6 + 6 + 4 tests (was 6 + 6 + 3).
**Why:** `disabled` by presence is the founder's own answer «По наличию (Рекомендую)» by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label; the `<div>` limit is named on the operator's condition. The list's keys are the operator's F5 decision. The `inert` reading corrects a claim about unedited code to its measured limit.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/

## 2026-10-14-stand-a11y-assertions — SC 1.4.3 met on the stand, three contrast tables, the a11y leg at six files
**Section:** §3 Contrast verification harness, A11y testing tool pick · §6 Color contrast and its NOT YET MEASURED note · §9 CI Integration
**Change:**
- §6: the stand's accent one step darker (rest `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`; white 5.47 · 6.83 · 8.27). `stand_contrast`'s record now 14 · 0 · 3 · 1, the five accent controls at 5.47 (was 9 · 5 · 3 · 1, `#4a6cf7` at 4.39). One shared reader classifies two more tables: `TEXT_ROWS` 14 (list rows, text outside the controls, all pass) and `STATE_ROWS` 19 (reached states, hover and active included; 18 pass, 1 exempt). Hover and active are read by pointer; a button with no `:active` rule keeps its hover pair. SC 1.4.3 met on every measured non-exempt row (was "not met on the five accent buttons"). The note no longer lists the list rows, outside text and colour states as unmeasured; the route-entry owner of all four is retired.
- §3: `stand_contrast` states three tables and runs in the `a11y` leg; the state check is "not in the `a11y` leg, which runs six files" (was "the leg staying its three files").
- §9: the leg runs six files — `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` added, the job unchanged; was "it runs existing tests and adds no stand assertion".
**Why:** the re-colour is the operator's F5, a stand change and no engine change; the leg's widening is the founder's F4 «Расширить задание a11y (Рекомендую)», by question dialog in the overseer session, relayed by the overseer as his chosen option label — no new job, no required status check, no repository setting.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/

## 2026-10-14-stand-a11y-assertions — the enabled-state walks, the list named, three gaps re-homed to residuals
**Section:** §1 Dioxus crates · §2 Accessibility tree lifecycle, Role derivation · §5 Keyboard handling, Script and framework exposure, Per-surface keyboard shortcuts, Test harness pattern · §7 Accessibility tree output
**Change:**
- §5: Enter and Space read as the click of `crud-update` and `crud-delete` once enabled (4 rows); the walk asserted in two enabled states against `ENABLED_TAB_ORDER` (six and eight stops), every control reached by Tab. The `inert` gap and the missing `aria-activedescendant` (SC 4.1.2 work outside the tier-0 target) are owned by `.andromeda/residuals.md` lines (target: next), was a CARRY on "Stand a11y assertions". The list is named `People`.
- §7: the list's empty name retired; every interactive-role node of the four lean tasks named over nine roles, `REGION_NAMES`; the `&dyn Document` tree equal to the document's, carrying stable ids only. One gap open, not two.
- §1 · §2: the refresh's two halves proven headlessly on the stand; the adapter on the stand binary and its windowed AT witness (ids, names, the three states, the refresh) owned by a residuals line (target: next), was "a route entry" / "owed on the working route's entry".
**Why:** the three re-homes are the founder's F1 «Пока нет, перенести (Рекомендую)», F2 «Оставить записанным (Рекомендую)» and F3 «Оставить, перенести (Рекомендую)», each by question dialog in the overseer session, relayed by the overseer as his chosen option label, placed as residuals lines by the operator's P5 word. No engine, boundary or PROVISIONAL mark moves.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/

## 2026-10-14-stand-requirement-sweep — every stand task keyed; the click-without-role count; three deferrals' owners
**Section:** §7 Accessibility tree output (the actionable-key rule bullet · the snapshot-model bullet) · §5 Focus order per layout (the `disabled` bullet) · §4 Per-component pattern catalog (the ARIA state attributes bullet)
**Change:**
- §7: `unkeyed_actionable()` returns nothing on the three non-lean tasks too, 0 · 0 · 0, Circle Drawer's dialog open included; was 2 · 3 · 676. The elements that react to a click and state no role are counted and pinned — 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676; no snapshot field says it, that reading owned by a `.andromeda/residuals.md` line (target: next). The password click caret is measured, index 5 vs 2, its fix a residuals line; was "not measured", a CARRY on "Stand requirement sweep".
- §5: the `disabled` narrowing's owner is a residuals line; was a CARRY on that entry.
- §4: reading `aria-checked` (as `checked` or `pressed`) is owned by a residuals line, `accessibility_states`' fourth test its pin; was "a route entry".
**Why:** measured at this chunk. The owners rest on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)», C7 «Отложить (Рекомендую)», C11 «Замерить сейчас, исправить потом (Рекомендую)» and C15 «Оставить по наличию, отложить (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option labels, placed as residuals lines by the operator's word on the P5 card.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/

## 2026-10-14-quality-gates — the stand's a11y files also run in `stand-repeat`
**Section:** §9 CI Integration
**Change:** `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`, as `stand_*` files, also run three times in the `stand-repeat` leg and its job `Stand checks repeated`, under the zero-failure budget (test-plan §10). The a11y leg and job are unchanged, their per-file counts as before.
**Why:** the stand-repeat leg runs every `stand_*.rs` file, on the founder's «Повторное задание, ноль сбоев (Рекомендую)», relayed verbatim by the overseer as his chosen option label.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/

## 2026-10-16-handler-splits-and-readme — §5's click and range rows cite the named functions
**Section:** §5 Keyboard Navigation (the button click focus row · the range click row · the range keys row)
**Change:** the button click focus row cites the private `click_button`; the range click row cites `press_range_input` for the pointerdown; the range keys row cites the range arm's gate beside the key mapping. Every claim stands as it read — focus through `generate_focus_events` / `set_focus_to`, the keys, the order and the `disabled` presence reads unchanged.
**Why:** the code audit's complexity split moved those bodies into private functions with no behaviour changed for any document; the a11y leg passed over its six files unchanged.
**Ref:** .andromeda/runs/2026-10-16T14-54-43-wrap/

## 2026-10-17-upstream-sync-engine-debts — upstream's six builder reads; `aria-labelledby` names; the two merged checks (§2, §3, §7)
**Section:** §2 A11y Strategy (Role derivation) · §3 A11y Assertion Harness Contract (A11y testing tool pick) · §7 Screen Reader Support (Accessibility tree output)
**Change:**
- Role derivation: gains upstream's six reads as upstream wrote them — `aria-labelledby` (after the `<label>` → input loop, its targets' text the computed `label`, outranking a `<label>` and an `aria-label`, accname 2B, the `aria-label` the fallback), `aria-describedby` (`described_by`), `aria-description` (the description), `aria-expanded`, `aria-disabled` (`true` → disabled) and `aria-level` (tag and role defaults); the three closed-set state reads and the trimmed `aria-label` read kept, one read per attribute; the description and the three closed values leave only to the adapter, which no stand binary builds, so what an assistive technology reads of them is not measured.
- A11y testing tool pick: gains `accessibility_label` 10 and `accessibility_description` 3, asserted with `verify_that!` outside the a11y leg's six files.
- Accessibility tree output: the `aria-label` bullet — an `aria-labelledby` with text outranks it; the `<label>` bullet — an input carrying `aria-labelledby` is named by its targets instead; the snapshot's `name` takes that computed text; was «the tree gives an element node no value, checked or disabled property»; now no value or checked property, a disabled one only from `aria-disabled`, which the snapshot's `enabled` never reads.
**Why:** the merge, keeping upstream's reads as written; `aria-labelledby` names inside the ratified name crossing and the description to the adapter ratified as upstream's behaviour by the founder's own word at P4 fork 2, relayed by the overseer, both description sources confirmed by his word on this wrap's answer card.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
## 2026-10-18-engine-debts — §2–§4, §7: the toggle per role and the snapshot's sources
**Section:** §2 A11y Strategy (Role derivation) · §3 A11y Assertion Harness Contract (testing tool pick) · §4 ARIA Patterns & Roles (ARIA state attributes) · §7 Screen Reader Support (Accessibility tree output)
**Change:**
- Role derivation: was «with no role consulted … `aria-checked` is not read, so a node's `toggled` is exactly its `aria-pressed` reading»; now `aria-pressed` off the five checkable roles, `aria-checked` on `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` alone, `role="switch"` → `Switch`, a native checkbox or radio reading neither; native checkedness written on no tree node — a measured absence, owned by a `.andromeda/residuals.md` line.
- §4: was «`aria-checked` sets none of the three … owned by a `.andromeda/residuals.md` line … C7 … the fourth test standing as its pin»; now the per-role read, the fourth test its pin.
- §3: `accessibility_states` 5 tests (was 4 at its row); a bullet for `accessibility_merged_states` 3, `inert_subtree` 4, `accessibility_active_descendant_unread` 1, outside the a11y leg's six files.
- §7: the snapshot's `pressed` off the checkable roles, `checked` a native input's state else a checkable role's toggle, the help states its source; the password click caret by the bullets (was «the real characters' widths … C11»); the unread `aria-activedescendant` kept and pinned by its check (was «owned by a `.andromeda/residuals.md` line»).
**Why:** F2 A with the `switch` mapping and the native exclusion, F3 B and F6 A — PROVISIONAL, delegate operator, standing for the founder's own word at the Epoch 2 Provisional discharge.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
## 2026-10-18-engine-debts — §5, §6: `inert` and `disabled` in focus order, keys and the hit
**Section:** §5 Keyboard Navigation (Focus order per layout · Focus restoration · Keyboard handling · Per-surface keyboard shortcuts · Script and framework exposure · Test harness pattern) · §6 Visual Design Verification (Visibility and hit-testing)
**Change:**
- Focus order: an element is focusable when it is not inert (presence, on itself or an ancestor — `Node::is_inert`) and no disabled form control; `disabled` read by presence on a form control alone (`ElementData::is_disabled`), so a `<div disabled tabindex=0>` takes focus — was «on any element … the C15 limit»; the focus-fixup limit stated, owned by a `.andromeda/residuals.md` line; disabled-ness keyed by `is_disabled` for the pointer, click, activation and focus gates, the `DISABLED` state still by presence.
- Focus restoration: a disabled form control stops the click walk, a disabled ancestor `div` not.
- Keyboard handling: a `disabled` attribute on a `summary` or an `a` gates neither Enter nor Space.
- CRUD list: the unread `aria-activedescendant` pinned by `accessibility_active_descendant_unread` (was owned by a residuals line).
- Script and framework exposure: was «the engine honours no `inert` … F3 … whether an inert subtree takes a hit unmeasured»; now a truthy `inert` takes focusability, hits and the tree away, a falsy one is removed.
- Test harness: `stand_keyboard`'s `disabled` and `inert` tests restated.
- Visibility and hit-testing: an element carrying `inert` and its subtree leave the tree and the snapshot, a hit passes through it, removing `inert` restores both.
**Why:** F1 A, F3 B and F7 A — PROVISIONAL, delegate operator, as the first entry states.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
## 2026-10-18-one-virtual-clock — §5: the double click on the document's clock, PROVISIONAL
**Section:** §5 Keyboard Navigation (Focus restoration)
**Change:** new: two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock — on a harness-built document the harness's `VirtualClock`, so two driver `click`s on one point with no `advance` between them are a double click, selecting a word in a text input — while each still focuses its target as one click does. PROVISIONAL.
**Why:** the founder's fork F3 (A: the double-click window reads the one clock), answered by the operator as the founder's delegate (delegate the operator, 2026-10-18), PROVISIONAL, for the founder's epoch-boundary batch (the operator, 2026-10-18, at the wrap resume). Single-click focus is unchanged.
**Ref:** .andromeda/runs/2026-10-18T08-45-11-wrap/
## 2026-10-18-engine-debts — the PROVISIONAL batch ratified by the founder (Epoch 2 close): two batches, 13 marks
**Section:** §2 A11y Strategy · §4 ARIA Patterns & Roles · §5 Keyboard Navigation · §6 Visual Design Verification · §7 Screen Reader Support
**Change:** every PROVISIONAL mark now reads `ratified by the founder (2026-10-18)` (inside parentheses `ratified by the founder, 2026-10-18`), the clause each one qualifies unchanged:
- engine-debts, 12 marks: the toggle read per role and `role="switch"` mapping to `Switch`, `disabled` narrowed to form controls and the `inert` read in focus order, the click walk stopped by a disabled form control alone, the activation keys' disabled gate, `inert` taking focusability, hits and the tree, `accessibility_active_descendant_unread`'s pin, the snapshot's `checked` source and its mixed-reads-false, the password caret by the bullets;
- one-virtual-clock, 1 mark: two `click`s with no `advance` between them a double click on the document's clock.
Partly retires the PROVISIONAL clause of the earlier 2026-10-18-engine-debts and 2026-10-18-one-virtual-clock entries here; each one's other claims stand. The native-checkedness residual (`.andromeda/residuals.md`, target next) stays open, on the founder's «Исправить позже (Рекомендую)».
**Why:** the Provisional discharge at the epoch boundary, on the founder's own word: «Утвердить всё (Рекомендую)» on the engine-debts batch and on one-virtual-clock F1 B · F2 A · F2b A · F3 A (the founder, 2026-10-18, by question dialog in the overseer session, relayed by the overseer with his chosen labels verbatim), discharged at this wrap on the operator's direction (the operator, 2026-10-18, at the gated wrap's resume).
**Ref:** .andromeda/runs/2026-10-18T20-46-02-wrap/

## 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — `:focus-within` matches
**Section:** §5 Keyboard Navigation (Focus order per layout (engine))
**Change:** was «`:focus-visible` and `:focus-within` never match»; now `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state, while `:focus-visible` still never matches (PROVISIONAL). Focus order is unmoved: no stand style uses `:focus-within`, `stand_keyboard` and `inert_subtree` are green, and the a11y leg reads 6 · 43.
**Why:** upstream #1226's behaviour for every document, taken on F1 answered by the operator as delegate (2026-10-19, by question dialog in the phase session). It is PROVISIONAL until the Epoch 4 boundary batch.
**Ref:** .andromeda/runs/2026-10-19T19-54-24-wrap/

## 2026-10-19-shared-fixtures — the MCP masked password input reads the shared page
**Section:** §7 Screen Reader Support (the snapshot model's MCP masked-password clause, `stand_mcp_masked`)
**Change:** the MCP `type` password input was «a test-only element of `stand_mcp_masked`'s fixture» and is now a test-only element of the shared password page `fixtures::password_named`; its citation moves from the removed file-private page to `tests/blitz-tests/tests/fixtures/mod.rs`. The reading itself is unchanged: `MASKED_VALUE`, role `PasswordInput`, a non-empty name, the sentinel 0 times, both layout modes.
**Why:** the citation sweep read the cited lines deleted. The page moved into the shared module this chunk, with the same markup and ids (the playbook's «Accurate this-chunk addition»).
**Ref:** .andromeda/runs/2026-10-19T23-02-04-wrap/

## 2026-10-19-visual-lint — `text_contrast`'s first non-test caller
**Section:** §3 Contrast verification harness · §6 Color contrast
**Change:**
- §3: `Harness::text_contrast` was read by tests alone; its first non-test caller is now escher-driver's in-process visual lint (`Session::lint`, PROVISIONAL: in process only), whose `contrast` reading keeps the same exempt rule.
- §6: a new bullet — the lint's `contrast` reading: a node with direct non-whitespace text or a text control, read through `text_contrast` and reported below `required_ratio()`, exempt exactly where `enabled` reads `Some(false)`, `None` not measured, no second formula or threshold. The stand at boot reads no `contrast` finding on the four lean tasks in both layout modes, the nodes outside `stand_contrast`'s rows included; a planted low-contrast row reads 2.8490272 against 4.5, equal to `text_contrast`'s; the `a11y` leg reads 43 tests over 6 targets, unmoved.
**Why:** this chunk's own reader of SC 1.4.3 (the plan's reviewed expected amendment); the lint's in-process scope is PROVISIONAL for the founder's own word (delegate the operator, 2026-10-20).
**Ref:** .andromeda/runs/2026-10-20T00-54-29-wrap/

## 2026-10-20-select-by-id — the select's keyboard, tree and label, and the PROVISIONAL discharge
**Section:** §2 A11y Strategy · §3 A11y Assertion Harness Contract · §4 ARIA Patterns & Roles · §5 Keyboard Navigation · §6 Visual Design Verification · §7 Screen Reader Support
**Change:**
- §2: a select's changed choice marks the changed set (none for a choice of the chosen option); the builder's state read excepts a select's option (`selected` its chosen state, no `aria-selected`; its `label` attribute its `label`).
- §3: `accessibility_label` 10 → 13 tests.
- §4: ARIA state attributes — the same exception; Hidden content — an option under a `select` stays in the tree under `display: none` (was: `display: none` excludes every node), `hidden`, `inert` and `visibility: hidden` still excluding it.
- §5: a new keyboard bullet — the select's arrows, `Home` / `End`, typeahead over 1 s of the document's clock, `input` then `change` per change, the limits; a click focuses a select (was: a click on any other element unchanged), an option not clicked; a label's click reaches its bound labelable element; a new harness bullet — `select_interaction` and `stand_act_select`.
- §6: the visibility bullet's exclusion excepts a select's option; a new contrast bullet — the user-agent select box, its contrast not measured (no stand select).
- §7: a `<label>` names every labelable element, a hidden input none (was: the bound element must be an `input`); a new bullet — a select's options in the tree and the snapshot, its `value`, the `stand_act_select` reading.
- §5's `:focus-within` «(PROVISIONAL)» and §3 / §6's visual-lint «PROVISIONAL: in process only» dropped.
**Why:** this chunk's engine change (the plan's expected amendments), ratified by the founder, his own word relayed verbatim by the overseer (2026-10-20): F1–F4 «Утвердить все четыре (Рекомендую)» and his confirm of this wrap's answer card «Подтвердить все (Рекомендую)» (chunk inputs#I3, inputs#I4). The marks drop on his word of the same day (the take «Утвердить (Рекомендую)», the lint F1 «Новая команда lint (Рекомендую)»), on the operator's «Drop all now» at this wrap's halt.
**Ref:** .andromeda/runs/2026-10-20T05-49-12-wrap/
