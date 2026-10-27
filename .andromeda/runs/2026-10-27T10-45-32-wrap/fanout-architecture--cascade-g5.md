proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 96
    section: "## Standard Contracts"
    change: "In the Paint and shell entry, right after the clause `BlitzShellProvider` implements `ShellProvider` for redraw, cursor, title, IME, close, minimize, maximize, decorations, drag, clipboard, file dialog (its citation unchanged), add the clause: since 2026-10-27-upstream-sync-engine-debts (upstream `6587e7e79`, taken on the founder's word) the windowed shell's redraw requests travel through its event loop (packages/blitz-shell/src/lib.rs:90; packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/lib.rs:104-106; packages/blitz-shell/src/window.rs:193-194)"
    sidecar: "standard-contracts Paint and shell: the shell provider's redraw requests now travel through the windowed shell's event loop (upstream 6587e7e79, 2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report's Symbols / APIs bullet (report.md:15) states blitz-shell lib.rs:90, :93-98, :104-106 and window.rs:193-194 route the windowed shell's redraw requests through its event loop; the provider's contract clause names redraw but not its route"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 128
    section: "## Standard Contracts"
    change: "Remove line 128, the gap marker `OPEN — the stand's typography stated here rather than on design-system's product surface · owner: F2`, now that the founder has ratified F2 («Подтвердить (Рекомендую)», inputs#I7); the ratification is carried by the sidecar entry, not by the body"
    sidecar: "standard-contracts Headless stand: the OPEN gap marker owned by F2 dropped — F2 ratified by the founder's own word «Подтвердить (Рекомендую)» (inputs#I7, 2026-10-27)"
    rationale: "The report's Decisions & corrections (report.md:103, :105) and Counts stand-prov (report.md:47) name architecture-detail/standard-contracts.md:128 as an OPEN gap marker owned by F2 that drops at P2 under classification-contract rules 6-7"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:105
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 211
    section: "## Standard Contracts"
    change: "In the JavaScript (blitz-vibey-script) entry, end line 211 with `;` and append a continuation line in the entry's clause shape: since 2026-10-27-upstream-sync-engine-debts (upstream `cee6b8731`, taken on the founder's word) the public `document_may_load(document_url: Option<&Url>, url: &Url) -> bool` lets a `file:` URL load only for a `file:` document, a refusal is a further `FetchError` variant re-exported from the crate root, and `fetch()`, a module `import` and a `<script src>` each check it before loading — a narrowing of the default fetcher, which still takes only `file:` and `data:`, same-origin and CORS checks still absent (packages/blitz-vibey-script/src/fetch.rs:10-12; packages/blitz-vibey-script/src/fetch.rs:23; packages/blitz-vibey-script/src/fetch.rs:40-46; packages/blitz-vibey-script/src/lib.rs:43; packages/blitz-vibey-script/src/runtime/fetch.rs:40-44; packages/blitz-vibey-script/src/document.rs:163-169; packages/blitz-vibey-script/tests/dom.rs:904-928; packages/blitz-vibey-script/tests/dom.rs:930-950)"
    sidecar: "standard-contracts JavaScript: document_may_load and its FetchError refusal — a file: URL loads only for a file: document, checked by fetch(), import and script src (upstream cee6b8731, a narrowing)"
    rationale: "Task fact F5; the report's Symbols / APIs (report.md:16), Counts fetcher (report.md:33) and Outcome (report.md:118) state the new public function, its FetchError variant and its three check sites, the fetcher still taking only file: and data:"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:16
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 211
    section: "## Standard Contracts"
    change: "In the JavaScript (blitz-vibey-script) entry, after the F5 continuation line (it ending with `;`), append a continuation line: since 2026-10-27-upstream-sync-engine-debts (upstream `efdc5472a`, taken on the founder's word) CSS properties are accessors on `CSSStyleDeclaration`, their names Stylo's property list — `css_property_names`, read by the bootstrap's `installStyleProperties` through `__blitz_css_property_names` — a realization on the registered script-to-DOM crossing, not a widening (packages/blitz-vibey-script/src/runtime/css.rs:8-18; packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48; packages/blitz-vibey-script/tests/dom.rs:455-487)"
    sidecar: "standard-contracts JavaScript: CSS properties as CSSStyleDeclaration accessors named from Stylo's property list (upstream efdc5472a, a realization on the registered script-to-DOM crossing)"
    rationale: "Task fact F6; the report's Symbols / APIs (report.md:16), Decisions & corrections (report.md:99) and Coverage (report.md:84) state the accessors, their Stylo-named property list and their test style_property_accessors; Counts cssdecl (report.md:50) finds no master stating it"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:16
    fact: F6
note: "Not this group's: F9 (browser tab lenses) is established-decisions'; F7 (the runtime/ and dom/element/ module trees, the nine bootstrap scripts) is existing-scopes' — the JavaScript entry's 'a JS bootstrap string' claim still holds and its moved code is the citation sweep's; ElementData::flush_open_state (pub) and the set_attribute open arm sit in contract-nodes-and-widgets and contract-mutation-query-and-cssom, which no group-5 entity picked, a D-arch-resources check for the new-detail pass"
