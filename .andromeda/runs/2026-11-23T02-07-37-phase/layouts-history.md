@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: skips Home, mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans (origin of decision/headless-stand-pinned-viewport). A canvas booted on the stand for the measurement rides this mount and viewport; the screenshot contract's 800 × 600 shot rests on it.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations to the `Default` impl holding 800x600, scale 1, light and the viewport built from them. The harness `Harness::screenshot` this chunk reads draws on those defaults; a harness edit for widget painting shifts these citations again.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author ids, adding no class, style, wrapper or order (markup-only gate). Precedent for any canvas element or page the stand gains: ids by attribute, layout untouched.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — every element an agent can act on reads an author key (the founder's rule); added `crud-person-{person.id}` and Home's `task-card-{slug}` ids. A new canvas element or stand page owes author ids under this rule, and a new served task would need a task-card id.

@history_item n=5 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed the non-lean tasks, including Circle Drawer's `circle-canvas` (contract/stand-element-ids/circle-canvas) and Cells' ids, as `id:` lines alone. The existing "canvas" id is a div-based Circle Drawer surface, not a painted canvas; moving it onto a real canvas is out of scope («Circle Drawer on a canvas»), so this id must not move here.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — named the session host a third consumer of the `task_in_shell` mount, booting through `seven_guis::stand` only, adding no markup; a held instance reads as a fresh boot and the mount and pinned viewport hold after commands (decision/held-instance-reads-as-fresh-boot, decision/session-roles-share-stand-boot). A canvas shot through `escher-session` must come through that same boot.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session` the driver's command line (`<verb> [arguments] --session <dir>`, endings 0 · 1 · 2 · 3) with the host role `serve` as the mount's consumer. Fixes the exit codes (contract/escher-session-exit-codes/0, /1) a canvas shot answers under; the scope forbids a new verb, argument or exit unless a fork's ruling adds one.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role, a stdio MCP server local to the user, no listener or auth surface, a further consumer of the TaskShell mount; its crossing is the founder's ratified one in the security plan (decision/mcp-server-local-stdio, contract/escher-session-mcp/stdio). App-painted canvas pixels leaving through MCP is one of the chunk's candidate founder forks.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `screenshot` verb: optional `--id` (element cropped to its visible part), answer `{"width","height","png"}`, `fonts-unavailable` and `off-screen` refusals, MCP image block then text block, result bounded at 1,048,576 bytes; the image in band is the founder's fork 1, ratified. This is the shot a canvas must appear in (contract/escher-session-screenshot/answer, contract/escher-session-mcp/screenshot-result, /result-bound, contract/escher-session-refusals/*); the contract stands per scope and a canvas-content class of pixels is a new crossing to put to the founder.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` on `start`, the host and `mcp` roles: owner-only `check-{n}.json` 0600 in a 0700 dir, `record-dir-not-private` and `recording-failed` session errors; a founder-ratified boundary widening. Rows contract/escher-session-record/*; a canvas shot kept under `--record` is one of the carriers the scope's fork lists.

@history_item n=11 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call its verb, arguments, acceptance, answer digest and masked answer, a shot kept as `{"width","height"}` without its `png` (founder's F3 B). Governs how a canvas shot's pixels are, and are not, kept in a check (decision/session-recording-replayable-check); its digest makes a shot's bytes part of a replayed reading.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint at boot in both layout modes, no stand markup moved; the gated part (same marker) made it the `lint` verb with `fonts-unavailable` refusal (contract/escher-session-lint/answer). Any canvas page or element the stand gains must keep decision/stand-lints-clean-at-boot or be ruled.

@history_item n=13 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the stand gained served screens beside the lean four (Cells by `boot_cells`, the load page in its own TaskShell by `boot_load`), the host and `mcp` start enum and usage `apps:` widened to six, the lint at boot re-pinned per page. The precedent for how a canvas page or task would be added to the stand, and what it drags along (served-task rows, start enum, cold-agent apps, lint at boot); whether the stand gains one is a founder fork per scope.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the flight booker's boot lint reading) `at_boot` moved to `[0, 0, 0, 0]` by skipping a closed select's options in `overlaps` (founder F9); also extended `stand_contrast` to the load page and Cells in both modes. Shows a new stand surface is read by the lint and contrast checks in both layout modes, and that a lint exclusion for an engine-drawn element was a founder fork, not a default.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the engine paints an open select list in `paint_scene` after the root's render and before devtools overlays; the founder confirmed the boundary widening card "labels leaving as pixels" and a new input class. The nearest precedent for new engine-painted content in a shot: the scope names it as the ratified widening a canvas's painted content is distinct from, so a canvas's pixels need their own ruling; also a precedent for paint-order placement of appended scenes.

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added the `coverage` role, booting the checks' app once per check through the same boot and replaying each on a fresh mount, adding no markup (ratified at 2026-11-15T05-21-42-wrap); bounded check read `MAX_CHECK_BYTES + 1`; also re-cited the skeleton notes in `dioxus_document.rs` (decision/fixed-document-skeleton-main-mount). A canvas shot replayed by coverage goes through this boot; edits to the Dioxus bridge shift those citations.

@history_item n=17 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role as a TaskShell-mount consumer and as a cli role (and the graph verb), their texts standing. The rows decision/session-roles-share-stand-boot and contract/escher-session-coverage/* this chunk rests on are founder-ratified, not provisional.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence), each verified against the code; the picked rows and the absence absence/arbitrary-index-html-templates come from it. Any change this chunk makes lands as row edits (`+`/`~`/`−` handles), not prose.

@history_item n=19 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grew to fifteen with `layout`, and decision/no-caller-input-echoed gained its one exception (the layout answer's id), operator-resolved then founder-ratified (2026-11-17-a-component-in-isolation). Precedent that a new verb or echo exception is a ruled fork; the scope bars a new verb or carrier without one.

@history_item n=20 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — (Surface: cli and Surface: desktop-native) the order page added as the seventh served task (contract/escher-session-served-tasks/order, listed after load), cold-agent apps six to seven, decision/stand-lints-clean-at-boot widened to the order page, six order-page ids added. The latest precedent for adding a stand page: the row set it moves (served task, lint-at-boot list, stand ids, cold-agent count) if the stand gains a canvas page or task.
