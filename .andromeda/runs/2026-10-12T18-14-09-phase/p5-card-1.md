# P5 review — 2026-10-12-headless-screenshot

**Plan:** `escher-0.1.0/chunks/2026-10-12-headless-screenshot/plan.md` (498 lines; about 310 outside the gate fence) ·
19 steps · 10 acceptance criteria · 21 gate entries · mechanical checks clean · validation-1 aligned.

## Goal
- `screenshot`, the eleventh verb, takes the whole screen or one element by id with no display, served in process,
  on the CLI and over MCP.
- The image is a PNG, base64 in the one JSON answer and as an MCP `image` block (founder fork 1, In-band — `inputs#I1`,
  `inputs#I2`).
- A password input paints U+2022 in every document (founder fork 2, Engine masks).
- An instance with no usable font is refused `fonts-unavailable`; nothing skips.
- The same stand state gives byte-identical PNGs in separate processes.

## Decisive leans (no fork asked; each argued in the plan)
- The raster lives in `blitz-test-harness`, which owns the document; intent §4 names the harness's gap.
- The driver adds one dependency, `png`, and writes base64 by hand; no package enters `Cargo.lock`.
- An element shot: `stale` / `not-found` as every id verb, `off-screen` by the same centre-in-view reading as `click`,
  never `disabled` / `covered`. The crop is the bounds ∩ visible region, rounded outward: `crud-list` → 369 × 420.
- No usable font becomes a ninth cause, `fonts-unavailable`, because `Session::run` answers only `Outcome` or `Refusal`.
  The reading is a new read-only `BaseDocument` method; no behaviour changes.
- No new span field. The image, its size and its bytes are never fielded.
- The MCP result is the image block, then a text block holding the outcome line without `png`.

## Concretization previews (the never-weaken checkpoint)
- **v010-07**
  - Now: "With no display present, the driver returns an image of the whole stand screen and an image cropped to one
    element addressed by its id."
  - Proposed: on each of the four lean tasks, both layout modes, `screenshot` is accepted as an 800 × 600 PNG. It
    decodes to the harness's own raster, and text paints inside `#task-title`. `screenshot --id crud-list` is 369 × 420
    and equals that rectangle of the whole shot. The same verb answers through the binary (status 0, `png` decodes to
    800 × 600) and over MCP (image block, then text block).
- **v010-08**
  - Now: "Two screenshots of the same stand state taken in separate runs on the same host are pixel-identical."
  - Proposed: two child processes each shoot the four tasks whole, `crud-list`, the counter's value, and the flight
    booker with a focused date input (caret painted), in both modes. They print byte-identical PNGs, each equal to the
    parent's in-process shot, under the per-package build and the workspace build (renderer `multithreading` on). No
    scroll precedes a shot: a scrollbar thumb fades by wall time, a limit the verb's help states.

## Top acceptance criteria (beyond the two caps)
- Engine: equal-length password values paint identical RGBA, while a text input's differ; bullets are drawn and the
  caret paints when focused. The same holds through the driver; the snapshot still reads `MASKED_VALUE`.
- No font → `fonts-unavailable`, nothing run; no skip path anywhere.
- Logs at `trace` hold 0 ids, 0 names, 0 sentinels and 0 image bytes under both builds; one `command` line per call.
- A shot marks no change, moves no focus and leaves an empty diff.

## Touchpoints
- Engine: `blitz-dom/src/document.rs` (font reading), `blitz-paint/src/render.rs` (mask), maybe `blitz-dom/src/node/text.rs`.
- Harness: `screenshot.rs` (new), its manifest and `lib.rs`.
- Driver: `image.rs` (new), schema, refusal, command, execute, json, mcp, cli, describe, client, lib, manifest.
- Checks: `password_painted_masked`, `stand_act_screenshot`, `stand_screenshot_repeat` (new); spans, mcp_tools,
  act_scroll, session_common, the blitz-tests manifest; `cli_commands`, `mcp_server`, `host_log`, `mcp_log`.

## Test commands (baselines read 18:37Z)
- Unit: driver, blitz-dom lib, harness — all green.
- Smoke: `escher-session help` lists the verb and the cause — red, the predicted red.
- New-file integration, per-package and workspace — red: no target yet.
- The standing neighbours and the binary's roles — green.
- Lock, no-panic and write-set probes — green, each with must-fail and must-pass controls on a scratch clone
  (`controls-p5.md`). Audit — green.
- Then `run stand`, `fast`, `doc`, and the operator's hygiene, push and CI read.

## Warnings
- Size: 498 lines against the ~400 guide. Prose outside the fence is about 310, inside the band; the last chunk's plan
  was 542.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | Our merge surface in the inherited engine files this chunk edits: `packages/blitz-paint/src/render.rs` (the password mask) and `packages/blitz-dom/src/document.rs` (the font reading), plus `packages/blitz-dom/src/node/text.rs` if step 3 touches it | a `CARRY:` on the existing entry "Upstream sync ahead of polish and ship" (working-route.md:86), written at this chunk's wrap |
| 2 | The overlay scrollbar's fade reads the wall clock (`packages/blitz-dom/src/document.rs:1782-1801`), so a shot within 0.7 s of a scroll can differ. Stated as a limit in the verb's help; moving the fade onto the harness clock is an engine change for every document, not asked here | a `CARRY:` on the existing entry "Stand requirement sweep" (working-route.md:96), written at this chunk's wrap |

Apply? (yes / review / cancel)
