proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 15
    section: '## 8. Mocking & Stubbing Discipline'
    change: 'After line 15 add the row: | Network (stand load page) | `LoadNet`, the stand''s own in-process stand-in since 2026-10-25-cold-agent-bar and escher''s third hand-written network stand-in beside `HeldNet` and `LateNet`, the one in non-test code: `seven_guis::stand::boot_load` replaces its options'' `net_provider` with `Some(Arc::new(LoadNet))` through `..options`, every field else kept; each `fetch` answers the fixed `LOADED_CSS` (`.load-body .load-done { display: block; }`) for the fixed `LOAD_URL` `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), so the load page''s click leaves a load in flight that a driver step waits out inside `LOAD_WAIT`; it does not override `is_noop`, so the harness counts its load; it names no `blitz_net` and holds no print, env read, socket or listener; every other stand boot keeps `net_provider: None` (the unit test `the_stand_options_stay_offline`); read by `stand_load_page` and the unit test `the_load_page_click_leaves_a_load_in_flight` | (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175) |'
    sidecar: 'test-plan §8 detail: the stand''s own `LoadNet` stand-in (boot_load, 200 ms self-answer, load counted, no blitz_net) added to the stand-in table — 2026-10-25-cold-agent-bar.'
    rationale: 'The report''s Symbols / APIs name `boot_load` and the private `struct LoadNet` (`impl NetProvider` 167-175, each fetch answering `LOADED_CSS` after `LOAD_ANSWER_DELAY`, no `is_noop` override, no `blitz_net`, print, env read, socket or listener), and F17 names it a third hand-written network stand-in beside `HeldNet` and `LateNet`, the stand no longer wholly offline; the stand-in table holds no row for it.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:185
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 14
    section: '## 8. Mocking & Stubbing Discipline'
    change: 'In the `HeldNet` row, after «answering a head stylesheet and then its `@import` on `.test` URLs with no real fetch» add: «; since 2026-10-25-cold-agent-bar it is one of three hand-written network stand-ins, beside the self-answering `LateNet` and the stand''s own `LoadNet` (rows below), and still the one that holds»'
    sidecar: 'test-plan §8 detail: the `HeldNet` row says it is one of three network stand-ins since `LoadNet` — 2026-10-25-cold-agent-bar.'
    rationale: 'F17 marks mocking-stubbing-discipline.md:14 stale under `stand-offline`: the row reads `HeldNet` as escher''s one held network stand-in with no word of the stand''s new `LoadNet` beside it.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:187
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 15
    section: '## 8. Mocking & Stubbing Discipline'
    change: 'In the `LateNet` row, after «and its threads end with the test process» add: «; since 2026-10-25-cold-agent-bar the stand''s own `LoadNet` answers the same way, from a thread of its own after the same 200 ms (`LOAD_ANSWER_DELAY`), in the stand''s code rather than a test file (next row)»'
    sidecar: 'test-plan §8 detail: the `LateNet` row points to the stand''s `LoadNet`, which self-answers the same way — 2026-10-25-cold-agent-bar.'
    rationale: 'F17 marks mocking-stubbing-discipline.md:15 stale under `stand-offline`: `LateNet` is no longer the one self-answering network stand-in, `LoadNet` answering each fetch from its own thread after `LOAD_ANSWER_DELAY` (200 ms).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:35
    dependent-of: D-new-detail
note: F17's other stale sites in test-plan's body (test-plan.md:336 the stand options, :352 the §8 Network line) are the body's to amend, not this file's; the Time row's citation examples/seven_guis/src/stand.rs:92-109 (line 26) may have moved with stand.rs's added lines 1-9 · 12 · 17, which the report's citation dispositions do not list — the P2 citation sweep's to re-read.
