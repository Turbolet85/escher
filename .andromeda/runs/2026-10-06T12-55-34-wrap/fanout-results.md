# Fan-out results — 2026-10-06-accessibility-tree-identity

Report: `escher-0.1.0/chunks/2026-10-06-accessibility-tree-identity/report.md`. There were 7 doc-agents in one batch, with 15 detectors (arch 2 · security 3 · design 1 · layouts 1 · tests 3 · obs 3 · a11y 2 = the drift-base's 15). Keyed-contract renders: test-plan, obs-plan and a11y-plan `{doc}-contracts.md` (1 key each); architecture `NOT MIGRATED`. The returns needed no entity decoding (no `&lt;`/`&gt;`/`&amp;` in any return).

## Verdicts
- **architecture** — 4 proposals (D-arch-resources ×4, one `dependent-of`); D-arch-decisions no drift.
- **security-plan** — 4 proposals (D-security-input ×4, severity escalate); D-security-auth and D-security-deps no drift. Stripped: its trailing comments, which list out-of-scope re-points (security-plan:10, :265, :300, :306, :321, :331, :334, :352). The orchestrator's line-map pass covers them.
- **design-system** — `proposals: []`. Stripping removed commentary (out-of-scope flight_booker re-points) → raw twin `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripping removed commentary (the flight_booker.rs:68-108 → 68-110 re-point at layout-templates:10; the attribute note) → raw twin `.raw-fanout-layout-templates.md`.
- **test-plan** — `proposals: []`. Stripping removed commentary (the §1 stand-file list :25, §3 Proof :111, §9 Local baseline :314 counts; re-points) → raw twin `.raw-fanout-test-plan.md`.
- **obs-plan** — `proposals: []`. Stripping removed commentary (document.rs +10 and window.rs −1 re-points) → raw twin `.raw-fanout-obs-plan.md`.
- **a11y-plan** — `proposals: []`. Stripping removed commentary (a11y-plan:14 · :55 · :16 feature claims; §7 :266-268 name sources; re-points) → raw twin `.raw-fanout-a11y-plan.md`.

## architecture — parsed proposals and dispositions
1. D-arch-resources · warning · §Standard Contracts → Document core — register the defaulted, accessibility-gated `Document::accessibility_tree`. **apply** (check 1: playbook "Accurate this-chunk addition", since the symbol is in the report's Symbols bullet 1). The proposal's re-point clause is subsumed by the line-map pass.
2. D-arch-resources · warning · §Standard Contracts → Mutation, query and CSSOM — the two name sources and the shell's `update_tree(&dyn Document)`. **apply** (check 1: "Accurate this-chunk addition"; check 5: the expected amendment names this section and change).
3. D-arch-resources · warning · §Standard Contracts → Dioxus DOM bridge — the `author_id` override and its feature wiring. **apply** (check 1: same rule; check 5: expected amendment). The id's outbound crossing is held to escalation S1's resolution for wording.
4. D-arch-resources · warning · §Cross-cutting Patterns (arch:189) · dependent-of D-arch-resources — the feature-forwarding claim. **apply** (check 6: disposes report "Spec claims disproved" 2 for arch; its primary, 3, applies).

## security-plan — parsed proposals and dispositions
- S1. D-security-input · escalate · §Input Validation → `id` row — "written nowhere" retired: the id is carried as AccessKit `author_id` and leaves the process through the platform accessibility adapter when an AT is active. **escalate** (check 1: the detector's severity is escalate. The subject — author-derived content newly crossing the process boundary to the platform accessibility API — sits on the playbook's never-routine "Boundary widening" class. Whether an existing crossing gaining a new data class is a widening is the operator's call, so it needs the operator's own word). **Resolved:** the operator, 2026-10-06, at this wrap's P2 (AskUserQuestion), chose "Ratify + track". The crossing is ratified; the `id` row and the S2 row state the platform-API exposure; P5 pins a CARRY on "Stand a11y assertions" to assert that `author_id` never carries a `NodeId`, `ElementId` or pointer form on the platform tree. → **apply**.
- S2. D-security-input · escalate · §Input Validation → a new Markup attributes row for `aria-label` / `<label for>`. **apply** (check 1: the plan's P5-approved expected amendment names this change itself — "`aria-label` and `<label for>` are read into the tree from parsed content, including the browser app's remote HTML, as an in-process reader and not a boundary widening". Phase P4 read the playbook's Boundary-widening rule against both name-source options and recorded that it does not apply, because the parser already admits these attributes and no new input class enters. The names' outbound path to the AT is the same crossing S1 asks about, so the row's wording waits on S1).
- S3. D-security-input · escalate · `data-dioxus-id` row re-point (dioxus_document.rs:30-37 → 32-39). **apply** (mechanical line map; the detector's escalate severity attaches to the detector, not to a re-point; subsumed by the line-map pass).
- S4. D-security-input · escalate · `<style>` row re-point (document.rs:1119-1124 → 1129-1134). **apply** (same; subsumed by the line-map pass).

## Orchestrator-raised (check 5 · check 6)
- R1. a11y-plan §1 (:14, :55) — the Dioxus crates no longer only forward `accessibility` to blitz. dioxus-native-dom's feature enables its optional `accesskit` and gates its `accessibility_tree` override, and dioxus-native forwards to dioxus-native-dom. **raise · routine** (check 6, disproved claim 2).
- R2. a11y-plan §1 (:16) — blitz-tests builds dioxus-native-dom with `accessibility`. **raise · routine** (report Dependencies; the operator's widening).
- R3. a11y-plan §2 (:36) and §7 (:266-268) — the shell builds through `Document::accessibility_tree`; the name sources; the carried `author_id` on element nodes. **raise · routine** (check 5: expected amendment "a11y-plan §2 · §7").
- R4. test-plan §1 (:25), §3 Proof (:111), §9 Local baseline (:314) — the two new files, `run stand` 25, workspace 454 · 0 · 5. **raise · routine** (check 5).
- R5. layout-templates §Surface: desktop-native (:10) — four `for` and two `aria-label` attributes name the stand inputs, with no element, class, style or order change. **raise · routine** (check 5).
- R6. Line-map re-point over all seven masters + registries (`repoint.py`; dry-run listing `repoint-masters-dry.txt`, 159 citations: arch 66 · a11y 33 · test 22 · design 13 · security 11 · obs 10 · layouts 4; registries 0). **raise · routine** (check 5: the expected "re-points" entries for arch, design, layouts, obs and a11y; spot-checked: 18 target lines read in the new files).
- design-system and obs-plan expected entries — re-points only, carried by R6.
- Disproved claim 1 (the plan's feature premise) — disposed: fixed in-impl (scope record), and the arch registration is proposal 3. Disproved claim 3 (seven_guis ships without AccessKit) — routed to P5 as a working-route note on the operator's direction. Disproved claim 4 (gate 6) — disposed: chunk artifact amended on the operator's direction.

## Applied
All 8 detector proposals and R1-R6 were applied: the bodies were edited first, the line-map pass ran before any new text (so the new citations carry current lines), and the sweep and dispositions (`cascade-dispositions.md`) came before the sidecar entries. One entry per master, 7 sidecars, each checked `on-form` and read back as the file's last block. Escalations: 1 (S1), resolved on the operator's own word. Open: 0.

## Check 3 (intent-consistency)
Scope record: companion `packages/dioxus-native/Cargo.toml` (serves dioxus-native-dom's manifest, holds); widening `tests/blitz-tests/Cargo.toml` carrying the operator's word → the justified branch. No other divergence.
