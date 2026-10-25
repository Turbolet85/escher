# security · cascade read · group 4 of 4

## Carried
- (.andromeda/security-plan-detail/data-protection.md:8) The statement's account of session recording and replay must state a kept check's content at rest as it stands — call content, masked answers, the screens' readings, never a pixel, never logged — and owner-only.
  > holding call content (the target ids, `press` keys, `advance` ms and text typed into unmasked controls; a masked or unresolved `type`'s text as `MASKED_STAND_IN`) and the screens' readings at rest — ids, names, values, snapshot text and diffs, never a pixel — never logged, and never removed by escher but for its own check when a recording ends
  (.andromeda/security-plan.md:185)
  > under `--record <dir>` on `escher-session` or `Session::recording_into`, one `check-{n}.json` (`0600`) per recorded session in a `0700` directory the launcher named — per call its verb, arguments, acceptance, digest and masked answer, never a pixel; owner-only, except the two committed checks (below); never logged, never removed by escher but for its own check when a recording ends.
- (.andromeda/security-plan-detail/data-protection.md:14) The statement may point to the two committed Counter checks as the public example of a kept check, and must state that a check of any other session stays owner-only and that a kept check holds no `screenshot` step where bytes are compared across OSes.
  > holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls — no typed text, host path, name or secret (both read by the operator, 2026-10-19) and no `screenshot` step (a shot's PNG bytes differ by OS, as measured at CI#39225375577)
  (.andromeda/security-plan.md:187)
  > a check of any other session stays owner-only and is never committed.
- (.andromeda/security-plan-detail/api-security.md:8) README's MCP-role line and the statement must name twelve verbs served as MCP tools over the role's own stdio, with no listener, port or socket.
  > Since 2026-10-10-mcp-surface the `escher-session mcp` role also serves the driver's verbs as MCP tools — twelve: the tenth, `help` (since 2026-10-12-self-description), answered by the server itself from the schema's fixed tables and running nothing on a session, the eleventh, `screenshot`, and the twelfth, `lint` — over its own stdin and stdout — no listener, port or socket, so the session socket stays the one listener
  (.andromeda/security-plan.md:209)
  > the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results
- (.andromeda/security-plan-detail/api-security.md:73) The statement's socket `call` carrier must be described as reachable by the owning user only, with no authentication beyond the file modes.
  > Reachable by the owning user only (state directory `0700`, socket file `0600`, a directory open to its group or to others refused); no handshake secret, token or peer-credential check
  (.andromeda/security-plan.md:219)
  > Local session socket: reachable by the owning user only (`0700` directory, `0600` socket, a group- or other-open directory refused), no handshake secret, token or peer-credential check
- (.andromeda/security-plan-detail/api-security.md:30) The statement's load wait must state its bound — `LOAD_WAIT`, 5 s of wall time, no time moved, a load still in flight answered `settled: false` — on the five acting verbs, the three reading verbs waiting on none, and that the absent request timeout is what the bound answers.
  > the driver's acting step over it (`Session::act` and the executor's settled step, so `click` · `type` · `press` · `advance` · `scroll` on every carrier) settles again every 10 ms while the answer is `Loads`, for at most `LOAD_WAIT`, 5 s of wall time under the 30 s answer read below, moving no time on the held instance, then returns the load still in flight as `settled: false` with its busy class
  (.andromeda/security-plan-detail/api-security.md:31)
  > an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot`, `screenshot` and `lint` wait on none
  (.andromeda/security-plan-detail/api-security.md:27)
  > Observed absent — searched: `timeout` over the 16 listed s08 files
  (.andromeda/security-plan.md:213)
  > Settle and loads in flight: a harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`, so the absent request timeout cannot hang a settle or a command; no verb has an argument or a result field of a wait.
- (.andromeda/security-plan-detail/secret-management.md:14) The statement's cold-agent instance must describe the model provider reached through the operator's own login held outside the repository, and the tracked statement must hold no key, auth header, credential-file name, token field or e-mail address.
  > the cold-agent pipe's `claude -p` session reaches the model provider through the operator's own Claude Code claude.ai login, held by the `claude` CLI outside the repository — no API key
  (.andromeda/security-plan-detail/secret-management.md:15)
  > the committed live evidence of both runs holds no key, auth header, credential-file name, token field or e-mail address
  (.andromeda/security-plan.md:290)
  > both reach the model provider through the operator's own Claude Code login, held by the `claude` CLI outside the repository — no API key, no env read by the pipe, no CI secret, no new secret source and no auth library; the committed live evidence holds no key, auth header, credential-file name, token field or e-mail address.
- (.andromeda/security-plan-detail/secret-management.md:21) A claim in the statement that escher's source reads no secret must rest on these recorded slice searches as they stand, and the statement, a tracked file, must add no credential.
  > searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
  (.andromeda/security-plan-detail/secret-management.md:22)
  > searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 s03 files
  (.andromeda/security-plan-detail/secret-management.md:23)
  > searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 s04 files; only the formula `Token` enum in cells.rs matched
  (.andromeda/security-plan-detail/secret-management.md:24)
  > searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files; the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:519)
  (.andromeda/security-plan-detail/secret-management.md:25)
  > searched: `env::var|std::env|api_key` over the 17 listed s06 files
  (.andromeda/security-plan-detail/secret-management.md:26)
  > searched: `env::var|secret|token` over the 8 s07 files
  (.andromeda/security-plan-detail/secret-management.md:27)
  > searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (no credential match)
  (.andromeda/security-plan-detail/secret-management.md:28)
  > searched: `std::env|env!\(|getenv` and `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
  (.andromeda/security-plan-detail/secret-management.md:30)
  > searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files
  (.andromeda/security-plan-detail/secret-management.md:31)
  > searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
  (.andromeda/security-plan-detail/secret-management.md:32)
  > searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files
  (.andromeda/security-plan.md:292)
  > secret reads in source are observed absent in every slice that searched:
- (.andromeda/security-plan-detail/error-handling.md:9) The statement's stated limits must hold the three measured-off readings and the boolean model's two limits as the refusal meanings and help texts state them, and its refusal account must give the cause set at eight.
  > a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows (measured by `stand_act_obstructed::a_target_beside_a_box_clipped_on_one_axis_is_refused_covered`, both layout modes, the instance unchanged; tests/blitz-tests/tests/stand_act_obstructed.rs:560-605), and a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it (tests/blitz-tests/tests/stand_act_ids.rs:60) — and `off-screen`'s that a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows (tests/blitz-tests/tests/transformed_client_rect.rs:183-220), its remedy, the cause set (nine) and the cause names unchanged, a fixed `'static` text served by `describe()` and in every `covered` refusal's JSON line — the program's own fixed text inside the ratified carriers, no widening —, the help of `changed` and of `snapshot`'s `text`, restated at 2026-10-10-snapshot-states, name the state words and say the two limits of the boolean model (a mixed toggle reads unpressed, `pressed` false; a grammar or spelling error reads invalid)
  (.andromeda/security-plan-detail/error-handling.md:8)
  > the cause set is closed at eight (ratified by the founder, 2026-10-18) — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen` and, since 2026-10-12-headless-screenshot, `fonts-unavailable` — each with a fixed non-empty `&'static str` name, meaning and remedy that holds no path
  (.andromeda/security-plan.md:343)
  > escher-driver's command refusal is a typed value, `Refusal`: a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and a `Fault` only beside `malformed`; no `String` field, so a refusal cannot hold anything a call supplied; a `SessionError` is written from a fixed table likewise; MCP protocol errors are fixed codes and messages with no `data`; of a refusal only its cause's fixed name reaches a log.
- (.andromeda/security-plan-detail/logging-monitoring.md:8) The statement must state that what escher logs holds no content — a lint's findings, a part and a shot's image reaching no log — and state the sink's reach and the unmeasured paged read through the host's sink as limits.
  > an event from any other target is not printed at all — dropped whole, zero bytes, at every level, WARN and ERROR included, and whatever `RUST_LOG` names
  (.andromeda/security-plan-detail/logging-monitoring.md:10)
  > Its reach is that sink only: the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed, the chained std panic hook still prints the raw panic message to stderr
  (.andromeda/security-plan-detail/logging-monitoring.md:13)
  > so a lint's findings and a part's number, count and text reach no log either — the command span never records the image, its size or its dimensions
  (.andromeda/security-plan-detail/logging-monitoring.md:14)
  > while through the `escher-session` host's own sink it is unmeasured, the host booting only the four lean tasks and none paging, its measurement owned by a `.andromeda/residuals.md` line (target: next)
  (.andromeda/security-plan.md:396)
  > its reach is that sink only: upstream's `fmt::init()` subscribers, `env_logger` and the panic hook stay unscrubbed, and an allowlisted `log.file` can still carry a host path.
- (.andromeda/security-plan-detail/logging-monitoring.md:15) The statement's command-line and MCP carriers must be described as stdout carrying the answer alone — one JSON line per command, JSON-RPC lines in the `mcp` role — and nothing the caller typed written back.
  > `escher-session`, since 2026-10-10-driver-cli, writes one uncoloured line of JSON to stdout per command — the answer (status 0), the refusal (status 1) or the session-error object (status 3) — its usage line (status 2, stdout empty) and a session error's fixed message going to stderr, its host role writing nothing on its quiet end, and since 2026-10-10-mcp-surface its `mcp` role writing JSON-RPC protocol lines only on stdout
  (.andromeda/security-plan.md:398)
  > `escher-session` writes one uncoloured JSON line per command on stdout, its `mcp` role JSON-RPC lines only, usage and error messages on stderr, and no line holds anything the caller typed but the `mcp` role's echoed request `id`
- (.andromeda/security-plan-detail/logging-monitoring.md:21) The statement's measured instance of onward carriage must state that the cold-agent client's raw stream holds the stand's screen beyond escher's scrub, never printed, each committed copy host-path-masked.
  > `target/cold-agent/transcript.jsonl` is the session's raw stream — model output, tool I/O (since 2026-10-15-cold-agent-test the stand's screen), host paths, the client's local socket path and a rate-limit line — unscrubbed by design, never printed, gitignored under `target/`; each committed copy (the live runs' evidence) has its host paths masked — the session dir and the home prefix
  (.andromeda/security-plan.md:403)
  > its server log, check and raw transcript are never printed, the transcript unscrubbed by design and gitignored, each committed copy's host paths masked.

## Bindings
- **api-security (the twelve MCP tools and the socket) ↔ logging-monitoring (stdout carries the answer)**: the carriers the statement lists — socket `call`, command line, MCP tools — are the ones whose stdout or reply carries the answer and nothing the caller typed, and README's verb count must read twelve on both sides (.andromeda/security-plan-detail/api-security.md:8) (.andromeda/security-plan-detail/logging-monitoring.md:15)
- **data-protection (kept checks at rest) ↔ logging-monitoring (never logged)**: a kept check holds the screens' readings at rest while no finding, part or image reaches a log; the statement states both halves together (.andromeda/security-plan-detail/data-protection.md:8) (.andromeda/security-plan-detail/logging-monitoring.md:13)
- **data-protection (committed checks hold no shot) ↔ error-handling (stated limits)**: the PNG bytes differing by OS is the reason the committed checks hold no `screenshot` step and one of the limits the statement gathers beside the refusal texts' measured-off readings (.andromeda/security-plan-detail/data-protection.md:14) (.andromeda/security-plan-detail/error-handling.md:9)
- **secret-management (the operator's login) ↔ logging-monitoring (the cold-agent transcript)**: the onward-carriage instance reaches the model provider through a login held outside the repository, and its raw stream carries the stand's screen beyond escher's boundary; the statement names both and holds no credential (.andromeda/security-plan-detail/secret-management.md:14) (.andromeda/security-plan-detail/logging-monitoring.md:21)
- **api-security (the load wait) ↔ api-security (the absent request timeout)**: `LOAD_WAIT` is the bound that answers the absent request timeout, so the statement states the wait with its bound (.andromeda/security-plan-detail/api-security.md:30) (.andromeda/security-plan-detail/api-security.md:27)
- **error-handling (cause set at eight) ↔ error-handling (the measured-off readings' line)**: the refusal line that states the three measured-off readings also reads «the cause set (nine)» where the line above it closes the set at eight; the statement gives eight, and the disagreement is a finding for the wrap (.andromeda/security-plan-detail/error-handling.md:8) (.andromeda/security-plan-detail/error-handling.md:9)
