@history_item n=1 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the stand's nested scroller: added the IA rule that an into-view scroll moves every scrolling box that holds its target, then the viewport, and that `visible_region` reads what an element is seen through; `crud-list` (24, 112.796875, 369 × 419.203125 at boot) is the stand's nested scroller, the last row after twelve Creates refused `off-screen` until a driver `scroll`. The engine rule is founder-ratified (2026-10-07) for every document. It is the source of rows decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes, which this chunk must keep while it takes upstream's reversed scroll origins and changes `scroll_into_view` to read `scroll_state` as `(current, min, max)`.

@history_item n=2 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — set what a scrolled `crud-list` reads: the same four bounds figures once its content is scrolled, not named in that scroll's diff, and a driver `click` naming the list landing on the row at its centre at 12 and 14 Creates. Home's column stands in a box that scrolls, and Cells (615 to 684) shows and is hit only once scrolled into view. Why: the engine's bounds reader and hit were fixed, and a check that clicked Cells without scrolling went red. This is the escher reading (own bounds with no own-scroll subtraction, the hit's stop) that the chunk's conflict resolutions in `writing_mode.rs` and `node.rs` `hit_inner` must preserve against upstream's scroll-shifted readings.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — added the exception: a row or group of an open select list is seen through the list's scrollport alone, and `scroll_into_view` on a row writes the list's `scroll_y` at once, with no scrolling box or viewport moving. This is the `except:` on both the scroll-into-view and visible-region rows. The chunk's seam edit to `scrolling.rs` (`scroll_into_view` skipping a box whose `min` and `max` are both zero) and the new `is_scroll_container` guard for nodes with no computed styles, found in the `select_popup` panic, must keep this early-return path whole.

@history_item n=4 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list, the engine's select list in every document) — while a select list is open each row reads its own `bounds` in the snapshot and the diff, and after a close the rows read back at the select's client rect. The list scrolls by key and wheel. This is the source of row decision/open-list-rows-own-bounds (guard `stand_act_popup`). Upstream's `ScrollableOverflow` and negative scroll range change list scrolling and bounds reads, so these readings must hold unchanged.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_select_box`) — `overlaps` skips entries with `Entry.in_select_box`; `clip_region` takes an open row's region from `visible_region` alone; `at_boot` stays `[0, 0, 0, 0]` in both layout modes, and `crud-person-14` still reads `clipped` after twelve Creates. These lint readings rest on the clip and visible-region math that this chunk's take touches (row decision/stand-lints-clean-at-boot).

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (citations re-pointed) — six citations re-pointed, among them `scrolling.rs` 1 (its range grew), because the chunk changed the cited files. Precedent: this chunk edits `scrolling.rs`, so the wrap owes the same re-point of the scrolling boxes note's citation.

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading, a closed select's options skipped) — the boot lint moved from `[0, 1, 0, 0]` to `[0, 0, 0, 0]` in both layout modes (founder's F9). The snapshot still gives each closed-select option its select's client rect. This is the basis of rows decision/stand-lints-clean-at-boot and decision/flight-type-select-boot-reading, which the take's measurement reads as unmoved.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed) — thirteen citations re-pointed, among them `scrolling.rs` 1 (range shrank) and `stand_lint.rs`. The scrolling boxes note's `visible_region` statement was kept in its words. Precedent for the citation re-point this chunk's `scrolling.rs` edits will owe.

@history_item n=9 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the flight booker's mode became a select (`flight-type`, `flight-type-one-way`, `flight-type-return`), with the snapshot giving each option its select's client rect at boot; six served tasks (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`) are booted by the host and `mcp` roles; the load page and the lint at boot were added. This is the source of rows decision/flight-type-select-boot-reading and contract/escher-session-served-tasks/counter, which the take must leave unchanged.

@history_item n=10 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand skips Home and mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. This is the source of row decision/headless-stand-pinned-viewport. Every stand reading (the cross-OS digests in `stand_cross_os_readings`) is measured on this mount.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became a third consumer of the TaskShell mount: a held instance reads as a fresh boot, and the mount and pinned viewport hold after commands. This is the source of row decision/held-instance-reads-as-fresh-boot (guard `stand_session_fresh`).

@history_item n=12 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, cli and TaskShell consumer entries) — `escher-session coverage <app> <check>...` replays each kept check on a fresh boot and answers `{"unexercised","checks","held"}`; a failed boot is the session error, exit 3. It adds no markup, and the mount and pinned viewport hold in both layout modes. This is the source of row contract/escher-session-coverage/answer. A kept check's replay must read the same if the engine's scroll and hit readings stay as ratified (ratified by the founder at 2026-11-15T05-21-42-wrap).

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` with `--id` crops the element to its visible part, and a shot of no pixel is refused `off-screen`, exit 1. This is the source of rows contract/escher-session-screenshot/answer and contract/escher-session-refusals/off-screen. Both read visible bounds through scrolling and clipping boxes, which the reversed-scroll-origin take touches.

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` keeps a session as one owner-only replayable check (founder-ratified boundary widening). This is the source of row decision/session-recording-replayable-check. The chunk's boundary is that no carrier, field or ending is added.

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check keeps, for each call, its answer's digest and its masked answer, which `Check::replay` reads back and replays. Any engine reading that moves a bounds or hit answer would break a kept check's replay. This is why the chunk's measurement must read the driver's answers unchanged on the stand.

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — the stand reads clean to the visual lint in process and through the `lint` verb, which answers `{"findings":[…]}`. After twelve Creates `crud-person-14` reads `clipped` inside `crud-list`, agreeing with the `off-screen` refusal. This is the source of rows contract/escher-session-lint/answer and decision/stand-lints-clean-at-boot. Its clip readings depend on the scrolled-box geometry this take touches.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — `actions` answers an unpaged `{"nodes"}` of id, owner, keyed and verbs with cause, never a value, typed text or `NodeId`. This is the source of row contract/escher-session-actions/answer. Its causes (for example off-screen) read the engine's visibility, so they must not move under the take.

@history_item n=18 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answers `{"text"}` or `{"text","part","parts"}`, with a part past the last giving empty text. This is the source of row contract/escher-session-snapshot/answer, whose bounds text this chunk's scroll readings feed.

@history_item n=19 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the stand states roles and states by attributes alone (`aria-invalid`, `role="listbox"`/`option`, `aria-selected`), adding no element, class, style, key, id or order. This is a source of row decision/stand-a11y-by-attributes-alone. The take moves no stand markup.

@history_item n=20 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — after an upstream merge, the `file:line` citations into merged upstream files were re-pointed by the merge's measured line map, with no claim text changed. Precedent for this upstream take's re-point of citations into `scrolling.rs` and the other blitz-dom files it touches.

@history_item n=21 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into the files the take moved) — an upstream take re-pointed citations for IA notes on the scrolling element, pointer coordinates and the root element's client size, rewriting paths where code moved files. Precedent and trap: an upstream take can move cited code between files, not only shift lines.

@history_item n=22 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (121), each verified against the code. The picked rows' handles (decisions, contracts and the absence `arbitrary-index-html-templates`) date from this conversion, so later amendments name them by `+`, `~` or `−` handle.
