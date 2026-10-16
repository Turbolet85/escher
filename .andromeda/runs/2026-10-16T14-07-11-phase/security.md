# security extract

## Relevance
partial — part A refactors engine code that the plan records as two ratified boundary widenings for every document, remote pages included (the range input, keyboard activation); a behaviour-preserving split crosses no new boundary, so the domain's job is to keep the validated gates and the input-parsing rules exactly as recorded. Part B (README) touches the domain only through the secret rules.

## Constraints
- `parse_float` split into steps must keep the reading security-plan §Input Validation (the `input type="range"` row) requires of the four attributes `min` · `max` · `step` · `value` from any document: the HTML floating-point parsing rules, with a parse error, NaN or an infinity reading as absent and `-0` reading as `0`. The sanitize path that follows it must keep the current value finite, clamped to `[min, max]` and on its step grid, including its overflow-safe arithmetic (midpoint `min/2 + max/2`, grid index `value/step − base/step`). Whether the extracted steps keep every one of these readings is research's question for the diff (per security-plan §Input Validation).
- The extracted range arms must keep the gates security-plan §Input Validation (the `input type="range"` row) requires. Keys act only on a focused range input with no `disabled` attribute (presence): the arrows step it, `Home` · `End` go to min and max, and every other key does nothing. The pointerdown arm sits after the `disabled` arm, its fraction clamped to `[0, 1]` and a zero-width content box reading 0. The click arm sits after the `disabled` check and focuses without moving the value. Every change still goes through the one path that marks the changed set and fires one `input` event, and a move that changes nothing fires none (per security-plan §Input Validation).
- The extracted keyboard-activation and button-focus arms must keep the gates security-plan §Input Validation (the keyboard-activation row) requires: a closed element-name and type match plus the `disabled` presence check. Enter is handled after the text-input and range arms, for `button`, `summary`, `input` of type `submit` · `reset` · `button`, and `a` with `href`. A released Space is handled through `handle_key_up`, for `button`, `summary`, and `input` of type `submit` · `reset` · `button` · `checkbox` · `radio`; a link, a text input and a range input take no Space. The Click is queued through the event driver, so the app's handlers run before `handle_click`. In the button arm of `handle_click`, a `disabled` attribute must still stop the walk first (per security-plan §Input Validation).
- `disabled` must stay a presence reading wherever these arms test it (bare, `"false"` or `"true"` all disable), never a parsed boolean through `attr_parsed`, as security-plan §Input Validation (the `disabled` row) requires. That row also requires the arm order: the range pointerdown arm after the `disabled` arm, and the range click arm and the activation arms after the `disabled` check (per security-plan §Input Validation).
- The plan's two widening rows each record "no print, log site, env read, `unwrap`, `expect` or panic" in the range rules module and in the added engine lines. They also record "no carrier, verb, key name (`KEY_NAMES` unchanged), field, log field or span added". The new named functions and the `parse_float` steps must hold to both (per security-plan §Input Validation).
- Any engine log call the refactor touches stays behind the crate's `tracing` feature, as the plan requires of engine crates. The split adds no new log site (per security-plan §Logging & Monitoring, Log format and backends).
- The README is committed text. It must name no secret, credential or login value: not the operator's Claude Code login the cold-agent live run uses, no CI secret, no token (per security-plan §Secret Management, What counts as secret / Never in code).

## Patterns to follow
- One change path. Range moves go through the single function that marks the changed set and fires one `input` event carrying the formatted value. Extracted arms call that path and do not re-implement it (per security-plan §Input Validation, the `input type="range"` row).
- Synthetic activation. Enter and Space dispatch a Click built from `Node::synthetic_click_event_data(modifiers)` and queued by the event driver. They never call `handle_click` directly, so the app's handlers keep running first (per security-plan §Input Validation, the keyboard-activation row).
- Closed-set gates. Element-name and `type` matches are fixed `matches!`-style sets compared ASCII-case-insensitively, with no text carried out. This is the same shape the plan records for the `aria-*` state reads (per security-plan §Input Validation, the `aria-selected` · `aria-pressed` · `aria-invalid` row).
- Graceful degradation on markup. A degenerate range attribute reads as absent or default, never as an error or a panic. `parse_float` returning "absent" is the contract for the split steps to keep (per security-plan §Error Handling, Graceful degradation, and §Input Validation, the range row).

## Anti-patterns to avoid
- Adding an `unwrap` / `expect` / `panic!` / `todo!`, a print or log site, or an env read inside an extracted handler function or a `parse_float` step. The plan's census of these lines must keep reading 0, and the upstream `expect` it lists under panic paths for keyboard.rs must not gain a sibling in escher's arms (per security-plan §Input Validation and §Error Handling, Panic paths).
- Reordering an extracted arm so it runs before the `disabled` check, or before the text-input arm (the Enter case) (per security-plan §Input Validation, the `disabled` and keyboard-activation rows). Either change would alter behaviour for remote documents. Per the scope, a behaviour change is a fork for the founder, not a step of this chunk.
- Adding a dependency or touching `Cargo.lock` for the split. Builds stay `--locked`, and any new crate goes through the cargo-deny audit and a hand review (per security-plan §Dependency Security, Pinning / Lockfile verification).

## Contract bindings
- security ↔ tests: the widening rows are witnessed by the checks the scope keeps green unchanged:
  - `tests/blitz-tests/tests/range_input_interaction.rs`: the degenerate-markup rows and the disabled rows, both layout modes;
  - the 8-row unit table inline in `packages/blitz-dom/src/node/range.rs`;
  - `tests/blitz-tests/tests/stand_keyboard.rs`;
  - `tests/blitz-tests/tests/focusability_updates.rs`.

  These checks are the security evidence that the refactor changed no gate (per security-plan §Input Validation, the range, keyboard-activation and `disabled` rows).
- security ↔ wrap drift: the plan's §Input Validation rows (range, keyboard-activation, `disabled`) and §Error Handling (Panic paths) cite exact line ranges in `packages/blitz-dom/src/events/{pointer,keyboard}.rs` and `packages/blitz-dom/src/node/range.rs`. Moving the arms into named functions shifts those lines, so the wrap must re-point the citations.
- security ↔ obs: no log field or span is added. The engine's `tracing`-gated sites and escher's sink allowlist stay as they are (per security-plan §Logging & Monitoring).
- security ↔ a11y: keyboard activation is SC 2.1.1 surface. `stand_keyboard`'s Enter and Space tables bind both domains to the same unchanged gates.

## Acceptance criteria contributions
- A census over the extracted functions and the split `parse_float` (non-test code) reads 0 `unwrap`, `expect`, `panic!`, `todo!`, `unreachable!`, print or `dbg!`, `tracing`/`log` macro, and `std::env` / `env!`. The diff adds no carrier, verb, `KEY_NAMES` entry, snapshot field, log field or span (per security-plan §Input Validation).
- These pass unchanged in both layout modes: `range_input_interaction` (degenerate-markup and disabled rows included), the `node/range.rs` unit table, `stand_keyboard` and `focusability_updates`. Each extracted arm still sits after its `disabled` check, and `disabled` is read by presence (per security-plan §Input Validation).
- The diff touches neither `Cargo.toml` dependency lines nor `Cargo.lock`. `bash .github/scripts/ci-leg.sh audit` stays green if any dependency file changes (per security-plan §Dependency Security).
- `README.md` holds no credential, token, login or secret value (per security-plan §Secret Management).
