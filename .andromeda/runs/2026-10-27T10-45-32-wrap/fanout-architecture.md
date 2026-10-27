proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables
    change: replace "`TMPDIR`, in one unit test only" with "`TMPDIR`, through `std::env::temp_dir()` at two test-only sites — escher-driver's unix-only client unit test and blitz-vibey-script's `tests/dom.rs` `temp_dir_with_files` — test code only"
    sidecar: Occupied Resources → Environment variables — the test-only `TMPDIR` read moves from one site to two (blitz-vibey-script `tests/dom.rs` `temp_dir_with_files`, taken with `cee6b8731`), test code only, not secret
    rationale: the report's Symbols / APIs bullet names one new test-only env read, `TMPDIR` through `std::env::temp_dir()` in `packages/blitz-vibey-script/tests/dom.rs` `temp_dir_with_files` (860-869 @861), a second test-only site beside escher-driver's client unit test; Counts `tmpdir` lists `architecture.md:216` "in one unit test only" as changed
    basis: .andromeda/architecture.md:216
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → JavaScript (blitz-vibey-script)
    change: add after the `ScriptFetcher` clause — "the public `document_may_load(document_url, url)` lets a `file:` URL load only for a `file:` document, its refusal a `FetchError` variant, checked by `fetch()`, a module `import` and `<script src>` (a narrowing of the `file:`-and-`data:` fetcher; same-origin and CORS still absent)"
    sidecar: Standard Contracts → JavaScript — registers blitz-vibey-script's new public `document_may_load` and its `FetchError` refusal (upstream `cee6b8731`, taken 2026-10-27 on the founder's «Одиннадцать коммитов (Рекомендую)»): a `file:` URL loads only for a `file:` document
    rationale: the report's Symbols / APIs bullet lands `pub fn document_may_load(document_url: Option<&Url>, url: &Url) -> bool` (`packages/blitz-vibey-script/src/fetch.rs:40-46`) and a `FetchError` variant (`fetch.rs:10-12`, `:23`, re-exported `lib.rs:43`) checked by `fetch()`, module `import` and `<script src>`; no architecture section names `document_may_load` (Counts `fetcher`: `architecture.md:173` read at P2 for the rule); Decisions & corrections record it as a narrowing
    basis: .andromeda/architecture.md:173
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Nodes and widgets
    change: add — "the public `ElementData::flush_open_state`, run at an element's construction and on an `open` attribute write or removal through `DocumentMutator`, sets `ElementState::OPEN` on a `details` or `dialog` carrying `open`, so `:open` matches it"
    sidecar: Standard Contracts → Nodes and widgets — registers blitz-dom's new public `ElementData::flush_open_state` (upstream, taken 2026-10-27 on the founder's word): a `details` or `dialog` carrying `open` holds `ElementState::OPEN` and `:open` matches, an engine change for every document
    rationale: the report's Symbols / APIs bullet lands `ElementData::flush_open_state` (`packages/blitz-dom/src/node/element.rs:543-549`, new, pub), called from `ElementData::new` (`:488-489`) and from `DocumentMutator::set_attribute` / `remove_attribute` (`packages/blitz-dom/src/mutator.rs:368-371`, `:520-523`), with `NonTSPseudoClass::Open` reading that state (`packages/blitz-dom/src/stylo.rs:510`); Counts `open` says no master states `:open` — a new fact; no architecture section names the method
    basis: packages/blitz-dom/src/node/element.rs:543-549
  - detector: D-arch-decisions
    severity: warning
    section: §Project Intent
    change: in the `Upstream sync:` line replace "upstream `main` last read at `f9cbb2756`, thirteen of its commits taken by no-commit cherry-picks with the merge base unmoved" with "upstream `main` last read at `ebacd68bd`, twenty-four of the forty commits past the merge base taken by no-commit cherry-picks with the merge base unmoved — thirteen at the 2026-10-24 take, eleven at the 2026-10-27 one, sixteen left out"; the waiting Stylo, Taffy and Dioxus clause unchanged
    sidecar: Project Intent → Upstream sync — upstream `main` read at `ebacd68bd`; eleven more commits taken by no-commit cherry-pick on the founder's «Одиннадцать коммитов (Рекомендую)», merge base `23b9b26a9` unmoved, sixteen left out; pins unchanged
    rationale: the report's Cross-project bullet reads upstream `main` at `ebacd68bd`, merge base `23b9b26a9` unmoved, eleven taken and sixteen left out (`evidence/lock.md`), the chunk line "27 of 40 past the merge base not yet in"; Counts `upstream` lists `architecture.md:287` as changed (a "take read at" reading); Dependencies none, so the Stylo 0.22.0 · taffy 0.15 · Dioxus 0.7.3 clause stands
    basis: .andromeda/architecture.md:287
note: no locked decision is contradicted and no library, runtime or crate enters (Dependencies none, Crates none); the body's [Unsupported features], [Shell and windowing], [JavaScript] bullets and §Infrastructure Patterns → Shell loop / JavaScript runtime stay true — the `:open` match, the pseudo vote, IME-on-construct, the redraw route, the runtime's module split and the `CSSStyleDeclaration` accessors (Counts `cssdecl`, a new fact) are new detail for existing category files (established-decisions, standard-contracts), the cascade's; the `report_js_error` and moved-file rows are the citation sweep's.
