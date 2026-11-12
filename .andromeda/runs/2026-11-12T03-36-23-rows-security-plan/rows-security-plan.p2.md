@row_none n=1 rec=1 type=4
relied: A parse error in inline SVG source is logged and leaves the element without image data instead of failing the document.
@row_none n=2 rec=2 type=4
relied: Invalid CSS declarations are ignored and unsupported property names or invalid values refused, so no invalid declaration reaches style.
@row_none n=3 rec=3 type=4
relied: An invalid selector list leaves its rule unchanged, and query parse errors are returned to the caller rather than panicking.
@row_none n=4 rec=4 type=4
@row_none n=5 rec=5 type=4
relied: Legacy presentational attributes pass closed parsers, and any other value is ignored or rejected.
@row_add n=1 row=decision/select-size-hint-cap area=input-validation rec=6
name: Select size hint capped
rule: The `size` attribute on a `multiple` select is read as a hint capped at 65534, with `0` or a non-numeric value giving no hint.
why: Ratified as an engine change for every document and a boundary widening.
@row_add n=2 row=decision/attribute-readers-no-log-or-panic area=input-validation rec=7
name: Markup attribute readers without log or panic
rule: Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
@row_add n=3 row=decision/attribute-text-leaves-by-ratified-carriers area=input-validation rec=7
name: Attribute text leaves only by ratified carriers
rule: Markup attribute values, names and descriptions leave escher only by the ratified carriers.
except: The page's own form submission and events.
@row_add n=4 row=decision/select-list-keys-and-painted-labels area=input-validation rec=7
name: Single select list keys and painted labels
rule: Enter and Space open a single select's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.
why: Ratified as an engine change for every document and a boundary widening.
@row_add n=5 row=contract/stable-element-id/derivation area=input-validation rec=8
boundary: The stable element id
item: derivation
promise: The HTML `id` is the author key when non-empty, `/`-free and first in document order; otherwise an anchored, component or document path is used, so no `id` can make two ids equal.
@row_add n=6 row=contract/stable-element-id/carriers area=input-validation rec=8
boundary: The stable element id
item: carriers
promise: The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
@row_add n=7 row=contract/stable-element-id/no-node-id area=input-validation rec=8
boundary: The stable element id
item: NodeId
promise: No `NodeId` crosses any carrier of the stable element id.
@row_add n=8 row=decision/masked-input-values area=input-validation rec=9,39
name: Password and file input values masked
rule: A password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path.
except: The DOM and form submission keep the value.
@row_add n=9 row=decision/password-painted-as-bullets area=input-validation rec=9
name: Password input painted as bullets
rule: A password input paints one bullet per character, never its text.
@row_add n=10 row=contract/masked-input-value/masked-stand-in area=input-validation rec=9
boundary: Masked input values
item: MASKED_STAND_IN
promise: A recorded masked `type` is kept as `MASKED_STAND_IN`.
@row_none n=6 rec=10 type=4
relied: `text_extent` is a read-only reader returning geometry only.
@row_none n=7 rec=11 type=4
@row_none n=8 rec=12 type=4
relied: Scroll targets are clamped to the scroll range, and a programmatic scroll to a missing node does nothing.
@row_none n=9 rec=13 type=4
relied: A file input's `accept` is not applied as a filter on the files chosen.
@row_none n=10 rec=14 type=4
relied: Form data validity in dioxus-native always reports valid, so no form is ever held back as invalid.
@row_none n=11 rec=15 type=4
relied: Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized.
@row_add n=11 row=decision/reached-handler-record-without-log area=input-validation rec=16
name: Reached-handler record without log or NodeId
rule: Where recording of reached handlers is switched on, each reached handler's stable id and event name is written into a root-context record ahead of the handler's dispatch, with no log site and no `NodeId`.
@row_add n=12 row=decision/signal-registry-holds-no-state-value area=input-validation rec=16
name: Signal registry holds no state value
rule: The in-process signal registry takes dioxus-core types alone, holds no value of a state, and has no log site, print, env read or `unwrap` on app input.
@row_add n=13 row=decision/js-fetch-get-or-head-only area=input-validation rec=17,47
name: JS fetch limited to GET and HEAD
rule: JS-facing `fetch()` takes GET or HEAD only.
@row_add n=14 row=decision/sqlite-bound-parameters area=input-validation rec=18
name: SQLite values bound through placeholders
rule: Values written to the sqlite persistence are bound through `params!` placeholders, never spliced into SQL text.
@row_add n=15 row=decision/serialization-escapes-text-and-attributes area=input-validation rec=19
name: Serialization escapes text and attribute values
rule: Serialized output escapes text and attribute values.
except: Raw-text elements' text is written unescaped.
@row_add n=16 row=contract/serialization-output/inline-svg area=input-validation rec=19
boundary: Serialization output
item: inline SVG
promise: Inline SVG serializes as well-formed XML.
@row_none n=12 rec=20 type=4
relied: The test harness query helper panics on an invalid selector, while selector parsing itself returns a `Result`.
@row_none n=13 rec=21 type=4
relied: The WPT runner runs only JavaScript script types and escapes its wrapper HTML.
@row_none n=14 rec=22 type=4
relied: The CI PR lookup takes only an open PR whose head sha equals the run's, and the comment splice replaces only when both markers exist in order.
@row_none n=15 rec=23 type=4
@row_none n=16 rec=24 type=5
@row_none n=17 rec=25 type=S
@row_none n=18 rec=26 type=S
@row_add n=17 row=decision/tls-via-native-tls area=data-protection rec=27
name: TLS through native-tls
rule: TLS is provided through reqwest's `native-tls` feature, `native-tls-vendored` on Android.
@row_none n=19 rec=28 type=7
@row_none n=20 rec=29 type=S
@row_add n=18 row=decision/history-in-sqlite area=data-protection rec=30
name: Browser history stored in sqlite
rule: Browser history is written to a sqlite file; on mobile it is in-memory only and does not persist across launches.
@row_add n=19 row=absence/database-encryption area=data-protection rec=30
capability: Encryption of the history database
@row_add n=20 row=decision/private-on-disk-http-cache area=data-protection rec=31
name: Private on-disk HTTP cache
rule: With feature `cache`, HTTP responses are cached on disk at the platform cache directory, and the cache policy is evaluated as a private, not shared, cache.
@row_add n=21 row=decision/cookies-and-cache-default-features area=data-protection rec=31
name: Cookies and HTTP cache on by default
rule: Cookies and the HTTP cache are default features of the browser.
@row_add n=22 row=contract/kept-check-file/medium area=data-protection rec=32
boundary: Kept check files
item: medium
promise: Under `--record <dir>` or `Session::recording_into`, one `check-{n}.json` with mode `0600` per recorded session is written in a `0700` directory the launcher named.
@row_add n=23 row=contract/kept-check-file/contents area=data-protection rec=32
boundary: Kept check files
item: contents
promise: A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
@row_add n=24 row=contract/kept-check-file/lifecycle area=data-protection rec=32
boundary: Kept check files
item: lifecycle
promise: A kept check is never logged and never removed by escher, but for its own check when a recording ends.
@row_add n=25 row=decision/kept-checks-owner-only area=data-protection rec=32,33
name: Kept checks owner-only and uncommitted
rule: A kept check stays in an owner-only medium and is never committed.
except: The two committed checks of the repository's own scripted stand-Counter sessions.
@row_add n=26 row=decision/cold-agent-checks-read-for-counts-only area=data-protection rec=32
name: Cold-agent checks read for bools and counts only
rule: The cold-agent pipe's live child keeps its checks in an owner-only record directory, read by the pipe for bools and counts only, never printed, copied or committed, and removed by its cleanup.
@row_add n=27 row=decision/committed-counter-checks area=data-protection rec=33
name: Committed stand-Counter checks
rule: The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`.
why: The one exception to a kept check's owner-only medium.
@row_none n=21 rec=34 type=4
relied: Request URLs and the cache directory path appear in the logs.
@row_add n=28 row=decision/log-sink-redacts-and-allowlists area=data-protection rec=35
name: Log sink redacts and allowlists targets
rule: Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
@row_none n=22 rec=36 type=S
@row_add n=29 row=decision/clipboard-via-arboard area=data-protection rec=37
name: Clipboard through arboard
rule: Clipboard text is read and written through arboard on desktop OSes, behind the clipboard feature.
@row_add n=30 row=decision/file-dialog-via-rfd area=data-protection rec=38
name: Native file dialog through rfd
rule: The native file dialog uses rfd, with optional name and extension filters, behind the file-dialog feature.
@row_none n=23 rec=40 type=5
@row_none n=24 rec=41 type=S
@row_none n=25 rec=42 type=4
relied: Clear history deletes every history row both in memory and on disk, and the HTTP cache can be cleared on demand.
@row_none n=26 rec=43 type=S
@row_add n=31 row=absence/served-network-api area=api-security rec=44
capability: A served network API on any TCP or UDP port
@row_add n=32 row=overview/api-security area=api-security rec=44
text: No served network API exists: no binary serves a TCP or UDP port. The one listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The remaining controls govern outbound requests and in-document navigation.
@row_add n=33 row=decision/per-host-request-concurrency area=api-security rec=45
name: Outbound requests limited per host
rule: At most 6 outbound requests run concurrently per host.
@row_add n=34 row=absence/cors-checks area=api-security rec=45
capability: CORS checks on outbound requests
@row_add n=35 row=absence/same-origin-checks area=api-security rec=45
capability: Same-origin checks on outbound requests
@row_add n=36 row=absence/content-security-policy area=api-security rec=45
capability: Content Security Policy enforcement
@row_add n=37 row=absence/response-size-limit area=api-security rec=45
capability: A response size limit on outbound requests
@row_add n=38 row=absence/request-timeout area=api-security rec=45
capability: A request timeout on outbound requests
@row_add n=39 row=decision/bounded-load-wait area=api-security rec=46
name: Bounded wait on loads in flight
rule: A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`.
why: So the absent request timeout cannot hang a settle or a command.
@row_add n=40 row=decision/no-wait-verb-fields area=api-security rec=46
name: No wait argument or result on verbs
rule: No driver verb has an argument or a result field of a wait.
@row_add n=41 row=decision/subresource-requests-get-only area=api-security rec=47
name: Sub-resource requests are GETs
rule: Sub-resource requests are issued as GETs only.
@row_add n=42 row=decision/form-dialog-method-rejected area=api-security rec=47
name: Form submission by scheme and method
rule: Form submission goes by scheme and method, and the `dialog` method is rejected.
@row_add n=43 row=decision/iframe-navigation-to-parent area=api-security rec=47
name: Iframe navigations routed to the parent
rule: Iframe navigations route back to the parent document.
@row_add n=44 row=decision/external-targets-to-system-browser area=api-security rec=47
name: External targets open in the system browser
rule: Navigation to external targets goes to the system browser.
@row_add n=45 row=decision/dioxus-scheme-local-files area=api-security rec=47
name: Dioxus scheme served from local files
rule: The `dioxus` scheme is served from the local file system.
@row_add n=46 row=decision/dioxus-native-js-eval-noop area=api-security rec=47
name: Dioxus-native JS evaluation is a no-op
rule: Dioxus-native JS evaluation goes to a no-op document.
@row_add n=47 row=decision/outbound-http-via-reqwest-or-blitz-net area=api-security rec=48
name: Outbound HTTP through reqwest or blitz-net
rule: Outbound HTTP in the examples and apps goes through reqwest or blitz-net.
@row_add n=48 row=decision/ci-api-via-gh-cli area=api-security rec=48
name: CI API calls through the gh CLI
rule: CI API calls go through the `gh api` CLI with the workflow token.
@row_add n=49 row=decision/session-socket-owner-only area=api-security rec=49
name: Session socket reachable by its owner only
rule: The local session socket is reachable by the owning user only, a `0700` directory and a `0600` socket, a group- or other-open directory refused, with no handshake secret, token or peer-credential check.
@row_add n=50 row=decision/session-socket-one-bounded-connection area=api-security rec=49
name: One bounded connection at a time
rule: The local session socket serves one connection at a time under fixed read, write and size bounds.
@row_add n=51 row=decision/session-host-idle-expiry area=api-security rec=49
name: Session host idle expiry
rule: A session host that answers no request for its idle expiry ends as on `stop`.
@row_none n=27 rec=50 type=5
@row_none n=28 rec=51 type=S
@row_add n=52 row=decision/dependency-audit-cargo-deny area=dependency-security rec=52
name: Dependency audit with cargo-deny
rule: cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features on the six CI platforms, configured by the root `deny.toml`.
except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
@row_add n=53 row=decision/audit-ignores-per-id-with-reason area=dependency-security rec=52
name: Audit ignores per ID with a written reason
rule: Audit ignores are per advisory ID only, each with a written reason, no blanket allow, and each ignore is re-read on every push.
@row_none n=29 rec=53 type=S
@row_add n=54 row=decision/no-git-dependencies area=dependency-security rec=54
name: No git dependencies in the root manifest
rule: The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile.
@row_add n=55 row=decision/upstream-sync-dependency-set area=dependency-security rec=55
name: Upstream sync takes the dependency set whole
rule: Each upstream sync takes upstream's dependency set whole or, by cherry-pick, enters no package; its lock is read name by name against upstream's or the prior tree, and the audit leg is green after it.
