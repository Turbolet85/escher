# obs · standing conventions

- S1 · each new stand or engine check installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, and the silence census reads 0 over the file and any shared module it reads · log-coverage
  (obs-plan/log-coverage fct-4julxjdl) logs
  (obs-plan/log-coverage fct-zxoieuap) logs
  (obs-plan/log-coverage fct-onrhzzvo) logs
  (obs-plan/log-coverage fct-2yjtpnwu) logs
  (obs-plan/log-coverage fct-pynegysb) logs
  (obs-plan/log-coverage fct-pae5nu2v) logs
  (obs-plan/log-coverage fct-ajxo3ln2) logs
  (obs-plan/pii-scrubbing-compliance fct-yvbbn3zt) pii
  (obs-plan/span-trace-coverage fct-j47hr7dn) spans
  (obs-plan/telemetry-strategy fct-tasmmgp5) strategy
- S2 · each new escher-driver module or answer type adds no tracing site, env read or print; an Outcome is written by to_json alone and never logged, fielded or printed with Debug · log-coverage
- S3 · escher-driver opens only the command and tool_call spans, their fields closed words, bools and counts, and emits no event; a new verb adds no span or field · span-trace-coverage
  (obs-plan/log-coverage fct-wdv62het) logs
- S4 · every field reaching escher's sink passes the allowlist scrub, content-named fields redacted at print time; host_log and mcp_log read 0 needles at trace under both builds after the change · pii-scrubbing-compliance
  (obs-plan/error-capture-reporting fct-kkhzj346) errors
  (obs-plan/log-coverage fct-h6shwcuf) logs
  (obs-plan/log-coverage fct-k3vr6esa) logs
  (obs-plan/log-coverage fct-4hkolobh) logs
  (obs-plan/log-coverage fct-jok55ozp) logs
  (obs-plan/pii-scrubbing-compliance fct-q64qhjfz) pii
  (obs-plan/pii-scrubbing-compliance fct-ukvug6pr) pii
  (obs-plan/pii-scrubbing-compliance fct-zofw4lw2) pii
  (obs-plan/telemetry-strategy fct-46rgllvo) strategy
