# P5 review — 2026-10-16-file-splits · «File splits»

**Plan:** `escher-0.2.0/chunks/2026-10-16-file-splits/plan.md` (425 lines, 225 of them prose) · **research:**
`escher-0.2.0/chunks/2026-10-16-file-splits/research.md` · **scope:** `escher-0.2.0/chunks/2026-10-16-file-splits/scope.md`

## Goal
This is v020-01's second and last leg, from the audit's M2 and M3. It is a pure structural move, with no change to
behaviour, wire bytes, MCP replies, spans or checks:
- **`document.rs`:** escher's two inline test modules, `changed_set_tests` and `font_reading_tests`, move into
  `#[path]` sibling files. Lines 1–3461 stay byte-for-byte as they are (upstream's body and its four test modules).
- **`mcp.rs`:** split along the audit's three seams into `mcp/framing.rs`, `mcp/listing.rs` and `mcp/result.rs`.
  `serve_mcp`, the server and the `tool_call` span stay in `mcp.rs`, and the 11 tests move whole to `mcp_tests.rs`.
- **`schema.rs`:** the table stays. The six shape types move to `schema/spec.rs` and are re-exported from `schema.rs`.
  The 6 tests move whole to `schema_tests.rs`.

This is your fork answer, «Seams + tests file».

## Corrected at P3 (scope premise closure)
- `document.rs` has six test modules, not the audit's five. Only two are escher's: `changed_set_tests` and
  `font_reading_tests`. `hover_state_tests` is upstream's (#546), with escher's quality-gates edits inside it, so it
  stays.
- After the move `document.rs` reads about 2,567 code lines, not 2,105. The file stays over 800, which v020-01 does
  not ask about.
- `schema.rs` has no help-text seam, because every help string is a field of its table row. Its seam is the shape
  types apart from the table.

## Top acceptance criteria
- **v020-01, on the audit's own instruments:**
  - the previous chunk's complexity reader still prints
    `over-ceiling: handle_click handle_key_or_input_event handle_pointerdown handle_pointermove touch_action_pan_axes`;
  - escher's two modules are gone and `document.rs` lines 1–3461 match the base `4f393991` byte for byte;
  - every driver file of the split is ≤ 800 tokei code lines;
  - the fast, doc and a11y legs pass, and the pushed sha's CI is green, `Stand checks repeated` and
    `Coverage report` included.
- **Byte identity:** `escher-session help` and `escher-session mcp` over seven requests (handshake, notification,
  `tools/list`, `help` and `status` results, a `-32700`, a `-32601`) match the records step 1 takes before any edit.
- **Test counts:** escher-driver 73 and `document::` 18, with both `-- --list` outputs unchanged.
- **Production files:** 0 panic-path, print, env, subscriber, feature-gate and serializer hits, except the handshake's
  compile-time `env!("CARGO_PKG_VERSION")` at `mcp.rs:339`. Nothing in the `mcp` submodules widens past `pub(super)`,
  and both `info_span!` sites read byte for byte.
- **Change set:** no tracked file outside the three sources changes (no manifest, `lib.rs`, check or lockfile edit),
  and the only new files are the eight named.

## Touchpoints
- **New:**
  - `packages/blitz-dom/src/document_changed_set_tests.rs`
  - `packages/blitz-dom/src/document_font_reading_tests.rs`
  - `packages/escher-driver/src/mcp/framing.rs`
  - `packages/escher-driver/src/mcp/listing.rs`
  - `packages/escher-driver/src/mcp/result.rs`
  - `packages/escher-driver/src/mcp_tests.rs`
  - `packages/escher-driver/src/schema/spec.rs`
  - `packages/escher-driver/src/schema_tests.rs`
- **Modified:** `packages/blitz-dom/src/document.rs`, `packages/escher-driver/src/mcp.rs`,
  `packages/escher-driver/src/schema.rs`

## Test commands
The block has 26 entries: 2 unit, 3 integration, 1 build, 15 probes, 2 local legs (a11y, fast, doc — the a11y leg
is filed as integration), and the operator triple (hygiene → push → CI read). No live leg and no boot smoke.
- **Baselines:** 19 entries were baseline-run on the untouched tree. The reds are the chunk's own subject (the
  inline modules, `mcp.rs` at 1,140 code lines) or the step-1 records and split files that don't exist yet. Every
  inline guard has its must-fail and/or must-pass control in a scratch clone, or a typed operand-only control for the
  record readers: `.andromeda/runs/2026-10-16T15-51-04-phase/controls.md`.
- **Seen green:** 4 entries read seen-green in the previous chunk's trail.
- **Two fixes P5 made:**
  - entries 15–17 now guard file presence (`ls … &&`), so a missing file can't read green under `pipefail`;
  - entry 15 excludes the one pre-existing compile-time `env!("CARGO_PKG_VERSION")` line, which its must-pass control
    found.

## Capability claimed — concretization preview
- **`v020-01` · Codebase structure** (method `integration`)
  - **Current acceptance:** "A re-read of the code audit's measures shows escher's handler branches for range and
    activation no longer over the complexity ceiling, escher's inline test modules gone from the engine's document
    file with upstream's body unchanged, and the two driver files split along their seams under the size line,
    while every existing check, the stand checks included, stays green."
  - **Proposed concrete form:** "Read on the audit's instruments at this chunk's wrap, with four parts:
    1. `rust-code-analysis-cli` 0.0.25 through `chunks/2026-10-16-handler-splits-and-readme/evidence/complexity.py`
       over `events/pointer.rs`, `events/keyboard.rs` and `node/range.rs` prints
       `over-ceiling: handle_click handle_key_or_input_event handle_pointerdown handle_pointermove touch_action_pan_axes`.
       So `parse_float` and every extracted range and activation function stand at or under 15, and the three
       parents stand below their pre-split figures at their upstream-floor remainder. This is the reading accepted
       at the previous chunk's P5 review.
    2. `changed_set_tests` and `font_reading_tests` are no longer inline in `packages/blitz-dom/src/document.rs`, and
       its lines 1–3461 equal the base `4f3939914e52` byte for byte.
    3. `mcp.rs` is split into `mcp/{framing,listing,result}.rs` plus `mcp_tests.rs`, and `schema.rs` into
       `schema/spec.rs` plus `schema_tests.rs`. Every one of these files and the two parents is ≤ 800 tokei code
       lines.
    4. `ci-leg.sh fast`, `doc` and `a11y` pass, and the CI run on the pushed sha reads `verdict: green` with every
       check succeeded, `Stand checks repeated` included."
  - **Never weaker:** each clause of the current form maps to a measured clause above, on the same instruments the
    audit used (tokei for size, rust-code-analysis for complexity).

## Expected amendments (wrap)
- architecture §Conventions → Tests: out-of-line `#[path]` test modules go from one to five.
- architecture §Existing Scopes rows for blitz-dom and escher-driver: `mcp`'s three children and `schema`'s `spec`.
- test-plan §2 and §4: the "bar one `#[path]` module" wording.
- Citation re-points into `mcp.rs`, `schema.rs` and `document.rs` past line 3461, across all seven masters, with no
  claim text changed. The leaves follow through the cascade.

## Beyond the plan
none
