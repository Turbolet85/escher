
## 2026-10-16-file-splits — five out-of-line test modules; mcp's and schema's child modules
**Section:** §Conventions (Tests) · §Existing Scopes (blitz-dom · escher-driver)
**Change:**
- Tests: was one unit-test module in a file of its own (dioxus-native-dom's `bridge_tests`); now five, each a `#[cfg(test)]` child declared through `#[path]` at its parent — `bridge_tests` and blitz-dom's `document::changed_set_tests` and `document::font_reading_tests` (`document_changed_set_tests.rs`, `document_font_reading_tests.rs`) and escher-driver's `mcp::tests` and `schema::tests` (`mcp_tests.rs`, `schema_tests.rs`), each moved out whole with its test paths unchanged.
- blitz-dom row: `document` names its two `cfg(test)` children in files of their own.
- escher-driver row: `mcp` has three private children, each widened to `pub(super)` and no further — framing (the request reader, the id, the protocol errors, the reply writers), listing (the handshake and the tool listing), result (a tool call's arguments read into a call, its result and its written form) — and its tests in `mcp_tests.rs`; `schema` holds its six shape types `ArgKind`, `ArgSpec`, `FieldKind`, `FieldSpec`, `Level`, `VerbSpec` in its private child spec, re-exported, and its tests in `schema_tests.rs`. The crate's fourteen private modules stay fourteen.
- Citations into the moved spans follow them to their new files.
**Why:** the file-splits chunk's move along the seams the code has, the split's shape the operator's answer at its P4 fork; no public item, crate module, test path or behaviour changed.
**Kept:** the dioxus-native-dom row's «one `cfg(test)` child module, `bridge_tests`» — true of that module.
**Ref:** .andromeda/runs/2026-10-16T17-27-46-wrap/
