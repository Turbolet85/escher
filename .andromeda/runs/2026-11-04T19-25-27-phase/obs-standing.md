# obs · standing conventions

- S1 · each new stand or engine check installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, the silence census reading 0 over its file · log-coverage
  (obs-plan/log-coverage fct-4julxjdl) logs
  (obs-plan/log-coverage fct-x7cwmno3) logs
  (obs-plan/log-coverage fct-onrhzzvo) logs
  (obs-plan/log-coverage fct-vhgizwef) logs
  (obs-plan/log-coverage fct-2yjtpnwu) logs
- S2 · a new reader of a stand check shared module (tests/common/mod.rs, session_common/mod.rs, fixtures/mod.rs) is counted among that module's readers, and the census over the module still reads 0 · section 3 Observability Harness Contract (Logging stack)
  (obs-plan/log-coverage fct-ajxo3ln2) logs
- S3 · a change adds no log site, span, field, log field or env read, escher-driver keeping its 2 info_span! sites and stand_act_spans its eight fields · span-trace-coverage
  (obs-plan/log-coverage fct-gwvelgmp) emitter-escher-driver
  (obs-plan/log-coverage fct-wdv62het) logs
  (obs-plan/log-coverage fct-jok55ozp) logs
  (obs-plan/metric-coverage fct-ujfm2d3p) metrics
  (obs-plan/pii-scrubbing-compliance fct-jkqmrdqe) pii
  (obs-plan/pii-scrubbing-compliance fct-ovjvwkua) pii
  (obs-plan/span-trace-coverage fct-hwmommmi) spans
  (obs-plan/span-trace-coverage fct-sgavnz6y) spans
  (obs-plan/span-trace-coverage fct-uvyy5siu) spans
  (obs-plan/span-trace-coverage fct-hb6qlpie) spans
  (obs-plan/span-trace-coverage fct-jfxkjttv) spans
  (obs-plan/span-trace-coverage fct-xc6f5zsc) spans
  (obs-plan/span-trace-coverage fct-gag5zsei) spans
  (obs-plan/span-trace-coverage fct-n4m37wsf) spans
  (obs-plan/span-trace-coverage fct-jyfscbc7) spans
  (obs-plan/span-trace-coverage fct-gf4hwplk) spans
  (obs-plan/span-trace-coverage fct-xswdmupu) spans
- S4 · a value read from a page or app (ids, names, control values, text) or a type deriving Debug that holds them is never logged, fielded or printed · pii-scrubbing-compliance
  (obs-plan/pii-scrubbing-compliance fct-v4kxqyim) pii
  (obs-plan/pii-scrubbing-compliance fct-oir2d6z3) pii
  (obs-plan/pii-scrubbing-compliance fct-stuzvy46) pii
- S5 · a check's failure or assertion message names a kind and a count, never a needle or a captured line · log-coverage
