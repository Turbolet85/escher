# arch · cascade read · group 2 of 6

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:120) The design system's stand DejaVu Sans item and the stand's viewport facts must land in architecture stating exactly the headless stand's pinned viewport and its one bundled font, with no value of their own beside it.
  > **Headless stand (seven_guis, native only):** `seven_guis::stand` — `LeanTask { Counter, FlightBooker, Timer, Crud }` with `ALL` and `task()`; pinned `VIEWPORT_WIDTH = 800`, `VIEWPORT_HEIGHT = 600`, `HIDPI_SCALE = 1.0`, `COLOR_SCHEME = ColorScheme::Light`; `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`, system fonts off;
  (.andromeda/architecture-detail/standard-contracts.md:121)
  > the crate-root `DEJAVU_SANS` woff2 bytes, one `include_bytes!` shared with the wasm entry
  (.andromeda/architecture.md:163)
  > `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font
  (.andromeda/architecture.md:246)
  > bundled DejaVuSans.woff2 — seven_guis' bytes are the crate-root `DEJAVU_SANS`, compiled on native too, where the headless stand registers the same font
- (.andromeda/architecture-detail/standard-contracts.md:128) The accent row moved out of the design system must keep the stand accent the load page's contract states — white on `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1` — and the load page must be counted among the six apps the README and the capabilities statement name for the 2026-10-25 run.
  > `boot_load(HarnessOptions) -> Harness<DioxusDocument>` mounts a load page in `TaskShell` titled "Load"
  (.andromeda/architecture-detail/standard-contracts.md:128)
  > `load-fetch`, a Button "Load" in white on the stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), whose click links the page's one stylesheet
- (.andromeda/architecture-detail/standard-contracts.md:127) The statement's description of the 2026-10-25 run (six apps, twelve tools) and the README's "Try it" must name the stand's served set as the contract states it — the four lean tasks, Cells and the load page — with Cells the screen read in parts, and the README's tool count must stay the schema's twelve verbs.
  > `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts `Task::Cells` in `TaskShell` through the private `cells_root`, Cells staying no `LeanTask`: its screen passes the snapshot text's budget and reads in parts
  (.andromeda/architecture.md:187)
  > `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks — the four lean ones, Cells and the load page, its usage line's apps and the MCP `start` tool's `enum` written from one private table —, with its `serve` host role and its `mcp` role, answering one line of JSON and exiting `0` accepted · `1` refused · `2` usage · `3` session error.
  (.andromeda/architecture.md:167)
  > the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied
  (.andromeda/architecture.md:193)
  > `scripts/agent-run.sh` and `scripts/cold-agent.sh` the agent-run test and cold-agent pipe contracts, in full in test-plan §3.

## Bindings
- **standard-contracts (headless stand) ↔ the design-system plan**: the four stand rows leave `.andromeda/design-system.md` and the accent and DejaVu Sans rows that land in architecture must read the values the stand contract holds, `#3a5ce5` with its hover and active steps and `build_single_font_ctx(DEJAVU_SANS)` at 800x600 Light, so no master states a second value (.andromeda/architecture-detail/standard-contracts.md:128) (.andromeda/architecture-detail/standard-contracts.md:120)
- **standard-contracts (headless stand) ↔ existing-scopes and infrastructure patterns**: the stand's font lands beside the deployment-model line naming the crate-root `DEJAVU_SANS`, and the accent, success and cards rows beside the seven_guis scope, each restating the stand contract rather than adding to it (.andromeda/architecture.md:246) (.andromeda/architecture-detail/standard-contracts.md:121)
- **standard-contracts (stand served set, CLIs, driver schema) ↔ project-intent and the test plan**: the front page's "Try it" and status, and the capabilities statement's 2026-10-25 run, count six apps and twelve tools from the stand and schema contracts, the run itself being the cold-agent pipe whose contract lives in test-plan §3, and the capability-statement pin holds the counts to the schema (.andromeda/architecture.md:187) (.andromeda/architecture.md:167) (.andromeda/architecture.md:193)
