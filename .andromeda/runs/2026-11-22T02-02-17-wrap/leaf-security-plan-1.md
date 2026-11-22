@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: a failed, empty or non-CSS stylesheet blocks no render
source: dec-iigg6kvs «In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render.»
replace-with: a failed, empty, non-CSS or non-UTF-8 stylesheet blocks no render
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=25 class=omission
leaf-text: or their secrets.
source: dec-4y2mlb46 «Skia is built by default in the browser app and the WPT runner from a prebuilt archive for the locked skia-bindings tag, fetched once per platform and verified against a committed per-target sha256»
replace-with: or their secrets. Skia is built by default in the browser app and the WPT runner from a prebuilt archive for the locked skia-bindings tag, fetched once per platform at build time and verified against a committed per-target sha256, a differing archive refused leaving no file and the build reading only the verified local copy (a supply-chain widening by upstream merge, ratified).
