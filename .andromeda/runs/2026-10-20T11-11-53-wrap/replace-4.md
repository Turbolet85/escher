- line: .andromeda/design-system.md:398
old: «by a search of the crate); since 2026-10-10-mcp-surface its `mcp` role»
new: «by a search of the crate); `escher-session snapshot [--part <part>]` answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`; since 2026-10-10-mcp-surface its `mcp` role»
- line: .andromeda/design-system.md:398
old: «a tool result's text the screen text for a `snapshot`,»
new: «a tool result's text the screen text for a `snapshot` (its JSON line `{"text","part","parts"}` for a screen read in parts),»
- line: .andromeda/layout-templates.md:74
old: «(`--id`, `--text`, `--key`, `--ms` each followed by one value, `--shift` bare), the seventh `screenshot`»
new: «(`--id`, `--text`, `--key`, `--ms`, `--part` each followed by one value, `--shift` bare), `snapshot` taking an optional `--part` (a whole number, 1 to 1000) and answering one uncoloured line on stdout, exit 0 — `{"text":…}` for a whole screen, `{"text":…,"part":n,"parts":m}` for a screen read in parts, from part 1 without `--part`, a part past the last the empty text with the same `parts`, a `--part` outside its bound refused `malformed`, exit 1 —, the seventh `screenshot`»
- line: .andromeda/layout-templates.md:74
old: «the usage line listing `help` after `stop`;»
new: «the usage line opening `snapshot [--part <part>]` and listing `help` after `stop`;»
- line: .andromeda/layout-templates.md:74
old: «an accepted `lint` answering its JSON line of findings as one text block,»
new: «an accepted `lint` answering its JSON line of findings as one text block, a paged `snapshot` its JSON line `{"text","part","parts"}` as one text block (a whole screen still the bare screen text),»
