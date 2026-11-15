@task_prose part=session
The eleventh upstream take: upstream/main read at ea07ee716 (16 new commits), 3 taken by cherry-pick as upstream wrote them, 13 left out, the left-out total 99 to 112, merge base unmoved.
Taken: an @font-face rule's font-stretch (font-width) descriptor now reaches font matching as a new public width field on the font-face overrides; the WPT pinned commit moves; the WPT timeout quarantine gains four dynamic-load-events entries.
The four Taffy-main commits were measured (they pick clean, every leg green) and left out for the no-git-dependency rule; their content arrives instead by a lock-only registry move taffy 0.15.0 to 0.15.1, all but Taffy #1303 (the caller-defined float id), which no 0.15.x release carries.
The manifest is unmoved (taffy stays "0.15" from crates.io), no package enters or leaves the lock, no git source; audit and MSRV green; no log site, IPC method, verb, carrier field or surface added.
The founder ratified both behaviour changes for every document (the font-stretch read, Taffy 0.15.1's layout fixes) and lifted the no-git-dependency rule for Taffy alone; a later chunk moves Taffy to upstream's git version with #1303 and the left-out chain.
The founder also put rdme and blitz, upstream's demo shells with no tests, outside the mutation tier's scope.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=merge
what: the blitz-dom document core (BaseDocument): where @font-face overrides are applied when a font face is registered, and its font-face override unit tests
@task_file n=2 path=packages/blitz-dom/src/net.rs mark=merge
what: blitz-dom's network resource handling: fetched fonts and @font-face rules read into font-face overrides (family, weight, style, now width) for font registration
@task_file n=3 path=wpt/WPT_COMMIT mark=merge
what: the WPT pinned commit the WPT runner tests against
@task_file n=4 path=wpt/runner/timeout-quarantine.txt mark=merge
what: the WPT runner's timeout quarantine list, each entry a test path with a known reason, skipped by default
@task_file n=5 path=Cargo.lock
what: the workspace lockfile: resolved crates.io registry versions with checksums, taffy among them
@task_item n=1 part=touches
decision/font-face-css-descriptors is stale: font faces are now registered under the CSS-declared family, weight, style and width (the font-stretch descriptor), ratified by the founder.
@task_item n=2 part=touches
decision/deferred-upstream-dependency-bumps is stale: the Taffy-main content upstream pins now arrives by registry taffy 0.15.1 save Taffy #1303; #1303 and the later git chain wait for a dedicated git-Taffy chunk, no longer for a registry release or the founder's answer.
@task_item n=3 part=touches
decision/no-git-dependencies is amended on the founder's word: an exception permits a git source for the taffy crate alone; every other crate stays a crates.io registry version; today no crate is a git source (taffy resolves to registry 0.15.1).
@task_item n=4 part=touches
decision/taffy-layout: Taffy 0.15 from the crates.io registry stays true; the resolved version moved 0.15.0 to 0.15.1 by a lock-only semver-compatible update.
@task_item n=5 part=touches
decision/upstream-take-engine-changes records the previous take (563d3db73, twenty-nine commits) and stays true of it; this take's two ratified behaviour changes stand beside it, not as an edit of it.
@task_item n=6 part=touches
decision/upstream-behaviour-changes-ratified-per-take governs this take: its two behaviour changes for every document were ratified by the founder.
@task_item n=7 part=touches
decision/upstream-sync-merge-base and decision/upstream-taken-by-cherry-pick: the take followed the cherry-pick shape with the merge base unmoved; the sync is now read at ea07ee716 with 3 taken and 112 left out, each with a measured reason.
@task_item n=8 part=touches
decision/mutation-tier-host-only: the mutation tier's scope now excludes rdme and blitz, which hold no tests; the exclusion belongs beside this row.
@task_item n=9 part=touches
name/crate/rdme and name/crate/blitz: the two units the mutation tier no longer runs; the names stay occupied and unchanged.
@task_item n=10 part=touches
decision/wpt-quarantine-skipped-by-default and decision/wpt-skipped-tests: the timeout quarantine list gained four entries with the known reason dynamic-load-events.
@task_item n=11 part=touches
decision/wpt-mirrors-upstream-tooling: the WPT pinned commit moved with upstream's.
@task_item n=12 part=touches
decision/skipped-font-formats and decision/no-local-font-sources: rules over the @font-face reading the session widened by one descriptor; they hold as written.
@task_item n=13 part=touches
decision/dependency-and-licence-audit, decision/cargo-deny-audit and decision/cargo-deny-advisory-gate: the lock move passed the audit (advisories ok, licenses ok); a future git Taffy must still pass the lock, audit and MSRV legs.
@task_item n=14 part=touches
decision/workspace-dependencies and decision/workspace-shared-dependencies: the workspace dependency declaration is unmoved (taffy "0.15"); only the resolved version changed.
@task_item n=15 part=touches
contract/layout/taffy-traits, contract/layout/stylo-taffy-exports, decision/table-layout-by-taffy-grid and decision/layout-modes-identical: Taffy 0.15.1 brings layout fixes for every document under these; both layout modes and the incremental oracle read no count moved.
@task_item n=16 part=touches
decision/msrv-policy and decision/msrv-job-build-only: taffy 0.15.1 declares rust-version 1.71, under the workspace's; the MSRV leg green.
@task_fact n=1 slug=established-decisions
fact: The upstream take read at ea07ee716 (three cherry-picks) brings two engine behaviour changes for every document, both ratified by the founder: an @font-face rule's font-stretch descriptor reaches font matching, and Taffy 0.15.1's layout fixes.
@task_fact n=2 slug=established-decisions
fact: The no-git-dependency rule has one exception: the taffy crate alone may come from a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
@task_fact n=3 slug=stack-and-technologies
fact: The mutation tier does not run rdme and blitz, upstream's demo shells that hold no tests, and no test is owed them for it; churn, sizes and the other collectors keep both units.
@task_item n=1 part=untouched
The headless driver, its verbs, the session socket, the MCP role, kept checks and the recording directory: no verb, carrier field, refusal cause or wire item moved.
@task_item n=2 part=untouched
The snapshot, the diff, stable element ids, the actionable rule and the UI graph contracts: no reader or key changed.
@task_item n=3 part=untouched
Telemetry, scrubbing and logging rows: no log site, span or field was added.
@task_item n=4 part=untouched
Accessibility tree, event dispatch, focus, select list and form control rows: no behaviour there changed.
@task_item n=5 part=untouched
Occupied resources: no listener, port, socket, environment variable or state directory was added or freed.
@task_item n=6 part=untouched
Paint, scrolling and hit testing, script runtime and the browser app rows: none of those components changed.
@task_item n=7 part=untouched
Project intent rows on naming, the README and the capability statement: no capability was claimed or removed.
