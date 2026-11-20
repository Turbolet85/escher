@task_prose part=session
The session restated one heading in the project README: the 0.3.0 version's state moved from built, its cold-agent run passed, to done and verified.
The interim wording built, its cold-agent run passed is retired; the README's Status section now states three closed versions in one form (0.1.0, 0.2.0, 0.3.0), up from two.
No code, symbol, API, dependency, schema, config, CI workflow, harness or gate surface changed; no new external surface, hot-path operation or UI element.
Measured: a secret search over the README reads 0 hits; the CI-scripts unittest pin is green; CI on the pushed HEAD reads verdict green, checks 17/17.
Read at the operator pass: the fork's default branch is build/escher-0.4.0, and GitHub's readme endpoint serves the new heading.
@task_file n=1 path=README.md
what: the project README, the page text whose Status section states each version's state (0.1.0, 0.2.0, 0.3.0 done and verified, 0.4.0 open); it holds no code, config or secret
@task_item n=1 part=touches
Any row stating the README's per-version state, or the 0.3.0 state as planned, built or interim (built, its cold-agent run passed): it now reads done and verified, three closed versions in one form.
@task_item n=2 part=touches
Any row naming the fork's default branch as build/escher-0.3.0 (a publish job repository guard, publish secrets reachable only upstream, release publishing only from upstream, read-only CI tokens, where a cell names the branch): the default branch reads build/escher-0.4.0.
@task_item n=3 part=touches
Cold-agent rows (cold-agent runs carry the screen to the provider, the cold-agent wrong-call bar, cold-agent task as a fixed prompt) only where a row cites the README's 0.3.0 heading or its cold-agent run passed wording as evidence: that wording is retired; the cold-agent run itself is unchanged.
@task_item n=4 part=touches
rdme passes raw HTML through: the README is page text rdme renders; the change is one plain Markdown heading and adds no raw HTML.
@task_item n=5 part=touches
Rows that hold that no credential or secret stands in committed text (live evidence holds no credentials): the secret search over the README reads 0 hits.
@task_item n=6 part=touches
CI matrix runs every job, only where a row's cell holds a count of CI checks: the pushed HEAD reads checks 17/17, verdict green.
@task_item n=1 part=untouched
The driver session socket, its closed grammar, verbs, refusal causes, ids, replies and exit codes: no driver code or contract changed.
@task_item n=2 part=untouched
The MCP server, its stdio transport, request and result bounds, protocol errors and tool results: unchanged.
@task_item n=3 part=untouched
Input validation of page content, markup, select lists, masked values, stable ids and recorded or kept checks: no engine or session code changed.
@task_item n=4 part=untouched
Outbound requests, fetch schemes, per-host limits, CORS and same-origin absences, CSP and the CDP devtools server: unchanged.
@task_item n=5 part=untouched
Dependency security: no dependency, lockfile pin, audit ignore, licence gate or CI action pin moved.
@task_item n=6 part=untouched
Secret management and environment variables: no environment variable read, CI secret or signing step changed.
@task_item n=7 part=untouched
Logging and monitoring, the telemetry sink allowlist, span lines and stdout or stderr contracts: unchanged.
@task_item n=8 part=untouched
Data protection: history, cookie store, HTTP cache, kept check files and recordings unchanged.
@task_item n=9 part=untouched
Error handling rows, session error tables, CSSOM and WPT result contracts: unchanged.
