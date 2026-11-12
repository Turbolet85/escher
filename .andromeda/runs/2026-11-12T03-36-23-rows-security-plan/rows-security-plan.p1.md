@row_add n=1 row=decision/security-tier area=threat-model-summary rec=1
name: Security tier
rule: The project's security tier is 0.
@row_none n=1 rec=2 type=S
@row_add n=2 row=decision/remote-content-nesting-limits area=threat-model-summary rec=3
name: Remote content nesting limits
rule: Remote web content is treated as an attack vector: iframe and @import nesting depths are limited, and stale iframe responses are discarded by request id.
@row_add n=3 row=decision/javascript-opt-in area=threat-model-summary rec=4
name: JavaScript opt-in in the browser app
rule: In the browser app JavaScript runs only behind the non-default `javascript` feature.
@row_add n=4 row=decision/default-script-fetcher-local-only area=threat-model-summary rec=4,44
name: Default script fetcher takes local URLs only
rule: The default script fetcher takes only `file:` and `data:` URLs; a `file:` URL loads only for a `file:` document, and fetch(), a module import and a script src are each refused otherwise with a fetch error.
why: A narrowing of what a document's scripts may fetch.
@row_add n=5 row=absence/same-origin-cors-checks area=threat-model-summary rec=4,44
capability: Same-origin and CORS checks on script fetches
@row_none n=2 rec=5 type=5
@row_add n=6 row=decision/rdme-raw-html-passthrough area=threat-model-summary rec=6
name: rdme passes raw HTML through
rule: The rdme markdown renderer runs in unsafe mode, passing raw HTML in markdown through.
@row_none n=3 rec=7 type=4
@row_add n=7 row=decision/wpt-post-results-trusted-scripts area=threat-model-summary rec=8
name: WPT post-results workflow runs trusted scripts
rule: The WPT post-results workflow runs only in the DioxusLabs/blitz repository, only for successful pull-request-triggered WPT runs, and checks out its scripts from the default branch.
@row_add n=8 row=decision/driver-session-unix-socket-only area=threat-model-summary rec=9
name: Driver session on one Unix socket
rule: The driver session binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and runs on unix only.
@row_add n=9 row=decision/driver-access-by-filesystem-permissions area=threat-model-summary rec=9,15,28
name: Driver session access by filesystem permissions
rule: The driver session's state directory is created 0700, a present one open to its group or others is refused, and the socket is 0600, so access to the session is the owning user's.
@row_add n=10 row=decision/driver-session-no-peer-authentication area=threat-model-summary rec=9,10,15
name: Driver session and MCP server carry no authentication
rule: The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check, and session start does not check the answering pid; the trust boundary is the owning user or the spawning client process.
@row_add n=11 row=contract/driver-session-reply/no-node-id area=threat-model-summary rec=9,26
boundary: driver session socket replies
item: NodeId
promise: No reply carries a NodeId.
@row_add n=12 row=contract/driver-session-reply/masked-control-values area=threat-model-summary rec=9,26
boundary: driver session socket replies
item: control values
promise: A password's or file input's control value is carried masked.
@row_add n=13 row=decision/mcp-server-stdio-only area=threat-model-summary rec=10,35
name: MCP server on stdio only
rule: The MCP server is a stdio child an MCP client spawns, speaking on the invoking user's stdin and stdout, with no listener, port or socket.
@row_add n=14 row=contract/mcp-result/masked-control-values area=threat-model-summary rec=10
boundary: MCP server results
item: control values
promise: A password's or file input's control value is carried masked.
@row_add n=15 row=decision/mcp-results-client-responsibility area=threat-model-summary rec=10
name: MCP results are the client's
rule: What an MCP client does with the results it receives, screenshots and lint findings among them, a model provider included, is the client's responsibility.
@row_none n=4 rec=11 type=S
@row_add n=16 row=absence/application-authentication area=authentication-authorization rec=12
capability: Application authentication or authorization
@row_none n=5 rec=13 type=7
@row_add n=17 row=decision/cookie-store-behind-feature area=authentication-authorization rec=14
name: Cookie store behind a feature
rule: The HTTP client keeps a cookie store only with the `cookies` feature.
@row_add n=18 row=decision/ci-token-read-only area=authentication-authorization rec=14
name: Read-only CI tokens
rule: CI tokens are scoped by GitHub Actions permissions per workflow, and every ci.yml job's GITHUB_TOKEN is read-only.
@row_add n=19 row=decision/publish-job-repository-guard area=authentication-authorization rec=14
name: Publish job repository guard
rule: The publish job is repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.
@row_none n=6 rec=16 type=5
@row_none n=7 rec=17 type=5
@row_none n=8 rec=18 type=5
@row_none n=9 rec=19 type=S
@row_none n=10 rec=20 type=4
@row_add n=20 row=contract/agent-run-cli/2 area=input-validation rec=21
boundary: agent-run.sh exit codes
item: 2
promise: A verb outside boot run status cleanup logs, a wrong argument count, or a selection other than stand, all or an existing test-file stem prints usage and exits 2 before any cargo call.
@row_add n=21 row=contract/cold-agent-cli/2 area=input-validation rec=22
boundary: cold-agent.sh exit codes
item: 2
promise: A verb outside run status cleanup logs, a wrong argument count or a task outside the allowlist stand exits 2 before any precondition or claude call.
@row_add n=22 row=decision/cold-agent-fixed-prompt area=input-validation rec=22
name: Cold-agent task as a fixed prompt
rule: A cold-agent task reaches the agent session only as a fixed prompt, naming no element id and no app slug.
@row_add n=23 row=contract/apt-install-cli/2 area=input-validation rec=23
boundary: apt-install.sh exit codes
item: 2
promise: Package names and option values that do not match closed forms print usage and exit 2 before sudo, timeout, apt-get or dpkg runs.
@row_add n=24 row=decision/apt-install-no-env-no-secrets area=input-validation rec=23
name: Install script holds no environment or secret
rule: The apt install script reads no environment variable of its own, writes no file and names no secret.
@row_add n=25 row=contract/coverage-floor-cli/2 area=input-validation rec=24
boundary: coverage_floor.py exit codes
item: 2
promise: A floor not matching CRATE_DIR=PERCENT (a percent at most 100, no crate twice), or a crate with no SF record, exits 2.
@row_add n=26 row=decision/coverage-floor-prints-no-paths area=input-validation rec=24
name: Coverage floor prints no paths
rule: The coverage floor check prints only crate directories and counts, never a source or lcov path.
@row_add n=27 row=decision/model-reply-untrusted-input area=input-validation rec=25
name: Model reply is untrusted input
rule: The cold-agent model's reply is untrusted: only its last fenced json block is read, bounded in bytes and steps, each step checked against closed keys; a check outside these is never replayed, and a valid one replays on fresh MCP children with every step validated again by the server.
@row_add n=28 row=decision/cold-agent-checks-never-printed area=input-validation rec=25
name: Cold-agent checks never printed
rule: The cold-agent check file is never printed, and the live child's kept checks are read for booleans and counts only and never printed.
@row_add n=29 row=decision/paged-answer-closed-decoding area=input-validation rec=25
name: Paged answer decoded by closed fields
rule: The cold-agent check decodes a paged answer only as exactly the fields text, part and parts.
@row_add n=30 row=decision/driver-socket-closed-grammar area=input-validation rec=26
name: Driver socket closed request grammar
rule: The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
@row_add n=31 row=contract/driver-session-reply/no-attribute-text area=input-validation rec=26
boundary: driver session socket replies
item: attribute text
promise: A reply never carries an attribute's text.
@row_add n=32 row=contract/driver-session-reply/state-readings area=input-validation rec=26
boundary: driver session socket replies
item: state readings
promise: State readings are carried as closed words and booleans.
@row_add n=33 row=decision/boundary-widening-rule area=input-validation rec=27
name: Boundary widening rule
rule: A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
@row_add n=34 row=decision/state-directory-long-path-refused area=input-validation rec=28
name: Too-long state directory path refused
rule: A state directory path too long for a socket address is refused with a fixed message holding no path.
@row_add n=35 row=decision/state-directory-non-recursive-delete area=input-validation rec=28
name: Non-recursive session cleanup
rule: Only the socket file and then the state directory are ever deleted, non-recursively.
@row_add n=36 row=decision/state-directory-path-is-parameter area=input-validation rec=28
name: State directory path is a parameter
rule: The state directory path is a parameter, never an environment read or a working-directory-relative default.
@row_add n=37 row=decision/recording-directory-owner-only area=input-validation rec=29
name: Owner-only recording directory
rule: The recording directory is created 0700, a group- or other-open one is refused with nothing written or spawned, each check is created 0600 and never replaced, and recording is unix only.
@row_add n=38 row=decision/recording-ends-on-failure area=input-validation rec=29
name: Recording ends on failure
rule: A failed write or a passed bound ends the recording.
@row_add n=39 row=decision/recorder-removes-only-own-files area=input-validation rec=29
name: Recorder removes only its own files
rule: A file the recorder did not create is never removed.
@row_add n=40 row=decision/recording-directory-never-named area=input-validation rec=29
name: Recording directory never named or logged
rule: No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
@row_add n=41 row=decision/recorded-check-untrusted area=input-validation rec=30
name: Recorded check bytes are untrusted
rule: A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
@row_add n=42 row=contract/driver-session-label/format area=input-validation rec=31
boundary: driver session label
item: format
promise: A label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs.
@row_add n=43 row=decision/session-label-only-caller-string area=input-validation rec=31
name: Session label is the one echoed caller string
rule: The session label is never text read from the document and is the one caller string a reply carries.
@row_add n=44 row=decision/single-validation-gate area=input-validation rec=32,35
name: Single command validation gate
rule: Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
@row_add n=45 row=decision/unknown-target-refused area=input-validation rec=32
name: Call on an unknown or unfit element refused
rule: A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
@row_add n=46 row=decision/no-panic-on-call-input area=input-validation rec=33
name: No panic on call input
rule: The command schema has no unwrap, expect or panic on anything a call supplies.
@row_add n=47 row=decision/command-schema-no-clock area=input-validation rec=33
name: Command schema reads no clock
rule: The command schema and its validation read no clock.
@row_add n=48 row=decision/no-node-id-between-calls area=input-validation rec=33
name: No NodeId kept between calls
rule: No NodeId is kept between calls.
@row_add n=49 row=decision/command-span-content-free area=input-validation rec=33
name: Command span holds no content
rule: One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
@row_add n=50 row=decision/driver-no-subscriber-no-env area=input-validation rec=33,34
name: Driver installs no subscriber and reads no env
rule: The driver crate and its session command line install no tracing subscriber and read no environment variable.
@row_add n=51 row=decision/cli-closed-argv-reader area=input-validation rec=34
name: Closed argv reader
rule: The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
@row_add n=52 row=contract/escher-session-exit-codes/0 area=input-validation rec=34
boundary: escher-session exit codes
item: 0
promise: The call was accepted.
@row_add n=53 row=contract/escher-session-exit-codes/1 area=input-validation rec=34
boundary: escher-session exit codes
item: 1
promise: The call was refused.
@row_add n=54 row=contract/escher-session-exit-codes/2 area=input-validation rec=34
boundary: escher-session exit codes
item: 2
promise: Usage error.
@row_add n=55 row=contract/escher-session-exit-codes/3 area=input-validation rec=34
boundary: escher-session exit codes
item: 3
promise: Session error.
@row_add n=56 row=decision/cli-echoes-nothing-typed area=input-validation rec=34
name: Command line echoes nothing typed
rule: No command-line ending writes back anything the caller typed, and no path on argv panics.
@row_add n=57 row=contract/mcp-server/transport area=input-validation rec=35
boundary: MCP server
item: transport
promise: Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.
@row_add n=58 row=contract/mcp-server/protocol-error area=input-validation rec=35
boundary: MCP server
item: protocol error
promise: A protocol error is a fixed code and message with no data.
@row_add n=59 row=contract/mcp-server/request-id area=input-validation rec=35
boundary: MCP server
item: request id
promise: The admitted request id is the one client-sent value written back.
@row_add n=60 row=decision/mcp-bounded-lines area=input-validation rec=35
name: Bounded MCP requests and results
rule: MCP request lines and results are bounded, and tool arguments are typed before validation.
@row_add n=61 row=decision/mcp-no-print-or-panic area=input-validation rec=35
name: MCP server never prints or panics
rule: The MCP server has no print, unwrap, expect or panic outside the tests.
@row_none n=11 rec=36 type=4
@row_add n=62 row=decision/external-navigation-schemes area=input-validation rec=37
name: External navigation limited
rule: Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
@row_none n=12 rec=38 type=4
@row_none n=13 rec=39 type=4
@row_none n=14 rec=40 type=4
@row_none n=15 rec=41 type=4
@row_none n=16 rec=42 type=4
@row_none n=17 rec=43 type=4
@row_none n=18 rec=45 type=4
@row_add n=63 row=decision/html-parsed-scripting-disabled area=input-validation rec=46
name: HTML parsed with scripting disabled
rule: HTML is parsed with scripting disabled.
@row_none n=19 rec=47 type=4
@row_none n=20 rec=48 type=4
@row_none n=21 rec=49 type=4
@row_none n=22 rec=50 type=4
@row_none n=23 rec=51 type=4
@row_none n=24 rec=52 type=4
@row_add n=64 row=decision/no-agent-urls-through-font-path area=input-validation rec=53
name: No agent URL through font sources
rule: escher routes no agent- or user-supplied URL through the font-face source path.
@row_none n=25 rec=54 type=4
@row_none n=26 rec=55 type=4
