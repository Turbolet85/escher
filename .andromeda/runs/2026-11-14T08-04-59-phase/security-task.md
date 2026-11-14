@task_prose part=chunk
Bundles a run's fonts: a run's text is shaped and measured from one bundled font set, the stand's DejaVu Sans asset already in the tree, with the host's system fonts off, on every path a run boots through — the stand's boot, the driver session's served tasks, the shared fixture pages and a harness-built document that names no font context of its own.
The generic families a run's styles name (sans-serif, serif, monospace, an unknown family) reach the bundled set and not the host.
The bundled set is held in the document's font context, where a later canvas painter can reach it; canvas text is named as a future reader of it, and nothing of canvas is built.
Keeps the fonts-unavailable refusal as it stands and re-asserts it: an app whose font context holds no family it can paint text with is refused before anything is painted or read.
Proves that every CI OS of the test leg (Linux, macOS, Windows) reads one state of a run the same way, compared by its readings — snapshot text, bounds, text extents, lint findings — never by a shot's bytes.
@task_file n=1 path=examples/seven_guis/src/stand.rs
what: the stand's boot and the fixture pages' document setup, which build the single-font context from the stand's DejaVu Sans
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: the test harness's options and document builder, whose font context defaults to none for a harness-built document
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the engine document's constructor, which turns an absent font context into the host's system font collection under the system-fonts feature
@task_file n=4 path=packages/escher-driver/src/refusal.rs mark=re-reads
what: the driver's fonts-unavailable refusal cause and its text, read back as kept unchanged
@task_file n=5 path=packages/escher-driver/src/screenshot.rs mark=re-reads
what: the driver's check that refuses fonts-unavailable before a screen is painted or read, read back as kept
@task_item n=1 part=needs
The engine document's font context: a run's documents get the bundled font set with the host's system fonts off, on the stand's boot, the driver session's served tasks, the shared fixture pages and a harness-built document with no font context of its own.
@task_item n=2 part=needs
Generic family resolution: sans-serif, serif, monospace and an unknown family a run's styles name resolve onto the bundled set, never onto a host font.
@task_item n=3 part=needs
The fonts-unavailable refusal cause stays in the closed refusal cause set with its text, meaning and remedy unchanged, and still refuses before anything is painted or read.
@task_item n=4 part=needs
The cross-OS proof reads one state through the driver's readings — snapshot text with its state words and fields, bounds, text extents, lint findings — and compares those, never a screenshot's PNG bytes across operating systems.
@task_item n=5 part=needs
The text extent reader is read-only and answers none where a node is unresolved; the cross-OS comparison reads it as it stands and adds no field to it.
@task_item n=6 part=needs
The boundary widening rule: the chunk adds no carrier, verb, argument, field, refusal cause, log field or span, and no driver answer gains a font field.
@task_item n=7 part=needs
The bundled font reaches the font context from the font asset file already in the tree, never from a URL or path a caller, agent or page supplies; font sources take no agent URL.
@task_item n=8 part=needs
Which font a remote page or the browser app reads stays as it is, including hint-less font-face sources fetched and sniffed; a change to it is an engine-change fork for the founder, not built here.
@task_item n=9 part=needs
No new dependency crate: the dependency set, workspace-inherited versions, engine dependency pins and locked builds stay as they are, and the dependency audit and licence gate still pass on the push.
@task_item n=10 part=needs
The test harness takes no tracing and no style dependency; giving a harness-built document the bundled font context adds neither.
@task_item n=11 part=needs
The CI matrix runs every job on every OS of the test leg; a cross-OS check, if added, runs on all three runners and none is skipped.
@task_item n=12 part=needs
A committed reading each OS replays, if P3/P4 settles on one, is a committed check like the stand-Counter checks: readings only, no user data, read back strictly as untrusted bytes.
@task_item n=13 part=needs
Screen content stays out of logs: a font context built or a family resolved logs no text, family name read from a page, or reading.
@task_item n=14 part=needs
The fonts-unavailable refusal is logged by its cause name only, and the refusal holds nothing a call supplied.
@task_standing n=1
convention: each new or changed CI job or step pins its actions to commit SHAs and builds locked
home: dependency-security
@task_standing n=2
convention: each new CI job runs with a read-only token and reaches no secret
home: authentication-authorization
@task_standing n=3
convention: each new log line goes through escher's allowlist stderr sink and carries no screen content or caller input
home: logging-monitoring
@task_standing n=4
convention: each new committed test fixture or check file holds readings only, no user data and no credentials
home: data-protection
@task_standing n=5
convention: each new test counts toward its crate's line coverage floor, which the change must keep
home: input-validation
@task_item n=1 part=not_needed
The MCP server's stdio transport, request and result bounds and protocol errors: the chunk adds no tool and changes no MCP framing.
@task_item n=2 part=not_needed
The driver session socket's request grammar, connection bounds, owner-only reach and idle expiry: the chunk opens no listener and changes no wire.
@task_item n=3 part=not_needed
The stable element id's derivation, carriers and refusals of unknown ids: fonts change no element's identity.
@task_item n=4 part=not_needed
Select lists, range inputs, keyboard and pointer activation of controls: the chunk changes no control's behaviour.
@task_item n=5 part=not_needed
Password and file input value masking and its stand-ins: the chunk masks or unmasks nothing.
@task_item n=6 part=not_needed
The cold-agent and agent-run scripts, their prompts, recordings and the provider crossing: the chunk runs no agent.
@task_item n=7 part=not_needed
The browser app's history store, cookies, HTTP cache, clipboard and file dialog: the chunk stores nothing.
@task_item n=8 part=not_needed
Outbound HTTP, script fetch methods, CORS and same-origin absences, external navigation schemes: the bundled font is a local file, fetched over no network.
@task_item n=9 part=not_needed
The WPT runner, its quarantine entries and its post-results workflow: the chunk changes no web platform test run.
@task_item n=10 part=not_needed
Release publishing, signing material and publish secrets: the chunk publishes nothing.
@task_item n=11 part=not_needed
The rdme tool's raw HTML passthrough and exit codes, and HTML parsing with scripting disabled: no markup path changes.
