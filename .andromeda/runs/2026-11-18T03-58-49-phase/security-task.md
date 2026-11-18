@task_prose part=chunk
Brings Escher's committed capability statement to 0.3.0: its opening and its stated version, every capability 0.3.0 adds (the UI graph, the actions read, handler coverage, starting in a state, exploring an app, a component alone, why an element sits where it does, errors with the action that caused them, seeded randomness, bundled fonts), each claim naming its witness, and 0.3.0's stated limits, each with how it stands and its owner.
Re-states the accessibility leg as eighteen files on a 0.3.0 run and adds the hidden-focus limit (a focused control that no longer shows keeps focus and takes keys, the accessibility tree's focus naming the window) as a named SC 2.4.3 gap.
Re-counts and re-states the stand-repeat targets and names the 0.3.0 requirement record beside the 0.2.0 one; the counts are re-measured at P3.
Re-points owners named as the retired residuals file to residual rows: three dependency-audit ignore reasons, the windowed-witness line and the convention line of the statement.
Brings the README's 0.3.0 status section to 0.3.0's state; the statement's pin stays green, and the audit leg re-reads the advisory ignore list. No engine, driver, harness or stand code changes.
@task_file n=1 path=docs/capabilities.md mark=edits
what: the committed capability statement of what Escher can do, its stated limits and where each claim is proven, including its statement of what the MCP server and the driver session carry — edits: the opening, §What it can do, §Stated limits, §Accessibility reach, §Where each claim is proven, §Building against it, the windowed-witness owner line, the convention line
@task_file n=2 path=README.md mark=edits
what: the project README — edits: §Status, its 0.3.0 section
@task_file n=3 path=deny.toml mark=edits
what: the cargo-deny dependency audit configuration, the advisory ignore list by ID read by the audit leg — edits: the written ignore reasons of RUSTSEC-2024-0436 and of RUSTSEC-2026-0370, 0371 and 0372 (IDs unchanged)
@task_file n=4 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement's pin run by the ci-scripts leg (verb and cause names, five counts, the README's MCP tool count) — edits: the pinned counts, only should P4 widen the pin to a count the rewrite states
@task_item n=1 part=needs
The advisory ignore list: each ignore stays keyed by its advisory ID with a written reason; only the reason text is re-pointed to a residual row, no ID is added, removed or changed, and the audit leg's reading of the IDs stays as it is.
@task_item n=2 part=needs
The committed statement of the MCP crossing: the rewritten page and the README's MCP sentence must keep stating what the MCP server carries (the fifteen driver commands as tools, over stdio, results the client's, no authentication) as the plan decides it.
@task_item n=3 part=needs
Statements of what each 0.3.0 answer carries out of the process — the graph answer, the actions answer, the coverage answer line, the caused text, the layout read, the exploration report — must match the plan's crossing decisions and answer contracts: no NodeId, stable ids and closed state values only, attribute text only by ratified carriers.
@task_item n=4 part=needs
Statements of the value mask: wherever the page describes control values on a carrier (session replies, MCP results, snapshot, recorded and kept checks), it states password and file values as masked on every carrier, as the plan decides.
@task_item n=5 part=needs
The stated limits that are security absences or carve-outs — the driver session and MCP server carry no peer authentication, the session socket is Unix only and owner-reachable, no served network API — must be stated as the plan records them, not widened or narrowed.
@task_item n=6 part=needs
The new hidden-focus limit concerns accessibility focus: a hidden focused control keeps focus and the tree's focus names the window; the statement must describe it as a named gap consistent with the plan's rule that accessibility focus names a built node, without claiming a check it lacks.
@task_item n=7 part=needs
Capability claims for starting in a state and seeded randomness: starting state names are checked against a closed table, and the seeded source takes its seed only from in-process code; the page must not claim a seed or state reachable from the call surface beyond that.
@task_item n=8 part=needs
Capability claims for bundled fonts and the fonts-unavailable refusal: the page states the refusal causes as the closed set the plan holds, with no new cause named.
@task_item n=9 part=needs
Handler coverage claims: the coverage role's answer line holds checks, held and unexercised counts and no user data; kept check files are read as untrusted and kept owner-only; the page states them so.
@task_item n=10 part=needs
The capability statement's pin, run in the ci-scripts leg, stays green; if widened, it reads only committed files and keeps the README's MCP tool count at fifteen.
@task_item n=11 part=needs
The residual owner lines re-pointed in product prose name residual rows by handle only; no residual content, credential or local path is copied into the committed page or the audit configuration.
@task_item n=12 part=needs
Where-proven statements name CI runs and witnesses by run number and test name only; live evidence quoted in the page holds no credentials, as the plan requires of live evidence.
@task_item n=1 part=not_needed
The browser app's own controls — JavaScript opt-in, cookie store and HTTP cache, history in sqlite, clipboard, file dialog, TLS, external navigation: the chunk states Escher's driver capabilities and touches no browser behaviour.
@task_item n=2 part=not_needed
Engine parsing and rendering rules — CSS declarations and selectors, @import and iframe nesting limits, inline SVG serialization, border radii, inline-box heights, presentational hints: no engine code changes.
@task_item n=3 part=not_needed
Select list and range input interaction rules — keys, pointer, wheel, typeahead, painted labels, rows shown: no control behaviour changes and the scope names none of them among the capabilities stated.
@task_item n=4 part=not_needed
Outbound fetch controls — script fetcher schemes, file URLs, GET and HEAD only, per-host limits, load waits, missing CORS, timeouts and response size limits: the chunk adds no network path.
@task_item n=5 part=not_needed
The WPT runner and its post-results workflow, quarantine entries and result kinds: not reached by a prose rewrite.
@task_item n=6 part=not_needed
Release publishing, signing material and publish secrets in CI: the chunk publishes nothing and adds no job.
@task_item n=7 part=not_needed
The cold-agent run and its scripts, recordings, prompt and wrong-call bar: no cold-agent run happens in this chunk; it belongs to the later cold-agent bar entry.
@task_item n=8 part=not_needed
Engine and driver telemetry rules — log sink allowlists, target scrubbing, span lines, network tracing and resource-load failure logs: no code is changed and no log line is added.
@task_item n=9 part=not_needed
Dioxus native bridge internals — dangerous inner HTML, render buffering, mounted events, JS evaluation no-op, scheme serving: untouched.
@task_item n=10 part=not_needed
Session socket wire details — request grammar, flags, exit codes, state directory handling, session error table, oversize and idle expiry: the session code is unchanged and the chunk restates none of them.
@task_item n=11 part=not_needed
Engine dependency choices beyond the audit ignore list — pinned engine dependencies, git-dependency bans, the png, color and serde_json crates, coverage tool: no manifest or lockfile changes.
