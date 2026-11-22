@task_prose part=session
An Epoch 2 corrective that split code without changing behaviour; no public symbol, span, event, log site, field or answer changed.
escher-driver's command validation module had its inline unit tests moved whole into a child test module file; the 7 tests keep their paths and the verb table, refusals and their callers are untouched.
Three upstream engine bodies in blitz-dom had escher's additions moved into private functions of escher's own, each one call in upstream's body: the accessibility node builder (expanded, selected, toggled, invalid and name reads), the keyboard handler (Tab closing an open list; the range, list box, drop-down, multiple select and Enter activation chain) and the presentational hints (a select's size height and a single list box's overflow).
The lock bumped url 2.5.8 to 2.5.9 and form_urlencoded 1.2.2 to 1.2.3, closing RUSTSEC-2026-0592; the syn edges of bindgen and cssparser-macros stay at 3.0.6 on the operator's ruling, a hazard a later url update would re-break.
Measured: escher-driver coverage 97.97 % to 97.80 % against its floor 97; the stand duplication reading 44 pairs / 372 lines under its 47 / 550 bar; CI on the pushed head green 17 of 17, the dependency audit job success.
@task_file n=1 path=packages/escher-driver/src/command.rs
what: escher-driver's command validation — the verb table that turns a driver call into a command, checks each verb at its session or instance level and refuses a malformed call or an unknown verb with its cause
@task_file n=2 path=packages/escher-driver/src/command_tests.rs
what: escher-driver's command validation unit tests — every admitted verb becomes its command, an unknown verb refused by that name, a malformed call refused with the rule it broke
@task_file n=3 path=packages/blitz-dom/src/accessibility.rs
what: blitz-dom's accessibility tree builder — each node's role, name and states (expanded, selected, toggled, invalid) that the snapshot and the UI graph answer read
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs
what: blitz-dom's keyboard event handling — key presses on a range input, a single list box, a drop-down select, a multiple select and Enter activation, and Tab closing an open select list
@task_file n=5 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: blitz-dom's presentational hints — a select's size attribute read into a list box's height, a single list box's overflow and its row blocks
@task_file n=6 path=Cargo.lock
what: the workspace dependency lock the dependency audit leg and the CI audit job read — url and form_urlencoded versions
@task_item n=1 part=touches
decision/select-size-attribute-read — the select size attribute is now read in a function of escher's own that iterates every size attribute of the select, as the loop's arm did; the read and its use are unchanged.
@task_item n=2 part=touches
contract/refusal-cause/unknown-verb and contract/refusal-cause/malformed — the tests holding these refusals moved to a child test module of the command validation; the verb table and every refusal cause are unchanged.
@task_item n=3 part=touches
decision/one-span-per-command, decision/driver-emits-no-event and decision/driver-library-installs-no-subscriber — rules over the driver whose command validation module changed; the split adds no span, event or subscriber and the command span is untouched.
@task_item n=4 part=touches
decision/select-chosen-set-never-logged, decision/select-list-label-never-logged and decision/open-list-shot-carries-labels — rules over the select list box and drop-down whose keyboard handling and selected reads moved into escher's own functions; no log site added, labels still carried only as pixels.
@task_item n=5 part=touches
decision/optgroup-label-crossing — the accessibility name read moved into its own function of the tree builder; group name and painted header unchanged.
@task_item n=6 part=touches
contract/snapshot/multiple-select-value, contract/snapshot-node/clickable and contract/snapshot-text/clickable — the snapshot reads accessibility states and names from the builder whose escher reads moved; each read once, values unchanged.
@task_item n=7 part=touches
contract/multiple-select-form-entries/chosen-option — the multiple select key arm moved into escher's own key function in its former order and gates.
@task_item n=8 part=touches
contract/page-events/checkbox-change, contract/page-events/radio-change and contract/page-events/single-change — Enter activation and the control key chain moved into escher's own function before upstream's text-input arm; exactly one acts, as before, so the change count holds.
@task_item n=9 part=touches
contract/snapshot-diff/open-list-row-bounds — the list box row blocks now come from escher's own presentational hint function; the row bounds are unchanged.
@task_item n=10 part=touches
decision/engine-sites-fixed-messages and decision/engine-tracing-feature-gated — rules over the engine bodies the session split; the moved code adds no log site and no tracing call.
@task_item n=11 part=touches
decision/ci-platform-github-workflows and decision/coverage-artifact-on-success — the CI run on the pushed head green 17 of 17 with the dependency audit job success; escher-driver coverage 97.80 % holds above its floor 97 where a row holds that floor.
@task_item n=12 part=touches
decision/committed-kept-checks-clean and decision/committed-checks-hold-no-private-data — the moved command tests hold the same typed calls as before; no new committed fixture.
@task_item n=1 part=untouched
escher's telemetry sink, its log line format, scrub set, target allowlist and RUST_LOG handling, and the escher-telemetry startup and panic events — no sink or telemetry code moved.
@task_item n=2 part=untouched
The tool_call span, its tool and ending fields and the MCP role's stdout and exit statuses — the MCP host is not reached.
@task_item n=3 part=untouched
The cold-agent pipe and the agent-run harness — their events, verdict fields, counted verbs, kept checks, transcripts and cleanup rules.
@task_item n=4 part=untouched
Script error capture — the window error event, window.onerror, script error source labels and the script diagnostics drain.
@task_item n=5 part=untouched
Session recording, the recording file and directory privacy, session lifecycle idle expiry and the session state directory.
@task_item n=6 part=untouched
escher-session's commands, exit codes, stderr, coverage role and host wire.
@task_item n=7 part=untouched
The absences of a telemetry backend, metrics exporters, metric emission and an error-reporting service, and WPT score publishing upstream only — no capability added.
@task_item n=8 part=untouched
The build-directory sweep's stdout line and the package-install diagnostic line — no CI script changed.
