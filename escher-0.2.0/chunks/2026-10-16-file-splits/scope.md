# Scope — 2026-10-16-file-splits

**Working entry (escher-0.2.0/working-route.md:13, verbatim):** File splits — escher's inline test modules out of
`document.rs`, upstream body untouched; `mcp.rs`, `schema.rs` along their seams, spans unchanged (v020-01; audit M2, M3)

**Epoch:** Epoch 1 — Foundation (second of three entries). **Capability:** v020-01 · Codebase structure — its second
leg; the first leg (the handler splits, `parse_float` in steps) shipped in 2026-10-16-handler-splits-and-readme, whose
P5 left the cap pooled for this entry.

## What it builds
A structural refactor with no behaviour change, in three files:
1. **`packages/blitz-dom/src/document.rs`** (audit M2, `sizes.file_max`) — escher's inline `#[cfg(test)]` test modules
   move out of the file into a sibling test location, and upstream's `BaseDocument` body is left byte-for-byte as it is,
   so later upstream syncs stay mostly additive.
2. **`packages/escher-driver/src/mcp.rs`** (audit M3, `sizes.over_800`, 1,140 code lines) — split along its existing
   seams into submodules, each file under the 800-code-line line.
3. **`packages/escher-driver/src/schema.rs`** (audit M3, 840 code lines) — split along its seams, its file under the
   800-code-line line.

The command and tool-call spans stay unchanged (`command` under `escher_driver` with its eight fields; `tool_call` with
`tool` and `ending`), as does every public path the crate exports, every wire byte, every MCP result, and every check
(the stand checks included), which stays green.

## Boundaries
- No behaviour change anywhere: no new verb, field, cause, span field, carrier, listener, env read or dependency.
- Upstream's own body in `document.rs` is not split or edited. The audit names splitting upstream-owned files
  (`node.rs`, `mutator.rs`, `runtime.rs`, …) as upstream's call, so they are out of scope.
- The six timeout-only mutants are the next entry («Fast-failing mutants», v020-02), not this one.
- No route, master, spec-source or matrix edit beyond phase's own promotion and the claim P5 takes on the word.

## Surfaces and contracts it touches
- blitz-dom's test layout (inline modules → a sibling test file or a `blitz-tests` target). Note the blitz-tests target
  pin in `.github/scripts/test_blitz_tests_targets.py` if a new target is added.
- escher-driver's module tree (`lib.rs` `mod` lines); the `pub` re-exports `serve_mcp`, `VERBS`, `NODE_FIELDS`,
  `validate`, `validate_session`, … unchanged.
- The CLAUDE.md module and pointer-table rows and `.claude/docs/services/escher-driver.md` name files by path, so the
  wrap's reconcile re-points them.

## Scope bullets for closure
- [premise-corrected: six modules from line 2897, two of them escher's — `git log -S` and `upstream/main`] escher's
  inline test modules in `document.rs` are `changed_set_tests` (3462–3694, added in 2026-10-07-change-tracking-and-diff,
  `fecb6f19d`) and `font_reading_tests` (3695–3719, added in 2026-10-12-headless-screenshot, `5173f7e12`). Neither one
  exists in `upstream/main`. The other four (`zoom_tests` #784 · `hover_state_tests` #546 · `hover_invalidation_tests` #765
  · `font_face_override_tests` #433) are upstream's modules. `hover_state_tests` carries escher's
  2026-10-14-quality-gates edits (the two-argument `qual_name!` fixtures, `assert_text_has_size`), but the module itself is
  upstream's, so it stays where it is, as part of upstream's body. The audit's list of five predates `font_reading_tests`.
- [premise-corrected: tokei on the file without the two modules reads 2,567] Moving escher's two modules out takes
  `document.rs` from 2,793 to about 2,567 code lines (`changed_set_tests` 204, `font_reading_tests` 22). The audit's
  2,105 is the figure without all of its test modules, upstream's included. The file stays the largest and over 800.
  v020-01's acceptance asks only that "escher's inline test modules [are] gone from the engine's document file with
  upstream's body unchanged".
- [verified: precedent `packages/dioxus-native-dom/src/dioxus_document.rs:781-783`] Each moved module stays a
  `#[cfg(test)]` child of `document`, declared through `#[path]` beside the file, as dioxus-native-dom's `bridge_tests`
  is. It keeps its `use super::*;`, its test path (`document::changed_set_tests::…`) and its access to private items. It
  does not become a blitz-tests target, so the target pin and the census stay as they are.
- [verified for `mcp.rs`: the outline at `mcp.rs:119-705`; premise-corrected for `schema.rs`: help texts are fields of
  each table row] `mcp.rs`'s seams stand in the code as written: framing and bounds (119–326), tool listing (327–417),
  tool-result writing (418–537), with the server (538–705) and `serve_mcp` (89–118) beside them. Its inline tests
  (706–1324, 11 tests over shared helpers) drive `serve_mcp` end to end. `schema.rs` has no help-text seam: every help
  string is a field of its `VerbSpec` / `FieldSpec` / `NodeFieldSpec` row, so cutting the help out would re-shape the
  table. The seam it has is its shape types (112–259) apart from the table written in them (1–111, 260–622), plus its
  tests (623–962). Moving the tests alone already brings both files under the line (`mcp.rs` about 561 code lines,
  `schema.rs` about 513). Whether the production seams are split too is P4's fork.
- [verified: tokei 14.0.0 reproduces 2,793 · 1,140 · 840 at HEAD] The 800-line measure is tokei code lines, the audit's
  own instrument. The acceptance re-reads it with the same tool.
- CI read at take-up (Setup 5a): `4f3939914e52` (the last wrap's flip, HEAD) is green, checks 17/17, wall 839 s, run
  CI#38922661070. Nothing is red, so nothing is folded.
