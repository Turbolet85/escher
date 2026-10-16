
## 2026-10-16-file-splits — the moved unit tests counted by their new files; escher-driver's 73 (§1, §2, §3, §4)
**Section:** §1 Coverage scope (blitz-dom · escher-driver) · §2 Directory pattern · §4 Test file location · §4 What unit tests cover (escher-driver) · §3 → Session lifecycle
**Change:**
- §1 blitz-dom: was «18 in document.rs»; now 18 `document::` tests — 9 in document.rs, 7 in document_changed_set_tests.rs (`document::changed_set_tests`) and 2 in document_font_reading_tests.rs (`document::font_reading_tests`); the 35 total unchanged.
- §1 escher-driver: was «73 inline unit tests», «schema.rs 6», «mcp.rs 11»; now 73 unit tests in thirteen files, 56 inline and 17 in two `#[path]` test files — schema_tests.rs 6 (`schema::tests`), mcp_tests.rs 11 (`mcp::tests`), each in its old file until this chunk.
- §2 Directory pattern and §4 Test file location: was one `cfg(test)` module out of line (`bridge_tests`); now five — `bridge_tests`, `document::changed_set_tests`, `document::font_reading_tests`, `mcp::tests`, `schema::tests` — each declared `#[cfg(test)]` with `#[path]` at its parent, moved out whole with test paths unchanged.
- §4 escher-driver: schema_tests.rs pins the verb table; mcp_tests.rs that `help` is answered by the server.
- §3 → Session lifecycle: was «escher-driver's 72 unit tests (… client.rs 2 … mcp.rs 11 … schema.rs 6 …)»; now 73 (client.rs 3, mcp_tests.rs 11, schema_tests.rs 6), «72 until 2026-10-14-quality-gates» in its history, its basis the 73 and client.rs's third measured at 2026-10-14-quality-gates and again at this chunk.
**Why:** the per-file counts follow the moved tests to their sibling files, and the key file's 72 — standing since 2026-10-14-quality-gates while §1 read 73 — is amended to the measured 73, both on the operator's directive at this wrap's resume (the operator, 2026-10-16).
**Ref:** .andromeda/runs/2026-10-16T17-27-46-wrap/
