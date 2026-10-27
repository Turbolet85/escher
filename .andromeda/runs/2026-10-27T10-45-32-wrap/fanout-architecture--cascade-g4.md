proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 78
    section: "## Standard Contracts"
    change: "add after the `ElementData::classes` row a row: `ElementData::flush_open_state(&mut self)`, public since 2026-10-27-upstream-sync-engine-debts (upstream's, taken on the founder's word): it sets `ElementState::OPEN` on a `details` or `dialog` that carries `open`, called by `ElementData::new` and by `DocumentMutator::set_attribute` / `remove_attribute` on an `open` write, and Stylo's `NonTSPseudoClass::Open` reads that state, so `:open` matches (packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/node/element.rs:488-489; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510)."
    sidecar: "Standard Contracts → Nodes and widgets: registered the public `ElementData::flush_open_state` (open state for `details` / `dialog`, read by `:open`), upstream's, taken at 2026-10-27-upstream-sync-engine-debts."
    rationale: "Report Changes → Symbols / APIs: `ElementData::flush_open_state` (element.rs:543-549, new, pub), called from `ElementData::new` (:488-489) and the mutator's `open` arms (mutator.rs:368-371, :520-523); no Nodes and widgets row names it, so a new public API goes unregistered."
    basis: .andromeda/architecture-detail/standard-contracts.md:78
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 23
    section: "## Standard Contracts"
    change: "after the `doc.mutate()` method list add: `set_attribute` and `remove_attribute` call `ElementData::flush_open_state` on an `open` write, so a `details` or `dialog` reads its open state at once — in `set_attribute` the `open` arm a second independent `if` after escher's range re-sanitize arm (`min` · `max` · `step` → `resanitize_range_input`), the take's resolution 3 at 2026-10-27-upstream-sync-engine-debts (packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523)."
    sidecar: "Standard Contracts → Mutation, query and CSSOM: the mutator's `open` arms in `set_attribute` and `remove_attribute`, set_attribute's standing after escher's range re-sanitize arm (2026-10-27-upstream-sync-engine-debts)."
    rationale: "Report Changes → Symbols / APIs: `DocumentMutator::set_attribute` / `remove_attribute` flush open state on an `open` write (mutator.rs:368-371, :520-523), the `open` arm after the range re-sanitize arm as a second independent `if`; mutator.rs is a conflict surface of the take."
    basis: .andromeda/architecture-detail/standard-contracts.md:23
    dependent-of: D-arch-resources
note: "Not this group's picked entities but standard-contracts facts the report moves: F5 (`document_may_load` + its `FetchError` variant, a new public API for D-arch-resources) and F6 (the CSSStyleDeclaration accessors) belong with the JavaScript contract (standard-contracts.md:210, body architecture.md:173 incl. its 'a JS bootstrap' now nine scripts under runtime/bootstrap/), the shell's redraw route and set_ime_enabled with Paint and shell (standard-contracts.md:96), and F2's PROVISIONAL gap marker on the Headless stand contract (standard-contracts.md:128) drops — none was picked here, so the new-detail pass or the orchestrator places them."
