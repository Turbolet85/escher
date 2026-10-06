# Cascade dispositions — 2026-10-06-id-stability-across-code-edits

Written from the listing `cascade.py sweep` printed (cascade v1.1, baseline `e4324247`, the trail `cascade-2026-10-06-id-stability-across-code-edits.json`), after every spec body of this pass was applied and before any sidecar entry.

## The search
22 patterns in `cascade-patterns.toml`, each with a control that fired on the pre-pass masters. They cover:
- the retired grammar and its mechanism wording — `component or document path`, `component path`, `document path`, `(1) the **author key**`, `appends to the parent's path`;
- the CRUD row's retired spelling — `div:0/div:1/div:0/div`, `div[{person.id}]`, `row's id`, ``a Dioxus key `{person.id}` ``;
- what the `accessibility` feature gates and who reads the id — `override and (its|the) snapshot`, `accessibility_tree` override, `second, in-process reader`, ``carries no `NodeId` ``;
- the moved counts — `18 unit tests`, `four files`, `six on the stable element id`, `element_id.rs and snapshot.rs`, `\b471\b`, `125 result lines`, ``33 `ok` ``;
- the lists a new file or module joins — `stand_snapshot|stand_{`, `mutation_writer, snapshot`.

Read over: the seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaf bodies. Not looked for by this sweep: line citations (they were re-pointed from the report's line maps and checked by a separate pass — 119 citations into the twelve edited or new files, 0 stale, 0 out of range) and any wording of the id grammar that uses none of the tokens above.

## Zero-row patterns (control fired, 0 rows after the pass)
`appends` · `row-path` · `row-dioxus-key` · `carries-no-id` · `four-files` · `six-on-id` · `test-census` — each retired wording is gone from the masters, the registries, the curation homes, the bases and the leaves. A statement about these seven patterns only.

## Masters — every row is this pass's own text or a true standing claim
- security-plan.md:107 `two-paths` (@c857) — amended: the phrase now sits in the uniqueness argument ("a component or document path has no empty segment"), true under four tiers.
- architecture.md:134 `component-path` ×2 (@c2500, @c4157) — amended: "(3) the **component path**" and its `{name}:{k}` clause; the tier stands, renumbered.
- security-plan.md:107 `component-path` (@c602) · `document-path` ×2 (@c627, @c870) — this pass's text.
- architecture.md:134 `document-path` (@c3674) — amended: "(4) the **document path**".
- test-plan.md:23 `document-path` — this pass's text (what the six older unit tests cover).
- architecture.md:134 `tier-one` (@c2101) — amended: the tier list now opens "one of four tiers".
- architecture.md:134 `row-id` (@c7330) — amended: "a row's id is its author key".
- architecture.md:189 `gates-snapshot` · `tree-override` — amended: "its `snapshot` and `actionable` modules".
- architecture.md:134 `tree-override` (@c9094) — no change: the snapshot clause's "builds from that `accessibility_tree` override", true.
- a11y-plan.md:55 `tree-override` — amended: the feature also gates the actionable-key check.
- security-plan.md:107 `second-reader` (@c1718) — no change: the snapshot is still the second in-process reader; the check is recorded after it as the third.
- architecture.md:84 `count-471` — no change: `net.rs:471-489`, a line citation.
- test-plan.md:314 `count-471` (@c2411) · `lines-125` (@c2393) — no change: the snapshot-model link of the baseline chain, history the chain keeps; this pass appended the 490 · 127 link after it.
- test-plan.md:111 `stand-33` · `stand-files` — no change: the snapshot-model link of the Proof chain; this pass appended the 41 link after it.
- `stand-files` at architecture.md:134 (@c11020), security-plan.md:107 (@c4187), test-plan.md:25 (@c2201), test-plan.md:314 (@c2543), a11y-plan.md:270, a11y-plan.md:314 — no change: `stand_snapshot.rs` citations and counts that still hold (a11y-plan.md:314's range was re-pointed by this pass).
- architecture.md:251 `module-list` — amended: the list opens with `actionable`.

Intra-line duplicates: the `×2` rows above were each read at both offsets; neither holds a second copy of a retired claim.

## Registries · curation homes · judgment bases
0 rows in each. Nothing routes to curation or to a playbook or drift-base proposal.

## Leaves — each row adds its leaf to the re-derivation, done in this pass
- CLAUDE.md:33 (`GENERATED:setup:modules`, the dioxus-native-dom line) — re-derived: four tiers and the actionable-key check.
- CLAUDE.md:99 (`GENERATED:setup:architecture`) — re-derived: the anchored path, the across-edits proof and the check.
- CLAUDE.md:76 (`GENERATED:setup:pointer-table`) — no change to the snapshot row; one row added above it for the id grammar and the check with their stand files.
- .claude/docs/services/dioxus-native-dom.md:15 · :16 · :38 — re-derived: the Publishes bullet states four tiers and the across-edits proof, a new bullet states the check, the entry points gain `actionable`, the test count reads 29.
- .claude/docs/services/seven_guis.md:36 — re-derived: the stand file list (eleven), the row's author key, Home's card keys, the actionable rule.
- .claude/docs/conventions.md:25 — re-derived: `snapshot` and `actionable` modules.
- .claude/docs/tests-summary.md:28 — re-derived: 490 · 0 · 5 over 127; the coverage line gains the two stand files' subjects.
- .claude/docs/commands.md:27 — re-derived: the package-alone stand command names the two new files.
- .claude/docs/services/dioxus-native-dom.md:16 `stand-files` — no change: the snapshot bullet's `stand_snapshot` pointer, true.

Leaves re-derived by provenance although no row named them: `.claude/rules/a11y.md` (the Accessibility tree section gains the actionable rule), `.claude/docs/a11y-summary.md` (the stand cue line), `.claude/docs/design-summary.md` (the author-id sentence gains the rows and the cards). Read and left: `.claude/docs/security-summary.md` (it restates no `id` row), `.claude/rules/security.md`, `.claude/rules/testing.md`, `.claude/rules/observability.md`, `.claude/rules/verification-harness.md`, `.claude/docs/obs-summary.md`, `.claude/docs/gotchas.md` (its two `anchor` hits are CSS anchor positioning).

## Lateral binds
test-plan §3 ↔ obs-plan §3: the harness, its commands and its log format are unchanged; test-plan §3 gained one measured count and obs-plan states no stand count. a11y schema ↔ obs schema: neither changed.
