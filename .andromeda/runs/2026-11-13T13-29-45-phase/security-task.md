@task_prose part=chunk
Takes upstream blitz at the measured pin 563d3db73 in the shape the founder picks at P4: a cherry-pick onto the registry pins, that plus upstream's registry moves (Stylo 0.24, Parley 0.13, winit beta.4), a whole merge that brings git dependencies, or nothing.
Fixes the two CI reds from outside: the clippy needless_bool lint under rustc 1.100, and the RUSTSEC-2026-0459 audit failure on libsqlite3-sys through rusqlite, settled at P4 by a bump or an ignore by ID with a written reason.
Pins every action in the three unpinned workflows to a 40-hex commit SHA and widens the pinning check past ci.yml to every workflow; adds two headless frame tests (blend/isolation and inline-box decorations), each in both layout modes.
Re-reads the left-out commits, the merge surfaces (anonymous-block deallocation, the hit_inner non-invertible answer, the buffered render) and the changed-set qualifier against the take.
Re-runs every escher proof (fast, doc, a11y and audit legs, stand checks, oracle, embolden witness, green fork CI), re-reads the per-OS tallies, and re-checks the sink allowlist over the taken delta. Every boundary widening the take brings is listed for the operator at P4.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document, home of anonymous block deallocation and one clippy needless_bool site — edits: the needless_bool site at line 3223
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the layout damage computation behind the changed set — edits: the two needless_bool sites at lines 410 and 426
@task_file n=3 path=.github/workflows/wpt.yml mark=edits
what: the upstream-only WPT CI workflow, whose third-party actions are not yet pinned — edits: every action reference, pinned to a commit SHA with its ref as a trailing comment
@task_file n=4 path=.github/workflows/publish-browser.yml mark=edits
what: the browser release publish workflow, holding the signing and publish secrets and the publish job repository guard — edits: every action reference, pinned to a commit SHA with its ref as a trailing comment
@task_file n=5 path=.github/workflows/wpt-post-results.yml mark=edits
what: the WPT post-results workflow, which runs trusted scripts and splices results into a PR — edits: every action reference, pinned to a commit SHA with its ref as a trailing comment
@task_file n=6 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow guard that checks actions are pinned to commit SHAs — edits: the pinning check, widened from ci.yml alone to every workflow
@task_file n=7 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus native document bridge holding the buffered render (its two render sites, the buffered_mutations field and the ported render-time document access test), a merge surface the take re-reads
@task_file n=8 path=packages/dioxus-native-dom/src/mutation_buffer.rs
what: escher's own mutation buffer behind the buffered render, which upstream's Dioxus 0.8 file replaces whole if that move is taken
@task_file n=9 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root, holding the one module line for the mutation buffer
@task_file n=10 path=Cargo.toml
what: the root workspace manifest: the coupled engine dependency pins (stylo, taffy, anyrender, parley, Dioxus, winit, vello), workspace members and the no-git-dependency rule's subject
@task_file n=11 path=Cargo.lock
what: the locked dependency graph the audit leg reads for advisories and licences, carrying libsqlite3-sys through rusqlite
@task_file n=12 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner behind the fast, doc, a11y and dependency-audit legs re-run after the take
@task_file n=13 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check that pins the blitz-tests target list, one file and one target per behaviour
@task_file n=14 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the telemetry sink's standing capture checks, held at 0 needles
@task_item n=1 part=needs
The no-git-dependency rule and the engine dependency pins: upstream's pins are taken as one set at the chosen sha or not at all, and a whole merge of 563d3db73 brings git Taffy and git AnyRender against that rule, so it is the founder's call.
@task_item n=2 part=needs
The dependency audit gate and its advisory ignores: RUSTSEC-2026-0459 on libsqlite3-sys is settled either by a semver-compatible update or by an ignore by ID with a written reason; a rusqlite 0.41 move is a bump outside upstream's set and goes to P4.
@task_item n=3 part=needs
The licence gate and the hand review of new dependencies: every crate the take brings in is read by hand and through the audit's licence check, and MSRV 1.91 holds against what each one declares.
@task_item n=4 part=needs
Locked builds and workspace-inherited versions: the lock moves only with the chosen take, and no members change or new package of ours comes in; upstream's new crates (parley_code_editor, devtools) are checked for membership.
@task_item n=5 part=needs
CI actions pinned to commit SHAs: every action in every workflow carries a 40-hex SHA with its ref as a trailing comment, and the guard checks every workflow. The pins are written after the take's own workflow lines are in place.
@task_item n=6 part=needs
The boundary widening rule: each new content reader or crossing the chosen take brings (a DevTools/CDP socket or port, lossy page decoding, new attribute reads such as font face/size, valign and alt-as-decorative, text-shadow and mask paint, the AccessKit refresh, new env reads) is listed for the operator at P4, never settled silently, and becomes a wrap amendment by his answer.
@task_item n=7 part=needs
The served-network-API absence and the session socket as the one listener: a DevTools/CDP port arriving with the take would widen them, and any new socket, port or IPC endpoint needs a security-plan amendment first.
@task_item n=8 part=needs
The telemetry sink target allowlist and the content-field scrub: the taken delta is read for every new tracing/log call site, target, print and env read, and the sink's capture checks stay at 0 needles.
@task_item n=9 part=needs
Env reads: the driver reads only RUST_LOG; any env read the take brings is read against that list.
@task_item n=10 part=needs
The script engine as a security surface: any script API the take adds is read for what it lets out, against the bootstrap-hidden native helpers, the fetch method limits and the local-only default script fetcher.
@task_item n=11 part=needs
Accessibility adapter rows (content classes on the adapter crossing, tree refresh on change, focus on built nodes only, author id): the AccessKit refresh must keep escher's accessibility state reads and the a11y leg green.
@task_item n=12 part=needs
Conflict resolutions keep escher's engine behaviour: the clock (time moves only on advance), the select model and select list, hit stops and the non-invertible transform taking no hit, scrolled-box bounds, inert, the range and keyboard models, text_extent and the password mask.
@task_item n=13 part=needs
The stable element id and the snapshot it rides in (derivation, stability, no NodeId, the snapshot's masked values and state words): the take must leave these as they are in dioxus-native-dom, as the id and snapshot stand checks decide.
@task_item n=14 part=needs
Error-handling rows over the merge surfaces: a dropped anonymous block clears its children's parent (upstream's anonymous-block transform fix is read against it), Dioxus renders into a buffer before borrowing, try_doc answers none while borrowed, and the mounted event of a removed element is skipped.
@task_item n=15 part=needs
The changed-set write sites, through the select-list rule: the changed set's never-list holds only while no select list is open, an amendment that lands at the wrap; P3 reads whether the AccessKit refresh or mutation changes move those write sites.
@task_item n=16 part=needs
The sink allowlist over taken markup readers: new attribute readers, such as font face/size, valign and alt, must not log or panic, and attribute text leaves only by ratified carriers.
@task_item n=17 part=needs
The WPT post-results and publish workflows' trust rows (trusted scripts only, read-only CI tokens, secrets reachable only upstream, repository guard): pinning their actions must keep their permissions and guards as they are.
@task_item n=18 part=needs
Upstream-sync dependency-set rule: commits upstream adds after 563d3db73 go to the next sync, and only the operator moves the pin.
@task_standing n=1
convention: each new or taken test, fixture or harness keeps the test harness free of tracing and style setup and holds no page content in a log line
home: dependency-security
@task_standing n=2
convention: each new log line or tracing target goes through escher's allowlist stderr sink, carrying only allowlisted, content-free fields
home: logging-monitoring
@task_standing n=3
convention: each new or changed CI job or workflow pins its actions to commit SHAs, runs with a read-only token and keeps secrets out of fork runs
home: dependency-security · authentication-authorization
@task_standing n=4
convention: each new crate that enters the lock is a registry version, reviewed by hand and passed through the licence and advisory gate
home: dependency-security
@task_item n=1 part=not_needed
The driver verb schema, the driver argument kinds, the refusal causes and refusal JSON: the chunk adds or changes no verb, argument or refusal, and adopting new upstream APIs in escher's own code is out of scope.
@task_item n=2 part=not_needed
The MCP server rows (stdio transport, protocol errors, request line and result bounds, tool listing and tool results): the chunk changes no MCP role, tool or protocol line.
@task_item n=3 part=not_needed
The recorded-check and kept-check file contracts and the recording directory rows: the chunk changes no recording, replay or kept check.
@task_item n=4 part=not_needed
The cold-agent and agent-run harness rows (call allowlists, fixed prompt, transcripts, events metadata, replay isolation): the chunk does not touch the agent harness.
@task_item n=5 part=not_needed
The escher-session command-line flags, roles, exit codes and stdout/stderr output contracts: the chunk adds no flag, role or output line to the session CLI.
@task_item n=6 part=not_needed
The coverage answer, coverage line and per-crate coverage floor rows: the chunk changes no coverage verb or floor.
@task_item n=7 part=not_needed
The apt-install script rows and the session state directory rows (owner-only, path as parameter, non-recursive cleanup): neither the install script nor the state directory changes.
