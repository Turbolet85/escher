@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set tracing as compiled only with the `tracing` feature in the engine and upstream crates, while escher-telemetry is ungated, and introduced the allowlist scrub. This is the origin of telemetry-via-tracing / engine-tracing-feature-gated. A canvas paint path in blitz-dom or blitz-paint gets log sites only behind that feature, and those sites are scrubbed only where escher's sink is installed.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — recorded that the headless stand and its in-process checks install no subscriber, read no env var and print nothing. This is the origin of decision/headless-stand-no-sink. A canvas stand check this chunk adds must stay silent in the same way.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — made `escher-session` a second sink installer and scoped "a headless boot has no escher sink" to a boot made in process (decision/scrub-sink-reach). Trap: a zero read from a host that installs no sink says nothing about one that does. A canvas shot measured in process does not speak for the `escher-session` binary.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink drops every target outside the engine and `escher_` prefixes. Standing rule: a target is admitted only together with a scrub rule for it. Any canvas or widget log target (for example a new crate's) is dropped unless both an allowlist entry and a scrub rule are added.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — standing rule: never field a `Command`, `Call`, `ArgValue` (and later `Outcome`) with `Debug`, nor an argument by its schema name; the span's cause field takes a `Cause` name. This is the root of decision/debug-types-never-logged and decision/command-span-no-call-content, which bind any canvas outcome or argument.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — built the `command` span with eight fields and took escher-driver's `tracing` ungated. Recorded that a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events. This is the origin of decision/engine-tracing-by-invocation and decision/escher-crates-always-trace. It is the same package-vs-workspace feature unification that this chunk finds behind blitz-paint's `custom-widget`.

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line (contract/command-span/line) as a second record class. A new record class or span field from the canvas would be another boundary widening for the founder.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (two entries) stated that each command's answer is one JSON line on stdout, an answer and never a log line (contract/escher-session-stdout/command-answer). Standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads. Also measured the session host's stderr by level under the package-alone and workspace builds (one engine `blitz_dom::document` line only in the workspace build). This is the precedent for reading a canvas shot under both build selections.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the ratified `tool_call` span (`tool`, `ending`) and the `mcp` role, whose stdout carries protocol lines only (decision/driver-span-fields-print, contract/tool-call-span/line, contract/mcp-role-stdout/json-rpc). A canvas shot over MCP leaves as an image block and adds no field to either span.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the direct precedent for this chunk. It added the `screenshot` verb and the `fonts-unavailable` and `off-screen` causes. An accepted shot records `verb` alone, and the image, its byte size and its width/height are never fielded. `Harness::screenshot` / `screenshot_rect` and the `image` encoder were recorded spanless and silent, and `stand_screenshot_repeat` was added as a re-executed child. A shot's image leaves only in its answer, with 0 image bytes on stderr under both builds. A canvas's pixels must stay inside that contract (contract/command-answer/shot, decision/shot-image-leaves-only-in-answer, contract/command-span/screenshot).

@history_item n=11 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — re-read the workspace-built host: its `focussed node` engine line printed twice, while the build alone was unchanged. The trap: per-build stderr readings move with feature unification, so a canvas chunk that changes which features a build selection resolves must re-read `host_log` / `mcp_log` under both builds.

@history_item n=12 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — read a client's stderr, the host and the workspace-built windowed `seven_guis_native` by build and level: 28 runs, 0 needles. This is the precedent for measuring the windowed binary under the workspace build that turns `custom-widget` on.

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — set eight cause names, `fonts-unavailable` among them, with `time-unavailable` retired. The founder ratified this on 2026-10-18 (see the next record). The scope says a canvas shot keeps `fonts-unavailable` before anything is painted, and the ruling allows no new cause unless a fork's ruling adds one (contract/refusal-cause/*, contract/command-span/cause).

@history_item n=14 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified) the founder ratified the eight cause names, `fonts-unavailable` and `off-screen` among them, which make up the closed cause domain a canvas shot's refusal uses.

@history_item n=15 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — kept checks rest in a `0700` recording directory and record calls with their accepted flag and digest (contract/kept-check/*). A canvas shot taken in a recorded session is kept at rest as a call.

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorder keeps each answer whole, a shot's without `png` (contract/kept-check/answer, contract/recording-file/answer). The committed Counter checks are in git (decision/committed-kept-checks-clean). A canvas shot's answer at rest is width and height only, and the committed checks' readings stand unless a ruling moves them.

@history_item n=17 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of those claims. If this chunk claims v040-09 there, any change to the span set or the scrub limits has to be made in that file by hand.

@history_item n=18 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the pipe's verdict fields) made `screenshot` a counted verb read from `server.log` span lines (contract/cold-agent-counted-verbs/screenshot, contract/cold-agent-verdict/screenshot-calls). The transcript carries a shot to the model provider on a live run, and `server.log` and `check.json` are never printed (decision/cold-agent-artifacts-never-printed). Canvas pixels in a shot would cross that transcript carrier, so the canvas-content fork must count it.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the closest boundary precedent. A shot of an open list carries a page's option and group labels as pixels, a boundary widening the founder ratified with the shot's fields unmoved (decision/open-list-shot-carries-labels). blitz-paint's `render::select_list` and blitz-dom's `node::select_list` add no log site or span. A hosted shot equals the in-process one, and `host_log` / `mcp_log` read the open list at `trace` with 0 needles. A canvas's painted content is a new pixel content class that goes to the operator as a fork in the same form, and its paint path follows the same silence and census readings.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `multiple` select's leave points) a `multiple` select's snapshot value is the first chosen option in tree order (contract/snapshot/multiple-select-value), and each new engine surface is read with the added-lines census at 0. This is a picked PII row that the canvas leaves unmoved.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the take read silent) `set_custom_widget`'s replay was recorded as opening no span: this is the custom-widget path this chunk wires. Standing trap from the failed-load entry: a trait default can hide a take's fix (it stopped at `CountedHandler`), so sweep the implementers of a trait that a change extends. That applies to blitz-dom's `Widget` trait. Kept: `telemetry-strategy.md` names `ci-leg.sh fast` among the workspace builds although `fast` ends on a `cargo test -p blitz-tests` step, which matters for which build selection each CI leg paints under.

@history_item n=22 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the mutation buffer holds attribute values in process memory only and its `Debug` prints its length (decision/mutation-buffer-memory-only). The replay re-fires `MutationWriter`'s `trace!` calls. The `<object data>` widget hand-off and any `<canvas src>` wiring run through this buffer and writer, so nothing may log or field a widget's or an attribute's contents.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the snapshot's `clickable` reading is a closed bool carrying no page text (contract/snapshot-node/clickable, contract/snapshot-text/clickable). These are picked snapshot rows that a canvas element must leave unmoved in the stand's snapshot digests.

@history_item n=24 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the actions answer's carried set (ids, owner ids, verb words, closed cause words, booleans) and its shape were ratified as a widening (contract/actions-answer/*), and `actions` joined the command span's verbs. The seven served tasks' actions digests are among the readings this chunk keeps unless a ruling moves one.

@history_item n=25 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — widened decision/checks-silent and decision/headless-stand-no-sink to name `stand_layout`'s recording subscriber as an exception. A new canvas check that captures records must be added to these named exceptions, or else stay silent.

@history_item n=26 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — reworded decision/headless-stand-no-sink, decision/driver-library-installs-no-subscriber and decision/driver-emits-no-event: the driver sets escher-telemetry's scoped capture around an acting step and that capture prints nothing. A shot is not an acting step, so a canvas shot runs outside the capture.

@history_item n=27 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — gave the current wording of decision/headless-stand-no-sink (the error-text founder check is an exception), decision/checks-silent and decision/driver-library-installs-no-subscriber (the chaining panic hook). It also added decision/engine-sites-fixed-messages: engine warn and error sites write fixed messages with their values in named fields. A canvas or widget warn site in blitz-paint must follow that convention.

@history_item n=28 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — set the exception in decision/answers-echo-nothing-typed: a caused item's `error` field is shown as written, cut at the text bound. This is the current wording of that picked row, and the founder ratified it on 2026-11-17.

@history_item n=29 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grew to fifteen words (contract/command-span/verb), and contract/command-span/settled lists the verb-alone reads, screenshot among them. This is the precedent for a new verb's per-verb span contract. The scope allows none for the canvas unless a fork's ruling adds one.

@history_item n=30 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** — added `refused_by_cause` (closed cause words to counts) and widened decision/verdict-fields-closed to "an object of closed words to counts". The pipe reads `server.log` by span name and cause word only. A new cause from a canvas ruling would enter that count's closed domain.

@history_item n=31 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `capability-unused` also fires when any of `caused_by_kind`'s kinds reads 0 (contract/cold-agent-verdict/capability-unused). The checks-silent exception now names the order page's log site (decision/checks-silent), which shows that a stand page with a log site must be named as an exception.
