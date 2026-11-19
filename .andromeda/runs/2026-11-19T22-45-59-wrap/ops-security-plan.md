@row_set n=1 row=decision/session-socket-sole-listener cell=rule seen=1 detector=D-security-input
value: The driver session's Unix-domain socket is the one listener of every default build, serving the session lifecycle and driver calls and their answers.
basis: fct-xcqgt2ri
because: The merge brings the opt-in devtools feature's CDP TCP listener, so the session socket is the one listener only where that feature is off.
@row_set n=2 row=decision/accessibility-tree-refresh-on-change cell=rule seen=1 detector=none
value: The platform accessibility adapter receives the tree on change as well as on its initial request: a redraw that finds a non-empty changed set rebuilds the platform tree after style resolution and clears the set.
basis: fct-xcqgt2ri
because: The refresh moved from the poll to the redraw after resolve, which reads and clears the changed set, so the row's «a poll that took a non-empty changed set» is false.
@row_set n=3 row=decision/presentational-hints-typed-parse cell=rule seen=1 detector=D-security-input
value: The font element's face and size, valign, cellspacing, cellpadding, the hr element's attributes and SVG presentation attributes and transforms are read as presentational hints by a typed parse, and a value that does not parse is ignored.
basis: fct-ymzbl26y
because: Upstream's presentational hint split adds hr and SVG presentation attributes to the typed parse, so the row's list of attributes is incomplete.
@row_set n=4 row=decision/engine-dependency-pins cell=rule seen=3 detector=D-security-deps
value: The pins stay: Stylo 0.24, Taffy the git pin of upstream DioxusLabs/taffy at commit a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.4.
basis: fct-wogrtpli
because: The merge bumps Stylo 0.22 to 0.24 and winit's exact pin to =0.31.0-beta.4, Dioxus held and Taffy unchanged.
@row_set n=5 row=decision/upstream-sync-dependency-set cell=rule seen=4 fact=F12 detector=D-security-deps
value: Each upstream sync takes upstream's dependency set whole, or by cherry-pick a coupled registry set upstream moved; past it only a precise semver-compatible lock update an operator rules to clear an audit advisory, named with its advisory id in the sync's lock record; git content only by Taffy's pin.
basis: fct-miqufxh6
because: The sync took four precise lock updates past upstream's lock, ruled by the operator to clear audit advisories and named with their ids, which the row's open «lock-only semver-compatible update» does not bound.
@row_retire n=1 row=decision/tls-via-native-tls seen=1 heir=decision/tls-via-rustls detector=D-security-auth
why: blitz-net's outbound TLS no longer runs on native-tls but on rustls, native-tls-vendored only on Android.
basis: fct-wogrtpli
because: The merge moved blitz-net's reqwest to rustls, so a row named for native-tls no longer says what provides TLS.
@row_retire n=2 row=decision/clipboard-via-arboard seen=1 heir=decision/clipboard-via-winit detector=D-security-deps
why: arboard is no longer a dependency; clipboard text goes through winit.
basis: fct-wogrtpli
because: The merge removes arboard and the Dioxus native default clipboard feature, so a row naming arboard as the clipboard no longer stands.
@row_add n=1 row=decision/cdp-server-opt-in-loopback area=api-security fact=F1 detector=D-security-input
name: CDP devtools server opt-in on loopback
rule: The CDP devtools server is built only under the opt-in devtools feature, off in every default feature set, and binds a TCP listener on 127.0.0.1 at the port BLITZ_CDP_PORT names, with no Origin check and no authentication.
why: A boundary widening by upstream merge, ratified.
basis: fct-xcqgt2ri
because: The merge brings upstream's CDP server crate and its TCP listener, a new external-input surface that future work must know is opt-in and unauthenticated.
@row_add n=2 row=contract/environment-variables/blitz-cdp-port area=secret-management fact=F2 detector=D-security-auth
boundary: Environment variables
item: BLITZ_CDP_PORT
promise: Read only under the devtools feature: the port the CDP server binds on 127.0.0.1.
basis: fct-xcqgt2ri
because: The merge brings a new environment read, gated on the devtools feature, that names the CDP server's port.
@row_add n=3 row=contract/environment-variables/blitz-devtools-frontend-dir area=secret-management fact=F2 detector=D-security-auth
boundary: Environment variables
item: BLITZ_DEVTOOLS_FRONTEND_DIR
promise: Read only under the devtools feature: the directory the CDP server serves the devtools frontend files from.
basis: fct-xcqgt2ri
because: The merge brings a new environment read, gated on the devtools feature, that names the devtools frontend directory.
@row_add n=4 row=decision/event-is-trusted-by-source area=input-validation fact=F3 detector=D-security-input
name: isTrusted by dispatch source
rule: A script-dispatched event reads isTrusted false, and an event the user agent dispatches reads isTrusted true.
why: A boundary widening by upstream merge, ratified.
basis: fct-trpvqtky
because: Script-dispatched events now read isTrusted false, disproving the standing claim that every runtime event is trusted.
@row_add n=5 row=decision/programmatic-focus-events area=input-validation fact=F4 detector=D-security-input
name: Programmatic focus changes fire focus events
rule: A programmatic focus change fires blur, focusout, focus and focusin.
why: An engine change for every document by upstream merge, ratified.
guard: event_queue, inert_subtree
except: The inert fix-up's focus clear and the driver's select focus fire none.
basis: fct-trpvqtky
because: The event-queue model makes programmatic focus fire the four focus events, while the two silent paths stay silent.
@row_add n=6 row=decision/cancelled-pointerup-still-clicks area=input-validation fact=F5 detector=D-security-input
name: A cancelled pointerup still clicks
rule: A pointerup whose handler cancels it still clicks and still ends a scrollbar drag.
why: An engine change for every document by upstream merge, ratified.
guard: pointerup_cancelled
basis: fct-ohaxmeez
because: Upstream's pointerup split takes the click and the scrollbar drag's end past a cancelled pointerup, an engine behaviour page scripts meet.
@row_add n=7 row=decision/svg-mask-url-fetch area=input-validation fact=F6 detector=D-security-input
name: SVG mask attribute URL fetched as a sub-resource
rule: An SVG mask attribute URL is fetched as a sub-resource through the one stamping constructor, and a fragment-only mask URL is fetched as a stamped data URL.
why: A boundary widening by upstream merge, ratified.
guard: svg_mask_attribute_fetch
basis: fct-ov42vbyr
because: The merge brings a new sub-resource fetch from an SVG mask attribute, measured to go only through the stamping constructor.
@row_add n=8 row=decision/visibility-collapse-hidden-from-accessibility area=input-validation fact=F7 detector=none
name: visibility collapse hidden from the accessibility tree
rule: An element with visibility collapse is hidden from the accessibility tree.
why: An engine change for every document by upstream merge, ratified.
guard: accessibility_hidden
basis: fct-ohaxmeez
because: The merge takes upstream's rule hiding a visibility-collapse element from the accessibility tree, narrowing what crosses to the adapter.
@row_add n=9 row=decision/tls-via-rustls area=data-protection fact=F8 detector=D-security-auth
name: TLS through rustls
rule: blitz-net's outbound TLS runs on rustls, with native-tls-vendored only on Android.
why: A change by upstream merge, ratified.
basis: fct-wogrtpli
because: The merge moves blitz-net's reqwest TLS backend to rustls, replacing the native-tls row.
@row_add n=10 row=decision/clipboard-via-winit area=data-protection fact=F9 detector=D-security-deps
name: Clipboard through winit
rule: Clipboard text is read and written through winit; arboard is no dependency and the Dioxus native default feature set carries no clipboard feature.
basis: fct-wakes6es
because: The merge removes arboard and the Dioxus native default clipboard feature in favour of winit's clipboard, replacing the arboard row.
@row_add n=11 row=decision/skia-default-build-fetch area=dependency-security fact=F10 detector=D-security-deps
name: Skia built by default with a build-time fetch
rule: Skia is built by default in the browser app and the WPT runner, its build script fetching prebuilt binaries from GitHub releases at build time with no content check.
why: A supply-chain widening by upstream merge, ratified.
basis: fct-wogrtpli
because: The merge makes Skia a default renderer, bringing an unchecked build-time binary download into default builds.
@row_none n=1 fact=F11 type=6
what: One entry of the licence table; deny.toml holds it, and decision/licence-gate already states the table is upstream's, held byte for byte.
@row_none n=2 fact=F13 type=5
what: Where one crate sits in the resolved dependency graph; the lockfile and the chunk's lock record hold it.
