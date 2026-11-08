@carried n=1 label=security-plan/data-protection fact=fct-cl72c3yo
why: The new verb's recorded step must be kept the same way, with its answer masked as on every carrier, and the graph content it holds at rest (component, handler and state names) must never be logged.
quote: one `check-{n}.json` (`0600`) per recorded session in a `0700` directory the launcher named — a JSON array: the app label, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer (a shot's PNG included) and, since 2026-10-19-recorded-check-replay, the answer itself as `answer` — the line the driver wrote, masked as on every carrier (a password's or file input's value as `MASKED_VALUE`)
@carried n=1 run=2 label=security-plan fact=fct-kdtkdrex
quote: per call its verb, arguments, acceptance, digest and masked answer, never a pixel; owner-only, except the two committed checks (below); never logged
@carried n=2 label=security-plan/data-protection fact=fct-6n6morf4
why: The committed Counter checks hold snapshot text that the new reacts-to-a-click field changes, so they must be re-recorded by the binary, still hold no typed text, host path, name or secret, and match a fresh recording byte for byte.
quote: holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls — no typed text, host path, name or secret (both read by the operator, 2026-10-19) and no `screenshot` step (a shot's PNG bytes differ by OS, as measured at CI#39225375577); read only through the strict `Check::from_bytes` (`stand_replay`) and compared byte for byte with a fresh recording (seven_guis `recording::the_committed_checks_are_what_the_binary_records`); a check a launcher records of any other session stays owner-only and is never committed
@carried n=2 run=2 label=security-plan fact=fct-sw7lpzl2
quote: holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`; a check of any other session stays owner-only and is never committed.
@carried n=3 label=security-plan/api-security fact=fct-svo6bgmr
why: The new verb's MCP tool joins the twelve this row counts, so the count must move, and its result must stay within the 1,048,576-byte tool-result bound over stdio with no new listener.
quote: Since 2026-10-10-mcp-surface the `escher-session mcp` role also serves the driver's verbs as MCP tools — twelve: the tenth, `help` (since 2026-10-12-self-description), answered by the server itself from the schema's fixed tables and running nothing on a session, the eleventh, `screenshot`, and the twelfth, `lint` — over its own stdin and stdout — no listener, port or socket, so the session socket stays the one listener — bounded by a 65,536-byte request line, a 1,048,576-byte tool result
@carried n=3 run=2 label=security-plan fact=fct-s4jqcqqv
quote: the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results
@carried n=4 label=security-plan/api-security fact=fct-brzrm7cu
why: The new verb joins the instance-level verbs, so it must take no wait argument and answer no wait field, and the verb count this row states must move with it.
quote: the driver's command schema keeps this: none of its eight instance-level verbs has an argument or a result field of a wait
@carried n=4 run=2 label=security-plan fact=fct-dlhusvs4
quote: no verb has an argument or a result field of a wait.
@carried n=5 label=security-plan/api-security fact=fct-t6pddbhw
why: The new verb's answer and outcome travel on the session socket, so a whole screen's graph must fit the answer bound or come back as `oversize`.
quote: one connection at a time, a 2 s read and write bound and 30 s on a client's read of a call's answer, a request line of at most 16,384 bytes, a lifecycle reply of at most 128 and an answer of at most 1,048,576 on its JSON line — over it the reply is `oversize`, read by the client as `AnswerTooLarge`
@carried n=5 run=2 label=security-plan fact=fct-sn4avmoz
quote: one connection at a time under fixed read, write and size bounds
@carried n=6 label=security-plan/dependency-security fact=fct-a4qh6ioi
why: The registry and graph reader must use dioxus-core's public API only, with no Dioxus fork, no package entering `Cargo.lock` and no ignore added, and the whole-workspace audit must stay green under `--locked`.
quote: **Audit tool:** cargo-deny 0.20.2 — `cargo deny --locked --workspace --all-features check advisories licenses` against the RustSec advisory database and the licence table, over the whole workspace's graph
@carried n=6 run=2 label=security-plan fact=fct-cappgruj
quote: ignores per ID only, each with a written reason, no blanket allow
@carried n=7 label=security-plan/dependency-security fact=fct-3t4xl422
why: serde_json must stay featureless and read-only: it reads the new verb's MCP arguments, its recorded step and its replayed answer, and the new written form must not be produced through it.
quote: inherited by escher-driver with no feature and no `[features]` table — so no `preserve_order` (packages/escher-driver/Cargo.toml:17): the reader of the MCP server's untrusted stdin (`from_str::<Value>`) and, since 2026-10-19-session-recording, of a kept check's bytes (`Check::from_bytes`) and, since 2026-10-19-recorded-check-replay, of the driver's own answer line the replay compares (`from_str` in `Check::replay`), writing nothing
@carried n=7 run=2 label=security-plan fact=fct-ldu2bkhj
quote: the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's own answer line the replay compares, writing nothing
@carried n=8 label=security-plan/error-handling fact=fct-bzel2phc
why: Any refusal or outcome the new verb adds must stay inside the closed eight-cause set of fixed `&'static str` texts with no String field, and the help texts the new field changes must stay fixed literals holding nothing a call supplies.
quote: the cause set is closed at eight (ratified by the founder, 2026-10-18) — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen` and, since 2026-10-12-headless-screenshot, `fonts-unavailable` — each with a fixed non-empty `&'static str` name, meaning and remedy that holds no path
@carried n=8 run=2 label=security-plan/error-handling fact=fct-bzel2phc
quote: since 2026-10-07-driver-command-spans exactly one thing of a refusal reaches a log: its cause's fixed name, recorded as the `cause` field of the call's `escher_driver` span at one site for every refusal `Session::run` returns
@carried n=8 run=3 label=security-plan fact=fct-ugqfmfy4
quote: a `SessionError` is written from a fixed table likewise; MCP protocol errors are fixed codes and messages with no `data`; of a refusal only its cause's fixed name reaches a log.
@carried n=9 label=security-plan/error-handling fact=fct-dpj6t54a
why: The graph reader and the signal registry must resolve elements through the option-returning `try_element_to_node_id`, as the mounted-event flush does, so that a stale or removed element is skipped and never panics.
quote: `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:242-246)
@carried n=9 run=2 label=security-plan fact=fct-c4rr3jfv
quote: `try_doc` returns `None` when the document is mutably borrowed (packages/dioxus-native-dom/src/events.rs:155-177); `try_element_to_node_id` returns an option (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
@carried n=9 run=3 label=security-plan fact=fct-f4v2lnec
quote: `NodeHandle::node`/`node_mut` panic with "Node does not exist in the Document" (packages/dioxus-native-dom/src/events.rs:155-177); `element_to_node_id` unwraps (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
@carried n=10 label=security-plan/logging-monitoring fact=fct-4hqe6pzr
why: The escher-target rule redacts only the eleven content-named fields, so no new span field may carry a node, component name, handler, state or value, and the needle checks must cover the new verb's graph content.
quote: an event whose target starts with `escher_` prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted; an event from any other target is not printed at all
@carried n=10 run=2 label=security-plan/logging-monitoring fact=fct-4hqe6pzr
quote: `host_log` and `mcp_log` each driving one `lint` and one `snapshot` part read with 0 needles, so a lint's findings and a part's number, count and text reach no log either
@carried n=10 run=3 label=security-plan fact=fct-ggtnnptr
quote: an `escher_` target all but eleven content-named fields; any other target dropped whole at every level, spans by the same rule
@carried n=11 label=security-plan/logging-monitoring fact=fct-uh4dhzjw
why: The new verb's answer, refusal or session error must go to escher-session's stdout as one uncoloured JSON line, holding nothing the caller typed.
quote: writes one uncoloured line of JSON to stdout per command — the answer (status 0), the refusal (status 1) or the session-error object (status 3) — its usage line (status 2, stdout empty) and a session error's fixed message going to stderr, its host role writing nothing on its quiet end, and since 2026-10-10-mcp-surface its `mcp` role writing JSON-RPC protocol lines only on stdout, nothing at its end, an `Err` ending's fixed message on stderr with status 3; no line holds anything the caller typed — in the `mcp` role but the admitted request `id` it echoes — or an ESC byte
@carried n=11 run=2 label=security-plan fact=fct-eabjzys7
quote: `escher-session` writes one uncoloured JSON line per command on stdout, its `mcp` role JSON-RPC lines only, usage and error messages on stderr, and no line holds anything the caller typed but the `mcp` role's echoed request `id`
@carried n=12 label=security-plan/logging-monitoring fact=fct-ksl3fslx
why: The new verb must add no span beyond escher-driver's two `info_span!` sites and no span field that carries graph content, or the count `stand_act_spans` reads must be re-pinned.
quote: `packages/escher-driver/src` holds two `info_span!`, `stand_act_spans` reads eight span fields
@binding n=1
**logging-monitoring ↔ data-protection**: the graph's component, handler and registered state names may be at rest in a kept check's masked answer (security-plan/data-protection fct-cl72c3yo) but must reach no log, because the escher-target rule prints every field outside its eleven content-named ones (security-plan/logging-monitoring fct-4hqe6pzr). So the command span keeps fixed words, bools and counts only (security-plan/logging-monitoring fct-ksl3fslx).
@binding n=2
**api-security ↔ error-handling**: the new verb's answer must stay within the socket's 1,048,576-byte answer bound and the MCP tool-result bound (security-plan/api-security fct-t6pddbhw; security-plan/api-security fct-svo6bgmr), and any refusal or outcome it adds must stay a fixed-text cause of the closed set (security-plan/error-handling fct-bzel2phc).
@binding n=3
**dependency-security ↔ data-protection**: a recorded step of the new verb must be read back only through the strict `Check::from_bytes`, over a serde_json that stays read-only and featureless (security-plan/dependency-security fct-3t4xl422). The re-recorded committed Counter checks must match a fresh recording byte for byte (security-plan/data-protection fct-6n6morf4).
@binding n=4
**error-handling ↔ logging-monitoring**: the graph reader must take the `try_element_to_node_id` and `try_doc` paths on a stale or missing id (security-plan/error-handling fct-dpj6t54a; security-plan fct-c4rr3jfv), and must not panic or log that id (security-plan/logging-monitoring fct-4hqe6pzr).
@binding n=5
**dependency-security ↔ the body**: the graph must be read through dioxus-core's public API only, with no package entering `Cargo.lock` and no ignore added to the whole-workspace audit (security-plan/dependency-security fct-a4qh6ioi; security-plan fct-cappgruj).
