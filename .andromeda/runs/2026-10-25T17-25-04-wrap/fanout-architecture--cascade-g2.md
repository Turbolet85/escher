proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 163
    section: ## Standard Contracts
    change: - **Headless stand (seven_guis, native only):** `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`) and a load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font, every boot offline but the load page's, whose one stylesheet fetch an in-process stand-in answers after 200 ms; `timer_step` hands the Timer its ticks for a driver session; it installs no telemetry, reads no env var and names no `blitz_net`.
    sidecar: architecture body §Standard Contracts → Headless stand — the stand boots Cells and the load page beside the lean four, offline but for the load page's in-process stand-in (2026-10-25-cold-agent-bar)
    rationale: the report's Symbols name `boot_cells` and `boot_load` with `LoadNet` answering after `LOAD_ANSWER_DELAY` (200 ms), and its sweep lists architecture.md:163 stale under both `served-tasks` (F1) and `stand-offline` (F17); the plan's Expected amendments name this site
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:185-187
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 120
    section: ## Standard Contracts
    change: on line 120, after "`net_provider: None` (offline through `DummyNetProvider`) and `base_url: None`" add "— `None` kept for every caller (unit test `the_stand_options_stay_offline`), so every stand boot is offline but `boot_load`'s, which replaces that one field (below)", and after "names no `blitz_net`" add "— its load page's stand-in included"
    sidecar: standard-contracts Headless stand — `options` stays offline for every caller; every boot is offline but `boot_load`'s (2026-10-25-cold-agent-bar)
    rationale: the report's sweep lists standard-contracts.md:120 stale under `stand-offline` (F17, the stand offline but for the load page's in-process stand-in); its Symbols state `stand::options` keeps `net_provider: None` for every other caller (unit test `the_stand_options_stay_offline`) and `LoadNet` names no `blitz_net`
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:33-38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 120
    section: ## Standard Contracts
    change: add to the Headless stand entity, after `boot_timer`, "since 2026-10-25-cold-agent-bar, `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts `Task::Cells` in `TaskShell` through the private `cells_root`, Cells staying no `LeanTask` (examples/seven_guis/src/stand.rs:131-139; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: standard-contracts Headless stand — `boot_cells` mounts Cells in TaskShell, still no `LeanTask` (2026-10-25-cold-agent-bar)
    rationale: report Symbols / APIs — `seven_guis::stand::boot_cells(HarnessOptions) -> Harness<DioxusDocument>` (stand.rs:131-135) mounts `Task::Cells` in `TaskShell` through `cells_root` (137-139); Cells stays no `LeanTask`
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:27-29
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 120
    section: ## Standard Contracts
    change: add to the Headless stand entity, after `boot_cells`, "`boot_load(HarnessOptions)` mounts a load page in `TaskShell` titled "Load" (`load_root`, `LoadPage`, its CSS `.load-root` · `.load-body` · `.load-btn` with `:hover` · `:active` · `.load-done`) holding two keyed elements — `load-fetch`, a Button "Load" on the stand accent `#3a5ce5` with white text (hover `#2a4cd3`, active `#1f3fc1`), and `load-done`, a Paragraph "Loaded" at `display: none` until its sheet lands, no CSS selecting by id — its options' `net_provider` replaced by `Some(Arc::new(LoadNet))` through `..options` and every other field kept (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:177-243)"
    sidecar: standard-contracts Headless stand — `boot_load` mounts the load page, replacing only `net_provider` with `LoadNet` (2026-10-25-cold-agent-bar)
    rationale: report Symbols / APIs — `boot_load` (141-152), `load_root` 177-183, `LoadPage` 185-200, its CSS 203-243, `net_provider` replaced by `Some(Arc::new(LoadNet))` through `..options`, every field else kept; the page's ids `load-fetch` and `load-done` with their colours and `display: none`
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:30-40
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 120
    section: ## Standard Contracts
    change: add to the Headless stand entity, after `boot_load`, "the private `LoadNet` implements `NetProvider`: each `fetch` answers the fixed `LOADED_CSS` (`.load-body .load-done { display: block; }`) for the fixed `LOAD_URL` `http://stand.test/load.css` from a thread of its own after `pub const LOAD_ANSWER_DELAY: Duration` (200 ms); it overrides no `is_noop`, so the harness counts its load, and holds no `blitz_net`, print, env read, socket or listener — a hand-written network stand-in beside the tests' `HeldNet` and `LateNet`, living in the stand module (examples/seven_guis/src/stand.rs:167-175)"
    sidecar: standard-contracts Headless stand — `LoadNet` answers the load page's fixed sheet after 200 ms, counted by the harness, no `blitz_net` (2026-10-25-cold-agent-bar)
    rationale: report Symbols / APIs — `LOAD_ANSWER_DELAY` = 200 ms, `LOAD_URL`, `LOADED_CSS`, `impl NetProvider` 167-175 answering from a thread of its own, no `is_noop` override, no `blitz_net`, print, env read, socket or listener (silence census entry 10 reads 0); F17 names it the third stand-in beside `HeldNet` and `LateNet`
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:33-37
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 108
    section: ## Standard Contracts
    change: on line 108, "an offline harness (the stand's) and a document passed to `wrap` carry no counter" becomes "an offline harness (every stand boot but `boot_load`) and a document passed to `wrap` carry no counter", followed by "; the stand's load page, whose `LoadNet` overrides no `is_noop`, is wrapped in the counter, so its one stylesheet fetch reads as a load in flight until answered (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: standard-contracts Test harness — the stand's load page carries the load counter; every other stand boot stays offline (2026-10-25-cold-agent-bar)
    rationale: the report states `LoadNet` has no `is_noop` override, so the harness counts its load, and the unit test `the_load_page_click_leaves_a_load_in_flight` pins it; the entity's "an offline harness (the stand's)" no longer holds for the stand as a whole (F17, the stand offline but for the load page's stand-in)
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:35-37
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 106
    section: ## Standard Contracts
    change: after "`dom_string()` renders `<div #box .a .b> @ (20,10) 100x50`" add "— a select choice moves no class or text it shows, so a check reading the DOM through it reads a select's choice as unchanged (`stand_id_persistence`; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: standard-contracts Test harness — a select choice moves nothing `dom_string` shows (2026-10-25-cold-agent-bar)
    rationale: report Decisions & corrections — measured, a select choice moves no class or text `Harness::dom_string` shows (`stand_id_persistence` read the DOM unchanged); Reverted facts — the separate re-render step was folded into the Book step for that reason
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:353-354
    fact: F10
note: not re-numbered for want of a report number — standard-contracts.md:120's `examples/seven_guis/src/stand.rs:21-129` (stand.rs added lines 1-9 · 12 · 17 above it) and :27's `stand_accessibility_ids.rs:310-352 · 363-410` (added line 181) may have shifted; F5 · F7 · F11 · F12 and the sweep's unpicked stale sites (standard-contracts.md:163 · :211 · :218 · :233 · :234, architecture.md:187) sit in no picked entity of this group.
