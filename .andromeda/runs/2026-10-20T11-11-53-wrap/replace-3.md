- line: .andromeda/security-plan.md:83
old: «the seven asked of the held instance (`snapshot`, `click`,»
new: «the seven asked of the held instance (`snapshot` — one optional `part`, its result `text` and, when the screen is read in parts, the counts `part` and `parts`; a screen over the snapshot budget read from part 1 with no `part`, a part past the last the empty text —, `click`,»
- line: .andromeda/security-plan.md:83
old: «and six argument kinds, each with its bound»
new: «and seven argument kinds, each with its bound»
- line: .andromeda/security-plan.md:83
old: «and the sixth, `name`, 1 to 32 bytes of `a-z`, `0-9` and `-` (eight arguments across the table)»
new: «the sixth, `name`, 1 to 32 bytes of `a-z`, `0-9` and `-`, and the seventh, `part`, a whole number from 1 to 1000 (`MAX_PART`) taken through `u32::try_from` (ten arguments across the table)»
- line: .andromeda/security-plan.md:83
old: «colour or control value), and the crate logs»
new: «colour or control value) and a screen's part (`Outcome::Part { text, part, parts }`, its text the snapshot text's own lines, written by `to_json` alone as `{"text":…,"part":n,"parts":m}`), and the crate logs»
- line: .andromeda/security-plan.md:83
old: «the screen text or its length, a shot's image,»
new: «the screen text or its length, a part's number or the part count, a shot's image,»
- line: .andromeda/security-plan.md:83
old: «(a `screenshot` or a `lint` records `verb` and, refused, `cause`)»
new: «(a `screenshot`, a `lint` or a part read records `verb` and, refused, `cause`)»
- line: .andromeda/security-plan.md:129
old: «a `snapshot` result is `to_text`'s output and an acting call's result its JSON line,»
new: «a whole-screen `snapshot` result is `to_text`'s output, a paged one its JSON line `{"text","part","parts"}` holding the same lines, and an acting call's result its JSON line,»
- line: .andromeda/security-plan.md:125
old: «an accepted `snapshot` the screen text itself, any other accepted call its JSON line»
new: «an accepted `snapshot` the screen text itself — a paged one its JSON line `{"text","part","parts"}` holding the same text —, any other accepted call its JSON line»
- line: .andromeda/security-plan.md:410
old: «`host_log` and `mcp_log` each driving one `lint` with 0 needles, so a lint's findings reach no log either»
new: «`host_log` and `mcp_log` each driving one `lint` and one `snapshot` part read with 0 needles, so a lint's findings and a part's number, count and text reach no log either»
