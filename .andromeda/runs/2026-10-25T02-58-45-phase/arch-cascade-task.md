# Task — arch · 2026-10-25-capability-statement

## The chunk
Writes one capability statement, a document and no code, saying what escher can do as of 0.2.0, read from the surfaces it ships: the driver's twelve verbs on their carriers, stable element ids, the snapshot with its text form, parts and diff, the headless screenshot, the visual lint, session recording and replay, the load wait, the select model, the range input, keyboard activation and the one virtual clock.
Collects in one place every limit the tool and the plans already state, and adds as a stated limit that an acting verb's diff and a lint's findings are not paged, held only by the answer bound.
States the a11y reach (what the WCAG target is proven on, and what stays unmeasured) and what an MCP client carries onward from escher's stdio, beyond escher's boundary, with the cold-agent pipe as the measured instance.
Corrects README.md's MCP-role verb count from eleven to twelve, and may name the statement once in README's 0.2.0 stage list; where Viola's planning reads the statement is an open fork (README, a dedicated tracked escher document, or a file in Viola's incubator in another repository).
Changes no code, engine, schema, verb, carrier, log field or span; a defect the statement finds is reported, never fixed here.

## Reached files
- README.md · edits the MCP role's line naming the driver commands as tools (its verb count), the 0.2.0 stage list

## Needs
- The front-page rule for the root README.md: it is escher's own page (what escher is, its status, how the driver's command line and MCP surface run, its lineage, licence and contact, the Escher spelling), no build, doc or test step reads it, and an upstream sync resolves a README conflict to escher's version.
- The driver session contract: twelve verbs in three levels, eight closed refusal causes, every call through `Session::run` after validation, the v3 wire, the command line with its four exit statuses, the MCP server and its tool listing, as the statement's source for what the driver can do.
- The driver session decision on what crosses: ids, accessible names, masked control values, snapshot text, diffs, shots and lint findings cross, never a `NodeId`; a new carrier or exit widens the boundary — the statement describes these crossings and must add none.
- The MCP surface as carried: the `escher-session mcp` role over its own stdio with no listener, the server name `escher`, a shot as an image block with a password painted masked — the content an MCP client receives and may forward.
- The one listener (the session's owner-only Unix-domain socket) and the outbound hosts, the model provider reached only by the spawned `claude` client on the operator-driven live cold-agent run, never by escher code — the measured onward-carriage instance.
- Session recording and replay: a kept check at rest, owner-only, a masked `type` as its stand-in and a shot without pixels, replay through `Session::run`, and the two committed Counter checks as the one exception to owner-only.
- The Dioxus DOM bridge: stable ids in four tiers, the snapshot of id, role, name, state and bounds, its text form under its budget and its parts, the diff, `unkeyed_actionable`, and `is_masked_input` as the one masked-control test.
- The test harness contract: screenshot through blitz-paint over opaque white, `text_contrast`, `settle` that never waits on a load or moves time, and the one shared `VirtualClock`.
- The document core contract: the document's clock through `DocumentConfig.clock`, the read-only `select_value`, `can_paint_text`, and `text_extent` as a read-only reader (whose RTL and vertical text read as LTR lines is a stated limit).
- The scrolling contract's stated limit: `visible_region` reads a scrolling box under a transform where it stands untransformed, so an element inside it can read off-screen; `scroll_into_view` through every scrolling box.
- The hit-test readings measured off and stated: a box clipped on one axis stops a hit at its whole padding box, and a box with no width or height is hit at its centre on its holder, each able to read covered though it shows.
- The select model's stated limits: no popup painted, an option not clicked, a `multiple` select and an `optgroup`'s options not modelled; its key and typeahead behaviour as a capability.
- The DOM semantics limit that HTML's focus fixup is not built (a node made inert keeps focus until focus moves), owned by a residuals line, and `inert` honoured by presence.
- The accessibility-tree contract's two measured limits: no native checkbox's or radio's checked state in the tree, and the platform-tree refresh's windowed witness still owed — part of the a11y reach the statement reports.
- The events contract for the capabilities stated: a range input's default actions, Enter and a released Space dispatching a Click, `change` after `input` on a select's choice.
- The headless stand contract: the four lean tasks booted offline at a pinned 800x600 Light viewport with one bundled font — what every capability is proven on.
- The shot's PNG bytes differing between linux x86_64 and macOS aarch64, so a check compared across OSes holds no screenshot step — a stated limit.
- The painting decision that a password input paints masked in every document, which the statement relies on for what a shot carries.
- `LOAD_WAIT` (5 s of wall time) bounding an acting step's wait for a load and moving no time, and `Session::lint` reading the screen for its six defects and changing nothing.
- The CI contracts that name the cold-agent pipe and the a11y leg's files, read for the a11y reach and the measured onward-carriage instance.

## Not needed
- Engine internals the statement neither describes nor changes: layout and the paint tree, CSS approximations and unsupported features, Stylo threading, resource loading, fonts, HTML parsing, networking and blitz-net, inline SVG, selection and animation.
- The JavaScript runtime and blitz-vibey-script contract: escher's driver uses none of it and the statement makes no claim about scripts.
- The browser app, rdme, the WPT runner and its outputs, the upstream-only publish and WPT workflows, Nix packaging and cross-compilation.
- Dependency pinning, build profiles, MSRV, default features and release versioning: nothing in the manifests or the workspace version moves; the 0.2.0 the statement names is escher's route version.
- Code conventions (formatting and lints, manifests, documentation comments, visibility and naming, feature gating, unsafe and borrowing, test layout, apps and examples, HTML fixtures): the chunk writes no code, test or fixture.
- The telemetry bootstrap, the logging pattern and the command and tool_call span fields: no log field or span changes, and the statement does not restate the sink.
- Environment variables, process-wide state and threads, system services, keyboard shortcuts and URL schemes: none is added or described.
- The filesystem registry's state-directory and recording-directory rules as rules to follow: the chunk creates no state or recording directory and runs no session.
- Infrastructure patterns for runtime threading, the shell loop, launch paths and headless loading.
- The CI pipeline's job layout, caches and SHA pins: no workflow or leg changes.
- The cold-agent bar (v020-11): the next entry owns it; this chunk only cites the pipe as the measured instance.
