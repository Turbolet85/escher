proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 13
    section: "## Input Validation"
    change: "Append to the end of the How line, after its citation: ; since 2026-10-24-upstream-sync-polish-and-ship the root example `multi_doc` reads its `MULTI_DOC_N` environment value the same way, `parse().ok()` with a default fallback (examples/multi_doc.rs:55-58)"
    sidecar: "input-validation#1.2: multi_doc's MULTI_DOC_N added to the examples' parse-with-default row"
    rationale: "The report's Changes → Symbols lists examples/multi_doc.rs as new, reading MULTI_DOC_N at 55-58 and parsing it with a default fallback; its Coverage of new surfaces marks that validation as present (parse().ok() with a default). A new external-input read in an example belongs on this row, and it is validated, so no detector invariant is broken."
    basis: "examples/multi_doc.rs:55-58"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 245
    section: "## Input Validation"
    change: "Append to the end of the How line, after its citation: ; since 2026-10-24-upstream-sync-polish-and-ship the ratio also coerces a replaced element's natural height in `compute_replaced_layout`, with `is_usable_ratio` (finite and > 0) and `.max(0.0)` kept and its signature unchanged. This is an engine change for every document, ratified by the founder «Десять коммитов (Рекомендую)» (2026-10-24) (packages/blitz-dom/src/layout/replaced.rs:201-211)"
    sidecar: "input-validation#1.67: the aspect-ratio guard now also covers the natural-height coercion upstream #1246 added"
    rationale: "The report's Changes → Symbols says compute_replaced_layout (replaced.rs added 144-145 and 201-211) coerces a replaced element's natural height through its aspect-ratio, keeping is_usable_ratio (finite and > 0) and .max(0.0). Its engine-change bullet says this is ratified for every document by the founder's word (inputs#I4). The row says degenerate ratios are discarded before use, but it never names this new use, which lies outside the range it cites."
    basis: "packages/blitz-dom/src/layout/replaced.rs:201-211"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 350
    section: "## Input Validation"
    change: "Append to the end of the How line, after its citation: ; since 2026-10-24-upstream-sync-polish-and-ship the reads attr_test.rs gains include a `get_node(node_id).unwrap()`, a panic path in the upstream-only runner (wpt/runner/src/test_runners/attr_test.rs, the reads added at 3 · 97 · 135-136 · 141-147 · 164-165)"
    sidecar: "input-validation#1.102: the WPT runner's new get_node(node_id).unwrap() in attr_test.rs recorded"
    rationale: "The report's Changes → Symbols (WPT runner) says attr_test.rs gains reads at added 3 · 97 · 135-136 · 141-147 · 164-165, including a get_node(node_id).unwrap() on an upstream-only path. Its Expected amendments name that unwrap under security-plan §Input Validation and §Error Handling, and this row is the picked carrier for the runner's checkLayout reads."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 20
    section: "## Dependency Security"
    change: "Add a new record after this line: - The 2026-10-24 upstream sync took ten commits, `28f848d3b` · `9eeebb302` · `e4ca5b9b5` · `46d7cf993` · `fb297a38c` · `50bf2f235` · `f8723f495` · `21edc92d9` · `f102663b8` · `f9cbb2756`, by no-commit cherry-picks of upstream's text (`34f6f051b`), the merge base staying `23b9b26a9`. No package enters or leaves `Cargo.lock` (987 → 987). tokio moves 1.53.2 → 1.54.1 in the lock alone (`cargo update -p tokio`), past RUSTSEC-2026-0386; the manifest's `tokio = \"1.42\"` is unchanged. Twelve edges are re-pointed by cargo to versions already locked: eleven `windows-sys`, plus `cssparser-macros 0.7.1`'s `syn` 3.0.6 → 2.0.119. mimalloc 0.1.52 / libmimalloc-sys 0.1.49, already locked, become `apps/browser`'s default global allocator and a direct dependency of the WPT runner (`mimalloc = \"0.1.48\"`, one added lock edge). They were hand-reviewed: MIT; repository `purpleprotocol/mimalloc_rust`; libmimalloc-sys builds vendored C through `cc`; neither is a proc-macro; they reach only upstream-only binaries. `Cargo.toml` and `deny.toml` are byte-identical, and the pins stay: Stylo 0.22.0, Taffy the registry \"0.15\", Dioxus 0.7.3, no git dependency. Not taken: upstream's six Taffy-main commits (`b8f4ba6fd` · `9babd796a`, git Taffy `802e497b…`, and the four `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`), git Stylo 0.23-pre (`06ecec886` with `3954ecb85` and `1c5265a8b`) and Dioxus 0.8.0-alpha.3 (`286874cc1`). The audit leg reads `advisories ok` and `licenses ok` with no ignore added. The take was ratified by the founder «Десять коммитов (Рекомендую)» (2026-10-24) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/lock.md"
    sidecar: "dependency-security: the 2026-10-24 upstream sync's lock record added (tokio 1.54.1 past RUSTSEC-2026-0386, mimalloc edge, six Taffy-main commits waiting)"
    rationale: "The report's Changes → Dependencies and Cross-project claims (the ten shas, the lock 987 → 987, tokio, the twelve edges, the mimalloc hand review, the pins, the six left-out commits). Its Expected amendments call this 'the 2026-10-24 sync record in dependency-security.md — a new record', with Taffy-main 'four → six'. The earlier records #2.4 and #2.5 are dated measurements, true at their syncs, so the six stand in this new record. mimalloc is hand-reviewed and no ban list exists, so D-security-deps is not violated."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 234
    section: "## Dependency Security"
    change: "Replace the line with: - Each upstream sync took upstream's dependency set whole or, by cherry-pick, entered no package. The 2026-10-24 sync's ten picks moved lock edges alone (one mimalloc edge added, twelve re-pointed to versions already locked), beside a lock-only tokio update past RUSTSEC-2026-0386. Each sync's lock was read name by name, against upstream's or against the tree before it, and the audit leg was green after it — one of them only with the zbus-family ignores (Audit tool)."
    sidecar: "security-plan body Pinning: 'by cherry-pick, moved none' no longer holds for every sync; reworded for the 2026-10-24 sync's edge moves"
    rationale: "The report's Changes → Dependencies: no package enters or leaves Cargo.lock, but the picks re-point twelve edges and add one mimalloc edge (the WPT runner), and tokio moves 1.53.2 → 1.54.1 by cargo update -p tokio. The body's 'by cherry-pick, moved none' is the broad line the dependency-security records detail, and it is now false for this sync."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/secret-management.md
    near: 22
    section: "## Secret Management"
    change: "Append to the end of the line: ; outside that searched set, the root example `examples/multi_doc.rs`, new since 2026-10-24-upstream-sync-polish-and-ship, reads `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT` through two `std::env::var` call sites (examples/multi_doc.rs:17-19; examples/multi_doc.rs:55-58), none of them secret-bearing (the body's Environment values read)"
    sidecar: "secret-management#2.2: multi_doc's three non-secret env reads noted beside the examples' absence search"
    rationale: "The report's Changes → Symbols says examples/multi_doc.rs is new and reads three env variables through two std::env::var call sites (env_flag at 17-19, MULTI_DOC_N at 55-58). Its Expected amendments register them under Environment values read. The dated absence search does not cover the new file. No identity, token or key is touched, so D-security-auth is not engaged."
    basis: "examples/multi_doc.rs:17-19"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 15
    section: "## Logging & Monitoring"
    change: "Insert after the parenthetical that holds the citations for the binary's own one eprintln!, before ', and a logging framework is observed absent there': ; the root example `multi_doc`, new since 2026-10-24-upstream-sync-polish-and-ship, writes its output to stdout with one `println!` (examples/multi_doc.rs:111-115)"
    sidecar: "logging-monitoring#1.2: multi_doc's one println! added to the examples' stdout output"
    rationale: "The report's Changes → Symbols says examples/multi_doc.rs has one println! (111-115). Its Coverage of new surfaces calls that println! example output on stdout (instrumentation n/a), which this examples-stdout row records."
    basis: "examples/multi_doc.rs:111-115"
note: "Left to the citation sweep (cites.py apply, report Expected amendments): line digits in the not-touched entities, and the layout/inline.rs → inline/{build,layout}.rs path moves. Left to the body's doc agent: registering MULTI_DOC_* under Environment values read (security-plan.md:294), multi_doc's unknown-mode panic! under Error Handling's Apps and examples, and hit testing's skip of dropped nodes under Graceful degradation."
