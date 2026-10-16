# Codebase Research — 2026-10-16-file-splits

## Scope
- **Depth:** moderate · **Reads:** 12 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — 1 addition read (the `timeout` exit-124 rule; no
  bounded boot smoke is planned), none applied · `.claude/rules/testing.md` — 8 additions read; applied: 2026-10-07
  (`qual_name!("div", html)` — the moved modules keep their fixtures as written), 2026-10-09 (count result lines under
  `--no-fail-fast` only), 2026-10-10 (search `tests/` and `examples/*/tests/` before calling a check unaffected — done,
  §Sweeps)
- **Platform issues consulted:** none — no runner-only bullet (CI on `4f3939914e52` green 17/17) and no CI-reading
  entry outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `packages/blitz-dom/src/document.rs` (2890–3719, and the diff against the upstream merge base `434e0661`) — six inline
  `#[cfg(test)]` modules, from line 2897: `zoom_tests` 2897 · `hover_state_tests` 2933 · `hover_invalidation_tests` 3023
  · `font_face_override_tests` 3389 · `changed_set_tests` 3462–3694 · `font_reading_tests` 3695–3719 (re-derived:
  `grep -n '^mod ' document.rs`, line > 2800). 18 `#[test]` functions in all, 7 in `changed_set_tests` and 2 in
  `font_reading_tests` (re-derived: `#[test]` counts over each span).
- `packages/escher-driver/src/mcp.rs` (full outline; 1,324 raw lines) — the top-level items by line: consts and the
  `START_APP` table 30–88 · `serve_mcp` 89–118 · framing and bounds 119–326 (`io_error`, `Request`, `read_request`,
  `Id`, `ProtocolError`, `Envelope`, `envelope`, `reply`, `result_reply`, `error_reply`, `write_line`, `list`,
  `strings`) · listing 327–417 (`initialized`, `description`, `tool`, `tool_list`) · tool results 418–537 (`call_of`,
  `Ending`, `ToolResult`) · the server 538–705 (`tool_call_span`, `Held`, `Server`) · `mod tests` 706–1324 (11
  `#[test]`, with the shared helpers `never` · `empty` · `exchange` · `request` · `tool_call` · `error` · `tool_reply` ·
  `lines` · `listed`).
- `packages/escher-driver/src/schema.rs` (full outline; 962 raw lines) — bounds and key/busy tables 1–30 ·
  `NodeFieldSpec` + `NODE_FIELDS` 32–111 · the shape types 112–259 (`ArgKind`, `ArgSpec`, `FieldKind`, `FieldSpec`,
  `Level`, `VerbSpec`) · the result-field consts and the eleven verb consts 260–572 · `VERBS` 573 · `ExitStatus` +
  `EXIT_STATUSES` 579–617 · `verb()` 619 · `mod tests` 623–962 (6 `#[test]`).
- `packages/escher-driver/src/lib.rs` (98–129) — the fourteen private `mod` lines and the `pub use` block; no change
  needed when the module's file path changes.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (775–783) and `dioxus_document_tests.rs` (1–12) — the one
  out-of-line test precedent: `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;` at the end of the
  parent, the file opening with a `//!` that names why it is a child module.
- `scripts/code-graph-cookbook.md` — read whole before the query.

## Graph impact (from the code-graph query; trace `tree-query-2026-10-16-file-splits.json`)
- **Into `mcp.rs` from outside it** — only `serve_mcp` (fn): `cli.rs` 2 sites, `lib.rs` 1, `stand_mcp_masked.rs` 2,
  `stand_mcp_tools.rs` 4. Every other item in `mcp.rs` is private to the file, so splitting it inside the module has no
  caller outside `mcp`.
- **Into `schema.rs` from outside it** — 40 distinct names (types, consts, fields, `verb`), reached from `cli.rs`,
  `command.rs`, `describe.rs`, `execute.rs`, `json.rs`, `session.rs`, `wire.rs`, `lib.rs` and the checks
  `stand_act_spans` · `stand_act_refused` · `stand_act_scroll` · `cli_commands` · `mcp_server`. Among them are the
  crate-private verb consts (`CLICK`, `START`, …) through `crate::schema::NAME`. So any split of `schema.rs` keeps every
  `crate::schema::NAME` and every crate-root re-export path resolving, by re-export from `schema.rs`.

## Patterns detected
- **Out-of-line test module as a `#[path]` child** (`packages/dioxus-native-dom/src/dioxus_document.rs:781-783`): the
  module stays a child of the module under test, so `use super::*` reaches private items and no `pub`, constructor or
  shim is added. The test path (`dioxus_document::bridge_tests::…`) is the module's own.
- **One hand-built span, explicit target** (`packages/escher-driver/src/mcp.rs:539`):
  `tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)`. The target is explicit, so
  moving the function between files leaves the printed line's target as it reads; `command`'s site
  (`execute.rs:86`) is untouched by this chunk.
- **Child modules reach a parent's private `use` imports by glob**: `mod tests { use super::*; … }` in both driver
  files already reaches the parent's imports (`Session`, `SessionError`, `Hello`, …), so a moved test file keeps
  `use super::*;` as its first import and needs no new path.

## Conventions to follow
- **Module docs**: every file opens with `//!` (`mcp.rs:1-8`, `schema.rs:1-2`, `dioxus_document_tests.rs:1-5`); a moved
  test file's `//!` says what it guards and why it is a child module (precedent `dioxus_document_tests.rs:1-5`).
- **Probe readings at HEAD, the baseline the split keeps**: over `mcp.rs` 1–705 and `schema.rs` 1–622 (outside
  `#[cfg(test)]`), `unwrap(` · `expect(` · `panic!` · `unreachable!` · `println!` · `eprintln!` · `print!` · `dbg!` ·
  `std::env` · `env::var` read 0 hits, and `cfg(` · `serde` read 0 in `schema.rs` (re-derived: `sed -n` the span |
  `grep -nE` the alternation). `is_terminal|IsTerminal|owo|colored` reads 0 over `packages/escher-driver/src`; the
  three `\u{1b}` hits are assertions in tests (`json.rs:439`, `describe.rs:154`, `cli.rs:757`).
- **Sizes by the audit's instrument** (tokei 14.0.0, code lines): at HEAD `document.rs` 2,793 · `mcp.rs` 1,140 ·
  `schema.rs` 840 — the audit's figures reproduce. On scratch copies of the spans: `document.rs` without escher's two
  modules 2,567 (`changed_set_tests` 204, `font_reading_tests` 22); `mcp.rs` head 51 · framing 172 · listing 80 ·
  results 103 · server 155 · tests 579; `schema.rs` bounds and node keys 88 · types 96 · table 329 · tests 327
  (re-derived: `tokei -o json` over `sed -n` copies in the session scratchpad).

## New files to create
- `packages/blitz-dom/src/document_changed_set_tests.rs` — the body of `changed_set_tests`, moved whole
- `packages/blitz-dom/src/document_font_reading_tests.rs` — the body of `font_reading_tests`, moved whole
- `packages/escher-driver/src/mcp_tests.rs` — the body of `mcp.rs`'s `mod tests`, moved whole
- `packages/escher-driver/src/schema_tests.rs` — the body of `schema.rs`'s `mod tests`, moved whole
- `packages/escher-driver/src/mcp/framing.rs` — the seam branch only: the request reader, the id, the protocol errors and the reply writers
- `packages/escher-driver/src/mcp/listing.rs` — the seam branch only: the handshake answer and the tool listing
- `packages/escher-driver/src/mcp/result.rs` — the seam branch only: the call reader, the ending and the bounded tool result
- `packages/escher-driver/src/schema/spec.rs` — the seam branch only: the shape types the table is written in

## Files to modify
- `packages/blitz-dom/src/document.rs` — the two escher modules' inline bodies replaced by two `#[cfg(test)] #[path]` declarations, nothing above them touched
- `packages/escher-driver/src/mcp.rs` — `mod tests` out to its file; in the seam branch, the three seams out to submodules
- `packages/escher-driver/src/schema.rs` — `mod tests` out to its file; in the seam branch, the shape types out to a submodule re-exported from here

## Sweeps
- `document.rs`, `mcp.rs`, `schema.rs` as path text over the tree outside `*.md` and `.andromeda/`: 5 hits · 0 changed ·
  5 no-change (committed evidence and inputs of 0.1.0 chunks, and the two version dirs' `verification-matrix.json`
  text — history, not configuration); 0 hits under `.github/`, `.cargo/`, `scripts/` or any `*.toml` (re-derived:
  `git grep -l` with `':!*.md' ':!.andromeda/*'`, then the scoped `git grep` over those four). No CI leg, lint table or
  mutation config names the files by path.
- `mcp::` / `schema::` paths from other files: the graph query above (40 schema names, 1 mcp name), every path kept by
  re-export or unchanged module — 0 changed.

## Open questions
- `mcp.rs` and `schema.rs`: split along the audit's production seams (framing and bounds · tool listing · tool-result
  writing; for `schema.rs` its shape types apart from its table), each with its tests in one child file — or move only
  their inline tests out, which already brings both files under the 800-code-line line (561 and 513)? → blocks:
  plan-decision. The four `mcp/` and `schema/` files above are the seam branch's; the two `*_tests.rs` files are both
  branches'.
