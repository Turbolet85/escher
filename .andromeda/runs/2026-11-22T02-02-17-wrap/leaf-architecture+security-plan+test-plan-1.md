@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=11 class=omission
leaf-text: Through `seven_guis → escher-driver` both seven_guis binaries therefore compile
source: dec-xxvmhvjw «The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var.»
replace-with: — and `escher-telemetry` (the workspace's), whose scoped capture default the driver sets around an acting step and whose capture installs once per process a chaining panic hook that prints and logs nothing. Through `seven_guis → escher-driver` both seven_guis binaries therefore compile
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=16 class=omission
leaf-text: and one a listener stops, or a handler's panic unwinds, counts its innermost listener alone
source: ctr-cjehnp7t «A bubbling dispatch that still propagates counts every listener it passed; one its target or an ancestor stops counts its innermost listener alone, and a non-bubbling event its target alone.»
replace-with: and one a listener stops, or a handler's panic unwinds, counts its innermost listener alone, and a non-bubbling event counts its target alone
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: so `type` fires no DOM focus events on a single select
source: ctr-zg6angke «The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.»
replace-with: so `type` fires no DOM focus events (no blur, focusout, focus or focusin) on any select
