# obs · standing conventions

- S1 · each new or extended test check installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, with the silence census reading 0 over the file and any shared module it reads · log-coverage
  (obs-plan/log-coverage fct-x7cwmno3) logs
  (obs-plan/log-coverage fct-onrhzzvo) logs
  (obs-plan/log-coverage fct-vhgizwef) logs
  (obs-plan/log-coverage fct-2yjtpnwu) logs
  (obs-plan/log-coverage fct-pynegysb) logs
  (obs-plan/log-coverage fct-pae5nu2v) logs
  (obs-plan/log-coverage fct-ajxo3ln2) logs
  (obs-plan/span-trace-coverage fct-j47hr7dn) spans
  (obs-plan/telemetry-strategy fct-tasmmgp5) strategy
- S2 · each new escher-driver or dioxus-native-dom module holds no tracing site, env read or print, and adds no log site, span, field, log field or env read · log-coverage
  (obs-plan/error-capture-reporting fct-kic3ctd2) errors
  (obs-plan/span-trace-coverage fct-uhhja7ws) spans
- S3 · each new outcome or read type that holds ids, names or values is written by to_json alone as an answer and is never logged, fielded or printed with Debug · pii-scrubbing-compliance
- S4 · each new verb or MCP tool adds no span and no span field beyond command's eight and tool_call's two, each a closed word, bool or count · span-trace-coverage
