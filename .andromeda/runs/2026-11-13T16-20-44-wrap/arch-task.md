@task_prose part=session
The session took upstream Blitz main up to 563d3db73 as 29 no-commit cherry-picks, every pick clean, the merge base 23b9b26a9 unmoved; the upstream take's reading moves from e42ee1f25 (twelve commits) to 563d3db73 (29 picks), and 99 upstream commits are left out, each with its measured reason, deferred to the next sync.
The take's behaviour widenings were ratified as upstream's by the founder for this take: pages decoded lossily, stylesheets decoded from bytes with BOM and @charset, the font element's face and size, valign, cellspacing and cellpadding as presentational hints, an img with an empty alt read as decorative, a default font when no listed family is available, an unresolvable href skipped instead of a panic, Windows absolute paths resolved, @font-face sources skipped by format and extension.
Every action of every CI workflow, the three upstream-only workflows included, is now pinned to a 40-hex commit SHA with its ref as a trailing comment, and the pin check moved into a new WorkflowPinTest that reads every workflow; the ci-scripts suite went from 110 to 112 tests.
The dependency audit gained one advisory ignore, RUSTSEC-2026-0459 (libsqlite3-sys via rusqlite 0.32 in browser-persistence), owned until the browser moves to rusqlite 0.41 or later; no dependency, crate, port, env var or public item was added.
Nine blitz-tests targets were added (seven upstream, two frame tests of blend modes and an inline box's own decorations), the two committed Counter checks were re-recorded (bounds and digests only), and the changed-set write-sites rule takes its select-list qualifier.
@task_file n=1 path=packages/blitz-dom/assets/default.css mark=merge
what: the user-agent stylesheet blitz-dom uses as given (textarea and select defaults)
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the blitz-dom accessibility tree builder (roles, names, states read from the DOM)
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=merge
what: BaseDocument, the core document: URL resolution, document flags, stylesheet loading, scale and resolve entry points
@task_file n=4 path=packages/blitz-dom/src/font_metrics.rs mark=merge
what: font metrics queries for CSS font-relative units (ch, ex) and their family selection
@task_file n=5 path=packages/blitz-dom/src/form.rs mark=merge
what: form controls and form submission data (checkbox and radio state)
@task_file n=6 path=packages/blitz-dom/src/mutator.rs mark=merge
what: DocumentMutator, the single DOM mutation path: attribute writes, checked state, link rel and href, cellpadding restyle
@task_file n=7 path=packages/blitz-dom/src/net.rs mark=merge
what: blitz-dom resource handlers: stylesheet decoding, @font-face source selection and font loading
@task_file n=8 path=packages/blitz-dom/src/resolve.rs mark=merge
what: the resolve pass that runs style, layout and transform resolution before paint and reads
@task_file n=9 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the Stylo integration: element presentational hints from legacy HTML attributes
@task_file n=10 path=packages/blitz-dom/src/stylo_device.rs mark=merge
what: the Stylo device: viewport, scale and media for the stylist
@task_file n=11 path=packages/blitz-dom/src/stylo_to_parley.rs mark=merge
what: the Stylo-to-Parley style conversion, font family lists and the default font fallback
@task_file n=12 path=packages/blitz-dom/src/url.rs mark=merge
what: document URL handling, file and Windows absolute path resolution
@task_file n=13 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: damage classes and the layout cache clearing they drive
@task_file n=14 path=packages/blitz-dom/src/layout/list.rs mark=merge
what: list item layout and markers
@task_file n=15 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: writing-mode layout support (vertical modes, opt-in)
@task_file n=16 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: inline layout of text and inline boxes, including inline-block and embedded boxes
@task_file n=17 path=packages/blitz-dom/src/node/element.rs mark=merge
what: element data on DOM nodes (attributes, special element data such as images and inputs)
@task_file n=18 path=packages/blitz-dom/src/node/node.rs mark=merge
what: Node, the DOM node: hit testing (hit_inner), cursor lookup, layout parent and DOM parent
@task_file n=19 path=packages/blitz-dom/src/node/text.rs mark=merge
what: text node data and its text content
@task_file n=20 path=packages/blitz-paint/src/render.rs mark=merge
what: blitz-paint rendering of the paint scene: boxes, inline content, transforms, decorations
@task_file n=21 path=packages/blitz/src/lib.rs mark=merge
what: the high-level blitz crate's launch and HTML loading API
@task_file n=22 path=tests/blitz-tests/tests/accessibility_roles.rs mark=merge
what: one of the fourteen named accessibility integration tests in the a11y leg (roles read from the tree)
@task_file n=23 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests target list, one mod line per target file, test = false kept
@task_file n=24 path=wpt/WPT_COMMIT mark=merge
what: the recorded commit the WPT suite is pinned to
@task_file n=25 path=.github/workflows/publish-browser.yml mark=merge
what: the upstream-only browser publish workflow, repository-guarded to DioxusLabs/blitz
@task_file n=26 path=.github/workflows/wpt.yml mark=merge
what: the upstream-only WPT CI workflow, repository-guarded, installing the wpt CLI and running the suite
@task_file n=27 path=tests/blitz-tests/tests/anonymous_block_transform.rs mark=merge
what: an upstream-taken blitz-tests integration target: transforms on a box wrapped in an anonymous block
@task_file n=28 path=tests/blitz-tests/tests/font_element_attributes.rs mark=merge
what: an upstream-taken blitz-tests integration target: the font element's face and size attributes
@task_file n=29 path=tests/blitz-tests/tests/font_family_fallback.rs mark=merge
what: an upstream-taken blitz-tests integration target: the default font when no listed family is available
@task_file n=30 path=tests/blitz-tests/tests/hidden_checkbox_input.rs mark=merge
what: an upstream-taken blitz-tests integration target: a label click toggling a hidden checkbox
@task_file n=31 path=tests/blitz-tests/tests/hit_inline_embedded_box.rs mark=merge
what: an upstream-taken blitz-tests integration target: hit testing an inline embedded box where it is painted
@task_file n=32 path=tests/blitz-tests/tests/link_cursor.rs mark=merge
what: an upstream-taken blitz-tests integration target: the pointer cursor over link content
@task_file n=33 path=tests/blitz-tests/tests/scale_change_overflow.rs mark=merge
what: an upstream-taken blitz-tests integration target: overflow rescaled on a scale change
@task_file n=34 path=deny.toml
what: the cargo-deny configuration: advisory ignore set with reasons, licence allow list
@task_file n=35 path=.github/workflows/wpt-post-results.yml
what: the upstream-only WPT results posting workflow (repository dispatch)
@task_file n=36 path=.github/scripts/test_ci_workflows.py
what: the ci-scripts unittest suite that pins the CI workflow contract, now with WorkflowPinTest over every workflow
@task_file n=37 path=tests/blitz-tests/checks/counter-cli.json
what: a committed stand-Counter kept check, recorded over the command line
@task_file n=38 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed stand-Counter kept check, recorded over the MCP role
@task_file n=39 path=tests/blitz-tests/tests/blend_mode_frame.rs
what: a new blitz-tests frame test: mix-blend-mode and isolation painted as composited, both layout modes
@task_file n=40 path=tests/blitz-tests/tests/inline_box_decorations_frame.rs
what: a new blitz-tests frame test: an inline box of blocks paints no decorations of its own, both layout modes
@task_item n=1 part=touches
decision/upstream-take-engine-changes is stale: its reading of the twelve commits at e42ee1f25 moves to the 29 picks read at 563d3db73.
@task_item n=2 part=touches
decision/upstream-sync-merge-base and decision/upstream-taken-by-cherry-pick hold for this take: no-commit cherry-picks, merge base 23b9b26a9 unmoved, the 99 left-out commits each with its measured reason; the Project Intent sync line moves to 563d3db73.
@task_item n=3 part=touches
decision/upstream-behaviour-changes-ratified-per-take: this take's widenings were ratified as upstream's by the founder's own word; the rule holds and is exercised.
@task_item n=4 part=touches
decision/pinned-actions is stale: it reads every ci.yml uses, while the pin check and the pinning now cover every workflow, all four pinned.
@task_item n=5 part=touches
decision/actions-pinned-to-commit-sha (every action a CI workflow uses pinned to a 40-hex SHA, ref as trailing comment) was stale at the landing and now holds: the probe over all workflows prints 0.
@task_item n=6 part=touches
A guard moved: the every-uses-pinned and rust-toolchain-names-its-toolchain checks (test_j, test_k) moved from CiWorkflowTest to the new WorkflowPinTest; any row whose guard names CiWorkflowTest for action pinning, such as decision/ci-contract-pinned, is stale in its guard.
@task_item n=7 part=touches
The upstream-only workflow rows (decision/upstream-only-workflows, decision/publish-workflow-upstream-only, decision/browser-publish-upstream-only, decision/upstream-ci-resources-upstream-only): their repository guards stand byte for byte, but their actions are now SHA-pinned, the WPT workflow installs the wpt CLI by cargo-binstall and its rust-toolchain step uses the pinned SHA with toolchain stable, and the publish workflow takes the latest Android NDK.
@task_item n=8 part=touches
decision/changed-set-write-sites is stale: its never-by-scroll list takes the select-list qualifier (a scroll that closes an open select list marks its select), within its 300-character bound.
@task_item n=9 part=touches
The dependency audit rows (decision/cargo-deny-advisory-gate, decision/dependency-and-licence-audit, decision/cargo-deny-audit): the advisory ignore set gains RUSTSEC-2026-0459 for libsqlite3-sys 0.30.1 via rusqlite 0.32, held until the browser moves to rusqlite 0.41 or later; rows that list ignore IDs or count them move.
@task_item n=10 part=touches
The rusqlite 0.32 rows (decision/browser-history-sqlite and its kin): the pin is unmoved, but it now carries an ignored advisory owned by a residual.
@task_item n=11 part=touches
decision/legacy-attributes-as-pres-hints: the presentational hints widen to the font element's face and size, valign, cellspacing and cellpadding; a table's cellpadding write restyles its descendants.
@task_item n=12 part=touches
decision/skipped-font-formats and decision/no-local-font-sources: @font-face sources are now chosen by a closed format-string match and, without a format, by the URL's file extension, unsupported sources skipped.
@task_item n=13 part=touches
Stylesheet loading rows (decision/failed-stylesheet-unblocks, decision/render-blocking-head-stylesheets, decision/failed-stylesheet-stops-blocking): stylesheets are decoded from bytes honouring BOM and @charset, link rel matches stylesheet case-insensitively, and removing a link's href with no stylesheet held returns early instead of unreachable.
@task_item n=14 part=touches
Font family rows (decision/generic-family-none-sans, decision/apple-system-font-alias, decision/parley-text-layout): when no listed family is available, the default font is used, including for ch and ex metrics.
@task_item n=15 part=touches
The accessibility tree contracts (contract/accessibility-tree/build-accessibility-tree, contract/accessibility-tree/name-sources): an img with alt empty reads as GenericContainer, decorative; the a11y leg goes 114 to 115 tests over the same fourteen files.
@task_item n=16 part=touches
Hit-test rows (contract/scroll-hit/hit-clip, decision/hit-stops-at-clip-padding-box, decision/hit-stops-at-paint-clip): hit_inner applies the inline root's content-box offset only to inline children and the self hit's returned x and y; escher's own clip early return now returns border-box coordinates for an inline root where the self hit returns content-box ones (no check reads them).
@task_item n=17 part=touches
The cursor walk for a link reads the DOM parent, not the layout parent, so text and inline-blocks inside a link get the pointer cursor; rows on anonymous blocks living only in layout children (decision/anonymous-blocks-in-layout-children-only) are beside it.
@task_item n=18 part=touches
Transform resolution: resolve_transforms never skips an anonymous box, and a scale change sets a document flag that resolves all transforms on the next resolve; rows on device-change coalescing, scale (contract/viewport/scale) and visible region through transforms are beside it.
@task_item n=19 part=touches
URL resolution: BaseDocument resolve_url now returns an Option where it panicked on an unresolvable href, its callers skip on none; Windows absolute paths resolve as file URLs; rows on default document URL, URL scheme routing and file URL loads are beside it.
@task_item n=20 part=touches
Page decoding: pages are decoded lossily with no panic on non-UTF-8; rows on the HTML document parser contract (contract/html-net/document-html-parser) are beside it.
@task_item n=21 part=touches
Checkbox and radio rows (decision/checkbox-change-from-engine, decision/input-control-data-at-construction): a checked write is read back into an existing checkbox state, and a checkbox or radio gets its state even with no box (display none).
@task_item n=22 part=touches
decision/committed-stand-checks: both committed Counter checks were re-recorded by the recording script; only bounds and digests moved (the first back-btn 73x25 to 72.578125x25).
@task_item n=23 part=touches
The both-layout-modes conventions (decision/both-layout-modes, decision/scenarios-both-layout-modes): six of the seven upstream-taken targets run one layout mode and were kept as taken, an exception; the two new frame tests run both modes; the rule binding the stand checks to both modes is unaffected.
@task_item n=24 part=touches
blitz-tests target rows (decision/blitz-tests-one-target-per-file, decision/blitz-tests-single-test-target, decision/integration-tests-in-blitz-tests): nine targets added, one file each, one mod line each in the target list.
@task_item n=25 part=touches
Frame-test coverage: blend modes and an inline box's own decorations now have frame tests; rows on mask isolation (decision/mask-isolation-forces-buffer) and paint emitting a paint scene are beside it, not changed.
@task_item n=26 part=touches
Toolchain rows (decision/latest-stable-toolchain, decision/fmt-and-clippy-gates, decision/fmt-clippy-gates): CI's stable rustc moved to 1.100.0 and its new clippy lint was cured by an upstream pick; no row names the version.
@task_item n=27 part=touches
decision/deferred-upstream-dependency-bumps and absence/served-network-port hold: the Dioxus 0.8 move and the CDP server's listener are among the left-out commits, deferred to the next sync, and the CDP listener needs a security-plan amendment first.
@task_item n=28 part=touches
The WPT pin moved to a newer recorded commit; WPT rows name no SHA and hold.
@task_item n=29 part=touches
CI Python rows (decision/ci-python-unittest, decision/ci-scripts-stdlib-python, contract/ci-leg-legs/ci-scripts): the ci-scripts suite grew from 110 to 112 tests on unittest and the stdlib; no row states the count.
@task_fact n=1 slug=established-decisions
fact: An unresolvable href or relative URL is skipped, never a panic: BaseDocument's URL resolution yields none and every caller skips it.
@task_fact n=2 slug=established-decisions
fact: Pages are decoded lossily: non-UTF-8 page bytes are replaced, never a panic.
@task_fact n=3 slug=established-decisions
fact: Stylesheets are decoded from their bytes honouring a BOM and @charset, and link rel matches stylesheet case-insensitively.
@task_fact n=4 slug=established-decisions
fact: The font element's face and size, valign, cellspacing and cellpadding cascade as presentational hints; a size without digits and a face that is not a family list are ignored.
@task_fact n=5 slug=established-decisions
fact: When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike.
@task_fact n=6 slug=standard-contracts
fact: In the accessibility tree an img whose alt is empty reads as GenericContainer, decorative and unnamed.
@task_fact n=7 slug=established-decisions
fact: A link's pointer cursor is found by walking DOM parents, so text and inline-blocks inside a link, even in an anonymous block, show the pointer.
@task_fact n=8 slug=established-decisions
fact: Transforms of boxes inside anonymous blocks are always resolved, and a scale change re-resolves every transform and rescales overflow on the next resolve.
@task_fact n=9 slug=established-decisions
fact: A checkbox or radio holds its checked state even without a layout box, and a checked attribute write is read back into it.
@task_fact n=10 slug=established-decisions
fact: Windows absolute paths given as a document or resource URL resolve as file URLs.
@task_fact n=11 slug=conventions
fact: Upstream-taken blitz-tests targets are kept as taken even where they run one layout mode, an exception to scenarios in both layout modes; escher's own targets run both.
@task_fact n=12 slug=infrastructure-patterns
fact: The action pin check reads every workflow file, not only the main CI workflow, and requires each pin's ref kept as a trailing comment.
@task_item n=1 part=untouched
The session driver, session socket wire, session host and session errors contracts: no verb, answer, wire version, bound or exit code moved.
@task_item n=2 part=untouched
The MCP role and MCP server contracts and the escher-session command line: no tool, method, revision or flag changed.
@task_item n=3 part=untouched
Snapshot, snapshot diff, snapshot text, stable element ids, actionable keys and the UI graph: none of their shapes or tiers changed.
@task_item n=4 part=untouched
The open select list rows (painting, rows, keys, wheel, default actions) other than the changed-set write-sites qualifier: no select behaviour changed.
@task_item n=5 part=untouched
Cold-agent, agent-run and apt-install contracts and their exit codes and events: untouched.
@task_item n=6 part=untouched
The names sheet: no crate, binary, server or env var name was taken or freed, and no crate was added or removed.
@task_item n=7 part=untouched
Occupied resources: no port, socket, listener, env var or IPC method was added; the one local Unix listener and the MCP stdio role stand.
@task_item n=8 part=untouched
Shared workspace dependency, lockstep versioning and unpublished-crate rows: Cargo manifests and the lockfile are byte-identical, versions and publishing unchanged.
@task_item n=9 part=untouched
Browser app rows (history, about pages, tabs, toolbar, persistence behaviour): the browser app's own behaviour did not change.
@task_item n=10 part=untouched
WPT runner contracts (result kinds, statuses, outputs, diff-to-PR script): the runner itself did not change, only its pinned suite commit.
