proposals:
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Driver command schema (escher-driver) row
    change: 'Restate the closed table as eleven verbs in three levels: seven asked of the held instance, `screenshot` the seventh after `scroll` (`Level::Instance`; one optional `id` of `ArgKind::Id` under its 1-to-1024-byte bound, an extra argument or a value of another kind refused `malformed`; result fields `width` and `height` (counts) and `png` of the new tenth field kind `FieldKind::Png`, base64 text, all always). This widens the validated surface on the founder''s fork 1 (in-band, «В самом ответе», relayed verbatim by the overseer, inputs#I1 / inputs#I2). The cause set becomes nine, `fonts-unavailable` last (meaning "the app this session holds has no font it can paint text with", remedy "boot the app with a font it can decode"), checked before any paint with nothing painted, so all nine are returned by code: two by `validate`, seven by the executor. An id-addressed shot is refused `stale` or `not-found` like every id-addressed call, and `off-screen` when its crop to the visible part of its bounds holds no pixel (the harness''s `EmptyRect`), each with the instance unchanged. `Outcome` gains `Shot { width, height, png }` (its `Debug` prints the PNG bytes), written only as a command''s answer and as an MCP tool result. The command span keeps its eight fields: a shot records `verb`, plus `cause` when refused, and never the image, its size or its dimensions.'
    sidecar: 'security-plan §Input Validation, Driver command schema row: eleven verbs (`screenshot` the seventh instance verb, optional `id`, `width` · `height` · `png`), nine causes (`fonts-unavailable`), `Outcome::Shot`, the span unchanged (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Symbols and Schema / config list `SCREENSHOT` in `VERBS`, `FieldKind::Png`, `Cause::FontsUnavailable`, `Command::Screenshot { id: Option<String> }`, `Outcome::Shot`, and the deviation that a crop to no pixel is `off-screen`. Coverage records validation on `validate` (optional `id`, `malformed` on a wrong kind or an extra argument) and states that the span never holds the image. Its Counts sweep names security-plan.md:81 under `verbs10`, `inst6` and `causes8`. The row''s "ten verbs", "six asked of the held instance" and "all eight causes" claims are retired.'
    basis: packages/escher-driver/src/schema.rs:415-448
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Driver session socket (escher-driver) row
    change: 'Add that an answer also carries the screen as an image: an accepted `screenshot`''s JSON line `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element''s visible bounds, under the unchanged 1,048,576-byte answer bound (largest whole-screen answer measured 22,404 bytes, the flight booker, 2.1 % of the bound). This is pixels of the screen, a new content class in the ratified carrier, on the founder''s fork 1 («В самом ответе», relayed verbatim by the overseer, inputs#I1 / inputs#I2). A password input in that image paints masked (the password row).'
    sidecar: 'security-plan §Input Validation, socket row: the answer carries a `screenshot`''s base64 PNG in-band under the unchanged answer bound, the founder''s fork 1 (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Schema / config gives the shot''s written line and says the wire''s 1,048,576-byte answer bound is unchanged. Coverage lists `screenshot` reached through the socket''s `call`. Outcome basis cites evidence/counts.md: 22,404 bytes, 2.1 % of the bound. Expected amendments record the in-band image as the founder''s fork 1 boundary widening. The row''s list of what an answer carries (ids, names, values, text, diff) no longer reads whole.'
    basis: packages/escher-driver/src/json.rs:242-248
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → CLI arguments (escher-session) row
    change: 'Add `screenshot` to the command line: an optional `--id`. Accepted, it answers one line `{"width":…,"height":…,"png":"<base64>"}` on stdout with status 0, the image by the ratified in-band crossing (the founder''s fork 1). `--id` on an id the screen does not read exits 1 writing nothing the caller typed. The usage line built from the verb table now names `screenshot`.'
    sidecar: 'security-plan §Input Validation, CLI row: `screenshot [--id]` answers its base64 PNG line on stdout, status 0; a refused `--id` writes nothing typed (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Schema / config gives the shot''s written line. Outcome v010-07 records that the binary answers status 0 with a `png` equal to the in-process PNG. Outcome (security) records that CLI `--id` on an unread id exits 1 writing nothing typed. Expected amendments name the CLI arguments row for `screenshot`.'
    basis: examples/seven_guis/tests/cli_commands.rs:310-353
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → MCP server (escher-driver `serve_mcp`, the `escher-session mcp` role) row
    change: 'Change `tools/list` to the eleven verbs, `help` last. Add that a `screenshot` result is two content blocks: `image` (`data` the base64 PNG, `mimeType` `image/png`), then `text` (the JSON line without `png`), `isError` false. A refused shot stays one text block with `isError` true. Restate the result bound: a result that holds an image is bounded at 1,048,576 bytes by its whole written form, and a text-only result by its text as before (`ToolResult::bounded`). Its `tool_call` span reads `tool=screenshot`. The image crosses inside the ratified MCP carrier on the founder''s fork 1 («В самом ответе», relayed verbatim by the overseer, inputs#I1 / inputs#I2).'
    sidecar: 'security-plan §Input Validation, MCP server row: eleven tools; a shot is an image block then a text block; an image result is bounded by its whole written form (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Schema / config states the two-block written form, the refused shot''s single text block and the new bound rule (mcp.rs `ToolResult::bounded`). Deviations record the MCP bound on an image result. Its Counts sweep names security-plan.md:83 under `verbs10`. The row''s "the ten verbs" and its text-only result bound are retired.'
    basis: packages/escher-driver/src/mcp.rs
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Vector local IPC (the driver session's socket), Trust boundary
    change: 'Add the screen''s pixels to what an answer takes out: since 2026-10-12-headless-screenshot, a `screenshot`''s PNG (base64, in the answer''s JSON line) beside ids, names, values, the snapshot''s text and the diff. This is the founder''s fork 1 («В самом ответе», relayed verbatim by the overseer, inputs#I1 / inputs#I2). A password paints masked in it.'
    sidecar: 'security-plan §Threat Model Summary, socket vector: the answer also carries the screen as a PNG (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Schema / config and Coverage show `screenshot` reached through the socket''s `call` with its image in-band, and Expected amendments name §Threat Model Summary for the pixels in the ratified crossing. The trust-boundary sentence lists the content classes that cross and omits the image.'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Vector local stdio (the driver's MCP server), Trust boundary
    change: 'Add to what a result carries to the client: since 2026-10-12-headless-screenshot, the screen as an MCP `image` block (a PNG, base64), on the founder''s fork 1 («В самом ответе», relayed verbatim by the overseer, inputs#I1 / inputs#I2). What a client does with it, a model provider included, is the client''s. A password input in it paints masked.'
    sidecar: 'security-plan §Threat Model Summary, MCP vector: a result also carries the screen as an image block (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Schema / config describes the MCP image block, and Outcome v010-07 records that MCP answers an image block then a text block. The vector''s trust boundary lists ids, names, values and snapshot text only.'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Markup attributes, password and file `input` value (the snapshot's `value`) row
    change: 'Retire "snapshot-only" for the password. Since 2026-10-12-headless-screenshot the mask also holds on the screen''s pixels: blitz-paint paints an `input` whose `type` reads `password` masked, as bullets, in every document and every renderer through blitz-paint, windowed included (`draw_masked_input_text`, laid out in the document''s own fonts by blitz-dom''s new public `BaseDocument::layout_as_text_input`). This engine change for every document is the founder''s fork 2 («Движок маскирует», relayed verbatim by the overseer, inputs#I1 / inputs#I2). Measured: two values of one length paint byte-identical frames, focused or not, in both layout modes, and a password typed through the driver never leaves as pixels in a `screenshot` (`password_painted_masked`, `stand_act_screenshot`). Unlike the snapshot''s fixed `MASKED_VALUE`, the frame depends on the value''s length. The open remainder is unmeasured: a click inside a password field holding a value places the caret by the real characters'' widths (blitz-dom''s pointer path is unchanged), so which bullet the caret paints at can depend on them, to be owned by a route entry. The snapshot''s mask and the file input''s are unchanged.'
    sidecar: 'security-plan §Input Validation, password row: a password input paints masked in every document (engine, the founder''s fork 2); equal-length values paint identical frames; the click-placed caret index is open (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Symbols list `draw_masked_input_text`, the `password` branch in `draw_text_input_text` and `layout_as_text_input` as reaching every document. Insufficient fixes records the byte-identical frames for equal-length values and the unmeasured click-placed caret. Outcome records "a typed password never leaves as pixels" as met. Expected amendments name the password row ("painted masked, measured on a frame"), and the `pwclear` zero makes it an addition. The row''s "Password- and file-only, and snapshot-only" no longer holds for the paint.'
    basis: packages/blitz-paint/src/render.rs:965-1039
  - detector: D-security-input
    severity: escalate
    section: §API Security (intro paragraph)
    change: 'Change the `escher-session mcp` role''s tools from ten to eleven since 2026-10-12-headless-screenshot, the eleventh `screenshot`. Restate its bound: a tool result is at most 1,048,576 bytes, a result holding an image measured by its whole written form and a text-only result by its text (the Input Validation MCP server row).'
    sidecar: 'security-plan §API Security intro: eleven MCP tools; the image result bound (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Schema / config sets the verb table at eleven and describes the bound rule for an image result. The paragraph''s "ten since 2026-10-12-self-description" and its single result-size bound are retired.'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §API Security → Settle and loads in flight row
    change: 'Replace "none of its six instance-level verbs" with seven. `screenshot` takes one optional `id` and no wait, and its result fields are `width`, `height` and `png`. A shot resolves at the harness''s own time with no tick and no pump, and moves no time. Add a named limit stated in the verb''s help: in a build that paints overlay scrollbars, a thumb painted within 0.7 s of a scroll fades by the engine''s wall clock, so such a shot can differ, owned by a route entry.'
    sidecar: 'security-plan §API Security, settle row: seven instance verbs; a shot waits on nothing and advances no time; the overlay-scrollbar wall-clock limit (2026-10-12-headless-screenshot)'
    rationale: 'Its Counts sweep names security-plan.md:212 under `inst6`. Harness / gate surface says the screenshot resolves "at the harness''s own time (no tick, no pump)". Schema / config states the verb''s help limit on the overlay-scrollbar fade, and the p5-card row 2 places it as a CARRY on "Stand requirement sweep".'
    basis: packages/blitz-test-harness/src/screenshot.rs:37-103
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Error Handling → Error format (typed errors), escher-driver refusal paragraph
    change: 'Close the cause set at nine: `fonts-unavailable` is added last, with its fixed meaning and remedy, holding no path. All nine are returned by code: two by `validate`, seven by the executor. A refused shot over MCP stays one text block with `isError` true.'
    sidecar: 'security-plan §Error Handling: the cause set nine, `fonts-unavailable` added (2026-10-12-headless-screenshot)'
    rationale: 'The report''s Schema / config gives causes eight → nine with `fonts-unavailable` last and its fixed texts. Its Counts sweep names security-plan.md:346 under `causes8`. The paragraph''s "closed at eight" and "all eight are returned by code — two by validate, six by the executor" are retired.'
    basis: packages/escher-driver/src/refusal.rs
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Logging & Monitoring → Log format and backends, escher's own sink bullet
    change: 'Restate the in-process capture of `stand_act_spans` as covering the seven instance-level verbs and a refused call for each of the nine causes (`cause=fonts-unavailable` on the span). Add that a shot''s image reaches no log: the command span never records the image, its size or its dimensions. Measured: 0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host''s and the `mcp` role''s stderr at `RUST_LOG=trace`, under both builds (`host_log`, `mcp_log`).'
    sidecar: 'security-plan §Logging & Monitoring: the span capture at seven verbs and nine causes; 0 image bytes on stderr at trace, host and mcp role, both builds (2026-10-12-headless-screenshot)'
    rationale: 'Its Counts sweep names security-plan.md:392 under `inst6` and `causes8`. Coverage (`screenshot` PII) records 0 occurrences of the first 64 base64 characters in the host''s and the `mcp` role''s stderr at `trace`, both builds. Schema / config states that the span never holds the image, its size or its dimensions. Coverage (`fonts-unavailable`) cites stand_act_spans.'
    basis: examples/seven_guis/tests/host_log.rs:158-171
    dependent-of: D-security-input
note: 'D-security-deps finds no violation: no package enters Cargo.lock, the audit is green, and the plan states no ban list. The report''s Dependencies bullet does add `png` 0.18.1 to escher-driver (its dependencies five), `png` to blitz-tests (dev), and `blitz-paint`, `anyrender` and `anyrender_vello_cpu` to blitz-test-harness. §Dependency Security → Pinning records serde_json''s entry in this shape, and the orchestrator may want a like entry; the plan''s Expected amendments name none. D-security-auth: nothing in the report touches auth, secrets or keys. Proposals 1-9 rest on two boundary widenings, the in-band image (fork 1) and the engine password mask for every document (fork 2). The founder''s recorded word is in inputs#I1 / inputs#I2, so they need the Validate halt card. README.md:48 is stale (the report''s Deviations), outside this doc.'
