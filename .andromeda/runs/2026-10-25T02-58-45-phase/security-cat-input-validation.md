# security · Input Validation — category read

## Carried
- (.andromeda/security-plan.md:67) The statement must describe only the ratified carriers and exits; it must not present a new carrier or exit as a capability, because by this rule that would be a widening, not a statement.
  > the widening rule: a new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
- (.andromeda/security-plan.md:135) Whatever the statement says leaves escher must name only the ratified carriers.
  > they leave escher only by the ratified carriers, the page's own form submission and events aside.
- (.andromeda/security-plan.md:137) The statement's account of stable ids must match this: no collision, no log, the four exits, and no `NodeId` crossing.
  > so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.
- (.andromeda/security-plan-detail/input-validation.md:54) The statement's verb count must be twelve in three levels. This is the value CARRY 1's README fix must match.
  > `validate` checks a call against a closed `'static` table of twelve verbs in three levels
- (.andromeda/security-plan-detail/input-validation.md:55) If the statement states argument bounds as limits, they must match these seven kinds and their bounds.
  > seven argument kinds, each with its bound — `id` 1 to 1024 bytes of text, `text` 0 to 4096 bytes, `key` one name of a closed twelve-name list, `flag` true or false, `milliseconds` a whole number from 1 to 60000, the sixth, `name`, 1 to 32 bytes of `a-z`, `0-9` and `-`, and the seventh, `part`, a whole number from 1 to 1000 (`MAX_PART`)
- (.andromeda/security-plan-detail/input-validation.md:56) The statement's refusal model must give this fixed order and say what `scroll`, `press` and `advance` refuse and what they do not.
  > with one answer, the first that holds in a fixed order — `disabled` (the snapshot reads the element not enabled), then `off-screen` (the centre of its bounds lies outside the viewport, or outside the visible part of a scrolling box that holds it), then `covered` (the hit at that point never reaches it) — and each refused call leaves the snapshot and both focus readings as they were, in both layout modes; `scroll` applies none of the three and says `in_view` in its result instead of refusing; `press` and `advance` are refused for no screen-level cause; all eight causes are returned by code
- (.andromeda/security-plan-detail/input-validation.md:37) The request bound on the socket carrier is a stated limit the statement must match.
  > anything else — a call the decoder does not read and a request line over 16,384 bytes included (was 64) — `refused malformed`, and runs nothing
- (.andromeda/security-plan-detail/input-validation.md:37) CARRY 2's unpaged diff and findings must be stated against this answer bound, the one bound that holds them.
  > an answer is bounded at 1,048,576 bytes on its JSON line
- (.andromeda/security-plan-detail/input-validation.md:38) The statement must say that an answer past the bound is replaced and the call is not undone. This is part of CARRY 2's limit.
  > over it the reply is `ok v3 oversize`, the call having run
- (.andromeda/security-plan-detail/input-validation.md:38) The list of what a driver answer carries out must match these content classes, with the masks as given.
  > carries what the screen reads: ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff, where until that chunk no reply carried any of them — the crossing ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Сокет несёт команды»; the English wording is the overseer's); since 2026-10-12-headless-screenshot an answer also carries the screen as an image — an accepted `screenshot`'s line `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element's visible part, a password input in it painted masked
- (.andromeda/security-plan-detail/input-validation.md:39) The socket carrier's platform limit must be stated.
  > no `NodeId` crosses; unix only, `Unsupported` elsewhere
- (.andromeda/security-plan-detail/input-validation.md:42) The state directory's path-length bound is a limit a consumer will hit, and it must be stated with its per-platform figures.
  > at most 94 bytes as given on Linux (measured: 94 bytes holds a session, 95 ends `state-dir-too-long`, status 3) and 90 on every other target
- (.andromeda/security-plan.md:81) The statement's command-line capability must give these four endings and their statuses.
  > CLI arguments (escher-session) — the argv is read by `escher_driver::command_line`, a closed reader with fixed rules; a call is checked by `validate` or `validate_session` before the session address is asked for; four endings with fixed statuses (0 accepted, 1 refused, 2 usage, 3 session error); no ending writes back anything the caller typed, no path on argv panics
- (.andromeda/security-plan-detail/input-validation.md:69) The MCP carrier must be stated as stdio only, unauthenticated and bounded on requests.
  > newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout — no listener, port, socket, authentication or env read; a request line of at most 65,536 bytes (`MAX_MCP_REQUEST_BYTES`)
- (.andromeda/security-plan-detail/input-validation.md:70) The MCP result bound is a stated limit, and CARRY 2's unpaged diff and findings are held by it over MCP as well.
  > a result's text over 1,048,576 bytes (`wire::MAX_ANSWER_BYTES`) replaced by `AnswerTooLarge`'s result
- (.andromeda/security-plan-detail/input-validation.md:70) The "what an MCP client carries onward" section must list exactly these content classes as what escher hands the client.
  > results carry ids, accessible names, control values (a password's or a file input's as the mask) and the snapshot text by the crossing and, since 2026-10-12-headless-screenshot, the screen as an image
- (.andromeda/security-plan-detail/input-validation.md:204) The statement must say that accessible names are page text, carried to the MCP client inside the name crossing.
  > the computed label reaches `label` and so the snapshot's `name`, and its page text stays inside the ratified name crossing below — the adapter, the driver's answers, MCP results
- (.andromeda/security-plan-detail/input-validation.md:197) What the statement says a lint finding carries onward must match this content set.
  > returns `Finding`s holding ids, closed defect words and numbers only (no `NodeId`, page text, colour or control value; a text control's value never measured)
- (.andromeda/security-plan-detail/input-validation.md:210) The statement must not claim descriptions among what the snapshot or MCP carries.
  > the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries it
- (.andromeda/security-plan-detail/input-validation.md:238) The masking claim in the statement must be exactly this: never the text, never its length, never a file's host path.
  > never the text, never its length and never a file's host path
- (.andromeda/security-plan-detail/input-validation.md:239) The statement must not overclaim the mask. The DOM and form submission still hold a file input's host path.
  > a file input's `value` attribute still holds the chosen file's host path and form submission still carries it (§Data Protection)
- (.andromeda/security-plan-detail/input-validation.md:240) A file input's mask over MCP holds by construction and was not run. The statement must not present it as measured.
  > a file input's over MCP by the same construction, not separately run
- (.andromeda/security-plan-detail/input-validation.md:241) A shot's masked password still reveals the value's length. That is a stated limit of what a client carries onward.
  > unlike `MASKED_VALUE` the frame depends on the value's length
- (.andromeda/security-plan-detail/input-validation.md:242) The statement on recording must say what a kept check holds of a `type`: the masked stand-in, and other text as it crossed.
  > kept in the check as `MASKED_STAND_IN` (`"masked"`) with `masked:true`, never the text, its length or a digest of it, while any other `type`'s text, one refused by `validate` included, is kept as it crossed
- (.andromeda/security-plan-detail/input-validation.md:45) Recording's bounds and its end on failure are limits the statement must give with these figures.
  > a failed write or a passed bound (400 records, 1,048,576 bytes) ends the recording — its check removed, nothing further recorded, `recording_failed()` true once, the call run and not rolled back, answered `recording-failed`
- (.andromeda/security-plan-detail/input-validation.md:48) The statement's replay capability must match this: steps run through `Session::run` alone, nothing runs on another app's session, and replay stops at the first difference.
  > `Check::replay` runs each kept step's call through `Session::run` alone, nothing on a session of another app, and stops at the first step whose acceptance or kept answer differs
- (.andromeda/security-plan-detail/input-validation.md:234) The select's stated limits must be carried into the statement in these words.
  > stated limits: a `multiple` select and an `optgroup`'s options are not modelled, no popup is painted and an option is not clicked (refused `covered`)
- (.andromeda/security-plan-detail/input-validation.md:221) The missing inert focus fixup is a stated limit that the statement's limits must include.
  > a known limit: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next)
- (.andromeda/security-plan-detail/input-validation.md:217) The boolean model's mixed-toggle reading is a stated limit, and the statement must state it as given.
  > `NodeState.selected`, `.pressed` (left out for the five checkable roles) and `.checked` (a native checkbox's or radio's own state, else a checkable role's toggle, a mixed one `Some(false)`), each an `Option<bool>` (only `Toggled::True` reads `Some(true)`;
  > `False` and `Mixed` read `Some(false)`), and `.invalid`, a `bool` (a grammar or spelling error reads invalid)
- (.andromeda/security-plan-detail/input-validation.md:213) `aria-disabled` moves no snapshot reading. This limit must be stated beside the enabled reading.
  > the snapshot's `enabled` keeps reading the `disabled` attribute by presence, so `aria-disabled` moves no snapshot reading
- (.andromeda/security-plan-detail/input-validation.md:216) The statement's a11y reach must state the native checkbox's checked state absent from the tree as a measured absence.
  > its native checked state reaches no tree node either — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next)
- (.andromeda/security-plan-detail/input-validation.md:216) If the statement covers state reading, its limits must include the case rule for ARIA state values, which is a choice and was not measured.
  > the lower-case exact match is a choice — the specification as read states no case rule and no browser was measured
- (.andromeda/security-plan-detail/input-validation.md:217) The statement's a11y reach must say that assistive-technology reading is unmeasured and owned by a residual.
  > what an assistive technology reads of them unmeasured until a windowed check (no stand binary builds a platform adapter and the dev host's AT-SPI bus reads inactive), the witness owned by a `.andromeda/residuals.md` line (target: next)
- (.andromeda/security-plan-detail/input-validation.md:193) The statement must say that the windowed refresh witness is still owed and that only the headless tree is proven.
  > its windowed witness is still owed — no windowed run witnesses the refresh on the dev host
- (.andromeda/security-plan-detail/input-validation.md:225) The range input's departure from the spec is a stated limit the limits list must include.
  > a `value` write re-seeds the current value and a `min`, `max` or `step` write or removal re-sanitizes it — a departure from the spec's dirty-value flag
- (.andromeda/security-plan-detail/input-validation.md:125) The statement must not claim URL loading. No driver verb takes a URL, and the stand boots offline.
  > escher routes no agent- or user-supplied URL through this path today — no driver verb takes a URL, and the stand boots offline on its bundled font
- (.andromeda/security-plan-detail/input-validation.md:207) If the statement says what leaves escher on stderr, it must match this measured absence on the MCP role.
  > the `mcp` role's stderr at `RUST_LOG=trace` holds 0 typed-sentinel occurrences, 0 id needles and 0 name needles under both builds
- (.andromeda/security-plan-detail/input-validation.md:33) The statement names the cold-agent pipe as its measured instance of an MCP client. That instance must match: the agent's calls go through the driver's own MCP server.
  > the agent's tool calls themselves reach the driver's own MCP server — the MCP server row

## Bindings
- **MCP results' content (.andromeda/security-plan-detail/input-validation.md:70) ↔ Threat Model body (.andromeda/security-plan.md:27)**: the content classes the statement lists as handed to the MCP client must be the ones the MCP row carries, and the scope's claim that anything the client forwards is beyond escher's boundary must rest on the vector line "what a client does with them, a model provider included, is the client's" (.andromeda/security-plan.md:27).
- **Unpaged diff and findings (CARRY 2) ↔ answer bounds**: the limit is stated against the socket's answer bound (.andromeda/security-plan-detail/input-validation.md:37) and the MCP result bound (.andromeda/security-plan-detail/input-validation.md:70). The snapshot budget on the other side is architecture's; name both.
- **File input mask ↔ Data Protection**: what the mask does not cover, the DOM attribute and form submission (.andromeda/security-plan-detail/input-validation.md:239), binds with Data Protection's local-user-data line (.andromeda/security-plan.md:195).
- **Kept check content ↔ Data Protection at rest**: recording's bounds and `type` stand-in (.andromeda/security-plan-detail/input-validation.md:45, .andromeda/security-plan-detail/input-validation.md:242) bind with the owner-only and committed-check media (.andromeda/security-plan.md:185, .andromeda/security-plan.md:187).
- **a11y reach ↔ a11y-plan and `.andromeda/residuals.md`**: the unmeasured assistive-technology witness (.andromeda/security-plan-detail/input-validation.md:217), the owed windowed refresh (.andromeda/security-plan-detail/input-validation.md:193), the native checked-state absence (.andromeda/security-plan-detail/input-validation.md:216) and the inert focus fixup (.andromeda/security-plan-detail/input-validation.md:221) are owned by residual lines. The statement's a11y section must cite those same residuals that a11y-plan cites.
- **No log of names, ids or typed text ↔ Logging & Monitoring**: the stderr absence (.andromeda/security-plan-detail/input-validation.md:207) binds with escher's sink line (.andromeda/security-plan.md:396).
- **Refusal model ↔ Error Handling**: the fixed refusal order and eight causes (.andromeda/security-plan-detail/input-validation.md:56) bind with the typed `Refusal` holding nothing of a call (.andromeda/security-plan.md:343).
- **Verb count (CARRY 1) ↔ the body's schema line**: twelve verbs (.andromeda/security-plan-detail/input-validation.md:54) is also the body's figure (.andromeda/security-plan.md:77). README's verb count must match both.
