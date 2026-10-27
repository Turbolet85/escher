# security extract

## Relevance
partial — a behaviour-preserving refactor that opens no surface, but one of its three functions, `lint.rs::findings`, is a reader of the stable id whose output crosses every ratified carrier (socket `call`, command line, MCP, kept check), so the refactor must leave that crossing exactly as ratified; the two test functions touch no boundary.

## Constraints
- Security-plan §Input Validation (Markup attributes, the stable element id row; detail `.andromeda/security-plan-detail/input-validation.md`, the `Session::lint` reader) requires every `Finding` to hold only ids, closed defect words and numbers: no `NodeId`, page text, colour or control value, and a text control's value never measured. A helper or table walk split out of `findings` must not add a field or carry a node handle out. Research must say whether the current `Finding` type already rules this out by construction.
- Security-plan §Input Validation (Driver session socket, the widening rule) treats any new field, open value set, carrier or exit as a boundary widening. The chunk declares no verb, field, carrier, log site or span change, so the lint's answer must stay byte-identical on every carrier, `{"findings":[…]}` included, in the same order (document order, then `Defect::ALL`).
- Security-plan §Logging & Monitoring (escher's own sink) limits the driver's log records to the `command` span's fixed words, bools and counts. A lint records `verb` and, when refused, `cause` alone. No new helper may add a `tracing` call, span field or print that carries a finding or the number of findings.
- Security-plan §Error Handling (typed refusal) requires the `fonts-unavailable` refusal to stay a typed `Refusal` from the fixed table with no `String` field. Its position before anything is read must survive the factoring.
- Security-plan §Input Validation (Driver command schema, the no-panic row) bans `unwrap`, `expect` and panics on anything a call supplies, and bars any `NodeId` from being kept between calls. Security-plan §Input Validation (Layout values, `text_extent`) requires the lint's geometry reader to stay read-only and `Option`-returning through `get_node`. Helpers must keep reading through `Option`/`get` and must not index a node id.
- Security-plan §Input Validation (Driver command schema, the id record) feeds the session's private record of ids from `Session::lint`'s own snapshot. That record must be fed the same way after the move, and it is never printed, logged or put in a field.
- Security-plan §Dependency Security (Audit tool) requires dependency changes to pass the audit leg over the whole workspace graph. A complexity refactor needs no new crate, so `Cargo.toml`, `Cargo.lock` and `deny.toml` should not change.

## Patterns to follow
- Walk the closed `'static` tables (`Defect::ALL`, `VERBS`) in their fixed order rather than adding open lists. This keeps output order and closed word sets the same as the schema's (per security-plan §Input Validation, Driver command schema).
- Use `Option`-returning reads (`get_node`, `text_extent` → `None`, `Harness::text_contrast` → `None`) and skip a node that does not resolve, never panic on it (per security-plan §Error Handling, graceful degradation for stale node ids).
- Produce typed values only (`Finding`, `Defect`, `Reason`, `Bounds`) and write the lint's JSON only through `Outcome::to_json` (per security-plan §Input Validation, the `Session::lint` reader row).

## Anti-patterns to avoid
- Adding a debug print, a `tracing` event or a span field inside a new lint helper that names a finding, an id or the number of findings (per security-plan §Logging & Monitoring).
- Moving the `fonts-unavailable` check after the first read of the screen, or letting a helper return a free-text error instead of the fixed `Refusal` (per security-plan §Error Handling).

## Contract bindings
- security ↔ obs: the lint's `command` span is held to the fields `verb` and `cause`. Logging detail `.andromeda/security-plan-detail/logging-monitoring.md` names `stand_act_spans` among the span-capture checks. Research must say whether that check reads a `lint` line today.
- security ↔ tests: the lint's output crossing is pinned by `stand_lint` and escher-driver's lint unit tests, and the MCP listing's fixed table text by `mcp_tests.rs`. These must pass unchanged as the security witnesses of "no crossing changed". `stand_requirement_sweep` stays a target held by `.github/scripts/test_blitz_tests_targets.py`.

## Acceptance criteria contributions
- `stand_lint` and escher-driver's lint unit tests pass unchanged in both layout modes. The findings stay the same, each holding ids, closed defect words and numbers only (per security-plan §Input Validation).
- `git grep -n -E 'unwrap\(|expect\(|panic!|println!|eprintln!|tracing::' packages/escher-driver/src/lint.rs` shows no new non-test hit compared with the chunk's base (per security-plan §Input Validation, Driver command schema).
- `bash .github/scripts/ci-leg.sh audit` is green, and `git diff --stat` at the chunk's HEAD shows `Cargo.toml`, `Cargo.lock` and `deny.toml` untouched (per security-plan §Dependency Security).
