@task_prose part=session
A behaviour-preserving refactor in the Epoch 2 corrective: escher's own additions moved out of three upstream blitz-dom function bodies into private functions of escher's own, one call each in upstream's body.
The accessibility node builder now calls one escher function for its ARIA states (expanded, name, selected, toggled, invalid); the keyboard event handler calls one function for its control-key chain (range, single list box, drop-down, multiple select, Enter activation) and one that closes an open select list before Tab; the presentational-hints pass calls one function for a list-box select's size height and overflow.
The escher-driver command module's inline unit tests moved whole into a child test file declared by a path attribute; the seven tests keep their command::tests paths and no production item moved.
Measured: command module 848 to 249 code lines; cognitive complexity of the three upstream functions 50 to 13, 58 to 46, 176 to 167, the highest fork-own function 14; escher-driver line coverage 97.97 to 97.80 percent against its floor 97; stand duplication 44 pairs and 372 lines under the 47 and 550 bar.
The lock moved url 2.5.8 to 2.5.9 and form_urlencoded 1.2.2 to 1.2.3, closing advisory RUSTSEC-2026-0592 with no audit ignore added; on the operator's ruling the bindgen and cssparser-macros syn edges stay at 3.0.6, because the lock cargo update -p url writes does not build.
No public symbol, schema, config, CI workflow or dev-tool version changed; nothing retired.
@task_file n=1 path=packages/escher-driver/src/command.rs
what: the driver's command validation, edited: validate, validate_session and call_of, the verb table and the call-to-command rules; its unit-test module now a path-declared child
@task_file n=2 path=packages/escher-driver/src/command_tests.rs
what: new file, the driver command module's unit tests moved whole (admitted calls, unknown verb refusal, per-level verb checks, malformed call refusal and the first broken rule, every verb reaching its own spec)
@task_file n=3 path=packages/blitz-dom/src/accessibility.rs
what: edited, the blitz-dom accessibility tree builder: the accessibility node's role, name and ARIA state attributes (expanded, selected, toggled or checked, invalid, label)
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs
what: edited, the blitz-dom keyboard event handling: key presses and text input, Tab with an open select list, range stepping, select list box and drop-down keys, multiple select keys, Enter activation
@task_file n=5 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: edited, the presentational hints pass: legacy attributes cascaded as hints, including a select's size as a height hint and a single list box's overflow and row blocks
@task_file n=6 path=Cargo.lock
what: edited, the workspace's dependency lock: url and form_urlencoded bumped, the syn edges of bindgen and cssparser-macros held at 3.0.6
@task_item n=1 part=touches
decision/cognitive-complexity-ceiling: the three upstream functions now stand under the ceiling's handling with escher's additions in functions of its own (scores 13, 46, 167; every fork-own function at most 14); the row stays true, the new rule beside it is F1.
@task_item n=2 part=touches
decision/each-aria-attribute-read-once and decision/aria-state-values-exact-lowercase: the ARIA reads moved out of the node builder into escher functions; each attribute is still read once across the builder and those functions, values unchanged.
@task_item n=3 part=touches
Accessibility tree contracts over states read by the moved code: contract/accessibility-tree/aria-expanded, aria-selected, aria-invalid, state-attributes, select-expanded, name-sources, native-control-toggled, checked-state, non-checkable-role and decision/a11y-aria-states, decision/toggle-read-per-role, decision/mixed-toggle-reads-unchecked, decision/aria-invalid-unknown-is-true: behaviour unchanged, the reading now in escher's own functions.
@task_item n=4 part=touches
decision/moved-unit-tests-keep-paths, decision/split-test-module-as-child, decision/driver-unit-tests-in-path-children, decision/unit-test-placement and decision/unit-tests-at-file-end: the driver command module's tests became a path-declared child test file keeping their command::tests paths; a new instance of the convention, an exception to tests at the end of the source file.
@task_item n=5 part=touches
Keyboard rules over the moved control-key chain: decision/select-keyboard-model, decision/range-keyboard, decision/keyboard-activation-enter, decision/multiple-select-row-keys, decision/select-list-tab-chooses-and-closes, contract/select-list-default-actions/tab, contract/select-list-default-actions/open-keys, contract/dom-events/range-input-default, contract/event-dispatch/enter-click: same arms, order and gates; the chain now runs before upstream's text-input arm and returns early on a text-input element.
@task_item n=6 part=touches
decision/select-size-hint, decision/single-select-list-box and decision/legacy-attributes-as-pres-hints: the select size height hint, the single list box's overflow-y auto and its row blocks now come from one escher function after the hints loop; it reads every size attribute of the select, not only the first.
@task_item n=7 part=touches
decision/direct-engine-changes-flagged-upstreamable, decision/select-list-engine-change, decision/select-model-engine-change and decision/a11y-input-focus-engine-changes: escher's direct engine changes in the three bodies are now isolated as one call each in upstream's text.
@task_item n=8 part=touches
decision/per-crate-coverage-floors, decision/per-crate-coverage-floor and decision/coverage-floors: escher-driver coverage reading moved 97.97 to 97.80 percent; the floor 97 these rows hold stands and holds.
@task_item n=9 part=touches
decision/dependency-and-licence-audit, decision/cargo-deny-advisory-gate and decision/cargo-deny-audit: advisory RUSTSEC-2026-0592 closed by url 2.5.9, no ignore added, the audit leg reads advisories ok and licenses ok.
@task_item n=10 part=touches
decision/locked-cargo-legs and decision/cargo-legs-locked: the committed lock is not what cargo resolves for a url update (syn edges held at 3.0.6); every locked leg builds on it; a later url update re-breaks the build until bindgen and prettyplease agree on syn.
@task_item n=11 part=touches
decision/workspace-dependencies, decision/workspace-shared-dependencies and decision/no-git-dependencies: the bump touched the lock alone, no manifest and no dependency added.
@task_item n=12 part=touches
decision/validated-commands-only, contract/verb-table/order, contract/validation-order/first-failure and contract/driver-session/command-schema: the command module's production items unchanged; only their unit tests moved file.
@task_item n=13 part=touches
decision/spec-url-citations and decision/rustdoc-warnings-are-errors: the moved spec links are now doc comments with angle-bracketed URLs to satisfy rustdoc bare_urls, text unchanged.
@task_item n=14 part=touches
decision/upstream-sync-by-two-parent-merge and decision/upstream-sync-merge-base: upstream's three function bodies now differ from upstream's text only by escher's call lines, which narrows future merge conflicts in them.
@task_fact n=1 slug=stack-and-technologies
fact: A fork addition to an upstream function over the cognitive complexity ceiling goes into a function of escher's own, with one call in upstream's body (as push_inline_box already is); the founder's ruling, beside the ceiling decision.
@task_fact n=2 slug=stack-and-technologies
fact: The workspace lock holds the syn edges of bindgen 0.73.2 and cssparser-macros 0.7.1 at syn 3.0.6, a state cargo update -p url does not produce, because bindgen fails to compile against syn 2.0.119 beside prettyplease 0.3.0; the operator's ruling, an exception to a lock that stays what cargo resolves.
@task_item n=1 part=untouched
Telemetry, tracing spans, log scrubbing and the telemetry line contracts: no span, field or target changed.
@task_item n=2 part=untouched
Paint, the paint scene, scrollbar and select list painting, layout through Taffy and Stylo traversal: no paint or layout code changed beyond the presentational hints already named.
@task_item n=3 part=untouched
The session socket wire, MCP stdio server, recording directory, kept checks and the escher-session command line contracts: no carrier, role or exit code changed.
@task_item n=4 part=untouched
The snapshot, snapshot text, diff, UI graph, actionable and stable element id contracts: no reader or written form changed.
@task_item n=5 part=untouched
WPT runner, browser app, browser persistence, script runtime and Dioxus bridge rows: none of their code was reached.
@task_item n=6 part=untouched
The names sheet: no crate, binary, server or environment name taken or freed.
@task_item n=7 part=untouched
CI workflow structure, ci-leg legs, action pins, caches and the Nix flake: no harness or gate surface changed in the project.
@task_item n=8 part=untouched
Project intent, the capability statement and the root README: no capability added or removed.
