@row_add n=1 row=decision/security-tier area=threat-model-summary rec=fct-muh4yxj3@1
name: Security tier
rule: The project's security tier is 0.
@row_none n=1 rec=fct-ljb2gi6k@1 type=S
@row_add n=2 row=decision/remote-content-nesting-limits area=threat-model-summary rec=fct-d35urlbv@1
name: Remote content nesting limits
rule: Remote web content is treated as an attack vector: iframe and @import nesting depths are limited, and stale iframe responses are discarded by request id.
@row_add n=3 row=decision/javascript-opt-in area=threat-model-summary rec=fct-pmsbanho@1
name: JavaScript opt-in in the browser app
rule: In the browser app JavaScript runs only behind the non-default `javascript` feature.
@row_add n=4 row=decision/default-script-fetcher-local-only area=threat-model-summary rec=fct-pmsbanho@1,fct-qlpwd5pv@1
name: Default script fetcher takes local URLs only
rule: The default script fetcher takes only `file:` and `data:` URLs; a `file:` URL loads only for a `file:` document, and fetch(), a module import and a script src are each refused otherwise with a fetch error.
why: A narrowing of what a document's scripts may fetch.
@row_add n=5 row=absence/same-origin-cors-checks area=threat-model-summary rec=fct-pmsbanho@1,fct-qlpwd5pv@1
capability: Same-origin and CORS checks on script fetches
@row_none n=2 rec=fct-lsqilazd@1 type=5
@row_add n=6 row=decision/rdme-raw-html-passthrough area=threat-model-summary rec=fct-ibx5eqs6@1
name: rdme passes raw HTML through
rule: The rdme markdown renderer runs in unsafe mode, passing raw HTML in markdown through.
@row_none n=3 rec=fct-hqa62nne@1 type=4
@row_add n=7 row=decision/wpt-post-results-trusted-scripts area=threat-model-summary rec=fct-yhokldke@1
name: WPT post-results workflow runs trusted scripts
rule: The WPT post-results workflow runs only in the DioxusLabs/blitz repository, only for successful pull-request-triggered WPT runs, and checks out its scripts from the default branch.
@row_add n=8 row=decision/driver-session-unix-socket-only area=threat-model-summary rec=fct-z5ttw3ol@3
name: Driver session on one Unix socket
rule: The driver session binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and runs on unix only.
@row_add n=9 row=decision/driver-access-by-filesystem-permissions area=threat-model-summary rec=fct-z5ttw3ol@3,fct-pgi7ssst@1,fct-dx3vh2da@1
name: Driver session access by filesystem permissions
rule: The driver session's state directory is created 0700, a present one open to its group or others is refused, and the socket is 0600, so access to the session is the owning user's.
@row_add n=10 row=decision/driver-session-no-peer-authentication area=threat-model-summary rec=fct-z5ttw3ol@3,fct-dgkl4a64@3,fct-pgi7ssst@1
name: Driver session and MCP server carry no authentication
rule: The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check, and session start does not check the answering pid; the trust boundary is the owning user or the spawning client process.
@row_add n=11 row=contract/driver-session-reply/no-node-id area=threat-model-summary rec=fct-z5ttw3ol@3,fct-6hwcr6nz@3
boundary: driver session socket replies
item: NodeId
promise: No reply carries a NodeId.
@row_add n=12 row=contract/driver-session-reply/masked-control-values area=threat-model-summary rec=fct-z5ttw3ol@3,fct-6hwcr6nz@3
boundary: driver session socket replies
item: control values
promise: A password's or file input's control value is carried masked.
@row_add n=13 row=decision/mcp-server-stdio-only area=threat-model-summary rec=fct-dgkl4a64@3,fct-5sreoq7r@1
name: MCP server on stdio only
rule: The MCP server is a stdio child an MCP client spawns, speaking on the invoking user's stdin and stdout, with no listener, port or socket.
@row_add n=14 row=contract/mcp-result/masked-control-values area=threat-model-summary rec=fct-dgkl4a64@3
boundary: MCP server results
item: control values
promise: A password's or file input's control value is carried masked.
@row_add n=15 row=decision/mcp-results-client-responsibility area=threat-model-summary rec=fct-dgkl4a64@3
name: MCP results are the client's
rule: What an MCP client does with the results it receives, screenshots and lint findings among them, a model provider included, is the client's responsibility.
@row_none n=4 rec=fct-d5xic4hg@1 type=S
@row_add n=16 row=absence/application-authentication area=authentication-authorization rec=fct-tf5bybpm@1
capability: Application authentication or authorization
@row_none n=5 rec=fct-gmnbkajs@1 type=7
@row_add n=17 row=decision/cookie-store-behind-feature area=authentication-authorization rec=fct-dnpx5hao@1
name: Cookie store behind a feature
rule: The HTTP client keeps a cookie store only with the `cookies` feature.
@row_add n=18 row=decision/publish-job-repository-guard area=authentication-authorization rec=fct-dnpx5hao@1
name: Publish job repository guard
rule: The publish job is repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.
@row_none n=6 rec=fct-4savfwrc@1 type=5
@row_none n=7 rec=fct-7ek3dcs2@1 type=5
@row_none n=8 rec=fct-k24bagpo@1 type=5
@row_none n=9 rec=fct-mgfmrmcl@1 type=S
@row_none n=10 rec=fct-skfeoxqz@1 type=4
@row_add n=19 row=contract/agent-run-cli/2 area=input-validation rec=fct-g4xhi6op@1
boundary: agent-run.sh exit codes
item: 2
promise: A verb outside boot run status cleanup logs, a wrong argument count, or a selection other than stand, all or an existing test-file stem prints usage and exits 2 before any cargo call.
@row_add n=20 row=contract/cold-agent-cli/2 area=input-validation rec=fct-xl3lbcjg@1
boundary: cold-agent.sh exit codes
item: 2
promise: A verb outside run status cleanup logs, a wrong argument count or a task outside the allowlist stand exits 2 before any precondition or claude call.
@row_add n=21 row=contract/apt-install-cli/2 area=input-validation rec=fct-rbli3fgb@1
boundary: apt-install.sh exit codes
item: 2
promise: Package names and option values that do not match closed forms print usage and exit 2 before sudo, timeout, apt-get or dpkg runs.
@row_add n=22 row=decision/apt-install-no-env-no-secrets area=input-validation rec=fct-rbli3fgb@1
name: Install script holds no environment or secret
rule: The apt install script reads no environment variable of its own, writes no file and names no secret.
@row_add n=23 row=contract/coverage-floor-cli/2 area=input-validation rec=fct-6uhcznyg@1
boundary: coverage_floor.py exit codes
item: 2
promise: A floor not matching CRATE_DIR=PERCENT (a percent at most 100, no crate twice), or a crate with no SF record, exits 2.
@row_add n=24 row=decision/coverage-floor-prints-no-paths area=input-validation rec=fct-6uhcznyg@1
name: Coverage floor prints no paths
rule: The coverage floor check prints only crate directories and counts, never a source or lcov path.
@row_add n=25 row=decision/model-reply-untrusted-input area=input-validation rec=fct-atmkljiq@1
name: Model reply is untrusted input
rule: The cold-agent model's reply is untrusted: only its last fenced json block is read, bounded in bytes and steps, each step checked against closed keys; a check outside these is never replayed, and a valid one replays on fresh MCP children with every step validated again by the server.
@row_add n=26 row=decision/cold-agent-checks-never-printed area=input-validation rec=fct-atmkljiq@1
name: Cold-agent checks never printed
rule: The cold-agent check file is never printed, and the live child's kept checks are read for booleans and counts only and never printed.
@row_add n=27 row=decision/paged-answer-closed-decoding area=input-validation rec=fct-atmkljiq@1
name: Paged answer decoded by closed fields
rule: The cold-agent check decodes a paged answer only as exactly the fields text, part and parts.
@row_add n=28 row=decision/driver-socket-closed-grammar area=input-validation rec=fct-6hwcr6nz@3
name: Driver socket closed request grammar
rule: The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
@row_add n=29 row=contract/driver-session-reply/no-attribute-text area=input-validation rec=fct-6hwcr6nz@3
boundary: driver session socket replies
item: attribute text
promise: A reply never carries an attribute's text.
@row_add n=30 row=contract/driver-session-reply/state-readings area=input-validation rec=fct-6hwcr6nz@3
boundary: driver session socket replies
item: state readings
promise: State readings are carried as closed words and booleans.
@row_add n=31 row=decision/boundary-widening-rule area=input-validation rec=fct-n4q5kc73@1
name: Boundary widening rule
rule: A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
@row_add n=32 row=decision/state-directory-long-path-refused area=input-validation rec=fct-dx3vh2da@1
name: Too-long state directory path refused
rule: A state directory path too long for a socket address is refused with a fixed message holding no path.
@row_add n=33 row=decision/state-directory-non-recursive-delete area=input-validation rec=fct-dx3vh2da@1
name: Non-recursive session cleanup
rule: Only the socket file and then the state directory are ever deleted, non-recursively.
@row_add n=34 row=decision/state-directory-path-is-parameter area=input-validation rec=fct-dx3vh2da@1
name: State directory path is a parameter
rule: The state directory path is a parameter, never an environment read or a working-directory-relative default.
@row_add n=35 row=decision/recording-directory-owner-only area=input-validation rec=fct-klizv3ng@1
name: Owner-only recording directory
rule: The recording directory is created 0700, a group- or other-open one is refused with nothing written or spawned, each check is created 0600 and never replaced, and recording is unix only.
@row_add n=36 row=decision/recording-ends-on-failure area=input-validation rec=fct-klizv3ng@1
name: Recording ends on failure
rule: A failed write or a passed bound ends the recording.
@row_add n=37 row=decision/recorder-removes-only-own-files area=input-validation rec=fct-klizv3ng@1
name: Recorder removes only its own files
rule: A file the recorder did not create is never removed.
@row_add n=38 row=decision/recording-directory-never-named area=input-validation rec=fct-klizv3ng@1
name: Recording directory never named or logged
rule: No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
@row_add n=39 row=decision/recorded-check-untrusted area=input-validation rec=fct-khxrpkj3@2
name: Recorded check bytes are untrusted
rule: A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
@row_add n=40 row=contract/driver-session-label/format area=input-validation rec=fct-76426k6u@1
boundary: driver session label
item: format
promise: A label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs.
@row_add n=41 row=decision/session-label-only-caller-string area=input-validation rec=fct-76426k6u@1
name: Session label is the one echoed caller string
rule: The session label is never text read from the document and is the one caller string a reply carries.
@row_add n=42 row=decision/single-validation-gate area=input-validation rec=fct-cvyxuuhn@3,fct-5sreoq7r@1
name: Single command validation gate
rule: Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
@row_add n=43 row=decision/unknown-target-refused area=input-validation rec=fct-cvyxuuhn@3
name: Call on an unknown or unfit element refused
rule: A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
@row_add n=44 row=decision/command-schema-no-clock area=input-validation rec=fct-vehorezx@1
name: Command schema reads no clock
rule: The command schema and its validation read no clock.
@row_add n=45 row=decision/no-node-id-between-calls area=input-validation rec=fct-vehorezx@1
name: No NodeId kept between calls
rule: No NodeId is kept between calls.
@row_add n=46 row=decision/command-span-content-free area=input-validation rec=fct-vehorezx@1
name: Command span holds no content
rule: One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
@row_add n=47 row=decision/driver-no-subscriber-no-env area=input-validation rec=fct-vehorezx@1,fct-4nuyglo3@1
name: Driver installs no subscriber and reads no env
rule: The driver crate and its session command line install no tracing subscriber and read no environment variable.
@row_add n=48 row=decision/cli-closed-argv-reader area=input-validation rec=fct-4nuyglo3@1
name: Closed argv reader
rule: The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
@row_add n=49 row=decision/cli-echoes-nothing-typed area=input-validation rec=fct-4nuyglo3@1
name: Command line echoes nothing typed
rule: No command-line ending writes back anything the caller typed, and no path on argv panics.
@row_add n=50 row=contract/mcp-server/transport area=input-validation rec=fct-5sreoq7r@1
boundary: MCP server
item: transport
promise: Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.
@row_add n=51 row=contract/mcp-server/protocol-error area=input-validation rec=fct-5sreoq7r@1
boundary: MCP server
item: protocol error
promise: A protocol error is a fixed code and message with no data.
@row_add n=52 row=contract/mcp-server/request-id area=input-validation rec=fct-5sreoq7r@1
boundary: MCP server
item: request id
promise: The admitted request id is the one client-sent value written back.
@row_add n=53 row=decision/mcp-bounded-lines area=input-validation rec=fct-5sreoq7r@1
name: Bounded MCP requests and results
rule: MCP request lines and results are bounded, and tool arguments are typed before validation.
@row_add n=54 row=decision/mcp-no-print-or-panic area=input-validation rec=fct-5sreoq7r@1
name: MCP server never prints or panics
rule: The MCP server has no print, unwrap, expect or panic outside the tests.
@row_none n=11 rec=fct-mko7xxip@1 type=4
@row_add n=55 row=decision/external-navigation-schemes area=input-validation rec=fct-bklgnkmy@1
name: External navigation limited
rule: Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
@row_none n=12 rec=fct-muwas3xf@1 type=4
@row_none n=13 rec=fct-c6ebacq5@1 type=4
@row_none n=14 rec=fct-uvqi5pjd@1 type=4
@row_none n=15 rec=fct-ofv5h6kr@1 type=4
@row_none n=16 rec=fct-tp3nl2za@1 type=4
@row_none n=17 rec=fct-iuhcm6lf@1 type=4
@row_none n=18 rec=fct-bjb5m6i6@1 type=4
@row_add n=56 row=decision/html-parsed-scripting-disabled area=input-validation rec=fct-hfzht5qa@1
name: HTML parsed with scripting disabled
rule: HTML is parsed with scripting disabled.
@row_none n=19 rec=fct-mp7lxj2l@1 type=4
@row_none n=20 rec=fct-4mm5bgqu@1 type=4
@row_none n=21 rec=fct-lg2pk5x3@1 type=4
@row_none n=22 rec=fct-uy4tgw3u@1 type=4
@row_none n=23 rec=fct-wyy55z4s@1 type=4
@row_none n=24 rec=fct-56elu7kz@1 type=4
@row_add n=57 row=decision/no-agent-urls-through-font-path area=input-validation rec=fct-tqsve6xl@1
name: No agent URL through font sources
rule: escher routes no agent- or user-supplied URL through the font-face source path.
@row_none n=25 rec=fct-ismuupie@1 type=4
@row_none n=26 rec=fct-g4ds76xh@1 type=4
@row_none n=27 rec=fct-cq2qgcv5@1 type=4
relied: A parse error in inline SVG source is logged and leaves the element without image data instead of failing the document.
@row_none n=28 rec=fct-um6i723w@1 type=4
relied: Invalid CSS declarations are ignored and unsupported property names or invalid values refused, so no invalid declaration reaches style.
@row_none n=29 rec=fct-uzjgvwv6@1 type=4
relied: An invalid selector list leaves its rule unchanged, and query parse errors are returned to the caller rather than panicking.
@row_none n=30 rec=fct-femqe25a@1 type=4
@row_none n=31 rec=fct-5wlhxhdv@1 type=4
relied: Legacy presentational attributes pass closed parsers, and any other value is ignored or rejected.
@row_add n=58 row=decision/select-size-hint-cap area=input-validation rec=fct-w5f54hx5@1
name: Select size hint capped
rule: The `size` attribute on a `multiple` select is read as a hint capped at 65534, with `0` or a non-numeric value giving no hint.
why: Ratified as an engine change for every document and a boundary widening.
@row_add n=59 row=decision/attribute-readers-no-log-or-panic area=input-validation rec=fct-gtdrmcll@1
name: Markup attribute readers without log or panic
rule: Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
@row_add n=60 row=decision/attribute-text-leaves-by-ratified-carriers area=input-validation rec=fct-gtdrmcll@1
name: Attribute text leaves only by ratified carriers
rule: Markup attribute values, names and descriptions leave escher only by the ratified carriers.
except: The page's own form submission and events.
@row_add n=61 row=decision/select-list-keys-and-painted-labels area=input-validation rec=fct-gtdrmcll@1
name: Single select list keys and painted labels
rule: Enter and Space open a single select's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.
why: Ratified as an engine change for every document and a boundary widening.
@row_add n=62 row=contract/stable-element-id/derivation area=input-validation rec=fct-4jl4t3ye@1
boundary: The stable element id
item: derivation
promise: The HTML `id` is the author key when non-empty, `/`-free and first in document order; otherwise an anchored, component or document path is used, so no `id` can make two ids equal.
@row_add n=63 row=contract/stable-element-id/carriers area=input-validation rec=fct-4jl4t3ye@1
boundary: The stable element id
item: carriers
promise: The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
@row_add n=64 row=contract/stable-element-id/no-node-id area=input-validation rec=fct-4jl4t3ye@1
boundary: The stable element id
item: NodeId
promise: No `NodeId` crosses any carrier of the stable element id.
@row_add n=65 row=decision/masked-input-values area=input-validation rec=fct-767qdy6s@1,fct-mkaeodr2@1
name: Password and file input values masked
rule: A password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path.
except: The DOM and form submission keep the value.
@row_add n=66 row=decision/password-painted-as-bullets area=input-validation rec=fct-767qdy6s@1
name: Password input painted as bullets
rule: A password input paints one bullet per character, never its text.
@row_add n=67 row=contract/masked-input-value/masked-stand-in area=input-validation rec=fct-767qdy6s@1
boundary: Masked input values
item: MASKED_STAND_IN
promise: A recorded masked `type` is kept as `MASKED_STAND_IN`.
@row_none n=32 rec=fct-ht5tidu5@1 type=4
relied: `text_extent` is a read-only reader returning geometry only.
@row_none n=33 rec=fct-vxx7z3uu@1 type=4
@row_none n=34 rec=fct-bh2jhziy@1 type=4
relied: Scroll targets are clamped to the scroll range, and a programmatic scroll to a missing node does nothing.
@row_none n=35 rec=fct-34fqtojx@1 type=4
relied: A file input's `accept` is not applied as a filter on the files chosen.
@row_none n=36 rec=fct-66vufcs4@1 type=4
relied: Form data validity in dioxus-native always reports valid, so no form is ever held back as invalid.
@row_none n=37 rec=fct-qk7naa6l@1 type=4
relied: Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized.
@row_add n=68 row=decision/reached-handler-record-without-log area=input-validation rec=fct-frlob5k5@2
name: Reached-handler record without log or NodeId
rule: Where recording of reached handlers is switched on, each reached handler's stable id and event name is written into a root-context record ahead of the handler's dispatch, with no log site and no `NodeId`.
@row_add n=69 row=decision/signal-registry-holds-no-state-value area=input-validation rec=fct-frlob5k5@2
name: Signal registry holds no state value
rule: The in-process signal registry takes dioxus-core types alone, holds no value of a state, and has no log site, print, env read or `unwrap` on app input.
@row_add n=70 row=decision/js-fetch-get-or-head-only area=input-validation rec=fct-ogo4rqfs@1,fct-7xstfz5r@1
name: JS fetch limited to GET and HEAD
rule: JS-facing `fetch()` takes GET or HEAD only.
@row_add n=71 row=decision/sqlite-bound-parameters area=input-validation rec=fct-urxev64g@1
name: SQLite values bound through placeholders
rule: Values written to the sqlite persistence are bound through `params!` placeholders, never spliced into SQL text.
@row_add n=72 row=decision/serialization-escapes-text-and-attributes area=input-validation rec=fct-ss2oxfgi@1
name: Serialization escapes text and attribute values
rule: Serialized output escapes text and attribute values.
except: Raw-text elements' text is written unescaped.
@row_add n=73 row=contract/serialization-output/inline-svg area=input-validation rec=fct-ss2oxfgi@1
boundary: Serialization output
item: inline SVG
promise: Inline SVG serializes as well-formed XML.
@row_none n=38 rec=fct-ydvrpbvz@1 type=4
relied: The test harness query helper panics on an invalid selector, while selector parsing itself returns a `Result`.
@row_none n=39 rec=fct-b6vsgrbb@1 type=4
relied: The WPT runner runs only JavaScript script types and escapes its wrapper HTML.
@row_none n=40 rec=fct-zwiprjdk@1 type=4
relied: The CI PR lookup takes only an open PR whose head sha equals the run's, and the comment splice replaces only when both markers exist in order.
@row_none n=41 rec=fct-qwmipbkb@1 type=4
@row_none n=42 rec=fct-eazpnfag@1 type=5
@row_none n=43 rec=fct-gnsraz7a@1 type=S
@row_none n=44 rec=fct-5kpsayi6@1 type=S
@row_add n=74 row=decision/tls-via-native-tls area=data-protection rec=fct-aszkhz5z@1
name: TLS through native-tls
rule: TLS is provided through reqwest's `native-tls` feature, `native-tls-vendored` on Android.
@row_none n=45 rec=fct-gj3e32rb@1 type=7
@row_none n=46 rec=fct-h6kn5x4s@1 type=S
@row_add n=75 row=decision/history-in-sqlite area=data-protection rec=fct-brg4y6b3@1
name: Browser history stored in sqlite
rule: Browser history is written to a sqlite file; on mobile it is in-memory only and does not persist across launches.
@row_add n=76 row=absence/database-encryption area=data-protection rec=fct-brg4y6b3@1
capability: Encryption of the history database
@row_add n=77 row=decision/private-on-disk-http-cache area=data-protection rec=fct-73ojmpsr@1
name: Private on-disk HTTP cache
rule: With feature `cache`, HTTP responses are cached on disk at the platform cache directory, and the cache policy is evaluated as a private, not shared, cache.
@row_add n=78 row=decision/cookies-and-cache-default-features area=data-protection rec=fct-73ojmpsr@1
name: Cookies and HTTP cache on by default
rule: Cookies and the HTTP cache are default features of the browser.
@row_add n=79 row=contract/kept-check-file/medium area=data-protection rec=fct-kdtkdrex@1
boundary: Kept check files
item: medium
promise: Under `--record <dir>` or `Session::recording_into`, one `check-{n}.json` with mode `0600` per recorded session is written in a `0700` directory the launcher named.
@row_add n=80 row=contract/kept-check-file/contents area=data-protection rec=fct-kdtkdrex@1
boundary: Kept check files
item: contents
promise: A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
@row_add n=81 row=contract/kept-check-file/lifecycle area=data-protection rec=fct-kdtkdrex@1
boundary: Kept check files
item: lifecycle
promise: A kept check is never logged and never removed by escher, but for its own check when a recording ends.
@row_add n=82 row=decision/cold-agent-checks-read-for-counts-only area=data-protection rec=fct-kdtkdrex@1
name: Cold-agent checks read for bools and counts only
rule: The cold-agent pipe's live child keeps its checks in an owner-only record directory, read by the pipe for bools and counts only, never printed, copied or committed, and removed by its cleanup.
@row_add n=83 row=decision/committed-counter-checks area=data-protection rec=fct-sw7lpzl2@1
name: Committed stand-Counter checks
rule: The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`.
why: The one exception to a kept check's owner-only medium.
@row_none n=47 rec=fct-63f2oo75@1 type=4
relied: Request URLs and the cache directory path appear in the logs.
@row_add n=84 row=decision/log-sink-redacts-and-allowlists area=data-protection rec=fct-mbxzqngq@1
name: Log sink redacts and allowlists targets
rule: Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
@row_none n=48 rec=fct-oamkovdl@1 type=S
@row_add n=85 row=decision/clipboard-via-arboard area=data-protection rec=fct-j52pcemz@1
name: Clipboard through arboard
rule: Clipboard text is read and written through arboard on desktop OSes, behind the clipboard feature.
@row_add n=86 row=decision/file-dialog-via-rfd area=data-protection rec=fct-e6uumj43@1
name: Native file dialog through rfd
rule: The native file dialog uses rfd, with optional name and extension filters, behind the file-dialog feature.
@row_none n=49 rec=fct-l5wivwzn@1 type=5
@row_none n=50 rec=fct-jac45bwt@1 type=S
@row_none n=51 rec=fct-ij663iad@1 type=4
relied: Clear history deletes every history row both in memory and on disk, and the HTTP cache can be cleared on demand.
@row_none n=52 rec=fct-kxutn4hq@1 type=S
@row_add n=87 row=overview/api-security area=api-security rec=fct-s4jqcqqv@1
text: No served network API exists: no binary serves a TCP or UDP port. The one listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The remaining controls govern outbound requests and in-document navigation.
@row_add n=88 row=decision/per-host-request-concurrency area=api-security rec=fct-5zz6ux3p@1
name: Outbound requests limited per host
rule: At most 6 outbound requests run concurrently per host.
@row_add n=89 row=absence/cors-checks area=api-security rec=fct-5zz6ux3p@1
capability: CORS checks on outbound requests
@row_add n=90 row=absence/same-origin-checks area=api-security rec=fct-5zz6ux3p@1
capability: Same-origin checks on outbound requests
@row_add n=91 row=absence/response-size-limit area=api-security rec=fct-5zz6ux3p@1
capability: A response size limit on outbound requests
@row_add n=92 row=decision/bounded-load-wait area=api-security rec=fct-dlhusvs4@1
name: Bounded wait on loads in flight
rule: A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`.
why: So the absent request timeout cannot hang a settle or a command.
@row_add n=93 row=decision/no-wait-verb-fields area=api-security rec=fct-dlhusvs4@1
name: No wait argument or result on verbs
rule: No driver verb has an argument or a result field of a wait.
@row_add n=94 row=decision/subresource-requests-get-only area=api-security rec=fct-7xstfz5r@1
name: Sub-resource requests are GETs
rule: Sub-resource requests are issued as GETs only.
@row_add n=95 row=decision/form-dialog-method-rejected area=api-security rec=fct-7xstfz5r@1
name: Form submission by scheme and method
rule: Form submission goes by scheme and method, and the `dialog` method is rejected.
@row_add n=96 row=decision/iframe-navigation-to-parent area=api-security rec=fct-7xstfz5r@1
name: Iframe navigations routed to the parent
rule: Iframe navigations route back to the parent document.
@row_add n=97 row=decision/external-targets-to-system-browser area=api-security rec=fct-7xstfz5r@1
name: External targets open in the system browser
rule: Navigation to external targets goes to the system browser.
@row_add n=98 row=decision/dioxus-scheme-local-files area=api-security rec=fct-7xstfz5r@1
name: Dioxus scheme served from local files
rule: The `dioxus` scheme is served from the local file system.
@row_add n=99 row=decision/dioxus-native-js-eval-noop area=api-security rec=fct-7xstfz5r@1
name: Dioxus-native JS evaluation is a no-op
rule: Dioxus-native JS evaluation goes to a no-op document.
@row_add n=100 row=decision/outbound-http-via-reqwest-or-blitz-net area=api-security rec=fct-dsoqkdxe@1
name: Outbound HTTP through reqwest or blitz-net
rule: Outbound HTTP in the examples and apps goes through reqwest or blitz-net.
@row_add n=101 row=decision/ci-api-via-gh-cli area=api-security rec=fct-dsoqkdxe@1
name: CI API calls through the gh CLI
rule: CI API calls go through the `gh api` CLI with the workflow token.
@row_add n=102 row=decision/session-socket-one-bounded-connection area=api-security rec=fct-sn4avmoz@1
name: One bounded connection at a time
rule: The local session socket serves one connection at a time under fixed read, write and size bounds.
@row_add n=103 row=decision/session-host-idle-expiry area=api-security rec=fct-sn4avmoz@1
name: Session host idle expiry
rule: A session host that answers no request for its idle expiry ends as on `stop`.
@row_none n=53 rec=fct-kcaaghma@1 type=5
@row_none n=54 rec=fct-xqvxiqdb@1 type=S
@row_add n=104 row=decision/audit-ignores-per-id-with-reason area=dependency-security rec=fct-cappgruj@1
name: Audit ignores per ID with a written reason
rule: Audit ignores are per advisory ID only, each with a written reason, no blanket allow, and each ignore is re-read on every push.
@row_none n=55 rec=fct-7bl5xu2z@1 type=S
@row_add n=105 row=decision/no-git-dependencies area=dependency-security rec=fct-hhqzsgtc@1
name: No git dependencies in the root manifest
rule: The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile.
@row_add n=106 row=decision/upstream-sync-dependency-set area=dependency-security rec=fct-p7wg5ump@1
name: Upstream sync takes the dependency set whole
rule: Each upstream sync takes upstream's dependency set whole or, by cherry-pick, enters no package; its lock is read name by name against upstream's or the prior tree, and the audit leg is green after it.
@row_add n=107 row=decision/locked-builds area=dependency-security rec=fct-kwog532k@1,fct-ehhtk64b@1
name: Locked builds
rule: Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified.
except: examples/wasm_hello, a standalone workspace with no Cargo.lock
@row_add n=108 row=decision/workspace-inherited-dependency-versions area=dependency-security rec=fct-wn5xussh@1
name: Workspace-inherited dependency versions
rule: Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
@row_none n=56 rec=fct-ms33hdhp@1 type=5
@row_none n=57 rec=fct-jgw23mmw@1 type=6
@row_add n=109 row=decision/serde-json-untrusted-input-reader area=dependency-security rec=fct-ldu2bkhj@1
name: serde_json as the untrusted input reader
rule: serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.
@row_add n=110 row=decision/png-shot-encoder area=dependency-security rec=fct-ei3z3lut@1
name: png as the shot encoder
rule: The png crate is the encoder of a shot in escher-driver.
@row_add n=111 row=decision/color-crate-luminance-source area=dependency-security rec=fct-qbfed6vz@1
name: color crate as luminance source
rule: The test harness's contrast reader takes its luminance from the color crate already in the graph through blitz-dom, adding no package to Cargo.lock.
@row_add n=112 row=decision/test-harness-no-tracing-no-style area=dependency-security rec=fct-qbfed6vz@1
name: Test harness takes no tracing and no style
rule: blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
@row_add n=113 row=decision/ci-actions-pinned-to-sha area=dependency-security rec=fct-4wzxuf2u@1
name: CI actions pinned to commit SHAs
rule: Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment.
guard: test_ci_workflows.py
@row_add n=114 row=decision/cargo-deny-audit-tool area=dependency-security rec=fct-4wzxuf2u@1,fct-cfff7mts@1
name: cargo-deny as the audit tool
rule: cargo-deny 0.20.2, with the root deny.toml, is the dependency audit tool.
@row_add n=115 row=decision/cargo-llvm-cov-coverage-tool area=dependency-security rec=fct-4wzxuf2u@1
name: cargo-llvm-cov as the coverage tool
rule: cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
@row_none n=58 rec=fct-l3h5hlj3@1 type=5
@row_add n=116 row=decision/blitz-tests-not-published area=dependency-security rec=fct-44piipwj@1
name: blitz-tests not published
rule: The blitz-tests crate is never published (publish = false).
@row_add n=117 row=absence/automated-dependency-updates area=dependency-security rec=fct-ivgzdi2j@1
capability: Automated dependency update tooling, such as dependabot
@row_add n=118 row=decision/dependency-audit-ci-gate area=dependency-security rec=fct-6cupfjou@1,fct-v457huyo@1
name: Dependency audit CI gate
rule: The CI audit job runs the dependency audit on every pull request and push, in the slow tier behind the fast jobs, uncached, its log uploaded on failure.
@row_none n=59 rec=fct-yvy6uwir@1 type=5
@row_add n=119 row=absence/signed-artifacts area=dependency-security rec=fct-eaz6kbxo@1
capability: Signed release artifacts produced by the fork
@row_add n=120 row=decision/signing-material-removed-after-build area=dependency-security rec=fct-eaz6kbxo@1,fct-gjoxy46h@1
name: Signing material removed after the build
rule: A signing key or keystore written from secrets for a build is removed in an always-run step.
@row_none n=60 rec=fct-ovf2eb7i@1 type=5
@row_none n=61 rec=fct-2cgb5j27@1 type=S
@row_add n=121 row=decision/telemetry-sink-target-allowlist area=bootstrap-phases-derive-for-route-setup-project rec=fct-eh3265j5@1
name: Telemetry sink target allowlist
rule: escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level.
except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed
@row_none n=62 rec=fct-pxydvv3o@1 type=S
@row_none n=63 rec=fct-gpqv3vsg@1 type=S
@row_add n=122 row=decision/ci-secrets-in-github-actions area=secret-management rec=fct-gjoxy46h@1
name: CI secrets in GitHub Actions
rule: Production secrets live in GitHub Actions secrets and vars and are passed only to the steps that use them.
@row_add n=123 row=decision/live-evidence-holds-no-credentials area=secret-management rec=fct-jtwyhkf2@1,fct-ipi52teh@1
name: Live evidence holds no credentials
rule: No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
@row_none n=64 rec=fct-p7pp73ko@1 type=7
@row_none n=65 rec=fct-77bajbne@1 type=S
@row_none n=66 rec=fct-4mvsrpxb@1 type=5
@row_none n=67 rec=fct-f54sui4c@1 type=5
@row_none n=68 rec=fct-wztu7kmt@1 type=5
@row_none n=69 rec=fct-hsmt2k5u@1 type=5
@row_none n=70 rec=fct-x42gr5q3@1 type=5
@row_add n=124 row=contract/environment-variables/rust-log area=secret-management rec=fct-kpqtgbic@1
boundary: Environment variables
item: RUST_LOG
promise: Holds the tracing filter directives escher-telemetry reads, defaulting to warn when unset or unparsable.
@row_add n=125 row=decision/driver-reads-only-rust-log area=secret-management rec=fct-pz4e3fuz@1
name: Driver reads only RUST_LOG
rule: The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.
@row_none n=71 rec=fct-wjxtmswg@1 type=5
@row_none n=72 rec=fct-ddawzqlr@1 type=5
@row_none n=73 rec=fct-q6ezxofh@1 type=S
@row_none n=74 rec=fct-c2hnlfa7@1 type=S
@row_none n=75 rec=fct-ecpelafu@1 type=4
relied: A failed fetch shows the user error.html with the error's Debug text in its error paragraph.
@row_none n=76 rec=fct-fcagkjyj@1 type=4
relied: Error pages are flagged is_error and skip history recording and favicon probing.
@row_add n=126 row=contract/rdme-exit-codes/1 area=error-handling rec=fct-itl5ofxf@1
boundary: rdme exit codes
item: 1
promise: The argument is neither URL nor file, or no README.md is found; a message is written to stderr.
@row_none n=77 rec=fct-bfg4inbf@1 type=5
@row_none n=78 rec=fct-6s4eftxh@1 type=S
@row_none n=79 rec=fct-mcdiomir@1 type=4
relied: Persistence callers never receive a Result; errors are logged at warn and swallowed.
@row_none n=80 rec=fct-svn2b5w4@1 type=4
relied: A failed stylesheet request leaves the pending critical resources, so it blocks no render.
@row_none n=81 rec=fct-lveo7mqo@1 type=4
@row_none n=82 rec=fct-xkfthmc4@1 type=4
relied: NetHandler::error is a default that does nothing, so a handler sees no failure unless it overrides it.
@row_none n=83 rec=fct-kcm5oy73@1 type=4
@row_none n=84 rec=fct-27ntkqqx@1 type=4
relied: Uncaught script errors are logged and not propagated, and a script that fails to resolve or fetch is skipped.
@row_none n=85 rec=fct-j62kbzd3@1 type=4
@row_none n=86 rec=fct-ymrlyila@1 type=4
@row_none n=87 rec=fct-4kkbivro@1 type=4
@row_none n=88 rec=fct-klc7ch6j@1 type=4
@row_none n=89 rec=fct-v5fkfrex@1 type=S
@row_add n=127 row=contract/cssom-script-api/error-names area=error-handling rec=fct-ol5tumaa@1
boundary: CSSOM script API
item: CSSOM errors
promise: CSSOM errors reach script as DOMException names.
@row_add n=128 row=contract/cssom-script-api/selectortext-assignment area=error-handling rec=fct-ol5tumaa@1
boundary: CSSOM script API
item: selectorText assignment
promise: Assigning selectorText never throws.
@row_add n=129 row=contract/cssom-script-api/csstext-assignment area=error-handling rec=fct-ol5tumaa@1
boundary: CSSOM script API
item: cssText assignment
promise: Assigning cssText never throws.
@row_add n=130 row=contract/js-module-loading/resolve-or-fetch-failure area=error-handling rec=fct-ylgvfhkt@1
boundary: JS module loading
item: resolve or fetch failure
promise: A module resolve or fetch failure reaches script as a JS TypeError.
@row_none n=90 rec=fct-wwenmixs@1 type=4
@row_add n=131 row=contract/mounted-element-operations/missing-node area=error-handling rec=fct-p5sm3bek@1
boundary: Dioxus mounted-element operations
item: missing node
promise: An operation on a node that does not exist returns MountedError::OperationFailed wrapping NodeNotExistErr.
@row_none n=91 rec=fct-eoknrijk@1 type=4
@row_none n=92 rec=fct-7vi4wxp5@1 type=4
@row_add n=132 row=decision/ci-matrix-runs-every-job area=error-handling rec=fct-6x32ysne@1
name: CI matrix runs every job
rule: CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.
@row_add n=133 row=decision/refusal-closed-cause-set area=error-handling rec=fct-ugqfmfy4@1
name: Command refusal as a closed set of causes
rule: An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field.
why: So a refusal cannot hold anything a call supplied.
except: A Fault is carried only beside the malformed cause.
@row_add n=134 row=decision/session-error-fixed-table area=error-handling rec=fct-ugqfmfy4@1
name: Session errors from a fixed table
rule: An escher-driver session error is written from a fixed table, holding nothing a call supplied.
@row_add n=135 row=decision/mcp-errors-fixed-no-data area=error-handling rec=fct-ugqfmfy4@1
name: MCP protocol errors fixed, no data
rule: MCP protocol errors are fixed codes and fixed messages and carry no data member.
@row_add n=136 row=decision/refusal-logged-by-cause-name area=error-handling rec=fct-ugqfmfy4@1
name: Refusal logged by cause name only
rule: Of an escher-driver command refusal, only its cause's fixed name reaches a log.
@row_add n=137 row=decision/stale-node-ids-never-panic area=error-handling rec=fct-kjyg4maj@1,fct-mgcd635n@1,fct-rng3zi7h@1
name: Stale node ids never panic
rule: A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve.
guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
@row_none n=93 rec=fct-hrr3a26v@1 type=4
relied: Resolve and hit testing may run with no DOM and return early with a warning, and a UI event on a document with no target is ignored.
@row_none n=94 rec=fct-pepdhmys@1 type=4
@row_none n=95 rec=fct-rpgeuznq@1 type=4
@row_none n=96 rec=fct-c4rr3jfv@1 type=4
relied: Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic.
@row_none n=97 rec=fct-qe5j3bt2@1 type=4
@row_add n=138 row=contract/query-apis/missing-element area=error-handling rec=fct-jfol2sty@1
boundary: Query APIs
item: missing element
promise: Query APIs return Result<Option<NodeId>>, and a missing element is None.
@row_add n=139 row=contract/fragment-navigation/unknown-target area=error-handling rec=fct-jfol2sty@1
boundary: Fragment navigation
item: unknown fragment target
promise: An unknown fragment target returns None or false and leaves scroll at 0.
@row_add n=140 row=contract/wpt-results/crash area=error-handling rec=fct-ip6hsnxe@1
boundary: WPT runner results
item: CRASH
promise: A test that panics is caught and reported as a CRASH result.
@row_add n=141 row=contract/wpt-results/fail-uncaught-js-error area=error-handling rec=fct-ip6hsnxe@1
boundary: WPT runner results
item: FAIL with Uncaught JS error
promise: A harness test that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest.
@row_add n=142 row=contract/wpt-results/timeout area=error-handling rec=fct-ip6hsnxe@1
boundary: WPT runner results
item: TIMEOUT
promise: A harness test that never reports and had no uncaught JS errors yields TIMEOUT.
@row_none n=98 rec=fct-bc53t2un@1 type=S
@row_none n=99 rec=fct-pmzozgdj@1 type=5
@row_none n=100 rec=fct-ewezgnov@1 type=5
@row_none n=101 rec=fct-oplylvcq@1 type=5
@row_none n=102 rec=fct-3myefzwr@1 type=5
@row_none n=103 rec=fct-l3r3gdc2@1 type=5
@row_none n=104 rec=fct-gnw4mzhy@1 type=5
@row_none n=105 rec=fct-ihf2uh6q@1 type=5
@row_none n=106 rec=fct-cdjzmvdp@1 type=5
@row_none n=107 rec=fct-f4v2lnec@1 type=5
@row_none n=108 rec=fct-4hxwthaq@1 type=5
@row_none n=109 rec=fct-wrgkix2k@1 type=5
@row_add n=143 row=absence/error-reporting-integration area=error-handling rec=fct-ueaekj7s@1
capability: An error reporting integration
@row_none n=110 rec=fct-hak4wvpc@1 type=S
@row_none n=111 rec=fct-i7a2ccfw@1 type=S
@row_none n=112 rec=fct-hgsjyuub@1 type=5
@row_add n=144 row=contract/network-tracing-log/url area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: url
promise: A network request log records the request URL as field url.
@row_add n=145 row=contract/network-tracing-log/status area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: status
promise: A network request log records the response status as field status.
@row_add n=146 row=contract/network-tracing-log/error area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: error
promise: A failed network request log records the failure as field error.
@row_add n=147 row=contract/network-tracing-log/path area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: path
promise: The cache directory path is logged at info level as field path.
@row_add n=148 row=contract/resource-load-failure-log/url area=logging-monitoring rec=fct-6le5eps5@1
boundary: Resource load failure log
item: url
promise: A resource load failure log carries structured field url.
@row_add n=149 row=contract/resource-load-failure-log/waiting-nodes area=logging-monitoring rec=fct-6le5eps5@1
boundary: Resource load failure log
item: waiting_nodes
promise: A resource load failure log carries structured field waiting_nodes.
@row_add n=150 row=contract/resource-load-failure-log/error area=logging-monitoring rec=fct-6le5eps5@1
boundary: Resource load failure log
item: error
promise: A resource load failure log carries structured field error.
@row_none n=113 rec=fct-ysjsa4jp@1 type=4
@row_add n=151 row=decision/js-console-to-debug-log area=logging-monitoring rec=fct-5qjgusc6@1
name: JS console routed to the debug log
rule: Script console output (log, info, warn, error) goes to the log crate at debug level under target js_console, never to stdout or stderr.
why: Keeps stdout and stderr clean.
@row_none n=114 rec=fct-o2zqeciv@1 type=4
@row_none n=115 rec=fct-lpzpej2o@1 type=5
@row_add n=152 row=decision/release-publish-upstream-only area=logging-monitoring rec=fct-2nfct2cx@1
name: Release publishing only from upstream
rule: The release-cli publish job is repository-guarded and runs only in the upstream DioxusLabs/blitz repository.
@row_none n=116 rec=fct-lfei2kpx@1 type=S
@row_add n=153 row=decision/tracing-for-engine-logging area=logging-monitoring rec=fct-u3c7wcoc@1
name: tracing for engine logging
rule: Engine crates log through the tracing crate behind an optional tracing feature, and their logging macros compile only with that feature on.
@row_add n=154 row=decision/tracing-feature-propagates area=logging-monitoring rec=fct-wkixw6vk@1
name: One tracing feature across the stack
rule: Turning on dioxus-native's tracing feature also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net.
@row_add n=155 row=decision/env-logger-for-wpt-runner area=logging-monitoring rec=fct-gfs2klpg@1
name: env_logger for the WPT runner
rule: The WPT runner logs through env_logger.
@row_add n=156 row=decision/escher-telemetry-allowlist-sink area=logging-monitoring rec=fct-ggtnnptr@1
name: escher's allowlist telemetry sink
rule: Both seven_guis binaries log through escher's own stderr-only allowlist subscriber.
except: The scrubbing reaches that sink only: upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.
@row_add n=157 row=decision/engine-target-field-allowlist area=logging-monitoring rec=fct-ggtnnptr@1
name: Engine log targets keep only allowlisted fields
rule: In escher's telemetry sink an engine target keeps only seven allowlisted fields, and every other field is written as [redacted].
except: An allowlisted log.file field can still carry a host path.
@row_add n=158 row=decision/escher-target-content-fields-withheld area=logging-monitoring rec=fct-ggtnnptr@1
name: escher_ targets withhold content-named fields
rule: In escher's telemetry sink an escher_ target keeps all fields but eleven content-named ones.
@row_add n=159 row=decision/other-log-targets-dropped area=logging-monitoring rec=fct-ggtnnptr@1
name: Other log targets dropped whole
rule: In escher's telemetry sink any target other than an engine or escher_ target is dropped whole at every level, and spans follow the same rule.
@row_add n=160 row=contract/escher-session-stdout/command-line area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stdout
item: command result line
promise: escher-session writes one uncoloured JSON line per command on stdout.
@row_add n=161 row=contract/escher-session-stdout/mcp-role area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stdout
item: mcp role
promise: The mcp role writes JSON-RPC lines only on stdout.
@row_add n=162 row=contract/escher-session-stderr/usage-messages area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stderr
item: usage messages
promise: escher-session writes usage messages on stderr.
@row_add n=163 row=contract/escher-session-stderr/error-messages area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stderr
item: error messages
promise: escher-session writes error messages on stderr.
@row_add n=164 row=decision/session-output-holds-no-caller-input area=logging-monitoring rec=fct-eabjzys7@1
name: Session output holds no caller input
rule: No escher-session output line holds anything the caller typed.
except: The mcp role's echoed request id.
@row_none n=117 rec=fct-byqhlzov@1 type=7
@row_add n=165 row=decision/agent-run-events-metadata-only area=logging-monitoring rec=fct-lm4cjfou@1
name: Agent-run events hold harness metadata only
rule: The agent-run test contract's JSON-line events carry no content-named field, only harness metadata, and never captured test output or panic text.
@row_add n=166 row=decision/agent-run-raw-log-unprinted area=logging-monitoring rec=fct-lm4cjfou@1
name: Agent-run raw log kept unprinted
rule: The agent-run raw cargo and libtest output log is unscrubbed, never printed by logs, and gitignored.
@row_add n=167 row=decision/cold-agent-events-metadata-only area=logging-monitoring rec=fct-3vxtdih5@1
name: Cold-agent events hold run metadata only
rule: The cold-agent pipe's events and verdict carry counts, booleans and closed words, never transcript text, a tool argument or answer, a reply or the check.
@row_add n=168 row=decision/cold-agent-record-unprinted area=logging-monitoring rec=fct-3vxtdih5@1
name: Cold-agent record kept unprinted
rule: The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
@row_add n=169 row=absence/monitoring-and-alerts area=logging-monitoring rec=fct-2j6yni6g@1
capability: Monitoring and alerting
@row_none n=118 rec=fct-43xeyzsr@1 type=S
@row_none n=119 rec=fct-o2jlcnmm@1 type=S
@row_none n=120 rec=fct-gnqolrzj@1 type=S
@row_add n=170 row=decision/session-socket-sole-listener area=api-security rec=fct-svo6bgmr@3
name: Session socket as the one listener
rule: The driver session's Unix-domain socket is the project's one listener, serving the session lifecycle and driver calls and their answers.
@row_add n=171 row=decision/mcp-server-over-stdio area=api-security rec=fct-svo6bgmr@3,fct-ghd6zkhy@1
name: MCP server over process stdio
rule: The MCP server role serves the driver's verbs as tools over its own stdin and stdout only, binding no listener, port or socket, and answers only the process that spawned it, with no handshake, token, secret or peer check.
why: The crossing was ratified as is.
@row_add n=172 row=contract/mcp-server/request-line area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: request line
promise: A request line is bounded at 65,536 bytes.
@row_add n=173 row=contract/mcp-server/tool-result area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: tool result
promise: A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.
@row_add n=174 row=contract/mcp-server/idle-expiry area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: idle expiry
promise: The idle expiry is checked when a request arrives.
@row_add n=175 row=contract/mcp-server/help area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: help
promise: The help tool is answered by the server itself from the schema's fixed tables, running nothing on a session.
@row_add n=176 row=contract/session-wire/recording-failed area=api-security rec=fct-svo6bgmr@3
boundary: Driver session wire
item: ok v3 recording-failed
promise: Wire v3 replies include the fixed reply ok v3 recording-failed.
@row_add n=177 row=decision/outbound-per-host-limit area=api-security rec=fct-ktpvvsv7@1
name: Outbound per-host concurrency limit
rule: Concurrent outbound requests are limited to 6 per host.
@row_add n=178 row=absence/cors-same-origin-policy area=api-security rec=fct-j3wtsshr@1
capability: A CORS or same-origin policy
@row_add n=179 row=absence/response-body-size-limit area=api-security rec=fct-7maio4kj@1
capability: A response body size limit on outbound requests
@row_add n=180 row=decision/settle-never-waits-on-loads area=api-security rec=fct-brzrm7cu@3
name: Settle never waits on a load
rule: A harness settle reports a load in flight at once as not settled with the class Loads, waits on no clock and moves none, and its pass loop is bounded at 64 passes.
why: The absent request timeout cannot hang a settle.
@row_add n=181 row=decision/load-wait-bound area=api-security rec=fct-brzrm7cu@3
name: Acting commands bounded by LOAD_WAIT
rule: An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class.
why: The absent request timeout cannot hang a command.
@row_add n=182 row=decision/time-moves-only-on-advance area=api-security rec=fct-brzrm7cu@3
name: Time moves only on advance
rule: Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
@row_add n=183 row=decision/scrollbar-fade-on-document-clock area=api-security rec=fct-brzrm7cu@3
name: Scrollbar fade reads the document clock
rule: The overlay scrollbar fade reads the document's clock, so on a harness-built document two shots of one state read the same pixels until an advance moves the fade; a document built with no clock still fades by wall time.
why: Ratified.
@row_add n=184 row=decision/failed-load-ends-wait area=api-security rec=fct-brzrm7cu@3
name: A failed load ends the wait
rule: A failed load ends a settle's wait, so a failed stylesheet never leaves a document not settled on Loads for good.
@row_add n=185 row=contract/driver-answer/settled area=api-security rec=fct-brzrm7cu@3
boundary: Driver command answer
item: settled
promise: A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the outcome names a class only, never a URL, request, node, id, name or value.
@row_add n=186 row=contract/driver-command-line/answer-bounds area=api-security rec=fct-brzrm7cu@3
boundary: Driver command line
item: answer bounds
promise: A started host has 30 s to answer and each read of a call's answer is bounded at 30 s; these bound a transport and wait on no load.
@row_none n=121 rec=fct-d2grrx3s@1 type=4
@row_none n=122 rec=fct-rbvhsytv@1 type=4
@row_none n=123 rec=fct-p2nck7ms@1 type=4
relied: Aborting the configured AbortSignal cancels every in-flight fetch tied to the document.
@row_none n=124 rec=fct-bj5snojm@1 type=4
@row_add n=187 row=decision/js-fetch-get-head-only area=api-security rec=fct-4xp4lixm@1
name: JS-facing fetch is GET and HEAD only
rule: Script-facing fetch allows only GET and HEAD, over the embedder's ScriptFetcher.
@row_none n=125 rec=fct-zyokxxre@1 type=4
@row_none n=126 rec=fct-3vrrikft@1 type=4
relied: Sub-document navigations route back to the parent document instead of navigating the host.
@row_none n=127 rec=fct-uljish3c@1 type=4
@row_none n=128 rec=fct-nhovh6p5@1 type=4
@row_none n=129 rec=fct-zdfj4pze@1 type=4
@row_none n=130 rec=fct-n5ajqukn@1 type=4
@row_add n=188 row=decision/outbound-http-through-blitz-net area=api-security rec=fct-yego5hpv@1
name: Outbound HTTP through blitz-net
rule: The apps' outbound HTTP goes through blitz-net with http2, and in rdme through reqwest.
@row_add n=189 row=decision/ci-github-api-through-gh area=api-security rec=fct-a7z4h4hw@1
name: CI GitHub API calls through gh
rule: CI calls the GitHub API through the gh CLI, with GH_TOKEN taken from the workflow token secret.
@row_add n=190 row=contract/session-socket/connections area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: connections
promise: One connection is served at a time, with a 2 s read and write bound.
@row_add n=191 row=contract/session-socket/request-line area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: request line
promise: A request line is at most 16,384 bytes.
@row_add n=192 row=contract/session-socket/lifecycle-reply area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: lifecycle reply
promise: A lifecycle reply is at most 128 bytes on its JSON line.
@row_add n=193 row=contract/session-socket/oversize area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: oversize
promise: An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
@row_add n=194 row=contract/session-socket/stop area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: stop
promise: stop removes the socket and the state directory and writes no record.
@row_add n=195 row=decision/host-idle-expiry area=api-security rec=fct-t6pddbhw@1
name: Host idle expiry
rule: A host that has answered no request for IDLE_EXPIRY, 1,800 s, ends exactly as on stop; every answered request, refused or malformed included, restarts the count, and a connection broken before its request is whole does not.
why: The answer to the orphaned-host question: a host whose client died without stop kept running.
@row_add n=196 row=decision/graph-answer-unpaged area=api-security rec=fct-s6ul72yw@1
name: The graph answer is unpaged
rule: The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound.
why: A part argument would widen the crossing.
guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
@row_add n=197 row=decision/actions-answer-unpaged area=api-security rec=fct-hbfh5ibd@1
name: The actions answer is unpaged
rule: The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound.
guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
@row_none n=131 rec=fct-e3bbwhnz@1 type=4
@row_none n=132 rec=fct-4w3pesu2@1 type=6
@row_none n=133 rec=fct-3ntkhij6@1 type=6
@row_add n=198 row=decision/publish-secrets-upstream-main-only area=authentication-authorization rec=fct-n7yhg2x4@1
name: Publish secrets reachable only upstream
rule: The publish job runs only in DioxusLabs/blitz and uses the Signed Builds environment only on main or ci-test branches, so no fork ref reaches the environment or its secrets.
@row_add n=199 row=decision/kept-check-owner-only area=data-protection rec=fct-cl72c3yo@4,fct-6n6morf4@1
name: Kept checks are owner-only
rule: A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends.
why: Ratified.
except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
guard: recording::the_committed_checks_are_what_the_binary_records
@row_add n=200 row=decision/kept-check-answer-whole area=data-protection rec=fct-cl72c3yo@4
name: Kept check keeps each answer whole
rule: A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels.
why: Ratified, a shot's pixels excepted.
@row_add n=201 row=decision/kept-check-call-content area=data-protection rec=fct-cl72c3yo@4
name: Kept check call-content class
rule: A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN.
why: Ratified.
@row_add n=202 row=decision/kept-check-graph-answer area=data-protection rec=fct-cl72c3yo@4
name: Kept check holds the graph answer
rule: A kept graph answer holds ids, owner ids, component names, registered state names, handler event names and closed flag words and booleans, never a control value or typed text.
why: A boundary widening, ratified.
@row_add n=203 row=decision/kept-check-actions-answer area=data-protection rec=fct-cl72c3yo@4
name: Kept check holds the actions answer
rule: A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId.
why: A boundary widening, ratified.
guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
state: provisional
@row_add n=204 row=contract/kept-check-file/entry area=data-protection rec=fct-cl72c3yo@4
boundary: Kept check file
item: entry
promise: A check is a JSON array: the app label, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
@row_add n=205 row=decision/cold-agent-records-private area=data-protection rec=fct-cl72c3yo@4
name: Cold-agent recordings stay private
rule: The cold-agent pipe records into an owner-only directory, never prints, copies or commits the checks, reads them for bools and counts only and removes them with its state on cleanup.
@row_add n=206 row=contract/coverage-line/unexercised area=data-protection rec=fct-3s6sesu6@1
boundary: Coverage answer line
item: unexercised
promise: An array of id, owner and event per unexercised handler, owner left out when absent.
@row_add n=207 row=contract/coverage-line/checks area=data-protection rec=fct-3s6sesu6@1
boundary: Coverage answer line
item: checks
promise: The count of checks replayed.
@row_add n=208 row=contract/coverage-line/held area=data-protection rec=fct-3s6sesu6@1
boundary: Coverage answer line
item: held
promise: The count of checks that held.
@row_add n=209 row=decision/coverage-line-no-user-data area=data-protection rec=fct-3s6sesu6@1
name: Coverage line carries no user data
rule: The coverage answer line holds ids, owner ids, event names and counts only, never a control value, typed text, path or NodeId.
guard: stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line
@row_add n=210 row=decision/advisory-ignores-by-id area=dependency-security rec=fct-a4qh6ioi@2
name: Advisory ignores by ID only
rule: An advisory is ignored only by its ID, each with a written reason, with no blanket allow and unmaintained and unsound at their defaults.
@row_add n=211 row=decision/local-audit-before-push area=dependency-security rec=fct-6a266lkz@1
name: Run the audit leg locally before a push
rule: The local fast leg runs no audit, so the audit leg is run locally before a push.
why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
@row_none n=134 rec=fct-zrg6urnw@1 type=8
@row_none n=135 rec=fct-nyplqb5c@1 type=8
@row_none n=136 rec=fct-uxldgqq5@1 type=8
@row_none n=137 rec=fct-37qbmwdh@1 type=8
@row_none n=138 rec=fct-sn3x6tqr@1 type=8
@row_add n=212 row=decision/no-git-dependency area=dependency-security rec=fct-f5jtvf4a@1,fct-tnbysnlo@1,fct-7aitszps@1
name: No git dependency
rule: The workspace takes no git dependency.
@row_add n=213 row=decision/engine-dependency-pins area=dependency-security rec=fct-tnbysnlo@1,fct-7aitszps@1
name: Engine dependency pins
rule: The pins stay: Stylo 0.22.0, Taffy the registry 0.15, Dioxus 0.7.3 and winit =0.31.0-beta.3.
@row_add n=214 row=decision/serde-json-read-only area=dependency-security rec=fct-3t4xl422@1
name: serde_json, read only
rule: serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing.
why: Chosen as read-only use.
@row_add n=215 row=decision/png-encodes-screenshots area=dependency-security rec=fct-q4iqo3xv@1
name: png encodes a screenshot
rule: The png crate is what encodes a screenshot in escher-driver, and blitz-tests takes it only as a dev-dependency.
@row_add n=216 row=decision/color-is-contrast-luminance-source area=dependency-security rec=fct-bf3tngmb@1
name: color is the contrast reader's luminance source
rule: The color crate, already in the graph through blitz-dom, is what supplies luminance to blitz-test-harness's contrast reader.
@row_add n=217 row=decision/free-disk-only-on-stand-repeat area=dependency-security rec=fct-pq2wmfyf@1,fct-kjnf7nfl@1
name: Runner disk freed only for the repeated stand job
rule: Only the `Stand checks repeated` CI job frees runner disk, with the free-disk-space action pinned to a commit; the `matrix_test` job holds no such step.
guard: test_only_stand_repeat_frees_disk
@row_add n=218 row=decision/embolden-locked-fast-leg area=dependency-security rec=fct-q5ecmhgy@1
name: font-embolden tested by a locked fast leg
rule: The blitz-paint font-embolden feature is tested by a locked cargo leg among the fast legs, with no CI job of its own.
@row_add n=219 row=decision/closed-refusal-cause-set area=error-handling rec=fct-bzel2phc@3
name: Closed refusal cause set
rule: A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause.
why: The closed set of eight was ratified.
@row_add n=220 row=decision/refusal-holds-no-call-input area=error-handling rec=fct-bzel2phc@3
name: A refusal holds nothing a call supplied
rule: Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
@row_add n=221 row=contract/refusal-causes/unknown-verb area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: unknown-verb
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=222 row=contract/refusal-causes/malformed area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: malformed
promise: The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
@row_add n=223 row=contract/refusal-causes/not-found area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: not-found
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=224 row=contract/refusal-causes/stale area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: stale
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=225 row=contract/refusal-causes/disabled area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: disabled
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=226 row=contract/refusal-causes/covered area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: covered
promise: The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
@row_add n=227 row=contract/refusal-causes/off-screen area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: off-screen
promise: The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
@row_add n=228 row=contract/refusal-causes/fonts-unavailable area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: fonts-unavailable
promise: One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
@row_add n=229 row=contract/refusal-json/cause area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: cause
promise: The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
@row_add n=230 row=contract/refusal-json/meaning area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: meaning
promise: The cause's fixed meaning text.
@row_add n=231 row=contract/refusal-json/remedy area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: remedy
promise: The cause's fixed remedy text.
@row_add n=232 row=contract/refusal-json/fault area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: fault
promise: Present only on a `malformed` refusal, naming the fault.
@row_add n=233 row=contract/session-error-json/kind area=error-handling rec=fct-bzel2phc@3
boundary: Session error JSON
item: kind
promise: One fixed kebab-case word per session error variant, inside `{"error":{...}}`.
@row_add n=234 row=contract/session-error-json/message area=error-handling rec=fct-bzel2phc@3
boundary: Session error JSON
item: message
promise: The error's fixed meaning, holding no path.
@row_add n=235 row=contract/session-error-json/remedy area=error-handling rec=fct-bzel2phc@3
boundary: Session error JSON
item: remedy
promise: The error's fixed remedy text.
@row_add n=236 row=decision/session-errors-fixed-table area=error-handling rec=fct-bzel2phc@3
name: Session errors as one fixed table
rule: Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text.
why: Ratified as the program's own fixed text inside the ratified carriers.
@row_add n=237 row=contract/mcp-tool-result/is-error area=error-handling rec=fct-bzel2phc@3
boundary: MCP tool result
item: isError
promise: A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.
@row_add n=238 row=contract/mcp-protocol-errors/parse-error area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: parse error
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=239 row=contract/mcp-protocol-errors/invalid-request area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: invalid request
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=240 row=contract/mcp-protocol-errors/method-not-found area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: method not found
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=241 row=contract/mcp-protocol-errors/invalid-params area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: invalid params
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=242 row=contract/mcp-protocol-errors/unknown-tool area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: unknown tool
promise: A fixed code and fixed message with no `data`; no reply holds the tool name.
@row_add n=243 row=decision/refusal-log-cause-name-only area=error-handling rec=fct-bzel2phc@3
name: Only a refusal's cause name is logged
rule: Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
@row_add n=244 row=decision/coverage-refuses-bad-checks-malformed area=error-handling rec=fct-iis2bn6b@1,fct-4spo7x3c@1
name: Coverage refuses an unusable check as malformed
rule: The `coverage` role refuses a check file it cannot read, one past the check bound, one that is no check or a check of another app as `malformed`, holding nothing of the path or the file, and adds no cause.
@row_none n=139 rec=fct-b44ukc3q@1 type=5
@row_none n=140 rec=fct-6rk5sgno@1 type=5
@row_add n=245 row=decision/parallel-style-traversal-serialised area=error-handling rec=fct-d2ojiy3n@1
name: Parallel style traversals serialised
rule: Every parallel style traversal takes one process-wide lock, its poison recovered, so two documents resolving parallel at once are serialised.
why: Two concurrent parallel traversals otherwise panic `already mutably borrowed`; the lock adds no panic path.
@row_none n=141 rec=fct-upgu656r@1 type=4
relied: The select list's public readers and writer read a row with no styles as None and add no panic path.
@row_add n=246 row=decision/dioxus-render-into-buffer area=error-handling rec=fct-7ni72vcj@3
name: Dioxus renders into a buffer before borrowing
rule: A Dioxus document renders mutations into a buffer with the document unborrowed, then borrows it once to replay, and flushes queued mounted events after the borrow is dropped.
why: So a task polled during render can reach the document through its mounted handle with no `RefCell already borrowed` panic.
guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_none n=142 rec=fct-wogfa4ah@1 type=4
relied: The render-path recorder holds no unwrap, expect or panic and has no slicing panic path.
@row_none n=143 rec=fct-h5tukp3g@2 type=9
@row_add n=247 row=decision/skip-mounted-event-of-removed-element area=error-handling rec=fct-dpj6t54a@1
name: Mounted event of a removed element skipped
rule: Flushing queued mounted events skips an element removed since its event was queued, for every document.
why: Differs from upstream: upstream's test asserting no mounted event is left out, as it reads one on Dioxus 0.7.
@row_add n=248 row=decision/dropped-anonymous-block-clears-parent area=error-handling rec=fct-5a4l32dj@1
name: A dropped anonymous block clears its children's parent
rule: Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a control whose block was dropped does not panic and still acts.
why: So the id does not go stale, curing every reader of a stale layout parent.
guard: key_press_after_container_lost_its_box
@row_none n=144 rec=fct-64ubemwx@1 type=4
relied: A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row.
@row_none n=145 rec=fct-5onuddg3@1 type=4
relied: A failed, empty or non-CSS stylesheet does not block render, and a net handler wrapper that does not forward error drops the failure silently.
@row_none n=146 rec=fct-rfjghjp2@1 type=4
@row_none n=147 rec=fct-nij6scrl@1 type=4
@row_none n=148 rec=fct-y5sakwj7@1 type=4
@row_none n=149 rec=fct-gnkizqzo@1 type=4
@row_add n=249 row=decision/agent-run-selection-allowlist area=input-validation rec=fct-biyghes6@1
name: agent-run.sh selection reaches cargo only as a test stem
rule: agent-run.sh exits 2 with usage before any cargo call on an unknown verb, a wrong argument count or a selection other than `stand`, `all` or an existing test-file stem matching `^[a-z0-9_]+$`.
@row_add n=250 row=decision/cold-agent-call-validation area=input-validation rec=fct-gmgkgl3p@1
name: cold-agent.sh call and task allowlist
rule: cold-agent.sh exits 2 with usage on stderr and empty stdout before any precondition or `claude` call on an unknown verb, a wrong argument count or a task outside the allowlist `stand`.
@row_add n=251 row=decision/apt-install-argument-validation area=input-validation rec=fct-ixnvecdo@1
name: apt-install.sh validates every argument first
rule: apt-install.sh accepts only package names of a fixed form and bounded numeric option values, else prints usage and exits 2 before sudo, timeout, apt-get or dpkg runs.
guard: InstallScriptTest.test_a_bad_call_exits_2_before_anything_runs
@row_add n=252 row=decision/apt-install-no-secrets area=input-validation rec=fct-ixnvecdo@1
name: apt-install.sh touches no environment or secret
rule: apt-install.sh reads no environment variable of its own, writes no file and names no secret, token or GitHub context variable.
@row_add n=253 row=decision/apt-install-bounded-repair area=input-validation rec=fct-ixnvecdo@1
name: Bounded dpkg repair before a retry
rule: Every apt install attempt after the first first runs a time-bounded `dpkg --configure -a` repair, a failed repair failing that attempt; the first attempt runs no dpkg.
@row_add n=254 row=decision/per-crate-coverage-floors area=input-validation rec=fct-jngrnarp@1
name: Per-crate line coverage floors
rule: The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90.
guard: coverage leg
@row_add n=255 row=decision/coverage-floor-input-validation area=input-validation rec=fct-jngrnarp@1
name: Coverage floor checker validates and prints no paths
rule: coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path.
guard: CoverageFloorTest
@row_add n=256 row=decision/cold-agent-check-validation area=input-validation rec=fct-tbmm2cwc@3
name: Agent-written check validated before replay
rule: Only the reply's last fenced json block is taken, at most 65,536 bytes and 400 steps, each step of closed keys and a known verb; a check outside these is never replayed, and the check is never printed.
why: A boundary widening to a new input class, ratified.
@row_add n=257 row=decision/cold-agent-replay-isolation area=input-validation rec=fct-tbmm2cwc@3
name: Agent-written check replayed on fresh servers
rule: A valid agent-written check is replayed on fresh MCP server children that validate every step again, each reply under a 30 s bound, each child killed and reaped, a failed child reading the check failed.
@row_none n=150 rec=fct-pbvi5fsv@2 type=4
@row_add n=258 row=decision/session-socket-closed-grammar area=input-validation rec=fct-7jixhrbw@3
name: Session socket request is a closed grammar
rule: A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
@row_add n=259 row=contract/session-socket-request/hello area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket request
item: hello
promise: `hello v3` asks for the session's lifecycle reply.
@row_add n=260 row=contract/session-socket-request/stop area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket request
item: stop
promise: `stop v3` stops the session once its reply is written.
@row_add n=261 row=contract/session-socket-request/call area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket request
item: call
promise: `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
@row_add n=262 row=contract/session-socket-reply/refused-version area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: refused version
promise: A known word with another wire version; nothing runs.
@row_add n=263 row=contract/session-socket-reply/refused-malformed area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: refused malformed
promise: A request line the grammar does not read or over 16,384 bytes; nothing runs.
@row_add n=264 row=contract/session-socket-reply/ok-accepted area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: ok v3 accepted
promise: The call ran and was accepted; its answer JSON follows on the line.
@row_add n=265 row=contract/session-socket-reply/ok-refused area=input-validation rec=fct-7jixhrbw@3,fct-bzel2phc@3
boundary: Session socket reply
item: ok v3 refused
promise: The call was refused; the refusal JSON follows on the line.
@row_add n=266 row=contract/session-socket-reply/ok-oversize area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: ok v3 oversize
promise: The call ran but its answer is over 1,048,576 bytes.
@row_add n=267 row=contract/session-socket-reply/ok-recording-failed area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: ok v3 recording-failed
promise: The call ran and the host's recording ended; the client reads it as a recording-failed session error.
@row_add n=268 row=decision/session-socket-bounds area=input-validation rec=fct-7jixhrbw@3
name: Session socket bounds
rule: One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes.
guard: an_answer_read_past_its_bound_is_a_timeout
@row_add n=269 row=decision/socket-carries-screen-reading area=input-validation rec=fct-7jixhrbw@3
name: Session socket carries what the screen reads
rule: An answer carries what the screen reads, ids, accessible names, control values with password and file input masked, snapshot text, diff, lint findings, UI graph, never a NodeId or an attribute value as typed.
why: The crossing was ratified.
@row_add n=270 row=decision/closed-value-field-no-widening area=input-validation rec=fct-7jixhrbw@3
name: A closed-value field in a ratified carrier is no widening
rule: A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does.
why: Ratified as the standing rule.
@row_add n=271 row=contract/screenshot-answer/png area=input-validation rec=fct-7jixhrbw@3
boundary: Screenshot answer
item: png
promise: `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element's visible part.
@row_add n=272 row=contract/graph-answer/nodes-states area=input-validation rec=fct-7jixhrbw@3
boundary: Graph answer
item: nodes and states
promise: `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
@row_add n=273 row=contract/snapshot-text/selected area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: selected
promise: A bare word written for an element that states it is selected.
@row_add n=274 row=contract/snapshot-text/unselected area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: unselected
promise: A bare word written for an element that states it is not selected.
@row_add n=275 row=contract/snapshot-text/pressed area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: pressed
promise: A bare word written for an element that states it is pressed.
@row_add n=276 row=contract/snapshot-text/unpressed area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: unpressed
promise: A bare word written for an element that states it is not pressed.
@row_add n=277 row=contract/snapshot-text/invalid area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: invalid
promise: A bare word written for an element that states it is invalid.
@row_add n=278 row=contract/diff-node/selected area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: selected
promise: A boolean, written only where the element states it.
@row_add n=279 row=contract/diff-node/pressed area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: pressed
promise: A boolean, written only where the element states it.
@row_add n=280 row=contract/diff-node/invalid area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: invalid
promise: A boolean, always written.
@row_add n=281 row=contract/diff-node/differs area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: differs
promise: On a changed node only, after `bounds`: a list of the node-key names that changed; an added node is written without it.
@row_add n=282 row=decision/session-socket-unix-only area=input-validation rec=fct-7jixhrbw@3
name: Session socket is Unix only
rule: The driver session socket exists on Unix only; elsewhere a session is `Unsupported`.
@row_add n=283 row=contract/snapshot-text/clickable area=input-validation rec=fct-i5kvbfe3@1
boundary: Snapshot text state words
item: clickable
promise: Written after `focused` only when the element's current listeners hold `click` and it is no disabled form control.
@row_add n=284 row=contract/diff-node/clickable area=input-validation rec=fct-i5kvbfe3@1
boundary: Diff node keys
item: clickable
promise: A boolean always written beside `focused`, true where the element's listeners hold `click` and it is no disabled form control; a changed node's `differs` can name it.
@row_none n=151 rec=fct-36ipyrv2@1 type=4
relied: An open select list's highlighted row is in no snapshot and no tree, and the lint does not read its contrast.
@row_add n=285 row=decision/state-dir-owner-only area=input-validation rec=fct-doehymzv@1
name: Session state directory is owner-only
rule: The session state directory is created mode 0700 and a present one granting group or others any bit is refused; the socket file is set 0600.
@row_add n=286 row=decision/state-dir-deletes-only-own area=input-validation rec=fct-doehymzv@1
name: Session cleanup deletes only its own files
rule: Stopping or idle expiry removes the socket file and then the directory, non-recursively, and nothing else is ever deleted.
@row_add n=287 row=decision/state-dir-path-is-parameter area=input-validation rec=fct-doehymzv@1
name: Session state path is a parameter
rule: The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
@row_add n=288 row=contract/session-errors/state-dir-too-long area=input-validation rec=fct-doehymzv@1
boundary: Session errors
item: state-dir-too-long
promise: The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.
@row_add n=289 row=decision/recording-dir-owner-only area=input-validation rec=fct-ifbk2u2m@3
name: Recording directory is owner-only
rule: The recording directory is created mode 0700 and a present one open to group or others is refused `record-dir-not-private` with status 3, nothing written and nothing spawned.
why: Ratified.
@row_add n=290 row=decision/recording-never-replaces-files area=input-validation rec=fct-ifbk2u2m@3
name: Recording never replaces or removes foreign files
rule: A recorded check file is created 0600 with create-new under the smallest free number, rewritten whole through a temp file and rename; no file is replaced and a file the recorder did not create is never removed.
@row_add n=291 row=decision/recording-bounds-end-recording area=input-validation rec=fct-ifbk2u2m@3
name: A failed or overlong recording ends
rule: A failed write or a passed bound of 400 records or 1,048,576 bytes ends the recording, its check removed, the call run and not rolled back and answered `recording-failed`.
@row_add n=292 row=decision/recording-dir-never-named area=input-validation rec=fct-ifbk2u2m@3
name: Recording directory is never exposed
rule: No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
why: Ratified.
@row_add n=293 row=decision/recorded-check-strict-reader area=input-validation rec=fct-v4gvn7w2@3
name: Recorded check is read strictly as untrusted
rule: A kept check is read as untrusted bytes, at most 1,048,576 bytes and 400 records of closed keys; anything else is refused `malformed` with no fault, holding nothing of the file, with no panic path.
@row_add n=294 row=contract/recorded-check/start-record area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: start record
promise: The array opens with `{"tool":"start","arguments":{"app":…}}`.
@row_add n=295 row=contract/recorded-check/tool area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: tool
promise: One of the ten instance verbs.
@row_add n=296 row=contract/recorded-check/arguments area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: arguments
promise: Argument names drawn from that verb's schema.
@row_add n=297 row=contract/recorded-check/masked area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: masked
promise: True on a `type` record alone.
@row_add n=298 row=contract/recorded-check/accepted area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: accepted
promise: One of the closed record keys, whether the call was accepted.
@row_add n=299 row=contract/recorded-check/digest area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: digest
promise: 16 lowercase hex characters.
@row_add n=300 row=contract/recorded-check/answer area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: answer
promise: Required and a JSON object; key order is not enforced.
@row_add n=301 row=decision/replay-through-session-run area=input-validation rec=fct-v4gvn7w2@3
name: Replay runs each step through the session
rule: A replayed check runs each kept step through the session's own validated run alone, runs nothing on a session of another app and stops at the first step whose acceptance or kept answer differs.
@row_add n=302 row=decision/cold-agent-check-reader-counts-only area=input-validation rec=fct-v4gvn7w2@3
name: Cold-agent recording reader keeps counts only
rule: The cold-agent pipe's reader of its live child's kept checks holds bools and counts only, reads an unreadable directory as no check kept, and never prints or copies a check's content.
@row_add n=303 row=decision/session-label-form area=input-validation rec=fct-mybm64sc@1
name: Session label form
rule: A session label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs; it is never document text, and is the one caller string a reply carries.
@row_add n=304 row=decision/closed-driver-verb-schema area=input-validation rec=fct-awfxp663@3
name: Closed driver verb schema
rule: Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing.
why: Each widening of the verb table is a widening of a validated surface, ratified.
@row_add n=305 row=decision/one-validation-path-for-every-carrier area=input-validation rec=fct-awfxp663@3,fct-gucmzkph@3,fct-f4eyvu3v@3
name: One validation path for every carrier
rule: The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation.
why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
@row_add n=306 row=contract/driver-argument-kinds/id area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: id
promise: Text of 1 to 1024 bytes.
@row_add n=307 row=contract/driver-argument-kinds/text area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: text
promise: Text of 0 to 4096 bytes.
@row_add n=308 row=contract/driver-argument-kinds/key area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: key
promise: One name of a closed twelve-name list.
@row_add n=309 row=contract/driver-argument-kinds/flag area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: flag
promise: True or false.
@row_add n=310 row=contract/driver-argument-kinds/milliseconds area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: milliseconds
promise: A whole number from 1 to 60000.
@row_add n=311 row=contract/driver-argument-kinds/name area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: name
promise: Text of 1 to 32 bytes of a-z, 0-9 and -.
@row_add n=312 row=contract/driver-argument-kinds/part area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: part
promise: A whole number from 1 to 1000.
@row_add n=313 row=decision/unknown-id-refused-stale-or-not-found area=input-validation rec=fct-awfxp663@3
name: Unknown id refused as stale or not-found
rule: An id that names no element on the screen as it reads now is refused with nothing run: stale when the session's record of ids holds it, not-found when not.
@row_add n=314 row=decision/target-refusal-order area=input-validation rec=fct-awfxp663@3
name: Fixed order of target refusals
rule: A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing.
why: The detection of refusal causes by code was ratified.
@row_add n=315 row=decision/advance-moves-virtual-clock-exactly area=input-validation rec=fct-awfxp663@3
name: Advance moves the virtual clock exactly
rule: The advance verb, accepted on every session, moves the session's virtual clock by exactly the milliseconds asked inside one settled step and reports exactly that time.
why: Ratified.
@row_add n=316 row=decision/id-record-bounded-and-session-fed area=input-validation rec=fct-awfxp663@3
name: Bounded id record fed by the session
rule: The session keeps no node handle between calls, only a record of at most 4096 id texts fed from its own snapshots and never from a call's argument.
@row_add n=317 row=decision/screen-pixels-in-band area=input-validation rec=fct-awfxp663@3,fct-gucmzkph@3,fct-f4eyvu3v@3
name: Screen pixels in band
rule: A screenshot's PNG travels inside the answer itself as base64 text, on the command line and as an MCP image block.
why: Ratified as the in-band crossing.
@row_add n=318 row=decision/driver-outcomes-never-logged area=input-validation rec=fct-awfxp663@3
name: Driver outcomes never logged
rule: The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer or an MCP tool result.
why: Outcomes leave the process only by the ratified crossing.
@row_add n=319 row=decision/one-command-span-per-call area=input-validation rec=fct-awfxp663@3
name: One command span per call
rule: Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
@row_add n=320 row=contract/driver-command-span/verb area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: verb
promise: The verb table's own word, never the caller's verb text.
@row_add n=321 row=contract/driver-command-span/cause area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: cause
promise: The name of the returned refusal's cause, unrecorded until it applies.
@row_add n=322 row=contract/driver-command-span/settled area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: settled
promise: A boolean, unrecorded until it applies.
@row_add n=323 row=contract/driver-command-span/busy area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: busy
promise: The schema's class word, unrecorded until it applies.
@row_add n=324 row=contract/driver-command-span/passes area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: passes
promise: A count, unrecorded until it applies.
@row_add n=325 row=contract/driver-command-span/added area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: added
promise: The length of the returned diff's added list.
@row_add n=326 row=contract/driver-command-span/removed area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: removed
promise: The length of the returned diff's removed list.
@row_add n=327 row=contract/driver-command-span/changed area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: changed
promise: The length of the returned diff's changed list.
@row_add n=328 row=contract/mcp-tool-call-span/tool area=input-validation rec=fct-awfxp663@3
boundary: The MCP tool_call span
item: tool
promise: The verb table's word for the called tool; the span parents the call's command span.
@row_add n=329 row=contract/mcp-tool-call-span/ending area=input-validation rec=fct-awfxp663@3
boundary: The MCP tool_call span
item: ending
promise: One of accepted, refused or error.
@row_add n=330 row=decision/driver-installs-no-subscriber area=input-validation rec=fct-awfxp663@3,fct-gucmzkph@3
name: Driver installs no subscriber
rule: The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, and the session's bounded load wait.
@row_add n=331 row=contract/driver-verbs/graph area=input-validation rec=fct-2njwkumb@1
boundary: Driver verbs
item: graph
promise: Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
@row_add n=332 row=contract/driver-verbs/actions area=input-validation rec=fct-gczaiqlz@1
boundary: Driver verbs
item: actions
promise: Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
@row_none n=152 rec=fct-dxkboam6@1 type=4
relied: Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused.
@row_none n=153 rec=fct-cfr7fwzo@1 type=4
relied: The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session.
@row_add n=333 row=contract/cli-exit-codes/0 area=input-validation rec=fct-gucmzkph@3
boundary: CLI exit codes
item: 0
promise: Accepted: the answer as one line of JSON on stdout.
@row_add n=334 row=decision/no-echo-of-caller-input area=input-validation rec=fct-gucmzkph@3,fct-jwfmxwld@1
name: No echo of caller input
rule: No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
@row_add n=335 row=decision/closed-pipe-drops-answer area=input-validation rec=fct-gucmzkph@3
name: Closed pipe drops the answer
rule: A write to a closed stdout is dropped and the command keeps its exit status, never a panic.
guard: a_closed_stdout_drops_the_answer_and_keeps_the_ending
@row_add n=336 row=contract/cli-flags/id area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --id
promise: Followed by one value, whatever that word is.
@row_add n=337 row=contract/cli-flags/text area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --text
promise: Followed by one value, whatever that word is.
@row_add n=338 row=contract/cli-flags/key area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --key
promise: Followed by one value, whatever that word is.
@row_add n=339 row=contract/cli-flags/ms area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --ms
promise: Followed by one value, whatever that word is.
@row_add n=340 row=contract/cli-flags/part area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --part
promise: Followed by one value, whatever that word is.
@row_add n=341 row=contract/cli-flags/shift area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --shift
promise: Bare, taking no value.
@row_add n=342 row=contract/cli-flags/session area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --session
promise: Followed by a directory, given exactly once, else usage.
@row_add n=343 row=contract/cli-flags/record area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --record
promise: Followed by a directory, taken at most once and only on start, the host role and the mcp role; anywhere else, twice or with no value is usage.
@row_add n=344 row=contract/cli-roles/serve area=input-validation rec=fct-gucmzkph@3
boundary: escher-session roles
item: serve
promise: serve <app> --session <dir> in exactly that shape, optionally with one --record <dir> after it, else usage.
@row_add n=345 row=contract/cli-roles/mcp area=input-validation rec=fct-gucmzkph@3
boundary: escher-session roles
item: mcp
promise: mcp alone or with one --record <dir> selects the MCP role; any other word after it is usage.
@row_add n=346 row=contract/cli-roles/coverage area=input-validation rec=fct-jwfmxwld@1
boundary: escher-session roles
item: coverage
promise: coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
@row_add n=347 row=contract/coverage-answer/line area=input-validation rec=fct-jwfmxwld@1
boundary: The coverage answer
item: line
promise: One line {"unexercised":[…],"checks":n,"held":m} on stdout, status 0, holding ids, owner ids, event names and the two counts only.
@row_add n=348 row=decision/mcp-over-stdio area=input-validation rec=fct-f4eyvu3v@3
name: MCP over stdio only
rule: The MCP server speaks newline-delimited JSON-RPC 2.0 on the invoking process's stdin and stdout, with no listener, port, socket, authentication or environment read.
why: The MCP carrier and its bounds were ratified as is.
@row_add n=349 row=contract/mcp-request/line-length area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP requests
item: request line length
promise: At most 65,536 bytes; a longer line is read to its terminator, dropped and answered -32700.
@row_add n=350 row=contract/mcp-request/id area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP requests
item: id
promise: Echoed only as an i64 integer or a string of at most 128 bytes, else -32600 with id null; the one client-sent value written back.
@row_add n=351 row=contract/mcp-request/notification area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP requests
item: a request with no id
promise: Treated as a notification: no reply, and nothing runs.
@row_add n=352 row=contract/mcp-errors/32700 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32700
promise: parse error, with no data: a non-UTF-8, non-JSON or overlong line.
@row_add n=353 row=contract/mcp-errors/32600 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32600
promise: invalid request, with no data: not an object with jsonrpc "2.0" and a string method, or an id of another form.
@row_add n=354 row=contract/mcp-errors/32601 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32601
promise: method not found, with no data: every method but initialize, ping, tools/list and tools/call.
@row_add n=355 row=contract/mcp-errors/32602 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32602
promise: invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
@row_add n=356 row=contract/mcp-initialize/protocol-revision area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP initialize
item: protocol revision
promise: Matched against a closed table of four supported revisions; any other revision asked for is answered with the newest of the four.
@row_add n=357 row=contract/mcp-tool-arguments/value-kinds area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP tool arguments
item: value kinds
promise: A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
@row_add n=358 row=contract/mcp-tool-result/size-bound area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP tool results
item: size bound
promise: A result's text over 1,048,576 bytes is replaced by the answer-too-large result; a result holding an image is bounded by its whole written form.
@row_add n=359 row=contract/mcp-tool-result/screenshot area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP tool results
item: screenshot
promise: An accepted shot is an image block (base64 PNG, image/png) then a text block of the shot's line without png, isError false; a refused shot is one text block with isError true.
@row_add n=360 row=decision/mcp-listing-fixed-text area=input-validation rec=fct-f4eyvu3v@3
name: MCP listing is fixed program text
rule: The MCP tool listing's descriptions are the program's own fixed table text: each verb's help and each result field's name, kind and help.
why: Ratified as the program's own text inside the ratified carrier.
@row_none n=154 rec=fct-uysume23@1 type=4
@row_add n=361 row=decision/external-navigation-get-http-mailto area=input-validation rec=fct-3uvcy3lv@1
name: External navigation limited to safe schemes
rule: Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
@row_none n=155 rec=fct-b5nx4un7@1 type=4
@row_none n=156 rec=fct-wqjbaxzx@1 type=4
@row_none n=157 rec=fct-qhaepvto@1 type=4
@row_none n=158 rec=fct-evgmbubw@1 type=4
@row_none n=159 rec=fct-hbpzpyqz@1 type=4
@row_none n=160 rec=fct-mgkywpyf@1 type=4
@row_none n=161 rec=fct-52dzlja5@1 type=4
@row_add n=362 row=decision/file-urls-only-from-file-documents area=input-validation rec=fct-kd7pemzk@1
name: file URLs load only from file documents
rule: A script fetch, a module import or a script src may load a file: URL only for a file: document, in every document, remote pages included.
why: A narrowing, not a boundary widening.
guard: remote_document_cannot_load_file_urls
@row_add n=363 row=decision/default-fetcher-file-and-data-only area=input-validation rec=fct-kd7pemzk@1
name: Default script fetcher takes file and data only
rule: The default script fetcher takes only file: and data: URLs.
@row_add n=364 row=absence/same-origin-and-cors-checks area=input-validation rec=fct-kd7pemzk@1
capability: Same-origin and CORS checks on script fetches
@row_none n=162 rec=fct-nrsyi4y5@1 type=4
@row_add n=365 row=decision/html-parsed-without-scripting area=input-validation rec=fct-mf6hp2ln@1
name: HTML parsed with scripting disabled
rule: HTML is parsed with the parser's scripting flag disabled.
@row_add n=366 row=decision/failed-stylesheet-blocks-no-render area=input-validation rec=fct-3e6nbz2f@1
name: Failed stylesheet blocks no render
rule: In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render.
guard: failed_stylesheet_unblocks_rendering, empty_or_non_css_stylesheet_unblocks_rendering
@row_add n=367 row=decision/unresolvable-stylesheet-href-no-load area=input-validation rec=fct-zppva2v4@1
name: Unresolvable stylesheet href starts no load
rule: In every document, a stylesheet href that cannot be resolved starts no load and blocks no render.
why: Resolution goes through the non-panicking resolver instead of the one that panics.
guard: invalid_stylesheet_url_does_not_block_rendering
@row_none n=163 rec=fct-ktl6rbzh@1 type=4
@row_none n=164 rec=fct-c32xddbc@1 type=4
@row_none n=165 rec=fct-zd4tccjz@1 type=4
@row_none n=166 rec=fct-o7jypgh2@1 type=4
@row_none n=167 rec=fct-cbyszk2x@1 type=4
@row_add n=368 row=decision/hintless-font-face-source-sniffed area=input-validation rec=fct-kgjrj3js@1
name: Hint-less font-face source fetched and sniffed
rule: An @font-face source whose URL cannot be resolved is skipped; a source with no format hint and no extension is fetched and its format sniffed from its bytes.
why: Ratified as upstream's behaviour, with a check asked for.
guard: font_face_sniffed_source
@row_add n=369 row=absence/url-taking-driver-verb area=input-validation rec=fct-kgjrj3js@1
capability: A driver verb that takes a URL
@row_none n=168 rec=fct-lx55falu@1 type=4
@row_none n=169 rec=fct-64wcuq4o@1 type=4
@row_none n=170 rec=fct-qkm5zac7@1 type=4
@row_none n=171 rec=fct-43ncaxev@1 type=4
@row_none n=172 rec=fct-hvylo7bs@1 type=4
@row_none n=173 rec=fct-zfte7qjy@1 type=4
@row_none n=174 rec=fct-gxjnow3r@1 type=4
@row_none n=175 rec=fct-ghvkx7uf@1 type=4
@row_none n=176 rec=fct-5kk7bggr@1 type=4
@row_none n=177 rec=fct-vc67mws7@1 type=4
@row_none n=178 rec=fct-yjp6z6oe@1 type=4
@row_none n=179 rec=fct-uoitt34g@1 type=4
@row_none n=180 rec=fct-inz4w2qp@1 type=4
@row_none n=181 rec=fct-iqioh5ix@1 type=4
@row_none n=182 rec=fct-zgrakrej@1 type=4
@row_none n=183 rec=fct-7fzrroih@1 type=4
@row_add n=370 row=decision/multiple-select-size-rows area=input-validation rec=fct-j5csdx44@1
name: Multiple select size sets its rows
rule: A multiple select's size, a whole number above zero capped at 65534, makes it that many rows tall and a key scrolls the next row in; zero or a non-numeric value gives no hint, and a single select's size is unread.
why: A boundary widening, ratified.
guard: size_reads_a_whole_number_above_zero_capped
@row_none n=184 rec=fct-zba4zqnk@1 type=4
@row_none n=185 rec=fct-3obvilj2@1 type=4
@row_add n=371 row=decision/disabled-read-by-presence area=input-validation rec=fct-mbi5iago@2
name: disabled read by presence on form controls
rule: The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer and keyboard activation and the snapshot's enabled and clickable readings.
why: The presence reading was ratified.
@row_none n=186 rec=fct-lnusqzqb@1 type=4
relied: The :open pseudo-class matches a details or dialog by its open attribute's presence, kept current by attribute writes and removals.
@row_none n=187 rec=fct-ohebonan@1 type=4
@row_none n=188 rec=fct-riwkju6q@1 type=4
@row_none n=189 rec=fct-5bmmkpof@1 type=4
@row_add n=372 row=decision/custom-widget-initial-attribute-replay area=input-validation rec=fct-ka33jnh7@1
name: Custom widget initial attribute replay
rule: Behind the custom-widget feature, setting a custom widget replays each existing attribute, parsed page content included, to the widget with no old value, so the widget receives them at set-up and checking each value stays the widget's own job.
why: An engine change for every document behind the feature, ratified.
guard: custom_widget_replays_initial_attributes
@row_add n=373 row=contract/stable-element-id/author-key area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: author key
promise: An element's HTML `id` is its author key only when it is non-empty, holds no `/` and is the first element in document pre-order to claim it; an empty, `/`-bearing or later-duplicate value gives no key and anchors nothing.
@row_add n=374 row=contract/stable-element-id/anchored-path area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: anchored path
promise: An element with no author key reads `{key}//{segment}` (then `/{segment}` per further DOM level) under the nearest keyed element of its own component instance, else its component path, else its document path; an anchor acts inside one component only.
@row_add n=375 row=decision/anchored-id-tier area=input-validation rec=fct-d2yejoup@5
name: Anchored stable-id tier
rule: An unkeyed element's stable id is anchored under the nearest author-keyed element of its own component instance before falling back to its component path or document path.
why: The anchored tier was ratified.
@row_add n=376 row=contract/stable-element-id/no-collision area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: uniqueness
promise: An author key holds no `/`, an anchored path is the only id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal, a key spelled like a component name included.
@row_add n=377 row=contract/stable-element-id/unresolved-node area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: non-element, stale or detached node
promise: A non-element, stale or detached node reads no id, and no path computation panics.
@row_add n=378 row=contract/stable-element-id/keyed-list-segment area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: keyed list row segment
promise: A keyed list row's segment carries its Dioxus key as `{tag}[{key}]`; a Dioxus key that is empty or holds `/` is refused and the row reads the positional `{tag}:{n}` segment instead.
@row_add n=379 row=contract/stable-element-id/stability area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: stability
promise: A stable id is never a list index, pointer, hash, clock or process-local value, so a row reads the same id across a re-render, a remount and a second process.
@row_add n=380 row=decision/no-process-handle-crosses area=input-validation rec=fct-d2yejoup@5
name: No process-local handle crosses with an id
rule: Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
@row_add n=381 row=decision/stable-id-not-logged area=input-validation rec=fct-d2yejoup@5
name: Stable ids kept out of logs
rule: Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level.
why: Those third-party targets are outside the sink's allowlist.
@row_add n=382 row=decision/author-id-on-accessibility-nodes area=input-validation rec=fct-d2yejoup@5
name: Stable id as accessibility author id
rule: Under the accessibility feature each element node of the accessibility tree carries its stable id as its AccessKit author id, never on a text run, the document root or the synthetic window node, so it leaves the process only through the platform adapter.
why: Ratified; the crossing text runs and labels already take.
@row_add n=383 row=decision/accessibility-tree-refresh-on-change area=input-validation rec=fct-d2yejoup@5
name: Accessibility tree refreshed on change
rule: The platform accessibility adapter receives the tree on change as well as on its initial request: a poll that took a non-empty changed set rebuilds the platform tree.
why: Ratified: the same crossing and the same content classes, more often.
@row_add n=384 row=decision/accessibility-adapter-content-classes area=input-validation rec=fct-d2yejoup@5,fct-q2xi4t6k@3
name: Content classes on the accessibility adapter crossing
rule: The platform accessibility adapter crossing carries four author-derived content classes: element ids, accessible names, control states and descriptions.
why: Each widening ratified; the states by ruling, the names' and descriptions' upstream additions ratified as upstream's behaviour.
@row_add n=385 row=contract/snapshot-text/id-field area=input-validation rec=fct-d2yejoup@5
boundary: Snapshot text
item: id field
promise: Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
@row_add n=386 row=decision/stable-id-exits area=input-validation rec=fct-d2yejoup@5
name: Ratified exits of the stable id
rule: The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files.
why: Each carrier ratified as a boundary widening.
@row_add n=387 row=decision/diff-counts-only-in-logs area=input-validation rec=fct-d2yejoup@5
name: Only diff sizes reach a log
rule: A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below.
why: Ratified: a diff's content never reaches a log while its three sizes may.
@row_add n=388 row=decision/id-lookup-anew-each-call area=input-validation rec=fct-d2yejoup@5
name: Caller-supplied id looked up anew
rule: An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
@row_add n=389 row=contract/driver-id-refusal/stale area=input-validation rec=fct-d2yejoup@5
boundary: Driver id refusal
item: stale
promise: A call naming an id that no element on the current screen carries is refused as `stale` when the session's record holds that id.
@row_add n=390 row=contract/driver-id-refusal/not-found area=input-validation rec=fct-d2yejoup@5
boundary: Driver id refusal
item: not-found
promise: A call naming an id that no element on the current screen carries is refused as `not-found` when the session's record does not hold that id.
@row_add n=391 row=contract/driver-call-id/length-bound area=input-validation rec=fct-d2yejoup@5
boundary: Driver call id
item: length bound
promise: An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
@row_add n=392 row=decision/session-id-record-bounded area=input-validation rec=fct-d2yejoup@5
name: Bounded in-process record of read ids
rule: A session keeps a private record of ids its own snapshots read, at most 4096 ids of at most 1024 bytes each, the oldest forgotten first, read by no public item and written to no log, event, socket or file.
why: Kept to tell the stale and not-found refusals apart.
guard: the_record_holds_no_id_longer_than_a_call_can_name
@row_add n=393 row=decision/lint-findings-closed-content area=input-validation rec=fct-d2yejoup@5
name: Lint findings hold no page content
rule: A lint finding holds ids, closed defect words and numbers only, never page text, colour or a control value, and no log site, print, span or field carries a finding.
why: A ratified boundary widening.
@row_none n=190 rec=fct-rc3jl2xp@1 type=4
relied: Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts.
@row_add n=394 row=contract/escher-session-coverage/answer-line area=input-validation rec=fct-ptvbryzl@1
boundary: escher-session coverage role
item: answer line
promise: The coverage answer holds handler references of ids, owner ids and event names and two counts, no process-local handle, and crosses only as one line on the command line's stdout, with no verb, socket or MCP carrier.
@row_none n=191 rec=fct-6dak5se6@1 type=4
relied: The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`.
@row_add n=395 row=contract/stable-element-id/virtual-node-form area=input-validation rec=fct-kc374vpe@1
boundary: Stable element id
item: virtual-node form
promise: The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/`; no served id holds `///`, and the anchored path stays the only served id holding `//`.
@row_add n=396 row=decision/aria-label-trimmed-once area=input-validation rec=fct-r5yt6pat@1
name: One trimmed aria-label read
rule: An `aria-label` non-empty after trimming becomes the node's label and a whitespace-only one names nothing; there is one trimmed, non-empty `aria-label` read.
why: Follows accname-1.2; upstream's unfiltered read was dropped in the merge.
guard: accessibility_label
@row_add n=397 row=decision/aria-labelledby-outranks-label area=input-validation rec=fct-r5yt6pat@1
name: aria-labelledby outranks label and aria-label
rule: An element's `aria-labelledby` targets' text, gathered cycle-guarded, is its computed label, outranking a `<label>` and an `aria-label`; an id naming no element is ignored and `aria-label` is the fallback when the targets hold no text.
why: Upstream's behaviour, ratified at the upstream sync.
guard: accessibility_label
@row_add n=398 row=decision/accessible-names-not-logged area=input-validation rec=fct-r5yt6pat@1
name: Accessible names kept out of logs
rule: Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole.
why: That third-party target is outside the sink's allowlist.
guard: mcp_log
@row_add n=399 row=decision/accessible-name-exits area=input-validation rec=fct-r5yt6pat@1
name: Ratified exits of accessible names
rule: Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log.
why: Each carrier ratified.
@row_add n=400 row=contract/snapshot-text/name-field area=input-validation rec=fct-r5yt6pat@1
boundary: Snapshot text
item: name field
promise: Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
@row_add n=401 row=decision/descriptions-adapter-only area=input-validation rec=fct-q2xi4t6k@3
name: Descriptions cross only to the accessibility adapter
rule: `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one.
why: Upstream's behaviour, ratified as a boundary widening by upstream merge.
guard: accessibility_description
@row_add n=402 row=decision/merged-aria-states-closed-parse area=input-validation rec=fct-lxb6gzob@1
name: Closed parse of expanded, disabled and level
rule: `aria-expanded`, `aria-disabled` and `aria-level` are read as closed values carrying none of the attribute's text and exit only on the accessibility adapter; `aria-disabled` moves no snapshot reading.
why: As upstream wrote them, inside the ratified states carrier.
guard: accessibility_merged_states
@row_add n=403 row=decision/select-expanded-from-engine-state area=input-validation rec=fct-qmjsf46a@1
name: Select expansion from engine state only
rule: An author's `open` on a select and `aria-expanded` on a single select steer nothing: a single select's expanded state is the engine's own closed state, and `open` on a select does not match `:open`.
why: An engine change for every document.
guard: an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match
@row_add n=404 row=decision/aria-state-lowercase-exact-match area=input-validation rec=fct-agnqdsgq@3
name: Control-state values matched exactly in lower case
rule: `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` values are trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set; an unrecognized value sets nothing, but for `aria-invalid`, which reads invalid.
why: A choice: the specification as read states no case rule and no browser was measured; the `aria-invalid` fallback is WAI-ARIA 1.2's rule.
except: A select's option reads its chosen state as selected, never `aria-selected`.
@row_add n=405 row=decision/toggle-read-per-role area=input-validation rec=fct-agnqdsgq@3,fct-nkkmfss5@1
name: Toggle state read per role
rule: `aria-checked` sets the toggle only on the five checkable roles and `aria-pressed` only on every other role; a native checkbox or radio reads neither attribute and gives its own checked state under a checkable role and nothing under any other.
why: Ratified.
@row_add n=406 row=decision/control-states-closed-values-only area=input-validation rec=fct-agnqdsgq@3
name: Control states cross as closed values
rule: Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed.
why: A ratified widening of the adapter crossing by a new author-derived class.
@row_add n=407 row=contract/snapshot-text/state-words area=input-validation rec=fct-agnqdsgq@3
boundary: Snapshot text
item: state words
promise: Control states are written as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid`; in JSON answers they are booleans.
@row_add n=408 row=decision/inert-subtree area=input-validation rec=fct-v47atmzs@1
name: Inert subtree
rule: An element carrying `inert`, or under an ancestor that does, is not focusable, takes no hit, leaves the accessibility tree and the snapshot, and a node holding focus when it comes under `inert` gives focus up; presence only, no value read.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_add n=409 row=decision/select-list-open-gate area=input-validation rec=fct-jhscqn7a@1
name: Which select opens a list
rule: A disabled, inert or hidden select, a select with no row to show, and one under a non-invertible transform open no list.
why: An engine change for every document and a boundary widening, ratified.
guard: a_disabled_inert_hidden_singular_or_empty_select_opens_nothing
@row_add n=410 row=decision/select-list-row-choice area=input-validation rec=fct-jhscqn7a@1
name: What a select list row takes
rule: A disabled row of an open select list changes nothing and leaves the list open, and a group header chooses nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: a_disabled_row_changes_nothing_and_leaves_the_list_open
@row_add n=411 row=decision/accessibility-focus-built-nodes-only area=input-validation rec=fct-dcrmx3m6@1
name: Accessibility focus names a built node
rule: The accessibility tree's focus names the focused node only when that node is among the built nodes, else the window, so the tree never names a node it did not build.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_add n=412 row=decision/inert-focus-clear-silent area=input-validation rec=fct-dcrmx3m6@1,fct-v47atmzs@1
name: Inert focus clear fires no blur
rule: A node that loses focus by coming under `inert` fires no DOM `blur` or `focusout`.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_add n=413 row=decision/range-current-value-apart area=input-validation rec=fct-wujpgoms@1
name: Range input current value apart from its attribute
rule: A range input holds a current value apart from its `value` attribute; a `value` write re-seeds it and a `min`, `max` or `step` write or removal re-sanitizes it.
why: A departure from the spec's dirty-value flag, the cost the ratified ruling took on.
@row_add n=414 row=decision/range-value-sanitized area=input-validation rec=fct-wujpgoms@1
name: Range value always finite and on grid
rule: Every range seed, attribute write and move is clamped to its bounds and moved to the nearest grid value, the greater on a tie, so the value stays finite, bounded and on its grid on degenerate markup; unparsable numbers read as absent.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=415 row=decision/range-keyboard-actions area=input-validation rec=fct-wujpgoms@1
name: Range input key actions
rule: On a focused range input with no `disabled` attribute, `ArrowRight` or `ArrowUp` steps up, `ArrowLeft` or `ArrowDown` steps down, `Home` and `End` go to the minimum and maximum, and every other key does nothing.
why: Ratified.
@row_add n=416 row=decision/range-pointer-actions area=input-validation rec=fct-wujpgoms@1
name: Range input pointer actions
rule: A main-button pointerdown on an enabled range input sets the value from the hit's fraction of the content box, and a click focuses it and changes no value.
why: Ratified.
@row_add n=417 row=decision/range-value-exits area=input-validation rec=fct-wujpgoms@1
name: Range current value exits
rule: A range input's current value leaves only as a form submission entry, the `input` event's value and the snapshot's control value, and no accessibility node value or action changes.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=418 row=decision/enter-activates-focused-control area=input-validation rec=fct-7zto53sb@1
name: Enter activates the focused control
rule: A pressed `Enter` on the focused node dispatches a click on it when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=419 row=decision/space-keyup-activates-focused-control area=input-validation rec=fct-7zto53sb@1
name: Space release activates the focused control
rule: A released Space on the focused node dispatches a click for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled; a link, a text input and a range input take no Space.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=420 row=decision/button-click-focuses area=input-validation rec=fct-7zto53sb@1
name: A button click focuses the button
rule: A click on a `button` of any type or an `input` of type `submit`, `reset` or `button` focuses it and submits its form owner when it is a submit button, a disabled form control stopping the walk first.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=421 row=decision/engine-change-event-checkable area=input-validation rec=fct-glmvut7v@1
name: Engine fires change on checkbox and radio
rule: A click on a checkbox always, and on a radio only where it was not checked before, sends `change` after `input` with the same value; the page script runtime synthesizes no `change`, so a script hears one, from the engine.
why: An engine change for every document and a boundary widening, ratified.
guard: checkable_change_event
@row_add n=422 row=decision/single-select-selectedness area=input-validation rec=fct-43yggwqh@1
name: Single-choice select selectedness
rule: In every document, remote pages included, a select without multiple holds its choice as its options' checked state and settles every recorded select by HTML's selectedness setting algorithm: the last chosen, else the first option not disabled.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
except: An option of a closed single select has no click arm and stays refused covered; a single select's size is not read.
@row_add n=423 row=decision/select-keys-closed-match area=input-validation rec=fct-43yggwqh@1,fct-dn4bmcz5@1,fct-cgy5awnj@1
name: Select keys pass a closed match
rule: A focused select's keys, an open list's keys and a multiple select's keys each pass one closed key match, and every key outside the match does nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup
@row_add n=424 row=decision/select-typeahead-private area=input-validation rec=fct-43yggwqh@1
name: Select typeahead kept private and short-lived
rule: Select typeahead keeps the typed text in the document's private buffer, starts afresh for another select or after 1 s of the document's clock, and the text is compared only, never kept past 1 s and never carried out.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
@row_add n=425 row=decision/select-choice-one-write-path area=input-validation rec=fct-43yggwqh@1,fct-dn4bmcz5@1,fct-cgy5awnj@1
name: Every select choice through one write path
rule: Every select choice, by key, click or open list, goes through one write path that marks the changed set and dispatches input then a non-cancelable, bubbling change, a choice of the already chosen option firing nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup
@row_add n=426 row=decision/select-choice-exits area=input-validation rec=fct-43yggwqh@1
name: Where a select's choice leaves
rule: A select's choice leaves only as form submission's entries, the input and change events' value, the snapshot's value and the options' accessibility-tree nodes, inside the ratified name and control-value crossings.
why: A boundary widening, ratified.
@row_add n=427 row=decision/select-list-live-reader area=input-validation rec=fct-p63xuhfj@1
name: Open select list read through one live reader
rule: The open select list is one document field, read through one live-list reader everywhere but reconciliation, the removal arm and row hover, which read the field directly.
@row_add n=428 row=decision/select-list-tab-chooses-and-closes area=input-validation rec=fct-dn4bmcz5@1
name: Tab chooses and closes an open select list
rule: On an open select list, Tab chooses the current row and closes the list before focus moves, in either direction.
why: An engine change for every document and a boundary widening, ratified.
guard: tab_chooses_and_closes_before_focus_moves_either_way
@row_add n=429 row=decision/select-list-pointer-release-chooses area=input-validation rec=fct-5eqwsk5u@1
name: Pointer in an open select list
rule: In an open select list a press starts no text drag and the release chooses the row under it.
why: An engine change for every document and a boundary widening, ratified.
guard: a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends
@row_add n=430 row=decision/select-list-closing area=input-validation rec=fct-5eqwsk5u@1
name: What closes an open select list
rule: An open select list closes on a press outside it, a page scroll or a move of its box, while a list a click just opened survives a focus rule's restyle.
why: An engine change for every document and a boundary widening, ratified.
guard: escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event
@row_add n=431 row=decision/select-list-hostile-markup-finite area=input-validation rec=fct-iqeqvhsa@1
name: Hostile markup under an open select list
rule: A hostile size, thousands of options and very long labels under an open select list keep every box finite and panic nowhere.
why: An engine change for every document and a boundary widening, ratified.
guard: hostile_markup_keeps_every_box_finite_and_panics_nowhere
@row_add n=432 row=decision/select-list-twenty-rows area=input-validation rec=fct-iqeqvhsa@1
name: Open select list shows at most twenty rows
rule: An open select list shows at most 20 rows, those past that scrolled.
why: An engine change for every document and a boundary widening, ratified.
guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
@row_add n=433 row=decision/multiple-select-keys-and-clicks area=input-validation rec=fct-cgy5awnj@1
name: Multiple select keys and clicks
rule: In a multiple select, a plain arrow, Home, End or click chooses that row alone, Shift spans from the anchor, Ctrl or Meta moves the current row by key and flips a row by click, and disabled rows are never chosen.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
except: Ctrl+A is not modelled.
@row_add n=434 row=decision/optgroup-options-and-label area=input-validation rec=fct-63uvlsf6@1
name: Optgroup options and label
rule: A select's options include its optgroup children's options in tree order, and a group's non-empty label is its name in the accessibility tree and a header painted in an open list.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup, select_popup_painted
@row_add n=435 row=decision/option-disabled-one-rule area=input-validation rec=fct-63uvlsf6@1
name: One option-disabled rule
rule: Whether an option is disabled is decided by one rule that the readings and form submission share.
why: An engine change for every document, ratified.
@row_add n=436 row=decision/multiple-select-set-exits area=input-validation rec=fct-2gvxsqoe@1
name: Where a multiple select's chosen set leaves
rule: A multiple select's chosen set leaves only as form entries, as input and change values carrying one entry per chosen option, as the snapshot's value of the first chosen option in tree order, and as the options' tree nodes.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=437 row=decision/driver-type-on-select area=input-validation rec=fct-d4potb3l@1,fct-43yggwqh@1
name: Driver type on a select
rule: The driver's type on any select focuses it with no click and no DOM focus events, clears the typeahead, closes any list, and a text opening on a space types nothing.
why: A click on a multiple select would land on the centre row and a Space would flip a row; a boundary widening, ratified.
@row_none n=192 rec=fct-pnrbnsjg@2 type=4
relied: Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open.
@row_add n=438 row=decision/select-list-label-never-logged area=input-validation rec=fct-t4jwjgu4@1
name: Select list row labels never logged
rule: The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
@row_add n=439 row=decision/password-file-value-masked area=input-validation rec=fct-3l4zlejk@1
name: Password and file input values masked
rule: The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path.
why: Ratified.
guard: stand_act_filled, stand_mcp_masked
except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
@row_add n=440 row=contract/snapshot-value/masked-value area=input-validation rec=fct-3l4zlejk@1
boundary: The snapshot's value
item: MASKED_VALUE
promise: Eight U+2022 characters, one fixed marker standing for a password or file input's non-empty value on every carrier.
@row_add n=441 row=decision/mask-holds-on-every-carrier area=input-validation rec=fct-3l4zlejk@1
name: Value mask holds on every carrier
rule: A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results.
why: Ratified.
guard: stand_act_filled, stand_mcp_masked
@row_add n=442 row=decision/password-painted-masked area=input-validation rec=fct-3l4zlejk@1
name: Password input painted masked
rule: A password input is painted as one bullet per character in every document and through every renderer that paints with the engine's painter, so a typed password never leaves as pixels in a screenshot.
why: An engine change for every document, ratified.
guard: password_painted_masked, stand_act_screenshot
except: The frame depends on the value's length; a file input's paint is unchanged.
@row_add n=443 row=decision/password-caret-placed-by-bullets area=input-validation rec=fct-3l4zlejk@1
name: Password caret placed by the bullets
rule: A click inside a password field places the caret by the painted bullets, so no width of the real characters reaches the caret.
why: Ratified.
@row_add n=444 row=decision/recorded-type-masked area=input-validation rec=fct-3l4zlejk@1
name: Recorded type to a masked control kept masked
rule: A recorded type whose element resolves to a masked control or to none is kept as a stand-in marked masked, never the text, its length or a digest of it, while any other type's text is kept as it crossed.
why: Ratified.
@row_add n=445 row=contract/recorded-check/masked-stand-in area=input-validation rec=fct-3l4zlejk@1
boundary: A recorded check
item: MASKED_STAND_IN
promise: The text "masked", with masked:true, stands for a recorded type to a masked or unresolved control.
@row_none n=193 rec=fct-exdlckxb@1 type=4
relied: Degenerate aspect ratios are discarded before layout uses them.
@row_add n=446 row=decision/inline-box-heights-finite area=input-validation rec=fct-zdncpoo3@1
name: Inline-box heights kept finite
rule: Inline-box heights are kept finite.
why: Huge author lengths cannot stall the line breaker.
@row_none n=194 rec=fct-o56d7tkg@1 type=4
relied: The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout.
@row_add n=447 row=decision/visible-region-through-transforms area=input-validation rec=fct-issnlebr@1
name: Visible region read through transforms
rule: A clipping box's visible region is read through its transforms, exact under translate and scale, a rotation reading the rotated box's bounding box, and a mapped box with a non-finite edge answers none.
why: An engine change for every document, ratified.
guard: visible_region_transformed, stand_act_transformed, transformed_client_rect
@row_add n=448 row=decision/visible-region-open-list-row area=input-validation rec=fct-issnlebr@1
name: Visible region of an open select list row
rule: For a row or a group of an open select list, the visible region answers the list's scrollport.
why: An engine change for every document, ratified.
@row_none n=195 rec=fct-pwj6p2fo@1 type=4
relied: Geometry whose transformed bounds are non-finite or beyond the f32 range is culled before painting.
@row_add n=449 row=decision/select-list-painted-over-page area=input-validation rec=fct-gkpzlahs@1
name: Select list painted over the page
rule: The open select list is painted after the root's render and before the devtools overlays, a non-finite list box left unpainted.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=450 row=decision/select-list-engine-colours area=input-validation rec=fct-gkpzlahs@1
name: Select list colours are the engine's own
rule: The open select list's colours are the engine's own constants, never a page value.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=451 row=decision/non-invertible-transform-no-hit area=input-validation rec=fct-z6bo7mnt@1
name: Non-invertible transform takes no hit
rule: In every document, the whole subtree of a transform whose determinant is 0 or not finite takes no hit.
why: An engine change for every document, ratified.
guard: hit_non_invertible_transform
@row_none n=196 rec=fct-gkohgcpp@1 type=4
@row_add n=452 row=decision/border-radii-scaled-down area=input-validation rec=fct-updndazb@1
name: Oversized border radii scaled down
rule: Oversized border radii are scaled down uniformly.
why: So adjacent radii do not overlap.
@row_none n=197 rec=fct-lk3nconf@1 type=4
@row_none n=198 rec=fct-gukbahaa@1 type=4
relied: Scroll targets are clamped to the scrollable range, and a programmatic scroll to a missing node does nothing.
@row_add n=453 row=decision/select-list-scroll-own-offset area=input-validation rec=fct-gvptjp26@1
name: Select list row scrolled into view by its own offset
rule: Scrolling an open select list's row into view writes the list's own offset at once, moving no scroll chain or viewport, the offset staying within its rows.
why: An engine change for every document and a boundary widening, ratified.
guard: a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement
@row_add n=454 row=decision/select-list-wheel area=input-validation rec=fct-gvptjp26@1
name: Wheel over and outside an open select list
rule: A wheel over an open select list scrolls the list, and a wheel elsewhere closes the list and then scrolls.
why: An engine change for every document and a boundary widening, ratified.
guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
@row_none n=199 rec=fct-ucse43ob@1 type=4
@row_none n=200 rec=fct-q43mkq7y@1 type=4
relied: Form submission excludes controls under a datalist, disabled controls, non-submitter buttons and unchecked checkboxes and radios.
@row_none n=201 rec=fct-skuqzr72@1 type=4
@row_none n=202 rec=fct-xdyrrrnw@1 type=4
@row_add n=455 row=absence/dioxus-native-form-validation area=input-validation rec=fct-3ulwqhxa@1
capability: Form data validation in the Dioxus native bridge: form data always reads valid
@row_none n=203 rec=fct-omhuceqe@1 type=4
@row_none n=204 rec=fct-3atajnay@1 type=4
relied: The Dioxus event handler records a dispatch only where a signal registry is registered, and a page with none dispatches as before.
@row_add n=456 row=contract/declared-write/target area=input-validation rec=fct-nzg4gdto@1
boundary: A DeclaredWrite of the signal registry
item: target
promise: An element's stable id, or a prefix ending in *.
@row_add n=457 row=contract/declared-write/event area=input-validation rec=fct-nzg4gdto@1
boundary: A DeclaredWrite of the signal registry
item: event
promise: An event name without the on prefix.
@row_none n=205 rec=fct-et47i5mq@1 type=4
relied: Dioxus dangerous_inner_html is applied as inner HTML.
@row_none n=206 rec=fct-t6dnzfpt@1 type=4
@row_none n=207 rec=fct-wlz77kgt@1 type=4
relied: A falsy value of a listed boolean attribute clears the attribute rather than writing it.
@row_none n=208 rec=fct-vewbxbie@1 type=4
@row_add n=458 row=decision/fetch-get-head-only area=input-validation rec=fct-5u3gkmvy@1
name: Script fetch limited to GET and HEAD
rule: A page script's fetch rejects with TypeError any method other than GET or HEAD.
@row_none n=209 rec=fct-54euhkme@1 type=4
@row_none n=210 rec=fct-hcokxzim@1 type=4
@row_none n=211 rec=fct-ezagoq6r@1 type=4
@row_none n=212 rec=fct-7iuit6z6@1 type=4
@row_none n=213 rec=fct-5oh2q7pg@1 type=4
@row_none n=214 rec=fct-su437imu@1 type=4
@row_none n=215 rec=fct-73s6fq52@1 type=4
relied: Script reads and writes CSS properties through named accessors on CSSStyleDeclaration, one per Stylo property name.
@row_none n=216 rec=fct-croeh4b2@1 type=4
@row_none n=217 rec=fct-a3r5rhwh@1 type=4
@row_add n=459 row=decision/search-params-native-parser-hidden area=input-validation rec=fct-vvmg4w6v@1
name: Native search-params parser hidden from pages
rule: The native search-params parser is deleted from the global object before any page script runs.
why: An engine change for every document, ratified.
guard: url_search_params
@row_add n=460 row=decision/url-search-params-bootstrap area=input-validation rec=fct-vvmg4w6v@1
name: URLSearchParams through a bootstrap script
rule: URLSearchParams and a URL's live searchParams, href and search reach page script through a bootstrap script, bad calls and receivers throwing TypeError and values passing USV conversion.
why: A realization on the registered script crossing, no boundary widening; ratified.
guard: url_search_params
@row_none n=218 rec=fct-uwwmkxhm@1 type=4
@row_add n=461 row=decision/sql-values-bound-by-placeholders area=input-validation rec=fct-27ctq7ii@1
name: SQL values bound by placeholders
rule: Persistence statements bind every value through placeholders.
@row_none n=219 rec=fct-62emqkgs@1 type=4
relied: Stored URLs are re-parsed on load and unparsable rows are skipped.
@row_none n=220 rec=fct-txc2a4of@1 type=4
relied: Serialization escapes text and attribute values but writes text inside raw-text elements such as style and script unescaped.
@row_add n=462 row=decision/inline-svg-serializes-well-formed area=input-validation rec=fct-oolxy7gn@1
name: Inline SVG serializes as well-formed XML
rule: Serialized inline SVG must be well-formed XML, escaped text and xlink:href still parsing.
guard: inline_svg_serialize
@row_none n=221 rec=fct-anxwobo4@1 type=4
@row_add n=463 row=decision/wpt-quarantine-entries-valid area=input-validation rec=fct-elu7gey4@1
name: WPT timeout-quarantine entries well-formed
rule: A WPT timeout-quarantine entry must hold a non-empty path without whitespace and a reason among the known reasons, and must not duplicate another.
@row_none n=222 rec=fct-3u57ggad@1 type=4
@row_none n=223 rec=fct-w2qhbiox@1 type=4
@row_none n=224 rec=fct-f4erdbji@1 type=4
@row_none n=225 rec=fct-as2qocp2@1 type=4
@row_none n=226 rec=fct-zomjwwb3@1 type=4
@row_none n=227 rec=fct-ub5ruuk7@1 type=4
@row_none n=228 rec=fct-xfn3rqzp@1 type=4
@row_none n=229 rec=fct-iti42js5@1 type=4
@row_none n=230 rec=fct-z2c6uyfa@1 type=4
@row_add n=464 row=decision/cells-grid-keyboard area=input-validation rec=fct-ppqmcyh2@1
name: Cells grid keyboard model
rule: The 7GUIs Cells grid takes focus with tabindex 0 and a closed key match and no role: arrows move the current cell, clamped, Enter opens the editor on it, focused, and the editor's Enter commits, stops propagation and returns focus to the grid.
why: Ratified as to the grid.
@row_none n=231 rec=fct-736ntlrb@1 type=4
@row_none n=232 rec=fct-qt4tvftd@1 type=5
@row_add n=465 row=decision/stderr-only-telemetry-sink area=logging-monitoring rec=fct-4hqe6pzr@2
name: Escher's own stderr telemetry sink
rule: Both seven_guis binaries, the native app and the escher-session driver host, install escher's own telemetry sink first thing: a stderr-only subscriber whose formatter applies an allowlist of targets.
except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed, and the chained std panic hook still prints the raw panic message to stderr.
@row_add n=466 row=decision/engine-target-safe-fields area=logging-monitoring rec=fct-4hqe6pzr@2
name: Engine targets print only safe fields
rule: An event whose target starts with blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer or js_console prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line; every other field, the message included, prints as [redacted].
@row_add n=467 row=decision/escher-target-content-scrub area=logging-monitoring rec=fct-4hqe6pzr@2
name: Escher targets scrub content-named fields
rule: An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which print as [redacted].
@row_add n=468 row=decision/outside-target-dropped area=logging-monitoring rec=fct-4hqe6pzr@2
name: Outside targets dropped whole
rule: An event or bridged log record from a target outside the allowlist is not printed at all, zero bytes, at every level including WARN and ERROR, whatever RUST_LOG names.
why: The named cost: a third-party WARN or ERROR no longer prints either.
@row_add n=469 row=decision/closed-span-lines area=logging-monitoring rec=fct-4hqe6pzr@2
name: One sink line per closed span
rule: The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored.
why: A boundary widening of the sink, ratified as built.
@row_add n=470 row=contract/tool-call-span/line area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The tool_call span line
item: line
promise: One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
@row_add n=471 row=contract/tool-call-span/tool area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The tool_call span line
item: tool
promise: The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.
@row_add n=472 row=contract/tool-call-span/ending area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The tool_call span line
item: ending
promise: One of the closed words accepted, refused or error.
@row_add n=473 row=contract/command-span/line area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The command span line
item: line
promise: A sink-installing host writes one closed command span line per call it runs.
@row_add n=474 row=contract/command-span/cause area=logging-monitoring rec=fct-4hqe6pzr@2,fct-5agrcrof@1
boundary: The command span line
item: cause
promise: Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
@row_add n=475 row=decision/no-screen-content-in-logs area=logging-monitoring rec=fct-4hqe6pzr@2,fct-llvjoueu@1,fct-5agrcrof@1,fct-xenxqlq3@1
name: Screen content stays out of logs
rule: No driver call puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer and a coverage check reach no log line at any level.
guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
@row_add n=476 row=contract/escher-session-output/caller-text area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: escher-session output
item: caller-typed text
promise: No output line holds anything the caller typed, except the admitted request id the mcp role echoes.
@row_add n=477 row=contract/escher-session-output/esc-byte area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: escher-session output
item: ESC byte
promise: No output line holds an ESC byte; output is uncoloured whatever the stream is.
@row_add n=478 row=contract/escher-session-output/accepted-stderr area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: escher-session output
item: stderr of an accepted command
promise: An accepted command's stderr is empty with RUST_LOG unset.
@row_add n=479 row=contract/escher-session-mcp-stdout/protocol-lines area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: The escher-session mcp role's stdout
item: protocol lines
promise: The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.
@row_add n=480 row=contract/escher-session-coverage-line/content area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: The escher-session coverage role's stdout line
item: content
promise: The coverage line holds ids, owner ids, event names and two counts, and nothing of a named path.
@row_none n=233 rec=fct-4ck7btcl@1 type=4
relied: A failed load's error text stops in the load response and the redacted Resource load failed line, and no carrier takes it.
@row_none n=234 rec=fct-26u4zyfp@1 type=5
@row_none n=235 rec=fct-ksl3fslx@1 type=4
relied: A select list's option and group labels leave only as painted pixels and the snapshot's existing name, the list holding no log site, span or field.
@row_none n=236 rec=fct-4cylhur3@1 type=4
relied: The render-path mutation buffer logs nothing itself and its replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.
@row_add n=481 row=contract/command-span/verb area=logging-monitoring rec=fct-llvjoueu@1,fct-5agrcrof@1
boundary: The command span line
item: verb
promise: Records the instance-level verb; a graph or actions call records the verb alone, plus cause when refused.
@row_add n=482 row=decision/cold-agent-logs-metadata-only area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent pipe logs run metadata only
rule: The cold-agent pipe's events and verdict carry counts, booleans and closed words only: no content-named field and no transcript text, tool argument, tool answer, reply or check.
guard: The scrub-set key check and four planted markers, under the shim
@row_add n=483 row=decision/cold-agent-transcript-raw area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent raw transcript kept unscrubbed
rule: The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.
@row_add n=484 row=decision/cold-agent-server-output-unprinted area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent server output never printed
rule: The cold-agent pipe reads its server's stderr log only by counting span lines and writes the agent's check as parsed, printing or copying neither; the replay children's stderr goes to /dev/null.
@row_add n=485 row=decision/cold-agent-recordings-owner-only area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent recordings owner-only
rule: The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.
@row_add n=486 row=decision/cold-agent-wrong-call-bar area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent wrong-call bar
rule: The cold-agent verdict decides its wrong calls against a 5 % bar, failing with the reason wrong-calls-over-bar above it.
@row_add n=487 row=decision/ci-secrets-in-actions area=secret-management rec=fct-gwmzm7pv@1
name: CI secrets in GitHub Actions secrets and vars
rule: CI secrets live in GitHub Actions secrets and vars: signing keys, the keystore and its passwords, the Apple certificate, the cross-repo dispatch token and the workflow token are read from them.
@row_add n=488 row=decision/ci-signing-material-removed area=secret-management rec=fct-gwmzm7pv@1
name: CI signing material removed after use
rule: A signing key or keystore that CI writes to disk from a secret is removed in an always() step.
@row_none n=237 rec=fct-tbouybgj@1 type=7
@row_none n=238 rec=fct-r7u7wjon@1 type=7
@row_none n=239 rec=fct-q25a2pwc@1 type=7
@row_none n=240 rec=fct-5k3imw3w@1 type=7
@row_none n=241 rec=fct-tgxeps3a@1 type=7
@row_none n=242 rec=fct-s6div4md@1 type=7
@row_none n=243 rec=fct-hoy5axpi@1 type=7
@row_none n=244 rec=fct-ujc6szjz@1 type=7
@row_none n=245 rec=fct-cdb7ihdt@1 type=7
@row_none n=246 rec=fct-jphiqah2@1 type=7
@row_none n=247 rec=fct-trkab6m3@1 type=7
@row_none n=248 rec=fct-oge3aocv@1 type=7
@row_none n=249 rec=fct-yffuq3co@1 type=7
@row_add n=489 row=decision/iframe-nesting-depth-limit area=threat-model-summary rec=fct-utdigfl4@1
name: Iframe nesting depth limit
rule: Iframe nesting depth is limited.
why: Guards against infinitely recursive self-embedding pages.
@row_add n=490 row=decision/import-nesting-depth-limit area=threat-model-summary rec=fct-utdigfl4@1
name: CSS @import nesting depth limit
rule: CSS @import nesting depth is limited.
why: Prevents unbounded recursion, such as an import URL that grows geometrically per level.
@row_add n=491 row=decision/stale-iframe-responses-discarded area=threat-model-summary rec=fct-utdigfl4@1
name: Stale iframe responses discarded
rule: An iframe response whose iframe was removed or re-navigated since the request is discarded by request id.
@row_add n=492 row=decision/skip-snapshot-of-unstyled-node area=threat-model-summary rec=fct-utdigfl4@1
name: Never-styled nodes not snapshotted
rule: Snapshotting a never-styled node is skipped.
why: Stylo invalidation would otherwise panic.
@row_add n=493 row=decision/script-fetcher-file-data-only area=threat-model-summary rec=fct-qfjr2qtu@1
name: Script fetcher takes file and data URLs only
rule: The default script fetcher supports only file: and data: URLs; any other scheme returns UnsupportedScheme.
@row_add n=494 row=decision/file-url-only-for-file-documents area=threat-model-summary rec=fct-qfjr2qtu@1
name: File URLs load only for file documents
rule: A file: URL loads only for a file: document; for any other document, remote pages included, JS fetch, module import and script src alike refuse it with a FetchError.
@row_add n=495 row=decision/browser-javascript-opt-in area=threat-model-summary rec=fct-qfjr2qtu@1
name: Browser JavaScript opt-in
rule: In the browser app JavaScript execution is opt-in, behind the non-default javascript feature.
@row_add n=496 row=decision/bootstrap-native-helper-removed area=threat-model-summary rec=fct-qfjr2qtu@1
name: Bootstrap native helper hidden from pages
rule: The URLSearchParams bootstrap's native search-params helper is deleted from globalThis before any page script runs.
@row_add n=497 row=decision/session-socket-carries-calls area=threat-model-summary rec=fct-ljkdert4@2
name: Session socket carries calls and answers
rule: The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses.
why: A boundary widening, ratified.
@row_add n=498 row=decision/snapshot-state-readings area=threat-model-summary rec=fct-ljkdert4@2
name: Three state readings per control
rule: The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, as closed words and booleans.
why: Inside the ratified crossing, ratified.
@row_add n=499 row=decision/lint-findings-crossing area=threat-model-summary rec=fct-ljkdert4@2,fct-gphdtjgv@1
name: Lint findings cross every ratified carrier
rule: A lint's findings cross every ratified carrier as stable ids, closed defect words and numbers, never page text, colour or value.
@row_add n=500 row=decision/open-list-shot-pixels area=threat-model-summary rec=fct-ljkdert4@2,fct-gphdtjgv@1
name: Open select list shot carries labels as pixels
rule: A shot of an open select list carries the page's option labels and group labels as pixels, on the socket and MCP carriers alike, the shot's fields unmoved.
why: A boundary widening, ratified.
guard: cli_commands, pinning a hosted shot equal to an in-process one
@row_add n=501 row=decision/select-list-crossing area=threat-model-summary rec=fct-ljkdert4@2,fct-gphdtjgv@1
name: Select list crossing limited to bounds and pixels
rule: The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff and as pixels in a shot; node state holds no field of it and no expanded node key stands.
@row_add n=502 row=decision/graph-answer-crossing area=threat-model-summary rec=fct-fcig2nza@1,fct-algpllm7@1
name: Graph answer crossing
rule: A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, never a control value, typed text or NodeId.
why: A boundary widening, ratified.
@row_add n=503 row=decision/actions-answer-crossing area=threat-model-summary rec=fct-nvvkgxt4@1,fct-pxfsuahu@1
name: Actions answer crossing
rule: An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId.
why: A boundary widening, ratified.
@row_add n=504 row=contract/actions-answer/id area=threat-model-summary rec=fct-nvvkgxt4@1
boundary: The actions answer
item: id
promise: The node's id.
@row_add n=505 row=contract/actions-answer/owner area=threat-model-summary rec=fct-nvvkgxt4@1
boundary: The actions answer
item: owner
promise: The node's owner id.
@row_add n=506 row=contract/actions-answer/verbs area=threat-model-summary rec=fct-nvvkgxt4@1
boundary: The actions answer
item: verbs
promise: A list of entries each holding verb, a word of the verb table, and cause, a closed cause word.
@row_add n=507 row=decision/coverage-check-files-untrusted area=threat-model-summary rec=fct-cz244xre@1
name: Kept check files read as untrusted
rule: The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.
@row_add n=508 row=decision/coverage-answer-crossing area=threat-model-summary rec=fct-cz244xre@1
name: Coverage answer crossing
rule: The coverage answer line carries ids, owner ids, event names and the checks and held counts only.
why: A boundary widening, ratified.
@row_add n=509 row=decision/mcp-stdio-no-auth area=threat-model-summary rec=fct-gphdtjgv@1
name: MCP server over stdio, no authentication
rule: The driver's MCP server runs as a stdio child a client spawns under the invoking user, JSON-RPC 2.0 on stdin and stdout, no listener, port or socket; the spawning client is the trust boundary, with no authentication.
@row_add n=510 row=decision/mcp-result-crossing area=threat-model-summary rec=fct-gphdtjgv@1
name: MCP result crossing
rule: An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's.
why: A boundary widening, ratified.
@row_add n=511 row=decision/cold-agent-screen-to-provider area=threat-model-summary rec=fct-gphdtjgv@1
name: Cold-agent runs carry the screen to the provider
rule: A cold-agent live run carries the stand's screen, element ids, accessible names and control values in snapshot texts and diffs, through the claude client to the model provider, and the model's reply returns as an agent-written check the pipe parses and replays.
why: A boundary widening, ratified.
@row_add n=512 row=decision/capability-statement-mcp-crossing area=threat-model-summary rec=fct-r5teidpw@3
name: Committed statement of the MCP crossing
rule: A committed capability statement tells its readers what an MCP client carries onward: the spawning client as the boundary with no authentication, the carried set, and onward use as the client's; it adds no carrier.
@row_add n=513 row=decision/ci-token-read-only area=authentication-authorization rec=fct-dnpx5hao@1,fct-iqaxure2@1
name: Read-only CI tokens
rule: CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only.
guard: test_ci_workflows.py
@row_add n=514 row=decision/cold-agent-fixed-prompt area=input-validation rec=fct-xl3lbcjg@1,fct-gmgkgl3p@1
name: Cold-agent task as a fixed prompt
rule: A cold-agent task reaches the agent session only as the fixed prompt, naming no element id and no app slug.
guard: test_the_prompt_names_no_stand_id
@row_add n=515 row=decision/no-panic-on-call-input area=input-validation rec=fct-vehorezx@1,fct-awfxp663@3
name: No panic on call-supplied input
rule: Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
@row_add n=516 row=contract/escher-session-exit-codes/0 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 0
promise: The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
@row_add n=517 row=contract/escher-session-exit-codes/1 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 1
promise: The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
@row_add n=518 row=contract/escher-session-exit-codes/2 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 2
promise: A usage error: the usage line goes to stderr and stdout stays empty.
@row_add n=519 row=contract/escher-session-exit-codes/3 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 3
promise: A session error: the session-error object on stdout and the session error's fixed message on stderr.
@row_add n=520 row=decision/kept-checks-owner-only area=data-protection rec=fct-kdtkdrex@1,fct-sw7lpzl2@1,fct-d2yejoup@5
name: Kept checks owner-only and uncommitted
rule: A kept check stays in an owner-only medium, is never logged and is never committed.
except: The two committed checks of the repository's own scripted stand-Counter sessions.
@row_add n=521 row=absence/served-network-api area=api-security rec=fct-s4jqcqqv@1,fct-svo6bgmr@3
capability: A served network API: no binary serves a TCP or UDP port
@row_add n=522 row=absence/content-security-policy area=api-security rec=fct-5zz6ux3p@1,fct-ovh4s4cx@1
capability: CSP enforcement
@row_add n=523 row=absence/request-timeout area=api-security rec=fct-5zz6ux3p@1,fct-b7bygsx3@1
capability: A request timeout on outbound requests
@row_add n=524 row=decision/session-socket-owner-only area=api-security rec=fct-sn4avmoz@1,fct-t6pddbhw@1,fct-c3aavvn3@1,fct-ljkdert4@2
name: Session socket reachable by the owning user only
rule: Access to the driver session socket is the filesystem's alone: the state directory is 0700, the socket file 0600, a directory open to its group or others is refused, and there is no handshake secret, token or peer-credential check.
@row_add n=525 row=decision/dependency-audit-cargo-deny area=dependency-security rec=fct-cappgruj@1,fct-a4qh6ioi@2
name: Dependency audit with cargo-deny
rule: cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push.
why: Ratified: audit the whole workspace and check licences.
except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
@row_add n=526 row=decision/no-base64-crate area=dependency-security rec=fct-ei3z3lut@1,fct-q4iqo3xv@1
name: No base64 crate
rule: No base64 crate is taken as a dependency; a shot's base64 is written by hand.
@row_add n=527 row=decision/licence-gate area=dependency-security rec=fct-k5ocwtgz@1,fct-n5tq3u6e@1
name: Licence gate on every push
rule: The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences.
why: An open question, decided as the fork's chosen option.
guard: the audit leg
except: upstream's own licenses job stays reverted
@row_add n=528 row=absence/authentication-authorization area=authentication-authorization rec=fct-iytem5zf@1,fct-xkfvr2q2@1,fct-yi4p2lut@1,fct-56jnxaxp@1,fct-a43rhdr5@1,fct-ayiqfjqx@1,fct-2pewmhrd@1,fct-sa6lfhsv@1,fct-lfns6t7f@1,fct-inooefgh@1,fct-q44pfj6r@1,fct-p5tk7ool@1
capability: Authentication or authorization code
@row_add n=529 row=decision/live-runs-use-operator-login area=secret-management rec=fct-jtwyhkf2@1,fct-ov4w6i7j@1
name: Live runs use the operator's own login
rule: The cold-agent live run and the MCP live trial reach the model provider only through the operator's own Claude Code login held outside the repository: no API key, no env read by the pipe, no CI secret, no new secret source and no auth library.
why: Ratified.
@row_add n=530 row=contract/cli-exit-codes/1 area=error-handling rec=fct-bzel2phc@3,fct-iis2bn6b@1,fct-gucmzkph@3
boundary: escher-session CLI exit codes
item: 1
promise: The call was refused; the refusal JSON is written to stdout.
@row_add n=531 row=contract/cli-exit-codes/2 area=error-handling rec=fct-iis2bn6b@1,fct-gucmzkph@3
boundary: escher-session CLI exit codes
item: 2
promise: Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
@row_add n=532 row=contract/cli-exit-codes/3 area=error-handling rec=fct-iis2bn6b@1,fct-gucmzkph@3
boundary: CLI exit codes
item: 3
promise: Session error: the error object with kind, message and remedy on stdout and its fixed message on stderr; a boot that fails ends with its session error.
@row_add n=533 row=decision/screenshot-in-band area=input-validation rec=fct-7jixhrbw@3,fct-ljkdert4@2,fct-gphdtjgv@1
name: Screenshot carried in band
rule: A screenshot's PNG crosses in band, as base64 inside its JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked, under the unchanged answer bound.
why: The in-band image was the ratified choice, a boundary widening.
@row_add n=534 row=contract/actions-answer/nodes area=input-validation rec=fct-7jixhrbw@3,fct-nvvkgxt4@1
boundary: The actions answer
item: nodes
promise: `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
@row_add n=535 row=contract/actions-answer/keyed area=input-validation rec=fct-gczaiqlz@1,fct-nvvkgxt4@1
boundary: The actions answer
item: keyed
promise: A boolean on each listed node; the field is provisional.
@row_add n=536 row=decision/actions-keyed-field area=input-validation rec=fct-d2yejoup@5,fct-pxfsuahu@1
name: Keyed flag in the actions answer
rule: The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId.
why: A ratified boundary widening.
state: provisional
