proposals:
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Unsupported features]
    change: 'Drop `:focus-within` from the always-false list — it now reads `:focus-visible`, `:valid`, `:invalid`, `:required`, `:target` always match false — and add that, since 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness (upstream #1226, `38ca4e777`, PROVISIONAL on the delegate''s F1 answer, inputs#I2), `:focus-within` matches the focused element and its DOM ancestors: `NonTSPseudoClass::FocusWithin` reads `ElementState::FOCUS_WITHIN` (packages/blitz-dom/src/stylo.rs:477-479), the bit kept on the focus''s ancestor chain by `set_focus_to`, `clear_focus`, `clear_interaction_state_for_removed_node` and `add_children_to_parent`.'
    sidecar: 'arch [Unsupported features]: `:focus-within` no longer listed as always false — it matches the focused element and its ancestors since the 2026-10-19 upstream take of #1226 (PROVISIONAL).'
    rationale: 'The report''s Symbols / APIs and Counts › `:focus-within` matching: `match_non_ts_pseudo_class` now reads `self.element_state().contains(ElementState::FOCUS_WITHIN)` where it read `false` (added stylo.rs:477-479), contradicting the Established Decision at architecture.md:85 that lists `:focus-within` as always matching false; Expected amendment 2. `:focus-visible` still never matches. A sweep of architecture finds no other site stating the claim.'
    basis: 'packages/blitz-dom/src/stylo.rs:477-479; .andromeda/architecture.md:85'
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Nodes and widgets
    change: 'Register the three public `Node` methods taken from upstream #1226: `add_focus_within(&mut self)` and `remove_focus_within(&mut self)` set or clear `ElementState::FOCUS_WITHIN` and call `mark_ancestors_dirty()`, and `contains_focus(&self) -> bool` reads the bit, placed after `is_hovered` and before `focus` (packages/blitz-dom/src/node/node.rs:736-757) — PROVISIONAL (inputs#I2).'
    sidecar: 'arch §Standard Contracts → Nodes and widgets: registers `Node::add_focus_within` · `remove_focus_within` · `contains_focus` (upstream #1226, 2026-10-19 take).'
    rationale: 'The report''s Symbols / APIs: "`Node` gains three pub methods" (new-text rows node.rs 736-743 @738, 745-751 @746, 753-757 @754); none appears in architecture''s registry sections, so the new public API is unregistered.'
    basis: 'packages/blitz-dom/src/node/node.rs:736-757'
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [DOM semantics]
    change: 'Add, after the interaction-state teardown clause: since 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness (upstream #1226, PROVISIONAL on the delegate''s F1 answer, inputs#I2) `BaseDocument::set_focus_to` returns `false` for a node not in the document, after the same-node check and before the `Focussed node` INFO line (packages/blitz-dom/src/document.rs:1800-1803), moves `FOCUS_WITHIN` from the old focus''s ancestor chain to the new one, the shared prefix staying (packages/blitz-dom/src/document.rs:1824-1842), and requests a redraw through `ShellProvider::request_redraw` (packages/blitz-dom/src/document.rs:1845-1849); `clear_focus` removes `FOCUS_WITHIN` from the cleared node''s element ancestors and requests a redraw (packages/blitz-dom/src/document.rs:1780-1784; packages/blitz-dom/src/document.rs:1786); `clear_interaction_state_for_removed_node` clears the removed focus''s chain (packages/blitz-dom/src/document.rs:959-973); `DocumentMutator::add_children_to_parent` carries the state with a moved subtree that contains focus, or only clears it when focus left with the subtree (packages/blitz-dom/src/mutator.rs:734-739; packages/blitz-dom/src/mutator.rs:747-751; packages/blitz-dom/src/mutator.rs:810-836); the `FOCUS_WITHIN` writes go through `snapshot_node_and` and mark no changed-set entry, so a focus move still marks exactly its two nodes (tests/blitz-tests/tests/focus_within.rs:173-192; tests/blitz-tests/tests/focus_redraw.rs:73-120).'
    sidecar: 'arch [DOM semantics]: records the not-in-document focus refusal, the `:focus-within` chain moves and the focus set/clear redraw taken from upstream #1226 (PROVISIONAL).'
    rationale: 'The report''s Symbols / APIs (`set_focus_to`, `clear_focus`, `clear_interaction_state_for_removed_node`, `add_children_to_parent`) and Counts › A node outside the document is not focused / A focus set or cleared requests a redraw: a changed public contract of `set_focus_to` and `clear_focus` that no architecture section states; Expected amendment 3 names [DOM semantics] (architecture.md:98) as new text, PROVISIONAL per card row 5.'
    basis: 'packages/blitz-dom/src/document.rs:1800-1803; packages/blitz-dom/src/document.rs:1845-1849; .andromeda/architecture.md:98'
  - detector: D-arch-resources
    severity: warning
    section: §Cross-cutting Patterns → Service wiring
    change: 'The redraw clause reads: redraws are requested through `ShellProvider::request_redraw` after mutations, resource loads, hover, device changes and, since 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness (PROVISIONAL), a focus set or cleared — `set_focus_to` and `clear_focus` request it (packages/blitz-dom/src/document.rs:1786; packages/blitz-dom/src/document.rs:1845-1849), the Tab arm of `handle_key_or_input_event` no longer requesting its own.'
    sidecar: 'arch §Cross-cutting Patterns → Service wiring: focus set and cleared now request a redraw; the Tab arm''s own request is gone.'
    rationale: 'The report''s Counts › A focus set or cleared requests a redraw: architecture.md:268 is stale by omission — its list of redraw triggers names mutations, resource loads, hover and device changes only, while `set_focus_to` and `clear_focus` now request one and the Tab arm''s `request_redraw` was removed (keyboard.rs lines removed only). A restatement of the redraw contract the [DOM semantics] proposal adds.'
    basis: '.andromeda/architecture.md:268'
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: §Project Intent
    change: 'The `Upstream sync:` line reads: merge base `23b9b26a9` unmoved (merge commit 3eda07bc, 2026-10-17); upstream `main` last read at `17de049b1`, 7 commits past it, of which `fb8d6c15e` (upstream #1220) was taken alone at the 2026-10-18 sync (`275950dc0`) and `38ca4e777` (`:focus-within`, upstream #1226) and `17de049b1` (the WPT pin, upstream #1228) are taken by no-commit cherry-picks onto `67f17b9d6` (`79603d1ef`, both `(cherry picked from commit …)` trailers); the four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) wait again, since they put a git Taffy in the root manifest, so Taffy stays the registry "0.15"; this take is F1 «Cherry-pick two (Recommended)» answered by the operator as delegate, PROVISIONAL until the founder''s own word (inputs#I2) (as measured at escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/take.md).'
    sidecar: 'arch §Project Intent: upstream sync line re-read at `17de049b1` — #1226 and #1228 taken by cherry-pick, base `23b9b26a9` unmoved, Taffy-main commits still out (PROVISIONAL).'
    rationale: 'The report''s Counts › The upstream sync line: architecture.md:278 (×6 syncline, ×2 taffymain) is stale — "last read at `fb8d6c15e`, 5 commits past it … taken alone" — now `main` is read at `17de049b1`, 7 past `23b9b26a9`, two commits taken; Expected amendment 1. The recorded sync decision (partial take, Taffy out per [Dependency pinning]) moves. No other architecture site states the last-read commit; the "merged at `23b9b26a9`" sites (:75, :90, :150) stay true.'
    basis: '.andromeda/architecture.md:278'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: 'The seven engine changes'' merge surface, by file, is owned by a CARRY on «Upstream sync ahead of polish and ship», re-read per row at `17de049b1` (escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/readings.md), in place of «Upstream sync ahead of visual lint and real-app readiness».'
    sidecar: 'arch [Scrolling and input] detail: the engine-debts merge-surface CARRY moves to «Upstream sync ahead of polish and ship», this sync''s re-read cited.'
    rationale: 'The report''s Counts › The CARRY owner and the P5 card rows 3 (inputs#I3, the operator''s own word): the CARRY that architecture.md:90 names on «Upstream sync ahead of visual lint and real-app readiness» — the chunk now wrapped — moves to «Upstream sync ahead of polish and ship», so the named owner is stale.'
    basis: '.andromeda/architecture.md:90'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Scrolling and input]
    change: 'The document clock''s merge surface is owned by the same CARRY on «Upstream sync ahead of polish and ship», re-read at `17de049b1` (escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/readings.md), in place of «Upstream sync ahead of visual lint and real-app readiness».'
    sidecar: 'arch [Scrolling and input] detail: the document clock''s merge-surface CARRY moves to «Upstream sync ahead of polish and ship».'
    rationale: 'The report''s Counts › The CARRY owner and P5 card row 4 (inputs#I3): architecture.md:91 restates the owner the primary retires; a second occurrence of the same claim.'
    basis: '.andromeda/architecture.md:91'
    dependent-of: D-arch-decisions
note: 'D-retired: the report''s Retired reads none, so no correction. No new port, socket, env var, crate, dependency or log site (report Changes), so §Occupied Resources and §Stack need nothing. Every behaviour fact proposed is PROVISIONAL (inputs#I2, card row 5). The [DOM semantics] / Service-wiring pair could also sit under §Standard Contracts → Events if the orchestrator prefers the contract home; the plan''s Expected amendment 3 names [DOM semantics]. Citation re-points (architecture.md:85 stylo.rs range, :268 keyboard.rs and document.rs lines, :270 document.rs:897-999) are the citation sweep''s, not proposed here.'
