@task_prose part=session
The session added two unit tests to the browser crate and changed no product behaviour, no public, crate-visible or IPC surface, no dependency, no schema and no gate.
The tests pin the private scene-render step of the browser's screenshot capture: it returns the viewport size at scale 1.0 (800 by 600) and the physical size at a hidpi scale of 2.0 (1280 by 720).
They sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) module by path, under the existing convention for out-of-line unit tests.
It measured the 0.3.0 Epoch 6 audit's four surviving scene-render mutants as caught, under hand controls and then under a cargo-mutants run fired on the founder's word, after the run's filter was mended from an unanchored name to the anchored verb form.
A fifth mutant, the capture of the anyrender scene replaced with nothing, read missed outside the founder's word; it is carried as a residual and a next-version incubator line, with no work on it here.
Nothing was retired.
@task_file n=1 path=apps/browser/src/capture.rs
what: the browser's screenshot capture, the driver's screenshot of the page, an element or an open list's row, rendering the scene at the viewport size and the hidpi scale; edited only to declare its unit-test module, no line of capture changed
@task_file n=2 path=apps/browser/src/capture_tests.rs
what: new, the unit tests of the screenshot capture's scene-render size at the viewport size and at a hidpi scale
@task_item n=1 part=touches
decision/screenshot-changes-nothing: a rule over the screenshot capture the session changed by a test module; the tests read the render size alone and change nothing, so the rule stands.
@task_item n=2 part=touches
contract/driver-screenshot/element-shot and contract/driver-screenshot/row-shot: contract items over the screenshot capture; no field, size rule or refusal moved, the tests pin the render size the shots are taken at.
@task_item n=3 part=touches
decision/viewport-scheme-and-zoom-level: the viewport carries scale, 1.0 unzoomed; the new tests render at scale 1.0 and 2.0 and assert the viewport size at 1.0 and the physical size at 2.0, consistent with the row.
@task_item n=4 part=touches
decision/hidpi-or-zoom-change-reshapes-text: a rule over hidpi scale; the session adds a test of the capture's physical size at a hidpi scale, no change to how a scale change reshapes text.
@task_item n=5 part=touches
decision/a11y-assertions-as-rust-tests: the new tests are Rust unit tests, but of capture size, not accessibility assertions over AccessKit; no accessibility assertion was added or moved.
@task_item n=6 part=touches
decision/a11y-test-leg, decision/a11y-ci-leg and decision/a11y-ci-gate: the new tests run in the fast CI leg as browser unit tests, not in the accessibility leg; no leg, tier or gate changed.
@task_item n=1 part=untouched
The accessibility tree, its roles, names, states, hidden and inert content, and the AccessKit adapter: no line of them changed.
@task_item n=2 part=untouched
ARIA property reads, ARIA role mapping and the accessible-name precedence: untouched.
@task_item n=3 part=untouched
Keyboard navigation, focus, Tab order, select lists, range inputs, the seven_guis Cells grid and CRUD list contracts: untouched.
@task_item n=4 part=untouched
Contrast: the lint's contrast finding, the WCAG ratio helper, the stand's contrast assertions and colour constants: untouched.
@task_item n=5 part=untouched
The snapshot, its text form, diff, actionable nodes, the actions answer, the UI graph and MCP tool results: untouched.
@task_item n=6 part=untouched
The accessibility feature flags, the platform adapter choice and the tier-0 reach: no dependency or feature changed.
@task_item n=7 part=untouched
Cognitive accessibility, stand focus-order tables and the stand's three-round repetition: untouched.
