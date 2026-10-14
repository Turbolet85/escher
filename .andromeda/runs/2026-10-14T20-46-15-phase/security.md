# security extract

## Relevance
partial. The sweep is mostly checks run through the driver's validated surfaces. Several folded CARRYs (C7 · C8 · C9 · C11 · C15 · C16, and C4 · C5 · C14 if a fork answers "change") touch input-validation rows, masking, the state-directory bound and the log census.

## Constraints
- Every sweep call that is "by an agent" must enter through `validate` / `validate_session` before anything runs: `Session::run` in process, the `escher-session` command line, or an MCP `tools/call`. The sweep must add no verb, no argument kind, no refusal cause, no carrier and no exit. A field holding authored text or an open value set, a new carrier or a new exit is a widening for the founder; a new field of a closed value set inside an already ratified carrier is not (per security-plan §Input Validation, the Driver command schema row and the Driver session socket row).
- A refusal and a session error must stay fixed table text that holds nothing a call supplied: no id, typed text, path or label. `Cause`, `Fault` and `Refusal` must carry no `String` field, and `StateDirTooLong`'s message must state its bound as a figure with no path (per security-plan §Error Handling).
- A password's or a file input's value must read `MASKED_VALUE` in the snapshot, its text, the diff, an answer's JSON and an MCP result. A password input must paint masked in a shot. For C11, security-plan §Input Validation (password row) records the caret-by-real-characters reading as unmeasured and owned by this entry. Any fix in blitz-dom's pointer path is an engine change for every document. A file input's written form and its MCP result are recorded "by the same construction, not separately run"; whether the sweep measures them is P4's (per security-plan §Input Validation, password row).
- If C7 (`aria-checked`) or C8 (Cells' `aria-selected`) is taken, a new attribute read by the accessibility builder must keep the shape of the existing state row:
  - a closed set, matched on the trimmed lower-case value;
  - no log site and no panic path;
  - only closed enums or booleans written on the node, never the attribute's text;
  - exits only through the platform adapter and the snapshot.

  Whether that is a widening or an inventory fact is the founder's rule for closed value sets, and the engine change is his fork (per security-plan §Input Validation, the `aria-selected` · `aria-pressed` · `aria-invalid` row).
- `disabled` is read by presence, and that one reading drives focusability, the `DISABLED` element state, pointer selection and click default actions, a range input's key arms, the Enter and Space activation arms, and the snapshot's `enabled`. A C15 narrowing to form controls is an engine change for every document. Whether narrowing focusability alone would split it from those other readers is research's question (per security-plan §Input Validation, `disabled` row).
- The off-Linux bound of the session state directory is recorded as 90 bytes, read from the code and not measured; on Linux it is 94 bytes, measured. The path must stay a parameter, never an env read or a cwd-relative default (per security-plan §Input Validation, Driver session state directory row).
- A binary that installs escher's sink must print no element id, accessible name, typed text or shot base64 at any `RUST_LOG` level. Engine targets print only the safe fields, and records from other targets are dropped whole. C16's `focussed node` info event and the href log site reached from Enter on `a href` must be judged against that rule. Whether the dated census counts still hold is research's question (per security-plan §Logging & Monitoring).

## Patterns to follow
- A refused driver call leaves the snapshot and both focus readings as they were, in both layout modes, checked by comparing the snapshot before and after (per security-plan §Input Validation, Driver command schema row).
- Masking is measured with a typed sentinel that must occur 0 times in the call's whole written form, with the node reading `MASKED_VALUE`. `stand_act_filled` and `stand_mcp_masked` do this (per security-plan §Input Validation, password row).
- The stderr census runs the `escher-session` host and its `mcp` role at `RUST_LOG=trace` under both builds and counts 0 id, 0 name and 0 typed-sentinel needles, plus 0 of a shot's first 64 base64 characters. `host_log` and `mcp_log` do this (per security-plan §Logging & Monitoring; security-plan §Input Validation, `aria-label` · `<label for>` row).
- A state attribute is read by a closed-set match into AccessKit enums or booleans. The snapshot takes the value from the tree node and never parses the element a second time (per security-plan §Input Validation, the `aria-selected` · `aria-pressed` · `aria-invalid` row).
- Stand fixtures boot offline on the bundled font, and no driver verb takes a URL (per security-plan §Input Validation, `@font-face` source row).

## Anti-patterns to avoid
- Never log or field an `Outcome`, `Call`, `Command`, `Refusal`, id, name, value, or a shot's image, size or dimensions. Never add a field to the `command` or `tool_call` span beyond its closed words and counts (per security-plan §Input Validation, Driver command schema row).
- Never carry an attribute's typed text out of a new accessibility read. Unrecognized or very long values must be written nowhere as typed (per security-plan §Input Validation, the `aria-selected` · `aria-pressed` · `aria-invalid` row).
- Never route a fixture-, agent- or user-supplied URL through blitz-net or the sniffed `@font-face` path from a sweep fixture. Responses there have no size cap and requests no timeout (per security-plan §API Security; security-plan §Input Validation, `@font-face` source row).

## Contract bindings
- security ↔ obs: C16 re-reads the dated `host_log` / `mcp_log` line counts in obs-plan §2, §6 and §8 against the sink census in security-plan §Logging & Monitoring.
- security ↔ tests: the sweep record's refusal and masking checks bind the stand tables and the refusal readers in `tests/blitz-tests/tests/session_common/mod.rs` (test-plan §3), held to security-plan §Input Validation (Driver command schema row and password row).
- security ↔ a11y: if taken, C7 and C8 put new state readings on the platform-adapter crossing, whose assistive-technology witness is owned by a `.andromeda/residuals.md` line (security-plan §Input Validation, the `id` row and the aria state row), and on a11y-plan's state rows.
- security ↔ CI: a C9 check on the macOS leg binds `.github/workflows/ci.yml` and its invariants in `.github/scripts/test_ci_workflows.py`. The workflow-level `permissions` must stay read-only and every `uses:` must stay SHA-pinned (security-plan §Authentication & Authorization, RBAC rows; security-plan §Dependency Security).

## Acceptance criteria contributions
- Every refused call the sweep makes, through any surface, returns a refusal holding none of the call's id, text or argument value, and the snapshot after it equals the one before, in both layout modes (per security-plan §Input Validation, Driver command schema row; security-plan §Error Handling).
- If C9 is checked off Linux: a state directory whose socket path is 90 bytes holds a session, and one at 91 bytes ends `state-dir-too-long` with status 3 and a message holding no path (per security-plan §Input Validation, Driver session state directory row).
- If C7 or C8 is taken: an unrecognized value and a value of several thousand characters each build the tree without a panic, and neither appears in the tree's `Debug`, the snapshot text or a diff's JSON. Only closed words or booleans are read out (per security-plan §Input Validation, the `aria-selected` · `aria-pressed` · `aria-invalid` row).
- After any engine change this chunk makes (C15, C16 or a fork answered "change"), the `escher-session` host and `mcp` role at `RUST_LOG=trace` still show 0 id, 0 name and 0 typed-sentinel needles on stderr under both builds (per security-plan §Logging & Monitoring).
