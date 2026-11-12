@row_add n=1 row=contract/actions-answer/node-fields area=occupied-resources rec=1
boundary: The actions verb's answer
item: node fields
promise: Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
@row_add n=2 row=contract/actions-answer/argument area=occupied-resources rec=1
boundary: The actions verb's answer
item: argument
promise: The actions verb takes no argument; a call that gives one is refused malformed.
@row_add n=3 row=contract/actions-answer/paging area=occupied-resources rec=1
boundary: The actions verb's answer
item: paging
promise: The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
@row_none n=1 rec=2 type=5
@row_add n=4 row=decision/model-provider-only-on-live-cold-agent-run area=occupied-resources rec=3
name: Model provider reached only on the live cold-agent run
rule: The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request.
why: ratified
@row_add n=5 row=decision/cold-agent-run-carries-stand-screen area=occupied-resources rec=3,4
name: Live cold-agent run carries the stand's screen
rule: The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts and diffs, from the driver's own MCP server.
why: chosen as one run capped at $3, with a JSON check and a retry
@row_none n=2 rec=5 type=4
relied: The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network.
@row_add n=6 row=decision/session-state-dir-caller-named-private area=occupied-resources rec=6
name: Session state directory caller-named and owner-only
rule: The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others.
@row_add n=7 row=decision/session-state-dir-removal area=occupied-resources rec=6
name: Session state directory holds the socket alone
rule: The session state directory holds the socket file session.sock alone and is removed at stop or idle expiry: the socket file, then the directory by a non-recursive remove, nothing else deleted.
@row_add n=8 row=decision/run-state-areas-local-only area=occupied-resources rec=6
name: Agent-run and cold-agent state areas local only
rule: The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job.
@row_add n=9 row=decision/raw-run-output-never-printed area=occupied-resources rec=6,7
name: Raw run output never printed
rule: Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
@row_none n=3 rec=8 type=5
@row_none n=4 rec=9 type=5
@row_add n=10 row=decision/record-dir-launcher-named-private area=occupied-resources rec=10
name: Recording directory launcher-named and owner-only
rule: The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others.
@row_add n=11 row=contract/recording-directory/check-file area=occupied-resources rec=10
boundary: The recording directory
item: check-{n}.json
promise: One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename.
@row_add n=12 row=decision/recording-removes-only-own-checks area=occupied-resources rec=10
name: Recording removes only its own checks
rule: Escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the recording directory.
@row_add n=13 row=contract/recording-directory/platform area=occupied-resources rec=10
boundary: The recording directory
item: platform
promise: Recording is unix only; elsewhere it is refused Unsupported.
@row_add n=14 row=decision/committed-stand-checks area=occupied-resources rec=11
name: Committed stand-Counter checks
rule: Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
why: A shot's PNG bytes differ by OS, so no screenshot step is kept.
guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies
except: The one exception to a kept check's owner-only medium.
@row_add n=15 row=decision/telemetry-one-install-site area=occupied-resources rec=12,20
name: One telemetry install site per binary
rule: Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread.
@row_add n=16 row=decision/cold-agent-run-adds-no-listener area=occupied-resources rec=12
name: Cold-agent run adds no listener
rule: The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code.
why: ratified
@row_add n=17 row=decision/session-host-single-process area=occupied-resources rec=12
name: Session host is one process with one listener
rule: The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
@row_add n=18 row=decision/mcp-role-no-listener area=occupied-resources rec=12
name: MCP role holds one session and no listener
rule: The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
@row_none n=5 rec=13 type=4
relied: The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener.
@row_none n=6 rec=14 type=5
@row_add n=19 row=decision/parallel-traversals-serialised area=occupied-resources rec=15
name: Parallel style traversals serialised
rule: Every Parallel style traversal takes one process-wide lock, so two documents never traverse at once on Stylo's global rayon pool.
why: Two documents resolving Parallel at once panicked already mutably borrowed (blitz issue 430).
@row_add n=20 row=decision/headless-documents-sequential-style area=occupied-resources rec=15
name: Headless documents style sequentially
rule: Sequential style threading stays the default; Parallel is set only by the windowed launch and the browser, never by a headless document such as the harness, stand or driver.
@row_none n=7 rec=16 type=4
relied: Stylo's prefs are set once per process before the user-agent sheet is parsed.
@row_add n=21 row=decision/no-escher-allocator-change area=occupied-resources rec=17
name: No escher binary changes allocator
rule: No escher binary changes the global allocator; mimalloc is the allocator only of the two upstream-only binaries, the WPT runner and the browser app.
@row_add n=22 row=contract/command-span/graph-call area=occupied-resources rec=18
boundary: The command span
item: graph call
promise: A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
@row_add n=23 row=contract/command-span/actions-call area=occupied-resources rec=19
boundary: The command span
item: actions call
promise: An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role.
@row_add n=24 row=decision/coverage-role-in-process area=occupied-resources rec=20
name: Coverage role runs in process
rule: The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
@row_add n=25 row=contract/environment-variables/rust-log area=occupied-resources rec=21
boundary: Environment variables
item: RUST_LOG
promise: escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
@row_add n=26 row=name/crate/browser area=occupied-resources rec=22
class: crate
name: browser
@row_add n=27 row=name/crate/browser-persistence area=occupied-resources rec=22
class: crate
name: browser-persistence
@row_add n=28 row=name/crate/rdme area=occupied-resources rec=22
class: crate
name: rdme
@row_add n=29 row=name/crate/counter area=occupied-resources rec=22
class: crate
name: counter
@row_add n=30 row=name/crate/seven-guis area=occupied-resources rec=22
class: crate
name: seven_guis
@row_add n=31 row=name/crate/todomvc area=occupied-resources rec=22
class: crate
name: todomvc
@row_add n=32 row=name/crate/transparent area=occupied-resources rec=22
class: crate
name: transparent
@row_add n=33 row=name/crate/wasm-hello area=occupied-resources rec=22
class: crate
name: wasm_hello
@row_add n=34 row=name/crate/wgpu-texture area=occupied-resources rec=22
class: crate
name: wgpu_texture
@row_add n=35 row=name/crate/accesskit-xplat area=occupied-resources rec=22
class: crate
name: accesskit_xplat
@row_add n=36 row=name/crate/blitz-dom area=occupied-resources rec=22
class: crate
name: blitz-dom
@row_add n=37 row=name/crate/blitz-traits area=occupied-resources rec=22
class: crate
name: blitz-traits
@row_add n=38 row=name/crate/blitz-vibey-script area=occupied-resources rec=22
class: crate
name: blitz-vibey-script
@row_add n=39 row=name/crate/blitz area=occupied-resources rec=22
class: crate
name: blitz
@row_add n=40 row=name/crate/debug-timer area=occupied-resources rec=22
class: crate
name: debug_timer
@row_add n=41 row=name/crate/escher-telemetry area=occupied-resources rec=22
class: crate
name: escher-telemetry
@row_add n=42 row=name/crate/escher-driver area=occupied-resources rec=22
class: crate
name: escher-driver
@row_add n=43 row=name/binary/blitz area=occupied-resources rec=22
class: binary
name: blitz
@row_add n=44 row=name/binary/seven-guis-native area=occupied-resources rec=22
class: binary
name: seven_guis_native
@row_add n=45 row=name/binary/escher-session area=occupied-resources rec=22
class: binary
name: escher-session
@row_add n=46 row=name/binary/todomvc-native area=occupied-resources rec=22
class: binary
name: todomvc_native
@row_add n=47 row=name/server/escher area=occupied-resources rec=22
class: server
name: escher
@row_add n=48 row=name/other/cold-agent-sh area=occupied-resources rec=22
class: other
name: cold-agent.sh
@row_add n=49 row=name/other/cold-agent-ps1 area=occupied-resources rec=22
class: other
name: cold-agent.ps1
@row_add n=50 row=decision/upstream-ci-resources-upstream-only area=occupied-resources rec=23
name: Upstream CI resources reached only from upstream
rule: Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them.
@row_add n=51 row=decision/free-disk-space-only-in-stand-repeat area=occupied-resources rec=25,24
name: Disk freed only in the stand-repeat job
rule: The Stand checks repeated job is the workflow's one run of the free-disk-space action, pinned by sha and run after the toolchain and before the rust cache; the matrix test job holds no such step.
guard: test_only_stand_repeat_frees_disk
@row_add n=52 row=decision/upstream-taken-by-cherry-pick area=project-intent rec=26
name: Upstream changes taken by cherry-pick
rule: Upstream commits are taken by no-commit cherry-picks of upstream's text, each with its cherry-picked-from trailer, and every commit left out carries its measured reason.
@row_add n=53 row=decision/no-git-dependencies area=project-intent rec=26
name: No git dependency on upstream's moving engines
rule: Stylo stays 0.22.0, Taffy the registry 0.15 and Dioxus 0.7.3, with no git dependency; upstream's git Stylo 0.23-pre, git Taffy and Dioxus 0.8 alpha wait for escher's next version.
@row_none n=8 rec=27 type=0
what: an observed panic in upstream's own tree
@row_add n=54 row=overview/project-intent area=project-intent rec=28
text: Escher is a Rust library workspace for an HTML/CSS engine: high-level APIs for rendering HTML with Blitz, a core DOM abstraction whose primary entry point is BaseDocument, and a Document trait that lets wrappers around BaseDocument all be driven by blitz-shell.
@row_add n=55 row=decision/readme-is-escher-own area=project-intent rec=29
name: Front page is escher's own
rule: The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version.
@row_add n=56 row=decision/escher-capitalised-in-prose area=project-intent rec=29
name: Escher capitalised in prose
rule: The front page spells the name Escher with a capital in its prose and title, while crate, binary and path names, URLs and the e-mail stay unchanged.
@row_none n=9 rec=30 type=0
what: where GitHub renders the front page from, as measured
@row_none n=10 rec=31 type=5
@row_none n=11 rec=32 type=5
@row_add n=57 row=decision/capability-statement area=project-intent rec=33
name: Capability statement
rule: docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims.
guard: .github/scripts/test_capability_statement.py
@row_add n=58 row=contract/capability-statement/sections area=project-intent rec=33
boundary: The capability statement
item: sections
promise: Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.
@row_none n=12 rec=34 type=5
@row_none n=13 rec=35 type=5
@row_add n=59 row=decision/rust-cargo-workspace area=stack-and-technologies rec=36
name: Rust Cargo workspace
rule: Rust is the implementation language of every crate, in one Cargo workspace on resolver 2, edition 2024 and rust-version 1.91.0.
except: wgpu_texture and debug_timer stay on edition 2021.
@row_add n=60 row=decision/shared-workspace-package area=stack-and-technologies rec=37
name: Shared workspace package metadata
rule: Member crates inherit one workspace version, the MIT OR Apache-2.0 license and metadata, and the in-repo blitz crates are pinned to that exact version.
@row_add n=61 row=decision/stylo-css-engine area=stack-and-technologies rec=38
name: Servo Stylo as the CSS engine
rule: Servo Stylo 0.22.0, with selectors and cssparser, does styling and selector matching.
@row_add n=62 row=decision/html5ever-parsing area=stack-and-technologies rec=39
name: html5ever and xml5ever for parsing
rule: HTML is parsed with html5ever and XML or XHTML with xml5ever, on markup5ever's names and namespaces.
@row_add n=63 row=decision/taffy-layout area=stack-and-technologies rec=40
name: Taffy for layout
rule: Taffy 0.15 from the crates.io registry does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
@row_add n=64 row=decision/parley-text-layout area=stack-and-technologies rec=41
name: Parley for text and fonts
rule: Parley 0.12 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.
@row_add n=65 row=decision/icu-text-transform-optional area=stack-and-technologies rec=41
name: ICU case mapping behind a default feature
rule: ICU4X case mapping for language-aware text-transform is optional behind blitz-dom's default text-transform-icu feature, while its locale, properties and segmenter crates are unconditional dependencies.
