proposals:
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (subscriber installation), the headless-stand census bullet
    change: >-
      Restate the census over the extended checks: `stand_contrast` (its three tests of 2026-10-14-stand-a11y-assertions
      — `TEXT_ROWS`, `STATE_ROWS` with hover and active reached by pointer, the control record — included) and
      `stand_keyboard` (its three enabled-state tests included, now reading the shared module's `tab_order`, `focused`,
      `controls` and `ENABLED_TAB_ORDER`) still install no subscriber, read no env var, print nothing and bind no
      listener; add `stand_accessibility_ids` (since 2026-10-14-stand-a11y-assertions, its three tests reading the
      accessibility tree through `&dyn blitz_dom::Document` in both layout modes) as installing no subscriber, reading
      no env var, printing nothing and binding no listener, its readings going into assertions only; the silence probe
      (`println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`)
      reading 0 over `stand_contrast.rs`, `stand_keyboard.rs`, `stand_accessibility_ids.rs` and
      `tests/blitz-tests/tests/common/mod.rs` (implement gate 6), with
      escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md added to the bullet's as-measured list.
    sidecar: >-
      obs-plan §3 Logging stack: the headless-stand silence census restated over stand_contrast, stand_keyboard and,
      newly named, stand_accessibility_ids and the shared common module — 0 at 2026-10-14-stand-a11y-assertions.
    rationale: >-
      The report's Counts / qualifiers moved (the silent census) says the census now runs over stand_contrast.rs,
      stand_keyboard.rs, stand_accessibility_ids.rs and common/mod.rs and reads 0 at exit 1 (implement gate 6), its
      claim site obs-plan.md:70; its Expected amendments list "obs-plan §3. The silent census restated over the extended
      checks" (sites obs-silent obs-plan.md:70 ×12, stand-checks obs-plan.md:70 ×6). obs-plan.md:70 names
      stand_contrast and stand_keyboard but never stand_accessibility_ids, which gained three tests this chunk
      (stand_accessibility_ids.rs 255-308, 310-352, 363-410), so the subscriber-installation census omits a check the
      chunk measured silent. No telemetry library, subscriber, env read or dependency was added (Dependencies none;
      Coverage: no new hot-path operation).
    basis: .andromeda/obs-plan.md:70
note: >-
  D-obs-instrumentation — no drift: the report states no new hot-path operation, public API, crate or dependency; every
  new symbol is test-only. D-obs-pii — no drift: the chunk adds no log site; `crud-list`'s new accessible name `People`
  joined host_log / mcp_log's name needles and reads 0 at `trace` under both builds (gate 3), so §8's dated needle
  counts (obs-plan.md:296) stay true as history and no amendment is owed. The "19 of which read the shared module"
  count on obs-plan.md:70 is left as it reads: the report states no change to the shared module's reader count; if
  stand_accessibility_ids is a new reader of common/mod.rs, that count moves and the orchestrator should re-derive it.
