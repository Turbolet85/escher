# Fan-out results — 2026-10-20-select-by-id (wrap 2026-10-20T05-49-12, resumed)

Path check: before `tree c4e0a9506c5756c8100232d83f6e74b8bad11d90 index 83edcd0df277aba59f2ce98472c64dca4b7bedb8 head 14818317c39789697ce4dc2ddb8572ad157a4d74`;
after `tree b9a376a0a707f2352e79d0fa1408b165d0935756`, index and head unchanged; `git diff-tree -r --name-status` printed
exactly the seven `A …/fanout-{doc}.md` lines. No file holds text before `proposals:` or an HTML entity; no file
fixed, so no twin. Every receipt's count equals its file's (17 · 5 · 0 · 0 · 25 · 5 · 13 = 65). The security-plan
agent's task notification said it stopped with background work of its own still running; the after-snapshot shows
no path but its own file.

Validate: 0 proposals rejected for a source the report does not carry (every coordinate is the report's or a span of
its New-text rows). Two classes escalate: the select model is an engine change for every document and a boundary
widening (remote markup's `selected` · `label` · `value` read, a new keyboard input class, a new `change` event),
answered by the founder himself (F1–F4 «Утвердить все четыре (Рекомендую)», inputs#I3) → the answer card
(`answer-card.md`); and the discharge of the standing PROVISIONAL marks the founder ruled (inputs#I3) — the
playbook's «Provisional discharge», escalate — raised by main, its timing (before the Epoch 4 close) put in the same
halt. The provenance the detectors wrote into their `change` lines («since 2026-10-20-select-by-id», «ratified by the
founder (2026-10-20 …)») is not pasted: the applied text is re-derived and the ratification goes to the sidecar
(classification writing rule 7).

architecture: 17 proposals · fanout-architecture.md
note: D-arch-decisions found no new library or runtime; architecture.md:354's `session_common` reader list reads «twenty-five readers» while :363 reads 27 (now 28).
1. escalate (answer card) — check 1 (Engine behaviour change, a widening by subject): [Scrolling and input] gains the select interaction model — keys, typeahead on the document's clock, click focus, `input` then `change` per change, the stated limits
2. apply — check 1 (Accurate this-chunk addition): [Scrolling and input]'s click-focus clause names a `select` (86@c9361,9394)
3. escalate (answer card) — check 1: [DOM semantics] gains the select's choice model — `CHECKED`, the selectedness settle, the mutator's re-seed, form entries, the UA select box
4. escalate (answer card) — check 1: the engine-change list (architecture.md:90) gains the select model, its merge surface by file owned by a CARRY on the Epoch 5 upstream-sync entry (P5)
5. apply — check 1, with citation row 159@c2284: Mutation, query and CSSOM — a `<label>` binds every labelable element (`is_labelable`); `label_bound_input_element`'s citation re-pointed to 677-715
6. apply — check 1, with citation row 159@c6035: Mutation, query and CSSOM — a select's option: `selected` from its chosen state, no `aria-selected`, kept under `display: none`, named by its `label` attribute
7. apply — with 6: [Scrolling and input]'s builder clause (86) excepts a select's option
8. apply — with 6: Dioxus DOM bridge's `selected` clause (205) — a select's option reads its chosen state
9. apply — check 1: Events — `DomEventKind::Change` / `DomEventData::Change`, dispatched after `input` on a select's choice alone
10. apply — check 1: Document core — `BaseDocument::select_value` (public, read-only) and the crate-private choice writers, the `select_typeahead` field
11. apply — check 1: Nodes and widgets — the crate-private `node::select` module
12. apply — check 1: Dioxus DOM bridge — `Input | Change` handed to the app, a select's snapshot `value`, an option's `bounds` its select's
13. apply — check 1 (the program's own fixed text inside a ratified carrier): Driver session — `type`'s help sentence for a select (192@c21636)
14. apply — check 1: Invalidation and state integrity — `apply_select_choice` marks the changed set
15. apply — check 1: Conventions → Tests — fourteen → fifteen `stand_act_*` (137)
16. apply — with 15: Existing Scopes — the blitz-tests row gains `stand_act_select` and `select_interaction`; «the fourteen `stand_act_*`» → fifteen (354)
17. apply — check 1: Existing Scopes — `session_common` readers 27 → 28, `fixtures` 21 → 23 (363)
18. (raised by main, Provisional discharge — inputs#I3, the upstream sync's take) escalate (halt Q2) — the `:focus-within` take's marks at 85, 103, 287 and 297 drop
19. (raised by main, Provisional discharge — inputs#I3, visual lint F1 and F2) escalate (halt Q2) — the lint's «PROVISIONAL» marks at 192, 198 and 349 drop; «in process only» stays as built, the `lint` verb owed to «Visual lint»'s gated part, named once at 198
20. (raised by main, the detector's note — a stale count on a line this pass edits) apply — Existing Scopes (354): «twenty-five readers» becomes a pointer to the reader count below it (363), stated once

security-plan: 5 proposals · fanout-security-plan.md
note: D-security-auth and D-security-deps do not fire; the `disabled` and `inert` rows are no-change.
1. escalate (answer card) — D-security-input, a boundary widening by subject: §Input Validation gains a `select` row — choice as `CHECKED`, seeding and settling, the closed keys and the 1 s typeahead under the `is_disabled` gate, the click's focus, `input` + `change`, the choice's exits
2. apply — check 1, with citation row 126@c5732: the `aria-label` · `<label for>` row — a `<label>` binds every labelable element (a closed element-name match, a hidden input excluded); citation re-pointed to 677-715
3. apply — check 1: the same row — `accessibility_label` 10 → 13 tests
4. apply — check 1: the keyboard-activation row — a click on a `select` focuses it (132)
5. apply — check 1 (the program's own fixed text inside a ratified carrier): §Error Handling — `type`'s help sentence (363@c3254)
6. (raised by main, check 5 — citation row 129@c4133 `claim false`) apply — §Input Validation, the `aria-*` state row: a select's option reads no `aria-selected`, its `selected` its chosen state
7. (raised by main, Provisional discharge — inputs#I3, the upstream sync's take) escalate (halt Q2) — the take's mark at 252 drops
8. (raised by main, Provisional discharge — inputs#I3, visual lint F1) escalate (halt Q2) — the lint's two «PROVISIONAL» marks at 125 drop, «in process only» stays as built

design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens holds; the plan's expected §Surface: desktop-native → Tokens amendment left to main.
1. (raised by main, check 5 — Expected amendment design-system §Surface: desktop-native → Tokens) apply — routine, the report's Schema / config: the user-agent `select` box takes `input, textarea`'s border, padding and background (engine defaults, no design token) and shows its chosen option

layout-templates: 0 proposals · fanout-layout-templates.md
note: D-layout-surface holds; §Surface: cli quotes no `type` help text, so the plan's expected amendment has no site (pattern `type-help`: 0 layout-templates rows).
1. (raised by main, Provisional discharge — inputs#I3, visual lint F1) escalate (halt Q2) — the lint's mark at 46 drops, «in process only» stays as built

test-plan: 25 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness hold; escher-driver's unit count unmoved.
1. apply — check 1: §1 tests/blitz-tests — `select_interaction` 7 and `stand_act_select` 5 entered
2. apply — check 1, with citation row 27@c13747: §1 — `accessibility_label` 10 → 13
3. apply — check 1: §1 blitz-dom covered behaviours — the single-choice select model and its proofs; its stated limits asserted by no check
4. apply — check 1: §1 — a click on a select's label (re-dispatched to the select) asserted by no check of this chunk
5. apply — check 1: §1 blitz-dom form — a named `select`'s entries, with no test of their own
6. apply — check 1: §1 blitz-dom (node) — `node/select.rs`'s 4 inline unit tests
7. apply — with 6: §4 blitz-dom (node) — the four tests described
8. apply — check 1: §1 dioxus-native-dom — 73 → 75 unit tests (the two snapshot tests)
9. apply — with 8, citation row 25@c4571: §1 — the snapshot builder's tests eighteen → twenty
10. apply — check 1: §2 Directory pattern — 141 → 143 `.rs` files, 140 → 142 targets
11. apply — check 1: §2 — `session_common` readers 27 → 28, `stand_act_*` fourteen → fifteen, six of them declaring both
12. apply — check 1: §2 — `fixtures` readers 21 → 23
13. apply — with 12: §3 fixtures helpers — 21 → 23 readers (104)
14. apply — with 12: §3 test-data bootstrap — 21 `stand_*` readers, two non-stand readers, `run stand` 190 · 0 · 7 (132)
15. apply — with 11: §3 session-check helpers — fifteen driver-action checks, 28 readers (103)
16. apply — with 11: §3 stand-check helpers — six of the fifteen (102)
17. apply — with 11: §1 escher-driver — fifteen `stand_act_*` files (22)
18. apply — with 11: §4 escher-driver — fifteen `stand_act_*` files (146)
19. apply — with 11: §5 Session ↔ held instance — `type` on a select, `stand_act_select`; fifteen (185)
20. apply — with 11: §3 → Session lifecycle (key file, session-proof) — fifteen driver-action files, `stand_act_select` 5, 50 → 55
21. apply — check 1: §3 Agent-run Proof — `run stand` 190 · 0 · 7 over 47 files (119), the newest reading only
22. apply — with 21: §9 Legs — stand-repeat over 47 files (324)
23. apply — with 21: §10 Flakiness budget — 47 files (353)
24. apply — check 1: §9 Per-OS tallies — the CI#39371468367 reading (334), the newest reading only
25. apply — check 1: §9 Local baseline — 204 · 1083 · 0 · 12 (340), the newest reading only
26. (raised by main, Provisional discharge — inputs#I3, the take and visual lint F1) escalate (halt Q2) — the two marks on line 27 (the take's checks «the behaviour they pin PROVISIONAL»; `stand_lint`'s «PROVISIONAL: in process only») drop

obs-plan: 5 proposals · fanout-obs-plan.md
note: D-obs-stack and D-obs-pii read no drift; no log site, span or field added.
1. apply — check 1: §3 Logging stack — the select checks join the silence census; the select model adds no log site, span, field or env read
2. apply — with 1: §3 — fifteen `stand_act_*`, `stand_act_select` the fifteenth; `select_interaction` in the census (70@c5360)
3. apply — with 1: §3 — the `session_common` reader list's fifteen `stand_act_*` (70@c6267)
4. apply — with 1: §3 — «twenty-seven checks read» → twenty-eight (70, a form the report's sweep missed)
5. apply — with 1: §3 — the fixtures module's 21 → 23 readers
6. (raised by main, Provisional discharge — inputs#I3, the upstream sync's take) escalate (halt Q2) — the take's mark at 77 drops
7. (raised by main, Provisional discharge — inputs#I3, visual lint F2) escalate (halt Q2) — «PROVISIONAL: no new field» at 120 drops

a11y-plan: 13 proposals · fanout-a11y-plan.md
note: D-a11y-obs-schema holds; the `:focus-within` mark is the discharge's, the lint's marks wait on the verb.
1. apply — check 1, with citation row 70: §3 — `accessibility_label` 10 → 13
2. apply — check 1, with citation row 269: §4 Hidden content — a select's option kept under `display: none` (112)
3. apply — with 2: §6 Visibility and hit-testing — the same exception (269)
4. apply — check 1, with citation row 46@c1576: §2 Role derivation — a select's option writes `selected` from its chosen state, its `label` attribute its name
5. apply — with 4, citation row 105: §4 ARIA state attributes — the same exception
6. apply — check 1: §2 changed set — a select's changed choice marks it
7. escalate (answer card) — check 1: §5 Keyboard handling — the select's keys and typeahead, `input` + `change`, the limits
8. apply — check 1: §5 Focus restoration — a click focuses a select (161)
9. apply — check 1, with citation row 284: §7 — a `<label>` names every labelable element; citation re-pointed to 677-715
10. apply — with 9: §5 — a label's click reaches its bound labelable element
11. escalate (answer card) — check 1: §7 — a select's options in the tree and the snapshot, its value read
12. apply — check 1: §5 Test harness pattern — `select_interaction` and `stand_act_select`
13. apply — check 1: §6 Color contrast — the UA select box recorded, its contrast not measured (no stand select)
14. (raised by main, Provisional discharge — inputs#I3, the upstream sync's take) escalate (halt Q2) — the take's mark at 155 drops
15. (raised by main, Provisional discharge — inputs#I3, visual lint F1) escalate (halt Q2) — the lint's marks at 80 and 234 drop, «in process only» stays as built

Check 5 (expected amendments): every plan entry is carried — architecture [Scrolling and input] · [DOM semantics] ·
Events · Dioxus bridge · Nodes and widgets (arch 1–12); security §Input Validation (sec 1–6); a11y §2 · §4 · §5 · §7
(a11y 2–11); design-system Tokens (design 1, raised); layout-templates cli — not carried, no site (the detector's
sweep and pattern `type-help`, 0 rows); obs §3 (obs 1); test-plan §1 · §3 · §4 · §9 · §10 (test 1–25); the engine
behaviour pattern (arch 4, the founder's word); the merge-surface CARRY and the residual line → P5. Every citation
row read `claim false` is settled by a numbered item above (arch 5, 6; sec 2, 6; test 2, 9; a11y 1, 3, 4, 5, 9).
Check 6: the two disproved claims (plan gate entry 12; research's «before») are DISPOSED — no master site.
