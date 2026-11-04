@amendment marker=2026-11-04-dioxus-runtime-probe
title: the Dioxus runtime probe's silence row
section: §6 Log Coverage (obs-plan/log-coverage: the observed-absent rows)
change:
  - A new row records a print, an env read, a subscriber or a listener observed absent in the test target `dioxus_runtime_probe` (blitz-tests, 10 tests, both layout modes, no `stand_` prefix). The target drives `blitz_test_harness::Harness` over Counter, CRUD, Cells and its own pages `toggled_listener`, `shared_state` and `hazard`, and its failure messages name a layout mode, a task or page and a link or index only.
  - Its search is the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over the file: exit 1, reading 0, as measured at the chunk's report (its gate entry 2).
why: the chunk's gate fired the silence census over the new file and it read 0, and the report marks the new target's instrumentation n/a, with no print, env read, subscriber or listener. The pre-pass sweep's silence pattern found the 13 standing rows were each an earlier chunk's, which left the new file's row as new text.
kept: §3 Logging stack's body keeps its silence rows unchanged and takes no row for this target. The new row stands in log-coverage alone, as the earlier blitz-tests target with no `stand_` prefix does.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the Dioxus runtime probe's read values stay in the test
section: §8 PII Scrubbing & Compliance (obs-plan/pii-scrubbing-compliance)
change:
  - A new item records that the test target `dioxus_runtime_probe` has no runtime surface and no external input. It reads each element's rendering component, its current listeners and a held signal's reader scopes through public dioxus-core, dioxus-signals and dioxus-native-dom API, and no read value leaves it.
  - Its failure messages name a layout mode, a task or page and a link or index. They never name a component tied to a node, a listener read off a node, a signal value or a node id.
  - Its reading `evidence/probe-reading.md` quotes no captured stderr, panic payload, signal value or node id. The backtrace that confirmed the hazard's panic site enters it by its coordinates alone.
  - It holds no print, env read, subscriber or listener, and the silence census over it reads 0, as measured at the chunk's report.
why: the report marks the new target's PII as n/a, its failure messages carrying no read value. Its obs outcome was met because no failure message or document carries a read value. Only a captured backtrace's coordinates entered the reading, never a payload or stderr. These are the component, listener and subscriber readings that the UI graph (values masked, never logged) will build on.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
