@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/lay
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=30 class=omission
leaf-text: Android browser APK (built, not tested in CI).
source: dec-ffuxmrxz «decision/mobile-targets-built-not-tested · Mobile targets built, not tested — CI builds the iOS and Android targets but runs no tests on them.»
replace-with: Android browser APK; CI builds the iOS and Android targets but runs no tests on them.
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=28 class=omission
leaf-text: `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`
source: ctr-gnaerhxc «contract/escher-session-snapshot/answer · escher-session snapshot · answer — One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.»
replace-with: `{"text":…,"part":n,"parts":m}` for a screen read in parts, a part past the last giving empty text with the same parts, status `0`
