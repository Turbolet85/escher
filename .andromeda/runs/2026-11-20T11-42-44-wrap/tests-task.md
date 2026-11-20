@task_prose part=session
The session edited one line of page text: the 0.3.0 heading of the project's status section now reads done and verified, in place of built, its cold-agent run passed.
It retired that interim 0.3.0 wording; it stood at one tracked site.
The status section now states 3 closed versions in one heading form, done and verified, up from 2 (0.1.0 and 0.2.0).
The founder's word closed the citation backlog as read; no master row was written for it.
No code, test, check, CI job, schema, dependency or harness surface changed; the CI-scripts unittest pin ran green, and CI on the pushed head read green, 17 of 17 checks.
The fork's default branch was read as build/escher-0.4.0, while a harness doc still names build/escher-0.3.0.
@task_file n=1 path=README.md
what: the project's front page, its status section with one heading per version stating that version's state (0.1.0, 0.2.0, 0.3.0 closed, 0.4.0 open)
@task_item n=1 part=touches
A row stating the 0.3.0 version's state as built, interim or awaiting closure, or quoting the wording built, its cold-agent run passed, is now stale: 0.3.0 reads done and verified.
@task_item n=2 part=touches
A row whose cell holds the count of closed versions (2) or the form of a closed version's heading now meets 3 closed versions in the one form, version followed by done and verified.
@task_item n=3 part=touches
Indirect, through the closed 0.3.0 state: the cold-agent live run rows (live run fired once, live run never in CI, real MCP client only in the operator pass) where a row names that run as the last step 0.3.0 still owed or as its closing evidence.
@task_item n=4 part=touches
Indirect, through the closed 0.3.0 state: mutation and version-end audit rows (mutation baseline skipped or read per unit, mutation on the dev host) where a row states a 0.3.0 audit or mutation reading as still owed; the session states none under 0.3.0.
@task_item n=5 part=touches
A row naming the fork's default or build branch as build/escher-0.3.0 (CI trigger branches, cache saved on main and build branches) is stale: the default branch reads build/escher-0.4.0.
@task_item n=6 part=touches
A capability statement or capability claim row where it rests on the front page's status section for a version's closed or open state.
@task_item n=1 part=untouched
The harness contract rows (construction, core, input, inspection, screenshot, settle, contrast): no harness code or behaviour changed.
@task_item n=2 part=untouched
The session lifecycle rows (session wire, session host, session errors, session recording, session state directory, escher-session command line and exit status): untouched.
@task_item n=3 part=untouched
Driver verbs, refusal causes, lint defects, snapshot and MCP server or MCP role rows: no driver or MCP surface moved.
@task_item n=4 part=untouched
Coverage floors, the cognitive-complexity ceiling, oversize-file and upstream-kept rulings, and the zero-failure stand-repeat budget: no threshold, floor or bound moved.
@task_item n=5 part=untouched
CI integration rows on legs, caches, package installs, apt-install and coverage-floor exit codes, clippy, MSRV and dependency audit: no CI job or script changed.
@task_item n=6 part=untouched
WPT runner, status set, expectations and pinned commit rows, and the test data, fixtures and mocking discipline rows: untouched.
@task_item n=7 part=untouched
Agent-run and cold-agent contracts (verbs, events, exit codes, verdict fields, state, invocation): no pipe or verdict changed.
