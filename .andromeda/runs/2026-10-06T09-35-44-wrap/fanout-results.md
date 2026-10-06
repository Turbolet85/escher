# Fan-out results — 2026-10-06-stable-element-ids

Report: escher-0.1.0/chunks/2026-10-06-stable-element-ids/report.md · 7 doc-agents, one parallel batch. Each return
was stripped of its trailing `#` commentary lines (the agents' per-detector reasoning, summarised in the verdict
line); no HTML entity appeared in any return (probe: `&lt;` `&gt;` `&amp;` absent).

## Verdicts
- architecture — 2 proposals (D-arch-resources ×2)
- security-plan — 1 proposal (D-security-input); stripped: D-security-auth / D-security-deps no drift (no identity, secret or dependency change)
- design-system — `proposals: []`; stripped: both surfaces carry tokens n/a; the 17 citations noted as the cascade's
- layout-templates — `proposals: []`; stripped: no new surface or region; the expected `id` amendment on §Primary screens is outside D-layout-surface; its 3 citations sit at or below `dioxus_document.rs:182`
- test-plan — `proposals: []`; stripped: tests at both tiers present, runner standard, harness unchanged; count re-counts and citation re-points named as outside its detectors
- obs-plan — `proposals: []`; stripped: no hot path, no logger, no log field; obs cites none of the shifted files
- a11y-plan — `proposals: []`; stripped: no new interactive element, no schema change; 8 citations named as the cascade's

## Proposals and dispositions

### architecture-1 · D-arch-resources · warning · §Standard Contracts → Dioxus DOM bridge
- change: register `DioxusDocument::element_id` / `element_ids` and the id grammar as built (author key → component path with framework-scope exclusion, last-segment component names, `{name}:{k}`, `{tag}[{key}]` / `{tag}:{n}`, append vs restart → document path); re-point `dioxus_document.rs:342-348` → `367-373`
- basis: architecture.md:134; dioxus_document.rs:198, :204 (inside the report's carried +25 insert after old 182)
- **disposition: APPLY** — check 1: playbook "Accurate this-chunk addition" (the named symbols are in the report's Symbols / APIs); also the plan's Expected amendment names this change itself. Check 3: the grammar departs from the plan's text, justified by Spec claims disproved 1–3 (dioxus-core reality) and the matrix acceptance as written holds → intent incomplete, the body records the grammar as built. Applied text re-derived from the report, adding the measured detach fact (report claim 4) to the same bullet.

### architecture-2 · D-arch-resources · warning · §Existing Scopes → dioxus-native-dom
- change: module list gains crate-private `element_id`; `lib.rs:12-15` → `12-16`
- **disposition: APPLY** — check 1: "Registry over-reach" weighed and NOT matched: this row ENUMERATES the crate's modules, so the body's list is now incomplete; "Accurate this-chunk addition" governs (report Crates / modules + Files).

### security-plan-1 · D-security-input · escalate · §Input Validation (Markup attributes)
- change: add a `Markup attributes | id` row: the author-key rule (non-empty, `/`-free, first in pre-order), the fallback to a path, `None` for non-element / stale / detached, never panics, computed on demand, no engine id in the string
- **disposition: APPLY (routine)** — check 1: "Accurate this-chunk addition"; the plan's Expected amendment names this change itself ("the HTML `id` gains a reader (the author-key rule …)"). "Boundary widening" weighed by its SUBJECT and not matched: `id` is an already-admitted markup attribute with existing readers (CSS matching, `getElementById`); no input class is newly admitted, no channel gains a write, no new crossing — the new reader is an in-process computed read. The detector's `escalate` severity is its default, and its own finding is "the boundary is validated".

## Orchestrator-raised (check 5 — Expected amendments with no detector proposal)
- test-plan §9 Local baseline (line 314) — append the re-count 441 · 0 · 4, 121 result lines → **APPLY routine** (report Counts / qualifiers moved)
- test-plan §3 Agent-run contract Proof (line 111) — append-only re-count `run stand` 17 → **APPLY routine** (report Counts)
- test-plan §1 tests/blitz-tests coverage (line 25) — the stand list gains stand ids (`stand_element_ids.rs:1`) → **APPLY routine** (report Files; plan entry "stand_element_ids.rs, 17 stand checks")
- test-plan §3 Crate-local helpers → dioxus-native-dom (line 96) — the `element_id` unit tests' shape → **APPLY routine** (report Files)
- layout-templates §Surface: desktop-native §Primary screens (line 10) — the lean tasks' controls carry author ids → **APPLY routine** (report Files + measured stand ids; plan entry names the change)
- file:line citations into the six shifted files, every master → **APPLY routine** (report line maps; per-master counts architecture 23 · design-system 17 · test-plan 10 · a11y-plan 8 · layout-templates 3 · security-plan 3)

## Check 2 — cross-contradiction
none — no two proposals edit one section in opposing directions.

## Check 4 — absence needs evidence
the citation sweep is a scripted pass over every `{path}:{line}` citation of the six files in the seven masters and the registries (`cascade-citations.md`), every hit listed with its disposition.

## Check 6 — disproved claims
1–3 (base scope, component name, grammar uniqueness) → DISPOSED by architecture-1 (the grammar as built); the plan / scope text is chunk-local and immutable.
4 (re-render removal only detaches) → DISPOSED by architecture-1 (the detach fact lands in the bridge bullet) + curation (hazard).
5 (index-keyed CRUD delete removes the last row's node) → DISPOSED to the report (a test-design fact no master states; `grep -n 'Delete' .andromeda/*.md` names no row-node identity) + curation.

## Found by the cascade sweep
- architecture §Existing Scopes → tests/blitz-tests row (line 258) restates the stand-check list → **APPLIED routine** (a same-master duplicate of the test-plan §1 claim; cascade-dispositions.md)

Applied: 11 amendments over 6 masters (architecture 3 · security-plan 1 · test-plan 4 · layout-templates 1 · citation re-points in a11y-plan and design-system 1 each), 36 citations re-pointed. Escalations: 0.
