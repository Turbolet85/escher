@leaf_group group=architecture+layout-templates
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_edit n=1 leaf=.claude/docs/services/seven_guis.md line=11 class=omission
leaf-text: The former two-argument form `<task> <state-dir>` is gone.
source: fct-exqct2ck «a fourth role, `escher-session coverage <app> <check>...` — one app the binary boots, then one or more kept-check paths, no `--session` and no `--record` word among them, any other shape usage»
replace-with: The former two-argument form `<task> <state-dir>` is gone. A fourth role, `escher-session coverage <app> <check>...` — one app the binary boots, then one or more kept-check paths, no `--session` and no `--record` word among them, any other shape usage (stderr, exit 2) — reads each path bounded at `MAX_CHECK_BYTES + 1` and parses it by `Check::from_bytes` (unreadable, past the bound, no check or a check of another app refused `malformed`, exit 1), replays each check on a fresh boot through the same boot `main` hands to the command line and answers one uncoloured line `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}` on stdout, exit 0, `owner` left out when absent; a boot that fails ends with the session error, exit 3.
@leaf_edit n=2 leaf=.claude/docs/services/seven_guis.md line=3 class=tree
leaf-text: `.andromeda/architecture.md` §Existing Scopes
source: fct-gp7aqjse «Existing scopes: the engine crates under `packages/` — blitz-dom by area, blitz-html, blitz-net, blitz-paint, blitz-shell, blitz-traits, blitz-vibey-script, blitz, stylo_taffy»
replace-with: `doc:architecture` §Existing Scopes
@leaf_edit n=3 leaf=.claude/docs/services/seven_guis.md line=3 class=tree
leaf-text: `.andromeda/layout-templates.md` desktop-native
source: fct-nxmh4fah «- seven_guis Home and TaskShell — Home is a centered 640px column of task cards inside a box that scrolls»
replace-with: `doc:layout-templates` desktop-native
