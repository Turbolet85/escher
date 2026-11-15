@task_prose part=session
The session moved the workspace's Taffy dependency from the crates.io registry 0.15.1 to upstream Taffy's git repository at rev e90751fd (package version 0.15.0, MIT, rust-version 1.71), its features unchanged and with no patch section.
Taffy is now the lock's one git source: the workspace's git-source count went from 0 to 1, under the Taffy-only exception to the no-git-dependencies rule. No other package moved.
Eight upstream blitz commits (Taffy #1303 and the chain behind it) were taken by no-commit cherry-pick. The lock file was the only conflict in each pick, resolved as ours and re-resolved.
The one layout call that places a floated box in inline layout now passes a caller-defined float id (the node's Taffy id) as its first argument. That is Taffy #1303's API, and no public symbol of the workspace changed.
The founder ratified the eight commits' layout changes for every document for this take (not provisional). Taffy pins from #1372 on (reversed scroll origins, its panic fix and five later bumps) stay deferred to their own route entry, because they leave blitz-dom uncompiled.
Measured: every test tally equals the base's (test 1464 passed, stand 294, stand-repeat 3 of 3, a11y 115), msrv and audit green, and CI 17 of 17 green.
@task_file n=1 path=Cargo.toml
what: edited — the workspace manifest's shared dependency declarations; the taffy entry moved from a registry version to a git source pinned to a rev
@task_file n=2 path=Cargo.lock mark=conflict
what: edited, with a conflict resolved in it on each cherry-pick — the workspace lock file and its resolved dependency set; the taffy package's version and source changed to git and its checksum line is gone
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: edited, by a cherry-picked upstream commit — blitz-dom inline layout, the inline layout pass that places floated boxes through Taffy
@task_item n=1 part=touches
decision/taffy-layout: Taffy for layout is now upstream Taffy git at rev e90751fd (0.15.0), no longer the registry 0.15 version, and it sits under the Taffy-only git exception.
@task_item n=2 part=touches
decision/no-git-dependencies: the half of the rule that names taffy a registry version is stale. Taffy is the lock's one git source, and the except clause stands as written.
@task_item n=3 part=touches
decision/deferred-upstream-dependency-bumps: Taffy is no longer 0.15.x waiting on #1303. It is git at e90751fd with #1303 taken, and the later Taffy chain (from #1372, reversed scroll origins, and the five later bumps) still waits for its own take.
@task_item n=4 part=touches
decision/upstream-take-engine-changes: the eight taken commits change layout for every document as an upstream take. They are ratified by the founder for this take, on the condition that gates, stand, a11y and CI are green, which held.
@task_item n=5 part=touches
decision/upstream-behaviour-changes-ratified-per-take: this take's layout behaviour changes were ratified per take by the founder, landing ratified and not provisional.
@task_item n=6 part=touches
decision/upstream-taken-by-cherry-pick and decision/upstream-sync-merge-base: this session took upstream by no-commit cherry-pick of eight commits, with the lock re-resolved after each.
@task_item n=7 part=touches
decision/workspace-dependencies and decision/workspace-shared-dependencies: the shared workspace declaration of taffy is now a git source with a rev, not a registry version.
@task_item n=8 part=touches
decision/dependency-and-licence-audit, decision/cargo-deny-advisory-gate and decision/cargo-deny-audit: the audit now covers a git source (taffy, MIT). It passed with advisories ok and licenses ok, and any allowed-git-source setting a row holds is touched.
@task_item n=9 part=touches
decision/msrv-policy: the git Taffy declares rust-version 1.71, below the workspace's, and the msrv leg stays green.
@task_item n=10 part=touches
contract/layout/taffy-traits and contract/layout/stylo-taffy-exports: blitz-dom's Taffy traversal and layout traits now sit on git Taffy 0.15.0 at e90751fd. Placing a floated box takes a caller-defined float id.
@task_item n=11 part=touches
decision/deferred-inline-layout and decision/one-incremental-pipeline: the inline layout pass's float placement changed through #1303's float id, with layouts identical in both layout modes per the tallies.
@task_item n=12 part=touches
decision/direct-engine-changes-flagged-upstreamable: the inline layout change is upstream's own, taken by pick, and not a direct escher engine change.
@task_item n=13 part=touches
decision/locked-cargo-legs and decision/cargo-legs-locked: the locked legs now resolve one git source from the lock.
@task_fact n=1 slug=project-intent
fact: Upstream Taffy pins from #1372 (c2560562) on leave blitz-dom uncompiled. The reversed-scroll-origin chain (upstream 0980cce5f, its panic fix and five later Taffy bumps) is deferred to its own take, where its behaviour change is asked.
@task_item n=1 part=untouched
Stylo, Parley, Dioxus, winit, AccessKit, Boa and every other engine dependency: none moved, and no other lock package changed.
@task_item n=2 part=untouched
The driver, session, MCP, snapshot, kept-check, lint and recording contracts, with their verbs, wire forms and exit codes: no carrier or answer shape changed.
@task_item n=3 part=untouched
Accessibility tree, event dispatch, select list, form control, scroll and hit-testing contracts: their behaviour and tallies are unchanged.
@task_item n=4 part=untouched
CI workflow, ci-leg legs, coverage floors, stand-repeat budget and gate thresholds: no leg, floor or bound moved.
@task_item n=5 part=untouched
Occupied names (crates, binaries, servers, environment variables): none taken or freed.
@task_item n=6 part=untouched
Telemetry, persistence, networking and browser app rows: not reached by this take.
