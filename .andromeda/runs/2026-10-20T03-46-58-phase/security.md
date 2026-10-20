# security extract

## Relevance
partial — the chunk crosses the driver's validated command surface and the snapshot's control-value crossing, and may add engine default actions that reach every document, remote pages included. It adds no listener, credential or dependency.

## Constraints
- If an option is chosen through a new verb, a new argument or a new argument kind, that widens the driver's validated surface. security-plan §Input Validation (Driver command schema row) requires the change to go into the closed `'static` table (`VERBS`, eleven today), with a bound for each argument kind. `validate` must run before anything else, the refusal order must stay fixed, nothing a call supplies may reach an `unwrap`, `expect`, `panic!`, `unreachable!` or an index, and the change needs the founder's ratification, as `scroll` (2026-10-07) and `screenshot` (2026-10-12) had. Reusing an existing verb (`click`, `type`, `press`) leaves the table as it is. Which way the chunk goes is a P4 fork for the founder.
- A select's keyboard or pointer default actions would reach every document, the browser app's remote pages included. per security-plan §Input Validation (range-input row and keyboard-activation row), they must gate on `disabled` by its presence on a form control (`ElementData::is_disabled`), which security-plan §Input Validation (`disabled` row) requires to cover `select`. Any match must be a closed element-name and type match. The added engine lines may hold no print, log site, env read, `unwrap`, `expect` or panic. Such an engine change for every document is a boundary widening that the founder must ratify.
- The chosen value may leave only through channels that already exist: form submission's entry, the `input` or `change` event's value, and the snapshot's `NodeState.value` inside the ratified carriers. per security-plan §Input Validation (range-input row), the chunk adds no carrier, verb, field, log field or span. Whether the code already reads any value for a `select` is research's question.
- The snapshot may read the select's chosen value in the existing `value` key. per security-plan §Input Validation (Driver session socket row, the founder's rule of 2026-10-10), a new key that holds authored text (such as an option list or the option labels) or an open value set is a widening for the founder. A new field of a closed value set inside a ratified carrier is not.
- A refusal holds nothing a call supplies. per security-plan §Error Handling (escher-driver `Refusal`), the cause set is closed at eight with fixed texts, so an option the select does not hold must be refused under an existing cause, or by a new cause the founder ratifies. Neither `Refusal` nor `Fault` may gain a `String` field.
- The command span keeps its eight fixed-word and count fields, and the `tool_call` span keeps `tool` and `ending`. per security-plan §Input Validation (Driver command schema row), no field may hold an argument such as an option value, the chosen value or screen text. per security-plan §Logging & Monitoring (escher's own sink), `value` and `text` stay in the redacted set for `escher_*` targets.
- A recorded session keeps each call's arguments as they crossed, and each answer whole. per security-plan §Data Protection (Files (kept checks)), a new argument naming an option, and the select's value in a kept answer, sit at rest in the owner-only check. They are masked only for a masked control (`is_masked_input`). A select is not one, and the chunk must not add it there silently.

## Patterns to follow
- The range input's interaction model (per security-plan §Input Validation, range-input row): a current value held apart from the attribute, every seed and move sanitized, one change path that marks the changed set and fires one event, a move that changes nothing firing none, and the gates read after the `disabled` check.
- Keyboard activation (per security-plan §Input Validation, keyboard-activation row): a closed element-name and type match on the focused node, gated by `ElementData::is_disabled`, with the app's handlers dispatched before the default action.
- The executor's id path (per security-plan §Input Validation, Driver command schema row): the target is resolved by stable id against the screen as it reads now (`stale` or `not-found`), then `disabled`, `off-screen` and `covered` in that order, with nothing run on a refusal.
- Attribute reads in the closed shape (per security-plan §Input Validation, control-states row): a closed-set match, with closed enums or booleans written on the accessibility node and never the attribute's text, unless a value crossing is ratified.

## Anti-patterns to avoid
- A new verb, argument kind, refusal cause, snapshot key holding authored text, or engine default action for every document added without the founder's ratification (per security-plan §Input Validation, Driver session socket row, the 2026-10-10 rule).
- A panic path or log site on page-supplied option markup or a call-supplied option argument, for example indexing the options by a caller's number unchecked or `unwrap` on an option lookup (per security-plan §Input Validation, Driver command schema row).
- An option value, label or chosen value in a span field, a log line, a `Refusal`, a test name or an assertion message (per security-plan §Error Handling, escher-driver `Refusal`).

## Contract bindings
- security ↔ obs: the command span's eight fields and the `tool_call` span's two stay fixed, and the sink's redaction of `value` and `text` holds (obs-plan's span contract).
- security ↔ tests: the fixture page lives in the shared fixtures module, offline (the held network stand-in), and is checked in both layout modes. The page holds no secret or host path.
- security ↔ a11y: `disabled` read by presence governs the select's focusability, and so its Tab-order place (`stand_keyboard` tables unchanged).
- security ↔ arch: a new verb or argument kind moves `VERBS`, `describe()`, the MCP `tools/list` and the security plan's Driver command schema row together.

## Acceptance criteria contributions
- A select carrying a `disabled` attribute is not focusable and takes no choice through the driver (refused `disabled`, with the snapshot and focus unchanged), in both layout modes (per security-plan §Input Validation, `disabled` row).
- A census of the chunk's added engine and driver lines reads 0 for print, log site, env read, `unwrap`, `expect` and panic on page- or call-supplied input (per security-plan §Input Validation, range-input row).
- A sentinel option value and label, chosen through the driver, occur 0 times in the escher-session host's stderr at `trace`. The command span still records only its eight fields (per security-plan §Logging & Monitoring, escher's own sink).
- If a verb, argument kind or refusal cause is added, `validate` refuses each out-of-bound or wrong-kind argument as `malformed`, with nothing run, and no `Refusal` holds call text (per security-plan §Error Handling, escher-driver `Refusal`).
