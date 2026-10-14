proposals:
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand census bullet
    change: >-
      The bullet's reader count of the stand checks' shared module reads "21 of which read the shared module
      `tests/blitz-tests/tests/common/mod.rs` — `stand_contrast` the eighteenth, since 2026-10-13-stand-contrast-harness,
      `stand_keyboard` the nineteenth, since 2026-10-13-stand-keyboard-harness, and `stand_requirement_sweep` and
      `stand_non_lean_tasks` the twentieth and twenty-first, since 2026-10-14-stand-requirement-sweep", where it reads
      "19 of which … `stand_keyboard` the nineteenth"; its as-measured list gains "the twentieth and twenty-first
      readers of the shared module at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md".
    sidecar: >-
      obs-plan §3 Logging stack: the stand checks' `common/mod.rs` readers 19 → 21, `stand_requirement_sweep` and
      `stand_non_lean_tasks` joining (2026-10-14-stand-requirement-sweep).
    rationale: >-
      The report's Counts / qualifiers moved → Shared-module readers states "`tests/common/mod.rs` readers go from 19 to
      **21** (both new stand files declare `mod common;`; `password_caret_click` declares none)", its change site
      "obs-plan.md:70 ('19 of which read the shared module')"; Expected amendments carry obs-plan §3 with
      "`common-readers`: obs-plan 2 rows (change :70; :331 no-change)".
    basis: .andromeda/obs-plan.md:70
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand census bullet
    change: >-
      The bullet's `session_common` clause reads "the module twenty-one checks read — four of the five
      `stand_session_*`, `stand_settle`, the thirteen `stand_act_*`, `stand_mcp_tools`, `stand_screenshot_repeat` and,
      since 2026-10-14-stand-requirement-sweep, `stand_requirement_sweep` — `tests/blitz-tests/tests/session_common/mod.rs`",
      where it reads "the module twenty checks read", the dated "(measured at 2026-10-12-headless-screenshot, where
      eighteen stood)" kept.
    sidecar: >-
      obs-plan §3 Logging stack: `session_common/mod.rs` readers 20 → 21, `stand_requirement_sweep` joining
      (2026-10-14-stand-requirement-sweep).
    rationale: >-
      The report's Counts / qualifiers moved → Shared-module readers states "`tests/session_common/mod.rs` readers go
      from 20 to **21** (`stand_requirement_sweep`)". The same obs-plan.md:70 bullet that carries the `common` count
      restates the `session_common` count as "the module twenty checks read", a second occurrence of the reader census
      the primary moves; the report's no-change :331 is a citation into the module, not a count.
    basis: .andromeda/obs-plan.md:70
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand census bullet
    change: >-
      The silent census gains, after the 2026-10-14-stand-a11y-assertions clause: since 2026-10-14-stand-requirement-sweep
      `stand_requirement_sweep` (the driver sweep through `Session::run` on the four lean tasks in both layout modes,
      and its `WITNESSES` table read through `include_str!`), `stand_non_lean_tasks` (the three non-lean tasks and
      Circle Drawer's open dialog, in process in both layout modes) and the engine check `password_caret_click` (its
      own parsed page, declaring no shared module) each install no subscriber, read no env var and print nothing, their
      readings going into assertions only (the implement pass's silence census, gate entry 7, reading 0); the
      extended `stand_act_obstructed`, `hit_clipped_at_scrolling_box`, `scrolled_box_client_rect` and
      `range_input_interaction` stay as named; the as-measured list gains "`stand_requirement_sweep`,
      `stand_non_lean_tasks` and `password_caret_click` at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md".
    sidecar: >-
      obs-plan §3 Logging stack: `stand_requirement_sweep`, `stand_non_lean_tasks` and `password_caret_click` join the
      silent census — no subscriber, no env read, no print (gate entry 7 reads 0) (2026-10-14-stand-requirement-sweep).
    rationale: >-
      The report's Files list three new test targets (`stand_requirement_sweep.rs` 613 lines, `stand_non_lean_tasks.rs`
      147, `password_caret_click.rs` 95); its Expected amendments carry "obs-plan §3 (the census over the three new
      files) — carried. The two new stand files and `password_caret_click` install no subscriber, read no env and print
      nothing (gate entry 7 reads 0)", and note obs-plan states the census as "installs no subscriber … and no
      `println!`" at :70. Outcome → "Met — the silence census. It reads 0 (gate 7)." No telemetry dependency was added
      (Dependencies: none), so the stack matches §3 and only the census misses the three files.
    basis: .andromeda/obs-plan.md:70
  - detector: D-obs-pii
    severity: warning
    section: §8 PII Scrubbing & Compliance → Values logged as-is, the "Past escher's scrub" bullet
    change: >-
      After the dated host reading ("the host built alone writes … 742 B — and the workspace-built host 5, one
      `blitz_dom::document` INFO line more, 860 B"), append the re-reading: re-measured at
      2026-10-14-stand-requirement-sweep on the same snapshot · click · type flow, the host built alone is unchanged (4
      lines, 742 B) and the workspace-built host writes 6 lines, 978 B — the `blitz_dom::document` `focussed node` INFO
      line now printing twice where it printed once, its message `[redacted]` — with the id, name and typed-sentinel
      needles still 0 at `info` and `trace` under both builds (as measured at
      escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md).
    sidecar: >-
      obs-plan §8 Values logged as-is: the workspace-built host re-read at 6 lines / 978 B (a second `focussed node`
      line), alone unchanged at 4 / 742 B, needles 0 (2026-10-14-stand-requirement-sweep).
    rationale: >-
      No PII drift — the chunk adds no log site and the needles read 0 everywhere — so the detector's escalate does not
      apply; the drift is the dated count of its own census. The report's Counts / qualifiers moved → The host's and
      the `mcp` role's log counts: "the `blitz_dom::document` `focussed node` INFO line now prints **twice** where it
      printed once. The host goes from 5 lines / 860 B to **6 / 978 B** … Built alone it is unchanged: 4 / 742 B …
      Needles are 0 everywhere"; change site "`log-counts`: 1 standing, change — obs-plan.md:296"; Expected amendments
      carry obs-plan §2, §6, §8 "**amended** (workspace build 6 / 978 B and 11; alone unchanged)".
    basis: .andromeda/obs-plan.md:296
  - detector: D-obs-pii
    severity: warning
    section: §6 Log Coverage → Logged events → escher-driver, the `tool_call` closed-span bullet
    change: >-
      After "over one CRUD flow at `info` the role's stderr reads 9 lines built alone and 10 built with the workspace —
      the tenth `INFO blitz_dom::document … message=[redacted]` — with 5 `tool_call` and 3 `command` lines in each",
      append: re-measured at 2026-10-14-stand-requirement-sweep, 9 built alone and 11 built with the workspace — the
      `blitz_dom::document` INFO line now printing twice — the 5 `tool_call` and 3 `command` lines unchanged, needles 0
      (as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md).
    sidecar: >-
      obs-plan §6 Logged events: the `mcp` role's CRUD flow re-read at 9 lines alone and 11 with the workspace (a second
      `blitz_dom::document` line) (2026-10-14-stand-requirement-sweep).
    rationale: >-
      The report's Counts / qualifiers moved: "the `mcp` role from 10 lines to **11**. Built alone it is unchanged: … 9";
      change site "`mcp-lines`: 3 standing, change — obs-plan.md:181". The same re-measurement the §8 primary records,
      restated for the `mcp` role in §6.
    basis: .andromeda/obs-plan.md:181
    dependent-of: D-obs-pii
  - detector: D-obs-pii
    severity: warning
    section: §2 Telemetry Strategy → Feature wiring, the "By invocation" bullet
    change: >-
      After "Measured at 2026-10-10-driver-cli for the session host: on a snapshot · click · type flow the
      workspace-built `escher-session` host prints that one engine line, `blitz_dom::document` at INFO, from `info` up
      and no further line at `debug` or `trace`, and the host built alone (`-p seven_guis`) prints none", append:
      re-measured at 2026-10-14-stand-requirement-sweep on the same flow, the workspace-built host prints that engine
      line twice from `info` up — `trace` adding no line — and the host built alone still none (as measured at
      escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md); the in-process
      `stand_act_spans` reading earlier in the bullet is not touched.
    sidecar: >-
      obs-plan §2 Feature wiring: the workspace-built host's `blitz_dom::document` INFO line re-read at two on the
      snapshot · click · type flow, built alone none (2026-10-14-stand-requirement-sweep).
    rationale: >-
      The report's Counts / qualifiers moved: "Under the workspace build: the `blitz_dom::document` `focussed node` INFO
      line now prints **twice** where it printed once … Built alone it is unchanged"; Expected amendments name obs-plan
      §2 among the sites "(the dated host and `mcp` counts confirmed or amended) — … **amended**". The Feature wiring
      bullet restates the host's count as "that one engine line", a third occurrence of the claim the §8 primary moves;
      the report gives no line for it.
    basis: .andromeda/obs-plan.md (§2 → Feature wiring)
    dependent-of: D-obs-pii
note: >-
  D-obs-instrumentation — no drift: the report adds no hot-path operation, public API, verb, wire field, log field or
  span ("No public API, no wire field, no verb, no node key, no log field and no span changed"); `Cause::Covered`'s
  new meaning sentence is fixed `'static` text (Coverage: instrumentation n/a, the `command` span keeps eight fields
  and `tool_call` two). D-obs-pii carries no PII finding — the three proposals under it re-date its census counts,
  severity warning, not escalate. Left as no-change by the report: obs-plan.md:188 (`focussed node` site citation)
  and :331 (citation into `session_common/mod.rs`). Not proposed: the accessible-name needle count in the new
  evidence (7) — the report's Changes state only "Needles are 0 everywhere".
