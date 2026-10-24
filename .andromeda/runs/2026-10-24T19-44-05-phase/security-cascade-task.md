# Task — security · 2026-10-24-a11y-re-read

## The chunk
A test-and-CI re-read, no engine change: the stand's Tab order, Enter/Space activation and text contrast are asserted again against the stand as 0.2.0 leaves it, in both layout modes.
A sequence or row that 0.2.0 moved is re-recorded with the change named. A stand control that 0.2.0 made keyboard-operable gets a new row; the flight booker's single-choice select, with its arrow, Home/End and typeahead model, is the candidate.
The `a11y` CI leg grows from its six test files to include the 0.2.0 a11y witnesses (accessibility states, label, description, merged states, active-descendant-unread, inert subtree, select interaction, stand act select), and its pin in the CI-workflow invariants moves with it. The ci.yml job keeps its tier and gets no required status check.
No carrier, verb, key name, field, log field, span, listener, env read, crate or dependency is added. A defect that needs engine behaviour changed for every document goes to the founder as a fork and is not fixed in this chunk.

## Reached files
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/common/mod.rs
- .github/scripts/ci-leg.sh · merge surface
- .github/scripts/test_ci_workflows.py · merge surface

## Needs
- Markup attributes read by the engine in every document, remote pages included: `disabled` by presence on a form control alone, `inert` by presence on an element or an ancestor, ARIA states through closed parses that carry no attribute text, and names and descriptions as page text. The re-read asserts their effects on focus, hits and the accessibility tree.
- Keyboard activation: a pressed Enter or a released Space on the focused button, summary, link or input type dispatches a click only when the target is not a disabled form control. Each stand button's Enter and Space row exercises this.
- The single-choice select model: arrows, Home/End and typeahead on a focused select that is not disabled. The typeahead text is held in a private buffer for at most 1 s of the document's clock and never carried out. The choice leaves only through form submission, the input/change events, the snapshot's `value` and the options' accessibility nodes. A new keyboard row on the flight booker's select touches all of this.
- The range input's key model (arrows, Home/End, gated on no `disabled` attribute), where the stand's Timer slider is in the Tab sequence.
- Focusability rules: `disabled` read on form controls alone, and no inert node focusable. These set the boot and enabled-state Tab sequences that `common::tab_order` and `common::ENABLED_TAB_ORDER` hold.
- The accessibility builder's state reads (`aria-selected`, `aria-pressed`, `aria-checked` on the five checkable roles, `aria-invalid`), plus upstream's merged `aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled` and `aria-level` reads. The newly added leg targets witness these; descriptions reach the adapter crossing only, never the snapshot.
- The stable element id and the snapshot readings (selected, pressed, invalid, value) that the stand checks read. The select's chosen value is read unmasked. A password's or file input's value reads as the fixed mask and must stay masked in any assertion the chunk adds.
- Indirect, through `stand_act_select` joining the leg: it drives the driver session in process through `Session::run`, with no socket. Validation, the refusal causes and the no-call-content rule for a `Refusal`, a `Fault`, a test name or an assertion message all apply to any row or assertion message this chunk writes.
- Indirect, through the contrast rows: blitz-test-harness's read-only contrast reader (`Harness::text_contrast`, with `color` as its luminance source). The reader is used as is and no dependency enters `Cargo.lock`.
- Indirect, through the edit to ci-leg.sh: every cargo command a leg runs, the a11y leg included, passes `--locked`. The widened test list must keep that.
- Indirect, through the edit to test_ci_workflows.py: the same file asserts that every ci.yml `uses:` is pinned to a 40-hex commit SHA. Its a11y-tuple change must leave that assertion and the other CI invariants intact.
- Indirect, through the CI leg: the ci.yml jobs' read-only `GITHUB_TOKEN` scope and the rule that the a11y job reads no secret. The job's tier and status-check standing stay unchanged.
- Test-harness queries: selector parsing returns a `Result` while the harness query helper panics on an invalid selector, and stale `NodeId`s are read through `get`/`contains_key`. Both apply to new test code.
- The escher log sink rule that no element id, accessible name or typed text reaches stderr. Test code may print nothing that carries these into a captured log the chunk commits as evidence.

## Not needed
- The driver session's Unix-domain socket, its wire, state directory, idle expiry and answer bounds: the chunk's tests run any driver call in process, and no socket test is added to the leg.
- The MCP stdio server (`serve_mcp`, the `escher-session mcp` role), its framing, bounds and `tool_call` span: no MCP check is edited or added.
- Session recording, the recording directory, kept checks, `Check::from_bytes`, replay and the two committed Counter checks: none is touched.
- The cold-agent pipe, the model provider crossing and the agent-written check: not run or changed.
- Remote content fetching (blitz-net `file:`/`data:` reads, fonts, images, SVG, `@font-face` sniffing, iframe and import depth caps): the stand boots offline with its bundled font.
- Script execution (blitz-vibey-script, the ScriptFetcher, `isTrusted`): the stand runs no document script.
- Secret management beyond the CI token scope (signing material, the operator's Claude Code login, the env-read registry): no env read or secret path is added.
- Dependency audit ignores in `deny.toml`, licence gating, serde_json/png use, upstream-sync lock edges and supply-chain signing: no package or lock edge changes.
- The platform accessibility adapter crossing and its windowed assistive-technology witness: out of scope (residuals, target next).
- HTML's inert focus fixup and native checkedness read by the builder: residuals marked target next, each an engine change, not absorbed.
- Browser app persistence (sqlite history), HTTP cache, clipboard and file dialog data handling, the WPT runner, and rdme's markdown rendering: not reached.
- Engine panic paths and graceful-degradation entries in the error-handling plan: no engine code is edited.
- The screenshot verb and its in-band PNG crossing, and the visual lint's findings: no screenshot or lint check is edited or added to the leg.
