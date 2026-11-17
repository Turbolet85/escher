@task_prose part=session
The session split the driver's hand-written answer writer into seven private child modules by answer family: snapshot, diff, caused, layout, graph, actions and lint.
Every carrier's answers stay byte for byte the same. No written key, order, field or byte moved, the 49 literal answer pins still pass, and the stand replay of both committed Counter checks compares every kept answer whole and passes unchanged.
No public symbol, crate re-export, verb, schema, dependency, log site or environment read was added or removed. Moved items are pub(super) at most, and the parent keeps the writer's entry points: string, list, Object, the shot without its image, the session lines and the four to_json methods.
The writer's inline unit tests (9) moved out of line to a path test file under the same json::tests names.
Measured: escher-driver line coverage is 97.68 percent against its 97 percent floor. CI is green 17/17 after two re-runs of an iOS job that no runner took. The span census still reads 2 info_span sites.
@task_file n=1 path=packages/escher-driver/src/json.rs
what: edited. The driver's answer writer, the crate-private module that writes every driver answer by hand as JSON. It now holds the primitives, the screenshot answer, the coverage answer, the outcome dispatch, the refusal JSON, the session error JSON and the session lines.
@task_file n=2 path=packages/escher-driver/src/json/snapshot.rs
what: new, moved out of the answer writer. Writes snapshot nodes: the state readings, the masked control value, the stable id, bounds and the differs flag.
@task_file n=3 path=packages/escher-driver/src/json/diff.rs
what: new, moved out of the answer writer. Writes the acting verbs' answer: settled, busy and the snapshot diff nodes with their keys.
@task_file n=4 path=packages/escher-driver/src/json/caused.rs
what: new, moved out of the answer writer. Writes the caused and caused-past items: errors, logs, owner, event and the cut caused text.
@task_file n=5 path=packages/escher-driver/src/json/layout.rs
what: new, moved out of the answer writer. Writes the layout answer: placement, shift, size reasons, closed layout words and boxes.
@task_file n=6 path=packages/escher-driver/src/json/graph.rs
what: new, moved out of the answer writer. Writes the graph answer: nodes and states, with the declared and confirmed write readings of the signal registry.
@task_file n=7 path=packages/escher-driver/src/json/actions.rs
what: new, moved out of the answer writer. Writes the actions answer nodes: id, owner, keyed and verbs.
@task_file n=8 path=packages/escher-driver/src/json/lint.rs
what: new, moved out of the answer writer. Writes lint findings: their closed reasons and their bounds.
@task_file n=9 path=packages/escher-driver/src/json_tests.rs
what: new, moved out of the answer writer. The answer writer's unit tests (json::tests, 9 tests, 49 literal JSON pins) for every outcome, refusal, session error, session answer and coverage line.
@task_item n=1 part=touches
Any rule that names where driver answers are written (a crate-private module that writes each answer by hand) now covers a parent module and seven private children. The rule still holds because every child is private and pub(super) at most. A row that names a single module as the writer may need its wording checked.
@task_item n=2 part=touches
The answer crossing decisions for actions, graph, caused text, lint findings, coverage and the root-boundary Err: the writer behind each crossing moved into its family module. What crosses is unchanged byte for byte.
@task_item n=3 part=touches
The value mask on every carrier, the masked password and file input values, and the MASKED_VALUE and MASKED_STAND_IN contract items: the snapshot node writer that emits the masked value moved into the snapshot family module. The mask output is unchanged and is still pinned by the tests.
@task_item n=4 part=touches
The snapshot state readings, the snapshot text state words, the control states as closed values and the diff node keys (clickable, differs, invalid, pressed, selected): their writers moved into the snapshot and diff modules. Key names and order are unchanged.
@task_item n=5 part=touches
The actions answer contract items (id, owner, verbs, keyed, nodes), the actions answer being unpaged, and the kept check holding the actions answer: the writer moved into the actions module, and its output is unchanged.
@task_item n=6 part=touches
The graph answer contract (nodes and states), the graph answer being unpaged, and the DeclaredWrite event and target: the writer moved into the graph module, and its output is unchanged.
@task_item n=7 part=touches
The lint findings holding no page content and having closed reasons: the finding writer moved into the lint module, and its output is unchanged.
@task_item n=8 part=touches
The kept check layout answer and the in-process layout read: the layout answer writer and its closed words (placer, shift, cause, keyword) moved into the layout module, and its output is unchanged.
@task_item n=9 part=touches
The stable id exits, no NodeId crossing in replies, and no process-local handle crossing: the node writers that place the stable id moved. They still write no NodeId.
@task_item n=10 part=touches
The refusal JSON (cause, fault, meaning, remedy), the closed refusal cause set, the session error JSON (kind, message, remedy) and session errors from a fixed table: these writers stay in the parent. Their tests moved to the path test file with the same names.
@task_item n=11 part=touches
The screenshot answer (png, carried in band, base64 without a crate): the shot-without-image writer stays in the parent. Only its doc comment now stands on its own item.
@task_item n=12 part=touches
The coverage answer line and the coverage line carrying no user data: the writer stays in the parent, and its test moved with the others.
@task_item n=13 part=touches
Per-crate line coverage floors: escher-driver measures 97.68 percent against its 97 percent floor. The floor value is unchanged.
@task_item n=14 part=touches
Driver outcomes never logged, the driver installing no subscriber and reading no env, the command span holding no content, and screen content staying out of logs: the new modules hold no log site, print or env read, and the info_span count stays 2.
@task_item n=15 part=touches
Kept check answers compared whole and the committed stand Counter checks: the stand replay re-read both checks unchanged, with every answer compared whole.
@task_item n=16 part=touches
The CI matrix running every job: an iOS job that no runner took was cancelled twice and passed on its third run. This was re-run, not fixed, and the gate's rule itself did not change.
@task_item n=1 part=untouched
Dependency security: no crate, lockfile entry, version, licence or audit ignore changed.
@task_item n=2 part=untouched
Outbound network, fetch, CORS, same-origin, iframe, stylesheet and script-loading rows: no network or engine code changed.
@task_item n=3 part=untouched
The session socket transport, its bounds, closed grammar, owner-only access and idle expiry, and the MCP stdio transport, its errors and its bounds: the transport and validation code is unchanged.
@task_item n=4 part=untouched
Recording, kept-check and state directory lifecycle rows (owner-only, deletes only its own files, never named): no file handling changed, and no check file was edited.
@task_item n=5 part=untouched
Select list, password painting, range input, accessibility adapter, aria and engine input handling rows: no engine or DOM code changed.
@task_item n=6 part=untouched
Cold-agent pipe rows (prompt, call allowlist, transcript, recordings): no script or pipe changed.
@task_item n=7 part=untouched
Secret management, CI tokens, publish guards and signing material: no workflow, secret or publish job changed.
@task_item n=8 part=untouched
Authentication and authorization absences, data protection storage (history sqlite, HTTP cache, cookies) and the telemetry sink allowlist: none were reached.
@task_item n=9 part=untouched
The driver verb schema, the argument kinds, the CLI flags, the roles and the exit codes: no verb, argument, flag or exit code moved.
