group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:18   class: omission
  leaf-text: «`run stand` 205 · 0 · 7 over 50 files at 2026-10-25-cold-agent-bar (»
  source: .andromeda/test-plan.md:176 «`run stand`'s `run.end` unmoved at passed 205 · failed 0 · ignored 7 over 50 files, and `run all` reads passed 633 · failed 0 · ignored 12 — `pseudo_element_flow_classification` 3, a new file»
  replace-with: «`run stand` 205 · 0 · 7 over 50 files at 2026-10-25-cold-agent-bar, unmoved at 2026-10-27-upstream-sync-engine-debts, where `run all` reads 633 · 0 · 12 with upstream's `pseudo_element_flow_classification` 3, a new file with no `stand_` prefix picked up by `run all` with no script change (»

- leaf: .claude/docs/tests-summary.md:27   class: stale
  leaf-text: «27 DOM API tests (`tests/dom.rs`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:71 «- **blitz-vibey-script:** tests/dom.rs holds 30 `#[test]` functions covering the JS DOM APIs»
  replace-with: «30 DOM API tests (`tests/dom.rs`)»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1117 · 0 · 12 over 208 result lines with the served Cells and load page at 2026-10-25-cold-agent-bar (from»
  source: .andromeda/test-plan-detail/ci-integration.md:71 «re-counted with upstream's eleven commits: 209 result lines, 1125 passed · 0 failed · 12 ignored — +1 result line (`pseudo_element_flow_classification`) and +8 passed»
  replace-with: «`cargo test --workspace` 1125 · 0 · 12 over 209 result lines with upstream's eleven commits at 2026-10-27-upstream-sync-engine-debts (from 208 · 1117: +1 result line — `pseudo_element_flow_classification` — and +8 passed — `pseudo_element_flow_classification` 3, blitz-vibey-script's `tests/dom.rs` 3, dioxus-native-dom 1, `apps/browser/src/tab.rs` 1; timings not re-measured); 1117 · 0 · 12 over 208 result lines with the served Cells and load page at 2026-10-25-cold-agent-bar (from»

- leaf: .claude/docs/tests-summary.md:31   class: omission
  leaf-text: «only the browser crates test (about pages, history, favicon, suggestions, persistence)»
  source: .andromeda/test-plan.md:13 «- **apps/browser:** tests exist only in the browser crates: about_pages (3), browser_history (4), favicon (5), url_suggestions (12 `#[test]` plus 4 `#[tokio::test]`), persistence (10), tab (1)»
  replace-with: «only the browser crates test (about pages, history, favicon, suggestions, persistence, tab)»
