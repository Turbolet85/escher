proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 61
    section: ## Existing Scopes
    change: append to blitz-vibey-script's What it is — "; since the 2026-10-27 upstream sync the runtime is a module tree, `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs`, with nine JS bootstrap scripts under `runtime/bootstrap/` (`css`, `dom_exception`, `fetch`, `fonts`, `interfaces`, `node_constants`, `style`, `stylesheets`, `window`) concatenated into `BOOTSTRAP_JS` (packages/blitz-vibey-script/src/runtime/bootstrap.rs:14-36), `runtime.rs` keeping 151 lines and `ScriptRuntime`'s public methods their names, and the element bindings are the `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs` children of `dom/element.rs` — upstream's split, a move with nothing removed"
    sidecar: existing-scopes — blitz-vibey-script's scope names the runtime's and the element bindings' module trees and the nine JS bootstrap scripts taken at the 2026-10-27 upstream sync (F7)
    rationale: report Changes › Symbols / APIs (`e6f4615ec`, the runtime split into `runtime/{…}.rs` with nine bootstrap scripts and the element bindings into `dom/element/{…}.rs`, a move, `runtime.rs` keeping 151 lines) and › Crates / modules (13 Rust modules, 9 JS bootstrap files); the entity lists the crate's and `dom`'s modules but no child module of `runtime` or `dom::element`
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:21
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: ## Existing Scopes
    change: after the inline text layout item of blitz-tests' What it is, add — "; `::before`/`::after` pseudo-elements voting in flow classification, since the 2026-10-27 upstream sync — `pseudo_element_flow_classification`, upstream's, 3 tests, an auto-discovered target with its `mod` line in the unbuilt `all.rs`: an inline-block whose only child is a pseudo exporting its line baseline and bottom-aligned, and a grid row growing by the baseline shim of pseudo-only items, both layout modes not asserted by the file (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78)"
    sidecar: existing-scopes — blitz-tests' scope names `pseudo_element_flow_classification`, the target the 2026-10-27 upstream sync added (`2963b65a2`)
    rationale: report Changes › Crates / modules (blitz-tests gains the auto-discovered target `pseudo_element_flow_classification`, `all.rs:54` its `mod` line) and › Coverage of new surfaces (3 tests, both layout modes not asserted by the file); the entity enumerates blitz-tests' covered behaviours by file and names no flow-classification check
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:87
note: the 147 → 148 file and 146 → 147 target counts are test-plan's (`bt-files`); F1 (`:open`), F2 (the pseudo vote) and F9 (the tab lens) belong to established-decisions; the second test-only `TMPDIR` read in blitz-vibey-script `tests/dom.rs` belongs to occupied-resources.
