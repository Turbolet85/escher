# obs · standing conventions

- S1 · each new or rewritten check installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, and the silence census reads 0 over its file · log-coverage
  (obs-plan/log-coverage fct-x7cwmno3) logs
  (obs-plan/log-coverage fct-vhgizwef) logs
  (obs-plan/log-coverage fct-2yjtpnwu) logs
  (obs-plan/telemetry-strategy fct-tasmmgp5) strategy
- S2 · each engine-crate log site is compiled only behind the crate's tracing cargo feature, with a no-op path when the feature is off · telemetry-strategy
  (obs-plan/log-coverage fct-4za533k3) logs
- S3 · page text, attribute values and ids held in process are never logged, fielded or printed with Debug · pii-scrubbing-compliance
