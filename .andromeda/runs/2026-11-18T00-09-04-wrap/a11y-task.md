@task_prose part=session
The session is a test-only sweep: it shows every 0.3.0 capability is proven headless on the stand or a fixture, by a check that runs inside the stand-repeat CI leg.
It added a 0.3.0 requirement table to the stand requirement sweep: sixteen rows, each pinned to its witness test by name, with v030-02, v030-03 and v030-16 alone naming no witness.
The sweep's leg check now reads only the stand-repeat arm of the CI leg script, and its named-target list grew from ten to seventeen, matching the arm.
It added a stand mask check: in both layout modes, graph, actions and layout answers, a kept check and every explored path's check hold no typed password or file path, and over MCP a file input's value reads the mask.
It added five unit tests on the browser's document config (root shell provider, HTML parser provider, font context, abort signal, parallel style).
No public symbol, verb, feature, dependency, schema, CI script or gate changed; stand-repeat stayed green 3 of 3 rounds and every coverage floor held.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the stand requirement sweep, the check that every requirement names its witness inside stand-repeat, now holding the 0.3.0 table and reading the stand-repeat arm's named targets
@task_file n=2 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: a new stand check that the password and file-input masks hold on the graph, actions and layout answers, on kept and explored-path checks, and on the file input's value over MCP
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser's document loader, the browser binary's document config; only its unit tests changed, the config itself did not
@task_item n=1 part=touches
Password value masked and password text is never revealed: a new stand check now asserts the typed password appears zero times in graph, actions and layout answers, kept checks and explored-path checks, in both layout modes. This is a new guard beside those rules, and the rules themselves do not change.
@task_item n=2 part=touches
Snapshot file input value: over MCP, a file input's value now reads the mask, and a new stand check guards this. If a row's guard names only the earlier mask checks, it is now missing one.
@task_item n=3 part=touches
Snapshot password input value: a kept check's recorded type step reads the stand-in mask and replays green. A guard that lists the mask checks gains the new target.
@task_item n=4 part=touches
UI graph is read-only and carries no values: the new check asserts the graph answer holds no masked value. A guard that lists checks on this rule gains the new target.
@task_item n=5 part=touches
The actions answer fields and the actions verb changes nothing: the new check reads the actions answer for masked values. This is a guard addition only; the fields are unchanged.
@task_item n=6 part=touches
Stand checks repeated three rounds: the sweep checks the stand-repeat arm's seventeen named targets, a stand_ file counts only where the arm holds the glob, and seven_guis targets count only on the arm's seven_guis line. If a row's cell holds a count of named targets (ten), it now reads seventeen.
@task_item n=7 part=touches
A row whose guard names the stand requirement sweep: the target still stands under its name, and its test count went from three to four. This is a touch only if the cell holds that count.
@task_item n=8 part=touches
Accessibility test leg in the slow tier and accessibility checks gating CI: stand-repeat runs one more stand_ file (64 in all), the new mask check. Neither the leg script nor its gate changed.
@task_item n=9 part=touches
MCP tool results and the snapshot through MCP: the file input's value is now asserted through the served MCP session. This adds a guard; the wire shape is unchanged.
@task_item n=10 part=touches
The keyed field on the carriers: in the new mask check, a `caused` item sits outside the mask by a ratified exception, and no hosted acting call returns one. This touches a row only if it rules on what a carrier masks.
@task_item n=11 part=untouched
ARIA role mapping, accessible-name precedence, ARIA state and property reads, and the accessibility tree states: the session changed no reader, role or state rule.
@task_item n=12 part=untouched
Keyboard navigation, focus, Tab order, range input, select open and closed list keys, and click activation: no event or focus rule moved.
@task_item n=13 part=untouched
Visual design: contrast, the lint, the select list's colours, scrollbars and zoom are unchanged; no visual check or threshold moved.
@task_item n=14 part=untouched
A11y scope: the accessibility feature defaults, the AccessKit adapter, the platform-adapter absence on the stand binaries, and the tier-0 reach statement are unchanged.
@task_item n=15 part=untouched
Hidden and inert content exclusion, the snapshot text form, the snapshot diff, node ids, and the screenshot contracts: none of these is reached.
