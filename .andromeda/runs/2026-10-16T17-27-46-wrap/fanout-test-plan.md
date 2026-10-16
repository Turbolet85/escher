proposals:
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Directory pattern"
    change: "Inline `#[cfg(test)]` modules in source files, bar five `cfg(test)` modules held in files of their own through `#[path]` — `dioxus_document::bridge_tests` (dioxus_document.rs:780-782) and, since 2026-10-16-file-splits, `document::changed_set_tests` and `document::font_reading_tests` (packages/blitz-dom/src/document.rs:3462-3464; packages/blitz-dom/src/document.rs:3466-3468), `mcp::tests` (packages/escher-driver/src/mcp.rs:285-287) and `schema::tests` (packages/escher-driver/src/schema.rs:477-479); the rest of the line unchanged."
    sidecar: "test-plan §2 Directory pattern: out-of-line `#[path]` test modules one → five (document's two, mcp::tests, schema::tests beside bridge_tests), 2026-10-16-file-splits."
    rationale: "Report Changes → Crates / modules: four new `cfg(test)` children held in files of their own through `#[path]`, the workspace's out-of-line `#[path]` test modules going from one (`dioxus_document::bridge_tests`) to five; Expected amendments name test-plan.md:66 (§2, «bar one `cfg(test)` module held in a file of its own through `#[path]`») → changed."
    basis: "escher-0.2.0/chunks/2026-10-16-file-splits/report.md:105-107; report.md:180-182; .andromeda/test-plan.md:66"
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → Conventions → Test file location"
    change: "Five unit-test modules sit out of line, each declared `#[cfg(test)]` with `#[path]` at its parent: `dioxus_document::bridge_tests` in `dioxus_document_tests.rs` (a child because `DioxusEventHandler`'s fields are private, as now) and, since 2026-10-16-file-splits — moved out whole, test paths unchanged, to keep the parent files under the size ceiling — `document::changed_set_tests` in `document_changed_set_tests.rs` and `document::font_reading_tests` in `document_font_reading_tests.rs` (packages/blitz-dom/src/document.rs:3462-3464; packages/blitz-dom/src/document.rs:3466-3468), `mcp::tests` in `mcp_tests.rs` (packages/escher-driver/src/mcp.rs:285-287) and `schema::tests` in `schema_tests.rs` (packages/escher-driver/src/schema.rs:477-479); replaces «one unit-test module sits out of line»."
    sidecar: "test-plan §4 Test file location: «one unit-test module sits out of line» → five, the four moved by 2026-10-16-file-splits named with their declarations."
    rationale: "Report Changes → Crates / modules (four new `#[path]` cfg(test) children, `-- --list` outputs cmp-equal so test paths unchanged) and Outcome (3) (driver files ≤ 800 tokei code lines); Expected amendments name test-plan.md:137 (§4, «one unit-test module sits out of line») → changed."
    basis: "escher-0.2.0/chunks/2026-10-16-file-splits/report.md:94-95; report.md:105-109; report.md:183; .andromeda/test-plan.md:137"
    dependent-of: D-tests-framework
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom (document, mutator, net, form)"
    change: "«18 in document.rs» → «18 `document::` tests — 9 in document.rs and, since 2026-10-16-file-splits, 9 in its two `#[path]` sibling files: `document::changed_set_tests` 7 in document_changed_set_tests.rs and `document::font_reading_tests` 2 in document_font_reading_tests.rs (packages/blitz-dom/src/document_changed_set_tests.rs:1-235; packages/blitz-dom/src/document_font_reading_tests.rs:1-26)»; the 35 total and the other files' counts unchanged."
    sidecar: "test-plan §1 blitz-dom: document.rs's 18 tests restated by file — 9 in document.rs, 7 + 2 in its two sibling test files (2026-10-16-file-splits, operator inputs#I3)."
    rationale: "Report Counts / qualifiers moved: blitz-dom `document::` 18 (changed_set 7, font_reading 2), 9 of document.rs's 18 now standing in its two sibling files, site test-plan.md:11 «18 in document.rs» listed; the operator's directive inputs#I3 (Addendum): the per-file test counts follow the moved tests to their sibling files."
    basis: "escher-0.2.0/chunks/2026-10-16-file-splits/report.md:117-125; report.md:348-354; .andromeda/test-plan.md:11"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: "«73 inline unit tests in thirteen files» → «73 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 17 in two `#[path]` test files»; «schema.rs 6» → «schema_tests.rs 6 (`schema::tests`)»; «mcp.rs 11» → «mcp_tests.rs 11 (`mcp::tests`)»; the dated history clauses («mcp.rs's eleventh», «mcp.rs's nine») and the test descriptions unchanged."
    sidecar: "test-plan §1 escher-driver: schema.rs 6 and mcp.rs 11 now stand in schema_tests.rs and mcp_tests.rs, 17 of the 73 out of line (2026-10-16-file-splits, operator inputs#I3)."
    rationale: "Report Counts / qualifiers moved: mcp.rs's 11 and schema.rs's 6 now stand in mcp_tests.rs and schema_tests.rs as `mcp::tests` and `schema::tests`, escher-driver still 73; site test-plan.md:21 «schema.rs 6», «mcp.rs 11» listed; operator directive inputs#I3 moves the per-file counts to the sibling files."
    basis: "escher-0.2.0/chunks/2026-10-16-file-splits/report.md:117-126; report.md:348-354; .andromeda/test-plan.md:21"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "«schema.rs pins the verb table» → «schema_tests.rs (`schema::tests`) pins the verb table»; «mcp.rs that `help` is answered by the server» → «mcp_tests.rs (`mcp::tests`) that `help` is answered by the server»; nothing else in the row changes."
    sidecar: "test-plan §4 escher-driver: the schema and mcp unit-test claims name schema_tests.rs and mcp_tests.rs, where the tests now stand (2026-10-16-file-splits)."
    rationale: "Report Counts / qualifiers moved lists test-plan.md:143 «schema.rs pins the verb table», «mcp.rs that `help` is answered» as a by-file site of the moved tests; operator directive inputs#I3."
    basis: "escher-0.2.0/chunks/2026-10-16-file-splits/report.md:124-127; .andromeda/test-plan.md:143"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-proof: «escher-driver's 72 unit tests (cli.rs 5 · client.rs 2 · … · mcp.rs 11 · … · schema.rs 6 · …; 66 until …)» → «escher-driver's 73 unit tests (cli.rs 5 · client.rs 3 · command.rs 7 · describe.rs 2 · error.rs 5 · execute.rs 3 · image.rs 5 · json.rs 7 · mcp_tests.rs 11 · refusal.rs 3 · schema_tests.rs 6 · session.rs 4 · wire.rs 12; 72 until 2026-10-14-quality-gates, 66 until …)», adding to its basis the count of 73 and client.rs's third at escher-0.1.0/chunks/2026-10-14-quality-gates/report.md, measured again at 73 by 2026-10-16-file-splits."
    sidecar: "test-plan §3 Session lifecycle (session-proof): escher-driver's unit tests 72 → the measured 73 (client.rs 3), mcp and schema counts under mcp_tests.rs and schema_tests.rs (2026-10-16-file-splits, operator inputs#I3)."
    rationale: "Report Spec claims disproved by measurement: the key file session-lifecycle.md:11 reads «escher-driver's 72 unit tests (cli.rs 5 · client.rs 2 · …)», measured 73 at HEAD before and after the split (`cargo test -p escher-driver --locked` → 73 passed); Counts / qualifiers moved lists the same key's «mcp.rs 11 · … · schema.rs 6» (pattern bare-file); operator directive inputs#I3 amends the 72 to the measured 73 and moves the per-file counts."
    basis: "escher-0.2.0/chunks/2026-10-16-file-splits/report.md:128-129; report.md:155-164; report.md:348-356; .andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
    dependent-of: D-tests-coverage
note: "D-tests-obs-harness holds (report: Harness / gate surface none, no harness, status or log-format change). D-tests-coverage holds on its letter — no new code path, the moved code keeps its unit tests (73 · 18, `-- --list` cmp-equal); its four proposals carry the by-file count sites the report lists and the operator's inputs#I3. The P2 citation sweep already re-pointed test-plan's mcp/schema/document cites (lines 11, 12, 21, 103, 143 read the new paths). No-change rows per the report: test-plan.md:24, :99 (bridge module, true), :331 (dated history), :55 (`font_reading_tests` names the module, still true). The leaf `.claude/docs/services/escher-driver.md:60` holds the same 72 — the cascade's."
