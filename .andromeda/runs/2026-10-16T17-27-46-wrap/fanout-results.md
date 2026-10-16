# Fan-out results — 2026-10-16-file-splits (resume window)

Snapshot before: `tree 7a5e3917fc69b6547c35b1be45764b5838caed7b index 168724d610176b8afca6b1c65105e521d3cd7126 head
54d4ea21ff66afbd297b1904f297deb9ad571830` · after: `tree b31d271fc621c84f0f5911ce11dfa44a71079a37`, index and head
unchanged; `git diff-tree` printed exactly the seven `A` lines. Every receipt's count equals its file's.

architecture: 0 proposals · fanout-architecture.md
- note: neither detector fires; §Conventions → Tests (architecture.md:115) and the escher-driver row (:257) are the
  plan's expected amendments outside both detectors — carried by the orchestrator (items 1–3 below).
  1. raised (check 5) · §Conventions → Tests — one out-of-line `#[path]` test module → five: `bridge_tests` and, since
     2026-10-16-file-splits, `document::changed_set_tests`, `document::font_reading_tests`, `mcp::tests`,
     `schema::tests` · apply (playbook: Accurate this-chunk addition; the plan's reviewed entry names the change)
  2. raised (check 5) · §Existing Scopes → escher-driver — `mcp`'s three private children (framing · listing · result)
     and its tests' file, `schema`'s private child `spec` (its six shape types, re-exported) and its tests' file; the
     count stays fourteen · apply (Accurate this-chunk addition; the plan's entry)
  3. raised (check 5) · §Existing Scopes → blitz-dom — the row's `document` entry names its two `cfg(test)` children
     in files of their own, as the dioxus-native-dom row names `bridge_tests` · apply (Accurate this-chunk addition;
     the plan's entry names this section)
security-plan: 0 proposals · fanout-security-plan.md
- note: all three detectors hold; its citations are the citation sweep's (done).
design-system: 0 proposals · fanout-design-system.md
- note: D-design-tokens holds; :397's citation the sweep's (done).
layout-templates: 0 proposals · fanout-layout-templates.md
- note: D-layout-surface holds; :74's citations the sweep's (done).
test-plan: 6 proposals · fanout-test-plan.md
- note: D-tests-obs-harness holds; D-tests-coverage holds on its letter, its proposals carry the by-file count sites
  and inputs#I3; the leaf `.claude/docs/services/escher-driver.md:60` holds the 72 — the cascade's.
  1. D-tests-framework · §2 Directory pattern — bar one `#[path]` module → five · apply (Accurate this-chunk addition;
     the plan's entry; text re-derived)
  2. D-tests-framework (dependent-of) · §4 Test file location — one module sits out of line → five · apply (as 1)
  3. D-tests-coverage · §1 blitz-dom — «18 in document.rs» → 9 in document.rs and 9 in its two sibling files · apply
     (the operator's recorded direction, inputs#I3; Accurate this-chunk addition)
  4. D-tests-coverage · §1 escher-driver — «inline», «schema.rs 6», «mcp.rs 11» → the sibling files · apply (as 3)
  5. D-tests-coverage (dependent-of) · §4 What unit tests cover → escher-driver — «schema.rs pins», «mcp.rs that» →
     the sibling files · apply (as 3)
  6. D-tests-coverage (dependent-of) · §3 → Session lifecycle — 72 → the measured 73 (client.rs 3), mcp and schema
     counts under their sibling files · apply (inputs#I3 settles it — the report's Spec claims disproved, a
     pre-existing drift the operator directed amended; check 6 disposes it here, the leaf's 72 by the cascade)
obs-plan: 0 proposals · fanout-obs-plan.md
- note: all three detectors hold; its four mcp.rs citations already read the line map's targets.
a11y-plan: 0 proposals · fanout-a11y-plan.md
- note: both detectors hold; :281's citation the sweep's (done).

Checks: 0 proposals rejected for a source the report does not carry (every coordinate is the report's: document.rs
3462-3464 · 3466-3468, mcp.rs 285-287, schema.rs 477-479, the sibling files' 1-235 · 1-26 from New text, by line;
dioxus_document.rs:780-782 the doc's own standing citation) · cross-contradiction none · intent-consistency: the scope
record holds no line (`scope: clean`) and the proposals stay inside the plan's entries and the operator's directive ·
check 5: every expected entry carried (1–3 above; test-plan 1–2; citations by the sweep; leaves by the cascade) ·
check 6: the one disproved claim (72) disposed by test-plan 6 and the cascade's leaf.
