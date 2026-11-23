@history_item n=1 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the screen's pixels in band on the three ratified carriers: added the `screenshot` verb (optional `id`, `width` · `height` · `png` base64) on socket, command line and MCP (image block + text block; a result holding an image bounded by its whole written form), `fonts-unavailable` before anything is painted, a refused shot changing nothing, the image never fielded. Ratified as a boundary widening because pixels are no closed value set (founder's fork 1). Matters here: a canvas's app-painted content leaving through the same shot is a new content class in those pixels — the precedent says it is the founder's fork, not a default (decision/screen-pixels-in-band, decision/screenshot-in-band, contract/screenshot-answer/png, contract/mcp-tool-result/screenshot).

@history_item n=2 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the engine's password mask, nine causes, no image in a log, `png`) — blitz-paint paints a password as bullets in every document and renderer (an engine change for every document, founder's fork 2); a shot's image reaches no log (first 64 base64 chars 0 times at `trace`, both builds); `png` taken by escher-driver, blitz-tests dev-only; blitz-test-harness takes `blitz-paint` with default features off, `anyrender`, `anyrender_vello_cpu`; no `base64` crate. Matters here: the harness's feature-off `blitz-paint` is exactly why `custom-widget` is off in per-package builds — turning it on is a dependency/feature change for every build selection and must keep the mask (decision/password-painted-masked) and decision/no-base64-crate, decision/png-shot-encoder.

@history_item n=3 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — recorded that a click-placed password caret leaked relative glyph widths into a shot as a known limit. Matters here: a precedent that what a shot paints can carry information beyond the snapshot's masked fields; a canvas shot over a page with a password must keep the masked frame (fixed in entry 2026-10-18-engine-debts below).

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§Input Validation: `disabled` on form controls, the password caret by the bullets) — the caret is laid out over U+2022 bullets so no real glyph width reaches a shot; nothing new crosses. Matters here: the screenshot contract this chunk must keep ("a password input painted masked") rests on this.

@history_item n=5 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the overlay scrollbar fade reads the document's clock, so two shots of one state read the same pixels until `advance` moves it; `time-unavailable` retired, cause set eight; `advance` on every session. Matters here: the shot's byte-identity contract and decision/scrollbar-fade-on-document-clock — a canvas widget that animates or reads wall time would break same-state shots; time moves only on `advance` (decision/time-moves-only-on-advance).

@history_item n=6 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting verbs wait on a load at most `LOAD_WAIT`; `snapshot` and `screenshot` wait on none, still no wait argument or result field. Matters here: a canvas shot must not add a wait to `screenshot` (decision/no-wait-verb-fields, decision/time-moves-only-on-advance).

@history_item n=7 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check at rest: `0700` directory, `0600` `check-{n}.json` create-new, temp+rename, 400 records / 1,048,576 bytes bound ending the recording `recording-failed`, a foreign file never removed; strict `Check::from_bytes`. Matters here: the forks list names a kept check's shot record as a carrier canvas pixels might reach (decision/recording-never-replaces-files, decision/recording-bounds-end-recording, decision/recorded-check-strict-reader).

@history_item n=8 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check keeps each answer whole but a shot only as `{"width","height"}`, never its pixels; the two committed stand-Counter checks hold no `screenshot` step. Matters here: canvas pixels do not reach a kept check today; any change to that (or a canvas step in a committed check) is a widening (decision/kept-check-answer-whole, decision/committed-counter-checks, contract/kept-check-file/contents).

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a shot of an open list holds labels as pixels) — a shot of an open select list carries page option and group labels as pixels on socket and MCP, the shot's fields unmoved, hosted shot pinned equal to in-process by `cli_commands`; ratified by the founder as a boundary widening. Matters here: the closest precedent — a new content class inside the shot's pixels with no new field was still put to the founder (decision/open-list-shot-pixels).

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's visible region, paint and scroll) — `paint_scene` calls `paint_select_list` after the root's render and before devtools overlays, a non-finite box left unpainted, colours engine constants; an engine change for every document and a boundary widening, ratified. Matters here: precedent for where an engine-appended scene sits in blitz-paint and how a non-finite box is handled, which the custom widget scene path mirrors (decision/select-list-painted-over-page).

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list in the log) — the painted surface holds no log site, span or field; page text leaves only as a painted pixel and existing snapshot fields; `host_log`/`mcp_log` drive it with 0 needles at `trace`. Matters here: the silence check a new paint surface was held to (decision/no-screen-content-in-logs).

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses the carriers) — the list crosses only as a row's own `bounds` and as pixels; no `expanded` node key. Later widened by 2026-11-17-layout-on-the-carriers and 2026-11-20-select-list-and-selection-debts (highlighted bool). Matters here: decision/select-list-crossing's form is the model for stating how a painted surface crosses.

@history_item n=13 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — ~ decision/select-list-crossing gained the closed `highlighted` bool on the open list's highlighted row. Matters here: current wording of a picked row; a closed field inside a ratified carrier is no widening, pixels are.

@history_item n=14 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a custom widget's attributes replayed at set-up) — behind `custom-widget`, `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed`, the value's check the widget's own; an engine change for every document behind the feature, ratified; pinned by `custom_widget_replays_initial_attributes`. Matters here: enabling `custom-widget` in every build selection turns this on wherever the feature is now off (decision/custom-widget-initial-attribute-replay).

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` leg, locked) — a blitz-paint feature test (`--features blitz-paint/font-embolden --test select_popup_painted`) is a locked cargo leg in `LEGS`/`FAST_LEGS`. Matters here: precedent for a feature-gated blitz-paint check as a locked leg, should the canvas check need a feature selection (decision/locked-builds).

@history_item n=16 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — a Dioxus document renders into a buffer unborrowed, then replays under one borrow; a task polled during render acts on the previous render's document. Matters here: a Dioxus app handing a widget through the mutation writer (`<object data>` / `WriteOnceAttr`) goes through this buffered path (decision/dioxus-render-into-buffer).

@history_item n=17 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP crossing ratified: bounded request lines, typed arguments before `validate`, results ≤ 1,048,576 bytes, hand-written replies, `serde_json` read-only; the next carrier is a new question. Matters here: canvas pixels reach a model provider through MCP image blocks (decision/mcp-result-crossing, decision/mcp-bounded-lines, contract/mcp-tool-result/size-bound).

@history_item n=18 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers) — the socket widened to carry calls in and answers out, no NodeId, owner-only by file mode, bounds (16,384-byte request, 1,048,576-byte answer, 2 s / 30 s). Matters here: the shot travels this carrier under the unchanged bound (decision/session-socket-carries-calls, decision/session-socket-bounds, contract/session-socket/oversize).

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors) — the socket grammar moved to `v3` with `ok v3 accepted|refused|oversize|recording-failed`. Matters here: the reply forms a canvas shot answer rides (contract/session-socket-reply/ok-accepted, ok-oversize, ok-refused).

@history_item n=20 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings inside the ratified carriers; the closed-field rule) — set the standing rule: a new field of a closed value set inside a ratified carrier is no widening; authored text, an open value set, a new carrier or exit still is. Matters here: the test the canvas forks are judged by — app-painted pixels are an open value set (decision/boundary-widening-rule, decision/closed-value-field-no-widening).

@history_item n=21 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` states the MCP crossing and its carried set, adding no carrier; its secret search. Matters here: if canvas pixels join the carried set, the committed statement names them (decision/capability-statement-mcp-crossing).

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the pipe's second live run, a third carriage) — the live run carried a screenshot and six apps' screens to the model provider, ratified as a content widening on the same carrier. Matters here: a canvas shot on a live run carries app-painted pixels to the provider; the content widening is the founder's call (decision/mcp-result-crossing).

@history_item n=23 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — every driver call checked against a closed verb table and bounded argument kinds; a refusal a closed cause set with fixed texts holding nothing a call supplied. Matters here: the scope adds no verb, argument or cause unless a fork's ruling does (decision/closed-driver-verb-schema, decision/closed-refusal-cause-set, decision/refusal-holds-no-call-input).

@history_item n=24 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — one INFO `command` span per call, eight fixed-word or count fields, the verb the table's word, a refusal only by cause name; never an argument or screen text. Matters here: a shot records `verb` (and `cause` when refused) only; no new log field or span (decision/one-command-span-per-call, decision/command-span-content-free, contract/driver-command-span/*).

@history_item n=25 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's sink drops every outside-target record whole at every level, engine targets print only safe fields, escher targets redact content-named fields. Matters here: a canvas check reading logs must run against a sink-installing host; the "nothing in the logs" trap (decision/outside-target-dropped, decision/telemetry-sink-target-allowlist).

@history_item n=26 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — ~ decision/mask-holds-on-every-carrier and ~ decision/socket-carries-screen-reading: caused message and `error` kept as written, other content-named fields redacted. Matters here: current wording of picked rows; a widget paint panic would surface in an acting step's `caused`, not in a shot.

@history_item n=27 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — ~ decision/mask-holds-on-every-carrier guard gained `stand_masks_on_new_answers`. Matters here: the mask's guards a canvas change must keep green.

@history_item n=28 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `layout --id` on every carrier, answers ids, closed words and numbers; kept checks hold layout answers; the select list gains a layout placing word. Matters here: a canvas element read by `layout` stays closed words and numbers (decision/kept-check-layout-answer, decision/socket-carries-screen-reading); placement itself is out of scope.

@history_item n=29 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` verb ratified: ids, owner ids, `keyed`, verb and closed cause words, never a value; unpaged under the answer bound. Matters here: a canvas host element may appear in an actions answer; its shape must not change (contract/driver-verbs/actions, decision/actions-answer-unpaged, decision/one-actionable-rule).

@history_item n=30 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the `coverage` role reads caller-named kept checks through the strict reader; exit 0/1 forms. Matters here: picked rows contract/escher-session-exit-codes/0 and /1 name the coverage role; a canvas check in a kept check is read by the same reader (decision/recorded-check-untrusted).

@history_item n=31 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (API Security) — ~ decision/session-socket-sole-listener scoped to default builds; + decision/cdp-server-opt-in-loopback. Matters here: `--workspace` feature unification already turns on extra features; making `custom-widget` reach every selection must not pull an opt-in listener into a default build.

@history_item n=32 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Dependency Security) — ~ decision/engine-dependency-pins to Stylo 0.24 / winit =0.31.0-beta.4; + decision/skia-default-build-fetch. Matters here: the pins a canvas change must leave as they stand; no new crate beyond a taken fork.

@history_item n=33 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the audit runs `--locked --workspace --all-features` over the whole graph and gates licences. Matters here: enabling `custom-widget` in more selections is already inside the audited all-features graph; any new crate meets the licence gate (decision/dependency-audit-cargo-deny, decision/licence-gate).

@history_item n=34 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (advisories published after a green audit) — a CI audit can go red over an unchanged lock; run the audit leg locally before a push. Matters here: the CI verdict for `ac8e6d58a` is still unread; standing rule before this chunk's push (decision/local-audit-before-push).

@history_item n=35 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Dependency Security) — ~ decision/locked-builds reaches every cargo run of the build-directory sweep. Matters here: every build, test and CI leg this chunk adds passes `--locked`, as the scope repeats.

@history_item n=36 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness took `color`, reviewed as a Pinning bullet; the harness takes no `tracing` and no `style`. Matters here: any feature or crate the harness gains to paint a widget is recorded as a reviewed Pinning fact and keeps decision/test-harness-no-tracing-no-style.
