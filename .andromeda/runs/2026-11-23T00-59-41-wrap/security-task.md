@task_prose part=session
The session merged upstream main at 165d698c3 into the fork as a two-parent merge, taking ten upstream commits on the founder's rulings, every one of them as upstream wrote it.
Upstream brought six engine changes: DOMTokenList replace (classList.replace) for page scripts, logical float and clear sides, two display contents fixes, an IME composition underline painted in every document with a preedit, and deferred text layout for the textarea.
Two hand edits: the WPT workflow's merge conflict, resolved by keeping the SHA-pinned deploy-pages action together with upstream's per-run-attempt Pages artifact name and both repository guards; and the RUSTSEC-2026-0192 advisory ignore, whose reason now names the residual row that drops it and says the audit leg re-checks it on every push.
No dependency, lockfile, crate, workspace member, IPC method, endpoint, port, socket or environment variable changed. Dioxus stays held at 0.7.3 and was re-read at this sync. Upstream's AGENTS.md files stay out of the tree.
The upstream-sync position moved from 547d7514b to 165d698c3, the left-out set went from 14 to 15 commits, and blitz-tests gained one target, ime_preedit_underline. CI was green 17 of 17 and the audit leg reported advisories ok and licenses ok.
@task_file n=1 path=.github/workflows/wpt.yml mark=conflict
what: the WPT workflow, upstream-only: its publish and Pages deploy job behind the github.repository guard, its actions pinned to commit SHAs; resolved a conflict in it, kept the pinned deploy-pages action with upstream's per-run-attempt artifact name and both repository guards
@task_file n=2 path=deny.toml
what: the cargo-deny audit and licence configuration, its advisory ignores by ID each with a written reason; edited the RUSTSEC-2026-0192 ignore's reason to name the residual row that drops it and the audit leg's re-check on every push
@task_file n=3 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout box construction over page markup, including anonymous blocks, display contents children and the textarea's text layout; merged into from upstream
@task_file n=4 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the engine's incremental layout damage, the relayout of changed nodes including the textarea's deferred text layout; merged into from upstream
@task_file n=5 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the engine's inline layout building from page markup, inline boxes and floats; merged into from upstream
@task_file n=6 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout, inline-box heights and float placement by logical sides; merged into from upstream
@task_file n=7 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the engine's document node, its element data and block containment through display contents; merged into from upstream
@task_file n=8 path=packages/blitz-dom/src/traversal.rs mark=merge
what: the engine's document tree traversal helpers, the children-and-pseudo-elements iteration; merged into from upstream, one upstream macro removed
@task_file n=9 path=packages/blitz-paint/src/text.rs mark=merge
what: the engine's text paint, including input and textarea text, the password bullets and now the IME composition underline; merged into from upstream
@task_file n=10 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the engine's text decoration paint, now holding the composition underline decoration; merged into from upstream
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the JavaScript bootstrap script defining the DOM interfaces page scripts see, alongside the hidden native helpers; merged into from upstream, adding DOMTokenList replace
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script runtime's DOM API tests, now including classList replace; merged into from upstream
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-layout conversion, float and clear now converted by logical side with a direction flag; merged into from upstream
@task_file n=14 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the style-to-layout wrapper that reads computed styles for layout, the one caller of the float and clear conversion; merged into from upstream
@task_file n=15 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests combined layout test target's module list; merged into from upstream
@task_file n=16 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the blitz-tests display contents layout tests; merged into from upstream
@task_file n=17 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: a new blitz-tests target checking that composing IME text paints an underline until committed; added by the merge from upstream
@task_file n=18 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT test-suite revision the WPT runner checks out; merged into from upstream
@task_file n=19 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: the WPT post-results script run by the trusted workflow, its PR lookup by head sha and marker-bounded splice of the results comment; merged into from upstream, now showing results beside a revision mismatch
@task_file n=20 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the tests of the WPT post-results script; merged into from upstream
@task_file n=21 path=AGENTS.md mark=conflict
what: upstream's agent instructions file, kept out of the fork's tree; resolved its modify and delete conflict by deleting it, so the committed tree is unchanged
@task_item n=1 part=touches
The advisory ignore rules, both the ignores by ID only and the per-ID ignores with a written reason: the reason on the RUSTSEC-2026-0192 ignore was rewritten to name the residual row that drops it once a winit 0.31 beta chain release leaves ttf-parser 0.25.1. The ignore itself and its ID are unchanged.
@task_item n=2 part=touches
The dependency audit CI gate and cargo-deny as the audit tool, plus the dependency-security overview's line that the audit leg reads the lockfile's advisory state on every push: the ignore's new reason relies on that per-push re-check. The audit passed with advisories ok and licenses ok, and the one NCSA licence-not-encountered warning is the same as at the last sync.
@task_item n=3 part=touches
CI actions pinned to commit SHAs: the WPT workflow's merge conflict was resolved by keeping the SHA-pinned deploy-pages action over upstream's form.
@task_item n=4 part=touches
The publish job repository guard, and the rules that publishing and secrets are reachable only upstream: both github.repository guards in the WPT workflow were kept through the conflict, and the Pages deploy step now names its artifact per run attempt.
@task_item n=5 part=touches
The decision that the WPT post-results workflow runs trusted scripts, and WPT PR lookup by head sha with a marker-bounded splice: upstream changed the post-results script to show results beside a revision mismatch.
@task_item n=6 part=touches
Read-only CI tokens: the WPT workflow file changed, though the merge recorded no permission change in it.
@task_item n=7 part=touches
Upstream sync takes the dependency set whole: this sync took ten commits and changed no dependency at all, with the manifest and lockfile byte-equal to the parent.
@task_item n=8 part=touches
Engine dependency pins and locked builds: Dioxus was re-read and held at 0.7.3 (lock 0.7.10) on the founder's ruling. The next re-read is at the next upstream sync or at the first sync after a Dioxus 0.8 release, and the locked build passed.
@task_item n=9 part=touches
The rules over the JavaScript bootstrap script, namely that the bootstrap native helper and the native search-params parser stay hidden from pages and that URLSearchParams goes through a bootstrap script: the bootstrap's DOM interfaces gained DOMTokenList replace for page scripts. No native helper was exposed.
@task_item n=10 part=touches
JavaScript opt-in in the browser app: the new classList replace is reachable only by page scripts, so only where JavaScript is enabled.
@task_item n=11 part=touches
Page and stylesheet bytes decoded without panic, and inline-box heights kept finite: upstream changed layout over hostile markup, with floats placed by logical side, block containment and box construction through display contents, and deferred textarea text layout.
@task_item n=12 part=touches
A dropped anonymous block clears its children's parent: upstream rewrote how box construction sorts children into inline and block flow under display contents.
@task_item n=13 part=touches
Password input painted as bullets or masked, and the password caret placed by the bullets: the text paint they rule over now also paints a composition underline under IME preedit text in any input.
@task_item n=14 part=touches
Stale node ids never panic: an upstream traversal macro was removed and its one use replaced by the children-and-pseudo-elements iteration.
@task_item n=15 part=touches
Accessibility focus and inert-subtree rules that walk display contents subtrees: upstream changed how a display contents node counts as containing a block. Focus kept by a display contents control is unchanged.
@task_fact n=1 slug=input-validation
fact: Page scripts get DOMTokenList replace (classList.replace) through the script bootstrap's DOM interfaces, taken from upstream as written on the founder's ruling. It adds no input surface beyond page script.
@task_fact n=2 slug=input-validation
fact: The engine paints an underline under IME composition text in every document with a preedit, until the text is committed. Upstream's change was taken as written on the founder's ruling. It is engine paint only, with no new input channel and no log.
@task_fact n=3 slug=dependency-security
fact: The RUSTSEC-2026-0192 advisory ignore's reason names the residual row that owns dropping it, once a winit 0.31 beta chain release leaves ttf-parser 0.25.1, and the audit leg re-checks it on every push.
@task_item n=1 part=untouched
The driver session socket, its request grammar, replies, bounds and lifecycle: the session added no IPC method, socket or connection rule.
@task_item n=2 part=untouched
The MCP server over stdio, its requests, results, errors and tool spans: none changed.
@task_item n=3 part=untouched
The driver verbs, argument kinds, refusal causes, the actions, graph and layout answers, and the stable element id contracts: no verb or answer moved.
@task_item n=4 part=untouched
Snapshot text, state words, masked values and the value mask on every carrier: no carrier or masked field changed.
@task_item n=5 part=untouched
Recording, kept checks, cold-agent runs and their owner-only directories: none were reached.
@task_item n=6 part=untouched
Environment variable contracts and the driver's environment reads: no environment variable was added or read.
@task_item n=7 part=untouched
Data protection, namely history in sqlite, cookies, the HTTP cache, TLS through rustls, the clipboard and the file dialog: none changed.
@task_item n=8 part=untouched
Outbound HTTP, per-host limits, script fetch schemes and methods, file URL rules, and the absent CORS, same-origin, timeout and size limits: no network path changed.
@task_item n=9 part=untouched
Logging targets, the telemetry allowlist sink, span lines and the network tracing log: the sink sweep found zero new log sites.
@task_item n=10 part=untouched
Select list rules, range input, multiple select and the keyboard activation rules: no form-control behaviour beyond the textarea's text layout changed.
@task_item n=11 part=untouched
Authentication and authorization: the absences stand, and no auth code was added.
@task_item n=12 part=untouched
Error handling refusal sets, session error tables, CSSOM script errors and WPT runner result kinds: none moved.
