@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summa
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=21 class=omission
leaf-text: — and, as a dev-dependency, by blitz-tests.
source: dec-xxvmhvjw «decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture»
replace-with: — by the driver library escher-driver, which sets only the scoped log capture around an acting step and installs no sink — and, as a dev-dependency, by blitz-tests.
