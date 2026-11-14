# architecture · the rows that bind 2026-11-14-seeded-randomness — picked 288 of 1481 (179 by every draw) · rule any · pin 741

## design-philosophy · ## Design Philosophy
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/traits-crate-for-interop · Shared traits crate for interoperability — Types and traits shared between Blitz crates live in blitz-traits. · why: so the crates interoperate without circular or unnecessary dependencies
decision/boa-for-javascript · Boa executes page JavaScript — blitz-vibey-script executes script JavaScript with Boa and exposes DOM APIs backed by blitz-dom.
decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines.
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/modular-embeddable-engine · Radically modular, embeddable engine — The engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle

## stack-and-technologies · ## Stack and Technologies
decision/cli-flow-under-plain-sh · CLI flow tests under plain sh — seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement. · why: no dependency, manifest line or test framework is added for it
decision/boa-javascript-engine · Boa as the JavaScript engine — Boa, with annex-b, is what executes script elements in blitz-vibey-script.
decision/dioxus-drives-blitz · Dioxus as the UI framework — The Dioxus virtual DOM drives Blitz through dioxus-native and dioxus-native-dom.
decision/serde-json-driver-reader · serde_json as the driver's JSON reader — serde_json is escher-driver's reader of MCP requests, of kept checks and of the live answer line a replay compares. · why: already in the graph through dioxus-html

## established-decisions · ## Established Decisions
decision/coverage-reads-named-checks · Coverage role reads named check files — The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly. · why: a ratified boundary widening
decision/coverage-role-reads-named-checks · Coverage role reads kept checks its caller names — The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path. · why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency.
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no subscriber and reads no env var.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call it runs and one tool_call span per MCP tools/call request, and emits no event, installs no subscriber, reads no env var and has no default state location.
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, closed words, booleans and counts cross a driver session, never a `NodeId`.
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
decision/serde-json-reader-only · serde_json as a reader only — The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing. · why: Ratified as read only.
decision/session-library-depends-on-no-app · Session library depends on no app — The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/examples-no-unicode-urls · Examples without unicode URL support — Examples pin `idna_adapter` to disable unicode URL support.
decision/examples-pin-idna-adapter · Examples pin idna_adapter — Examples pin idna_adapter to exactly 1.0.0. · why: to disable unicode URL support
decision/hosted-session-idle-expiry · Hosted session idle expiry — A hosted session expires: a host a command line starts takes IDLE_EXPIRY of 1,800 s, and a host that has answered no request for that long ends exactly as on stop; every answered request, a refused or malformed one included, starts the count again. · why: Answers the orphaned-host question.
decision/incremental-layout-default · Incremental layout on by default — Incremental layout defaults to on.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/single-frame-scripting · Scripting in a single frame only — JavaScript runs in a single frame only.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
contract/mcp-stdio/request-line · the mcp role's stdio · request line — A request line is at most 65,536 bytes.
contract/snapshot-verb/part · the snapshot verb · part — An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.

## conventions · ## Conventions
decision/blitz-js-native-prefix · Internal JS native prefix — Internal JS natives are prefixed `__blitz_`.
decision/builder-method-naming · Consuming builder method names — Consuming builder methods are named `with_…` / `without_…`. · except: blitz-traits' Request builders are named by their field, `signal` and `initiator`.
decision/consuming-builder-methods · Consuming builder methods — Builder methods on the script document consume and return the value, named `with_…` or `without_…`.
decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/driver-hand-built-spans · Driver spans built by hand — escher-driver builds its spans by hand, with no `#[instrument]`.
decision/error-enums · Error types as enums — Error types are enums with `Display` and a `From` per wrapped error.
decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
decision/in-repo-crates-no-default-features · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: the `seven_guis` path entry keeps its renderer feature
decision/in-repo-default-features-off · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: The `seven_guis` path entry keeps its default `hybrid` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer.
decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.

## standard-contracts · ## Standard Contracts
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only.
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal.
decision/hand-written-answers · Driver answers written by hand — Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks.
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/one-self-description-for-cli-and-mcp · One self-description for command line and MCP — The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing. · why: Ratified as the one table riding the ratified carriers.
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps
decision/session-apps-one-table · Session apps from one table — The escher-session usage line's apps and the MCP start tool's enum are written from one private table.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified.
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
contract/check-replay/another-app · Check replay · another app — A replay runs nothing on a session whose label is not the check's app.
contract/check-replay/first-difference · Check replay · first difference — Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
contract/cli-coverage-role/1 · Command line coverage role · 1 — A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file.
contract/cli-exit-statuses/0 · CLI exit statuses · 0 — accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
contract/cli-exit-statuses/1 · CLI exit statuses · 1 — refused: the refusal's JSON on stdout.
contract/cli-exit-statuses/2 · CLI exit statuses · 2 — usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
contract/command-line/serve · command line · serve — serve <app> --session <dir> [--record <dir>] in exactly that shape is the host role, ending quiet with status 0 after a stop or an expiry; any other shape is usage.
contract/command-line/start · command line · start — start <app> [--record <dir>] spawns the running binary as the host with standard streams closed, waits up to 30 s for it to answer and neither kills nor reaps it; an app the binary does not boot is usage.
contract/command-line/verb-flags · command line · verb flags — --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
contract/command-span/cause · command span · cause — The refusal's cause name, recorded for every refused call.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/command-span/verb · command span · verb — Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/driver-json/diff · Driver JSON answer · diff — added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
contract/driver-public-api/no-serde-json-types · Driver public API · serde_json types — No public accessor of a check, a step, a replay or a change returns or holds a `serde_json` type.
contract/driver-recording/handler-coverage · escher-driver recording · handler_coverage — handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/help · driver verbs · help — Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/driver-verbs/start · driver verbs · start — Session level; takes app; result label, pid and idle_expiry_s.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused; one line of JSON answers it on stdout.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage error: a fixed line on stderr, stdout empty, nothing booted and no state created.
contract/escher-session/help · escher-session command line · help — Needs no --session, admits at most one and reads it not, reaches no session, and answers the self-description's one line with status 0.
contract/escher-session/mcp · escher-session command line · mcp [--record <dir>] — An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
contract/escher-session/serve · escher-session command line · serve <task> --session <dir> [--record <dir>] — The host role, which the start command spawns.
contract/escher-session/start · escher-session command line · start <task> [--record <dir>] — The verb start takes a task and an optional --record directory and spawns the host role.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/kept-check-reader/refusal · Kept check reader · malformed refusal — Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path.
contract/kept-check-reader/strict-read · Kept check reader · strict read — A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the ten instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
contract/kept-check/arguments · Kept check file · arguments — A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
contract/kept-check/opening-record · Kept check file · opening record — A kept check opens with the record `{"tool":"start","arguments":{"app":<label>}}`.
contract/launch/dioxus-native-configs · high-level launch · dioxus-native launch configs — dioxus-native's launch, launch_cfg and launch_cfg_with_props downcast configs and drop unmatched ones.
contract/mcp-server/tool-result · MCP server · tool result — {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
contract/mcp-server/tools-call · MCP server · tools/call — Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
contract/mcp-server/tools-list · MCP server · tools/list — The fourteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/node-keys/invalid · node keys · invalid — Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent.
contract/node-keys/pressed · node keys · pressed — Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written.
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it.
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent.
contract/refusal-causes/malformed · refusal causes · malformed — An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
contract/refusal-faults/missing · refusal faults · Missing — A required argument is absent; beside malformed only.
contract/refusal-faults/out-of-bound · refusal faults · OutOfBound — An argument's value is outside its bound, or a text is no key name; beside malformed only.
contract/refusal-faults/repeated · refusal faults · Repeated — An argument name was passed before in the same call; beside malformed only.
contract/refusal-faults/unnamed · refusal faults · Unnamed — An argument names nothing the verb takes; beside malformed only.
contract/refusal-faults/wrong-kind · refusal faults · WrongKind — An argument's value is of another kind than the schema's; beside malformed only.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/script-document/first-poll · Script document · script execution — Scripts run on first poll if not run explicitly.
contract/session-client/call-answer-read-bound · session client · call answer read bound — A call is validated before sending and fits the request bound; its answer is read under a 30 s bound on each read, timeout past it.
contract/session-client/start · session client · start — Attaches first and spawns nothing when answered; else spawns the host, polls every 25 ms until it answers, exits or the ready time passes; it does not check the answering pid is its child.
contract/session-error-json/shape · session error JSON · shape — A session error is written {"error":{"kind","message","remedy"}}; every message is a fixed string carrying no path, label, id or value.
contract/session-errors/unsupported · session errors · unsupported — On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere.
contract/session-wire/call-request · session wire · call v3 — call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
contract/session-wire/ok-refused · session wire · ok v3 refused — A call's refusal, its JSON line after the words.
contract/session-wire/refused-malformed · session wire · refused malformed — Any other request, a call the decoder does not read included; changes nothing.
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
contract/snapshot-diff/order · Snapshot diff · order — added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
contract/snapshot-diff/reading · Snapshot diff · Reading — Twelve variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Bounds, with Reading::ALL.
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
contract/snapshot-diff/values · Snapshot diff · values — Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot-text/lines · Snapshot text · lines — One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
contract/snapshot-text/quoting · Snapshot text · quoted strings — A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
contract/snapshot-text/state-tokens · Snapshot text · state tokens — In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable; absent where the model holds nothing, invalid and clickable written only when true.
contract/snapshot-text/to-text · Snapshot text · to_text — The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/invalid · Snapshot · invalid — True where the accessibility node carries any invalid value, a grammar or spelling error included.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/name · Snapshot · name — The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries eight fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable.
contract/snapshot/pressed · Snapshot · pressed — The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/snapshot/value · Snapshot · value — A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted.
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/tool-call-span/ending · the tool_call span · ending — One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
contract/validation-order/first-failure · call validation · first failure — One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
decision/boa-script-engine · Boa runs document scripts — A document's scripts run on Boa, JS listeners before Blitz default actions.
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
decision/js-engine-boa · Boa as the JavaScript engine — Script documents run HTML with JavaScript enabled on a Boa-based engine.
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified.
decision/refused-call-records-no-ids · A call validation refuses records no ids — A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too. · why: Ratified.
contract/argument-kinds/flag · argument kinds · flag — A boolean.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/argument-kinds/key · argument kinds · key — One of the twelve key names, else out of bound.
contract/argument-kinds/milliseconds · argument kinds · milliseconds — A whole number from 1 to 60000.
contract/argument-kinds/name · argument kinds · name — 1 to 32 bytes of a-z, 0-9 and -, the app a start names.
contract/argument-kinds/part · argument kinds · part — A whole number from 1 to 1000, the snapshot part asked for.
contract/argument-kinds/text · argument kinds · text — 0 to 4096 bytes of text.
contract/blitz-dom-clock/virtual-clock · blitz-dom clock API · VirtualClock — Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
contract/check-replay/change-keys · Check replay · change keys — A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cli-coverage-role/0 · Command line coverage role · 0 — Accepted: the coverage line is printed on stdout.
contract/command-line/closed-pipe · command line · closed pipe — A write to a closed pipe is dropped, not a panic, and the ending keeps its status; no ending writes back anything the caller typed.
contract/command-line/help · command line · help — help with no argument and at most one unread --session answers the self-description on stdout, status 0, reaching no session; an argument is refused malformed.
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control.
contract/document-clock/virtualclock · blitz-dom document clock · VirtualClock — Every clone of a VirtualClock reads one time.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary.
contract/escher-session-cli/answer · escher-session command line · answer — escher-session answers one line of JSON.
contract/escher-session-cli/usage · escher-session command line · usage — escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session/answer-line · escher-session command line · answer — A command answers one uncoloured line of JSON on stdout.
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
contract/escher-session/session-flag · escher-session command line · --session — Every verb but help takes --session <dir> exactly once.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/graph-registry/use-graph-state · Graph registry · use_graph_state — Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
contract/handler-coverage/per-check-boot · Handler coverage reader · per-check boot — A fresh session is booted for each check; a check whose app is not the label runs nothing and contributes nothing.
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/kept-check/digest · Kept check file · digest — `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits.
contract/kept-check/kept-verbs · Kept check file · kept verbs — Only calls of the ten instance verbs are kept, `lint`, `graph` and `actions` among them with their answers whole; any other call is not kept.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/recording/failure · Session recording · recording failure — Past the bounds or on a failed write the recording ends: its check is removed, nothing further is kept, the call is not rolled back, and the session reports the failure once.
contract/session-errors/already-running · session errors · already-running — A host already answers on the state directory's socket; start spawns nothing and an MCP start while a session is held reads it.
contract/session-errors/host-exited · session errors · host-exited — The spawned host exited before the session answered, carrying its exit code where there is one.
contract/session-errors/record-dir-not-private · session errors · record-dir-not-private — One of the fifteen fixed session error kinds, with a fixed message and remedy; a start or mcp given --record checks the recording directory first.
contract/session-errors/timeout · session errors · timeout — A bound passed: stop waited past 5 s, start past its ready time with the host killed and reaped, or an answer read past 30 s.
contract/session-wire/ok-accepted · session wire · ok v3 accepted — A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
contract/session-wire/reply-bound · session wire · reply bound — A hello, stop or refused reply is at most 128 bytes.
contract/session-wire/version · session wire · version — The wire is versioned v3: one request line and one reply line per connection, each newline-terminated.
contract/snapshot-node-keys/expanded · Snapshot node keys · expanded — The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fourteen keys.
contract/snapshot-text/text-parts · Snapshot text · text_parts — Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
contract/snapshot/multiple-select-bounds · Snapshot · multiple select rows — A multiple select's rows and groups read their own client rects as their bounds.
contract/telemetry-line/span-close-line · Telemetry line · span close line — One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
contract/telemetry-scrub/other-targets · Telemetry scrub · other targets — An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}.
contract/verb-table/order · Driver verb table · VERBS — The verb table holds fourteen verbs, `graph` listed after `lint` and `actions` after `graph`.

## occupied-resources · ## Occupied Resources
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium.
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified.
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
contract/driver-diff/differs · the driver diff · differs — A changed node's differs names closed node-key words, bounds among them.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
contract/mcp-role/help · the mcp role · help — The help tool is answered by the server with no session run.
contract/mcp-role/self-description · the mcp role · self-description and remedy — A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text.
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
contract/session-socket-wire/request-line-bound · the session socket's wire · request line bound — A request line is at most 16,384 bytes.
contract/session-socket-wire/version · the session socket's wire · version — The wire version is v3.
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
decision/kept-checks-owner-only · Kept checks on an owner-only medium — A kept check lives on an owner-only medium. · except: the two committed checks under the blitz-tests checks directory.
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener.
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified.
decision/recording-removes-only-own-checks · Recording removes only its own checks — Escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the recording directory.
decision/recording-removes-own-checks-only · Recording removes only its own checks — escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
decision/session-state-dir-caller-named-private · Session state directory caller-named and owner-only — The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others.
decision/session-state-directory · Driver session state directory — The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry.
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out
contract/command-span/actions-call · The command span · actions call — An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role.
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
contract/recording-directory/check-file · The recording directory · check-{n}.json — One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename.
contract/session-socket-wire/call · the session socket's wire · call — The call request runs a driver call and returns its answer.
contract/session-socket-wire/non-unix-targets · the session socket's wire · non-unix targets — On targets other than unix the lifecycle and a call return SessionError::Unsupported.
contract/session-socket-wire/recording-failed · the session socket's wire · recording-failed — A call's replies include the fixed recording-failed reply.
contract/telemetry/rust-log · escher-telemetry bootstrap · RUST_LOG — RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.

## infrastructure-patterns · ## Infrastructure Patterns
decision/js-entry-redraws-and-arms-timer · JS entry points request a redraw and arm the timer thread — Each JS entry point requests a redraw and re-arms the timer thread that wakes the event loop at the next JS timer deadline. · why: The windowed shell repaints after script work and wakes for due JS timers only through this; the WPT runner opts out with without_timer_thread
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry` and `reached` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing
decision/dioxus-root-contexts · Dioxus services as root contexts — Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct.
decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no subscriber.
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables.
decision/shared-state-rc-refcell · Shared state as Rc RefCell — Shared state is `Rc<RefCell<…>>`: the blitz document, and JS state in a `DomCtx` on the Boa context.
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve
decision/telemetry-closed-span-lines · Telemetry prints closed spans — The telemetry sink prints a closed span's line beside an event's. · why: Ratified.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
decision/escher-name-spelling · Spelling of the Escher name — The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged.
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.

## inherited-defaults · ## Inherited Defaults
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.

## existing-scopes · ## Existing Scopes
contract/escher-session-command-line/coverage · escher-session command line · coverage <app> <check>... — The coverage role lists the handlers its named kept checks left unexercised, the same boot closure called once per check.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port: no TCP or UDP port any binary serves
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener
