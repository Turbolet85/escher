@task_prose part=chunk
Re-asserts three WCAG 2.2 success criteria on the 7GUIs stand and the pages the escher-session binary serves, at HEAD, after the 0.3.0 engine changes: SC 2.1.1 Keyboard, SC 2.4.3 Focus Order, SC 1.4.3 Contrast (Minimum).
Re-reads the existing stand keyboard and stand contrast checks and the a11y fixtures, and extends them only where a 0.3.0 surface is unasserted: the engine debts (checkbox and radio change, multiple select keys, row clicks and size, optgroup), the engine-painted select list, inert and disabled by presence, and the bundled DejaVu Sans fonts.
Every re-assertion runs in the a11y CI leg, and a stand-facing one also in stand-repeat; the leg pins follow.
Pins the carried SC 2.4.3 limit as a known gap: a focused control hidden by the hidden attribute, display:none or CSS keeps focus and takes keys, the focus fixup reading inert alone.
Widens no engine behaviour; adds no driver verb, carrier, argument, field, log field, span or crate dependency.
@task_file n=1 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand keyboard check — SC 2.1.1 keyboard reach and operation and SC 2.4.3 Tab and Shift+Tab focus order on each stand screen and served page — edits: its keyboard and focus-order assertions, extended to the 0.3.0 surfaces left unasserted
@task_file n=2 path=tests/blitz-tests/tests/stand_contrast.rs mark=edits
what: the stand contrast check — SC 1.4.3 text/background contrast pairs the stand paints, read through the contrast reader's luminance — edits: its contrast assertions, extended to the 0.3.0 surfaces left unasserted and read with the bundled fonts
@task_file n=3 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script, home of the a11y test leg and the stand-repeat named list run three rounds — edits: the a11y leg's --test list; the stand-repeat named list if a new non-stand_ witness joins it
@task_file n=4 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow pins test — the legs' pins over the CI legs and their test lists — edits: the legs' pins
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests targets pins test — which test targets each CI leg runs — edits: the legs' pins
@task_item n=1 part=needs
SC 2.1.1 Keyboard on the stand and served pages: every control an agent or a person can act on is reached and operated by keyboard alone — Enter activating the focused control, Space release activating it, the Cells grid keyboard model, range key actions where a page has one.
@task_item n=2 part=needs
SC 2.4.3 Focus Order: Tab and Shift+Tab walk each screen's stated sequence and wrap, and focus lands somewhere meaningful after an open select list closes, after a multiple select's keys, and after a control turns disabled, hidden or inert — inert focus clear firing no blur, disabled read by presence on form controls alone.
@task_item n=3 part=needs
The carried focus limit (founder's F1 A): a focused control that stops showing through the hidden attribute, display:none or CSS alone keeps focus and takes keys because the focus fixup reads inert only; the check reads it as it stands and names it a known SC 2.4.3 gap, never removes it.
@task_item n=4 part=needs
SC 1.4.3 Contrast (Minimum): every text/background pair the stand paints meets 4.5:1, or 3:1 for large text, read with the bundled DejaVu Sans fonts and system fonts off — glyph metrics, the large-text threshold and the reader's sampled text may have moved; luminance comes from the color crate as the contrast reader's source.
@task_item n=5 part=needs
The engine-painted select list over the page: opened by click, Enter or Space; its closed key match; Tab choosing and closing before focus moves; at most twenty rows; closing when its select becomes unfit; its border, highlight and disabled-text colours the engine's own constants, read for contrast as painted pixels.
@task_item n=6 part=needs
The engine debts the checks now reach: the engine firing change on checkbox and radio, the multiple select's keys and row clicks, size setting a multiple select's rows, optgroup options and label, the one option-disabled rule.
@task_item n=7 part=needs
Inert subtree and disabled by presence: an inert or disabled control is skipped by Tab, takes no keys and is refused as disabled where a driver call names it; the focus fixup acts on inert.
@task_item n=8 part=needs
Served pages: the six tasks the escher-session binary serves (Counter, Flight Booker, Timer, CRUD, Cells and the load page) reached through the existing driver session and its existing verbs; which of them the checks leave out is closed by P3.
@task_item n=9 part=needs
Engine behaviour is re-asserted, not widened: a failure found at HEAD is fixed here only inside an already ratified engine behaviour; a fix changing behaviour for every document without a ruling is the founder's fork (P4) under the boundary widening rule.
@task_item n=10 part=needs
The closed driver schema stays as it is: no verb, carrier, argument kind, answer field, command span field or log field is added for these checks.
@task_item n=11 part=needs
No new crate dependency: the checks keep to the workspace's existing dependency set, the test harness taking no tracing and no style, builds locked.
@task_item n=12 part=needs
The checks run in the a11y CI leg and, stand-facing ones, in stand-repeat's three rounds; the a11y leg's --test list and the legs' pins agree with the targets the leg runs.
@task_item n=13 part=needs
Masked values hold under the new reads: a password or file input on a served page reads only its masked stand-in, painted as bullets, in any answer or shot a check reads.
@task_item n=14 part=needs
Screen content stays out of logs: a check reading labels, select list row labels, accessible names or stable ids prints none of them to a log line.
@task_item n=15 part=needs
CI read at take-up: the base 712e09b3c's verdict was still in progress and is re-read by P3; watches carried — a job cancelled with no runner acquired, re-run and not fixed; the coverage report red on a corrupt instrumentation profile at the merge, green on re-run with no edit, under the per-crate coverage floors.
@task_standing n=1
convention: each new test or fixture is a test-only target in the unpublished blitz-tests crate, takes no new dependency, no tracing and no style, and builds locked
home: dependency-security
@task_standing n=2
convention: each new test, fixture or log line keeps page content, labels, accessible names, stable ids and masked values out of every log and printed output
home: logging-monitoring
@task_standing n=3
convention: each new CI job or leg step runs under read-only tokens, actions pinned to commit SHAs, and every matrix job runs
home: authentication-authorization · dependency-security
@task_item n=1 part=not_needed
Data protection rows over browser history in sqlite, cookies, the on-disk HTTP cache, TLS, clipboard and file dialog, and kept check files: the chunk stores nothing and keeps no check file.
@task_item n=2 part=not_needed
API security rows over outbound HTTP, per-host limits, CORS and same-origin, request timeouts and response size limits: the stand and served pages are local and the chunk adds no fetch.
@task_item n=3 part=not_needed
Secret management rows over CI secrets, signing material and the operator's login: the chunk touches no secret.
@task_item n=4 part=not_needed
Release and publish rows: the publish job guard, publish secrets upstream-only and release publishing — the chunk publishes nothing.
@task_item n=5 part=not_needed
WPT rows: the WPT runner, its quarantine entries, results and post-results workflow — not part of the a11y re-read.
@task_item n=6 part=not_needed
MCP protocol error rows, the MCP initialize revision and the MCP tool-call span: the checks do not go through the MCP role.
@task_item n=7 part=not_needed
Cold-agent run rows and the recording of checks (recorded check files, recording directory, replay): the chunk records and replays nothing.
@task_item n=8 part=not_needed
The windowed assistive-technology witness through the accessibility adapter: it stays the founder's residual F1, out of scope.
@task_item n=9 part=not_needed
Script-engine rows: JavaScript opt-in, JS fetch methods, CSSOM script API errors, JS module loading and the URLSearchParams bootstrap — the stand's checks run no page script.
@task_item n=10 part=not_needed
Dioxus native bridge rows and rdme raw HTML passthrough: neither is the stand nor a served page.
