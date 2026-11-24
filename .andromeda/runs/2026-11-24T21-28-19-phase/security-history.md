@history_item n=1 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — Threat Model Summary: 4 row(s). Retired `decision/canvas-content-crosses-as-shot-pixels` (− → `decision/canvas-content-crossing`) and added `+ decision/canvas-content-crossing` and `+ decision/cold-agent-canvas-to-provider`. Also changed `~ contract/actions-answer/verbs`, so canvas is listed with no cause on an app that can paint text. These are the founder's «Подтвердить все 15». Why it matters here: this is the crossing a canvas lint finding would ride. The canvas answer is limited to closed words, numbers and 64-bit digests and never carries a glyph id, position, text, font bytes or family name. Adding finding words inside it, or inside `lint`, must stay in that closed class. The cold-agent pipe already carries canvas answers to the model provider.

@history_item n=2 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — Input Validation: 4 row(s). Added `+ contract/driver-verbs/canvas` and `+ contract/cli-flags/full`, both PROVISIONAL (delegate escher overseer, inputs#I3). Added `+ name/command/canvas`. Changed `~ contract/recorded-check/tool`: twelve instance verbs. Why it matters here: the canvas verb contract is still provisional. It takes an id, `full` and `part` 1–1000, and the verb adds no cause, argument kind, session error or exit. A canvas lint that puts a field on the canvas answer, or new defect words in `lint`, builds on a provisional row and must not add a cause or exit (scope fct-itdvagbc).

@history_item n=3 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — Input Validation: 1 row(s). Changed `~ decision/socket-carries-screen-reading` so an answer carries "layout and canvas readings". Why it matters here: this is the carried-set row that a canvas finding would extend, whether it goes in `lint` findings or as a canvas field.

@history_item n=4 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — Logging & Monitoring: 2 row(s). Changed `~ decision/no-screen-content-in-logs` to add a canvas answer, and `~ contract/command-span/verb` so a canvas call records the verb alone, plus cause when refused. Why it matters here: a canvas lint must add no span field or log site. Its findings and their count stay out of every log, which the host_log/mcp_log needle checks read.

@history_item n=5 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — Data Protection: 1 row(s). Added `+ decision/kept-check-canvas-answer`: a kept check holds a canvas answer whole, and replay stops naming `digest`. Why it matters here: if the `lint` or `canvas` answer gains canvas findings, kept checks hold them at rest and replay compares them. Scope fct-nhcldnmn names the kept-check recording if the `lint` answer changes.

@history_item n=6 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — API Security: 1 row(s). Changed `~ decision/time-moves-only-on-advance` to name canvas among the verbs that wait on no load. Why it matters here: a canvas lint read must move no clock and wait on no load, like `lint` and `canvas`.

@history_item n=7 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the in-process lint as an id reader, and the text extent reader (amd-iqjzkagx). This entry made `Session::lint` an in-process reader: its findings hold ids, closed defect words and numbers only, never page text, colour or a control value. It also added the read-only Text extent reader (`BaseDocument::text_extent`), which answers None for an unresolved id or a node with no inline layout. Why it matters here: it is the origin of `decision/lint-findings-closed-content` and `decision/text-extent-read-only-none-when-unresolved`. It is also the in-process-first precedent the scope's "in process only" option would follow.

@history_item n=8 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the gated part: the `lint` verb's findings on every carrier (amd-dvtrjrxz). Made `lint` a verb with no argument, one field `findings` and `fonts-unavailable` refusing before any read. `Outcome::Linted` holds ids, closed defect words and numbers, written by `to_json` alone. Findings and their count are never fielded in a span. Why it matters here: this is the ratified `lint` carrier, founder F1 «Новая команда lint» and F2 «Без поля». New closed defect words inside it are a field of a closed set inside a ratified carrier, so no widening. A new field or open-valued content would be one.

@history_item n=9 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — Dropped the visual lint's two PROVISIONAL marks on the founder's word (lint F1 «Новая команда lint», its verb owed to the gated part). Why it matters here: the lint's standing ratification rests here and on n=8, together with `decision/lint-findings-crossing` and `decision/lint-findings-closed-content`.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — three state readings inside the ratified carriers; the closed-field rule (amd-trieeiqe). Stated the founder's rule: a new field of a closed value set inside an already ratified carrier is no widening. A field holding authored text, an open value set, a new carrier or a new exit still is. Why it matters here: this is the rule the scope cites for the crossing fork (fct-oyidili5) and the source of `decision/boundary-widening-rule` / `decision/closed-value-field-no-widening`. It decides whether canvas defect words in `lint`, or a canvas-answer field, need the founder.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` and the program's fixed text through the validated surfaces (amd-t3m3x2fx). The program's own fixed `'static` table text, meaning verb help and result field help, may cross on the ratified carriers. Text from a page, a call or a screen still widens. Why it matters here: `describe.rs`/`help` changes for new defect words or a canvas field (scope fct-nhcldnmn) are inside this rule. This is the source of `decision/mcp-listing-fixed-text`.

@history_item n=12 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the carried sets name a lint's findings and the diff. Reconciled every carried-set site so a lint's findings appear as stable ids, closed defect words and numbers. Added the committed statement `docs/capabilities.md`, which states the MCP crossing and adds no carrier, plus a secret search over it. Why it matters here: if a carrier's content changes, `docs/capabilities.md` and the carried sets must follow (scope fct-nhcldnmn). This is the source of `decision/capability-statement-mcp-crossing`.

@history_item n=13 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — Extended the capability statement's secret search (`/home/|api[_-]?key|authorization:|bearer `) to README.md, docs/capabilities.md and the pin, with 0 hits. Why it matters here: an edit to `docs/capabilities.md` for a canvas finding falls under this standing search.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the select readers and the list's public surface (amd-txxcahkq) and an open list's visible region (amd-kp7dm775). `lint.rs` `clip_region` takes an open row's region from `visible_region` alone, which is the founder's F10 «Строки без overlap». Why it matters here: this is the precedent for how the lint decides a box's region and clip. A canvas label's region (content box plus pushed clip) is a new region class beside it.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — what else of the list crosses the carriers (amd-c2vdxtbp). The highlighted row crosses no structured carrier, and the lint does not read its contrast, as a closed set inside the ratified crossing. Why it matters here: this is precedent for scoping which painted content the lint's contrast check reads. Text contrast against the fill painted under a canvas glyph run is a candidate (fct-siov2su3).

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — `visible_region` through transforms (amd-uqyxh2wm). `visible_region` reads each clipping box through its transforms and answers None on a non-finite edge. Its callers include `lint.rs` `clip_region`. This is an engine change for every document, ratified. Why it matters here: it is the source of `decision/visible-region-through-transforms`. A canvas region check that composes with the canvas element's visible region inherits its rotation bounding-box limit.

@history_item n=17 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness takes `color` (workspace 0.3) as the luminance source of its contrast reader. It was reviewed, and no package entered the lock. Why it matters here: a canvas text-contrast candidate would reuse this reader. Any new dependency must be recorded as a reviewed Pinning bullet in the same way.

@history_item n=18 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — Input Validation: 2 row(s). Added `+ decision/custom-widget-content-box-frame` and `+ decision/custom-widget-document-font-context`. Why it matters here: the widget paints at its content origin inside its content box, unclipped, which is the region the scope infers for a label (fct-vugcafji). Reading glyph advances for a run's true extent (fct-ondmtmdz) would go through this shared document font context. The advances must stay in process and never cross as glyph data.

@history_item n=19 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — Threat Model Summary and Dependency Security. Added `+ decision/canvas-content-crosses-as-shot-pixels`, later replaced by n=1, and `+ decision/harness-blitz-paint-custom-widget`: the harness turns on blitz-paint's custom-widget feature in every build selection. Why it matters here: the canvas lint reads the commands that this paint pass records, so it lives in builds where the feature is on.

@history_item n=20 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — Input Validation: 1 row(s). Added `+ decision/custom-widget-click-synthesis`: a click is synthesized on a custom-widget target. Why it matters here: Circle Drawer's canvas is the stand this chunk must read clean, and this is the widget input path that stand rests on.

@history_item n=21 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the screen's pixels in band, and the engine's password mask, nine causes, no image in a log, `png`. Added `fonts-unavailable`, which refuses before anything is painted (later also a `lint`), and in-band PNG shots. Why it matters here: this is the source of `contract/refusal-causes/fonts-unavailable`, `decision/screenshot-in-band` and the png rows. A canvas lint must refuse within the existing causes, `fonts-unavailable` among them, and add none.

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — Narrowed the cause set from nine to eight, retiring `time-unavailable`. Why it matters here: the closed cause set of eight (`decision/closed-refusal-cause-set`) is what a canvas lint must stay inside. The scope forbids a new cause without a fork.

@history_item n=23 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — A driver call is validated against a closed table before anything runs, and a refusal holds nothing of the call. This includes no unwrap/expect/panic on call input. Why it matters here: it is the origin of `decision/single-validation-gate`, `decision/no-panic-on-call-input` and `decision/refusal-holds-no-call-input`, which any argument a canvas lint adds must keep.

@history_item n=24 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — what the driver logs: one span with fixed-word or count fields only, never an argument, Outcome or screen text. Why it matters here: it is the origin of `decision/one-command-span-per-call` / `decision/command-span-content-free`. A canvas lint adds no span field, and a finding count is never fielded (n=8).

@history_item n=25 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer whole at rest, the committed checks, the replay. A kept check holds each answer as the driver wrote it, masked, a shot without pixels. Two committed stand-Counter checks are the one exception to owner-only. Why it matters here: it is the source of `decision/kept-check-answer-whole` / `decision/kept-check-owner-only`. If the `lint` answer changes, kept checks and the committed checks' byte-for-byte replay must still hold.

@history_item n=26 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the pipe's second live run carried a lint's findings, among other content, to the model provider. The live child's kept checks are read for bools and counts only. Why it matters here: canvas findings in `lint` would reach the provider through the cold-agent pipe under these rows and `decision/cold-agent-checks-read-for-counts-only`.

@history_item n=27 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — Threat Model Summary and Logging & Monitoring. Changed `~ decision/cold-agent-screen-to-provider` to add caused text, and `~ decision/cold-agent-events-metadata-only`, which counts closed words per kind and reads only the kind. Why it matters here: these are the current forms of the picked cold-agent rows. Any lint or canvas count the pipe's verdict reads must be a count of closed words only.

@history_item n=28 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — Input Validation: 1 row(s). Changed `~ contract/driver-session-reply/no-attribute-text`: a reply never carries attribute text, except author text in a record's message or a caused item's `error`. Why it matters here: a canvas finding must carry no glyph run's text and no attribute text, and falls outside that exception.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the reserved virtual-node id form (amd-bkpsbxrm). The id grammar reserves `{owner}///{key}`, and no served id holds `///`. Why it matters here: it is the source of `contract/stable-element-id/virtual-node-form`. A canvas finding names the canvas element by its stable id and mints no command identity, since virtual nodes are a later entry (scope fct-mcp3vbox).

@history_item n=30 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — Input Validation, Threat Model Summary, Logging & Monitoring and Data Protection. Moved an in-process read (`decision/session-layout-read-in-process`) onto every carrier as a verb, with a kept-check row, carried-set rows and a span row changed together. Why it matters here: this is the precedent for the "in process only now, carrier later" option the canvas lint crossing fork weighs, and for the set of rows that move together when a reading crosses.

@history_item n=31 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — Logging & Monitoring and API Security. Changed `~ contract/command-span/verb` so the part asked and the part count are never fielded, and introduced the bounded `part` 1–1000 for graph. Why it matters here: the canvas verb's `part` follows this form. A canvas lint over a paged canvas answer must not log a part or its count.

@history_item n=32 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — Input Validation: 1 row(s). Added `+ decision/fixture-name-closed-table`: a fixture state name is compared whole against the app's closed table before boot. Why it matters here: if the planted overflowing-label fixture is reached as a named state, it must be added to that closed table.
