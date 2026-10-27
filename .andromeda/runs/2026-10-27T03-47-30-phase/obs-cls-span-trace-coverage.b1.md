# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · a span absence in the 32 examples-slice files; no example code changes and the chunk states no span there, only the driver's two
span-trace-coverage#1.2 · no · a span absence in the 86 apps-slice files (apps/browser, apps/rdme), an upstream emitter the chunk does not reach (Not needed)
span-trace-coverage#1.3 · no · a span absence in the 15 s05 files; an engine emitter the chunk does not reach, and it states no span there
span-trace-coverage#1.4 · no · a span absence in the 17 listed s06 files; an emitter outside the chunk's reach, and it states no span there
span-trace-coverage#1.5 · no · a span absence in the 8 layout-slice files; an engine emitter the chunk does not reach (Not needed)
span-trace-coverage#1.6 · no · a span absence in 16 listed files, its one match a doc comment; the chunk states no span there, only the driver's two
span-trace-coverage#1.7 · no · a span absence in the 32 listed s09 files; no code change reaches them, and the chunk states no span there
span-trace-coverage#1.8 · no · a span absence in the blitz-vibey-script and blitz slice files; engine and app emitters the chunk does not reach (Not needed)
span-trace-coverage#1.9 · no · a span absence in the 21 listed s11 files; no code change reaches them, and the chunk states no span there
span-trace-coverage#1.10 · yes · records no `opentelemetry` or `metrics` over the 61 test-crate files, one of the two absence records behind the README's claim that no OTel export or metrics exporter exists
span-trace-coverage#1.11 · yes · records no `opentelemetry` or `metrics` over the 12 WPT-runner files, the other absence record behind the README's claim that no OTel export or metrics exporter exists
span-trace-coverage#2.1 · yes · the `command` span's eight fields, the eight cause names the statement's pin re-reads, the never-fielded list (a part's number or count, a lint's findings) and the load wait's no-span reading, which the README's driver paragraph and telemetry line state
span-trace-coverage#2.2 · yes · the hosted part read's closed `command` lines and host_log's fourth test's 0 id and 0 name needles at `trace` under both builds, the recorded value any README line on that read must agree with
span-trace-coverage#3.1 · yes · the `tool_call` span's two fields, `tool` (the twelve verb words the README's tool count and the pin read) and `ending` (accepted, refused, error), which the telemetry line restates
span-trace-coverage#3.2 · yes · the cold-agent counts read from `tool_call` lines in the live `server.log` (`lint_calls`, `screenshot_calls`, `advance_calls`, and the refused and error counts feeding `wrong_calls`), the source of the README's 66-call, 0-wrong, lint and screenshot figures
