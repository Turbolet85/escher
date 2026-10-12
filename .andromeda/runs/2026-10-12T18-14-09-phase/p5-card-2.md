# P5 review — 2026-10-12-headless-screenshot (card 2, after the review's fold)

**Plan:** `escher-0.1.0/chunks/2026-10-12-headless-screenshot/plan.md` (541 lines; the gate fence unchanged since card 1)
· 19 steps · 11 acceptance criteria · 21 gate entries · the whole mechanical set re-run after the fold: clean
(planlint 0 hits; check 2 reads 2 ledger notes, both on verified caps) · validation-1 aligned.

## Folded from the review (the operator, 2026-10-12, at card 1)
- **Two dated ledger notes**, in Expected amendments (wrap) in the ledger-note form, written by wrap P7.3:
  - **matrix#v010-13** — "the ten verbs of `VERBS`" reads "the eleven verbs". The basis is the operator's direction of
    2026-10-07 (later entries extend the one table) and the founder's fork 1 (`inputs#I1`); the rest is unchanged. The
    `ref` pin `mcp.rs::tests::the_listing_is_the_nine_verbs_of_the_table` becomes `…_eleven_verbs_…` (by way of
    `…_ten_…`). `mcp_server.rs::the_mcp_role_completes_the_counter_flow_by_tool_calls_alone` keeps its name, and its
    tool count moves to eleven (`mcp_server.rs:123`).
  - **matrix#v010-14** — "ten verbs" reads "eleven", "eight causes" reads "nine", and "the eight covered together"
    reads "the nine covered together"; the same basis applies, and the rest is unchanged. The `ref` pins keep their
    names. `stand_mcp_tools.rs::every_cause_a_tool_call_reaches_is_served_with_its_remedy` moves from seven to eight
    served. `describe.rs::tests::every_text_in_it_is_its_tables` and
    `cli_commands.rs::help_answers_the_tools_description_and_reaches_no_session` read the larger tables.
- **Step 12** names the count-named unit tests renamed with their counts. The ten-verbs listing and schema tests
  become eleven, and the eight-causes test becomes nine. The span's eight-fields test keeps its name.
- **Step 17 (added work).** v010-14's "nine covered together" holds only if the ninth cause is served. The binary
  cannot boot a fontless stand, so the every-cause MCP check gains a no-font fixture app, served the way `covered` is
  today: a `screenshot` there is refused `fonts-unavailable` with its remedy.
- **A new criterion:** v010-13 and v010-14 stay proved at eleven verbs and nine causes under their renamed or moved pins.

## Goal (unchanged)
- `screenshot`, the eleventh verb, takes the whole screen or one element by id with no display, in process, on the CLI
  and over MCP.
- The PNG travels base64 in the one JSON answer and as an MCP `image` block (founder fork 1, In-band — `inputs#I1`,
  `inputs#I2`).
- A password input paints U+2022 in every document (founder fork 2, Engine masks).
- No usable font is refused `fonts-unavailable`; nothing skips.
- The same stand state gives byte-identical PNGs in separate processes.

## Concretization previews (unchanged from card 1)
- **v010-07**
  - Now: "With no display present, the driver returns an image of the whole stand screen and an image cropped to one
    element addressed by its id."
  - Proposed: each lean task, both modes, accepted as an 800 × 600 PNG equal to the harness's raster, with text painted
    in `#task-title`. `--id crud-list` gives 369 × 420, equal to that rectangle of the whole shot. The same verb answers
    through the binary (status 0, `png` decodes to 800 × 600) and over MCP (image block, then text block).
- **v010-08**
  - Now: "Two screenshots of the same stand state taken in separate runs on the same host are pixel-identical."
  - Proposed: two child processes shoot the four tasks whole, `crud-list`, the counter's value, and a focused date input,
    in both modes. They print byte-identical PNGs, each equal to the parent's in-process shot, under both builds. No
    scroll precedes a shot (the wall-clock scrollbar fade, stated in the verb's help).

## Decisive leans, touchpoints, test commands, warnings
As card 1 (`p5-card-1.md`). The touchpoints are unchanged: `stand_mcp_tools.rs` was already listed. The baselines are
unchanged. Size: 541 lines against the ~400 guide. The fence plus the two ledger notes account for the growth; prose
outside the fence is about 350.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | Our merge surface in the inherited engine files this chunk edits: `packages/blitz-paint/src/render.rs` (the password mask) and `packages/blitz-dom/src/document.rs` (the font reading), plus `packages/blitz-dom/src/node/text.rs` if step 3 touches it | a `CARRY:` on the existing entry "Upstream sync ahead of polish and ship" (working-route.md:86), written at this chunk's wrap |
| 2 | The overlay scrollbar's fade reads the wall clock (`packages/blitz-dom/src/document.rs:1782-1801`), so a shot within 0.7 s of a scroll can differ. Stated as a limit in the verb's help; moving the fade onto the harness clock is an engine change for every document, not asked here | a `CARRY:` on the existing entry "Stand requirement sweep" (working-route.md:96), written at this chunk's wrap |

Apply? (yes / review / cancel)
