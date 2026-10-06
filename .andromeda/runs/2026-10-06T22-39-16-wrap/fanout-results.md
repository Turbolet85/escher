# Fan-out results — 2026-10-06-snapshot-state-fidelity

Seven doc-agents, one batch, each sent the prompt of `amendment-flow.md` verbatim with its doc, the report and its
detectors (15 detector ids over the seven prompts = the drift-base's 15). Entity probe: the seven returns were read
whole — 0 `&lt;` / `&gt;` / `&amp;` (raw `<label for>`, `<object>`, `<col span>` arrive unescaped). Each return's
commentary lines (`#`-led) were stripped before the parse; their substance is in the verdict lines. A proposal's
`rationale`, `basis` and `sidecar` fields are not copied here — `change` is given (condensed where marked), and each
disposition names the check that decided it.

Total: 33 proposals — architecture 12 · security-plan 10 · test-plan 5 · a11y-plan 4 · obs-plan 2 · design-system 0 ·
layout-templates 0. All `warning`. 0 escalations.

**A note that applies to every re-point below.** Several returns measured new line numbers on the working tree
(source locations the report does not carry — the re-derivation tell). The FACT each rests on is the report's: the
two files' lines moved (Counts / qualifiers moved). So the proposals stand as sites, and every line target applied is
the orchestrator's own measurement (`grep -n` over `mutation_writer.rs` and `snapshot.rs`, this run), never the
proposal's number taken on trust. One fact the report did not carry entered this way: the old
`dangerous_inner_html` citation `:408-409` was already one line early at the base (security-plan #4).

## design-system — `proposals: []`
Verdict: no drift. D-design-tokens: the four Coverage rows read `tokens n/a`; no UI rendered; no citation of the two
shifted files. (Stripped: a commented basis block; no raw twin — stripping removed commentary only, the YAML body was
`proposals: []` as returned.)

## layout-templates — `proposals: []`
Verdict: no drift. D-layout-surface: no new surface or region (the fixtures are components inside a test file, no
`examples/seven_guis/` file changed, `LeanTask` not grown); 0 hits for `mutation_writer`, `snapshot.rs`, `falsy`,
`hidden`, `password`, `MASKED`, `stand_snapshot`, `fx-` in the doc. (Stripped: a commented basis block.)

## obs-plan — 2 proposals
Verdict: D-obs-instrumentation, D-obs-stack, D-obs-pii all hold (no log site added, `trace!` 13, no dependency); the
drift is stale line citations only.
1. D-obs-pii · warning · §8 PII Scrubbing & Compliance → Values logged as-is — re-point `mutation_writer.rs` `:150` →
   `:183`, `:202` → `:235`, `:388` → `:421`; sentence unchanged.
   **apply** — check 1 (Accurate this-chunk addition: a citation reconciled to the shipped file); check 5 (the plan's
   obs-plan entry). Targets measured: `:183` create_text_node, `:235` set_node_text, `:421` set_attribute.
2. D-obs-instrumentation · warning · §6 Log Coverage → Logged events — re-point `:119-205` → `:152-238`, `:305` →
   `:338`, `:388` → `:421`. dependent-of #1.
   **apply** — check 1 (same rule), with its primary. Targets measured: first `trace!` `:152`, last of the range `:235`
   (the range's close `:238`), `:338` load_template.

## test-plan — 5 proposals
Verdict: D-tests-framework and D-tests-obs-harness hold; D-tests-coverage's invariant holds (every new surface has a
test) — the census it reads against moved at five sites.
1. D-tests-coverage · warning · §1 Coverage scope → dioxus-native-dom — "33 unit tests in five files" (was 29),
   thirteen on the snapshot builder (was nine) naming the four new behaviours; citation `snapshot.rs:195-347` →
   `:210-527`.
   **apply** — check 1 (Accurate this-chunk addition), check 5 (test-plan §1). Count basis: the report (33; 13).
2. D-tests-coverage · warning · §1 Coverage scope → tests/blitz-tests — add the two new files to the coverage sentence
   and its citations (condensed). dependent-of #1.
   **apply** — check 1, check 5.
3. D-tests-coverage · warning · §3 Crate-local test helpers → dioxus-native-dom — re-point `snapshot.rs:201-206` →
   `:216-221`; record that the reader pin writes its attributes through `DocumentMutator` because the helper's
   `DocumentConfig::default()` has the no-op HTML parser. dependent-of #1.
   **apply** — check 1; check 6 (it disposes the report's first disproved claim, the plan's step-4 fixture).
4. D-tests-coverage · warning · §3 Agent-run contract → Proof — append a dated link: `run stand` 48 `ok` stand events
   (+7 `stand_snapshot_state`). dependent-of #1.
   **apply** — check 1, check 5 (test-plan §3).
5. D-tests-coverage · warning · §9 Local baseline → `cargo test --workspace` — append a dated link: 129 result lines,
   504 passed · 0 failed · 5 ignored. dependent-of #1.
   **apply** — check 1, check 5 (test-plan §9).

## a11y-plan — 4 proposals
Verdict: D-a11y-obs-schema holds; D-a11y-surface read literally holds (the only new interactive elements are the
in-test fixture controls, covered). The four are the §3 / §5 / §7 coverage records the report retires or extends.
1. D-a11y-surface · warning · §5 Script and framework exposure (the falsy-attribute bullet) — the falsy clear covers
   the bridge's 27 boolean attributes on both attribute paths; a `hidden: false` element stays displayed, in the tree
   and in the snapshot; drop "every other Dioxus boolean attribute … is still written as the literal `"false"`" and
   "recorded, not established" (condensed).
   **apply** — check 1 (Accurate this-chunk addition), check 5, check 6 (disposes two disproved claims). The proposal
   writes PROVISIONAL into the body; applied WITHOUT it — see "Provisional status" below.
2. D-a11y-surface · warning · §5 Test harness pattern (the first "Observed absent" bullet) — replace with the present
   fact: a Tab / Shift+Tab key-press focus check exists on the stand (condensed).
   **apply** — check 1, check 5.
3. D-a11y-surface · warning · §3 Keyboard test harness — the same retired claim, second occurrence. dependent-of #2.
   **apply** — with its primary.
4. D-a11y-surface · warning · §7 Accessibility tree output (the snapshot-model bullet) — state where `enabled`,
   `checked` and `value` are read from, the password mask and the three focus proofs; re-point `snapshot.rs:72-90` →
   `:79-97`, `:155-173` → `:162-180` (condensed).
   **apply** — check 1, check 5.

## security-plan — 10 proposals
Verdict: D-security-input's escalate condition is NOT met (no new external-input surface; Coverage reads validation
n/a); D-security-auth and D-security-deps hold. All `warning`.
1. D-security-input · warning · §Input Validation → a new row for the snapshot's password mask (condensed).
   **apply** — check 1 (Accurate this-chunk addition; it NARROWS what the snapshot carries — not the Boundary-widening
   class), check 5.
2. D-security-input · warning · §Input Validation → `disabled` row — append the reader the snapshot follows.
   **apply** — check 1, check 5.
3. D-security-input · warning · §Input Validation → a new Dioxus-mutations row for the 27-name falsy clear (condensed).
   **apply** — check 1 (an in-process mutation path, no new input class, crossing or write — not Boundary widening),
   check 5. Applied without PROVISIONAL in the body — below.
4. D-security-input · warning · `dangerous_inner_html` row — re-point `:408-409` → `:441-442`.
   **apply** — check 1. Target measured: `:441` the branch, `:442` `set_inner_html`.
5. · Attribute value types row — `:284-286` → `:317-319`. dependent-of. **apply** (measured `:317-319`).
6. · §Error Handling → Graceful degradation — `:41-49` → `:74-82`. dependent-of. **apply** (measured `:75`, `:80`).
7. · §Error Handling → Panic paths — `:41-49` → `:74-82`. dependent-of. **apply**.
8. · `id` row — `snapshot.rs:72-90` → `:79-97`. dependent-of. **apply** (measured `impl DioxusDocument` `:79-97`).
9. · `id` row — `snapshot.rs:105-151` → `:112-158`. dependent-of. **apply** (measured `fn visit` from `:112`).
10. · accessible-names row — `snapshot.rs:155-173` → `:162-180`. dependent-of. **apply** (measured `fn name` `:162-180`).

## architecture — 12 proposals
Verdict: D-arch-decisions holds (no dependency, runtime, port, env var or serialization). D-arch-resources: one new
public item unregistered (`MASKED_VALUE`), one changed bridge contract, and moved citations.
1. D-arch-resources · warning · §Standard Contracts → Dioxus DOM bridge — the falsy-clear clause names the 27-name
   list, its source, both attribute paths and what is still written (condensed).
   **apply** — check 1 (Accurate this-chunk addition), check 5. Not a reversal of a locked decision: the prior clause
   described the two-name state and the route's CARRY named the rest as the defect. Body without PROVISIONAL — below.
2. · same section — delete "every other boolean attribute (…) is still written with the literal value `"false"`".
   dependent-of #1. **apply** — check 6.
3. · §Existing Scopes → blitz-tests row — name `dioxus_falsy_boolean_attrs.rs` beside `dioxus_falsy_disabled.rs`.
   dependent-of #1. **apply** — check 1 (the report's own site list names this row).
4. D-arch-resources · warning · Dioxus DOM bridge, the snapshot field readers — `value` reads `MASKED_VALUE` for a
   password input (condensed). **apply** — check 1, check 5.
5. · the snapshot type sentence — the re-export set is the three types plus `MASKED_VALUE`. dependent-of. **apply**.
6. · §Existing Scopes → dioxus-native-dom row — "its three types and the `MASKED_VALUE` constant re-exported".
   dependent-of. **apply**.
7. · Dioxus DOM bridge — re-point `snapshot.rs:14-53; :72-90; :102-193`. dependent-of. **apply** — targets measured
   `:11-13` (the constant), `:18-60` (the three types — the proposal read `:17-60`), `:79-97`, `:109-208`.
8. · the snapshot field readers — `enabled` as the presence reading with its pinned disagreement. dependent-of.
   **apply** — check 5 (the plan's fourth architecture fact).
9. · Dioxus DOM bridge — `mutation_writer.rs:316-323` → `:349-356`; `:226-262` → `:259-295`. dependent-of. **apply**.
10. · §Occupied Resources → Process-wide state — `:22-23; :36` → `:55-56; :69`. dependent-of. **apply**.
11. · §Conventions → Known gaps — `:125` → `:158`, `:285` → `:318`, `:291` → `:324`, `:181-185` → `:214-218`.
    dependent-of. **apply**.
12. · §Cross-cutting Patterns → Event processing — `:327-337` → `:360-370`; `:309-314` → `:342-347`. dependent-of.
    **apply**.

## Validate — the six checks
1. **Playbook.** Every applied proposal matches "Accurate this-chunk addition" (routine): its named symbols and
   values are in the report's Changes and the invariant survives. None matches "Boundary widening": the mask narrows
   what the snapshot carries, and the falsy clear removes attributes on an existing in-process mutation path — no
   read-only channel gains a write, no validated surface admits a new input class, no boundary gains a crossing (the
   plan's own playbook read, plan.md Provenance, re-read here against the report). No two rules collide.
2. **Cross-contradiction.** None: architecture #1/#2, a11y-plan #1 and security-plan #3 state one change the same way.
3. **Intent-consistency.** The report's six deviations are justified against the route entry and the plan's acceptance
   (the report's Outcome re-asserts every criterion met). The plan's acceptance names a test that does not exist
   (`disabled_in_parsed_markup_…`); the clause it proves holds by the other test it names. Scope record: none (0 lines).
4. **Absence needs evidence.** Line profiles read before any hit was dispositioned (`architecture.md:134` is 13 815
   chars, `security-plan.md:107` 4 369, `test-plan.md:25` 3 623): every site above was read through a bounded window
   at its offset, never from a grep view. The detectors' 0-hit claims re-run here: `falsy` 0 and `NodeState` 0 in
   security-plan; `MASKED_VALUE` 0 in every master and registry file.
5. **Expected amendments.** All five plan entries are covered: architecture (4 facts → #1/#2, #4, #5/#6, #8);
   security-plan (3 → #1, #2, #3); a11y-plan (3 → #1, #2/#3, #4); test-plan §1 · §3 · §9 (→ #1–#5); obs-plan §8
   (→ #1, with §6). None under-run; none raised by the orchestrator alone.
6. **Disproved claims.** (a) the plan's step-4 fixture → disposed in the implementation, recorded by test-plan #3;
   (b) a11y-plan's "recorded, not established" → a11y-plan #1; (c) the route's password-hypothesis on the next entry
   → P5 route-resolve (a markerless-tail rewrite to the measured state); (d) "every other boolean attribute is still
   written `"false"`" → architecture #2 and a11y-plan #1; (e) the two stale clauses of `.claude/rules/a11y.md` → the
   cascade re-derives that leaf from a11y-plan.

## Provisional status
The bridge's 27-name falsy clear rests on a provisional direction (answered at phase P4's question round, recorded
PROVISIONAL at the P5 review, 2026-10-06; pending the founder's own word). Three detectors proposed writing that
status into the body. It is recorded in the sidecar entries instead, as its precedent was: the falsy-`disabled`
clearing's bodies never carried a provisional clause (architecture-amendments, 2026-10-06-headless-stand; ratified
with "no body text changes" at 2026-10-06-founder-rulings), and the founder's standing rule discharges PROVISIONAL
items in one batch at each epoch boundary. The work proceeds on the provisional direction; it is not an escalation.
