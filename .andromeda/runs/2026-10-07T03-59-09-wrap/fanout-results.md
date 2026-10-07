# Fan-out results — 2026-10-07-audit-corrections

Seven doc-agents, one parallel batch, each given its document, the chunk report and its detectors (15 detector
ids over 15 `doc:` names: architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 ·
obs-plan 3 · a11y-plan 2). Keyed-contract renders went to test-plan, obs-plan and a11y-plan; architecture read
`NOT MIGRATED` and security-plan, design-system and layout-templates carry no keyed-contract section, so their
line was dropped. Every return was stripped of commentary and probed for entities: 0 in all seven.

## Verdicts
- architecture — 12 proposals. Stripped: four comment lines saying neither invariant is broken by an unregistered resource, a new library or a contradicted decision, and that what follows is registry accuracy the report's Changes carry.
- security-plan — 7 proposals. Stripped: seven comment lines; the agent rated all seven `warning` because no external-input surface was added.
- design-system — `proposals: []`. Stripped: two comment lines (no new UI; its one citation into an edited file is unmoved).
- layout-templates — `proposals: []`. Stripped: five comment lines (no surface added; its three citations into `dioxus_document.rs` are unmoved).
- test-plan — 12 proposals. Stripped: five comment lines (the three invariants hold; the coverage record and the layout convention drifted; sites swept and found holding).
- obs-plan — 1 proposal. Stripped: six comment lines (the three invariants hold; the one proposal is a scope completion).
- a11y-plan — `proposals: []`. Stripped: comment lines relaying the report's five moved citations and the tables' new home, outside both detectors.

## Dispositions
Numbered in each return's own order (the parsed lists are under Parsed returns).

### architecture
| # | Section | Disposition | Decided by |
|---|---|---|---|
| 1 | §Standard Contracts → Dioxus DOM bridge — `element_id.rs:78-133` → `:78-161` | apply | check 1: routine (Accurate this-chunk addition); the report's line map, read by hand |
| 2–5 | same line — `:159-255` → `:186-282`, `:181-187` → `:208-214`, `:198-243` → `:225-270`, `:370-405` → `:397-432` | apply | check 1: routine; dependents of 1, applied with it |
| 6 | same line — `stand_element_ids.rs:129-240` → `:129-255` | apply | check 1: routine; read by hand |
| 7 | same line — `stand_actionable_keys.rs:86-106` → `:85-105` | apply | check 1: routine |
| 8 | §Occupied Resources → Process-wide state and threads — `stand_id_persistence.rs:273-283` → `:289-299` | apply | check 1: routine |
| 9 | §Existing Scopes → dioxus-native-dom — the test-only child module `bridge_tests` | apply | check 1: routine (Accurate this-chunk addition); the plan's expected-amendments entry names the row and the file. Registry over-reach was read and does not govern: this row enumerates the crate's modules one by one |
| 10 | §Existing Scopes → blitz-tests — the shared module | apply | check 1: routine; the plan's entry names it |
| 11 | §Conventions → Tests — the shared module beside per-file repetition | apply | check 1: routine; the plan's entry names it |
| 12 | §Conventions → Tests — the one unit-test module in its own file | apply | check 1: routine (reconciles the convention's wording to what shipped); not on the plan's list |

### security-plan
| # | Section | Disposition | Decided by |
|---|---|---|---|
| 1 | §Input Validation, the `id` row — state the `{tag}[{key}]` filter, cite its two witnesses | reject as proposed; its fact raised by the orchestrator and applied | the re-derivation tell: its `basis` cites a source location the report does not carry. Raised under check 5 as routine — the report's Coverage rows and Expected amendments carry the fact. Applied from the report alone: the two witnessed conditions and the witness citation. Not applied: "first among its siblings" (architecture's grammar, not witnessed by this chunk) and "no Dioxus key can put a `/` or an empty segment into an id" (a claim the report does not make) |
| 2 | same row — `element_id.rs:159-255` → `:186-282` | apply | check 1: routine; the primary of the re-point group |
| 3–6 | same row — `:310-334` → `:337-361`, `:370-423` → `:397-450`, `:425-576` → `:452-603`, `stand_element_ids.rs:242-275` → `:257-290` | apply | check 1: routine; dependents of 2 (they name the detector both 1 and 2 carry; each re-points a citation, as 2 does, so 2 is their primary) |
| 7 | the masked-value row — `stand_snapshot_state.rs:490-539` → `:445-494` | apply | check 1: routine; dependent of 2 |

### test-plan
| # | Section | Disposition | Decided by |
|---|---|---|---|
| 1 | §1 Coverage scope → dioxus-native-dom — 49 in seven files → 55 in eight, the six bridge tests | apply | check 1: routine; the plan's entry |
| 2 | same bullet — `element_id.rs:257-577` → `:284-604` | apply | check 1: routine |
| 3 | §1 → dioxus-native and stylo_taffy — the list of test-holding files gains the new file | apply | check 1: routine; dependent of 1 — the same census restated without its tokens, a site the report's search did not name |
| 4 | §9 Local baseline — re-counted 548 · 0 · 5 over 131 | apply | check 1: routine; the plan's entry |
| 5 | §3 Crate-local test helpers → dioxus-native-dom — `:264-268` → `:291-295`, `:370-405` → `:397-432` | apply | check 1: routine |
| 6 | same bullet — `stand_snapshot_state.rs:120-127` → `:75-82` | apply | check 1: routine |
| 7 | §4 Unit Test Strategy → Test file location — the one out-of-line unit-test module | apply | check 1: routine; agrees with architecture 12 (check 2: no contradiction) |
| 8 | §2 Directory pattern — the same exception | apply | check 1: routine; dependent of 7 |
| 9 | §2 Directory pattern — the shared module beside one file per behavior | apply | check 1: routine; the report carries the site |
| 10 | §1 Coverage scope → tests/blitz-tests — the same | apply | check 1: routine; dependent of 9 |
| 11 | §3 Crate-local test helpers → dioxus-native-dom — how the bridge tests reach the private handler | apply | check 1: routine; the plan's entry |
| 12 | §3 Crate-local test helpers — a new blitz-tests bullet, the shared module's inventory | apply | check 1: routine (Accurate this-chunk addition, inside the section that inventories test helpers); 9 and 10 point here rather than restating the inventory |

### obs-plan
| # | Section | Disposition | Decided by |
|---|---|---|---|
| 1 | §3 Logging stack, the headless-stand bullet — the census also covers the checks' shared module | apply | check 1: routine; the plan's entry. The entry's other file, `dioxus_document_tests.rs`, has no owning sentence in obs-plan (`dioxus-native-dom` → obs-plan.md:19, :43, :213, :214, :287: the crate's features and the `trace!` sites in `mutation_writer.rs`, none a census a unit-test file belongs to); its census of 0 stays in the report |

### a11y-plan — raised by the orchestrator (check 5: the plan's expected amendments no detector proposed)
| # | Section | Disposition | Decided by |
|---|---|---|---|
| o1 | §3 Keyboard test harness and §5 Test harness pattern — `stand_snapshot_state.rs:308-344` → `:263-299` (two sites) | apply | check 5: routine; the report's line map |
| o2 | §8 — `stand_accessibility_ids.rs:162-177` → `:86-101`, `:303-320` → `:227-244`; `stand_snapshot.rs:337-390` → `:210-263` | apply | check 5: routine; the report's line map |
| o3 | §7 Accessibility tree output — the tables the 15 controls' roles and names are checked against are stated once, in the shared module | apply | check 5: routine; the report's Symbols / APIs |

### design-system · layout-templates
No proposal and none expected: the plan lists no amendment for either, and their four citations into an edited file keep their numbers.

## Checks 2 to 6
- Check 2 (cross-contradiction): none. architecture 11–12 and test-plan 7–10 say the same thing in two masters.
- Check 3 (intent-consistency): the report matches the entry and the plan's acceptance criteria; its deviations are justified (the plan's revision on the operator's word, `inputs#I1`; the operator pass on the operator's direction). Scope record: none.
- Check 4 (absence needs evidence): every 0-hit claim in the report names its search; the detectors' sweep claims are re-run by the cascade's step-2 sweep before any sidecar entry lands.
- Check 5 (expected amendments): every entry of the plan's list is covered — architecture (9, 10, 11 and the eight re-points), test-plan (1, 4, 11 and the four re-points), security-plan (1 raised, 2–7), a11y-plan (o1–o3), obs-plan (1).
- Check 6 (disproved claims): the report's three entries are plan- and research-level and each is disposed in the report (the plan carries its revision record; its Implementation notes record the detector reading; research was corrected at the revision). No master claim was measured false.

Escalations: 0.

## Parsed returns
Each doc-agent return as it arrived (extracted from the agent transcript by script, not retyped).

### architecture
```yaml
# Strict reading: no new IPC method / endpoint / event / socket / port / env var / workspace crate (report Changes :24-25) and no dependency added or bumped (report :26) -- neither invariant is broken by an unregistered resource, new library or contradicted locked decision.
# What follows is registry ACCURACY drift the report's Changes carry (line map :35-46, new modules :20-21, :25) and its Expected amendments name for architecture (:62-64).
# Sweep: each old line range below occurs exactly once in architecture.md (grep count 1 each); the 26 other citations into the ten edited files are unmoved (report :46) and need nothing.
# Every new range was read against the work tree.
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the four-tier id clause's citation group, re-point `packages/dioxus-native-dom/src/element_id.rs:78-133` to `packages/dioxus-native-dom/src/element_id.rs:78-161` (the vdom walk: `VdomWalk::walk` and the three private helpers split from it, `record_roots`, `walk_component`, `child_chain`); no prose changes, the id grammar is unchanged.
    sidecar: >-
      2026-10-07-audit-corrections: Dioxus DOM bridge — the walk citation element_id.rs:78-133 → :78-161 (`VdomWalk::walk` split into three private helpers, output unchanged).
    rationale: >-
      Report Symbols / APIs (:18) lands three new private symbols as the body of `walk` at element_id.rs:97-161; the registry's walk citation ends at :133 and so no longer covers the walk it registers. Report line map (:37) lists this as read by hand: architecture.md:134 `:78-133` → `:78-161`.
    basis: packages/dioxus-native-dom/src/element_id.rs:78-161 (walk :78, record_roots :97, walk_component :119, child_chain :145, template_roots :163); architecture.md:134
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the `element_id` / `element_ids` signature clause ("compute the id on demand from a vdom walk joined with a DOM walk"), re-point `packages/dioxus-native-dom/src/element_id.rs:159-255` to `packages/dioxus-native-dom/src/element_id.rs:186-282`.
    sidecar: >-
      2026-10-07-audit-corrections: Dioxus DOM bridge — citation element_id.rs:159-255 → :186-282 (+27, text unchanged).
    rationale: >-
      Report line map (:37): every old line ≥ 134 of element_id.rs moves +27; architecture.md:134 `:159-255` → `:186-282`. `element_ids` now sits at :186 and `element_id` ends at :282 (report :19).
    basis: packages/dioxus-native-dom/src/element_id.rs:186-282; architecture.md:134
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the four-tier id clause's citation group, re-point `packages/dioxus-native-dom/src/element_id.rs:181-187` to `packages/dioxus-native-dom/src/element_id.rs:208-214`.
    sidecar: >-
      2026-10-07-audit-corrections: Dioxus DOM bridge — citation element_id.rs:181-187 → :208-214 (+27, text unchanged).
    rationale: >-
      Report line map (:37): architecture.md:134 `:181-187` → `:208-214`. The seven lines at the base's :181-187 (the keyed-element anchor match) read identically at :208-214 in the work tree.
    basis: packages/dioxus-native-dom/src/element_id.rs:208-214; architecture.md:134
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the four-tier id clause's citation group, re-point `packages/dioxus-native-dom/src/element_id.rs:198-243` to `packages/dioxus-native-dom/src/element_id.rs:225-270`.
    sidecar: >-
      2026-10-07-audit-corrections: Dioxus DOM bridge — citation element_id.rs:198-243 → :225-270 (+27, text unchanged).
    rationale: >-
      Report line map (:37): architecture.md:134 `:198-243` → `:225-270`; `place_children` now sits at :225 (report :19).
    basis: packages/dioxus-native-dom/src/element_id.rs:225-270; architecture.md:134
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the detached-node clause ("a node a re-render removes is DETACHED … and reads no id"), re-point `packages/dioxus-native-dom/src/element_id.rs:370-405` to `packages/dioxus-native-dom/src/element_id.rs:397-432`.
    sidecar: >-
      2026-10-07-audit-corrections: Dioxus DOM bridge — citation element_id.rs:370-405 → :397-432 (+27, in-file `mod tests` byte-identical).
    rationale: >-
      Report line map (:37): architecture.md:134 `:370-405` → `:397-432`; the in-file `mod tests` moved from :257-577 to :284-604 unchanged (report :11). `a_removed_element_reads_no_id` now spans :397-432.
    basis: packages/dioxus-native-dom/src/element_id.rs:397-432; architecture.md:134
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the stand clause ("on the stand the chrome reads its author keys …"), re-point `tests/blitz-tests/tests/stand_element_ids.rs:129-240` to `tests/blitz-tests/tests/stand_element_ids.rs:129-255` (the same three tests plus the three file-local functions split from the third).
    sidecar: >-
      2026-10-07-audit-corrections: Dioxus DOM bridge — citation stand_element_ids.rs:129-240 → :129-255 (`unkeyed_elements_read_their_component_path` split into three file-local functions, assertions kept).
    rationale: >-
      Report line map (:41), read by hand: architecture.md:134 `:129-240` → `:129-255`. Report :23: the third test now sits at :244 and calls functions at :176, :197, :224; it ends at :255.
    basis: tests/blitz-tests/tests/stand_element_ids.rs:129-255; architecture.md:134
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the same stand clause, re-point `tests/blitz-tests/tests/stand_actionable_keys.rs:86-106` to `tests/blitz-tests/tests/stand_actionable_keys.rs:85-105`; leave the later `stand_actionable_keys.rs:1-4` citation as it is.
    sidecar: >-
      2026-10-07-audit-corrections: Dioxus DOM bridge — citation stand_actionable_keys.rs:86-106 → :85-105 (the file reads the shared `boot`, 169 → 168 lines).
    rationale: >-
      Report line map (:39): stand_actionable_keys.rs, 5 citations, 1 moved — architecture.md:134 `:86-106` → `:85-105`. `every_actionable_stand_element_reads_an_author_key` now spans :85-105.
    basis: tests/blitz-tests/tests/stand_actionable_keys.rs:85-105; architecture.md:134
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads"
    change: >-
      In the clause on the two blitz-tests integration binaries that re-execute their own test binary, re-point `tests/blitz-tests/tests/stand_id_persistence.rs:273-283` to `tests/blitz-tests/tests/stand_id_persistence.rs:289-299`; the prose (fixed argv, no env var set or removed, no socket or port) stands.
    sidecar: >-
      2026-10-07-audit-corrections: Process-wide state and threads — citation stand_id_persistence.rs:273-283 → :289-299 (`ids_hold_across_a_remount` restructured above it; the re-exec is untouched).
    rationale: >-
      Report line map (:42): stand_id_persistence.rs, 5 citations, 1 moved — architecture.md:148 `:273-283` → `:289-299`. Report :23: `ids_in_a_child_process`, its two `println!` and the parent's argv are untouched, so only the pointer moves.
    basis: tests/blitz-tests/tests/stand_id_persistence.rs:289-299; architecture.md:148
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → dioxus-native-dom"
    change: >-
      After the module list, add that `dioxus_document` carries one `#[cfg(test)]` child module, `bridge_tests`, in its own file `dioxus_document_tests.rs` (declared by `#[path]` at the end of `dioxus_document.rs`, not in `lib.rs`): six unit tests of the bridge, a child of `dioxus_document` because `DioxusEventHandler`'s fields are private to that module; it is test-only, crate-private, feature-ungated and adds no public symbol (packages/dioxus-native-dom/src/dioxus_document.rs:468-470; packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-5). The existing `lib.rs:15-27` citation stays.
    sidecar: >-
      2026-10-07-audit-corrections: Existing Scopes, dioxus-native-dom — registered the test-only child module `dioxus_document::bridge_tests` (file `dioxus_document_tests.rs`, six unit tests).
    rationale: >-
      Report Symbols / APIs (:20) and Crates / modules (:25): a new source file and a new module land in the crate; the row's module list cites `lib.rs:15-27`, where `bridge_tests` is not declared, and `bridge_tests` / `dioxus_document_tests` read 0 hits in architecture (report :63). Named by the report's Expected amendments (:63).
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:468-470; packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-5; architecture.md:251
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      Add to the row that the stand checks share one module, `tests/blitz-tests/tests/common/mod.rs`, declared by `mod common;` in six of them (`stand_accessibility_ids`, `stand_actionable_keys`, `stand_diff`, `stand_snapshot`, `stand_snapshot_state`, `stand_snapshot_text`): the one statement of the tables `controls`, `INPUT_NAMES` and `rendered` and the helpers `boot`, `node`, `by_id`, `tree_node`, `name` and `editor_text`; it holds no test, is no cargo test target, needs no manifest line and is not matched by the agent-run `stand_*.rs` selection (tests/blitz-tests/tests/common/mod.rs:1-6).
    sidecar: >-
      2026-10-07-audit-corrections: Existing Scopes, blitz-tests — registered the shared test module `tests/common/mod.rs` (nine `pub` items, read by six stand checks, no test target).
    rationale: >-
      Report Symbols / APIs (:21-22) and Crates / modules (:25): the first shared module among the integration tests lands; `tests/common` / `mod common` read 0 hits in architecture (report :62). Named by the report's Expected amendments (:63). The row's existing stand citations (`stand_element_ids.rs:1-5`, `stand_snapshot_state.rs:1-6`) are unmoved.
    basis: tests/blitz-tests/tests/common/mod.rs:1-6 (pub items at :16, :48, :60, :99, :105, :112, :118, :130, :146); architecture.md:259
  - detector: D-arch-decisions
    severity: warning
    section: "§Conventions → Tests"
    change: >-
      Beside "a pointer-event builder is repeated per file" (which still holds for the three files it cites), add that the seven_guis stand checks instead state their shared tables and helpers once, in the module `tests/blitz-tests/tests/common/mod.rs`, which a check reads by declaring `mod common;` after its `use` block — a module in a subdirectory of the integration-test directory is no test target, and "each integration test file covers one behavior" still holds for every target (tests/blitz-tests/tests/common/mod.rs:1-6).
    sidecar: >-
      2026-10-07-audit-corrections: Conventions, Tests — recorded the first shared module among the integration tests (`tests/common/mod.rs`), a departure from per-file repetition decided by the operator (2026-10-07).
    rationale: >-
      Not a contradicted locked decision and no new library: a pattern departure the operator decided (report Decisions :95 — "the helpers restated across stand checks are stated once with the tables"). Report Crates / modules (:25): "until this chunk every integration test file stood alone and restated what it shared", which the bullet's per-file-repetition clause still implies for the whole crate. Named by the report's Expected amendments (:62). Sweep for the retired reading (`repeated per file`, `one behavior`, `stands alone`, `helper`, `tests/common`) finds it at architecture.md:114 only.
    basis: tests/blitz-tests/tests/common/mod.rs:1-6; architecture.md:114
  - detector: D-arch-decisions
    severity: warning
    section: "§Conventions → Tests"
    change: >-
      Qualify the opening clause "unit tests in `#[cfg(test)] mod tests` at the end of the source file": one exception — dioxus-native-dom's bridge tests live in a sibling file `dioxus_document_tests.rs`, declared at the end of `dioxus_document.rs` as `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;`, a child of the module under test so the tests reach its private fields without a constructor, a `pub` or a `cfg(test)` shim (packages/dioxus-native-dom/src/dioxus_document.rs:468-470).
    sidecar: >-
      2026-10-07-audit-corrections: Conventions, Tests — recorded the one unit-test module kept in its own file (`dioxus_document::bridge_tests`), beside the in-file `mod tests` convention.
    rationale: >-
      Not a contradicted locked decision: a pattern departure the operator decided (report Decisions :95 — "the new test file lives in the crate, as a child module of `dioxus_document`"). Report Files (:9-10) and Symbols / APIs (:20): the six new unit tests sit in a new file reached by `#[path]`, which the convention's stated form ("at the end of the source file") does not describe; the report's Expected amendments name §Conventions → Tests for the shared module only (:62), so this site is one the plan's list does not name. `dioxus_document.rs` keeps its own in-file `mod tests` (1 test, report :30), so the convention itself still holds there.
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:468-470; architecture.md:114
```

### security-plan
```yaml
# security-plan drift — chunk 2026-10-07-audit-corrections
# D-security-auth: no drift — the report touches no identity / session / token / key (report.md:24; the per-file `SECRET` at report.md:22 is a typed-password test fixture, not a secret source).
# D-security-deps: no drift — "Dependencies: none added, none bumped" (report.md:26); manifests, lockfile and deny.toml unchanged (report.md:15).
# D-security-input: the invariant's escalate case did NOT occur — no external-input surface was added (report.md:24, report.md:82), so nothing is unvalidated.
#   What drifted is the baseline §Input Validation text itself: one incomplete claim and six stale file:line citations, all listed by the report (report.md:69, :37, :41, :44).
#   I therefore rate all seven `warning`, not the detector's `escalate`; raise them if the pass wants citation re-points escalated.
# Every new range below was read in the work tree and matches the old range's text at 48f8b5f2. Sweep: the filter claim occurs once in security-plan (line 107);
#   the other 9 citations into edited files (lines 104, 107, 109, 122) cite file heads or dioxus_document.rs:32-39 and are unmoved.
proposals:
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | id (stable element id)` (security-plan.md:107)"
    change: >-
      Replace "a keyed list row's segment carries its Dioxus key (`{tag}[{key}]`)" with a statement of the filter: a keyed list row's segment carries its Dioxus key
      (`{tag}[{key}]`) only when that key is non-empty, holds no `/` and is the first among its siblings to claim that segment; an empty or `/`-bearing Dioxus key reads
      the positional `{tag}:{n}` segment (`/ul:0/li:1` beside `/ul:0/li[ok]`), so no Dioxus key can put a `/` or an empty segment into an id — and add the witness citation
      `packages/dioxus-native-dom/src/dioxus_document_tests.rs:47-77` to the row's citation list, with "the Dioxus-key filter's two witnesses as measured at
      escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md" appended to the row's trailing "as measured at" clause.
    sidecar: "2026-10-07-audit-corrections: §Input Validation `id` row now states the `{tag}[{key}]` filter (non-empty, `/`-free, first among siblings, else positional) and cites its two unit witnesses in dioxus_document_tests.rs."
    rationale: >-
      report.md:69 carries this amendment and notes the row "does not state the filter" while architecture.md:134 does; report.md:75 names the two new witnesses
      (`a_slashed_dioxus_key_reads_a_positional_segment` :48, `an_empty_dioxus_key_reads_a_positional_segment` :64) and report.md:106 records the security acceptance
      criterion as met by them. The row's own no-collision argument ("a component or document path has no empty segment") depends on this filter and did not state it.
    basis: "packages/dioxus-native-dom/src/element_id.rs:251-256 (the filter); packages/dioxus-native-dom/src/dioxus_document_tests.rs:47-77 (the witnesses)"
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | id (stable element id)` (security-plan.md:107)"
    change: "Re-point the citation `packages/dioxus-native-dom/src/element_id.rs:159-255` to `packages/dioxus-native-dom/src/element_id.rs:186-282` (`element_ids` through `element_id`)."
    sidecar: "2026-10-07-audit-corrections: security-plan.md:107 citation element_id.rs:159-255 → :186-282 (the `VdomWalk::walk` split moved every line past old :134 by +27)."
    rationale: "report.md:37 lists this move for security-plan.md:107; report.md:19 places `element_ids` at :186 and `element_id` at :272, text unchanged."
    basis: "packages/dioxus-native-dom/src/element_id.rs:186-282"
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | id (stable element id)` (security-plan.md:107)"
    change: "Re-point the citation `packages/dioxus-native-dom/src/element_id.rs:310-334` to `packages/dioxus-native-dom/src/element_id.rs:337-361` (the duplicate-id and empty-or-slashed-id tests)."
    sidecar: "2026-10-07-audit-corrections: security-plan.md:107 citation element_id.rs:310-334 → :337-361 (+27, text unchanged)."
    rationale: "report.md:37 lists this move; the in-file `mod tests` is byte-identical to the base and now sits at :284-604 (report.md:11)."
    basis: "packages/dioxus-native-dom/src/element_id.rs:337-361"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | id (stable element id)` (security-plan.md:107)"
    change: "Re-point the citation `packages/dioxus-native-dom/src/element_id.rs:370-423` to `packages/dioxus-native-dom/src/element_id.rs:397-450` (the removed-element and non-element tests)."
    sidecar: "2026-10-07-audit-corrections: security-plan.md:107 citation element_id.rs:370-423 → :397-450 (+27, text unchanged)."
    rationale: "report.md:37 lists this move; same +27 shift of the byte-identical test module (report.md:11)."
    basis: "packages/dioxus-native-dom/src/element_id.rs:397-450"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | id (stable element id)` (security-plan.md:107)"
    change: "Re-point the citation `packages/dioxus-native-dom/src/element_id.rs:425-576` to `packages/dioxus-native-dom/src/element_id.rs:452-603`."
    sidecar: "2026-10-07-audit-corrections: security-plan.md:107 citation element_id.rs:425-576 → :452-603 (+27, text unchanged)."
    rationale: "report.md:37 lists this move; the old range would now end 28 lines short of the module's last test in the 604-line file."
    basis: "packages/dioxus-native-dom/src/element_id.rs:452-603"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | id (stable element id)` (security-plan.md:107)"
    change: "Re-point the citation `tests/blitz-tests/tests/stand_element_ids.rs:242-275` to `tests/blitz-tests/tests/stand_element_ids.rs:257-290` (`text_and_removed_nodes_read_no_id`)."
    sidecar: "2026-10-07-audit-corrections: security-plan.md:107 citation stand_element_ids.rs:242-275 → :257-290 (the restructured `unkeyed_elements_read_their_component_path` above it grew the file 276 → 291)."
    rationale: "report.md:41 lists this move; report.md:23 records the restructure that shifted it. The old range now starts inside `unkeyed_elements_read_their_component_path` (:244)."
    basis: "tests/blitz-tests/tests/stand_element_ids.rs:257-290"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → row `Markup attributes | password and file input value (the snapshot's value)` (security-plan.md:109)"
    change: "Re-point the citation `tests/blitz-tests/tests/stand_snapshot_state.rs:490-539` to `tests/blitz-tests/tests/stand_snapshot_state.rs:445-494` (`a_typed_password_never_appears_in_the_snapshot`)."
    sidecar: "2026-10-07-audit-corrections: security-plan.md:109 citation stand_snapshot_state.rs:490-539 → :445-494 (the file shrank 539 → 494 when its helpers moved to tests/common/mod.rs)."
    rationale: >-
      report.md:44 lists this move; report.md:13 records the file's new length of 494, so the old range lies wholly past the end of the file. The mask assertions
      themselves stand unchanged (report.md:115).
    basis: "tests/blitz-tests/tests/stand_snapshot_state.rs:445-494"
    dependent-of: D-security-input
```

### test-plan
```yaml
# drift-detector: test-plan · chunk 2026-10-07-audit-corrections
# D-tests-obs-harness: NO drift — report.md:49 "Harness / gate surface: none changed" (agent-run.sh untouched, `run stand` 63 ok unchanged, no status shape / event field / log format change; report.md:27 schema none). test-plan §3 lines 98-111 and obs-plan §3 (obs-plan.md:59-70) still agree; nothing one-sided.
# D-tests-coverage: the invariant itself HOLDS (every new path is tested at tier 0: bridge_tests are tests, the private `walk` split is held by the 13 byte-identical in-file tests + `run stand` 63, `tests/common/mod.rs` is read by six passing stand checks — report.md:74-81). What drifted is test-plan's own coverage RECORD (census, baseline count, line citations), proposed below.
# D-tests-framework: framework / runner on spec (libtest via `cargo test`, `agent-run.sh`, `ci-leg.sh`; no dependency added — report.md:26). cargo-mutants / rust-code-analysis-cli / jscpd ran as dev-host audit gates only, are no CI leg and named in no master (report.md:34, :48) — no proposal. What drifted is the unit / integration LAYOUT convention §2 + §4 state, proposed below.
# Swept and found holding as worded (no proposal): test-plan.md:43 ("inside `#[cfg(test)]` modules next to the code under test"), :153 ("the other seven files of slice s07" — unrelated), :155 (`#\[cfg` absent in blitz-tests — common/mod.rs carries `#![allow(dead_code)]` only), :163 (blitz-tests depends only on dev-dependencies), :111 (63 ok), :138 (Ran 64 tests), the 8 `dioxus_document.rs` citations (all unmoved, report.md:36), every `stand_*.rs:1` citation. Keyed contract `bootstrap-phases-derive-for-route-setup-project` read: untouched by this chunk.
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom"
    change: >-
      Replace "49 unit tests in seven files" with "55 unit tests in eight files" and add to the enumeration a clause for the six new tests:
      six on the bridge, in the `cfg(test)` child module `dioxus_document::bridge_tests` (no feature gate) — a Dioxus key holding `/` and an empty one each reading the positional `{tag}:{n}` segment beside a usable key's `{tag}[{key}]`, `create_head_element` appending one element under `<head>` with its attribute and text, `mounted` listeners firing after the build and after a poll, `Document::id` being the inner document's id, and a handled event reporting cancel and stop as its listener left them (packages/dioxus-native-dom/src/dioxus_document_tests.rs:47-218).
    sidecar: "2026-10-07-audit-corrections: §1 dioxus-native-dom census 49 unit tests / seven files → 55 / eight files (+6 `bridge_tests` in the new dioxus_document_tests.rs)."
    rationale: >-
      Report Counts / qualifiers moved: "`dioxus-native-dom` unit tests 49 → 55, in seven → eight files … stated at test-plan.md:23" (report.md:30); the six tests and what each asserts are report.md:74-79; module is a child of `dioxus_document`, no feature gate (report.md:20). Listed under Expected amendments (report.md:65). The coverage invariant holds; the census that records it is stale.
    basis: "/home/turbolet/dev/projects/escher/.andromeda/test-plan.md:23 ; report.md:30"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom"
    change: "Re-point the stable-element-id tests' citation `packages/dioxus-native-dom/src/element_id.rs:257-577` → `packages/dioxus-native-dom/src/element_id.rs:284-604` (text unchanged)."
    sidecar: "2026-10-07-audit-corrections: §1 citation element_id.rs:257-577 → :284-604 (in-file `mod tests` moved +27 by the private `walk` split, byte-identical)."
    rationale: >-
      Report Files: "the in-file `mod tests` is byte-identical to the base and now sits at `:284-604` (was `:257-577`)" (report.md:11); line map "test-plan.md:23 `:257-577` → `:284-604`" (report.md:37). Verified: `#[cfg(test)]` at element_id.rs:284, file ends at 604.
    basis: "/home/turbolet/dev/projects/escher/packages/dioxus-native-dom/src/element_id.rs:284-604 ; test-plan.md:23 ; report.md:37"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native and stylo_taffy"
    change: >-
      In the parenthetical listing where the slice's test matches sit, extend the "since" list to include the new file:
      "(matches only in dioxus-native-dom: dioxus_document.rs and events.rs at that search, element_id.rs, snapshot.rs, actionable.rs, snapshot_text.rs, snapshot_diff.rs and dioxus_document_tests.rs since)".
    sidecar: "2026-10-07-audit-corrections: §1 dioxus-native / stylo_taffy bullet — the list of dioxus-native-dom files holding tests gains dioxus_document_tests.rs (eighth)."
    rationale: >-
      Same claim as the "seven files" census, restated without its tokens: this bullet enumerates exactly the seven test-holding files. Report Crates / modules: "the crate now has eight source files holding unit tests (was seven)" (report.md:25); per-file `#[test]` census names `dioxus_document_tests.rs` 6 (new) (report.md:30).
    basis: "/home/turbolet/dev/projects/escher/.andromeda/test-plan.md:24 ; report.md:25"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline → `cargo test --workspace` (CI's test leg)"
    change: >-
      Append to the bullet's re-count chain, in its existing form:
      "; re-counted at 2026-10-07-audit-corrections: 131 result lines, 548 passed · 0 failed · 5 ignored (+6 `dioxus-native-dom` `bridge_tests` unit tests in the crate's existing lib result line; no result line added — the stand checks' shared module `tests/blitz-tests/tests/common/mod.rs` is no test target), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run on its pre-CI commit 4e90f108 (escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md)".
    sidecar: "2026-10-07-audit-corrections: §9 local baseline re-counted — 131 result lines, 542 → 548 passed · 0 failed · 5 ignored (+6 bridge_tests; no new result line)."
    rationale: >-
      The workspace count is the same census at workspace scope; its latest entry reads 542. Report: "Workspace: 131 result lines, 542 passed · 0 failed · 5 ignored → 131 result lines, 548 passed · 0 failed · 5 ignored (+6 … no result line added — the shared module is no target) … stated at test-plan.md:314" (report.md:31); gate `ci-leg.sh fast` green with those counts (report.md:142); commit 4e90f108 (report.md:5). Expected amendment (report.md:67). The bullet's convention is append-a-re-count, never rewrite the earlier readings.
    basis: "/home/turbolet/dev/projects/escher/.andromeda/test-plan.md:314 ; report.md:31"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → dioxus-native-dom"
    change: >-
      Re-point the `element_id` unit-test helper citation
      "(packages/dioxus-native-dom/src/element_id.rs:264-268; packages/dioxus-native-dom/src/element_id.rs:370-405)" →
      "(packages/dioxus-native-dom/src/element_id.rs:291-295; packages/dioxus-native-dom/src/element_id.rs:397-432)" (text unchanged).
    sidecar: "2026-10-07-audit-corrections: §3 crate-local helpers citations element_id.rs:264-268 → :291-295 and :370-405 → :397-432 (+27, text identical)."
    rationale: >-
      Report line map: "test-plan.md:96 `:264-268` → `:291-295`, `:370-405` → `:397-432`" — an old line ≥ 134 moves +27 (report.md:37). Verified: `fn build` at element_id.rs:291-295; `a_removed_element_reads_no_id` at :397-432.
    basis: "/home/turbolet/dev/projects/escher/packages/dioxus-native-dom/src/element_id.rs:291-295 ; test-plan.md:96 ; report.md:37"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → dioxus-native-dom"
    change: "Re-point the parsed-markup pin citation `tests/blitz-tests/tests/stand_snapshot_state.rs:120-127` → `tests/blitz-tests/tests/stand_snapshot_state.rs:75-82` (text unchanged)."
    sidecar: "2026-10-07-audit-corrections: §3 crate-local helpers citation stand_snapshot_state.rs:120-127 → :75-82 (the file shed its copies of the shared helpers, 539 → 494 lines)."
    rationale: >-
      Report line map: "`tests/blitz-tests/tests/stand_snapshot_state.rs`: … test-plan.md:96 `:120-127` → `:75-82`" (report.md:44). Verified: `fn parsed_markup` with the `disabled="false"` / bare `disabled` markup sits at stand_snapshot_state.rs:75-82.
    basis: "/home/turbolet/dev/projects/escher/tests/blitz-tests/tests/stand_snapshot_state.rs:75-82 ; test-plan.md:96 ; report.md:44"
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → Conventions → Test file location"
    change: >-
      Keep the bullet and add the one departure:
      "; one unit-test module sits out of line — `dioxus_document::bridge_tests` in `dioxus_document_tests.rs`, declared `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;` at the end of its parent, a child of `dioxus_document` because `DioxusEventHandler`'s two fields are private to that module and it has no constructor (packages/dioxus-native-dom/src/dioxus_document.rs:468-470; packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-5)".
    sidecar: "2026-10-07-audit-corrections: §4 Test file location — first out-of-line unit-test module recorded (`bridge_tests` via `#[path]`, a `cfg(test)` child of `dioxus_document`)."
    rationale: >-
      §4 states unit tests "sit in `#[cfg(test)] mod tests` blocks inside the source file"; the chunk adds a unit-test module in its own file. Report Files: `dioxus_document_tests.rs` new, 218 lines, "a `#[cfg(test)]` child module of `dioxus_document` named `bridge_tests`"; `dioxus_document.rs:468-470` the three declaring lines (report.md:9-10); why a child module (report.md:20); the operator's decision (report.md:95). Runner unchanged (`cargo test -p dioxus-native-dom --locked --lib`, report.md:122-123). Verified the three lines at dioxus_document.rs:468-470.
    basis: "/home/turbolet/dev/projects/escher/packages/dioxus-native-dom/src/dioxus_document.rs:468-470 ; test-plan.md:132 ; report.md:9-10"
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      First clause: "inline `#[cfg(test)]` modules in source files (as above)" → "inline `#[cfg(test)]` modules in source files (as above), bar one `cfg(test)` module held in a sibling file through `#[path]` — `dioxus_document::bridge_tests` (packages/dioxus-native-dom/src/dioxus_document.rs:468-470)".
    sidecar: "2026-10-07-audit-corrections: §2 Directory pattern — the inline-module rule gains its one out-of-line exception (`bridge_tests`)."
    rationale: >-
      Second occurrence of the claim the §4 amendment qualifies (unit tests live inside the source file), worded here as "inline `#[cfg(test)]` modules in source files". Same report facts: report.md:9-10, :20, :25.
    basis: "/home/turbolet/dev/projects/escher/.andromeda/test-plan.md:63 ; report.md:25"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      Second clause: "`tests/blitz-tests` holds integration tests, one file per behavior under `tests/`" → "`tests/blitz-tests` holds integration tests, one test-target file per behavior under `tests/`, and one shared module that is no target — `tests/common/mod.rs`, the stand checks' tables and helpers, read through `mod common;` by six `stand_*.rs` checks (tests/blitz-tests/tests/common/mod.rs:1-6)"; existing citations kept.
    sidecar: "2026-10-07-audit-corrections: §2 Directory pattern — blitz-tests' first shared integration-test module `tests/common/mod.rs` recorded beside one-file-per-behavior."
    rationale: >-
      Report Crates / modules: "`blitz-tests` — NEW shared module `tests/common/mod.rs` among the integration tests, the first one: until this chunk every integration test file stood alone and restated what it shared" (report.md:25); it is not a test target, not matched by the `stand_*.rs` glob, adds no result line (report.md:21); the report itself carries this site: "test-plan.md:25 and test-plan.md:63 say … 'one file per behavior under `tests/`'. That still holds for the test targets; the directory now also holds one shared module that is no target" (report.md:68).
    basis: "/home/turbolet/dev/projects/escher/.andromeda/test-plan.md:63 ; report.md:68"
  - detector: D-tests-framework
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: >-
      Opening clause: "holds Blitz's integration tests, one file per behavior under `tests/`" → "holds Blitz's integration tests, one test-target file per behavior under `tests/`, plus one shared module that is no target, `tests/common/mod.rs` — the stand checks' tables (`controls`, `INPUT_NAMES`, `rendered`) and helpers, each stated once"; existing citations and the rest of the bullet unchanged.
    sidecar: "2026-10-07-audit-corrections: §1 tests/blitz-tests bullet — shared module `tests/common/mod.rs` recorded beside one-file-per-behavior."
    rationale: >-
      Second occurrence of the "one file per behavior" claim (`one file per` → 2 hits, test-plan.md:25 and :63 — report.md:68). Nine `pub` items moved verbatim into the module, three tables and six helpers (report.md:21); six checks declare `mod common;` (report.md:13, :22).
    basis: "/home/turbolet/dev/projects/escher/.andromeda/test-plan.md:25 ; report.md:68"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → dioxus-native-dom"
    change: >-
      Add to the bullet: "the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no constructor — they build one over `doc.vdom` and `doc.vdom_state` and call `handle_event` with an `EventState` they keep, the only way to read `propagation_is_stopped()`; no function under test gained a constructor, a `pub` or a `cfg(test)` shim (packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-5; packages/dioxus-native-dom/src/dioxus_document_tests.rs:163-218)".
    sidecar: "2026-10-07-audit-corrections: §3 crate-local helpers — how `bridge_tests` reaches the private `DioxusEventHandler` (child module, kept `EventState`)."
    rationale: >-
      Report Expected amendments: "test-plan §3 Crate-local test helpers (a test module that is a child of `dioxus_document`) — carried: Symbols / APIs" (report.md:66). Facts: report.md:20 (child module, builds the handler over `doc.vdom` / `doc.vdom_state`, keeps the `EventState`), report.md:54 (no constructor, `pub` or shim), report.md:98 (`EventState::propagation_stopped` has no reader, so the mutant is killable only by a test that keeps the `EventState`), test at `:163` (report.md:79).
    basis: "/home/turbolet/dev/projects/escher/packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-5 ; test-plan.md:96 ; report.md:20"
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers"
    change: >-
      Add a bullet (the section has none for blitz-tests): "**blitz-tests (stand checks):** `tests/common/mod.rs` is the one statement of what the stand checks share, no test target and no test in it — the tables `controls(task)` (each lean task's controls with their roles), `INPUT_NAMES` (six input names) and `rendered(task)` (the ids rendered per task), and the helpers `boot`, `node`, `by_id`, `tree_node`, `name`, `editor_text`; a check declares `mod common;` after its `use` block and reads what it needs — `stand_accessibility_ids`, `stand_snapshot`, `stand_snapshot_text`, `stand_snapshot_state`, `stand_diff`, `stand_actionable_keys`; `stand_element_ids` keeps its own `boot`, and `tab_order`, `task_keys`, `ceiling`, `component`, the `cards` array and each file's `SECRET` stay per-file (tests/blitz-tests/tests/common/mod.rs:1-6; tests/blitz-tests/tests/common/mod.rs:16; tests/blitz-tests/tests/common/mod.rs:48; tests/blitz-tests/tests/common/mod.rs:60; tests/blitz-tests/tests/common/mod.rs:99-146)".
    sidecar: "2026-10-07-audit-corrections: §3 crate-local helpers — new blitz-tests bullet for the shared stand module `tests/common/mod.rs` (three tables, six helpers, six readers)."
    rationale: >-
      Not a retired claim but the helper-inventory home for the new shared test surface; NOT on the plan's expected list for test-plan (that list names :25 / :63 only, report.md:68) — drop if the two convention amendments above are judged enough. Facts: nine `pub` items with their lines (report.md:21), who reads what and what stays per-file (report.md:22), `mod common;` after the `use` block (report.md:87), a subdirectory module is no cargo test target (report.md:98). Verified the nine `pub` items at common/mod.rs:16, 48, 60, 99, 105, 112, 118, 130, 146.
    basis: "/home/turbolet/dev/projects/escher/tests/blitz-tests/tests/common/mod.rs:16 ; report.md:21-22"
```

### obs-plan
```yaml
# Detector verdicts (all three invariants hold; the one proposal is a scope-completeness amendment, not a violation):
#   D-obs-instrumentation — no drift. Report :82 "no hot-path operation ... was added"; VdomWalk::walk is a private restructure with "no log site added" (:80). §4/§5/§6 require nothing new.
#   D-obs-stack — invariant holds (no off-spec logger, no OTel setup, Dependencies "none added, none bumped" at :26). One §3 sentence is made incomplete in scope by the chunk; proposed below.
#   D-obs-pii — no drift. No logging added (census 0 at :114, :138); assertion messages carry fixture ids only (:74, :81).
# Key file read: contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md (otel-sdk-install, pii-scrubbing-wire) — neither label is touched by a Change; no proposal.
# Re-checked on the work tree: a grep for tracing / log macros / println! / eprintln! / dbg! / env:: / escher_telemetry / subscriber / fs:: over tests/blitz-tests/tests/common/mod.rs and packages/dioxus-native-dom/src/dioxus_document_tests.rs returns 0 hits; the only println! in stand_*.rs are stand_id_persistence.rs:331 and :334 (the re-executed child, untouched).
proposals:
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation) — the headless-stand bullet (obs-plan.md:68)"
    change: >-
      Widen the census scope from "it or its checks" to also name the checks' shared module: the headless stand
      `seven_guis::stand` (native only; `boot` / `boot_timer` / `options`, driven in-process by the `stand_*` checks,
      six of which declare the shared module `tests/blitz-tests/tests/common/mod.rs`) installs no subscriber — no
      `escher_telemetry::init` and no env read in it, its checks or their shared module, and no `println!` in any of
      them save `stand_id_persistence`'s re-executed child (rest of the sentence unchanged); append
      `tests/blitz-tests/tests/common/mod.rs:1-6` to the citations and
      `escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md` to the "as measured at" basis. The two existing
      citations keep their numbers.
    sidecar: >-
      2026-10-07-audit-corrections — §3 Logging stack: the headless-stand census (no subscriber, no env read, no
      println! save stand_id_persistence's child) now also covers the stand checks' new shared module
      tests/blitz-tests/tests/common/mod.rs; census re-measured 0, no citation moved.
    rationale: >-
      Not a violated invariant — the stack matches §3 and the sentence stays true — but the chunk makes its stated
      scope incomplete. Report :21 and :25 add `tests/blitz-tests/tests/common/mod.rs`, the first shared module among
      the integration tests, which is not a `stand_*` check and now holds the `boot` helper five checks call
      (:21-22); the sentence's census names only the stand and "its checks". Report :71 carries this as the expected
      obs-plan amendment ("the census sentence over the two new files; no citation moves") and :81 / :114 / :138
      record the census as 0. The other new file, `dioxus_document_tests.rs`, is a dioxus-native-dom unit-test module
      that never boots the stand; obs-plan has no sentence about that crate's unit tests, so it is left out of this
      bullet rather than given a new claim. Swept for other occurrences: obs-plan.md:83 ("the stand checks install no
      escher sink") restates the conclusion, not the census scope, and stands as written; §6 tests/blitz-tests
      (obs-plan.md:218-220) and §8 name no stand check. No dependent proposal.
    basis: "/home/turbolet/dev/projects/escher/.andromeda/obs-plan.md:68; /home/turbolet/dev/projects/escher/escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md:71; /home/turbolet/dev/projects/escher/tests/blitz-tests/tests/common/mod.rs:1-6"
```

### design-system
```yaml
proposals: []
# D-design-tokens: no drift. The report's Coverage of new surfaces carries `tokens n/a` on all three entries (report.md:74 `dioxus_document::bridge_tests`, :80 `VdomWalk::walk` restructure, :81 `tests/common/mod.rs`) and no `hardcoded✗` flag anywhere; report.md:82 states "No UI element, no rendered surface, no hot-path operation and no external-input surface was added", and report.md:15 confirms no stand markup under `examples/` changed. The invariant has no new UI to evaluate against §Color Palette / §Spacing / §Typography.
# Out of detector scope, noted for the orchestrator: design-system.md's single citation into an edited file (design-system.md:337, `packages/dioxus-native-dom/src/dioxus_document.rs:97-98`) is unmoved per report.md:36 (the only hunk is `@@ -466,0 +467,4 @@`), so no re-point is needed.
```

### layout-templates
```yaml
# D-layout-surface: no drift. The report's Changes add no user-facing surface or region:
#   report.md:82 "No UI element, no rendered surface, no hot-path operation and no external-input surface was added."
#   report.md:15 no stand markup under examples/, script or workflow changed; report.md:49 scripts/agent-run.sh untouched (its cli entry at layout-templates.md:71 stands).
#   The two new files (dioxus_document_tests.rs, tests/blitz-tests/tests/common/mod.rs) are test-only, "not a product surface" (report.md:74, :81).
# Not in this detector's scope, noted for the orchestrator: layout-templates.md's 3 citations into dioxus_document.rs (:35, :36, :37) are unmoved per report.md:36 and :46.
proposals: []
```

### a11y-plan
```yaml
proposals: []
# D-a11y-surface: no drift. The report adds no interactive UI element — report.md:82 "No UI element, no rendered surface, no hot-path operation and no external-input surface was added"; report.md:15 no stand markup under `examples/` changed. The chunk's new surfaces are test-only (`dioxus_document::bridge_tests`, `tests/common/mod.rs`) plus a private restructure of `VdomWalk::walk` with ids unchanged (report.md:80-81).
# D-a11y-obs-schema: no drift. report.md:27 "Schema / config: none. No log schema, event schema, violation schema, agent-run event field or config key changed." a11y-plan.md:87 still records the violation schema as not yet defined, so there is nothing to diverge.
# Keyed contracts: the one key (bootstrap-phases-derive-for-route-setup-project) is touched by no detector and no Change; its file was not read.
#
# Outside both detectors, so not proposed — relayed from the report, not re-measured by me:
# report.md:70 lists a11y-plan as an expected amendment for five moved file:line citations (report.md:38, :43, :44):
#   a11y-plan.md:73  (§3 Keyboard test harness)        stand_snapshot_state.rs:308-344     -> :263-299
#   a11y-plan.md:204 (§5 Test harness pattern)         stand_snapshot_state.rs:308-344     -> :263-299
#   a11y-plan.md:314 (§8 Error recovery)               stand_accessibility_ids.rs:162-177  -> :86-101
#   a11y-plan.md:314 (§8 Error recovery)               stand_accessibility_ids.rs:303-320  -> :227-244
#   a11y-plan.md:314 (§8 Error recovery)               stand_snapshot.rs:337-390           -> :210-263
# The old ranges are still in the body at those three lines (seen in my read of a11y-plan.md). The report says the other 11 a11y-plan citations are unmoved.
# report.md:70 also says "the tables' new home named" (`controls` / `INPUT_NAMES` / `rendered` at tests/blitz-tests/tests/common/mod.rs:16, :48, :60). a11y-plan.md:269 cites stand_accessibility_ids.rs:1-4 for the 15 controls' roles and names; that citation is unmoved, but the tables it rests on now live in the shared module.
```
