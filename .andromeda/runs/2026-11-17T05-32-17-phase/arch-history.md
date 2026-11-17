@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — (headless stand surface, HarnessOptions fields) registered `seven_guis::stand`: pinned 800 × 600 · scale 1.0 · Light viewport, `font_ctx()` (bundled DejaVu Sans, system fonts off), `options(incremental)` offline, a fresh `VirtualDom` per boot through `app::task_in_shell`, `TaskShell` private; `HarnessOptions` gained `font_ctx` and `incremental`. These are the exact boot conditions the isolated mount must reproduce (decision/stand-pinned-environment, decision/stand-offline, contract/headless-stand/boot); the boot path is what P3 must show can root a VirtualDom at a non-root component.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — recorded the id grammar as built: the component path tier is `/`-joined component segments below the app root, dioxus-core's four framework scopes adding none, a segment the last `::` segment of `VComponent.name`, a same-name sibling `{name}:{k}`; the app root's own elements read `/{tag}:{n}…`. Why it matters: an isolated component becomes the root, so its ids lose the in-app prefix and its own elements fall to the app-root form — the scope's «ids differ by a prefix» equality P4 writes rests on this grammar (contract/stable-element-id/component-path, contract/stable-element-id/document-path, contract/stable-element-id/tier-distinctness); trap kept: an app root rendering a top-level `html` would read the skeleton's `/html:0`.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — proved ids persist across re-render, remount and a second process, except the four document-skeleton elements `DioxusDocument::new` builds outside the VirtualDom (`/html:0`, head, body, `main`). The isolated document carries the same skeleton, so the snapshot equality must exclude or account for those four (contract/stable-element-id/persistence).

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the anchored tier `{key}//{segment}`, reaching inside one component instance only (a template root owned by another component restarts at its chain); anchoring across a component boundary was weighed and ruled out by the founder. Author-keyed and anchored ids do not depend on the app prefix, so they are the readings most likely to compare equal between isolated and in-app mounts; the one-component reach must not be widened to make ids match (decision/anchored-id-tier, contract/stable-element-id/anchored-path).

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the `Document` trait gained the defaulted `accessibility_tree()`, overridden by `DioxusDocument` to write each element's stable id as AccessKit `author_id`; under `default-features = false` a crate gets it only by naming the `accessibility` feature, and a feature-gated item can pass under `--workspace` yet be compiled out under `-p` (trap). The isolated mount's snapshot reads through this override (contract/document-trait/accessibility-tree, contract/accessibility-tree/author-id, decision/accessibility-feature-gate).

@history_item n=6 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — registered `DioxusDocument::snapshot` (nodes are author_id nodes whose element resolves; no NodeId, tree id, ElementId, ScopeId or pointer) and the standing rule that the snapshot derives no role or name of its own. The reading the chunk compares (ids, roles, names, states, bounds) is this model (contract/snapshot/membership, contract/snapshot/snapshot-node, contract/snapshot/no-process-local-values).

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (the driver session registered) set that the session library takes the app's boot from its caller and depends on no app, and the standing rule that the first request carrying screen content is a new crossing question. An isolated mount entry point must keep the boot in the app (seven_guis), not in escher-driver (decision/session-library-depends-on-no-app, decision/new-carrier-widens-boundary).

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the executor was built in process only on the operator's fork answer, with the rule «the first command carrying a call or an outcome out of the process is a new crossing question». Precedent for the founder's fork option «in process only» and for why any carrier of the isolated mount must wait for his word (scope boundary fct-7plya4en).

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (nine verbs in two levels) added the session-level `start` (`app`, `ArgKind::Name` 1 to 32 bytes of `a-z0-9-`), `validate_session`, the `command_line` reader and its endings, and ratified the socket crossing calls. A «closed table of named components and prop sets on `start`» option would extend this `start` row and `validate_session`; a new argument on the command line is itself a widening (decision/socket-carries-driver-calls, decision/validated-verb-set-widened-by-ratification, contract/validation-order/first-failure).

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP server and `mcp` role over stdio, ratified as a boundary widening, with the rule that the next surface carrying a call or a screen's content is a new crossing question; `start` boots tasks through the same boot. Any carrier option of the fork reaches the MCP `start` tool's schema here (decision/mcp-stdio-crossing, decision/mcp-role-no-listener).

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — ratified that the command line's stdout and the `mcp` role's stdio carry the program's own fixed `'static` table text, while text from a page, a call or a screen, a new carrier or a new exit still widens. A closed table of named components and prop sets would be fixed program text; open props over the carriers would be call-supplied text — the line the fork's prices turn on (decision/fixed-program-text-on-carriers).

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the harness's screenshot renders through the Vello CPU image renderer (no multithreading), composited over white, refused `FontsUnavailable` when no sans-serif family resolves. If the isolated reading's bounds or a shot is compared, it reads through this path and needs the bundled font (decision/headless-screenshot-cpu-renderer, contract/test-harness/screenshot, contract/base-document/can-paint-text).

@history_item n=13 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (the bridge drops a removed node; the default sheet from the config) `DioxusDocument::new` adds no sheet, the default user-agent sheet arriving once through `DocumentConfig::default()`, a config setting `ua_stylesheets` used as given; a removed node is dropped and reads no id. An isolated document built through a hand-made config must keep the default sheet or its layout and bounds will not match the in-app reading (decision/dioxus-no-own-stylesheet, decision/ua-stylesheets-used-as-given).

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Test harness and Headless stand) the harness holds one `VirtualClock` its documents share: `HarnessOptions::into_config` puts it in `DocumentConfig.clock`, so `from_html_with` and `from_vdom` (and `from_html`, `from_component`) read it, while `wrap(doc)` keeps the wrapped document's own clock. Which constructor the isolated mount uses (`from_vdom`/`from_component` vs `wrap`) decides its clock and seed, so the boot conditions match a stand task's (decision/one-virtual-clock, contract/test-harness/virtual-clock).

@history_item n=15 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — a session whose launcher names a recording directory keeps its answered calls at rest as an owner-only kept check opening with a `start` record of the app; ratified as a boundary widening (call content at rest, a new command-line argument). If the founder's answer gives the isolated mount a carrier, a kept check of it must state what its opening record names (decision/kept-check-masking, decision/kept-check-never-logged).

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (the replay API) made `pub fn TaskShell` public so a test-local variant mounts in the shell, and `Check::replay` refuses a check of another app (`another_app`). The isolated mount is a further case of mounting something other than a whole task inside the stand's shell; the replay's app-label match bears on any kept check of an isolated component (decision/kept-check-replays-through-run, contract/driver-recording/replay).

@history_item n=17 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`Parallel` style traversals serialised) recorded that no headless document — harness, stand or driver — sets `style_threading`, `Sequential` the default. The isolated mount is a new headless document and must keep that (decision/headless-documents-default-threading, decision/headless-documents-sequential-style).

@history_item n=18 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the headless stand serves Cells and a load page) added `boot_cells` and `boot_load` beside the lean boots, the private `enum Served`, `TASKS: [(&str, Served); 6]` from which the usage line's `apps:` and the MCP `start` tool's `enum` are written, every boot offline but the load page's. A closed table of named components would follow this one-table pattern; the cold-agent entries of the same marker set the pipe's isolation check against the server's tools (decision/stand-offline, contract/escher-session/invocation, contract/cold-agent-verdict/passed).

@history_item n=19 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the stand's typography, F2 ratified) the stand's typography is DejaVu Sans, its one bundled font, stated in the Headless stand contract: `font_ctx()` with system fonts off, every stand boot holding it through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included. The isolated mount is one more boot that must hold it (decision/stand-typography-dejavu-sans, decision/stand-uses-bundled-font).

@history_item n=20 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (a node's component through Dioxus public API) a node's owning component reads with no fork-level change by a walk from `VirtualDom::base_scope` through `ScopeState::try_root_node`, `VNode.dynamic_nodes`, `VComponent::mounted_scope`, each named by `VComponent.name`; hook-local `use_signal` state is unreachable without a fork change. This is how the graph names the component a mount is keyed by, and what an isolated root's `base_scope` changes (decision/owning-component-via-public-api, decision/graph-state-by-registration).

@history_item n=21 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — `initial_build` and `poll` render into the crate-private `MutationBuffer` unborrowed (`vdom.rebuild` / `render_immediate`), borrow once and replay, a ratified engine change for every Dioxus document; dioxus-core 0.7.10's first-poll paths read; limit: a node created in the same render is not yet in the document. The isolated mount builds through this path, and the folded CARRY on `boundary_marks()`'s fallback (a document built after its vdom's first build) is a re-run-`rebuild` trap measured there (decision/bridge-buffered-render, decision/render-with-document-unborrowed, decision/dioxus-mutation-buffer-replay).

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the UI graph crosses the ratified carriers as the `graph` verb's answer and `DioxusDocument::graph` reads one `GraphNode { id, owner, component, … }` per snapshot node; the graph registry's first registering hook provides a `SignalRegistry` at `ScopeId::ROOT`; seven_guis registers state through `tasks::use_graph_signal`. «Named across the graph's component boundary» means a component name this graph reads, and an isolated root moves which scope is `ScopeId::ROOT` for the registry (decision/ui-graph-crossing, contract/ui-graph/graph, contract/graph-registry/use-graph-state, decision/first-graph-state-hook-provides-registry, decision/seven-guis-graph-signals, decision/graph-signal-hook-in-seven-guis).

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the reached record; the coverage role) the reached record is provided at `ScopeId::ROOT` and the dispatch change stays inert without a registry or a switched-on record; the `coverage` role boots one fresh session per check through the host's boot closure. Another root-context consumer an isolated root must keep working, and a precedent for a role reading named inputs refused `malformed` holding nothing of the path (decision/dioxus-event-path-records-opt-in, decision/coverage-role-in-process).

@history_item n=24 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — one run seed as a Dioxus root context, `Harness::from_vdom` providing `SeededRandom::from_seed(DEFAULT_SEED)` where the root holds none and `Harness::wrap` providing none; no seed on any carrier (provisional, later ratified). The scope's «with the run's default seed» holds only if the isolated mount boots through `from_vdom`, not `wrap` (decision/one-run-seed, contract/test-harness/run-seed, decision/no-seed-on-carriers, decision/no-prng-crate).

@history_item n=25 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — every stand boot loads one font, DejaVu Sans, every generic and unavailable named family laying out in it by fallback (guard `stand_bundled_font_families`); the six served tasks read the same driver answer digests on linux, windows and macOS in both layout modes. The isolated mount's «stand's font» means this, and an equality test of it should hold across OSes like the stand readings (decision/stand-typography-dejavu-sans, decision/stand-readings-same-on-every-ci-os).

@history_item n=26 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — the scope's named precedent: the founder answered its F1 «in process only», landing `Session::reach`, `stand::start_in` (refusing a name outside the lean task's closed fixture table before anything boots) and the absence `absence/fixture-state-on-carriers`; the conventions entry adds spelling `std::result::Result` under the Dioxus prelude glob (Android-only failure). The fork's three options and their prices, the refuse-before-boot validation of named inputs, and the Result trap all carry over (decision/std-result-under-dioxus-prelude, contract/headless-stand/start-in).

@history_item n=27 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — exploration landed as one in-process library call on the founder's answer A, with `absence/exploration-on-carriers`, and the kept check's opening record gained `"format":1`. A second in-process-only precedent and the current opening-record shape any kept check of an isolated mount would follow (absence/exploration-on-carriers).

@history_item n=28 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — added `root_error_mark`, `take_step_reached` and the ungated `boundary` module (decision/accessibility-feature-gating now lists `signal_registry`, `reached`, `seed`, `boundary` ungated), and `driver-spans-only` / `driver-library-side-effect-free` narrowed to «no global subscriber». The boundary reading reads the vdom's root boundary, which an isolated root changes, and any new isolated-mount module must state its gating (decision/accessibility-feature-gating, decision/driver-spans-only).

@history_item n=29 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — registered `contract/dioxus-native-dom-api/boundary-marks`: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, `boundary_marks` reads them by identity. The folded freight cry-c4emakc4 is this reader's untested fallback for a document built after its vdom's first build; the chunk owes a test of it, by the isolated mount or a fixture, without re-running `rebuild` on a built vdom.

@history_item n=30 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — landed `decision/layout-read-in-process-only` PROVISIONAL on the delegate's answer, pending the founder's word, with `contract/driver-session/layout`. A recent precedent of an in-process-only answer later widened by his word, and of how a delegate answer is marked when the founder is slow — the fork for this chunk is raised to him and not taken for him.

@history_item n=31 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the fifteenth verb `layout` widened the verb table, MCP `tools/list`, the kept-check reader to eleven instance verbs and `decision/no-node-id-across-session`, retiring `layout-read-in-process-only` for `decision/layout-answer-crossing`; its operations carry the delegate marks the founder's «Подтвердить обе» now lifts. The cold-agent pipe's isolation check compares its verb list with these fifteen tools, so `layout` joining `scripts/cold-agent.sh` and `test_cold_agent.py` restores it; any carrier for the isolated mount would widen this same table again (contract/verb-table/order, contract/mcp-server/tools-list, contract/kept-check/kept-verbs, decision/no-node-id-across-session).

@history_item n=32 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe runs on the workspace's own `escher-session mcp`; `passed` needs `isolated` on exactly the server's `mcp__escher__{verb}` tools and one server, pinned by `test_cold_agent.py` under a `claude` shim. This is the isolation check `layout`'s absence from the pipe's list breaks (decision/cold-agent-mcp-server-is-own, decision/cold-agent-isolated-session, decision/cold-agent-client-host-only).

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (thirteen verbs where a surface counts them) `cold-agent.sh` and `test_cold_agent.py` `VERBS` gained `graph`, the pipe's isolation atom reading thirteen tools; the same move followed for `actions` at 2026-11-08-what-can-i-do-here and `lint` at 2026-10-19-visual-lint. The precedent for adding `layout` to the pipe's verb list in this chunk.

@history_item n=34 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (fourteen verbs) the cold-agent pipe's escher tools went from thirteen to fourteen with `actions`, and its `keyed` field rested PROVISIONAL on a delegate until the founder's word (lifted at 2026-11-13T07-43-19-wrap). Precedent both for the pipe verb-list edit and for recording a lift of a delegate mark at a wrap.

@history_item n=35 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — carried the founder's own word over the coverage role's PROVISIONAL (delegate overseer) entries as «was PROVISIONAL (delegate overseer) … now ratified, the entry's text stands as written». The form this chunk's wrap follows when it records the lift of the 24 delegate marks of the two last wraps.
