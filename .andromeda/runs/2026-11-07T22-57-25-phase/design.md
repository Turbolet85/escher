@extract_section name=relevance
partial — the chunk paints nothing; design-system binds only the agent-read carrier the graph read answers on (cli/mcp output form) and the select-list row precedent for a sub-target owned by an element
@extract_section name=constraints
- The graph read's answer on `escher-session` stdout must be one line of JSON per command, keyed by the driver schema's own words, and nothing else on stdout (per design-system §Surface: cli › Component Patterns, sec-f5ysaan4, fct-6fb4pz6c / fct-or2earye).
- The new verb's answer must carry no colour, no colour dependency and no ESC byte, and no code may test whether a stream is a terminal (per design-system §Surface: cli › Component Patterns, sec-f5ysaan4).
- The verb's exit status must stay within the ratified class table (`0` accepted · `1` refused · `2` usage · `3` session error) read from the schema's `EXIT_STATUSES`, adding no new exit (per design-system §Surface: cli › Component Patterns, sec-f5ysaan4).
- The verb's `mcp` tool must answer newline-delimited JSON-RPC 2.0 lines only, one line a reply, its tool result's text the JSON line (per design-system §Surface: cli › Component Patterns, sec-f5ysaan4).
- An agent-visible node keyed with an optional owner must keep the select-list row reading: a row of an open list carries its own `bounds` in the snapshot while the highlighted row is in no snapshot and no tree (per design-system §Surface: desktop-native › Tokens (platform-specific), sec-yfvfpbhy, fct-grd6lupt / fct-6br6qi3d).
- A sub-target served by its owner's handler must not change hit-testing: a row of an open select list is answered before the walk and a box clipping the select clips no row (per design-system §Depth Strategy, sec-z7dbidjj, fct-zdoz4c52).
@extract_section name=patterns
- The `escher-session snapshot` / `screenshot` answer shapes (one uncoloured JSON line; mcp image+text blocks) are the template a new verb's written form and MCP tool follow (design-system §Surface: cli › Component Patterns, sec-f5ysaan4).
- Open select-list rows are today's only agent-visible nodes that are not DOM elements of their own — owned by the select, with own bounds — the nearest precedent for "agent-visible node with an optional owner element" (design-system §Surface: desktop-native › Tokens (platform-specific), sec-yfvfpbhy); whether the code models them so is research's question.
- The seven_guis checks' shared reader asserts no-ESC on every answer line and the shared `Speaker` on every mcp reply; a new verb's lines pass through them (design-system §Surface: cli › Component Patterns, sec-f5ysaan4).
@extract_section name=anti_patterns
- No colour, ESC byte or terminal test on any answer line or mcp reply of the graph read (design-system §Surface: cli › Component Patterns, sec-f5ysaan4); design-system §Anti-Patterns reads NO RECORDED INTENT and is not cited.
- No second stdout line or stderr chatter on an accepted answer (design-system §Surface: cli › Component Patterns, sec-f5ysaan4).
@extract_section name=bindings
- design-system §Surface: cli ↔ security §Surfaces: a new verb, outcome or open value set on a ratified carrier is a boundary widening, the founder's fork; the carrier form here is design's, the widening call security's.
- design-system §Surface: desktop-native › Tokens ↔ a11y/snapshot: the "reacts to a click" boolean and owner-keyed nodes sit in the ratified snapshot carrier beside the select rows' `bounds` reading.
- design-system §Surface: cli ↔ tests harness: the shared no-ESC reader and `Speaker` assertions cover the new verb.
@extract_section name=acceptance
- The graph verb's CLI answer is exactly one stdout line of valid JSON with no ESC byte, status `0` when accepted, and the same piped or not (per design-system §Surface: cli › Component Patterns).
- The graph verb's mcp reply is one JSON-RPC 2.0 line with no ESC byte, asserted by the shared `Speaker` (per design-system §Surface: cli › Component Patterns).
- After the change, an open select list's row still carries its own `bounds` in the snapshot, the highlighted row still in no snapshot and no tree (per design-system §Surface: desktop-native › Tokens (platform-specific)).
