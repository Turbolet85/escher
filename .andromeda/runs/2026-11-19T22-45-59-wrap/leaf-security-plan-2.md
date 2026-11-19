@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=9 class=omission
leaf-text: (the root example `multi_doc` alone) only
source: ctr-i3f2zr5k «contract/environment-variables/blitz-cdp-port · Environment variables · BLITZ_CDP_PORT — Read only under the devtools feature: the port the CDP server binds on 127.0.0.1.»
replace-with: (the root example `multi_doc` alone) and, under the opt-in `devtools` feature alone, `BLITZ_CDP_PORT` (the port the CDP server binds on 127.0.0.1) and `BLITZ_DEVTOOLS_FRONTEND_DIR` (the directory it serves the devtools frontend from) only
@leaf_edit n=2 leaf=.claude/rules/security.md line=13 class=omission
leaf-text: a `use` element's href is never fetched
source: dec-j6dpjiro «decision/svg-mask-url-fetch · SVG mask attribute URL fetched as a sub-resource — An SVG mask attribute URL is fetched as a sub-resource through the one stamping constructor, and a fragment-only mask URL is»
replace-with: an SVG `mask` attribute URL is fetched as a sub-resource through that constructor, a fragment-only one as a stamped `data:` URL (a boundary widening by upstream merge, ratified); a `use` element's href is never fetched
@leaf_edit n=3 leaf=.claude/rules/security.md line=15 class=stale
leaf-text: every runtime event has `isTrusted = true`
source: dec-u72rg3cb «decision/event-is-trusted-by-source · isTrusted by dispatch source — A script-dispatched event reads isTrusted false, and an event the user agent dispatches reads isTrusted true.»
replace-with: a script-dispatched event reads `isTrusted` false and an event the user agent dispatches reads it true (a boundary widening by upstream merge, ratified)
@leaf_edit n=4 leaf=.claude/rules/security.md line=20 class=stale
leaf-text: No served network API, port or auth exists
source: dec-x2pm2oqo «decision/cdp-server-opt-in-loopback · CDP devtools server opt-in on loopback — The CDP devtools server is built only under the opt-in devtools feature, off in every default feature set, and binds a TCP»
replace-with: No default build serves a network API, port or auth — the opt-in `devtools` feature alone, off in every default feature set, builds a CDP server binding TCP on 127.0.0.1 at the port `BLITZ_CDP_PORT` names, with no Origin check and no authentication (a boundary widening by upstream merge, ratified)
@leaf_edit n=5 leaf=.claude/rules/security.md line=21 class=stale
leaf-text: The one listener is registered
source: dec-kap6zusi «decision/session-socket-sole-listener · Session socket as the one listener — The driver session's Unix-domain socket is the one listener of every default build, serving the session lifecycle and driver»
replace-with: The one listener of every default build is registered
@leaf_edit n=6 leaf=.claude/rules/security.md line=26 class=stale
leaf-text: winit 0.31.0-beta.3
source: dec-jlrj566g «decision/engine-dependency-pins · Engine dependency pins — The pins stay: Stylo 0.24, Taffy the git pin of upstream DioxusLabs/taffy at commit a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.4.»
replace-with: winit 0.31.0-beta.4
@leaf_edit n=7 leaf=.claude/rules/security.md line=28 class=omission
leaf-text: builds pass `--locked` — keep both.
source: dec-4y2mlb46 «decision/skia-default-build-fetch · Skia built by default with a build-time fetch — Skia is built by default in the browser app and the WPT runner, its build script fetching prebuilt binaries from GitHub»
replace-with: builds pass `--locked` — keep both. Skia is built by default in the browser app and the WPT runner, its build script fetching prebuilt binaries from GitHub releases at build time with no content check — a supply-chain widening by upstream merge, ratified.
