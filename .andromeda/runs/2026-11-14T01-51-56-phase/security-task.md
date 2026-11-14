@task_prose part=chunk
Seeded randomness (v030-13): a run of the driver (a held Session and the test harness it holds) is given one seed, and every randomness source the run's app and page can read draws from it.
The app-facing source: a Dioxus app hosted by the driver reads randomness through one engine-provided source (a root context or hook in dioxus-native-dom) fed by the run's seed, not from the host.
The page-facing source: a page script's Math.random in a seeded run draws from the run's seed, replaced in the blitz-vibey-script bootstrap scripts.
The seed enters through Session::start or a builder beside with_time, HarnessOptions, and, if start takes a seed, the driver verb table, the kept check (recording, replay), help's self-description and the committed capability statement with its pin.
The proof: an integration check in both layout modes that two runs of one seed read the same snapshot text and diffs at every step of a scripted sequence on an app or stand fixture that reads randomness, and that two different seeds read a difference.
@task_file n=1 path=docs/capabilities.md mark=edits
what: the committed capability statement of the driver, the statement of the MCP crossing and its pin — edits: the seed capability and the pin
@task_item n=1 part=needs
The driver session's start: a seed given to Session::start or a builder beside with_time, held for the run and passed to the test harness options; the virtual clock's existing rule that time moves only on advance stands beside it.
@task_item n=2 part=needs
If start takes a seed, a new argument kind on the driver verb table and argument kinds: it must pass the closed driver verb schema, the single validation gate and the one validation path for every carrier (command line, session socket, MCP, kept check), the closed argv reader and the MCP tool argument value kinds.
@task_item n=3 part=needs
Whether a seed argument is a widening of a ratified carrier is decided by the boundary widening rule and the rule that a closed-value field inside a ratified carrier is no widening while an open value set or a new carrier is; the chunk's carrier change must be checked against them.
@task_item n=4 part=needs
A malformed or out-of-range seed argument is refused through the closed refusal cause set (malformed) with no panic on call input, no echo of caller input, and a refusal that holds nothing the call supplied.
@task_item n=5 part=needs
The session socket request grammar and the command line flags: a seed reaching start over the socket or the escher-session command line stays inside the closed request grammar, its bounds and the escher-session flags list.
@task_item n=6 part=needs
The kept check / recorded check: if the seed is recorded, its start record carries it; the recorded check stays read strictly as untrusted bytes, replay runs each step through the session, and the kept check file contents, owner-only lifecycle and masking stay as they are.
@task_item n=7 part=needs
The seed's value stays out of every log line: the command span and tool_call span stay content-free, escher targets scrub content-named fields, the driver installs no subscriber and session output holds no caller input; no log field carries a seed.
@task_item n=8 part=needs
No new listener, port, socket or environment read: the session socket stays the one listener, no served network API appears, the driver reads only RUST_LOG, and the MCP server stays on stdio — a seed never arrives through an environment variable.
@task_item n=9 part=needs
The test harness (HarnessOptions) gains the seed; the test harness still takes no tracing and no style.
@task_item n=10 part=needs
The dioxus-native-dom app-facing randomness source: a root context or hook an app reads, engine-provided, drawing from the run's seed rather than the host's thread RNG.
@task_item n=11 part=needs
The blitz-vibey-script bootstrap scripts replace Math.random in a seeded run; any native helper the replacement installs stays hidden from pages as the bootstrap native helper and the native search-params parser are.
@task_item n=12 part=needs
The state compared across runs is the snapshot text (state words, id and name fields) and the diff nodes; the stable element id's derivation and stability must not come to depend on the seed or on per-process iteration order, which P3 measures.
@task_item n=13 part=needs
The MCP crossing's committed statement and help's self-description describe the seed if start takes one; MCP listing stays fixed program text and MCP results stay bounded.
@task_item n=14 part=needs
Carve-out: the windowed shell (blitz-shell) and the browser app are not seeded runs; their randomness and their JavaScript opt-in are left as they are.
@task_item n=15 part=needs
A seedable random generator, if the chunk adds one as a dependency, passes the dependency audit and licence gates, locked builds, workspace-inherited versions and the no-git-dependency rule.
@task_item n=16 part=needs
The stand (examples/seven_guis) or a fixture gains a surface that reads randomness for the proof; committed stand checks stay as they are.
@task_standing n=1
convention: each new test owes the per-crate line coverage floors of the crates it lands in
home: input-validation
@task_standing n=2
convention: each new log line or span field holds no content and passes the escher target scrub and the sink's target allowlist
home: logging-monitoring
@task_standing n=3
convention: each new dependency is workspace-inherited, locked, licence-gated and audited by cargo-deny
home: dependency-security
@task_standing n=4
convention: each new refusal is a cause of the closed refusal set with its fixed meaning and remedy
home: error-handling
@task_item n=1 part=not_needed
The browser app's networking and storage: outbound HTTP limits, cookies, HTTP cache, history in sqlite, TLS, clipboard and file dialogs — the browser app is not a seeded run.
@task_item n=2 part=not_needed
The select list's painting, keys, rows and typeahead, and password inputs painted as bullets — the chunk changes no control's behaviour or painting.
@task_item n=3 part=not_needed
CI publishing, signing material, publish secrets and the read-only CI token — the chunk adds no CI job and touches no release.
@task_item n=4 part=not_needed
The WPT runner, its quarantine entries, its results and its post-results workflow — no web-platform test is changed.
@task_item n=5 part=not_needed
CSS parsing, stylesheet loading, border radii, inline-box heights, iframe and import nesting limits and font-face sources — layout and style are untouched; bundled fonts are the next entry.
@task_item n=6 part=not_needed
The cold-agent and agent-run harness scripts, their call allowlists, prompts and recordings — auto-exploration and agent replay are this seed's later readers, not this chunk.
@task_item n=7 part=not_needed
Application authentication and authorization absences — the chunk adds no access control.
@task_item n=8 part=not_needed
Accessibility adapter decisions (accessible names, descriptions, focus, tree refresh) — the chunk reads no accessible name.
@task_item n=9 part=not_needed
Script fetch scheme and method rules, CORS, same-origin and CSP absences — page scripts gain a seeded Math.random, not a new fetch.
@task_item n=10 part=not_needed
The apt-install and coverage_floor.py helper scripts' argument validation and exit codes — not touched.
