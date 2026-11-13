@task_prose part=session
The session took upstream blitz main up to 563d3db73 as 29 no-commit cherry-picks, every pick clean, no dependency, lockfile or manifest moved; 99 upstream commits were left out, each with a measured reason, including the commit that would bring a CDP server listener.
The take changed engine behaviour, ratified by the founder as upstream's: href resolution answers none instead of panicking, pages decode lossily, stylesheets decode from bytes with BOM and charset, font-face sources are skipped by format string and URL extension, the font element's face and size, valign, cellspacing and cellpadding become presentational hints, an image with an empty alt reads as decorative, a default font backs an unavailable family list, and checkbox and radio state is kept even with no box.
The audit ignore set gained one advisory, RUSTSEC-2026-0459 (libsqlite3-sys 0.30.1 through rusqlite 0.32 under the browser's persistence), with its written reason, owned by a residual until the browser moves to rusqlite 0.41 or later.
Every live uses: line of every workflow is now pinned to a 40-hex commit SHA with its ref as a trailing comment, and the pin check widened from the CI workflow to every workflow, its tests moved into a new WorkflowPinTest; repository guards on the upstream-only workflows were kept byte for byte.
The two committed Counter checks were re-recorded (only bounds and digests moved), two frame tests of blend modes and inline-box decorations were added, and CI went green at attempt 3, the audit leg reading advisories ok, licenses ok.
@task_file n=1 path=apps/readme/src/main.rs mark=merge
what: the rdme readme app, which passes raw HTML through to the engine
@task_file n=2 path=apps/readme/src/readme_application.rs mark=merge
what: the rdme readme app's application shell that loads and navigates documents
@task_file n=3 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's default user-agent stylesheet for form controls such as textarea and select
@task_file n=4 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the accessibility tree builder, the content classes it hands the accessibility adapter and the roles it reads
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document, its URL resolution of hrefs, its document flags and transform resolution
@task_file n=6 path=packages/blitz-dom/src/font_metrics.rs mark=merge
what: the engine's font metrics query over the listed font families
@task_file n=7 path=packages/blitz-dom/src/form.rs mark=merge
what: form submission and form control state, by scheme and method
@task_file n=8 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the DocumentMutator, the one write path for attribute changes, checked state, link rel stylesheet loads and their removal
@task_file n=9 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's stylesheet and font-face loaders, the decoding of fetched stylesheet bytes and the choice of a font-face source
@task_file n=10 path=packages/blitz-dom/src/resolve.rs mark=merge
what: style and layout resolution, including transform resolution over anonymous boxes
@task_file n=11 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the markup attribute readers that turn presentational attributes into style hints
@task_file n=12 path=packages/blitz-dom/src/stylo_device.rs mark=merge
what: the style device, the viewport scale handed to style resolution
@task_file n=13 path=packages/blitz-dom/src/stylo_to_parley.rs mark=merge
what: the font family list handed to text layout and the default-font fallback
@task_file n=14 path=packages/blitz-dom/src/url.rs mark=merge
what: the engine's URL resolution, including file URLs and Windows absolute paths
@task_file n=15 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: layout damage tracking on a style or scale change
@task_file n=16 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: inline layout tree building, the anonymous blocks around inline boxes
@task_file n=17 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: inline layout of inline boxes and their heights
@task_file n=18 path=packages/blitz-dom/src/node/element.rs mark=merge
what: element data, including checkbox and radio checked state
@task_file n=19 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the node hit test and the cursor read for links
@task_file n=20 path=packages/blitz-dom/src/node/text.rs mark=merge
what: text node data and its decoding
@task_file n=21 path=packages/blitz-paint/src/render.rs mark=merge
what: the painter, blend modes, isolation and inline-box decorations
@task_file n=22 path=packages/blitz/src/lib.rs mark=merge
what: the blitz entry crate that loads a page from bytes or a URL
@task_file n=23 path=tests/blitz-tests/tests/accessibility_roles.rs mark=merge
what: the accessibility role integration tests
@task_file n=24 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests target list, each integration test a mod line
@task_file n=25 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit the WPT runner is pinned to
@task_file n=26 path=.github/workflows/publish-browser.yml mark=merge
what: the browser publish job, repository-guarded to upstream, its signing material and secrets
@task_file n=27 path=.github/workflows/wpt.yml mark=merge
what: the WPT workflow, upstream-only and repository-guarded, its pinned actions and rust-toolchain step
@task_file n=28 path=tests/blitz-tests/tests/anonymous_block_transform.rs mark=merge
what: an integration test of a transform inside an anonymous block
@task_file n=29 path=tests/blitz-tests/tests/font_element_attributes.rs mark=merge
what: an integration test of the font element's face and size attributes as style hints
@task_file n=30 path=tests/blitz-tests/tests/font_family_fallback.rs mark=merge
what: an integration test of the default font when no listed family is available
@task_file n=31 path=tests/blitz-tests/tests/hidden_checkbox_input.rs mark=merge
what: an integration test of a label click toggling a hidden checkbox
@task_file n=32 path=tests/blitz-tests/tests/hit_inline_embedded_box.rs mark=merge
what: an integration test of the hit test on an inline embedded box
@task_file n=33 path=tests/blitz-tests/tests/link_cursor.rs mark=merge
what: an integration test of the pointer cursor over a link
@task_file n=34 path=deny.toml
what: the cargo-deny audit and licence configuration, its advisory ignore list each by ID with a written reason
@task_file n=35 path=.github/workflows/wpt-post-results.yml
what: the WPT post-results workflow that runs trusted scripts, its pinned actions
@task_file n=36 path=.github/scripts/test_ci_workflows.py
what: the ci-scripts suite guarding the CI workflows, now with WorkflowPinTest reading every workflow's pins
@task_file n=37 path=tests/blitz-tests/checks/counter-cli.json
what: a committed stand-Counter check recorded through the CLI
@task_file n=38 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed stand-Counter check recorded through the MCP server
@task_file n=39 path=tests/blitz-tests/tests/blend_mode_frame.rs
what: a frame test of blend modes and isolation as painted
@task_file n=40 path=tests/blitz-tests/tests/inline_box_decorations_frame.rs
what: a frame test that an inline box of blocks paints no decorations of its own
@task_item n=1 part=touches
The rule that every action a CI workflow uses is pinned to a 40-hex commit SHA with its ref kept as a trailing comment, recorded unverified at the landing, now holds: all four workflows are pinned, 18 lines newly pinned in the upstream-only workflows.
@task_item n=2 part=touches
A guard moved: the pin assertions test_j and test_k left CiWorkflowTest for the new WorkflowPinTest, which reads every workflow; any row whose guard names CiWorkflowTest for the pin check is stale.
@task_item n=3 part=touches
The WPT workflow's rust-toolchain step moved from a stable tag to the CI workflow's pinned SHA with toolchain stable named; rows that the toolchain step names its toolchain still hold.
@task_item n=4 part=touches
The dependency audit with cargo-deny and its rule of advisory ignores by ID with a written reason: the ignore set gained RUSTSEC-2026-0459 with its reason; the audit leg reads advisories ok.
@task_item n=5 part=touches
Browser history stored in sqlite and the SQLite rows: the libsqlite3-sys under rusqlite 0.32 now carries an ignored advisory; the pin is unmoved.
@task_item n=6 part=touches
Upstream sync takes the dependency set whole, engine dependency pins and locked builds: this take moved no dependency, lockfile or manifest, and still holds them.
@task_item n=7 part=touches
The boundary widening rule: the take's behaviour widenings were ratified as upstream's by the founder's own word, not by a security-plan amendment.
@task_item n=8 part=touches
Unresolvable stylesheet href starts no load: href resolution now answers none for an unresolvable href and its callers skip it, where it panicked; a link's href removed with no stylesheet held returns early where it was unreachable.
@task_item n=9 part=touches
Failed stylesheet blocks no render and the non-UTF-8 handling: stylesheets now decode from bytes with BOM and charset, and pages decode lossily with no panic on non-UTF-8.
@task_item n=10 part=touches
Hint-less font-face source fetched and sniffed, and no agent URL through font sources: font-face sources are now skipped by a closed match on the format string and on the URL extension, so an unsupported source is never fetched.
@task_item n=11 part=touches
Markup attribute readers without log or panic, and attribute text leaving only by ratified carriers: new presentational-hint readers for the font element's face and size, valign, cellspacing and cellpadding; a table's cellpadding restyles its descendants.
@task_item n=12 part=touches
Link rel matching: rel now matches stylesheet case-insensitively, so more link elements start a stylesheet load under the remote content and import nesting limits.
@task_item n=13 part=touches
Content classes on the accessibility adapter crossing and accessible-name exits: an img with an empty alt now reads as a generic container, a decorative image.
@task_item n=14 part=touches
Toggle state read per role, control states as closed values, and the engine change event on checkbox and radio: a checked write is read back into an existing checkbox state, and a checkbox or radio's state is made even with no box.
@task_item n=15 part=touches
Non-invertible transform takes no hit and the visible region read through transforms: transform resolution never skips an anonymous box, a scale change resolves every transform, and the inline root's content-box offset now applies only to inline children and the self hit.
@task_item n=16 part=touches
The hit test's returned coordinates: escher's clip-to-padding-box early return now answers border-box coordinates for an inline root where the final self hit answers content-box ones; no check reads them and the stop reads its own offsets.
@task_item n=17 part=touches
A dropped anonymous block clears its children's parent, and the cursor walk: the cursor walk for a link follows the DOM parent, not the layout parent.
@task_item n=18 part=touches
Committed stand-Counter checks: both were re-recorded by their script, only bounds and digests moved, the first back-btn 73x25 to 72.578125x25.
@task_item n=19 part=touches
The log sink that redacts and allowlists targets: two tracing warn lines, feature-gated under the tracing feature and redacted under a blitz target, replace three removed; no new sink.
@task_item n=20 part=touches
File URLs load only for file documents: URL resolution now also turns Windows absolute paths into file URL paths.
@task_item n=21 part=touches
Publish job repository guard, publish secrets reachable only upstream and release publishing only from upstream: the publish job now takes the latest Android NDK and its actions are pinned; every repository guard is byte for byte.
@task_item n=22 part=touches
WPT post-results workflow runs trusted scripts, and WPT pinned to a recorded commit: that workflow's actions are pinned, the WPT workflow installs the wpt CLI by cargo-binstall, and the recorded WPT commit moved.
@task_item n=23 part=touches
Default script fetcher and the text-free font refusal: a default font now backs a family list with no available family, so a fonts-unavailable read may meet fewer cases.
@task_item n=24 part=touches
The repeated stand job count 1248 is unmoved, and the ci-scripts suite rose from 110 to 112 tests; no security-plan row states the latter.
@task_fact n=1 slug=dependency-security
fact: RUSTSEC-2026-0459, libsqlite3-sys 0.30.1 through rusqlite 0.32 under the browser's persistence, is ignored in the audit with its written reason, owned as a residual until the browser moves to rusqlite 0.41 or later.
@task_fact n=2 slug=dependency-security
fact: The pin check reads every workflow file, not the CI workflow alone, and asserts each live uses: line is a 40-hex SHA with its ref kept as a trailing comment, guarded by WorkflowPinTest.
@task_fact n=3 slug=input-validation
fact: Resolving an href answers none for an unresolvable URL and every caller skips it; no href resolution panics.
@task_fact n=4 slug=input-validation
fact: A page whose bytes are not UTF-8 is decoded lossily and never panics; a stylesheet is decoded from its bytes by BOM and charset rule.
@task_fact n=5 slug=input-validation
fact: A font-face source is chosen by a closed match on its format string, then on its URL extension ignoring query and fragment; an unsupported source is skipped and never fetched.
@task_fact n=6 slug=input-validation
fact: The font element's face and size, valign, cellspacing and cellpadding are read as presentational hints by a typed parse, and a value that does not parse is ignored.
@task_fact n=7 slug=input-validation
fact: An img with an empty alt is read as a generic container, decorative, on the accessibility adapter crossing.
@task_item n=1 part=untouched
The driver session socket, its closed grammar, bounds, replies, exit codes and the escher-session CLI flags and roles: no verb, reply, flag or bound moved.
@task_item n=2 part=untouched
The MCP server over stdio, its protocol errors, request and result bounds and tool results: no new port, socket, env var, IPC method or endpoint.
@task_item n=3 part=untouched
A served network API stays absent: the commit that would bring a CDP server listener was left out of this take.
@task_item n=4 part=untouched
The stable element id, snapshot text, masked control values and the actions and graph answers: none of their contracts moved.
@task_item n=5 part=untouched
The select list decisions, keys, rows, colours and its crossing: no select list behaviour changed.
@task_item n=6 part=untouched
Recording, kept checks, cold-agent runs and the coverage role and floors: no recorder, coverage or cold-agent change.
@task_item n=7 part=untouched
Outbound HTTP through blitz-net, per-host limits, the request timeout, response size, CORS and same-origin absences: no network fetch rule moved.
@task_item n=8 part=untouched
Secret management, CI tokens and CI secrets: no secret, token permission or env var changed.
@task_item n=9 part=untouched
Authentication and authorization absences, cookie store and HTTP cache features, TLS and clipboard choices: untouched.
@task_item n=10 part=untouched
Refusal causes, session error tables and their JSON shapes, and the command and tool_call span lines: no cause, error or span field moved.
