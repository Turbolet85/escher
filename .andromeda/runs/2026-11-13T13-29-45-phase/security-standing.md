# security · standing conventions

- S1 · each new or taken test, fixture or harness keeps the test harness free of tracing and style setup and holds no page content in a log line · dependency-security
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · each new log line or tracing target goes through escher's allowlist stderr sink, carrying only allowlisted, content-free fields · logging-monitoring
  decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
  decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
  decision/lint-findings-closed-content · Lint findings hold no page content — A lint finding holds ids, closed defect words and numbers only, never page text, colour or a control value, and no log site, print, span or field carries a finding. · why: A ratified boundary widening.
  decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
- S3 · each new or changed CI job or workflow pins its actions to commit SHAs, runs with a read-only token and keeps secrets out of fork runs · dependency-security · authentication-authorization
- S4 · each new crate that enters the lock is a registry version, reviewed by hand and passed through the licence and advisory gate · dependency-security
