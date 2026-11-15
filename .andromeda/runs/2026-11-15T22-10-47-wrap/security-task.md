@task_prose part=session
The session moved the Taffy layout engine dependency from the crates.io registry version 0.15.1, with its checksum, to upstream's git repository pinned at the full commit rev e90751fd (package version 0.15.0, licence MIT, rust-version 1.71), its features and dependency list unchanged.
Taffy is now the workspace's one git source (the git-source count moved 0 to 1), with no patch section; no other package moved, parley unmoved.
The content came by no-commit cherry-pick of eight upstream blitz commits, the lock file the only conflict in each, re-resolved; the later Taffy chain from #1372 on (reversed scroll origins and five later bumps) was left out by design for its own later route entry.
The one call of Taffy's float placement in blitz-dom's inline layout now passes a caller-defined float id (Taffy #1303); no public symbol, crate, schema, config, harness or gate changed.
The founder ratified the eight commits' layout changes for every document; the gates, stand, a11y leg and CI all ran green, the audit leg reading advisories ok and licenses ok, and no test tally moved.
@task_file n=1 path=Cargo.toml mark=merge
what: the root workspace manifest, holding the engine dependency pins and the workspace-inherited dependency versions; its taffy line now names the git repository at rev e90751fd in place of a registry version
@task_file n=2 path=Cargo.lock mark=conflict
what: the lockfile behind locked builds and the audit leg; the taffy package now carries a git source at e90751fd and no checksum line, the lock's only git source; the conflict in each cherry-pick was resolved here
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom's inline layout pass, the one caller of Taffy's float placement, now passing a caller-defined float id
@task_item n=1 part=touches
decision/engine-dependency-pins: Taffy is no longer pinned as a crates.io registry 0.15 version; it is pinned to upstream git at rev e90751fd, package version 0.15.0.
@task_item n=2 part=touches
decision/no-git-dependencies: its statement that taffy and parley are crates.io registry versions with checksums is stale for taffy; taffy is now a git dependency with no lock checksum, under the Taffy-only exception; parley unmoved. Its verified evidence cites lock lines that moved (the taffy version and source now at lines 8041-8042, the checksum line gone, later lines up one).
@task_item n=3 part=touches
decision/no-git-dependency: the workspace now holds exactly one git dependency, taffy; a row saying no git dependency exists is broken unless it carries the Taffy exception.
@task_item n=4 part=touches
decision/upstream-sync-dependency-set: the upstream take moved the taffy package alone in the lock, and its git source is the Taffy exception's own subject; the set was taken through eight cherry-picks with the lock re-resolved, not whole.
@task_item n=5 part=touches
decision/locked-builds: the lock's taffy entry is now a git source resolved by rev with no checksum; locked builds and the audit leg ran green over it.
@task_item n=6 part=touches
decision/licence-gate and decision/dependency-audit-ci-gate and decision/cargo-deny-audit-tool: the audit leg now reads a git-sourced crate (MIT) inside cargo-deny's resolved graph; it passed with advisories ok and licenses ok.
@task_item n=7 part=touches
decision/workspace-inherited-dependency-versions: the taffy workspace dependency is now inherited as a git rev pin rather than a registry version.
@task_item n=8 part=touches
absence/automated-dependency-updates: the bump was manual and the deferred later Taffy chain waits for its own route entry; no update tooling was added, so the absence stands but its context (a manual git pin to move) changed.
@task_item n=9 part=touches
dependency-security overview: its note that the lockfile's advisory state is read within cargo-deny's resolved graph now covers one git source.
@task_item n=10 part=touches
decision/inline-box-heights-finite and other inline-layout rules: indirectly, through blitz-dom's inline layout pass, whose float placement now passes a caller-defined float id and whose layout behaviour moved with the eight ratified upstream commits.
@task_fact n=1 slug=dependency-security
fact: Taffy is the workspace's one git dependency, pinned to upstream DioxusLabs/taffy at the full commit rev e90751fd with no patch section; its lock entry carries no checksum, so its integrity rests on the commit rev pin, and every other dependency stays a crates.io registry version with a checksum.
@task_item n=1 part=untouched
The driver session socket, the MCP server and their contracts (request lines, replies, refusals, exit codes, tool results) are untouched: no verb, wire form or bound moved.
@task_item n=2 part=untouched
Authentication and authorization rows, CI tokens and publish guards, and secret management rows are untouched: no workflow, secret or environment variable changed.
@task_item n=3 part=untouched
Data protection rows (history database, HTTP cache, cookies, kept check files, recording directories) are untouched.
@task_item n=4 part=untouched
Logging and monitoring rows, the telemetry sink allowlist and span contracts are untouched: no log target or field moved.
@task_item n=5 part=untouched
Outbound request controls (blitz-net, per-host limits, GET-only sub-resources, CORS and same-origin absences, external navigation schemes) and script fetcher rows are untouched.
@task_item n=6 part=untouched
Select list, password masking, stable element id, snapshot and accessibility rows are untouched: no control, carrier or reader changed.
@task_item n=7 part=untouched
CI action SHA pinning, signing material and the coverage floors are untouched; no test was added and no tally moved.
