@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summa
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=6 class=stale
leaf-text: escher 0.1.0 ships none
source: dec-4ostz6ne «decision/no-otel-export · No OTel export — The product ships no OTel export: no OTel crate, no egress and no OTEL_EXPORTER_OTLP_HEADERS credential path.»
replace-with: escher ships none — no OTel crate, no egress and no `OTEL_EXPORTER_OTLP_HEADERS` credential path
