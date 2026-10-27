# Task — arch · 2026-10-26-codebase-health

## The chunk
Refactors three escher-written functions over the cognitive-complexity ceiling of 15 into table walks or named helpers, with no change of behaviour (v030-01, Epoch 5 audit M1, the founder's word).
Builds: `lint.rs::findings` (the visual lint's per-node walk, cognitive 16), `mcp_tests.rs::the_listing_is_the_twelve_verbs_of_the_table` (18) and `stand_requirement_sweep.rs::every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (18), each brought to at most 15, every new helper also at most 15.
Test functions stay in the counted population; the engine function `clear_interaction_state_for_removed_node` and every other over-ceiling function (upstream's, `mutator.rs`'s closure) stay as they are.
Re-measures the audit's own complexity reading (rust-code-analysis 0.0.25, `kind == function`, cognitive `> 15`) on the three files before and after, `complexity.over_ceiling` at most 85 with no new entrant.
Re-asserts the same rows in both tests, the lint's unchanged findings through `stand_lint` and its inline unit tests, and the `fast` and `doc` CI legs green.

## Reached files
- packages/escher-driver/src/lint.rs · edits `findings` and the named helpers or table it is factored into
- packages/escher-driver/src/mcp_tests.rs · edits `the_listing_is_the_twelve_verbs_of_the_table`
- tests/blitz-tests/tests/stand_requirement_sweep.rs · edits `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
- tests/blitz-tests/tests/stand_lint.rs · re-reads `stand_lint`, both layout modes
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target pin holding `stand_requirement_sweep`'s name and target
- .github/scripts/ci-leg.sh · re-reads the `fast` and `doc` legs
- .andromeda/runs/2026-10-25T21-43-01-code-audit/over_ceiling.py · re-reads the audit's over-ceiling complexity reading

## Needs
- The visual lint `Session::lint` and its per-node walk `findings`: the same `Finding`s in the same order — document order, then `Defect::ALL` order (`overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`) — the same six defects, nothing changed on the held instance and no step run.
- The lint's `fonts-unavailable` refusal stays before anything is read; a helper that moves the font check after a read breaks it.
- `Finding` · `Defect` · `Reason` · `Bounds` keep holding stable ids, closed words and numbers only — a helper or table introduces no `NodeId`, page text, colour or control value into a finding.
- The lint's readers stay read-only: `BaseDocument::text_extent` (mutates nothing) and `Harness::text_contrast` (never moves the harness's `VirtualClock`), applied under `stand_contrast`'s exempt rule for the `contrast` defect.
- The lint answer `Outcome::Linted { findings }`, written by hand as `{"findings":[…]}` in the crate-private `json` module, stays byte-identical, so a kept check's recorded answer and its FNV-1a 64 digest still match on replay.
- The `command` span of `Session::run` for `lint` records `verb` and, refused, `cause` alone; the refactor adds no log site, print, span or span field, and none carries a finding or its count.
- `stand_lint`'s pinned readings hold: counter, timer, CRUD and the load page 0 findings at boot, the flight booker one `overlap` (`flight-type-one-way` × `flight-type-return`), the last CRUD row after twelve Creates `clipped` inside `crud-list`, and a clipped or off-screen node holding text also `truncated`, in both layout modes.
- The MCP `tools/list` answer the listing test asserts: twelve tools in `VERBS` order, `lint` after `screenshot` and `help` last, each described by its help and its result fields — the same rows, asserted against the `'static` `VERBS` table.
- The listing test lives in `mcp_tests.rs`, the `#[path]` child unit-test file of `mcp.rs`; its test path and name stay as they are.
- The requirement sweep asserts each 0.2.0 requirement names its witness inside the `stand-repeat` leg (the stand checks, the ten named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks); the same rows asserted, the test still counted.
- `stand_requirement_sweep` stays one blitz-tests test target of its own file, its name and target unchanged, as the blitz-tests target pin holds it.
- Asserting the same rows means each refactored test still fails on the row it guarded: dropping a row from its table (or from `VERBS` or the requirement witness list) turns it red.
- The complexity reading: rust-code-analysis 0.0.25's per-function cognitive sum, threshold `> 15`; each of the three reads at most 15, every helper or table walk introduced reads at most 15, and the population's over-ceiling count reads at most 85 (88 less the three) with no new entrant.
- The refactor changes no behaviour, verb, argument, result field, cause, carrier, log site or span — `VERBS` still holds twelve and the schema's tables are not edited.
- Gates: `bash .github/scripts/ci-leg.sh fast` (fmt, clippy `--locked -D warnings`, workspace tests, CI scripts) and `bash .github/scripts/ci-leg.sh doc` (rustdoc `-D warnings` over every workspace crate) pass; the two touched tests pass in both builds the workspace runs.

## Standing conventions
- S1 · Unit tests sit in `#[cfg(test)] mod tests` at the end of the source file or in a `#[path]` child file of their own, a moved module keeping its test paths · conventions.md (conv-tests)
- S2 · Each integration test file covers one behaviour and opens with a `//!` doc; scenarios run in both layout modes; stand checks share helpers through modules under `tests/` that are no test target · conventions.md (conv-tests)
- S3 · `cargo fmt --all --check` and clippy `--workspace --locked -D warnings` clean; `clippy::expect_used` / `clippy::unwrap_used` allowed only next to a justification · conventions.md (conv-formatting-and-lints)
- S4 · Modules and files open with `//!` docs, public items carry `///`, doc comments state the rationale for constants · conventions.md (conv-documentation)
- S5 · Crate internals are `pub(crate)` / `pub(super)` or private; a new helper is no public item · conventions.md (conv-visibility-and-naming)
- S6 · Known gaps marked `TODO` / `FIXME`, sections delimited with `// === … ===` · conventions.md (conv-known-gaps)
- S7 · Let-chains in `if` conditions; borrow conflicts worked around by taking a list out and restoring it · conventions.md (conv-unsafe-and-borrowing)

## Not needed
- blitz-dom's interaction-state clearing on a removed node and its invalidation rules: `clear_interaction_state_for_removed_node` is left as it is.
- `DocumentMutator` and the mutation API: `mutator.rs`'s over-ceiling closure stays as it is and no DOM mutation is added.
- The upstream engine's other over-ceiling functions across blitz-dom, blitz-paint, blitz-shell and the other engine crates: out of scope.
- The driver session's Unix-domain socket, its `v3` wire grammar and bounds, the state directory, idle expiry and the `escher-session` host role: no carrier or request changes.
- The MCP server's framing, handshake revisions, request and result bounds and protocol errors: only the listing test's assertion form changes.
- The recording directory, `--record`, `Check::from_bytes` and the two committed Counter checks: nothing is recorded or committed anew.
- Occupied Resources registrations (network ports, listeners, outbound hosts, environment variables, names): no listener, env read, binary or crate added.
- escher-telemetry's bootstrap, target allowlist and scrub sets: no log line added.
- The snapshot model, its text form, parts and diff, stable element ids and `unkeyed_actionable`: the lint reads a snapshot, none of their code changes.
- The engine's semantics — range input, select model, keyboard activation, `inert`, scrolling and hit testing, password painting: not reached.
- The headless screenshot and its renderer backend: no shot is taken or changed.
- The cold-agent pipe and the agent-run contract: not reached.
- The browser app, rdme, the WPT runner and blitz-vibey-script's JavaScript runtime: not reached.
- Release versioning, build profiles, the Nix flake and cross-compilation: unchanged.
