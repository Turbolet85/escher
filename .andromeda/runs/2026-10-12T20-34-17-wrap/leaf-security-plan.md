group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 31 · «_This section is owned by `/andromeda-wrap-session`. setup-p»

- leaf: .claude/docs/security-summary.md:20   class: omission
  leaf-text: «a call brings an element id and typed text in; no `NodeId` crosses — a second widening»
  source: .andromeda/security-plan.md:25 «Since 2026-10-12-headless-screenshot an answer also takes the screen's pixels out: a `screenshot`'s PNG, as base64 inside its JSON line, the whole screen or one element's visible part»
  replace-with: «since 2026-10-12-headless-screenshot an answer also takes the screen's pixels out — an accepted `screenshot`'s PNG as base64 inside its JSON line, the whole screen or one element's visible part, a password input in it painted masked, under the unchanged 1,048,576-byte bound (the in-band image the founder chose, 2026-10-12, «В самом ответе»); a call brings an element id and typed text in; no `NodeId` crosses — a second widening»

- leaf: .claude/docs/security-summary.md:21   class: omission
  leaf-text: «since 2026-10-10-mcp-surface `mcp` alone selects the MCP role»
  source: .andromeda/security-plan.md:82 «since 2026-10-12-headless-screenshot `screenshot` with an optional `--id` answers one line `{"width":…,"height":…,"png":"<base64>"}` on stdout, status 0»
  replace-with: «since 2026-10-12-headless-screenshot `screenshot` with an optional `--id` answers one line `{"width":…,"height":…,"png":"<base64>"}` on stdout, status 0 — the image by the ratified in-band crossing — and `--id` on an id the screen does not read exits 1 writing nothing the caller typed; since 2026-10-10-mcp-surface `mcp` alone selects the MCP role»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «a result at most 1,048,576 bytes, carrying ids, names, masked values and the snapshot text by the crossing,»
  source: .andromeda/security-plan.md:83 «since 2026-10-12-headless-screenshot, the screen as an image — an accepted `screenshot` two content blocks, an `image` block (`data` the base64 PNG, `mimeType` `image/png`) then a text block»
  replace-with: «a result at most 1,048,576 bytes (since 2026-10-12-headless-screenshot one that holds an image bounded by its whole written form, a text-only one by its text), carrying ids, names, masked values and the snapshot text by the crossing, since 2026-10-12-headless-screenshot the screen as an image — an accepted `screenshot` an `image` block (the base64 PNG, `image/png`) then a text block, the shot's line without `png`, a password input in it painted masked — the pixels in band, ratified by the founder (2026-10-12, «В самом ответе»), what a client does with it, a model provider included, being the client's,»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «a closed table of ten verbs in three levels — six asked of the held instance, `scroll` the sixth, a widening ratified by the founder (2026-10-07),»
  source: .andromeda/security-plan.md:81 «a closed `'static` table of eleven verbs in three levels — the seven asked of the held instance (`snapshot`, `click`, `type`, `press`, `advance`, `scroll`, `screenshot`»
  replace-with: «a closed table of eleven verbs in three levels — seven asked of the held instance, `scroll` the sixth, a widening ratified by the founder (2026-10-07), and `screenshot` the seventh since 2026-10-12-headless-screenshot (one optional `id`; result fields `width` and `height`, counts, and `png`, base64 text — the screen's pixels in band, ratified by the founder, 2026-10-12, «В самом ответе»),»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «a refusal is one of eight causes with fixed texts and holds nothing of the call»
  source: .andromeda/security-plan.md:347 «cause set is closed at nine — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen`, `time-unavailable` and, since 2026-10-12-headless-screenshot, `fonts-unavailable`»
  replace-with: «a refusal is one of nine causes with fixed texts (the ninth, `fonts-unavailable`, since 2026-10-12-headless-screenshot) and holds nothing of the call»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «then `off-screen`, then `covered`.»
  source: .andromeda/security-plan.md:81 «a `screenshot` whose crop to the visible part of its element's bounds holds no pixel is refused `off-screen`, and every refused shot leaves the instance unchanged»
  replace-with: «then `off-screen`, then `covered`; since 2026-10-12-headless-screenshot a `screenshot` of an app with no font it can paint text with is refused `fonts-unavailable` before anything is painted, and one whose crop to the visible part of its element's bounds holds no pixel is refused `off-screen`, every refused shot leaving the instance unchanged.»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «which a sink prints at `info` or below; of a refusal only its cause's fixed name is fielded.»
  source: .andromeda/security-plan.md:81 «a shot's image, its size or its dimensions (a `screenshot` records `verb` and, refused, `cause`)»
  replace-with: «which a sink prints at `info` or below; of a refusal only its cause's fixed name is fielded, and of a shot never its image, its size or its dimensions (a `screenshot` records `verb` and, refused, `cause`).»

- leaf: .claude/docs/security-summary.md:32   class: omission
  leaf-text: «a password's and a file input's value read the fixed mask, while the file input's DOM attribute and form submission still hold the host path»
  source: .andromeda/security-plan.md:126 «since 2026-10-12-headless-screenshot the mask also holds on the screen's pixels: blitz-paint paints an `input` whose `type` reads `password` masked»
  replace-with: «a password's and a file input's value read the fixed mask, while the file input's DOM attribute and form submission still hold the host path; since 2026-10-12-headless-screenshot the mask also holds on the screen's pixels — blitz-paint paints a password `input` masked, a bullet per character, in every document and through every renderer that paints with it (an engine change ratified by the founder, 2026-10-12, «Движок маскирует»), so a password typed through the driver never leaves as pixels in a `screenshot`, though the frame depends on the value's length; a file input's paint is unchanged, and a shot's image reaches no log (its first 64 base64 characters occur 0 times in the `escher-session` host's and the `mcp` role's stderr at `trace`, under both builds)»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «the snapshot's text and the diff out — a second widening, ratified by the founder (2026-10-10);»
  source: .andromeda/security-plan.md:25 «Since 2026-10-12-headless-screenshot an answer also takes the screen's pixels out: a `screenshot`'s PNG, as base64 inside its JSON line, the whole screen or one element's visible part»
  replace-with: «the snapshot's text and the diff out — a second widening, ratified by the founder (2026-10-10) — and, since 2026-10-12-headless-screenshot, the screen's pixels: an accepted `screenshot`'s PNG as base64 inside its JSON line, the whole screen or one element's visible part, a password input in it painted masked, under the unchanged answer bound (the in-band image the founder chose, 2026-10-12);»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «carries the same content — and,»
  source: .andromeda/security-plan.md:28 «Since 2026-10-12-headless-screenshot a result also carries the screen as an MCP `image` block (a PNG, base64), a password input in it painted masked»
  replace-with: «carries the same content — and, since 2026-10-12-headless-screenshot, the screen as an MCP `image` block (a PNG, base64, a password input in it painted masked; the in-band image the founder chose, 2026-10-12), and,»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «a result at most 1,048,576 bytes, every protocol error»
  source: .andromeda/security-plan.md:203 «a 1,048,576-byte tool result (one that holds an image measured by its whole written form, a text-only one by its text)»
  replace-with: «a result at most 1,048,576 bytes (one that holds an image measured by its whole written form, a text-only one by its text), every protocol error»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «a closed verb table of ten verbs in three levels»
  source: .andromeda/security-plan.md:81 «a closed `'static` table of eleven verbs in three levels — the seven asked of the held instance (`snapshot`, `click`, `type`, `press`, `advance`, `scroll`, `screenshot`»
  replace-with: «a closed verb table of eleven verbs in three levels (`screenshot`, the seventh asked of the held instance, since 2026-10-12-headless-screenshot)»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «A refusal is one of eight causes with fixed texts»
  source: .andromeda/security-plan.md:347 «cause set is closed at nine — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen`, `time-unavailable` and, since 2026-10-12-headless-screenshot, `fonts-unavailable`»
  replace-with: «A refusal is one of nine causes with fixed texts (the ninth, `fonts-unavailable`, since 2026-10-12-headless-screenshot)»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «`disabled`, `off-screen` or `covered`, in that order.»
  source: .andromeda/security-plan.md:81 «the ninth, `fonts-unavailable`, since 2026-10-12-headless-screenshot, refusing a `screenshot` of an app with no font it can paint text with before anything is painted»
  replace-with: «`disabled`, `off-screen` or `covered`, in that order; a `screenshot` of an app with no font it can paint text with as `fonts-unavailable` before anything is painted, and one whose crop to the visible part of its element's bounds holds no pixel as `off-screen`, every refused shot leaving the instance unchanged.»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «An `Outcome` holds ids, names and values, prints them under `Debug`»
  source: .andromeda/security-plan.md:81 «and, since 2026-10-12-headless-screenshot, a shot's PNG bytes (`Outcome::Shot`)»
  replace-with: «An `Outcome` holds ids, names and values and, since 2026-10-12-headless-screenshot, a shot's PNG bytes (`Outcome::Shot`), prints them under `Debug`»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «add no field that holds what a call supplied or a screen reads.»
  source: .andromeda/security-plan.md:81 «a shot's image, its size or its dimensions (a `screenshot` records `verb` and, refused, `cause`)»
  replace-with: «add no field that holds what a call supplied or a screen reads — a shot's image, its size or its dimensions included (a `screenshot` records `verb` and, refused, `cause`).»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «whose stdout is the command's answer (ids, names and values by the ratified crossing)»
  source: .andromeda/security-plan.md:82 «a shot's base64 PNG by the ratified crossing (the Driver session socket row)»
  replace-with: «whose stdout is the command's answer (ids, names, values and, since 2026-10-12-headless-screenshot, a shot's base64 PNG by the ratified crossing)»
