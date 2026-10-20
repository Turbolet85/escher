
## 2026-10-20-large-screen-in-parts — `snapshot`'s `part` and the paged answer on every carrier
**Section:** §Input Validation → Driver command schema · CLI arguments (`escher-session`) · Markup attributes (the states row · the `id` row) · §Logging & Monitoring
**Change:**
- Driver command schema: seven argument kinds (was six); the seventh, `part`, a whole number 1 to 1000 (`MAX_PART`) taken through `u32::try_from`. Ten arguments across the table (was «eight», already nine at the chunk's base). `snapshot` takes one optional `part`; its result `text`, and the counts `part` and `parts` when the screen is read in parts; a screen over the budget answers part 1 with no `part`, and a part past the last the empty text.
- An `Outcome` holds and prints a screen's part (`Outcome::Part { text, part, parts }`), written by `to_json` alone as `{"text":…,"part":n,"parts":m}`. A part read records `verb` and, refused, `cause`; a part's number and the part count are never fielded.
- CLI arguments: `--part` joins `--id`, `--text`, `--key` and `--ms` as a flag with one value.
- States row and `id` row: a whole-screen MCP `snapshot` result is the screen text itself, a paged one its JSON line holding the same lines (was the screen text alone).
- Logging & Monitoring: `host_log` and `mcp_log` each drive one `lint` and one part read with 0 needles, so a part's number, count and text reach no log.
**Why:** the founder's own ratification of a boundary widening of this validated surface onto every ratified carrier: F1 «Аргумент part, части по умолчанию (Рекомендую)» and F2 «Пустая часть (Рекомендую)», given by question dialog in the overseer session and relayed verbatim by the overseer (2026-10-20, chunk inputs#I2); his card confirm «Подтвердить все (Рекомендую)» (inputs#I5). The table's argument count is measured over `schema.rs`.
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/
