@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=44
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=30 class=omission
leaf-text: Android browser APK (built, not tested in CI).
source: dec-ffuxmrxz «decision/mobile-targets-built-not-tested · Mobile targets built, not tested — CI builds the iOS and Android targets but runs no tests on them.»
replace-with: Android browser APK and the iOS target (CI builds both, runs no tests on them).
