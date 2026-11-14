@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summa
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=32 class=omission
leaf-text: the session library emits one span per command under the target `escher_driver`
source: dec-rlke2ua7 «decision/exploration-adds-no-span · Auto-exploration adds no span of its own — Auto-exploration opens no span and emits no event of its own; each call it drives, re-reach steps included, runs through»
replace-with: the session library emits one span per command under the target `escher_driver` (an auto-exploration opens no span and emits no event of its own, each call it drives, re-reach steps included, leaving its own command span)
