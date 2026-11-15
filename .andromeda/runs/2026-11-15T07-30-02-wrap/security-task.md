@task_prose part=session
The eleventh upstream take: upstream/main read at ea07ee716, three upstream commits cherry-picked as written, 13 new commits left out with a measured reason each (112 left out in all).
The picks add the @font-face font-stretch (font-width) descriptor to the descriptors a web font is registered under, move the pinned WPT commit, and add four WPT timeout-quarantine entries with reason dynamic-load-events.
Taffy moves 0.15.0 to 0.15.1 by a lock-only, semver-compatible update from the crates.io registry; no manifest moves, no package enters or leaves, no git source enters the lock, and the audit reads advisories ok, licenses ok.
The git-Taffy take was measured green and not taken because of the no-git-dependency rule; the founder then ruled an exception to that rule for Taffy alone, and ratified both behaviour changes (the font-stretch read and the 0.15.1 layout fixes) for every document, remote pages included.
The lockfile's taffy entry now holds checksum 31f7bd2d97 (was 3c8480887e), and the source and checksum lines moved from 8040-8041 to 8042-8043.
@task_file n=1 path=Cargo.lock
what: the lockfile, which holds every crate's resolved version, its registry source and its checksum; here the taffy entry (registry version 0.15.1, new checksum)
@task_file n=2 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's @font-face rule reader, which fetches web font sources from page CSS and registers each font under its CSS-declared descriptors (now family, weight, style and width)
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document, which applies the @font-face descriptor overrides when a fetched web font is registered for font matching
@task_file n=4 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT commit the WPT runner reads its tests at
@task_file n=5 path=wpt/runner/timeout-quarantine.txt mark=merge
what: the WPT timeout-quarantine list, one entry a test path with a known reason, which the runner's validation test checks
@task_item n=1 part=touches
decision/no-git-dependency: the founder ruled an exception for the taffy crate alone; the rule must now carry it (every other crate stays a registry version, and today no crate is a git source).
@task_item n=2 part=touches
decision/no-git-dependencies: its rule takes the same Taffy-alone exception, and its verified evidence is stale: the quoted lockfile checksum moved from 3c8480887e to 31f7bd2d97 and the quoted range 8040-8041 no longer holds the source and checksum, which now stand at 8042-8043.
@task_item n=3 part=touches
decision/engine-dependency-pins: Taffy stays the registry 0.15, now resolved 0.15.1; the row stays true but its Taffy pin is the one this session moved.
@task_item n=4 part=touches
decision/upstream-sync-dependency-set: this take entered no package and moved a pinned crate by a lock-only, semver-compatible registry update that carries upstream's git-pinned content; the row's wording does not name that shape.
@task_item n=5 part=touches
decision/wpt-quarantine-entries-valid: four new quarantine entries, each reason dynamic-load-events, a known reason, no path repeated; the rule holds for them.
@task_item n=6 part=touches
decision/locked-builds: the lockfile moved (one crate's version and checksum); builds still read it locked.
@task_item n=7 part=touches
decision/dependency-audit-ci-gate, decision/cargo-deny-audit-tool and decision/licence-gate: the audit leg read the moved lock green locally and in CI.
@task_item n=8 part=touches
decision/workspace-inherited-dependency-versions: the taffy requirement in the manifest stays 0.15; only the resolved version moved.
@task_item n=9 part=touches
decision/hintless-font-face-source-sniffed and decision/no-agent-urls-through-font-path: the @font-face rule reader they rule over now also reads the font-stretch descriptor from page CSS; it adds no URL and no fetch.
@task_item n=10 part=touches
decision/untrusted-bytes-decoded-without-panic: page CSS now feeds one more @font-face descriptor (font-stretch, Stylo-parsed, numeric) into font matching for every document, remote pages included.
@task_item n=11 part=touches
contract/wpt-results/timeout: four more tests sit in the timeout quarantine.
@task_fact n=1 slug=dependency-security
fact: The no-git-dependency rule has one exception, ruled by the founder on 2026-11-15: the taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy, when taken, still passes the lock, audit and MSRV legs.
@task_fact n=2 slug=dependency-security
fact: An upstream sync by cherry-pick may carry upstream's git-pinned crate content through a lock-only, semver-compatible registry update of an already-pinned crate, entering no package and moving no manifest.
@task_item n=1 part=untouched
The driver session socket, its closed grammar, bounds, replies and lifecycle: no verb, socket or reply changed.
@task_item n=2 part=untouched
The MCP server over stdio, its requests, results, protocol errors and tool-call span: unchanged.
@task_item n=3 part=untouched
Recorded and kept checks, the recording directory and cold-agent runs: unchanged.
@task_item n=4 part=untouched
The stable element id, snapshot text, masked control values and the actions and graph answers: unchanged.
@task_item n=5 part=untouched
Data protection storage: history in sqlite, cookies, the HTTP cache, TLS, clipboard and file dialog: unchanged.
@task_item n=6 part=untouched
Secret management and CI publishing: CI secrets, signing material, the publish job guard and read-only tokens: unchanged.
@task_item n=7 part=untouched
Logging rules and the telemetry sink: no log site, scrub, redaction, target allowlist or log field added or changed.
@task_item n=8 part=untouched
Refusal causes, session errors, CLI exit codes and flags: no cause, code or flag changed.
@task_item n=9 part=untouched
Authentication and authorization absences and the served-network-API absence: no listener, port or auth surface added.
