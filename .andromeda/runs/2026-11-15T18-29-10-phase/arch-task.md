@task_prose part=chunk
Moves the workspace's taffy dependency from the crates.io registry 0.15.1 to upstream's git pin, rev a3997d55 of dioxuslabs/taffy (the rev upstream blitz names at ea07ee716), so the tree carries Taffy #1303 (the caller-defined float id) and the Taffy content of upstream's left-out chain.
Takes the blitz side of that chain as upstream wrote it where it touches escher's tree: 1a11761bc's float-placement line and the code edits of the nine "Bump Taffy" commits b8f4ba6fd … b4952e15f, and no other upstream pick; the lock moves with it, one taffy package from a git source and nothing further than the move forces.
Before any plan, re-measures the move in a trial worktree off HEAD: whether the workspace builds; the fast, clippy, audit, msrv (rust 1.91), doc and build legs, the incremental oracle, the stand checks and the WPT runner's build; the lock's added, removed and moved packages; and the wasm legs, wasm_hello's own lock read on its own.
Stops with the measurement if the move cannot be taken cleanly; writes any fork that widens a boundary or needs a master rule lifted beyond the Taffy-only exception to the run's forks file and stops.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace root manifest — the shared workspace dependency list every crate takes taffy from, and the build profiles — edits: the workspace taffy dependency entry (registry version to git source and rev)
@task_file n=2 path=Cargo.lock mark=edits
what: the workspace lockfile every cargo leg runs locked against — edits: the taffy package entry (registry source to a git+https source at the rev), and other packages only as the move forces
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=edits
what: blitz-dom's inline layout, where Parley inline content places floated boxes through Taffy's block-formatting context — edits: the place_floated_box call (its new first argument, the node's Taffy node id) and the float-inset and float-segment edits the chain's Taffy bumps bring
@task_file n=4 path=packages/blitz-dom/src/layout/mod.rs
what: blitz-dom's layout module, where the document implements Taffy's traversal, layout and cache traits (the cache read b4952e15f moves to a shared borrow)
@task_file n=5 path=packages/blitz-dom/src/node/node.rs
what: the DOM node, holding its Taffy layout cache and layout output — the cache accessor b4952e15f adds beside them
@task_file n=6 path=deny.toml mark=re-reads
what: the cargo-deny dependency and licence audit — its advisories and licences checks, re-run on the git Taffy
@task_file n=7 path=.github/scripts/ci-leg.sh mark=re-reads
what: the one CI leg runner — the fast, clippy, audit, msrv, doc, build and wasm legs it runs, re-run in the trial worktree
@task_file n=8 path=tests/blitz-tests/tests/incremental_oracle.rs mark=re-reads
what: the incremental-layout oracle — the check that incremental and full layout give identical layouts, re-run on the moved Taffy
@task_item n=1 part=needs
The Taffy layout dependency itself: the workspace takes taffy from upstream's git at rev a3997d55 in place of the registry 0.15.1, the one Taffy every crate shares through the workspace dependency list, ending the deferral of the Taffy bump.
@task_item n=2 part=needs
The no-git-dependencies rule and its Taffy-only exception: taffy alone may take a git source; any second git source the move would bring (a Taffy dependency from git, an active patch section) is outside the exception and is a fork for the forks file, never settled by the chunk.
@task_item n=3 part=needs
Every cargo build, test and CI leg stays locked: the moved lock must be the one the locked legs resolve, with one taffy package from a git source and no package moved beyond what the Taffy move forces.
@task_item n=4 part=needs
The dependency and licence audit: the cargo-deny advisories and licences checks stay as they are and must still pass with a git-sourced Taffy and whatever packages the move adds.
@task_item n=5 part=needs
The MSRV policy: rust-version stays 1.91 and the msrv leg (build only) must build the moved workspace at 1.91; an MSRV above 1.91 is a stop, not a raise.
@task_item n=6 part=needs
Taffy #1303, the caller-defined float id: inline layout's float placement into Taffy's block-formatting context now passes the floated node's Taffy node id as its first argument.
@task_item n=7 part=needs
The Taffy content of the left-out chain as it reaches blitz-dom: float insets and zero-height float segments in inline float placement (taffy#1385, taffy#1386), and the layout cache read taking a shared borrow in the Taffy traversal, layout and cache traits blitz-dom implements and the stylo_taffy exports it uses.
@task_item n=8 part=needs
The layout behaviour change: a Taffy move changes layout for every document, remote pages included; whether the founder's answer stands as the ruling or each layout change is ratified per take, as upstream behaviour changes are, is a fork the measurement names and does not settle.
@task_item n=9 part=needs
The upstream take is exactly this chain: the nine Taffy-bump commits and 1a11761bc's float line, taken as upstream wrote them under the cherry-pick and merge-base rules for upstream changes; no other upstream pick, and the direct engine change flagged as upstream's.
@task_item n=10 part=needs
The measured-before-planned stop rule: a red gate or leg with no in-scope fix, an MSRV above 1.91, a broken wasm leg or a lock move beyond Taffy that a master rule refuses ends the phase with the measurement and no plan.
@task_item n=11 part=needs
The incremental oracle invariant: one pipeline for incremental and full layout, both layout modes giving identical layouts, must hold on the moved Taffy.
@task_item n=12 part=needs
The wasm legs: wasm_hello is a standalone workspace with its own lock outside the root's locked legs, so its taffy resolution is read on its own, and the wasm leg and Trunk browser-target builds must still build.
@task_item n=13 part=needs
The stand checks: the committed stand checks and their readings, identical on every CI OS under the stand-repeat zero-failure budget, are re-run on the moved Taffy; a reading the layout change moves is part of the measurement.
@task_item n=14 part=needs
The fmt, clippy and rustdoc gates: clippy and the workspace rustdoc warnings gate (warnings are errors) must pass over the edited inline layout and layout trait code.
@task_item n=15 part=needs
The doc, build and WPT runner build legs: the workspace docs, the build leg and the WPT runner's build are among what the measurement re-runs.
@task_item n=16 part=needs
The commented local Taffy patch section in the workspace manifest stays commented: activating a patch is a second source outside the exception.
@task_item n=17 part=needs
A registry Taffy 0.16 is out of scope, none published as at the earlier record; whether one has since appeared is re-read at P3.
@task_item n=18 part=needs
CI at take-up: the last wrap's run is read as in progress, never as green, when the measurement compares against HEAD.
@task_standing n=1
convention: a regression test the move adds for a Taffy or float behaviour cites the upstream commit or issue it guards
home: conventions
@task_standing n=2
convention: an integration test the plan adds lives in blitz-tests, one behaviour per test file, one test target per file, running in both layout modes
home: conventions
@task_standing n=3
convention: a new test asserts its fixture's precondition first and carries a descriptive name
home: conventions
@task_item n=1 part=not_needed
The driver's carriers: the session socket wire, MCP role and server, command line verbs, answer JSON, refusal causes, kept checks and recordings; the chunk adds no verb, carrier or answer field.
@task_item n=2 part=not_needed
The accessibility tree, AccessKit adapter and ARIA state contracts; nothing the move takes touches accessibility.
@task_item n=3 part=not_needed
The browser app: its history, persistence, about pages, toolbar and Dioxus chrome; the move touches the engine's layout dependency only.
@task_item n=4 part=not_needed
Script and JavaScript: Boa, the script fetcher, script globals and the CSSOM; no script surface moves.
@task_item n=5 part=not_needed
Telemetry: the escher-telemetry bootstrap, log line, scrubbing and spans; the chunk adds no log line or span.
@task_item n=6 part=not_needed
The cold-agent and agent-run tooling, their events, exit codes, state directories and verdicts.
@task_item n=7 part=not_needed
Select lists, form controls, form submission and DOM event dispatch; the chain changes float placement and layout caching, not form or event behaviour.
@task_item n=8 part=not_needed
The Dioxus bridge, stable element ids, the UI graph and handler coverage readers.
@task_item n=9 part=not_needed
Occupied resources: listeners, the session state and recording directories, the model provider; the move opens no resource.
@task_item n=10 part=not_needed
The names register of crates, binaries and other names: the move adds no crate, binary or named value.
