# obs · standing conventions

- S1 · each new test or check installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, the silence census reading 0 over the file and any shared module it reads · log-coverage
  (obs-plan/log-coverage fct-x7cwmno3) logs
  (obs-plan/log-coverage fct-onrhzzvo) logs
  (obs-plan/log-coverage fct-vhgizwef) logs
  (obs-plan/log-coverage fct-2yjtpnwu) logs
  (obs-plan/log-coverage fct-pynegysb) logs
  (obs-plan/log-coverage fct-pae5nu2v) logs
  (obs-plan/log-coverage fct-ajxo3ln2) logs
  (obs-plan/pii-scrubbing-compliance fct-yvbbn3zt) pii
  (obs-plan/span-trace-coverage fct-j47hr7dn) spans
  (obs-plan/telemetry-strategy fct-tasmmgp5) strategy
- S2 · a new reader of tests/blitz-tests/tests/session_common/mod.rs, fixtures/mod.rs or common/mod.rs is recorded in that module's reader list and count · log-coverage
- S3 · a new escher-driver module or function holds no tracing site, env read or print; the driver keeps its 2 info_span sites and the command span its eight fields · span-trace-coverage
  (obs-plan/span-trace-coverage fct-gf4hwplk) spans
- S4 · a new Outcome, result field or read type derives Debug only if nothing logs, fields or prints it; it is written once, by to_json, as a command's answer · pii-scrubbing-compliance
  (obs-plan/pii-scrubbing-compliance fct-myhl4hwq) pii
- S5 · a committed kept check or fixture holds no typed text, host path or secret · ci-integration
  (obs-plan/ci-integration fct-7467zgun) ci
- S6 · a new span field or MCP tool word is a closed word, bool or count, judged pair by pair by the sink's allowlist scrub · § 8. PII Scrubbing & Compliance
  (obs-plan/pii-scrubbing-compliance fct-q64qhjfz) pii
