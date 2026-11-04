@carried n=1 label=security-plan/input-validation fact=fct-7jixhrbw
why: The probe must add no field, carrier, node key or exit to the driver session socket and let no NodeId cross, because the scope's «nothing crosses» rules over exactly what this boundary carries.
quote: which also sets the rule from that day on — a new field of a closed value set inside an already ratified carrier is not a widening and does not come to him, while a field holding authored text, an open value set, a new carrier or a new exit still does (packages/escher-driver/src/schema.rs:39-118; packages/escher-driver/src/json.rs:156-162; packages/escher-driver/src/json.rs:115-134 — the states as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md, `differs` at escher-0.1.0/chunks/2026-10-12-self-description/report.md); no `NodeId` crosses
@carried n=1 run=2 label=security-plan fact=fct-n4q5kc73
quote: Driver session socket (escher-driver) — the widening rule: a new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
@carried n=1 run=3 label=security-plan fact=fct-6hwcr6nz
quote: Driver session socket (escher-driver) — one request line per connection, parsed against a closed grammar under a size bound; anything else is refused with nothing run, and a decoded call runs through `Session::run` alone; one connection at a time under fixed bounds; bounded replies carry ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings, the state readings as closed words and booleans, never an attribute's text, and no `NodeId`.
@carried n=1 run=4 label=security-plan fact=fct-4jl4t3ye
quote: Markup attributes — the stable element id: the HTML `id` as an author key when non-empty, `/`-free and first in document order, else an anchored, component or document path, so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.
@carried n=1 run=5 label=security-plan fact=fct-767qdy6s
quote: Markup attributes — a password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path
@carried n=2 label=security-plan/input-validation fact=fct-awfxp663
why: The probe must add no verb, argument kind, result field or refusal cause to the driver command schema, and any call it makes through a session must go through `validate` inside `Session::run`, because the scope forbids every one of those additions.
quote: `validate` checks a call against a closed `'static` table of twelve verbs in three levels
@carried n=2 run=2 label=security-plan fact=fct-cvyxuuhn
quote: Driver command schema (escher-driver) — `validate` checks every call against a closed `'static` table of twelve verbs and seven bounded argument kinds, returning a typed `Command` or a `Refusal`, the first failure in a fixed order; nothing public runs a `Command` that did not come from `validate`, and the socket, the command line and MCP all pass through it
@carried n=2 run=3 label=security-plan fact=fct-vehorezx
quote: Driver command schema (escher-driver) — no `unwrap`, `expect` or panic on anything a call supplies, no clock read by the schema or its validation, no `NodeId` kept between calls; one `command` span per call holds fixed words, bools and counts only, never a call's content, the screen text, an image or findings; the crate installs no subscriber and reads no env var.
@carried n=3 label=security-plan/input-validation fact=fct-mybm64sc
why: If the probe runs its served task through a driver session, the label it passes must be 1 to 32 bytes of `a-z`, `0-9` and `-`, and must never be text read from the document.
quote: 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs — an invalid label returns `InvalidLabel` with nothing booted; it is never text read from the document, and it is the one caller string a reply carries
@carried n=3 run=2 label=security-plan fact=fct-76426k6u
quote: Driver session label (escher-driver) — 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs; never text read from the document; the one caller string a reply carries.
@carried n=4 label=security-plan/input-validation fact=fct-gucmzkph
why: The probe must add no exit and no argument, so escher-session's closed argv reader and its four fixed endings stay exactly as they are.
quote: four endings — accepted, the answer as one line of JSON on stdout, status 0; refused, the refusal on stdout, status 1; usage, one fixed line on stderr built from the verb table and the app names
@carried n=4 run=2 label=security-plan fact=fct-4nuyglo3
quote: CLI arguments (escher-session) — the argv is read by `escher_driver::command_line`, a closed reader with fixed rules; a call is checked by `validate` or `validate_session` before the session address is asked for; four endings with fixed statuses (0 accepted, 1 refused, 2 usage, 3 session error); no ending writes back anything the caller typed, no path on argv panics, and the command line installs no subscriber and reads no environment variable.
@carried n=5 label=security-plan/input-validation fact=fct-5kk7bggr
why: The probe's headless runs query the stand through the harness, whose query helper panics on an invalid selector where the underlying parse returns an error, so each probe query must pass only a valid, fixed selector.
quote: Parsed with `parse_author_origin_no_namespace` and parse errors are returned (packages/blitz-dom/src/query_selector.rs:204-210)
@carried n=5 run=2 label=security-plan fact=fct-ydvrpbvz
quote: Test harness queries — selector parsing returns a `Result`, while the harness query helper panics on an invalid selector; a text or comment scope matches nothing.
@binding n=1
**input-validation (the socket's widening rule) ↔ logging-monitoring and data-protection (nothing read is printed, logged or fielded)**: what the probe reads (a component name, a handler, a signal's value) must leave by no carrier, field or span. Stable ids and masked control values may leave only by the carriers already ratified, so the probe's test output and the founder's reading must hold no read value and no NodeId, and must add nothing that would count as a widening (security-plan/input-validation fct-7jixhrbw) (security-plan fct-n4q5kc73) (security-plan fct-767qdy6s) (security-plan fct-ggtnnptr).
@binding n=2
**input-validation (the command schema) ↔ logging-monitoring (the command span)**: every probe call through a session must go through `validate` inside `Session::run`, and must run inside a `command` span whose eight fixed fields stay unchanged, so the probe adds no verb, argument kind or cause and no span field, and the sink's allowlist lists stay as they are (security-plan/input-validation fct-awfxp663) (security-plan fct-vehorezx) (security-plan fct-ggtnnptr).
@binding n=3
**input-validation (the session label and state directory) ↔ secret-management (environment reads)**: a served task run through the driver session must take its state directory as a parameter under the registered test-time `CARGO_TARGET_TMPDIR`, never from an env read or a cwd-relative default, and must take a label from a fixed literal inside its closed bound (security-plan/input-validation fct-mybm64sc) (security-plan fct-dx3vh2da) (security-plan fct-pz4e3fuz).
@binding n=4
**input-validation (selectors) ↔ error-handling (query results and panic paths)**: probe queries must use valid fixed selectors so that the harness helper's panic is never reached, and must treat a missing element as the query's `None`, never as an unwrap (security-plan/input-validation fct-5kk7bggr) (security-plan fct-ydvrpbvz) (security-plan fct-jfol2sty).
