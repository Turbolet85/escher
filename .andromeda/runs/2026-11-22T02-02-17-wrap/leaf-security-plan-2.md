@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=28 class=stale
leaf-text: its build script fetching prebuilt binaries from GitHub releases at build time with no content check
source: dec-4y2mlb46 «fetched once per platform and verified against a committed per-target sha256; a differing archive is refused, leaving no file, and the build reads only the verified local copy»
replace-with: from a prebuilt archive for the locked skia-bindings tag, fetched once per platform and verified against a committed per-target sha256 (a differing archive refused, leaving no file; the build reads only the verified local copy)
@leaf_edit n=2 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: a disabled, inert or hidden one or one with no row to show opens none
source: dec-i3d6xc6s «and a disabled, inert or hidden one, one with no row to show, and one under a non-invertible transform open none»
replace-with: a disabled, inert or hidden one, one with no row to show, or one under a non-invertible transform opens none
