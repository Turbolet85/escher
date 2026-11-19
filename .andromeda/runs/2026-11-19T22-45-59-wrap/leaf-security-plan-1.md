@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=7 class=stale
leaf-text: no served network API, no served TCP or UDP port
source: abs-dvmy4lh2 «A served network API in a default build: no default feature set serves a TCP or UDP port»
replace-with: no served network API in a default build, no default feature set serving a TCP or UDP port
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=7 class=omission
leaf-text: its one listener is local
source: dec-x2pm2oqo «The CDP devtools server is built only under the opt-in devtools feature, off in every default feature set, and binds a TCP listener on 127.0.0.1 at the port BLITZ_CDP_PORT names, with no Origin check and no»
replace-with: the opt-in `devtools` feature, off in every default feature set, builds a CDP devtools server binding TCP on 127.0.0.1 at the port `BLITZ_CDP_PORT` names, with no Origin check and no authentication (a boundary widening by upstream merge, ratified); its one default listener is local
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: the clear firing no DOM `blur` or `focusout`
source: dec-tlflqmdv «A programmatic focus change fires blur, focusout, focus and focusin.»
replace-with: the clear firing no DOM `blur` or `focusout`, while any other programmatic focus change fires `blur`, `focusout`, `focus` and `focusin` (an engine change for every document by upstream merge, ratified)
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: a `use` element's href never fetched — a boundary widening by upstream merge, ratified.
source: dec-j6dpjiro «An SVG mask attribute URL is fetched as a sub-resource through the one stamping constructor, and a fragment-only mask URL is fetched as a stamped data URL.»
replace-with: a `use` element's href never fetched — a boundary widening by upstream merge, ratified; in every document an SVG `mask` attribute URL is fetched as a sub-resource through the same stamping constructor, a fragment-only mask URL as a stamped data URL — a boundary widening by upstream merge, ratified.
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=14 class=stale
leaf-text: every runtime event is `isTrusted`
source: dec-u72rg3cb «A script-dispatched event reads isTrusted false, and an event the user agent dispatches reads isTrusted true.»
replace-with: an event the user agent dispatches reads `isTrusted` true and a script-dispatched one false (a boundary widening by upstream merge, ratified)
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=17 class=stale
leaf-text: TLS via reqwest `native-tls`
source: dec-htgpvmpe «blitz-net's outbound TLS runs on rustls, with native-tls-vendored only on Android.»
replace-with: TLS via rustls, `native-tls-vendored` only on Android
