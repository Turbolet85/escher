@task_prose part=session
The session added a way to render one component alone, with given props, at the headless stand's pinned viewport and font, and read it as inside its app.
The test harness gained a public constructor from a component and a props maker, plus a public mount of the component under an anonymous root; the stand gained an isolated boot that seeds the Timer's tick context on that mount.
Measured: the shell alone with the stand's props reads the stand's Counter boot node for node and an equal UI graph; the Counter alone reads its four in-app accessibility subtree rows with the shell's positional id prefix removed, roles, names and states equal; a click moves the counter as on the stand.
Measured: the isolated document's viewport reads 800 x 600 at scale 1, but its root element sizes to its content, since the shell's CSS is absent; a Timer alone reads its ticks only from the root context.
The cold-agent pipe's verb list went from fourteen to fifteen, the layout read joining as an uncounted read beside graph and actions; no driver verb, carrier, MCP tool, span or log field was added and no driver or session host file changed.
A unit test confirmed a vdom rebuilt in place reads one root error boundary mark through the fallback; nothing was retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_component_alone.rs
what: a new stand check, one component rendered alone and read against the stand's expectation tables (Counter rows, ids, roles, names, states, bounds, UI graph), run in both layout modes and joining the stand repeat leg by its stand_ name
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: the test harness and its construction — the new isolated constructor from a component and a props maker, the anonymous-root mount, and the harness's first unit test reading the UI graph
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs
what: the test harness crate's public surface, re-exporting the isolated mount
@task_file n=4 path=packages/blitz-test-harness/Cargo.toml
what: the test harness crate's manifest — its test build now turns on the dioxus document's accessibility feature so a harness unit test can read the UI graph
@task_file n=5 path=examples/seven_guis/src/stand.rs
what: the headless stand's boots — a new isolated boot of one seven_guis component alone with the Timer's tick context, every existing boot unchanged
@task_file n=6 path=packages/dioxus-native-dom/src/boundary.rs
what: the dioxus document's error boundary registry — a new unit test that a document built after the first build reads the root boundary's error
@task_file n=7 path=scripts/cold-agent.sh
what: the cold-agent pipe's verb list, now fifteen with layout after actions
@task_file n=8 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests and fake server — the verb list at fifteen, layout answered as an uncounted read, and a test that a layout call is no act
@task_file n=9 path=docs/capabilities.md
what: the public capability statement — a new subsection on reading a component alone with its witnesses line; the accessibility reach statement is not edited
@task_item n=1 part=touches
decision/stand-expectation-tables and decision/role-name-tables-stated-once — a new stand check reads the Counter's expected rows (ids, roles, names, states, bounds) for a component alone; say whether the stated-once tables must name this isolated reading or whether the check's own Counter row table is a second statement of them.
@task_item n=2 part=touches
decision/ui-graph-one-node-per-snapshot-node and decision/snapshot-model-single-source — the shell alone reads an equal UI graph to the stand's Counter boot, and the Counter alone reads its in-app snapshot rows with the shell's positional id prefix removed; the snapshot and graph model now has an isolated reading.
@task_item n=3 part=touches
decision/agent-readers-behind-accessibility and decision/dioxus-accessibility-feature-by-name — the harness crate's test build now turns the dioxus document's accessibility feature on, by name, as a dev-dependency, because the UI graph reader is gated by that feature and a per-package test build would otherwise build it off.
@task_item n=4 part=touches
decision/accessibility-feature-default-on and decision/browser-accessibility-off-by-default — a per-package harness test build builds the dioxus document without accessibility unless the dev-dependency turns it on; check whether either row's statement of where the feature is on holds for a lone package's test build.
@task_item n=5 part=touches
decision/stand-checks-repeated-three-rounds and decision/a11y-test-leg — a new stand_ integration target joins the stand run and the stand repeat leg by its glob, so the repeated stand checks and the accessibility test leg now cover one more target.
@task_item n=6 part=touches
decision/a11y-assertions-as-rust-tests — the isolated reading is asserted as Rust tests over the accessibility tree and snapshot in both layout modes, with two controls holding.
@task_item n=7 part=touches
decision/viewport-scheme-and-zoom-level — the isolated document keeps the stand's pinned 800 x 600 viewport at scale 1 and its font, read through the document's viewport, while the root element's box reads content height, not 600.
@task_item n=8 part=touches
decision/actions-verb-changes-nothing and decision/screenshot-changes-nothing — the cold-agent pipe now exposes the layout read as a fifteenth tool, uncounted like graph and actions, and its contract test asserts a layout call is no act; any row listing the pipe's reads or tools at fourteen is stale.
@task_item n=9 part=touches
decision/harness-settle-leaves-changed-set and contract/ui-graph-state/use-graph-state — the harness gained a constructor that rides the existing build-from-vdom path; a rule over harness construction or graph state registration now covers a component mounted under an anonymous root, its props made once in that root's scope.
@task_item n=10 part=touches
contract/accessibility-tree/root — the root of an isolated document is an anonymous root holding the one component, named by its type path; check whether the tree root contract assumes the app's shell is the root.
@task_fact n=1 slug=screen-reader-support
fact: A component rendered alone with given props at the stand's viewport and font reads as inside its app — its accessibility subtree's roles, names and states equal the in-app rows, its ids those rows with the shell's positional prefix removed, and its UI graph equal — and this isolated reading is in-process only, on no carrier.
@task_fact n=2 slug=visual-design-verification
fact: A component rendered alone has no shell CSS, so its root element sizes to its content; a check of the pinned 800 x 600 viewport in an isolated document reads the document's viewport, never the root element's bounds.
@task_item n=1 part=untouched
Keyboard navigation contracts and decisions — Tab order, focus, keyboard activation, the select lists, the range input and the Cells and CRUD grids — no focus or key handling changed.
@task_item n=2 part=untouched
Visual contrast rows — the lint's contrast finding, the WCAG ratio helper, stand contrast and its exemptions, scrollbar and select list colours — no colour, style or lint changed.
@task_item n=3 part=untouched
ARIA state and property reads, role mapping and hidden or inert content exclusion — the tree builder's readings are unchanged.
@task_item n=4 part=untouched
The snapshot text form, snapshot diff, driver screenshot, driver refusals and MCP result contracts — no carrier, verb answer or reply field was added or changed.
@task_item n=5 part=untouched
Accessibility tier and platform adapter scope — no platform adapter, assistive technology reach or tier change; the stand binaries still build no adapter.
@task_item n=6 part=untouched
Cognitive accessibility and the obs log format — no invalid-state display, timeout or log line changed.
