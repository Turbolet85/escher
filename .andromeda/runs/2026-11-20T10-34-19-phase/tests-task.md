@task_prose part=chunk
This chunk changes one line of the README: the 0.3.0 version heading now reads «0.3.0 — done and verified», in the same form as the 0.1.0 and 0.2.0 headings, instead of the interim «built, its cold-agent run passed».
It adds no sentence about the version-end audit or the unfinished mutation tier, and it rewrites no other part of the README.
On the founder's word, it closes the citation backlog as read through and proves capability v040-03, "The version on the GitHub page".
It changes no engine, driver, stand or test code, adds no test, check or CI job, and writes no residual, master or harness file.
@task_file n=1 path=README.md mark=edits
what: the project's public statement on the GitHub page — what each version builds and its stated state, including the capability statement's count of driver commands as tools — edits: the 0.3.0 version heading line
@task_item n=1 part=needs
The README's 0.3.0 version heading changes from the interim state to «done and verified», in the form the 0.1.0 and 0.2.0 headings take. No sentence about the audit or the mutation tier is added anywhere in the 0.3.0 section.
@task_item n=2 part=needs
Indirect, through the capability-statement check that reads the README: the edit must leave the README's count of driver commands as tools exactly as it is. Only a version heading changes, and no test or CI pin may be moved by it.
@task_item n=3 part=needs
Indirect, through the README's other sections: the 0.3.0 section's body, the 0.1.0 and 0.2.0 sections and the «0.4.0 — planned, in six stages» section are not rewritten. The headings of 0.1.0 and 0.2.0 stay the form that the new 0.3.0 heading copies.
@task_item n=4 part=needs
The chunk is proved by reading the edited heading line back and by the existing CI legs staying green on its commit. It adds no new test, test file, fixture, CI job or log line.
@task_item n=5 part=needs
Indirect, through the CI trigger and the fast/slow CI split: a documentation-only commit on the build branch still runs every CI leg. The verdict on that commit is read the same way as for any change.
@task_item n=1 part=not_needed
The harness contract rows (harness construction, core stepping, settle, screenshot, contrast, input and inspection helpers). The chunk touches no harness code.
@task_item n=2 part=not_needed
The session lifecycle, session wire, session host, session errors and driver session contracts. No driver or session code changes.
@task_item n=3 part=not_needed
The escher-session command line, MCP role, exit status and stderr contracts, and the driver's refusal causes. The chunk adds and changes no verb, tool, cause or output.
@task_item n=4 part=not_needed
The cold-agent pipe and agent-run rows (events, verbs, exit codes, verdict, isolation, live run). The chunk fires no agent run and changes no harness script.
@task_item n=5 part=not_needed
The WPT runner, conformance, status set and pinned-commit rows. No WPT suite, runner or expectation is touched.
@task_item n=6 part=not_needed
The mocking and stubbing discipline rows (hand-written stand-ins, network stand-ins, timer ticks, virtual clock advance). The chunk writes no stand-in or timed check.
@task_item n=7 part=not_needed
The test data and fixtures rows (shared fixture pages, committed recorded checks, stand load page). The chunk adds or changes no fixture or recorded check.
@task_item n=8 part=not_needed
The integration and E2E test strategy rows about stand checks, layout modes, handler coverage, lint defects and snapshots. No stand behaviour or test target changes.
@task_item n=9 part=not_needed
The mutation testing and oversize-file rows of the quality gates (mutation baseline, dev-host mutation runs, oversize targets and upstream files kept). The chunk adds no sentence about the mutation tier and moves no file.
@task_item n=10 part=not_needed
The unit test strategy rows about inline test modules, table-driven tests, literal expectations, regression-test naming and red-before-green control. The chunk writes no test.
