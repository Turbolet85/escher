# design extract

## Relevance
partial — no rendered surface, token, colour, type, spacing or motion is touched; the one design binding is the agent-read `cli` surface (`escher-session` and its `mcp` role), whose output contract lives in two of the files this chunk splits (`packages/escher-driver/src/mcp.rs`, `packages/escher-driver/src/schema.rs`) and must read the same after the split.

## Constraints
- design-system §Surface: cli · Component Patterns requires every `escher-session` answer to be one uncoloured line of JSON on stdout keyed by the driver schema's own words, with no ESC byte and nothing else on stdout; the split of `schema.rs` and `mcp.rs` must leave that output byte-identical.
- design-system §Surface: cli · Component Patterns requires the exit status to carry the class (`0` accepted · `1` refused · `2` usage · `3` session error) read from the schema's `EXIT_STATUSES` table, whose meanings `help` serves; moving that table into a submodule must not change its rows or their order.
- design-system §Surface: cli · Component Patterns requires the `mcp` role's stdout to hold newline-delimited JSON-RPC 2.0 protocol lines only — one line per reply, a `snapshot` result the screen text, `help` the self-description's line, an accepted `screenshot` an `image` block (`image/png`, base64) then a text block — uncoloured, no ESC byte; the split along `mcp.rs`'s seams (framing and bounds · tool listing · tool-result writing) must keep each reply's form and block order.
- design-system §Surface: cli · Component Patterns requires no code in the CLI surface to ask whether a stream is a terminal and no colour dependency; a split must add neither (whether the crate already reads 0 on that search is research's question to re-measure).
- design-system §Surface: cli · Tokens (platform-specific) assigns colour tokens (`owo-colors`) to the WPT runner only; none extends to escher-driver, so no colour token may enter the split files.

## Patterns to follow
- The no-ESC reading is asserted on every answer line by the seven_guis checks' shared reader and on every MCP reply by the shared `Speaker`, per design-system §Surface: cli · Component Patterns — keep those checks green, unchanged, as the design witness for the split.
- The CLI surface's fixed texts (usage line, session-error message and `remedy`, `help`'s self-description) are the schema's `'static` table text, per design-system §Surface: cli · Component Patterns — move the tables whole, never re-wrap or re-word their strings when cutting `schema.rs` from its help texts.

## Anti-patterns to avoid
- Adding a terminal test, colour crate or ESC-bearing output while relocating code, which design-system §Surface: cli · Component Patterns rules out for `escher-session` and its `mcp` role.

## Contract bindings
- design ↔ tests: the cli surface's no-ESC and one-line readings bind to the seven_guis command-line checks' shared reader and the MCP `Speaker` (`examples/seven_guis/tests/common/mod.rs`), per design-system §Surface: cli · Component Patterns.
- design ↔ docs reconcile: design-system §Surface: cli · Component Patterns cites `packages/escher-driver/src/schema.rs:577-615` and `packages/escher-driver/src/mcp.rs:460-535` by line; the split moves those spans, so the wrap's reconcile re-points both citations to their new files.

## Acceptance criteria contributions
- (design) Every `escher-session` answer line and every `escher-session mcp` reply after the split is one uncoloured line holding no ESC byte, the existing shared-reader and `Speaker` assertions green unchanged (per design-system §Surface: cli · Component Patterns).
- (design) `escher-session help`'s self-description line, including the `EXIT_STATUSES` meanings, reads byte-identical before and after the `schema.rs` split (per design-system §Surface: cli · Component Patterns).
- (design) A search of escher-driver for a terminal test or colour dependency reads 0 after the split, as before (per design-system §Surface: cli · Component Patterns).
