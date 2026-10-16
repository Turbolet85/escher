# Implement outcome — 2026-10-16-file-splits

**Outcome: green.** The smoke was skipped because no boot path or UI surface changed. The touched `escher-session mcp`
role was run as the built binary by gate entries 6, 7 and 9.

## Gates
- The full block passed on its first firing: `entries 26 · green 23 · red 0 · recorded 0 · timeout 0 · not-run 3`
  (trail `.andromeda/runs/2026-10-16T16-56-12-implement/gate-2026-10-16-file-splits.json`).
- The three not-run entries are the operator legs, 24–26 (hygiene, push, CI read).
- Step 1's pre-edit control: entries 3, 4, 6 and 7 fired on the untouched tree before any edit,
  `entries 26 · green 4 · red 0 · … · not-run 22`, in the same trail.
- Readings from the exit-only entries' logs:
  - entry 21 (a11y): 6 · 6 · 4 · 8 · 5 · 14 passed;
  - entry 8: stand_accessibility_ids 8 · stand_act_spans 2 (2 ignored) · stand_mcp_masked 1 · stand_mcp_tools 5;
  - entry 9: cli_commands 12 · host_log 1 · mcp_log 1 · mcp_server 5;
  - entry 22 (fast): 186 `test result: ok`, 0 FAILED, and the CI scripts `Ran 100 tests … OK`.
- Deferrals: none. The change is Rust source, and every gate ran.
- Matrix: v020-01 went planned → implemented (`.andromeda/runs/2026-10-16T16-56-12-implement/matrix-2026-10-16-file-splits.json`).
  Its ref names entries 1–23. Its acceptance part (4) CI read is operator entry 26.

## Files changed
- Modified:
  - `packages/blitz-dom/src/document.rs` — lines 1–3461 are byte-identical to the base, followed by two `#[path]`
    declarations;
  - `packages/escher-driver/src/mcp.rs`;
  - `packages/escher-driver/src/schema.rs`.
- New:
  - `packages/blitz-dom/src/document_changed_set_tests.rs`
  - `packages/blitz-dom/src/document_font_reading_tests.rs`
  - `packages/escher-driver/src/mcp/framing.rs`
  - `packages/escher-driver/src/mcp/listing.rs`
  - `packages/escher-driver/src/mcp/result.rs`
  - `packages/escher-driver/src/mcp_tests.rs`
  - `packages/escher-driver/src/schema/spec.rs`
  - `packages/escher-driver/src/schema_tests.rs`
- Evidence: `escher-0.2.0/chunks/2026-10-16-file-splits/evidence/`
  - the five step-1 records: `mcp-requests.jsonl`, `help-before.json`, `mcp-before.jsonl`, `driver-tests-before.txt`,
    `dom-tests-before.txt`;
  - `sizes-after.md`.
- Scope: `scope: clean — changed 11 · listed 11 · recorded 0`. No `scope-record.md` was needed.

## Deviations from plan, with their justifications
1. **Seven intra-doc links now name their targets by path.**
   - The changes: in `serve_mcp`'s doc, `[`VERBS`]` became `[`VERBS`](schema::VERBS)`. In `schema/spec.rs`,
     `[`MAX_ID_BYTES`]`, `[`MAX_TEXT_BYTES`]`, `[`KEY_NAMES`]`, `[`MAX_MILLISECONDS`]`, `[`BUSY_CLASSES`]` and
     `[`NODE_FIELDS`]` became `[`NAME`](super::NAME)`.
   - Why: after the move their targets are no longer imported in those files. rustc flags an import used only by a doc
     link as `unused_imports`, which would fail clippy `-D warnings`. Without the import, rustdoc `-D warnings` would
     fail on a broken link instead.
   - The rendered text is unchanged. The plan said the items move "unchanged". This is the one textual change inside a
     moved item, and it is a doc-comment link target only.
2. **`mcp_tests.rs` has use lines beyond moved items.**
   - Lines naming moved items, as step 4 allows: `super::framing::{MAX_ID_BYTES, MAX_MCP_REQUEST_BYTES}` and
     `super::listing::{description, tool}`.
   - Lines naming items the parent no longer imports: `crate::command::ArgValue`, `crate::schema::{ArgKind, KEY_NAMES,
     MAX_MILLISECONDS, VERBS}` (merged with the existing `ArgSpec` line) and `crate::wire::MAX_ANSWER_BYTES`.
   - Why: once those imports left `mcp.rs` with the code that used them, the tests' `use super::*` no longer reached
     them. This is the same mechanism the plan's implementation note describes, applied to non-moved names.
   - No test, helper or assertion changed.
3. **rustfmt re-wraps.**
   - `call_of`'s signature wraps after its `pub(super)` prefix.
   - One assertion line in `mcp_tests.rs` (`… == bounded,`) joins after the dedent.
   - `mcp.rs`'s new `use framing::{…}` block is rustfmt's layout.
   - A base-vs-now line diff of every moved span reads identical apart from these re-wraps, deviation 1's links and the
     use lines.
4. **Span edges.**
   - The shape types moved from `schema.rs:110-258`, which includes `ArgKind`'s doc line and derive. The plan cites
     `:112-259`.
   - `changed_set_tests`' body is `document.rs:3464-3692`, with the module at 3462–3693.
   - `font_reading_tests`' body is `:3697-3718`, with the module at 3695–3719.
   - The cut is the plan's: after line 3461.
5. **Visibility.** These items are `pub(super)`, and nothing is `pub` or `pub(crate)`:
   - in framing: `MAX_MCP_REQUEST_BYTES`, `MAX_ID_BYTES`, `Request`, `read_request`, `Id`, `ProtocolError`, `Envelope`,
     `envelope`, `result_reply`, `error_reply`, `write_line`, `list`;
   - in listing: `initialized`, `description`, `tool`, `tool_list`;
   - in result: `call_of`, `Ending` with `word`, `ToolResult` with its `ending` field and its six methods.

   `reply`, `strings`, the three listing consts, `Id`'s methods, `ProtocolError`'s methods and `ToolResult`'s `text`
   and `image` stay private. Of the widened items, `MAX_*_BYTES`, `description` and `tool` are reached only by
   `mcp::tests`.

## Sizes (tokei 14.0.0, code lines; `evidence/sizes-after.md`)
| file | code lines |
|---|---|
| `document.rs` | 2,573 |
| `document_changed_set_tests.rs` | 201 |
| `document_font_reading_tests.rs` | 19 |
| `mcp.rs` | 217 |
| `mcp/framing.rs` | 167 |
| `mcp/listing.rs` | 91 |
| `mcp/result.rs` | 115 |
| `mcp_tests.rs` | 579 |
| `schema.rs` | 421 |
| `schema/spec.rs` | 97 |
| `schema_tests.rs` | 324 |

## Surfaced / stuck
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `target/debug/escher-session help` · `escher-session mcp` (step 1's records, twice for the repeat check) | this run | terminated — each exited on its own; mcp ended on stdin EOF |
| `cargo build` / `cargo test --list` (step 1) | this run | terminated |
| gate entries 1–23 (cargo, the binary, ci-leg.sh) | this run, through gate.py | terminated |

Measured with `ps -eo pid,ppid,comm,args` filtered by command name (`escher-sessio`, `seven_guis_nati`, `cargo`,
`rustc`), which read none at 2026-10-16T17:07Z.

## Decisions & corrections
- A guard refusal: the Bash guard refused `cd ~/.claude/skills/andromeda-implement/references && …`
  (`cd … moves the session cwd`). It was re-issued once with absolute paths and ran.
- A recurrence of a standing learning (2026-10-10, extended 2026-10-15): the per-package `cargo test -p escher-driver`
  warns blitz-paint's `variant Intrinsic is never constructed`. It is outside the change, and the `fast` leg is green.
- A measured fact for the wrap: rustc counts an import used only by intra-doc links as unused. A moved item's doc link
  must therefore name its target by path, or the item must keep a code use.

## Next
The operator pass, entries 24–26 by number:
- 24: `gate.py hygiene`, before the pre-CI commit;
- 25: the fast leg, the clean-tree guard and the push, after `chore(2026-10-16-file-splits): operator pre-CI commit`;
- 26: `ci.py conclusion --sha HEAD --wait 1800`.

Then `/andromeda-wrap-session`.
