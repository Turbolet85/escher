@task_prose part=session
The session added test code only, two unit tests in the browser app crate for its private scene-rendering helper, which returns the rendered size: one at the viewport size, one at the physical size under a 2.0 HiDPI scale.
The tests sit out of line in a sibling file with the _tests suffix, declared at the end of the capture module as a cfg(test) child module by path; no public, crate-visible or IPC surface changed, and no dependency, schema, CI workflow or gate surface moved.
It measured the four render_scene mutants the 0.3.0 Epoch 6 mutation audit left surviving: all four are now caught, under hand controls and under a host-run cargo-mutants run fired on the founder's word with the nextest mutants profile (4 caught).
A fifth mutant, the scene-capture function emptied, read missed when an unanchored filter reached past that word; it is carried as a residual row and a next-version incubator line, with no code changed for it.
CI read green on the pushed commit; nothing was retired.
@task_file n=1 path=apps/browser/src/capture.rs
what: the browser app's capture module, which renders a document's scene through anyrender and returns its size in physical pixels; edited, a cfg(test) unit-test module declaration appended at its end
@task_file n=2 path=apps/browser/src/capture_tests.rs
what: the browser app's capture module's unit tests, out of line in a sibling _tests file, a child test module of the capture module; new
@task_item n=1 part=touches
Unit-test placement conventions (unit tests at the end of the source file, unit-test placement, inline cfg(test) modules): the session uses their stated exception, tests out of line in a sibling _tests file declared as a cfg(test) module by path; a picker should confirm the exception still reads true and covers app crates, not only engine crates.
@task_item n=2 part=touches
Split test modules stay children of their module and moved unit-test modules keep their test paths: the new tests are a child module of the capture module and read as capture::tests paths.
@task_item n=3 part=touches
Crate internals restricted: the tests reach the capture module's private rendering helper from a child module, with no visibility widened.
@task_item n=4 part=touches
Descriptive test names: the two new tests are named for the behaviour they hold (returns the viewport size; returns the physical size at a HiDPI scale).
@task_item n=5 part=touches
Host-run mutation tier, nextest mutants profile and demo shells outside the mutation tier: the session fired a mutation run on the browser crate on the dev host under the mutants profile, its baseline read green; a row that names which crates the tier covers, or excludes, is touched if the browser app is among them.
@task_item n=6 part=touches
Per-crate coverage floors and the coverage-floor decisions: the browser crate gains two unit tests; touched only where a row's cell holds the browser crate's floor value.
@task_item n=7 part=touches
anyrender as the rendering abstraction: the tests render through anyrender's null scene painter, an existing dependency, with no backend added.
@task_item n=8 part=touches
Integration tests live in blitz-tests and one test target per file in blitz-tests: the new tests are unit tests inside an app crate, not integration targets; a picker should check no row implies all tests of the browser app live elsewhere.
@task_item n=9 part=touches
Dependencies declared once in the workspace and locked cargo legs: no dependency added; the tests ran locked, within the fast CI leg.
@task_item n=10 part=touches
Viewport scale in the provider traits: the HiDPI test reads the physical size as the viewport size times its scale (1280 by 720 at 2.0); a row stating how the viewport scale maps to physical pixels is reached indirectly.
@task_item n=1 part=untouched
Engine layout, style, paint-tree, select-list, scrolling, hit-test and accessibility rows: no engine crate changed.
@task_item n=2 part=untouched
The driver, session socket, MCP role, kept checks, cold-agent and command-line contracts: no carrier, verb, answer or exit code changed.
@task_item n=3 part=untouched
Browser history, persistence, about pages, toolbar and URL suggestion rows: the session reached none of them.
@task_item n=4 part=untouched
Telemetry, tracing and logging rows: no span, field or subscriber changed.
@task_item n=5 part=untouched
WPT runner rows and contracts: no WPT tooling, suite or output changed.
@task_item n=6 part=untouched
CI workflow, actions pinning, cache, permissions, cargo-deny and the ci-leg legs: no workflow, leg or gate changed; the CI wait that ran past its bound is a plan setting, not a row's.
@task_item n=7 part=untouched
Names, absences and occupied resources: no crate, binary, environment variable, port or listener taken or freed.
@task_item n=8 part=untouched
Project intent, upstream sync, versioning and the capability statement: nothing released, merged or restated.
