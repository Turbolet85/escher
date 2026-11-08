@carried n=1 label=security-plan/data-protection fact=fct-cl72c3yo
why: the owed replay test must keep its `actions` step in an owner-only kept check whose answer holds no control value, typed text or `NodeId`, and it discharges the replay this row records as owed.
quote: and an `actions` answer (ids, owner ids, `keyed` booleans, the table's verb words and closed cause words, never a control value, typed text or `NodeId`; a boundary widening, ratified, the `keyed` field PROVISIONAL; the replay of a kept `actions` step exercised by no test yet, owed on «Handler coverage»), never a pixel — never logged, and never removed by escher but for its own check when a recording ends;
@carried n=1 run=2 label=security-plan fact=fct-kdtkdrex
quote: one `check-{n}.json` (`0600`) per recorded session in a `0700` directory the launcher named — per call its verb, arguments, acceptance, digest and masked answer, never a pixel; owner-only, except the two committed checks (below); never logged
@carried n=2 label=security-plan/api-security fact=fct-svo6bgmr
why: a coverage verb, if the P4 fork chooses one, must join the MCP tool listing in its fixed order and move the count of fourteen everywhere it is pinned, under the same request and result bounds.
quote: Since 2026-10-10-mcp-surface the `escher-session mcp` role also serves the driver's verbs as MCP tools — fourteen: the tenth, `help` (since 2026-10-12-self-description), answered by the server itself from the schema's fixed tables and running nothing on a session, the eleventh, `screenshot`, the twelfth, `lint`, the thirteenth, `graph`, listed after `lint`, and the fourteenth, `actions`, listed after `graph`, each of the two taking no argument and answered as one text block — over its own stdin and stdout — no listener, port or socket, so the session socket stays the one listener — bounded by a 65,536-byte request line, a 1,048,576-byte tool result
@carried n=2 run=2 label=security-plan fact=fct-s4jqcqqv
quote: the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results
@carried n=3 label=security-plan/api-security fact=fct-brzrm7cu
why: a coverage verb must take no wait argument, answer no wait field, run no step and settle nothing, as the scope's read-only coverage read requires.
quote: the driver's command schema keeps this: none of its ten instance-level verbs has an argument or a result field of a wait
@carried n=3 run=2 label=security-plan fact=fct-dlhusvs4
quote: no verb has an argument or a result field of a wait.
@carried n=4 label=security-plan/api-security fact=fct-t6pddbhw
why: a coverage answer carried on the session socket must stay within the 1,048,576-byte answer bound, an oversize one read as `AnswerTooLarge`.
quote: a request line of at most 16,384 bytes, a lifecycle reply of at most 128 and an answer of at most 1,048,576 on its JSON line — over it the reply is `oversize`, read by the client as `AnswerTooLarge`;
@carried n=4 run=2 label=security-plan fact=fct-sn4avmoz
quote: one connection at a time under fixed read, write and size bounds;
@carried n=5 label=security-plan/api-security fact=fct-s6ul72yw
why: the coverage read reads the unpaged `graph` answer, which must keep its shape with no part argument, its stated limit kept in docs/capabilities.md.
quote: `graph` takes no argument and its answer is unpaged — no part argument, which would widen the crossing — so a screen's whole graph is one answer under the 1,048,576-byte answer bound
@carried n=6 label=security-plan/api-security fact=fct-hbfh5ibd
why: the kept `actions` answer the owed replay test replays must stay unpaged under the answer bound, its stated limit kept in docs/capabilities.md.
quote: `actions` takes no argument and its answer is unpaged — no part argument — so a screen's whole answer is one answer under the 1,048,576-byte answer bound: Cells' answer reads 106,088 bytes for 678 nodes through the binary, held within the bound in both layout modes by `stand_actions`' `the_whole_cells_answer_is_within_the_answer_bound` (tests/blitz-tests/tests/stand_actions.rs:546-560); its size the stated limit «An unpaged actions answer» in `docs/capabilities.md`, beside «An unpaged graph», owned by residual `res-7cc5bi4k` (target: next)
@carried n=7 label=security-plan/dependency-security fact=fct-3t4xl422
why: the owed replay test's read of the kept check and its `actions` answer equality must go through serde_json with no `preserve_order`, writing nothing, and no dependency enters.
quote: inherited by escher-driver with no feature and no `[features]` table — so no `preserve_order` (packages/escher-driver/Cargo.toml:17): the reader of the MCP server's untrusted stdin (`from_str::<Value>`) and, since 2026-10-19-session-recording, of a kept check's bytes (`Check::from_bytes`) and, since 2026-10-19-recorded-check-replay, of the driver's own answer line the replay compares (`from_str` in `Check::replay`), writing nothing
@carried n=7 run=2 label=security-plan fact=fct-ldu2bkhj
quote: the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's own answer line the replay compares, writing nothing; no ban list exists (§Security Anti-Patterns).
@carried n=8 label=security-plan/secret-management fact=fct-yffuq3co
why: docs/capabilities.md and the capability-statement pin, edited when a new verb moves the count, must keep zero host-path, key, auth-header and e-mail hits, and the pin must read no environment variable.
quote: searched: `/home/|api[_-]?key|authorization:|bearer ` (case-insensitive) over `README.md`, `docs/capabilities.md` and `.github/scripts/test_capability_statement.py` (0 hits, as measured at escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md), and two e-mail domains (case-insensitive) over `docs/capabilities.md` and the pin (0 hits); the pin reads no environment variable
@carried n=8 run=2 label=security-plan fact=fct-pz4e3fuz
quote: the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
@carried n=9 label=security-plan/error-handling fact=fct-bzel2phc
why: a coverage call given an argument must be refused `malformed` with nothing run on every carrier, the cause set staying eight with no cause added.
quote: a `graph` call with an argument is refused `malformed` with nothing run, no cause added, the set staying eight, and so is an `actions` call with an argument, in process, on the command line (status 1) and over MCP (`isError`); `actions` reads each listed node's cause through the executor's own per-verb check, running nothing, so it adds no cause;
@carried n=9 run=2 label=security-plan fact=fct-ugqfmfy4
quote: `Refusal`: a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and a `Fault` only beside `malformed`; no `String` field, so a refusal cannot hold anything a call supplied; a `SessionError` is written from a fixed table likewise; MCP protocol errors are fixed codes and messages with no `data`; of a refusal only its cause's fixed name reaches a log.
@carried n=10 label=security-plan/error-handling fact=fct-7ni72vcj
why: the reached-handler record added beside the dispatch in dioxus_document.rs must keep the buffered render, taking the document's borrow once and reintroducing no `RefCell already borrowed` panic.
quote: `DioxusDocument::initial_build` and `Document::poll` render into the crate-private `MutationBuffer` with the document unborrowed, then take `inner.borrow_mut()` once, replay through one `MutationWriter`, drop the borrow and flush the queued mounted events
@binding n=1
**api-security verb table ↔ error-handling refusals**: a coverage verb, if chosen at P4, joins the MCP listing as the fifteenth tool with no wait argument or field and runs no step (security-plan/api-security fct-svo6bgmr) (security-plan/api-security fct-brzrm7cu), and a call to it with an argument is refused `malformed` with nothing run, the cause set staying eight with fixed texts (security-plan/error-handling fct-bzel2phc).
@binding n=2
**data-protection kept checks ↔ dependency-security serde_json**: the owed replay test keeps its `actions` step in an owner-only check with no control value, typed text or `NodeId` (security-plan/data-protection fct-cl72c3yo), read back and compared through serde_json with no `preserve_order`, writing nothing (security-plan/dependency-security fct-3t4xl422).
@binding n=3
**api-security answer bounds ↔ data-protection kept answers**: the coverage answer and the replayed `actions` answer stay under the 1,048,576-byte answer bound on the socket (security-plan/api-security fct-t6pddbhw), unpaged as the `graph` and `actions` rows state (security-plan/api-security fct-s6ul72yw) (security-plan/api-security fct-hbfh5ibd), and a kept answer is recorded whole and masked (security-plan/data-protection fct-cl72c3yo).
@binding n=4
**secret-management pin search ↔ api-security verb count**: moving the verb count in docs/capabilities.md and test_capability_statement.py (security-plan/api-security fct-svo6bgmr) keeps both at zero host-path, key and e-mail hits and the pin reading no environment variable (security-plan/secret-management fct-yffuq3co).
@binding n=5
**error-handling buffered render ↔ the body**: the reached-handler record in dioxus_document.rs keeps the single-borrow buffered render (security-plan/error-handling fct-7ni72vcj) beside a dispatch that stays inert where no state is registered (security-plan fct-frlob5k5).
