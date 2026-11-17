# security · standing conventions

- S1 · every action a CI step uses is pinned to a 40-hex commit SHA, checked by the workflow pin test · dependency-security
- S2 · every CI job runs with a read-only contents token and reaches no secret it does not need · authentication-authorization
  decision/publish-job-repository-guard · Publish job repository guard — The publish job is repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.
  decision/publish-secrets-upstream-main-only · Publish secrets reachable only upstream — The publish job runs only in DioxusLabs/blitz and uses the Signed Builds environment only on main or ci-test branches, so no fork ref reaches the environment or its secrets.
- S3 · every log line or log target a taken change adds passes escher's allowlist telemetry sink, outside targets dropped whole and content-named fields withheld · logging-monitoring
  decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
  decision/diff-counts-only-in-logs · Only diff sizes reach a log — A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below. · why: Ratified: a diff's content never reaches a log while its three sizes may.
  decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
- S4 · every dependency a take adds or moves is a registry dependency under locked builds, taken as upstream's whole set and passing the audit's licence and advisory gate · dependency-security
