@row_add n=1 row=decision/security-tier area=threat-model-summary rec=fct-muh4yxj3@1
name: Security tier
rule: The project's security tier is 0.
@row_check n=1 row=decision/security-tier sha=cb0d95609246 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: security tier · in *.md, *.toml, *.yml, *.json outside the pipeline records
why: no evidence found
@row_add n=2 row=decision/remote-content-nesting-limits area=threat-model-summary rec=fct-d35urlbv@1
name: Remote content nesting limits
rule: Remote web content is treated as an attack vector: iframe and @import nesting depths are limited, and stale iframe responses are discarded by request id.
@row_check n=2 row=decision/remote-content-nesting-limits sha=eb6d760cfc2a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/iframe.rs:18
quote: pub(crate) const MAX_SUBDOCUMENT_DEPTH: usize = 10;
at: packages/blitz-dom/src/net.rs:186
quote: const MAX_IMPORT_DEPTH: usize = 16;
at: packages/blitz-dom/src/iframe.rs:164-168
quote: let is_current = self .iframe_loads .get(&node_id) .is_some_and(|load| load.request_id == Some(request_id)); if !is_current {
@row_add n=3 row=decision/javascript-opt-in area=threat-model-summary rec=fct-pmsbanho@1
name: JavaScript opt-in in the browser app
rule: In the browser app JavaScript runs only behind the non-default `javascript` feature.
@row_check n=3 row=decision/javascript-opt-in sha=6446c076c5de v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:36
quote: javascript = ["dep:blitz-vibey-script"]
at: apps/browser/Cargo.toml:14
quote: default = ["hybrid", "cookies", "cache", "screenshot", "apple-font-embolden", "scrollbars", "mimalloc"]
@row_add n=4 row=decision/default-script-fetcher-local-only area=threat-model-summary rec=fct-pmsbanho@1,fct-qlpwd5pv@1
name: Default script fetcher takes local URLs only
rule: The default script fetcher takes only `file:` and `data:` URLs; a `file:` URL loads only for a `file:` document, and fetch(), a module import and a script src are each refused otherwise with a fetch error.
why: A narrowing of what a document's scripts may fetch.
@row_check n=4 row=decision/default-script-fetcher-local-only sha=c4ac0e99e2fb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:44-45
quote: url.scheme() != "file" || document_url.is_some_and(|document_url| document_url.scheme() == "file")
at: packages/blitz-vibey-script/src/fetch.rs:68
quote: scheme => Err(FetchError::UnsupportedScheme(scheme.to_string())),
at: packages/blitz-vibey-script/src/runtime/fetch.rs:43
quote: Err(FetchError::Forbidden(url.clone()))
at: packages/blitz-vibey-script/src/runtime/modules.rs:80
quote: Err(FetchError::Forbidden(url.clone()))
at: packages/blitz-vibey-script/src/document.rs:163-166
quote: if !document_may_load(self.base_url.as_ref(), &url) { self.record_error(format!( "failed to fetch script {url}: {}", FetchError::Forbidden(url.clone())
@row_add n=5 row=absence/same-origin-cors-checks area=threat-model-summary rec=fct-pmsbanho@1,fct-qlpwd5pv@1
capability: Same-origin and CORS checks on script fetches
@row_check n=5 row=absence/same-origin-cors-checks sha=eb0d52888792 v=holds judge=sonnet head=edd0cfaea7f0
searched: [Cc]ors|same_origin|same-origin|Access-Control · in packages/blitz-vibey-script
@row_add n=6 row=decision/rdme-raw-html-passthrough area=threat-model-summary rec=fct-ibx5eqs6@1
name: rdme passes raw HTML through
rule: The rdme markdown renderer runs in unsafe mode, passing raw HTML in markdown through.
@row_check n=6 row=decision/rdme-raw-html-passthrough sha=d91854ba1968 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/readme/src/markdown/comrak.rs:28
quote: r#unsafe: true,
@row_add n=7 row=decision/wpt-post-results-trusted-scripts area=threat-model-summary rec=fct-yhokldke@1
name: WPT post-results workflow runs trusted scripts
rule: The WPT post-results workflow runs only in the DioxusLabs/blitz repository, only for successful pull-request-triggered WPT runs, and checks out its scripts from the default branch.
@row_check n=7 row=decision/wpt-post-results-trusted-scripts sha=9d4666bfd5ee v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/wpt-post-results.yml:15
quote: if: github.repository == 'DioxusLabs/blitz' && github.event.workflow_run.event == 'pull_request' && github.event.workflow_run.conclusion == 'success'
at: .github/workflows/wpt-post-results.yml:18
quote: - name: Checkout trusted scripts
@row_add n=8 row=decision/driver-session-unix-socket-only area=threat-model-summary rec=fct-z5ttw3ol@3
name: Driver session on one Unix socket
rule: The driver session binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and runs on unix only.
@row_check n=8 row=decision/driver-session-unix-socket-only sha=deee15cca66f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
at: packages/escher-driver/src/host.rs:49
quote: use std::os::unix::net::{UnixListener, UnixStream};
@row_add n=9 row=decision/driver-access-by-filesystem-permissions area=threat-model-summary rec=fct-z5ttw3ol@3,fct-pgi7ssst@1,fct-dx3vh2da@1
name: Driver session access by filesystem permissions
rule: The driver session's state directory is created 0700, a present one open to its group or others is refused, and the socket is 0600, so access to the session is the owning user's.
@row_check n=9 row=decision/driver-access-by-filesystem-permissions sha=e414edfc6e6d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:92
quote: match DirBuilder::new().mode(0o700).create(state_dir) {
at: packages/escher-driver/src/host.rs:99-102
quote: if mode & 0o077 == 0 { Ok(false) } else { Err(SessionError::StateDirNotPrivate)
at: packages/escher-driver/src/host.rs:124
quote: fs::set_permissions(socket, Permissions::from_mode(0o600)).map_err(io_error)?;
@row_add n=10 row=decision/driver-session-no-peer-authentication area=threat-model-summary rec=fct-z5ttw3ol@3,fct-dgkl4a64@3,fct-pgi7ssst@1
name: Driver session and MCP server carry no authentication
rule: The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check, and session start does not check the answering pid; the trust boundary is the owning user or the spawning client process.
@row_check n=10 row=decision/driver-session-no-peer-authentication sha=c894591b2e10 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:124
quote: fs::set_permissions(socket, Permissions::from_mode(0o600)).map_err(io_error)?;
at: packages/escher-driver/src/cli.rs:445-446
quote: io::stdin().lock(), io::stdout().lock(),
searched: TcpListener|UdpSocket|TcpStream|std::net:: · in packages/escher-driver/src
searched: token|handshake|secret · in packages/escher-driver/src/host.rs
@row_add n=11 row=contract/driver-session-reply/no-node-id area=threat-model-summary rec=fct-z5ttw3ol@3,fct-6hwcr6nz@3
boundary: driver session socket replies
item: NodeId
promise: No reply carries a NodeId.
@row_check n=11 row=contract/driver-session-reply/no-node-id sha=aa745987a70f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:145
quote: object.text("id", parts.id);
at: packages/escher-driver/src/json.rs:149
quote: object.text("role", parts.role);
@row_add n=12 row=contract/driver-session-reply/masked-control-values area=threat-model-summary rec=fct-z5ttw3ol@3,fct-6hwcr6nz@3
boundary: driver session socket replies
item: control values
promise: A password's or file input's control value is carried masked.
@row_check n=12 row=contract/driver-session-reply/masked-control-values sha=033bd89845a4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
@row_add n=13 row=decision/mcp-server-stdio-only area=threat-model-summary rec=fct-dgkl4a64@3,fct-5sreoq7r@1
name: MCP server on stdio only
rule: The MCP server is a stdio child an MCP client spawns, speaking on the invoking user's stdin and stdout, with no listener, port or socket.
@row_check n=13 row=decision/mcp-server-stdio-only sha=fc2027b428d9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:444
quote: Line::Mcp { record: None } => Ending::Spoken(serve_mcp(
at: packages/escher-driver/src/cli.rs:445-446
quote: io::stdin().lock(), io::stdout().lock(),
at: packages/escher-driver/src/mcp.rs:83
quote: /// log subscriber, reads no environment variable and writes nothing to `output` but replies.
@row_add n=14 row=contract/mcp-result/masked-control-values area=threat-model-summary rec=fct-dgkl4a64@3
boundary: MCP server results
item: control values
promise: A password's or file input's control value is carried masked.
@row_check n=14 row=contract/mcp-result/masked-control-values sha=4f6f224aa97b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
@row_add n=15 row=decision/mcp-results-client-responsibility area=threat-model-summary rec=fct-dgkl4a64@3
name: MCP results are the client's
rule: What an MCP client does with the results it receives, screenshots and lint findings among them, a model provider included, is the client's responsibility.
@row_check n=15 row=decision/mcp-results-client-responsibility sha=da11dcefd97d v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=16 row=absence/application-authentication area=authentication-authorization rec=fct-tf5bybpm@1
capability: Application authentication or authorization
@row_check n=16 row=absence/application-authentication sha=cc4fa604f466 v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)authenticat|authoriz|login|credential|x-api-key|api_key · in packages, apps, examples, tests (*.rs; only recording_tests.rs "login" label hits)
@row_add n=17 row=decision/cookie-store-behind-feature area=authentication-authorization rec=fct-dnpx5hao@1
name: Cookie store behind a feature
rule: The HTTP client keeps a cookie store only with the `cookies` feature.
@row_check n=17 row=decision/cookie-store-behind-feature sha=aaa560e458b2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:88-89
quote: #[cfg(feature = "cookies")] let builder = builder.cookie_store(true);
@row_add n=18 row=decision/publish-job-repository-guard area=authentication-authorization rec=fct-dnpx5hao@1
name: Publish job repository guard
rule: The publish job is repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.
@row_check n=18 row=decision/publish-job-repository-guard sha=f1ba97cf63b9 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:37
quote: if: github.repository == 'DioxusLabs/blitz'
@row_add n=19 row=contract/agent-run-cli/2 area=input-validation rec=fct-g4xhi6op@1
boundary: agent-run.sh exit codes
item: 2
promise: A verb outside boot run status cleanup logs, a wrong argument count, or a selection other than stand, all or an existing test-file stem prints usage and exits 2 before any cargo call.
@row_check n=19 row=contract/agent-run-cli/2 sha=12d91c8c1ac7 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:188
quote: [[ "$selection" =~ ^[a-z0-9_]+$ && -f "$TESTS_DIR/$selection.rs" ]] || usage
at: scripts/agent-run.sh:161
quote: [[ $# -ge 1 ]] && is_verb "$1" || usage
@row_add n=20 row=contract/cold-agent-cli/2 area=input-validation rec=fct-xl3lbcjg@1
boundary: cold-agent.sh exit codes
item: 2
promise: A verb outside run status cleanup logs, a wrong argument count or a task outside the allowlist stand exits 2 before any precondition or claude call.
@row_check n=20 row=contract/cold-agent-cli/2 sha=cacd16f5c207 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:556
quote: [[ $# -eq 2 ]] && contains "$2" "${TASKS[@]}" || usage
at: scripts/cold-agent.sh:16
quote: TASKS=(stand)
@row_add n=21 row=contract/apt-install-cli/2 area=input-validation rec=fct-rbli3fgb@1
boundary: apt-install.sh exit codes
item: 2
promise: Package names and option values that do not match closed forms print usage and exit 2 before sudo, timeout, apt-get or dpkg runs.
@row_check n=21 row=contract/apt-install-cli/2 sha=b1137b2a2f7d v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:39
quote: [[ "$1" =~ ^[a-z0-9][a-z0-9+.-]+$ ]] || usage
at: .github/scripts/apt-install.sh:30
quote: [[ $# -ge 2 ]] && is_count "$2" || usage
@row_add n=22 row=decision/apt-install-no-env-no-secrets area=input-validation rec=fct-rbli3fgb@1
name: Install script holds no environment or secret
rule: The apt install script reads no environment variable of its own, writes no file and names no secret.
@row_check n=22 row=decision/apt-install-no-env-no-secrets sha=ea316e3ca2bf v=holds judge=sonnet head=edd0cfaea7f0
searched: printenv|\$\{?[A-Z_]*(TOKEN|SECRET|PASSWORD|KEY)|export |>[^&] · in .github/scripts/apt-install.sh
@row_add n=23 row=contract/coverage-floor-cli/2 area=input-validation rec=fct-6uhcznyg@1
boundary: coverage_floor.py exit codes
item: 2
promise: A floor not matching CRATE_DIR=PERCENT (a percent at most 100, no crate twice), or a crate with no SF record, exits 2.
@row_check n=23 row=contract/coverage-floor-cli/2 sha=d7eb7de60392 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/coverage_floor.py:33
quote: if percent > 100 or crate in floors:
at: .github/scripts/coverage_floor.py:84-88
quote: missing = [crate for crate in floors if crate not in sums] if missing: for crate in missing: print(f"coverage floor: no lcov record under {crate}", file=sys.stderr) return 2
@row_add n=24 row=decision/coverage-floor-prints-no-paths area=input-validation rec=fct-6uhcznyg@1
name: Coverage floor prints no paths
rule: The coverage floor check prints only crate directories and counts, never a source or lcov path.
@row_check n=24 row=decision/coverage-floor-prints-no-paths sha=eb5f2e7238a5 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/coverage_floor.py:94
quote: print(f"{crate} {covered}/{lines} = {reading:.2f} % (floor {floor:g} %)")
@row_add n=25 row=decision/model-reply-untrusted-input area=input-validation rec=fct-atmkljiq@1
name: Model reply is untrusted input
rule: The cold-agent model's reply is untrusted: only its last fenced json block is read, bounded in bytes and steps, each step checked against closed keys; a check outside these is never replayed, and a valid one replays on fresh MCP children with every step validated again by the server.
@row_check n=25 row=decision/model-reply-untrusted-input sha=b4053627ecf9 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:276
quote: found = FENCE.findall(reply) if isinstance(reply, str) else []
at: scripts/cold-agent.sh:277
quote: if not found or len(found[-1].encode("utf-8")) > MAX_CHECK_BYTES:
at: scripts/cold-agent.sh:373
quote: if len(check) > MAX_CHECK_STEPS or not all(valid_step(step) for step in check):
at: scripts/cold-agent.sh:289
quote: and set(step) <= {"tool", "arguments", "expect"}
at: scripts/cold-agent.sh:341
quote: server = Server(binary)
@row_add n=26 row=decision/cold-agent-checks-never-printed area=input-validation rec=fct-atmkljiq@1
name: Cold-agent checks never printed
rule: The cold-agent check file is never printed, and the live child's kept checks are read for booleans and counts only and never printed.
@row_check n=26 row=decision/cold-agent-checks-never-printed sha=40d81d3d2cc3 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:259
quote: order). A kept check is parsed here and never printed or copied."""
at: scripts/cold-agent.sh:271
quote: return len(numbered), bool(starts) and len(names) == len(numbered) and opened == starts
@row_add n=27 row=decision/paged-answer-closed-decoding area=input-validation rec=fct-atmkljiq@1
name: Paged answer decoded by closed fields
rule: The cold-agent check decodes a paged answer only as exactly the fields text, part and parts.
@row_check n=27 row=decision/paged-answer-closed-decoding sha=09cc6ca7a3a0 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:184
quote: if (isinstance(line, dict) and set(line) == {"text", "part", "parts"} and isinstance(line["text"], str)
@row_add n=28 row=decision/driver-socket-closed-grammar area=input-validation rec=fct-6hwcr6nz@3
name: Driver socket closed request grammar
rule: The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
@row_check n=28 row=decision/driver-socket-closed-grammar sha=d94b99734da1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:182
quote: let parsed = match wire::read_line(&*stream, wire::MAX_REQUEST_BYTES)? {
at: packages/escher-driver/src/host.rs:210
quote: Err(refusal) => (refusal, false),
at: packages/escher-driver/src/host.rs:199
quote: let answer = session.run(&call);
@row_add n=29 row=contract/driver-session-reply/no-attribute-text area=input-validation rec=fct-6hwcr6nz@3
boundary: driver session socket replies
item: attribute text
promise: A reply never carries an attribute's text.
@row_check n=29 row=contract/driver-session-reply/no-attribute-text sha=3df925acd0dc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:150
quote: object.text("name", parts.name);
at: packages/escher-driver/src/json.rs:168
quote: object.flag("clickable", parts.clickable);
@row_add n=30 row=contract/driver-session-reply/state-readings area=input-validation rec=fct-6hwcr6nz@3
boundary: driver session socket replies
item: state readings
promise: State readings are carried as closed words and booleans.
@row_check n=30 row=contract/driver-session-reply/state-readings sha=185aa06f3287 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:155
quote: object.flag("checked", checked);
@row_add n=31 row=decision/boundary-widening-rule area=input-validation rec=fct-n4q5kc73@1
name: Boundary widening rule
rule: A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
@row_check n=31 row=decision/boundary-widening-rule sha=d206f0b83cc0 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=32 row=decision/state-directory-long-path-refused area=input-validation rec=fct-dx3vh2da@1
name: Too-long state directory path refused
rule: A state directory path too long for a socket address is refused with a fixed message holding no path.
@row_check n=32 row=decision/state-directory-long-path-refused sha=ff966c35cd5e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:131
quote: io::ErrorKind::InvalidInput => SessionError::StateDirTooLong,
at: packages/escher-driver/src/error.rs:65-66
quote: const STATE_DIR_TOO_LONG: &str = "the session state directory's path is too long for a socket: as given, it is at most 94 bytes";
@row_add n=33 row=decision/state-directory-non-recursive-delete area=input-validation rec=fct-dx3vh2da@1
name: Non-recursive session cleanup
rule: Only the socket file and then the state directory are ever deleted, non-recursively.
@row_check n=33 row=decision/state-directory-non-recursive-delete sha=c96a202ebc76 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:81
quote: let removed = fs::remove_file(&socket).map_err(io_error);
at: packages/escher-driver/src/host.rs:82
quote: let _ = fs::remove_dir(state_dir);
@row_add n=34 row=decision/state-directory-path-is-parameter area=input-validation rec=fct-dx3vh2da@1
name: State directory path is a parameter
rule: The state directory path is a parameter, never an environment read or a working-directory-relative default.
@row_check n=34 row=decision/state-directory-path-is-parameter sha=936026afa214 v=holds judge=sonnet head=edd0cfaea7f0
searched: env::var · in packages/escher-driver/src/host.rs
searched: current_dir · in packages/escher-driver/src/host.rs
searched: env:: · in packages/escher-driver/src
@row_add n=35 row=decision/recording-directory-owner-only area=input-validation rec=fct-klizv3ng@1
name: Owner-only recording directory
rule: The recording directory is created 0700, a group- or other-open one is refused with nothing written or spawned, each check is created 0600 and never replaced, and recording is unix only.
@row_check n=35 row=decision/recording-directory-owner-only sha=0c27aea1bd52 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:220
quote: Err(SessionError::RecordDirNotPrivate)
at: packages/escher-driver/src/recording.rs:231-232
quote: .create_new(true) .mode(0o600)
at: packages/escher-driver/src/recording.rs:286
quote: Err(SessionError::Unsupported)
@row_add n=36 row=decision/recording-ends-on-failure area=input-validation rec=fct-klizv3ng@1
name: Recording ends on failure
rule: A failed write or a passed bound ends the recording.
@row_check n=36 row=decision/recording-ends-on-failure sha=71d9585c578d v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=37 row=decision/recorder-removes-only-own-files area=input-validation rec=fct-klizv3ng@1
name: Recorder removes only its own files
rule: A file the recorder did not create is never removed.
@row_check n=37 row=decision/recorder-removes-only-own-files sha=055c632a2611 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:263
quote: let mut written = create_new(temporary)?;
at: packages/escher-driver/src/recording.rs:267-268
quote: if renamed.is_err() { let _ = fs::remove_file(temporary);
@row_add n=38 row=decision/recording-directory-never-named area=input-validation rec=fct-klizv3ng@1
name: Recording directory never named or logged
rule: No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
@row_check n=38 row=decision/recording-directory-never-named sha=0b9b1b22b57a v=holds judge=sonnet head=edd0cfaea7f0
searched: record · in packages/escher-driver/src/schema.rs
searched: info_span!|info!|warn!|debug! · in packages/escher-driver/src
@row_add n=39 row=decision/recorded-check-untrusted area=input-validation rec=fct-khxrpkj3@2
name: Recorded check bytes are untrusted
rule: A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
@row_check n=39 row=decision/recorded-check-untrusted sha=344efdfc75e9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:334
quote: if bytes.len() > MAX_CHECK_BYTES {
at: packages/escher-driver/src/recording.rs:340
quote: if records.len() > MAX_CHECK_STEPS {
at: packages/escher-driver/src/recording.rs:403
quote: let record = object_of(record, &["tool", "arguments"])?;
at: packages/escher-driver/src/cli.rs:503
quote: Check::from_bytes(&bytes).ok()
@row_add n=40 row=contract/driver-session-label/format area=input-validation rec=fct-76426k6u@1
boundary: driver session label
item: format
promise: A label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs.
@row_check n=40 row=contract/driver-session-label/format sha=eff791ef41f8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:56
quote: (1..=MAX_LABEL_BYTES).contains(&label.len())
at: packages/escher-driver/src/session.rs:153
quote: if !valid_label(label) {
@row_add n=41 row=decision/session-label-only-caller-string area=input-validation rec=fct-76426k6u@1
name: Session label is the one echoed caller string
rule: The session label is never text read from the document and is the one caller string a reply carries.
@row_check n=41 row=decision/session-label-only-caller-string sha=e316a1de0290 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:56
quote: (1..=MAX_LABEL_BYTES).contains(&label.len())
at: packages/escher-driver/src/host.rs:191
quote: label: session.label().to_string(),
@row_add n=42 row=decision/single-validation-gate area=input-validation rec=fct-cvyxuuhn@3,fct-5sreoq7r@1
name: Single command validation gate
rule: Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
@row_check n=42 row=decision/single-validation-gate sha=2a79c5d10fbe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:202
quote: let command = validate(call)?;
at: packages/escher-driver/src/cli.rs:205
quote: Level::Instance => validate(&call).map(|_| None),
at: packages/escher-driver/src/mcp.rs:238
quote: Level::Tool => match validate_session(call) {
@row_add n=43 row=decision/unknown-target-refused area=input-validation rec=fct-cvyxuuhn@3
name: Call on an unknown or unfit element refused
rule: A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
@row_check n=43 row=decision/unknown-target-refused sha=2996414c0537 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:223
quote: let (before, target) = resolve(harness, seen, id)?;
at: packages/escher-driver/src/execute.rs:224
quote: let (x, y) = check_target(harness, &command, &target)?.point();
@row_add n=44 row=decision/command-schema-no-clock area=input-validation rec=fct-vehorezx@1
name: Command schema reads no clock
rule: The command schema and its validation read no clock.
@row_check n=44 row=decision/command-schema-no-clock sha=124da46380fe v=holds judge=sonnet head=edd0cfaea7f0
searched: Instant::now|SystemTime · in packages/escher-driver/src/command.rs and schema.rs (none; clock reads stand in session.rs, client.rs, host.rs, mcp.rs only)
@row_add n=45 row=decision/no-node-id-between-calls area=input-validation rec=fct-vehorezx@1
name: No NodeId kept between calls
rule: No NodeId is kept between calls.
@row_check n=45 row=decision/no-node-id-between-calls sha=a4e1f4cb900a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:80
quote: last_read: HashMap<String, u64>,
@row_add n=46 row=decision/command-span-content-free area=input-validation rec=fct-vehorezx@1
name: Command span holds no content
rule: One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
@row_check n=46 row=decision/command-span-content-free sha=b1502e8a5b43 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:124
quote: command_span!(verb, cause, settled, busy, passes, added, removed, changed);
@row_add n=47 row=decision/driver-no-subscriber-no-env area=input-validation rec=fct-vehorezx@1,fct-4nuyglo3@1
name: Driver installs no subscriber and reads no env
rule: The driver crate and its session command line install no tracing subscriber and read no environment variable.
@row_check n=47 row=decision/driver-no-subscriber-no-env sha=1a2eec36aaad v=holds judge=sonnet head=edd0cfaea7f0
searched: set_global_default|tracing_subscriber|env::var|var_os · in packages/escher-driver/src (none; cli.rs:521 reads current_exe only)
@row_add n=48 row=decision/cli-closed-argv-reader area=input-validation rec=fct-4nuyglo3@1
name: Closed argv reader
rule: The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
@row_check n=48 row=decision/cli-closed-argv-reader sha=11ed4f52625e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:203-206
quote: let (call, state_dirs, records) = read_arguments(spec, rest); let checked = match spec.level { Level::Instance => validate(&call).map(|_| None), Level::Session | Level::Tool => validate_session(&call).map(Some),
@row_add n=49 row=decision/cli-echoes-nothing-typed area=input-validation rec=fct-4nuyglo3@1
name: Command line echoes nothing typed
rule: No command-line ending writes back anything the caller typed, and no path on argv panics.
@row_check n=49 row=decision/cli-echoes-nothing-typed sha=8927100dbd2e v=holds judge=sonnet head=edd0cfaea7f0
searched: unwrap()|expect(|panic!|println! · in packages/escher-driver/src/cli.rs outside tests (none)
@row_add n=50 row=contract/mcp-server/transport area=input-validation rec=fct-5sreoq7r@1
boundary: MCP server
item: transport
promise: Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.
@row_check n=50 row=contract/mcp-server/transport sha=b2a554f52779 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:445-446
quote: io::stdin().lock(), io::stdout().lock(),
@row_add n=51 row=contract/mcp-server/protocol-error area=input-validation rec=fct-5sreoq7r@1
boundary: MCP server
item: protocol error
promise: A protocol error is a fixed code and message with no data.
@row_check n=51 row=contract/mcp-server/protocol-error sha=d262d6797cc3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:190-194
quote: pub(super) fn error_reply(id: &Id, error: ProtocolError) -> String { let mut object = Object::new(); object.json("code", &error.code().to_string()); object.text("message", error.message()); reply(id, "error", &object.end())
@row_add n=52 row=contract/mcp-server/request-id area=input-validation rec=fct-5sreoq7r@1
boundary: MCP server
item: request id
promise: The admitted request id is the one client-sent value written back.
@row_check n=52 row=contract/mcp-server/request-id sha=21865ba2ea22 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:82
quote: Value::String(text) if text.len() <= MAX_ID_BYTES => Some(Id::Text(text.clone())),
@row_add n=53 row=decision/mcp-bounded-lines area=input-validation rec=fct-5sreoq7r@1
name: Bounded MCP requests and results
rule: MCP request lines and results are bounded, and tool arguments are typed before validation.
@row_check n=53 row=decision/mcp-bounded-lines sha=9d98a5574024 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:11
quote: pub(super) const MAX_MCP_REQUEST_BYTES: usize = 65_536;
at: packages/escher-driver/src/mcp/result.rs:74
quote: pub(super) fn bounded(self) -> ToolResult {
at: packages/escher-driver/src/mcp.rs:204
quote: let result = match call_of(verb, arguments) {
@row_add n=54 row=decision/mcp-no-print-or-panic area=input-validation rec=fct-5sreoq7r@1
name: MCP server never prints or panics
rule: The MCP server has no print, unwrap, expect or panic outside the tests.
@row_check n=54 row=decision/mcp-no-print-or-panic sha=8ab4c8e26b2b v=holds judge=sonnet head=edd0cfaea7f0
searched: println|print!|unwrap()|expect(|panic! · in packages/escher-driver/src/mcp.rs and packages/escher-driver/src/mcp/ outside tests (none)
@row_add n=55 row=decision/external-navigation-schemes area=input-validation rec=fct-bklgnkmy@1
name: External navigation limited
rule: Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
@row_check n=55 row=decision/external-navigation-schemes sha=2c65e782cde3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/link_handler.rs:10-11
quote: if options.method == Method::GET && matches!(options.url.scheme(), "http" | "https" | "mailto")
@row_add n=56 row=decision/html-parsed-scripting-disabled area=input-validation rec=fct-hfzht5qa@1
name: HTML parsed with scripting disabled
rule: HTML is parsed with scripting disabled.
@row_check n=56 row=decision/html-parsed-scripting-disabled sha=eb006c6dd781 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-html/src/html_sink.rs:109
quote: scripting_enabled: false, // Enables parsing of <noscript> tags
@row_add n=57 row=decision/no-agent-urls-through-font-path area=input-validation rec=fct-tqsve6xl@1
name: No agent URL through font sources
rule: escher routes no agent- or user-supplied URL through the font-face source path.
@row_check n=57 row=decision/no-agent-urls-through-font-path sha=48b4399f30e1 v=holds judge=sonnet head=edd0cfaea7f0
searched: fetch_font_face · in packages/ (callers: blitz-dom/src/document.rs:1762 and net.rs:286, both stylesheet-driven; none from driver commands)
@row_add n=58 row=decision/select-size-hint-cap area=input-validation rec=fct-w5f54hx5@1
name: Select size hint capped
rule: The `size` attribute on a `multiple` select is read as a hint capped at 65534, with `0` or a non-numeric value giving no hint.
why: Ratified as an engine change for every document and a boundary widening.
@row_check n=58 row=decision/select-size-hint-cap sha=0c4f085e5443 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select_list.rs:129
quote: (rows > 0).then_some(rows.min(MAX_SIZE))
at: packages/blitz-dom/src/node/select_list.rs:26
quote: const MAX_SIZE: u32 = 65534;
@row_add n=59 row=decision/attribute-readers-no-log-or-panic area=input-validation rec=fct-gtdrmcll@1
name: Markup attribute readers without log or panic
rule: Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
@row_check n=59 row=decision/attribute-readers-no-log-or-panic sha=21328722288c v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: tracing::|panic! · in packages/blitz-dom/src attribute readers (too broad a claim to bound)
why: no evidence found
@row_add n=60 row=decision/attribute-text-leaves-by-ratified-carriers area=input-validation rec=fct-gtdrmcll@1
name: Attribute text leaves only by ratified carriers
rule: Markup attribute values, names and descriptions leave escher only by the ratified carriers.
except: The page's own form submission and events.
@row_check n=60 row=decision/attribute-text-leaves-by-ratified-carriers sha=4ee7a75e3900 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: no evidence found (ratified-carrier set is a process matter; no bounded search settles it)
@row_add n=61 row=decision/select-list-keys-and-painted-labels area=input-validation rec=fct-gtdrmcll@1
name: Single select list keys and painted labels
rule: Enter and Space open a single select's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.
why: Ratified as an engine change for every document and a boundary widening.
@row_check n=61 row=decision/select-list-keys-and-painted-labels sha=c6d520175bd7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:195
quote: (Key::Named(NamedKey::Enter), None) => {
at: packages/blitz-dom/src/events/keyboard.rs:200
quote: if text == " " && !doc.select_text_stands(node_id) {
at: packages/blitz-paint/src/render/select_list.rs:100
quote: let Some(layout) = self.dom.select_row_layout(row.node, &row.label) else {
@row_add n=62 row=contract/stable-element-id/derivation area=input-validation rec=fct-4jl4t3ye@1
boundary: The stable element id
item: derivation
promise: The HTML `id` is the author key when non-empty, `/`-free and first in document order; otherwise an anchored, component or document path is used, so no `id` can make two ids equal.
@row_check n=62 row=contract/stable-element-id/derivation sha=6843f1e63baf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:346-350
quote: let author_key = node .element_data() .and_then(|element| element.attr(local_name!("id"))) .filter(|key| !key.is_empty() && !key.contains('/')) .filter(|key| claimed_keys.insert(key.to_string()));
@row_add n=63 row=contract/stable-element-id/carriers area=input-validation rec=fct-4jl4t3ye@1
boundary: The stable element id
item: carriers
promise: The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
@row_check n=63 row=contract/stable-element-id/carriers sha=9315ff7a2a34 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: stable element id · in packages/dioxus-native-dom/src/snapshot.rs
searched: stable element id · in packages/escher-driver/src
why: no evidence found for all four carriers and the no-log claim together
@row_add n=64 row=contract/stable-element-id/no-node-id area=input-validation rec=fct-4jl4t3ye@1
boundary: The stable element id
item: NodeId
promise: No `NodeId` crosses any carrier of the stable element id.
@row_check n=64 row=contract/stable-element-id/no-node-id sha=3e8ceed44fa0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:59
quote: let mut elements: HashMap<String, NodeId> = HashMap::new();
searched: NodeId · in packages/escher-driver/src (used only internally in actions.rs and lint.rs, never written to an answer)
@row_add n=65 row=decision/masked-input-values area=input-validation rec=fct-767qdy6s@1,fct-mkaeodr2@1
name: Password and file input values masked
rule: A password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path.
except: The DOM and form submission keep the value.
@row_check n=65 row=decision/masked-input-values sha=7405d1e37f50 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:13
quote: pub const MASKED_VALUE: &str = "••••••••";
at: packages/dioxus-native-dom/src/snapshot.rs:253
quote: input_type.eq_ignore_ascii_case("password") || input_type.eq_ignore_ascii_case("file")
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
@row_add n=66 row=decision/password-painted-as-bullets area=input-validation rec=fct-767qdy6s@1
name: Password input painted as bullets
rule: A password input paints one bullet per character, never its text.
@row_check n=66 row=decision/password-painted-as-bullets sha=86cf94f42743 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:1030
quote: let bullets: String = text.chars().map(|_| BULLET).collect();
@row_add n=67 row=contract/masked-input-value/masked-stand-in area=input-validation rec=fct-767qdy6s@1
boundary: Masked input values
item: MASKED_STAND_IN
promise: A recorded masked `type` is kept as `MASKED_STAND_IN`.
@row_check n=67 row=contract/masked-input-value/masked-stand-in sha=21205b0a1d32 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:121
quote: arguments.text(arg.name, MASKED_STAND_IN);
@row_add n=68 row=decision/reached-handler-record-without-log area=input-validation rec=fct-frlob5k5@2
name: Reached-handler record without log or NodeId
rule: Where recording of reached handlers is switched on, each reached handler's stable id and event name is written into a root-context record ahead of the handler's dispatch, with no log site and no `NodeId`.
@row_check n=68 row=decision/reached-handler-record-without-log sha=fb240d5635a6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:452
quote: if let Some(record) = runtime.has_context::<ReachedRecord>(ScopeId::ROOT) {
at: packages/dioxus-native-dom/src/dioxus_document.rs:465
quote: runtime.handle_event(event.name(), dx_event.clone(), id);
at: packages/dioxus-native-dom/src/reached.rs:93
quote: runtime.provide_context(ScopeId::ROOT, ReachedRecord::default());
@row_add n=69 row=decision/signal-registry-holds-no-state-value area=input-validation rec=fct-frlob5k5@2
name: Signal registry holds no state value
rule: The in-process signal registry takes dioxus-core types alone, holds no value of a state, and has no log site, print, env read or `unwrap` on app input.
@row_check n=69 row=decision/signal-registry-holds-no-state-value sha=2cbc793cd020 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:51-55
quote: pub fn use_graph_state( name: &'static str, writers: &'static [DeclaredWrite], subscribers: Subscribers, ) {
@row_add n=70 row=decision/js-fetch-get-or-head-only area=input-validation rec=fct-ogo4rqfs@1,fct-7xstfz5r@1
name: JS fetch limited to GET and HEAD
rule: JS-facing `fetch()` takes GET or HEAD only.
@row_check n=70 row=decision/js-fetch-get-or-head-only sha=32e37d45fc49 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:82
quote: if (method !== "GET" && method !== "HEAD") {
@row_add n=71 row=decision/sqlite-bound-parameters area=input-validation rec=fct-urxev64g@1
name: SQLite values bound through placeholders
rule: Values written to the sqlite persistence are bound through `params!` placeholders, never spliced into SQL text.
@row_check n=71 row=decision/sqlite-bound-parameters sha=f1d5712256df v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:242
quote: params![url_str, &entry.title, favicon_str, visited_secs],
at: apps/browser/persistence/src/lib.rs:271
quote: params![favicon_url.as_str(), page_url.as_str()],
@row_add n=72 row=decision/serialization-escapes-text-and-attributes area=input-validation rec=fct-ss2oxfgi@1
name: Serialization escapes text and attribute values
rule: Serialized output escapes text and attribute values.
except: Raw-text elements' text is written unescaped.
@row_check n=72 row=decision/serialization-escapes-text-and-attributes sha=2b8f883fee0c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/serialize.rs:158
quote: encode_text_to_string(data.content.as_str(), writer);
at: packages/blitz-dom/src/node/serialize.rs:193
quote: encode_quoted_attribute_to_string(&attr.value, writer);
at: packages/blitz-dom/src/node/serialize.rs:155-156
quote: if in_raw_text_element { writer.push_str(data.content.as_str());
@row_add n=73 row=contract/serialization-output/inline-svg area=input-validation rec=fct-ss2oxfgi@1
boundary: Serialization output
item: inline SVG
promise: Inline SVG serializes as well-formed XML.
@row_check n=73 row=contract/serialization-output/inline-svg sha=89d955024f37 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/serialize.rs:197-200
quote: if !has_children { writer.push_str(" /"); } writer.push('>');
@row_add n=74 row=decision/tls-via-native-tls area=data-protection rec=fct-aszkhz5z@1
name: TLS through native-tls
rule: TLS is provided through reqwest's `native-tls` feature, `native-tls-vendored` on Android.
@row_check n=74 row=decision/tls-via-native-tls sha=42d766507820 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/Cargo.toml:26
quote: reqwest = { workspace = true, features = ["charset", "native-tls", "form"] }
at: packages/blitz-net/Cargo.toml:38
quote: reqwest = { workspace = true, features = ["native-tls-vendored"] }
@row_add n=75 row=decision/history-in-sqlite area=data-protection rec=fct-brg4y6b3@1
name: Browser history stored in sqlite
rule: Browser history is written to a sqlite file; on mobile it is in-memory only and does not persist across launches.
@row_check n=75 row=decision/history-in-sqlite sha=6ae16e8b4dae v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:290
quote: Connection::open(&db_path).map_err(|e| format!("open({}): {e}", db_path.display()))
at: apps/browser/persistence/src/lib.rs:12
quote: //! - On mobile (Android, iOS) the store always opens an in-memory connection;
@row_add n=76 row=absence/database-encryption area=data-protection rec=fct-brg4y6b3@1
capability: Encryption of the history database
@row_check n=76 row=absence/database-encryption sha=b02ab1a6e865 v=holds judge=sonnet head=edd0cfaea7f0
searched: encrypt|cipher|sqlcipher (case-insensitive) · in apps/browser
@row_add n=77 row=decision/private-on-disk-http-cache area=data-protection rec=fct-73ojmpsr@1
name: Private on-disk HTTP cache
rule: With feature `cache`, HTTP responses are cached on disk at the platform cache directory, and the cache policy is evaluated as a private, not shared, cache.
@row_check n=77 row=decision/private-on-disk-http-cache sha=57e32af8bae4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:93
quote: let cache_manager = CACacheManager::new(get_cache_path(), true);
at: packages/blitz-net/src/lib.rs:110
quote: shared: false,
@row_add n=78 row=decision/cookies-and-cache-default-features area=data-protection rec=fct-73ojmpsr@1
name: Cookies and HTTP cache on by default
rule: Cookies and the HTTP cache are default features of the browser.
@row_check n=78 row=decision/cookies-and-cache-default-features sha=9cf532c83f6a v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:14
quote: default = ["hybrid", "cookies", "cache", "screenshot", "apple-font-embolden", "scrollbars", "mimalloc"]
@row_add n=79 row=contract/kept-check-file/medium area=data-protection rec=fct-kdtkdrex@1
boundary: Kept check files
item: medium
promise: Under `--record <dir>` or `Session::recording_into`, one `check-{n}.json` with mode `0600` per recorded session is written in a `0700` directory the launcher named.
@row_check n=79 row=contract/kept-check-file/medium sha=f8186bf0a190 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
at: packages/escher-driver/src/recording.rs:244
quote: let path = dir.join(format!("check-{n}.json"));
@row_add n=80 row=contract/kept-check-file/contents area=data-protection rec=fct-kdtkdrex@1
boundary: Kept check files
item: contents
promise: A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
@row_check n=80 row=contract/kept-check-file/contents sha=d55347b3d96b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:57
quote: Ok(Outcome::Shot { width, height, .. }) => shot_without_image(*width, *height),
at: packages/escher-driver/src/recording.rs:180
quote: self.records.push(record_of(verb, call, masked, answer));
@row_add n=81 row=contract/kept-check-file/lifecycle area=data-protection rec=fct-kdtkdrex@1
boundary: Kept check files
item: lifecycle
promise: A kept check is never logged and never removed by escher, but for its own check when a recording ends.
@row_check n=81 row=contract/kept-check-file/lifecycle sha=a7505b37a990 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:185-186
quote: if !written { imp::remove(&self.file);
searched: info!|debug!|trace!|warn!|error! with check/record · in packages/escher-driver/src
@row_add n=82 row=decision/cold-agent-checks-read-for-counts-only area=data-protection rec=fct-kdtkdrex@1
name: Cold-agent checks read for bools and counts only
rule: The cold-agent pipe's live child keeps its checks in an owner-only record directory, read by the pipe for bools and counts only, never printed, copied or committed, and removed by its cleanup.
@row_check n=82 row=decision/cold-agent-checks-read-for-counts-only sha=1898e755664c v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:571
quote: mkdir -m 700 "$STATE_DIR/record"
at: scripts/cold-agent.sh:595
quote: rm -rf "$STATE_DIR"
@row_add n=83 row=decision/committed-counter-checks area=data-protection rec=fct-sw7lpzl2@1
name: Committed stand-Counter checks
rule: The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`.
why: The one exception to a kept check's owner-only medium.
@row_check n=83 row=decision/committed-counter-checks sha=31739722653d v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_replay.rs:53
quote: let check = Check::from_bytes(bytes).expect("a committed check reads");
@row_add n=84 row=decision/log-sink-redacts-and-allowlists area=data-protection rec=fct-mbxzqngq@1
name: Log sink redacts and allowlists targets
rule: Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
@row_check n=84 row=decision/log-sink-redacts-and-allowlists sha=b40df2fddcca v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:98-99
quote: if is_outside_target(target) {         Verdict::Drop
at: packages/escher-telemetry/src/format.rs:107
quote: Verdict::Redact
@row_add n=85 row=decision/clipboard-via-arboard area=data-protection rec=fct-j52pcemz@1
name: Clipboard through arboard
rule: Clipboard text is read and written through arboard on desktop OSes, behind the clipboard feature.
@row_check n=85 row=decision/clipboard-via-arboard sha=911771e38de6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:175
quote: let mut cb = arboard::Clipboard::new().unwrap();
at: packages/blitz-shell/Cargo.toml:20
quote: clipboard = ["dep:arboard"]
@row_add n=86 row=decision/file-dialog-via-rfd area=data-protection rec=fct-e6uumj43@1
name: Native file dialog through rfd
rule: The native file dialog uses rfd, with optional name and extension filters, behind the file-dialog feature.
@row_check n=86 row=decision/file-dialog-via-rfd sha=e642026e44c4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:215-217
quote: let mut dialog = rfd::FileDialog::new();         if let Some(FileDialogFilter { name, extensions }) = filter {             dialog = dialog.add_filter(&name, &extensions);
at: packages/blitz-shell/Cargo.toml:22
quote: file-dialog = ["dep:rfd"]
@row_add n=87 row=overview/api-security area=api-security rec=fct-s4jqcqqv@1
text: No served network API exists: no binary serves a TCP or UDP port. The one listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The remaining controls govern outbound requests and in-document navigation.
@row_check n=87 row=overview/api-security sha=6d544db861a7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
at: packages/escher-driver/src/cli.rs:445
quote: io::stdin().lock(),
@row_add n=88 row=decision/per-host-request-concurrency area=api-security rec=fct-5zz6ux3p@1
name: Outbound requests limited per host
rule: At most 6 outbound requests run concurrently per host.
@row_check n=88 row=decision/per-host-request-concurrency sha=1291d083f4b9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:24
quote: const PER_HOST_MAX_CONCURRENT: usize = 6;
at: packages/blitz-net/src/lib.rs:191
quote: .or_insert_with(|| Arc::new(Semaphore::new(PER_HOST_MAX_CONCURRENT)))
@row_add n=89 row=absence/cors-checks area=api-security rec=fct-5zz6ux3p@1
capability: CORS checks on outbound requests
@row_check n=89 row=absence/cors-checks sha=4384e35bded0 v=holds judge=sonnet head=edd0cfaea7f0
searched: Access-Control · in packages/{blitz-net,blitz-dom,blitz-traits,dioxus-native,blitz-shell}/**/*.rs
searched: cross.origin · in packages/{blitz-net,blitz-dom,blitz-traits,dioxus-native,blitz-shell}/**/*.rs
@row_add n=90 row=absence/same-origin-checks area=api-security rec=fct-5zz6ux3p@1
capability: Same-origin checks on outbound requests
@row_check n=90 row=absence/same-origin-checks sha=9e83d00faeb0 v=holds judge=sonnet head=edd0cfaea7f0
searched: same.origin · in packages/{blitz-net,blitz-dom,blitz-traits,dioxus-native,blitz-shell}/**/*.rs
@row_add n=91 row=absence/response-size-limit area=api-security rec=fct-5zz6ux3p@1
capability: A response size limit on outbound requests
@row_check n=91 row=absence/response-size-limit sha=341280e77b6c v=holds judge=sonnet head=edd0cfaea7f0
searched: content_length · in packages/{blitz-net,blitz-dom,blitz-traits,dioxus-native,blitz-shell}/**/*.rs
searched: max_response · in packages/{blitz-net,blitz-dom,blitz-traits,dioxus-native,blitz-shell}/**/*.rs
@row_add n=92 row=decision/bounded-load-wait area=api-security rec=fct-dlhusvs4@1
name: Bounded wait on loads in flight
rule: A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`.
why: So the absent request timeout cannot hang a settle or a command.
@row_check n=92 row=decision/bounded-load-wait sha=d0cbeb3e89fb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:176
quote: return Err(NotSettled { busy: Busy::Loads });
at: packages/escher-driver/src/session.rs:28
quote: settle_within(|| harness.settle(), LOAD_WAIT, LOAD_POLL)
@row_add n=93 row=decision/no-wait-verb-fields area=api-security rec=fct-dlhusvs4@1
name: No wait argument or result on verbs
rule: No driver verb has an argument or a result field of a wait.
@row_check n=93 row=decision/no-wait-verb-fields sha=86d00e10309f v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)\bwait · in packages/escher-driver/src/{schema,wire,execute,mcp}.rs (only a doc comment and the step help text, no argument or result field)
searched: (?i)timeout · in packages/escher-driver/src/{schema,wire,execute,mcp}.rs
@row_add n=94 row=decision/subresource-requests-get-only area=api-security rec=fct-7xstfz5r@1
name: Sub-resource requests are GETs
rule: Sub-resource requests are issued as GETs only.
@row_check n=94 row=decision/subresource-requests-get-only sha=382295a4c339 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/net.rs:69
quote: method: Method::GET,
@row_add n=95 row=decision/form-dialog-method-rejected area=api-security rec=fct-7xstfz5r@1
name: Form submission by scheme and method
rule: Form submission goes by scheme and method, and the `dialog` method is rejected.
@row_check n=95 row=decision/form-dialog-method-rejected sha=32b7b917708a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:115
quote: match (scheme, method) {
at: packages/blitz-dom/src/form.rs:390
quote: FormMethod::Dialog => return Err("Dialog is not an HTTP method"),
@row_add n=96 row=decision/iframe-navigation-to-parent area=api-security rec=fct-7xstfz5r@1
name: Iframe navigations routed to the parent
rule: Iframe navigations route back to the parent document.
@row_check n=96 row=decision/iframe-navigation-to-parent sha=3e79b2f68847 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/iframe.rs:40
quote: let _ = self.tx.send(DocumentEvent::NavigateIframe {
@row_add n=97 row=decision/external-targets-to-system-browser area=api-security rec=fct-7xstfz5r@1
name: External targets open in the system browser
rule: Navigation to external targets goes to the system browser.
@row_check n=97 row=decision/external-targets-to-system-browser sha=a25186b50fa4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/link_handler.rs:12
quote: && let Err(_err) = webbrowser::open(options.url.as_str())
@row_add n=98 row=decision/dioxus-scheme-local-files area=api-security rec=fct-7xstfz5r@1
name: Dioxus scheme served from local files
rule: The `dioxus` scheme is served from the local file system.
@row_check n=98 row=decision/dioxus-scheme-local-files sha=6f7d4cba800a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/assets.rs:45
quote: match dioxus_asset_resolver::native::serve_asset(request.url.path()) {
@row_add n=99 row=decision/dioxus-native-js-eval-noop area=api-security rec=fct-7xstfz5r@1
name: Dioxus-native JS evaluation is a no-op
rule: Dioxus-native JS evaluation goes to a no-op document.
@row_check n=99 row=decision/dioxus-native-js-eval-noop sha=07548d42bcf2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/contexts.rs:20
quote: NoOpDocument.eval(_js)
@row_add n=100 row=decision/outbound-http-via-reqwest-or-blitz-net area=api-security rec=fct-dsoqkdxe@1
name: Outbound HTTP through reqwest or blitz-net
rule: Outbound HTTP in the examples and apps goes through reqwest or blitz-net.
@row_check n=100 row=decision/outbound-http-via-reqwest-or-blitz-net sha=a98a2213c020 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/screenshot.rs:45
quote: let client = reqwest::Client::new();
at: apps/browser/src/main.rs:19
quote: pub(crate) type StdNetProvider = blitz_net::Provider;
@row_add n=101 row=decision/ci-api-via-gh-cli area=api-security rec=fct-dsoqkdxe@1
name: CI API calls through the gh CLI
rule: CI API calls go through the `gh api` CLI with the workflow token.
@row_check n=101 row=decision/ci-api-via-gh-cli sha=bb2d387a4384 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/wpt-post-results.yml:36
quote: PR_NUMBER="$(gh api --paginate "repos/$REPO/pulls?state=open&per_page=100" --jq '.[] | select(.head.sha == env.HEAD_SHA) | .number' | head -n 1)"
@row_add n=102 row=decision/session-socket-one-bounded-connection area=api-security rec=fct-sn4avmoz@1
name: One bounded connection at a time
rule: The local session socket serves one connection at a time under fixed read, write and size bounds.
@row_check n=102 row=decision/session-socket-one-bounded-connection sha=a463dcd10a14 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:146
quote: let mut stream = match listener.accept() {
at: packages/escher-driver/src/host.rs:180-181
quote: stream.set_read_timeout(Some(wire::IO_BOUND))?; stream.set_write_timeout(Some(wire::IO_BOUND))?;
at: packages/escher-driver/src/host.rs:182
quote: let parsed = match wire::read_line(&*stream, wire::MAX_REQUEST_BYTES)? {
@row_add n=103 row=decision/session-host-idle-expiry area=api-security rec=fct-sn4avmoz@1
name: Session host idle expiry
rule: A session host that answers no request for its idle expiry ends as on `stop`.
@row_check n=103 row=decision/session-host-idle-expiry sha=9300af15923c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:149-150
quote: if last_answer.elapsed() >= idle_expiry { return Ok(());
@row_add n=104 row=decision/audit-ignores-per-id-with-reason area=dependency-security rec=fct-cappgruj@1
name: Audit ignores per ID with a written reason
rule: Audit ignores are per advisory ID only, each with a written reason, no blanket allow, and each ignore is re-read on every push.
@row_check n=104 row=decision/audit-ignores-per-id-with-reason sha=2dd038f10b43 v=holds judge=sonnet head=edd0cfaea7f0
at: deny.toml:17-21
quote: [advisories] # Per-ID ignores only, each a bounded deferral: the audit leg re-reads the advisory database on every # push, so a new advisory, or a fix landing upstream, surfaces at the next run. ignore = [
@row_add n=105 row=decision/no-git-dependencies area=dependency-security rec=fct-hhqzsgtc@1
name: No git dependencies in the root manifest
rule: The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile.
@row_check n=105 row=decision/no-git-dependencies sha=c40f14bcab82 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:105
quote: taffy = { version = "0.15", default-features = false, features = [
at: Cargo.toml:115
quote: parley = { version = "0.12", default-features = false, features = ["std"] }
at: Cargo.lock:8040-8041
quote: source = "registry+https://github.com/rust-lang/crates.io-index" checksum = "3c8480887e8803986d96a7173729565520d4bbb721ad6b7ad6941a22897207aa"
@row_add n=106 row=decision/upstream-sync-dependency-set area=dependency-security rec=fct-p7wg5ump@1
name: Upstream sync takes the dependency set whole
rule: Each upstream sync takes upstream's dependency set whole or, by cherry-pick, enters no package; its lock is read name by name against upstream's or the prior tree, and the audit leg is green after it.
@row_check n=106 row=decision/upstream-sync-dependency-set sha=ffa6fb481b31 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=107 row=decision/locked-builds area=dependency-security rec=fct-kwog532k@1,fct-ehhtk64b@1
name: Locked builds
rule: Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified.
except: examples/wasm_hello, a standalone workspace with no Cargo.lock
@row_check n=107 row=decision/locked-builds sha=b320820d79ac v=holds judge=sonnet head=edd0cfaea7f0
at: flake.nix:100
quote: cargoExtraArgs = "--locked --package ${package} ${
at: .github/workflows/publish-browser.yml:157
quote: if dx bundle --package browser --release --profile production --target ${{ matrix.platform.target }} --locked --verbose --trace ${{ matrix.platform.args }}; then
at: .github/scripts/ci-leg.sh:40
quote: audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
at: .github/workflows/ci.yml:415
quote: ${{ env.RUST_CARGO_COMMAND }} ${{ matrix.platform.command }} ${{ matrix.platform.args }} --target ${{ matrix.platform.target }} --locked 2>&1 | tee "target/ci-logs/matrix-${{ matrix.platform.name }}.log"
@row_add n=108 row=decision/workspace-inherited-dependency-versions area=dependency-security rec=fct-wn5xussh@1
name: Workspace-inherited dependency versions
rule: Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
@row_check n=108 row=decision/workspace-inherited-dependency-versions sha=0c56e3ef43df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:14
quote: blitz-test-harness = { workspace = true }
at: packages/blitz-test-harness/Cargo.toml:15
quote: blitz-dom = { workspace = true, features = ["accessibility", "system-fonts"] }
@row_add n=109 row=decision/serde-json-untrusted-input-reader area=dependency-security rec=fct-ldu2bkhj@1
name: serde_json as the untrusted input reader
rule: serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.
@row_check n=109 row=decision/serde-json-untrusted-input-reader sha=a9a4676a70f7 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:165
quote: serde_json = "1"
at: packages/escher-driver/Cargo.toml:17
quote: serde_json = { workspace = true }
@row_add n=110 row=decision/png-shot-encoder area=dependency-security rec=fct-ei3z3lut@1
name: png as the shot encoder
rule: The png crate is the encoder of a shot in escher-driver.
@row_check n=110 row=decision/png-shot-encoder sha=0c6b307380a0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:16
quote: png = { workspace = true }
@row_add n=111 row=decision/color-crate-luminance-source area=dependency-security rec=fct-qbfed6vz@1
name: color crate as luminance source
rule: The test harness's contrast reader takes its luminance from the color crate already in the graph through blitz-dom, adding no package to Cargo.lock.
@row_check n=111 row=decision/color-crate-luminance-source sha=cbd582206403 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/Cargo.toml:27
quote: color = { workspace = true }
@row_add n=112 row=decision/test-harness-no-tracing-no-style area=dependency-security rec=fct-qbfed6vz@1
name: Test harness takes no tracing and no style
rule: blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
@row_check n=112 row=decision/test-harness-no-tracing-no-style sha=27a024d79adf v=holds judge=sonnet head=edd0cfaea7f0
searched: tracing · in packages/blitz-test-harness/Cargo.toml
searched: style · in packages/blitz-test-harness/Cargo.toml
@row_add n=113 row=decision/cargo-deny-audit-tool area=dependency-security rec=fct-4wzxuf2u@1,fct-cfff7mts@1
name: cargo-deny as the audit tool
rule: cargo-deny 0.20.2, with the root deny.toml, is the dependency audit tool.
@row_check n=113 row=decision/cargo-deny-audit-tool sha=79c4a8663207 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:236
quote: tool: cargo-deny@0.20.2
at: deny.toml:1
quote: # cargo-deny configuration for the `audit` leg (`bash .github/scripts/ci-leg.sh audit`, which runs
@row_add n=114 row=decision/cargo-llvm-cov-coverage-tool area=dependency-security rec=fct-4wzxuf2u@1
name: cargo-llvm-cov as the coverage tool
rule: cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
@row_check n=114 row=decision/cargo-llvm-cov-coverage-tool sha=73dfaf7f4768 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:283
quote: tool: cargo-llvm-cov@0.9.1
@row_add n=115 row=decision/blitz-tests-not-published area=dependency-security rec=fct-44piipwj@1
name: blitz-tests not published
rule: The blitz-tests crate is never published (publish = false).
@row_check n=115 row=decision/blitz-tests-not-published sha=9f083687d248 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/Cargo.toml:4
quote: publish = false
@row_add n=116 row=absence/automated-dependency-updates area=dependency-security rec=fct-ivgzdi2j@1
capability: Automated dependency update tooling, such as dependabot
@row_check n=116 row=absence/automated-dependency-updates sha=e533f4d3a0e0 v=holds judge=sonnet head=edd0cfaea7f0
searched: dependabot · in .github/**, Cargo.toml, Cross.toml, flake.nix
searched: renovate · in .github/**, Cargo.toml, Cross.toml, flake.nix
@row_add n=117 row=decision/dependency-audit-ci-gate area=dependency-security rec=fct-6cupfjou@1,fct-v457huyo@1
name: Dependency audit CI gate
rule: The CI audit job runs the dependency audit on every pull request and push, in the slow tier behind the fast jobs, uncached, its log uploaded on failure.
@row_check n=117 row=decision/dependency-audit-ci-gate sha=a46f6e1911f2 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:225-228
quote: audit: name: "Dependency audit" runs-on: ubuntu-latest needs: [fmt, clippy, test-features-default, ci-scripts]
at: .github/workflows/ci.yml:237-242
quote: - run: bash .github/scripts/ci-leg.sh audit - uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7 if: failure() with: name: ci-log-audit path: target/ci-logs/audit.log
@row_add n=118 row=absence/signed-artifacts area=dependency-security rec=fct-eaz6kbxo@1
capability: Signed release artifacts produced by the fork
@row_check n=118 row=absence/signed-artifacts sha=67b773f4073f v=holds judge=sonnet head=edd0cfaea7f0
searched: Signed Builds · in .github/workflows/*.yml (only publish-browser.yml:41, behind if: github.repository == 'DioxusLabs/blitz' at line 37)
searched: signing · in .github/workflows/ci.yml (no signing step)
@row_add n=119 row=decision/signing-material-removed-after-build area=dependency-security rec=fct-eaz6kbxo@1,fct-gjoxy46h@1
name: Signing material removed after the build
rule: A signing key or keystore written from secrets for a build is removed in an always-run step.
@row_check n=119 row=decision/signing-material-removed-after-build sha=cc94ad3415fc v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:108
quote: run: echo "${{ secrets.APPLE_API_KEY_CONTENT }}" > ${{ vars.APPLE_API_KEY_PATH }}
at: .github/workflows/publish-browser.yml:180-181
quote: if: ${{ always() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/ci-test')) && startsWith(matrix.platform.os, 'macos') }} run: rm ${{ vars.APPLE_API_KEY_PATH }}
at: .github/workflows/publish-browser.yml:114
quote: base64 -d <<< "${{ secrets.ANDROID_KEYSTORE_BASE64 }}" > keystore.jks
at: .github/workflows/publish-browser.yml:184-185
quote: if: ${{ always() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/ci-test')) && matrix.platform.target == 'aarch64-linux-android' }} run: rm ./apps/browser/keystore.jks
@row_add n=120 row=decision/telemetry-sink-target-allowlist area=bootstrap-phases-derive-for-route-setup-project rec=fct-eh3265j5@1
name: Telemetry sink target allowlist
rule: escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level.
except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed
@row_check n=120 row=decision/telemetry-sink-target-allowlist sha=59161ddc2e59 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:100
quote: } else if is_engine_target(target) {
at: packages/escher-telemetry/src/format.rs:106
quote: } else if CONTENT_FIELDS.contains(&field) {
at: packages/escher-telemetry/src/format.rs:146
quote: if is_outside_target(metadata.target()) {
@row_add n=121 row=decision/ci-secrets-in-github-actions area=secret-management rec=fct-gjoxy46h@1
name: CI secrets in GitHub Actions
rule: Production secrets live in GitHub Actions secrets and vars and are passed only to the steps that use them.
@row_check n=121 row=decision/ci-secrets-in-github-actions sha=616b5e762472 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:168
quote: APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
at: .github/workflows/wpt.yml:111
quote: token: ${{ secrets.WPT_GITHUB_TOKEN }}
@row_add n=122 row=decision/live-evidence-holds-no-credentials area=secret-management rec=fct-jtwyhkf2@1,fct-ipi52teh@1
name: Live evidence holds no credentials
rule: No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
@row_check n=122 row=decision/live-evidence-holds-no-credentials sha=5064abd24553 v=holds judge=sonnet head=edd0cfaea7f0
searched: ANTHROPIC_|sk-ant|Authorization: Bearer|\.credentials\.json · in tracked source outside the pipeline records (repository root, excluding .claude, .andromeda, escher-0.x.0, target)
searched: apiKeySource · in tracked source (only scripts/cold-agent.sh:155 and .github/scripts/test_cold_agent.py check that it equals "none"; no key)
@row_add n=123 row=contract/environment-variables/rust-log area=secret-management rec=fct-kpqtgbic@1
boundary: Environment variables
item: RUST_LOG
promise: Holds the tracing filter directives escher-telemetry reads, defaulting to warn when unset or unparsable.
@row_check n=123 row=contract/environment-variables/rust-log sha=628d09984013 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:144
quote: let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
@row_add n=124 row=decision/driver-reads-only-rust-log area=secret-management rec=fct-pz4e3fuz@1
name: Driver reads only RUST_LOG
rule: The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.
@row_check n=124 row=decision/driver-reads-only-rust-log sha=c75c511dabcb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:144
quote: let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
searched: std::env::var|var_os|env::vars|getenv · in packages/escher-driver/src and examples/seven_guis/src (none; session_host.rs:37 reads only args_os; env! uses are build-time)
@row_add n=125 row=contract/rdme-exit-codes/1 area=error-handling rec=fct-itl5ofxf@1
boundary: rdme exit codes
item: 1
promise: The argument is neither URL nor file, or no README.md is found; a message is written to stderr.
@row_check n=125 row=contract/rdme-exit-codes/1 sha=76c8ce2797a6 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/readme/src/main.rs:181-182
quote: eprintln!("Cannot parse {raw_url} as url or find it as a file"); std::process::exit(1);
at: apps/readme/src/main.rs:233-234
quote: eprintln!("Could not find README.md file in the current directory"); std::process::exit(1);
@row_add n=126 row=contract/cssom-script-api/error-names area=error-handling rec=fct-ol5tumaa@1
boundary: CSSOM script API
item: CSSOM errors
promise: CSSOM errors reach script as DOMException names.
@row_check n=126 row=contract/cssom-script-api/error-names sha=337316e1d9ff v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/dom/stylesheet.rs:92-93
quote: fn cssom_error(err: CssomError, context: &mut Context, what: &str) -> boa_engine::JsError { dom_exception(context, err.dom_exception_name(), what)
at: packages/blitz-dom/src/cssom.rs:61
quote: Self::NotFound => "NotFoundError",
@row_add n=127 row=contract/cssom-script-api/selectortext-assignment area=error-handling rec=fct-ol5tumaa@1
boundary: CSSOM script API
item: selectorText assignment
promise: Assigning selectorText never throws.
@row_check n=127 row=contract/cssom-script-api/selectortext-assignment sha=ff5f68f15996 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/dom/stylesheet.rs:255-258
quote: let _ = ctx .doc .borrow_mut() .stylesheet_rule_set_selector_text(node_id, &path, &selectors);
@row_add n=128 row=contract/cssom-script-api/csstext-assignment area=error-handling rec=fct-ol5tumaa@1
boundary: CSSOM script API
item: cssText assignment
promise: Assigning cssText never throws.
@row_check n=128 row=contract/cssom-script-api/csstext-assignment sha=11042c746115 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/dom/stylesheet.rs:269-272
quote: let _ = ctx .doc .borrow_mut() .stylesheet_rule_style_set_css_text(node_id, &path, &css);
@row_add n=129 row=contract/js-module-loading/resolve-or-fetch-failure area=error-handling rec=fct-ylgvfhkt@1
boundary: JS module loading
item: resolve or fetch failure
promise: A module resolve or fetch failure reaches script as a JS TypeError.
@row_check n=129 row=contract/js-module-loading/resolve-or-fetch-failure sha=08e39bbd51c4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/modules.rs:70-71
quote: JsNativeError::typ() .with_message(format!("could not resolve module specifier {specifier:?}")),
at: packages/blitz-vibey-script/src/runtime/modules.rs:84
quote: JsNativeError::typ().with_message(format!("failed to fetch module {url}: {error}")),
@row_add n=130 row=contract/mounted-element-operations/missing-node area=error-handling rec=fct-p5sm3bek@1
boundary: Dioxus mounted-element operations
item: missing node
promise: An operation on a node that does not exist returns MountedError::OperationFailed wrapping NodeNotExistErr.
@row_check n=130 row=contract/mounted-element-operations/missing-node sha=7b41f6859b69 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/events.rs:181
quote: let err = MountedError::OperationFailed(Box::new(NodeNotExistErr(node_id)));
@row_add n=131 row=decision/ci-matrix-runs-every-job area=error-handling rec=fct-6x32ysne@1
name: CI matrix runs every job
rule: CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.
@row_check n=131 row=decision/ci-matrix-runs-every-job sha=d41ac0e48f8f v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:340
quote: fail-fast: false
at: .github/workflows/publish-browser.yml:44
quote: fail-fast: false
@row_add n=132 row=decision/refusal-closed-cause-set area=error-handling rec=fct-ugqfmfy4@1
name: Command refusal as a closed set of causes
rule: An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field.
why: So a refusal cannot hold anything a call supplied.
except: A Fault is carried only beside the malformed cause.
@row_check n=132 row=decision/refusal-closed-cause-set sha=59a70afe5391 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/refusal.rs:36
quote: pub const CAUSES: [Cause; 8] = [
at: packages/escher-driver/src/refusal.rs:162-165
quote: pub struct Refusal { cause: Cause, fault: Option<Fault>, }
at: packages/escher-driver/src/refusal.rs:176
quote: cause: Cause::Malformed,
@row_add n=133 row=decision/session-error-fixed-table area=error-handling rec=fct-ugqfmfy4@1
name: Session errors from a fixed table
rule: An escher-driver session error is written from a fixed table, holding nothing a call supplied.
@row_check n=133 row=decision/session-error-fixed-table sha=ab000b9bf0c1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/error.rs:80
quote: pub(crate) const SESSION_ERRORS: [ErrorSpec; 15] = [
at: packages/escher-driver/src/error.rs:215
quote: let meaning = self.spec().meaning;
@row_add n=134 row=decision/mcp-errors-fixed-no-data area=error-handling rec=fct-ugqfmfy4@1
name: MCP protocol errors fixed, no data
rule: MCP protocol errors are fixed codes and fixed messages and carry no data member.
@row_check n=134 row=decision/mcp-errors-fixed-no-data sha=2cef148e85dc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:192
quote: object.json("code", &error.code().to_string());
at: packages/escher-driver/src/mcp/framing.rs:193
quote: object.text("message", error.message());
at: packages/escher-driver/src/mcp/framing.rs:122
quote: ProtocolError::Parse => "parse error",
@row_add n=135 row=decision/refusal-logged-by-cause-name area=error-handling rec=fct-ugqfmfy4@1
name: Refusal logged by cause name only
rule: Of an escher-driver command refusal, only its cause's fixed name reaches a log.
@row_check n=135 row=decision/refusal-logged-by-cause-name sha=b0c3ed4331d7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
@row_add n=136 row=decision/stale-node-ids-never-panic area=error-handling rec=fct-kjyg4maj@1,fct-mgcd635n@1,fct-rng3zi7h@1
name: Stale node ids never panic
rule: A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve.
guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
@row_check n=136 row=decision/stale-node-ids-never-panic sha=a5c7fc3b0ac4 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:309
quote: if !doc.nodes.contains_key(child_id) {
at: packages/blitz-dom/src/resolve.rs:260
quote: if doc.nodes.get(node_id).is_none() {
at: packages/blitz-dom/src/tree.rs:90-91
quote: pub fn get(&self, id: NodeId) -> Option<&Node> { self.map.get(to_key(id))
at: packages/blitz-dom/src/stylo.rs:100-103
quote: let in_document = self .nodes .get(node_id) .is_some_and(|node| node.flags.is_in_document());
guard_at: tests/blitz-tests/tests/animations.rs:9
@row_add n=137 row=contract/query-apis/missing-element area=error-handling rec=fct-jfol2sty@1
boundary: Query APIs
item: missing element
promise: Query APIs return Result<Option<NodeId>>, and a missing element is None.
@row_check n=137 row=contract/query-apis/missing-element sha=e1241d0026d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/query_selector.rs:62
quote: pub fn query_selector(&self, selector: &str) -> Result<Option<NodeId>, ParseError> {
at: packages/blitz-dom/src/query_selector.rs:60
quote: ///   - Ok(None) if nothing matches
@row_add n=138 row=contract/fragment-navigation/unknown-target area=error-handling rec=fct-jfol2sty@1
boundary: Fragment navigation
item: unknown fragment target
promise: An unknown fragment target returns None or false and leaves scroll at 0.
@row_check n=138 row=contract/fragment-navigation/unknown-target sha=be71a08a059a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2865
quote: pub fn get_fragment_target(&self, fragment: &str) -> Option<NodeId> {
at: packages/blitz-dom/src/scrolling.rs:869
quote: None => false,
@row_add n=139 row=contract/wpt-results/crash area=error-handling rec=fct-ip6hsnxe@1
boundary: WPT runner results
item: CRASH
promise: A test that panics is caught and reported as a CRASH result.
@row_check n=139 row=contract/wpt-results/crash sha=2690b1de3bc8 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:629
quote: let result = catch_unwind(AssertUnwindSafe(|| {
at: wpt/runner/src/main.rs:641
quote: TestStatus::Crash,
@row_add n=140 row=contract/wpt-results/fail-uncaught-js-error area=error-handling rec=fct-ip6hsnxe@1
boundary: WPT runner results
item: FAIL with Uncaught JS error
promise: A harness test that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest.
@row_check n=140 row=contract/wpt-results/fail-uncaught-js-error sha=4cb34edad6cb v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:165-168
quote: None if !js_errors.is_empty() => {             let subtest_results = vec![SubtestResult {                 name: "Uncaught JS error".to_string(),                 status: TestStatus::Fail,
@row_add n=141 row=contract/wpt-results/timeout area=error-handling rec=fct-ip6hsnxe@1
boundary: WPT runner results
item: TIMEOUT
promise: A harness test that never reports and had no uncaught JS errors yields TIMEOUT.
@row_check n=141 row=contract/wpt-results/timeout sha=760f27fe46d6 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:178-180
quote: None => {             warn!("{relative_path}: testharness.js did not report results");             (TestStatus::Timeout, SubtestCounts::ZERO_OF_ONE, Vec::new())
@row_add n=142 row=absence/error-reporting-integration area=error-handling rec=fct-ueaekj7s@1
capability: An error reporting integration
@row_check n=142 row=absence/error-reporting-integration sha=d17f2ecbe841 v=holds judge=sonnet head=edd0cfaea7f0
searched: sentry|opentelemetry · in packages, examples, tests (*.rs, Cargo.toml) (only a doc comment naming OpenTelemetry resource keys in packages/escher-telemetry/src/lib.rs:46)
@row_add n=143 row=contract/network-tracing-log/url area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: url
promise: A network request log records the request URL as field url.
@row_check n=143 row=contract/network-tracing-log/url sha=2c6eaad0a98e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:221
quote: url = final_url.as_str(),
@row_add n=144 row=contract/network-tracing-log/status area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: status
promise: A network request log records the response status as field status.
@row_check n=144 row=contract/network-tracing-log/status sha=275843a6e6cd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:222
quote: status = status.as_u16(),
@row_add n=145 row=contract/network-tracing-log/error area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: error
promise: A failed network request log records the failure as field error.
@row_check n=145 row=contract/network-tracing-log/error sha=1db644d34d9c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:312
quote: tracing::error!(url = url.as_str(), error = ?e, "Error fetching");
@row_add n=146 row=contract/network-tracing-log/path area=logging-monitoring rec=fct-dk5r4tgw@1
boundary: Network tracing log
item: path
promise: The cache directory path is logged at info level as field path.
@row_check n=146 row=contract/network-tracing-log/path sha=79b43aec9342 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:58
quote: tracing::info!(path = ?path.display(), "Using cache dir");
@row_add n=147 row=contract/resource-load-failure-log/url area=logging-monitoring rec=fct-6le5eps5@1
boundary: Resource load failure log
item: url
promise: A resource load failure log carries structured field url.
@row_check n=147 row=contract/resource-load-failure-log/url sha=c2b602322e54 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1831
quote: url = url.as_str(),
@row_add n=148 row=contract/resource-load-failure-log/waiting-nodes area=logging-monitoring rec=fct-6le5eps5@1
boundary: Resource load failure log
item: waiting_nodes
promise: A resource load failure log carries structured field waiting_nodes.
@row_check n=148 row=contract/resource-load-failure-log/waiting-nodes sha=b79d282cbe61 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1832
quote: waiting_nodes = waiting_nodes.len(),
@row_add n=149 row=contract/resource-load-failure-log/error area=logging-monitoring rec=fct-6le5eps5@1
boundary: Resource load failure log
item: error
promise: A resource load failure log carries structured field error.
@row_check n=149 row=contract/resource-load-failure-log/error sha=8308d76b5009 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1833
quote: error = err.as_str(),
@row_add n=150 row=decision/js-console-to-debug-log area=logging-monitoring rec=fct-5qjgusc6@1
name: JS console routed to the debug log
rule: Script console output (log, info, warn, error) goes to the log crate at debug level under target js_console, never to stdout or stderr.
why: Keeps stdout and stderr clean.
@row_check n=150 row=decision/js-console-to-debug-log sha=10ce19315c3c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime.rs:36
quote: log::debug!(target: "js_console", "{msg}");
@row_add n=151 row=decision/release-publish-upstream-only area=logging-monitoring rec=fct-2nfct2cx@1
name: Release publishing only from upstream
rule: The release-cli publish job is repository-guarded and runs only in the upstream DioxusLabs/blitz repository.
@row_check n=151 row=decision/release-publish-upstream-only sha=8cf5389fb44a v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:37
quote: if: github.repository == 'DioxusLabs/blitz'
@row_add n=152 row=decision/tracing-for-engine-logging area=logging-monitoring rec=fct-u3c7wcoc@1
name: tracing for engine logging
rule: Engine crates log through the tracing crate behind an optional tracing feature, and their logging macros compile only with that feature on.
@row_check n=152 row=decision/tracing-for-engine-logging sha=4b0cb348cd90 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:24
quote: tracing = ["dep:tracing"]
at: packages/blitz-dom/src/util.rs:26-27
quote: #[cfg(feature = "tracing")] tracing::warn!("Failed to decompress woff1 font");
@row_add n=153 row=decision/tracing-feature-propagates area=logging-monitoring rec=fct-wkixw6vk@1
name: One tracing feature across the stack
rule: Turning on dioxus-native's tracing feature also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net.
@row_check n=153 row=decision/tracing-feature-propagates sha=d8e6b19bd556 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/Cargo.toml:69
quote: tracing = ["dep:tracing", "dioxus-native-dom/tracing", "blitz-shell/tracing", "blitz-dom/tracing", "blitz-html/tracing", "blitz-net/tracing"]
@row_add n=154 row=decision/env-logger-for-wpt-runner area=logging-monitoring rec=fct-gfs2klpg@1
name: env_logger for the WPT runner
rule: The WPT runner logs through env_logger.
@row_check n=154 row=decision/env-logger-for-wpt-runner sha=9c4191efc2ca v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:467
quote: env_logger::init();
@row_add n=155 row=decision/escher-telemetry-allowlist-sink area=logging-monitoring rec=fct-ggtnnptr@1
name: escher's allowlist telemetry sink
rule: Both seven_guis binaries log through escher's own stderr-only allowlist subscriber.
except: The scrubbing reaches that sink only: upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.
@row_check n=155 row=decision/escher-telemetry-allowlist-sink sha=dea8dafa541c v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/main.rs:6
quote: if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
at: examples/seven_guis/src/session_host.rs:32
quote: if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
@row_add n=156 row=decision/engine-target-field-allowlist area=logging-monitoring rec=fct-ggtnnptr@1
name: Engine log targets keep only allowlisted fields
rule: In escher's telemetry sink an engine target keeps only seven allowlisted fields, and every other field is written as [redacted].
except: An allowlisted log.file field can still carry a host path.
@row_check n=156 row=decision/engine-target-field-allowlist sha=61066a3a2b03 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:38-46
quote: pub const SAFE_FIELDS: &[&str] = &[ "node_id", "status", "waiting_nodes", "property", "log.module_path", "log.file", "log.line", ];
@row_add n=157 row=decision/escher-target-content-fields-withheld area=logging-monitoring rec=fct-ggtnnptr@1
name: escher_ targets withhold content-named fields
rule: In escher's telemetry sink an escher_ target keeps all fields but eleven content-named ones.
@row_check n=157 row=decision/escher-target-content-fields-withheld sha=4d087451a1ed v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:49
quote: pub const CONTENT_FIELDS: &[&str] = &[
at: packages/escher-telemetry/src/format.rs:52-61
quote: "src", "html", "text", "value", "attrs", "path", "request", "error", "panic.payload", ];
@row_add n=158 row=decision/other-log-targets-dropped area=logging-monitoring rec=fct-ggtnnptr@1
name: Other log targets dropped whole
rule: In escher's telemetry sink any target other than an engine or escher_ target is dropped whole at every level, and spans follow the same rule.
@row_check n=158 row=decision/other-log-targets-dropped sha=0d000a1146d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:146-147
quote: if is_outside_target(metadata.target()) { return Ok(());
@row_add n=159 row=contract/escher-session-stdout/command-line area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stdout
item: command result line
promise: escher-session writes one uncoloured JSON line per command on stdout.
@row_check n=159 row=contract/escher-session-stdout/command-line sha=1641f355fad0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:410-413
quote: let _ = stdout .write_all(line.as_bytes()) .and_then(|()| stdout.write_all(b"\n")) .and_then(|()| stdout.flush());
@row_add n=160 row=contract/escher-session-stdout/mcp-role area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stdout
item: mcp role
promise: The mcp role writes JSON-RPC lines only on stdout.
@row_check n=160 row=contract/escher-session-stdout/mcp-role sha=3498dfc6884c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:444-446
quote: Line::Mcp { record: None } => Ending::Spoken(serve_mcp( io::stdin().lock(), io::stdout().lock(),
@row_add n=161 row=contract/escher-session-stderr/usage-messages area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stderr
item: usage messages
promise: escher-session writes usage messages on stderr.
@row_check n=161 row=contract/escher-session-stderr/usage-messages sha=4d258c190e10 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:352
quote: stderr: Some(usage(apps)),
@row_add n=162 row=contract/escher-session-stderr/error-messages area=logging-monitoring rec=fct-eabjzys7@1
boundary: escher-session stderr
item: error messages
promise: escher-session writes error messages on stderr.
@row_check n=162 row=contract/escher-session-stderr/error-messages sha=cd102c07d75e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:416
quote: let _ = writeln!(io::stderr().lock(), "{line}");
@row_add n=163 row=decision/session-output-holds-no-caller-input area=logging-monitoring rec=fct-eabjzys7@1
name: Session output holds no caller input
rule: No escher-session output line holds anything the caller typed.
except: The mcp role's echoed request id.
@row_check n=163 row=decision/session-output-holds-no-caller-input sha=0b25d2dc0f4d v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: println!|eprintln!|write_all|writeln! · in packages/escher-driver/src
why: no evidence found
@row_add n=164 row=decision/agent-run-events-metadata-only area=logging-monitoring rec=fct-lm4cjfou@1
name: Agent-run events hold harness metadata only
rule: The agent-run test contract's JSON-line events carry no content-named field, only harness metadata, and never captured test output or panic text.
@row_check n=164 row=decision/agent-run-events-metadata-only sha=537613b3dca1 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:136-138
quote: emit({"event": "run.end", "ts": now(), "selection": selection, "passed": counts["ok"], "failed": counts["failed"], "ignored": counts["ignored"], "cargo_exit": cargo_exit, "outcome": "passed" if passed else "failed"})
@row_add n=165 row=decision/agent-run-raw-log-unprinted area=logging-monitoring rec=fct-lm4cjfou@1
name: Agent-run raw log kept unprinted
rule: The agent-run raw cargo and libtest output log is unscrubbed, never printed by logs, and gitignored.
@row_check n=165 row=decision/agent-run-raw-log-unprinted sha=cab3f7f0b633 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:226
quote: cat "$STATE_DIR/events.jsonl"
at: .gitignore:1
quote: /target
@row_add n=166 row=decision/cold-agent-events-metadata-only area=logging-monitoring rec=fct-3vxtdih5@1
name: Cold-agent events hold run metadata only
rule: The cold-agent pipe's events and verdict carry counts, booleans and closed words, never transcript text, a tool argument or answer, a reply or the check.
@row_check n=166 row=decision/cold-agent-events-metadata-only sha=459024818d5a v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:511
quote: emit({"event": "run.start", "ts": now(), "task": args[0]})
@row_add n=167 row=decision/cold-agent-record-unprinted area=logging-monitoring rec=fct-3vxtdih5@1
name: Cold-agent record kept unprinted
rule: The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
@row_check n=167 row=decision/cold-agent-record-unprinted sha=623f54afa953 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: masked|host path · in .github/scripts
searched: server\.log|SERVER_LOG|client\.log|print\( · in scripts/cold-agent.sh
why: no evidence found
@row_add n=168 row=decision/session-socket-sole-listener area=api-security rec=fct-svo6bgmr@3
name: Session socket as the one listener
rule: The driver session's Unix-domain socket is the project's one listener, serving the session lifecycle and driver calls and their answers.
@row_check n=168 row=decision/session-socket-sole-listener sha=8ed9b38b6595 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
@row_add n=169 row=decision/mcp-server-over-stdio area=api-security rec=fct-svo6bgmr@3,fct-ghd6zkhy@1
name: MCP server over process stdio
rule: The MCP server role serves the driver's verbs as tools over its own stdin and stdout only, binding no listener, port or socket, and answers only the process that spawned it, with no handshake, token, secret or peer check.
why: The crossing was ratified as is.
@row_check n=169 row=decision/mcp-server-over-stdio sha=0f3b65026712 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:445-446
quote: io::stdin().lock(), io::stdout().lock(),
@row_add n=170 row=contract/mcp-server/request-line area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: request line
promise: A request line is bounded at 65,536 bytes.
@row_check n=170 row=contract/mcp-server/request-line sha=83a460560673 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:11
quote: pub(super) const MAX_MCP_REQUEST_BYTES: usize = 65_536;
@row_add n=171 row=contract/mcp-server/tool-result area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: tool result
promise: A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.
@row_check n=171 row=contract/mcp-server/tool-result sha=db14f78c584a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
at: packages/escher-driver/src/mcp/result.rs:75-79
quote: let size = match self.image { Some(_) => self.written().len(), None => self.text.len(), }; if size > MAX_ANSWER_BYTES {
@row_add n=172 row=contract/mcp-server/idle-expiry area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: idle expiry
promise: The idle expiry is checked when a request arrives.
@row_check n=172 row=contract/mcp-server/idle-expiry sha=7e46d192d22a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:101-103
quote: Request::Line(line) => { server.expire(); server.handle(&line, &mut output)
@row_add n=173 row=contract/mcp-server/help area=api-security rec=fct-svo6bgmr@3
boundary: MCP server
item: help
promise: The help tool is answered by the server itself from the schema's fixed tables, running nothing on a session.
@row_check n=173 row=contract/mcp-server/help sha=7fd64dd7f8a6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:238-239
quote: Level::Tool => match validate_session(call) { Ok(_) => ToolResult::accepted(describe()),
@row_add n=174 row=contract/session-wire/recording-failed area=api-security rec=fct-svo6bgmr@3
boundary: Driver session wire
item: ok v3 recording-failed
promise: Wire v3 replies include the fixed reply ok v3 recording-failed.
@row_check n=174 row=contract/session-wire/recording-failed sha=60121a97f05e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:246
quote: Reply::RecordingFailed => format!("ok {VERSION} recording-failed\n"),
@row_add n=175 row=decision/outbound-per-host-limit area=api-security rec=fct-ktpvvsv7@1
name: Outbound per-host concurrency limit
rule: Concurrent outbound requests are limited to 6 per host.
@row_check n=175 row=decision/outbound-per-host-limit sha=8ee84571ded5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:24
quote: const PER_HOST_MAX_CONCURRENT: usize = 6;
@row_add n=176 row=absence/cors-same-origin-policy area=api-security rec=fct-j3wtsshr@1
capability: A CORS or same-origin policy
@row_check n=176 row=absence/cors-same-origin-policy sha=483c6c59ca85 v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)cors|content-security|same.origin|same_origin|cross.origin · in packages/**/*.rs
@row_add n=177 row=absence/response-body-size-limit area=api-security rec=fct-7maio4kj@1
capability: A response body size limit on outbound requests
@row_check n=177 row=absence/response-body-size-limit sha=9c970e4fac17 v=holds judge=sonnet head=edd0cfaea7f0
searched: body_limit|content_length|max_body|take\(|limit · in packages/blitz-net/src
@row_add n=178 row=decision/settle-never-waits-on-loads area=api-security rec=fct-brzrm7cu@3
name: Settle never waits on a load
rule: A harness settle reports a load in flight at once as not settled with the class Loads, waits on no clock and moves none, and its pass loop is bounded at 64 passes.
why: The absent request timeout cannot hang a settle.
@row_check n=178 row=decision/settle-never-waits-on-loads sha=94c6cd5aa629 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:176
quote: return Err(NotSettled { busy: Busy::Loads });
at: packages/blitz-test-harness/src/settle.rs:15
quote: pub const SETTLE_PASS_LIMIT: u32 = 64;
@row_add n=179 row=decision/load-wait-bound area=api-security rec=fct-brzrm7cu@3
name: Acting commands bounded by LOAD_WAIT
rule: An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class.
why: The absent request timeout cannot hang a command.
@row_check n=179 row=decision/load-wait-bound sha=b201ea4d378f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:19
quote: pub const LOAD_WAIT: Duration = Duration::from_secs(5);
@row_add n=180 row=decision/time-moves-only-on-advance area=api-security rec=fct-brzrm7cu@3
name: Time moves only on advance
rule: Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
@row_check n=180 row=decision/time-moves-only-on-advance sha=0799666faf34 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:15
quote: pub const MAX_MILLISECONDS: u32 = 60_000;
at: packages/escher-driver/src/schema/spec.rs:45
quote: ArgKind::Milliseconds => "a whole number from 1 to 60000",
@row_add n=181 row=decision/scrollbar-fade-on-document-clock area=api-security rec=fct-brzrm7cu@3
name: Scrollbar fade reads the document clock
rule: The overlay scrollbar fade reads the document's clock, so on a harness-built document two shots of one state read the same pixels until an advance moves the fade; a document built with no clock still fades by wall time.
why: Ratified.
@row_check n=181 row=decision/scrollbar-fade-on-document-clock sha=dccff8e46e04 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:523
quote: clock: config.clock.unwrap_or_else(|| Arc::new(WallClock::new())),
@row_add n=182 row=decision/failed-load-ends-wait area=api-security rec=fct-brzrm7cu@3
name: A failed load ends the wait
rule: A failed load ends a settle's wait, so a failed stylesheet never leaves a document not settled on Loads for good.
@row_check n=182 row=decision/failed-load-ends-wait sha=89187c7ae41e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:121-125
quote: fn error(mut self: Box<Self>, url: String, error: String) { if let Some(inner) = self.inner.take() { inner.error(url, error); } }
@row_add n=183 row=contract/driver-answer/settled area=api-security rec=fct-brzrm7cu@3
boundary: Driver command answer
item: settled
promise: A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the outcome names a class only, never a URL, request, node, id, name or value.
@row_check n=183 row=contract/driver-answer/settled sha=a6cd063bac90 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:372-374
quote: span.record("settled", settled); if let Some(busy) = busy { span.record("busy", busy_word(busy));
@row_add n=184 row=contract/driver-command-line/answer-bounds area=api-security rec=fct-brzrm7cu@3
boundary: Driver command line
item: answer bounds
promise: A started host has 30 s to answer and each read of a call's answer is bounded at 30 s; these bound a transport and wait on no load.
@row_check n=184 row=contract/driver-command-line/answer-bounds sha=ef00dc605916 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:49
quote: const START_BOUND: Duration = Duration::from_secs(30);
at: packages/escher-driver/src/wire.rs:18
quote: pub(crate) const ANSWER_BOUND: Duration = Duration::from_secs(30);
@row_add n=185 row=decision/js-fetch-get-head-only area=api-security rec=fct-4xp4lixm@1
name: JS-facing fetch is GET and HEAD only
rule: Script-facing fetch allows only GET and HEAD, over the embedder's ScriptFetcher.
@row_check n=185 row=decision/js-fetch-get-head-only sha=1145ac009a08 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:82
quote: if (method !== "GET" && method !== "HEAD") {
@row_add n=186 row=decision/outbound-http-through-blitz-net area=api-security rec=fct-yego5hpv@1
name: Outbound HTTP through blitz-net
rule: The apps' outbound HTTP goes through blitz-net with http2, and in rdme through reqwest.
@row_check n=186 row=decision/outbound-http-through-blitz-net sha=d96a9cbc1de7 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:57
quote: blitz-net = { workspace = true, features = ["http2"] }
at: apps/readme/Cargo.toml:57
quote: reqwest = { workspace = true }
@row_add n=187 row=decision/ci-github-api-through-gh area=api-security rec=fct-a7z4h4hw@1
name: CI GitHub API calls through gh
rule: CI calls the GitHub API through the gh CLI, with GH_TOKEN taken from the workflow token secret.
@row_check n=187 row=decision/ci-github-api-through-gh sha=64952502a440 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/wpt-post-results.yml:32
quote: GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
@row_add n=188 row=contract/session-socket/connections area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: connections
promise: One connection is served at a time, with a 2 s read and write bound.
@row_check n=188 row=contract/session-socket/connections sha=ce6dae89542a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:146
quote: let mut stream = match listener.accept() {
at: packages/escher-driver/src/host.rs:180-181
quote: stream.set_read_timeout(Some(wire::IO_BOUND))?; stream.set_write_timeout(Some(wire::IO_BOUND))?;
at: packages/escher-driver/src/wire.rs:15
quote: pub(crate) const IO_BOUND: Duration = Duration::from_secs(2);
@row_add n=189 row=contract/session-socket/request-line area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: request line
promise: A request line is at most 16,384 bytes.
@row_check n=189 row=contract/session-socket/request-line sha=bed717e9cbfb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:23
quote: pub(crate) const MAX_REQUEST_BYTES: usize = 16_384;
at: packages/escher-driver/src/host.rs:182
quote: let parsed = match wire::read_line(&*stream, wire::MAX_REQUEST_BYTES)? {
@row_add n=190 row=contract/session-socket/lifecycle-reply area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: lifecycle reply
promise: A lifecycle reply is at most 128 bytes on its JSON line.
@row_check n=190 row=contract/session-socket/lifecycle-reply sha=9102abbf97f3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:27
quote: pub(crate) const MAX_REPLY_BYTES: usize = 128;
at: packages/escher-driver/src/client.rs:145
quote: const LIFECYCLE: ReplyBounds = (wire::IO_BOUND, wire::MAX_REPLY_BYTES);
@row_add n=191 row=contract/session-socket/oversize area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: oversize
promise: An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
@row_check n=191 row=contract/session-socket/oversize sha=2c2c56ce5424 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:221-222
quote: if json.len() > MAX_ANSWER_BYTES { Reply::Oversize
at: packages/escher-driver/src/client.rs:105
quote: Reply::Oversize => Err(SessionError::AnswerTooLarge),
@row_add n=192 row=contract/session-socket/stop area=api-security rec=fct-t6pddbhw@1
boundary: Driver session socket
item: stop
promise: stop removes the socket and the state directory and writes no record.
@row_check n=192 row=contract/session-socket/stop sha=54bbb9a4e248 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:81-82
quote: let removed = fs::remove_file(&socket).map_err(io_error); let _ = fs::remove_dir(state_dir);
@row_add n=193 row=decision/host-idle-expiry area=api-security rec=fct-t6pddbhw@1
name: Host idle expiry
rule: A host that has answered no request for IDLE_EXPIRY, 1,800 s, ends exactly as on stop; every answered request, refused or malformed included, restarts the count, and a connection broken before its request is whole does not.
why: The answer to the orphaned-host question: a host whose client died without stop kept running.
@row_check n=193 row=decision/host-idle-expiry sha=f5a0268c2e73 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:11
quote: pub const IDLE_EXPIRY: Duration = Duration::from_secs(1800);
@row_add n=194 row=decision/graph-answer-unpaged area=api-security rec=fct-s6ul72yw@1
name: The graph answer is unpaged
rule: The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound.
why: A part argument would widen the crossing.
guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
@row_check n=194 row=decision/graph-answer-unpaged sha=1cd201bd8d8d v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
guard_at: tests/blitz-tests/tests/stand_graph.rs:382
@row_add n=195 row=decision/actions-answer-unpaged area=api-security rec=fct-hbfh5ibd@1
name: The actions answer is unpaged
rule: The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound.
guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
@row_check n=195 row=decision/actions-answer-unpaged sha=3cac76dd0516 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:281
quote: ("actions", []) => Ok(Command::Actions),
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
guard_at: tests/blitz-tests/tests/stand_actions.rs:547
@row_add n=196 row=decision/publish-secrets-upstream-main-only area=authentication-authorization rec=fct-n7yhg2x4@1
name: Publish secrets reachable only upstream
rule: The publish job runs only in DioxusLabs/blitz and uses the Signed Builds environment only on main or ci-test branches, so no fork ref reaches the environment or its secrets.
@row_check n=196 row=decision/publish-secrets-upstream-main-only sha=fc001852d71d v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:37
quote: if: github.repository == 'DioxusLabs/blitz'
at: .github/workflows/publish-browser.yml:41
quote: environment: ${{ (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/ci-test')) && 'Signed Builds' || '' }}
@row_add n=197 row=decision/kept-check-owner-only area=data-protection rec=fct-cl72c3yo@4,fct-6n6morf4@1
name: Kept checks are owner-only
rule: A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends.
why: Ratified.
except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
guard: recording::the_committed_checks_are_what_the_binary_records
@row_check n=197 row=decision/kept-check-owner-only sha=983017452584 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
at: packages/escher-driver/src/recording.rs:186
quote: imp::remove(&self.file);
guard_at: examples/seven_guis/tests/recording.rs:207
@row_add n=198 row=decision/kept-check-answer-whole area=data-protection rec=fct-cl72c3yo@4
name: Kept check keeps each answer whole
rule: A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels.
why: Ratified, a shot's pixels excepted.
@row_check n=198 row=decision/kept-check-answer-whole sha=d4a45e39bfa2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:55-58
quote: pub(crate) fn kept_line(answer: &Result<Outcome, Refusal>) -> String {     match answer {         Ok(Outcome::Shot { width, height, .. }) => shot_without_image(*width, *height),         answer => written(answer),
@row_add n=199 row=decision/kept-check-call-content area=data-protection rec=fct-cl72c3yo@4
name: Kept check call-content class
rule: A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN.
why: Ratified.
@row_check n=199 row=decision/kept-check-call-content sha=6aeb12552b46 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:120-121
quote: ArgValue::Text(_) if masked && arg.kind == ArgKind::Text => {                 arguments.text(arg.name, MASKED_STAND_IN);
at: packages/escher-driver/src/recording.rs:123
quote: ArgValue::Text(text) => arguments.text(arg.name, text),
@row_add n=200 row=decision/kept-check-graph-answer area=data-protection rec=fct-cl72c3yo@4
name: Kept check holds the graph answer
rule: A kept graph answer holds ids, owner ids, component names, registered state names, handler event names and closed flag words and booleans, never a control value or typed text.
why: A boundary widening, ratified.
@row_check n=200 row=decision/kept-check-graph-answer sha=2b5d6501aa49 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:274-278
quote: fn graph_node(node: &GraphNode) -> String {     let mut object = Object::new();     object.text("id", &node.id);     if let Some(owner) = &node.owner {         object.text("owner", owner);
@row_add n=201 row=decision/kept-check-actions-answer area=data-protection rec=fct-cl72c3yo@4
name: Kept check holds the actions answer
rule: A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId.
why: A boundary widening, ratified.
guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
state: provisional
@row_check n=201 row=decision/kept-check-actions-answer sha=efece5a93793 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:253-259
quote: fn action_node(node: &ActionNode) -> String {     let mut object = Object::new();     object.text("id", &node.id);     if let Some(owner) = &node.owner {         object.text("owner", owner);     }     object.flag("keyed", node.keyed);
guard_at: tests/blitz-tests/tests/stand_replay.rs:268
@row_add n=202 row=contract/kept-check-file/entry area=data-protection rec=fct-cl72c3yo@4
boundary: Kept check file
item: entry
promise: A check is a JSON array: the app label, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
@row_check n=202 row=contract/kept-check-file/entry sha=7d90021d9d60 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:129
quote: record.text("tool", verb.name);
at: packages/escher-driver/src/recording.rs:130
quote: record.json("arguments", &arguments.end());
at: packages/escher-driver/src/recording.rs:134
quote: record.flag("accepted", answer.is_ok());
at: packages/escher-driver/src/recording.rs:135
quote: record.text("digest", &hex(answer_digest(answer)));
at: packages/escher-driver/src/recording.rs:136
quote: record.json("answer", &kept_line(answer));
at: packages/escher-driver/src/recording.rs:142
quote: format!("[\n{}\n]\n", records.join(",\n"))
@row_add n=203 row=decision/cold-agent-records-private area=data-protection rec=fct-cl72c3yo@4
name: Cold-agent recordings stay private
rule: The cold-agent pipe records into an owner-only directory, never prints, copies or commits the checks, reads them for bools and counts only and removes them with its state on cleanup.
@row_check n=203 row=decision/cold-agent-records-private sha=1705093e67d5 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:571
quote: mkdir -m 700 "$STATE_DIR/record"
at: scripts/cold-agent.sh:595
quote: rm -rf "$STATE_DIR"
@row_add n=204 row=contract/coverage-line/unexercised area=data-protection rec=fct-3s6sesu6@1
boundary: Coverage answer line
item: unexercised
promise: An array of id, owner and event per unexercised handler, owner left out when absent.
@row_check n=204 row=contract/coverage-line/unexercised sha=97d70b34a861 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:427-436
quote: &list(self.unexercised.iter().map(|handler| { let mut entry = Object::new(); entry.text("id", &handler.id); if let Some(owner) = &handler.owner { entry.text("owner", owner); } entry.text("event", &handler.event);
@row_add n=205 row=contract/coverage-line/checks area=data-protection rec=fct-3s6sesu6@1
boundary: Coverage answer line
item: checks
promise: The count of checks replayed.
@row_check n=205 row=contract/coverage-line/checks sha=1e2b3607d7e2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:437
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX));
at: packages/escher-driver/src/coverage.rs:99
quote: checks: checks.len(),
@row_add n=206 row=contract/coverage-line/held area=data-protection rec=fct-3s6sesu6@1
boundary: Coverage answer line
item: held
promise: The count of checks that held.
@row_check n=206 row=contract/coverage-line/held sha=2595fbff873f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:438
quote: object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
at: packages/escher-driver/src/coverage.rs:84-85
quote: if replay.holds() { held += 1;
@row_add n=207 row=decision/coverage-line-no-user-data area=data-protection rec=fct-3s6sesu6@1
name: Coverage line carries no user data
rule: The coverage answer line holds ids, owner ids, event names and counts only, never a control value, typed text, path or NodeId.
guard: stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line
@row_check n=207 row=decision/coverage-line-no-user-data sha=7a6b09616013 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:429
quote: entry.text("id", &handler.id);
at: packages/escher-driver/src/json.rs:433
quote: entry.text("event", &handler.event);
at: packages/escher-driver/src/json.rs:437
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX));
guard_at: tests/blitz-tests/tests/stand_handler_coverage.rs:381
@row_add n=208 row=decision/advisory-ignores-by-id area=dependency-security rec=fct-a4qh6ioi@2
name: Advisory ignores by ID only
rule: An advisory is ignored only by its ID, each with a written reason, with no blanket allow and unmaintained and unsound at their defaults.
@row_check n=208 row=decision/advisory-ignores-by-id sha=96fedf5dec67 v=holds judge=sonnet head=edd0cfaea7f0
at: deny.toml:17-21
quote: [advisories] # Per-ID ignores only, each a bounded deferral: the audit leg re-reads the advisory database on every # push, so a new advisory, or a fix landing upstream, surfaces at the next run. ignore = [
at: deny.toml:29
quote: { id = "RUSTSEC-2026-0372", reason = "zvariant_derive 5.15.0 is unmaintained, folded into zbus 6.0.0 (z-galaxy/zbus#2043); it reaches the graph only through zbus 5.19 + atspi 0.29 under accesskit_unix 0.24.0 (the latest, requ
@row_add n=209 row=decision/local-audit-before-push area=dependency-security rec=fct-6a266lkz@1
name: Run the audit leg locally before a push
rule: The local fast leg runs no audit, so the audit leg is run locally before a push.
why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
@row_check n=209 row=decision/local-audit-before-push sha=de130bad516c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:9
quote: FAST_LEGS=(fmt clippy test ci-scripts embolden)
at: .github/scripts/ci-leg.sh:40
quote: audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
@row_add n=210 row=decision/no-git-dependency area=dependency-security rec=fct-f5jtvf4a@1,fct-tnbysnlo@1,fct-7aitszps@1
name: No git dependency
rule: The workspace takes no git dependency.
@row_check n=210 row=decision/no-git-dependency sha=4e99d39dd62e v=holds judge=sonnet head=edd0cfaea7f0
searched: git\s*= · in Cargo.toml, packages/*/Cargo.toml, apps/*/Cargo.toml, tests/*/Cargo.toml, examples/*/Cargo.toml
searched: git\+ · in Cargo.lock
@row_add n=211 row=decision/engine-dependency-pins area=dependency-security rec=fct-tnbysnlo@1,fct-7aitszps@1
name: Engine dependency pins
rule: The pins stay: Stylo 0.22.0, Taffy the registry 0.15, Dioxus 0.7.3 and winit =0.31.0-beta.3.
@row_check n=211 row=decision/engine-dependency-pins sha=56b8c6606e63 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:65
quote: style = { version = "0.22.0", package = "stylo" }
at: Cargo.toml:105
quote: taffy = { version = "0.15", default-features = false, features = [
at: Cargo.toml:86
quote: dioxus = { version = "0.7.3" }
at: Cargo.toml:141
quote: winit = { version = "=0.31.0-beta.3" }
@row_add n=212 row=decision/serde-json-read-only area=dependency-security rec=fct-3t4xl422@1
name: serde_json, read only
rule: serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing.
why: Chosen as read-only use.
@row_check n=212 row=decision/serde-json-read-only sha=cb87d23da448 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:165
quote: serde_json = "1"
at: packages/escher-driver/Cargo.toml:17
quote: serde_json = { workspace = true }
at: packages/escher-driver/src/mcp/framing.rs:152
quote: let Ok(value) = serde_json::from_str::<Value>(text) else {
at: packages/escher-driver/src/recording.rs:337
quote: let Ok(Value::Array(records)) = serde_json::from_slice::<Value>(bytes) else {
at: packages/escher-driver/src/replay.rs:84
quote: let now: Option<Value> = serde_json::from_str(&kept_line(answer)).ok();
@row_add n=213 row=decision/png-encodes-screenshots area=dependency-security rec=fct-q4iqo3xv@1
name: png encodes a screenshot
rule: The png crate is what encodes a screenshot in escher-driver, and blitz-tests takes it only as a dev-dependency.
@row_check n=213 row=decision/png-encodes-screenshots sha=86656a928163 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/image.rs:8
quote: let mut encoder = png::Encoder::new(&mut out, width, height);
at: tests/blitz-tests/Cargo.toml:38
quote: png = { workspace = true }
@row_add n=214 row=decision/color-is-contrast-luminance-source area=dependency-security rec=fct-bf3tngmb@1
name: color is the contrast reader's luminance source
rule: The color crate, already in the graph through blitz-dom, is what supplies luminance to blitz-test-harness's contrast reader.
@row_check n=214 row=decision/color-is-contrast-luminance-source sha=65378ce379c8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/Cargo.toml:27
quote: color = { workspace = true }
@row_add n=215 row=decision/free-disk-only-on-stand-repeat area=dependency-security rec=fct-pq2wmfyf@1,fct-kjnf7nfl@1
name: Runner disk freed only for the repeated stand job
rule: Only the `Stand checks repeated` CI job frees runner disk, with the free-disk-space action pinned to a commit; the `matrix_test` job holds no such step.
guard: test_only_stand_repeat_frees_disk
@row_check n=215 row=decision/free-disk-only-on-stand-repeat sha=32e1d3bf4958 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:312
quote: uses: jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be # v1.3.1
at: .github/scripts/test_ci_workflows.py:114-115
quote: self.assertEqual(free_disk_steps(self.jobs["matrix_test"]), []) self.assertEqual(len(free_disk_steps(self.jobs["stand-repeat"])), 1)
guard_at: .github/scripts/test_ci_workflows.py:110
@row_add n=216 row=decision/embolden-locked-fast-leg area=dependency-security rec=fct-q5ecmhgy@1
name: font-embolden tested by a locked fast leg
rule: The blitz-paint font-embolden feature is tested by a locked cargo leg among the fast legs, with no CI job of its own.
@row_check n=216 row=decision/embolden-locked-fast-leg sha=38e497272258 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:29
quote: embolden) cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted ;;
at: .github/scripts/ci-leg.sh:9
quote: FAST_LEGS=(fmt clippy test ci-scripts embolden)
@row_add n=217 row=decision/closed-refusal-cause-set area=error-handling rec=fct-bzel2phc@3
name: Closed refusal cause set
rule: A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause.
why: The closed set of eight was ratified.
@row_check n=217 row=decision/closed-refusal-cause-set sha=05ac0db6caa2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/refusal.rs:36
quote: pub const CAUSES: [Cause; 8] = [
at: packages/escher-driver/src/refusal.rs:176
quote: cause: Cause::Malformed,
@row_add n=218 row=decision/refusal-holds-no-call-input area=error-handling rec=fct-bzel2phc@3
name: A refusal holds nothing a call supplied
rule: Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
@row_check n=218 row=decision/refusal-holds-no-call-input sha=83eb8bd60fbf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/refusal.rs:49
quote: pub const fn name(self) -> &'static str {
at: packages/escher-driver/src/refusal.rs:63
quote: pub const fn meaning(self) -> &'static str {
at: packages/escher-driver/src/refusal.rs:96
quote: pub const fn remedy(self) -> &'static str {
at: packages/escher-driver/src/refusal.rs:135
quote: Missing(&'static str),
@row_add n=219 row=contract/refusal-causes/unknown-verb area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: unknown-verb
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_check n=219 row=contract/refusal-causes/unknown-verb sha=b6e260e3a3e7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:283
quote: _ => Err(Refusal::new(Cause::UnknownVerb)),
@row_add n=220 row=contract/refusal-causes/malformed area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: malformed
promise: The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
@row_check n=220 row=contract/refusal-causes/malformed sha=bba474ae6221 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
at: packages/escher-driver/src/schema.rs:393
quote: args: &[],
at: packages/escher-driver/src/schema.rs:440
quote: args: &[],
@row_add n=221 row=contract/refusal-causes/not-found area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: not-found
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_check n=221 row=contract/refusal-causes/not-found sha=d7da09e133ea v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:474
quote: None => Err(Refusal::new(Cause::NotFound)),
@row_add n=222 row=contract/refusal-causes/stale area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: stale
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_check n=222 row=contract/refusal-causes/stale sha=ba95eb329ab8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:473
quote: None if read_earlier => Err(Refusal::new(Cause::Stale)),
@row_add n=223 row=contract/refusal-causes/disabled area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: disabled
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_check n=223 row=contract/refusal-causes/disabled sha=a421729830de v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:532
quote: return Err(Refusal::new(Cause::Disabled));
@row_add n=224 row=contract/refusal-causes/covered area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: covered
promise: The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
@row_check n=224 row=contract/refusal-causes/covered sha=4e1c11785eb2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:543
quote: return Err(Refusal::new(Cause::Covered));
at: packages/escher-driver/src/refusal.rs:76-77
quote: "another element is hit at the point the action would land; an option of a closed \ select lies under it: click the select first, then the option; a box clipped on \
@row_add n=225 row=contract/refusal-causes/off-screen area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: off-screen
promise: The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
@row_check n=225 row=contract/refusal-causes/off-screen sha=5e4332f45007 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:535
quote: return Err(Refusal::new(Cause::OffScreen));
at: packages/escher-driver/src/refusal.rs:115
quote: "the element is out of view: bring it into view with `scroll`, then act again"
@row_add n=226 row=contract/refusal-causes/fonts-unavailable area=error-handling rec=fct-bzel2phc@3
boundary: Driver refusal causes
item: fonts-unavailable
promise: One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
@row_check n=226 row=contract/refusal-causes/fonts-unavailable sha=f58972d99b81 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:225
quote: return Err(Refusal::new(Cause::FontsUnavailable));
@row_add n=227 row=contract/refusal-json/cause area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: cause
promise: The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
@row_check n=227 row=contract/refusal-json/cause sha=826a73c251ae v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:525
quote: refused.text("cause", cause.name());
@row_add n=228 row=contract/refusal-json/meaning area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: meaning
promise: The cause's fixed meaning text.
@row_check n=228 row=contract/refusal-json/meaning sha=ade9e539887b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:526
quote: refused.text("meaning", cause.meaning());
@row_add n=229 row=contract/refusal-json/remedy area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: remedy
promise: The cause's fixed remedy text.
@row_check n=229 row=contract/refusal-json/remedy sha=7166e58ab061 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:527
quote: refused.text("remedy", cause.remedy());
@row_add n=230 row=contract/refusal-json/fault area=error-handling rec=fct-bzel2phc@3
boundary: Refusal JSON
item: fault
promise: Present only on a `malformed` refusal, naming the fault.
@row_check n=230 row=contract/refusal-json/fault sha=8a039ed7659a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:528-529
quote: if let Some(fault) = self.fault() { refused.text("fault", &fault.to_string());
@row_add n=231 row=contract/session-error-json/kind area=error-handling rec=fct-bzel2phc@3
boundary: Session error JSON
item: kind
promise: One fixed kebab-case word per session error variant, inside `{"error":{...}}`.
@row_check n=231 row=contract/session-error-json/kind sha=d40fc64c05a1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:542
quote: error.text("kind", self.kind());
@row_add n=232 row=contract/session-error-json/message area=error-handling rec=fct-bzel2phc@3
boundary: Session error JSON
item: message
promise: The error's fixed meaning, holding no path.
@row_check n=232 row=contract/session-error-json/message sha=9f3cb6fea3bc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:543
quote: error.text("message", &self.to_string());
@row_add n=233 row=contract/session-error-json/remedy area=error-handling rec=fct-bzel2phc@3
boundary: Session error JSON
item: remedy
promise: The error's fixed remedy text.
@row_check n=233 row=contract/session-error-json/remedy sha=1910b81863d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:544
quote: error.text("remedy", self.remedy());
@row_add n=234 row=decision/session-errors-fixed-table area=error-handling rec=fct-bzel2phc@3
name: Session errors as one fixed table
rule: Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text.
why: Ratified as the program's own fixed text inside the ratified carriers.
@row_check n=234 row=decision/session-errors-fixed-table sha=b61aa6cf374a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/error.rs:80
quote: pub(crate) const SESSION_ERRORS: [ErrorSpec; 15] = [
at: packages/escher-driver/src/describe.rs:45
quote: json::list(SESSION_ERRORS.iter().map(|error| {
@row_add n=235 row=contract/mcp-tool-result/is-error area=error-handling rec=fct-bzel2phc@3
boundary: MCP tool result
item: isError
promise: A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.
@row_check n=235 row=contract/mcp-tool-result/is-error sha=a262b6368e9e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:101
quote: result.flag("isError", self.ending != Ending::Accepted);
@row_add n=236 row=contract/mcp-protocol-errors/parse-error area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: parse error
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_check n=236 row=contract/mcp-protocol-errors/parse-error sha=cc84bfee3af4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:122
quote: ProtocolError::Parse => "parse error",
at: packages/escher-driver/src/mcp/framing.rs:190-194
quote: pub(super) fn error_reply(id: &Id, error: ProtocolError) -> String { let mut object = Object::new(); object.json("code", &error.code().to_string()); object.text("message", error.message()); reply(id, "error", &object.end())
@row_add n=237 row=contract/mcp-protocol-errors/invalid-request area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: invalid request
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_check n=237 row=contract/mcp-protocol-errors/invalid-request sha=ab8087f0f3ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:123
quote: ProtocolError::InvalidRequest => "invalid request",
at: packages/escher-driver/src/mcp/framing.rs:163
quote: let id = id.flatten().unwrap_or(Id::Null);
@row_add n=238 row=contract/mcp-protocol-errors/method-not-found area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: method not found
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_check n=238 row=contract/mcp-protocol-errors/method-not-found sha=642f6ddc6020 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:124
quote: ProtocolError::MethodNotFound => "method not found",
at: packages/escher-driver/src/mcp.rs:174
quote: _ => error_reply(&id, ProtocolError::MethodNotFound),
@row_add n=239 row=contract/mcp-protocol-errors/invalid-params area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: invalid params
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_check n=239 row=contract/mcp-protocol-errors/invalid-params sha=4b84aa268541 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:125
quote: ProtocolError::InvalidParams => "invalid params",
at: packages/escher-driver/src/mcp.rs:185
quote: return Err(ProtocolError::InvalidParams);
@row_add n=240 row=contract/mcp-protocol-errors/unknown-tool area=error-handling rec=fct-bzel2phc@3
boundary: MCP protocol errors
item: unknown tool
promise: A fixed code and fixed message with no `data`; no reply holds the tool name.
@row_check n=240 row=contract/mcp-protocol-errors/unknown-tool sha=f9502c050d47 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:126
quote: ProtocolError::UnknownTool => "unknown tool",
at: packages/escher-driver/src/mcp.rs:201
quote: return Err(ProtocolError::UnknownTool);
@row_add n=241 row=decision/refusal-log-cause-name-only area=error-handling rec=fct-bzel2phc@3
name: Only a refusal's cause name is logged
rule: Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
@row_check n=241 row=decision/refusal-log-cause-name-only sha=c7b64a31bd28 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
@row_add n=242 row=decision/coverage-refuses-bad-checks-malformed area=error-handling rec=fct-iis2bn6b@1,fct-4spo7x3c@1
name: Coverage refuses an unusable check as malformed
rule: The `coverage` role refuses a check file it cannot read, one past the check bound, one that is no check or a check of another app as `malformed`, holding nothing of the path or the file, and adds no cause.
@row_check n=242 row=decision/coverage-refuses-bad-checks-malformed sha=249cb9088924 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
at: packages/escher-driver/src/cli.rs:497
quote: fn read_check(path: &Path) -> Option<Check> {
at: packages/escher-driver/src/recording.rs:334
quote: if bytes.len() > MAX_CHECK_BYTES {
@row_add n=243 row=decision/parallel-style-traversal-serialised area=error-handling rec=fct-d2ojiy3n@1
name: Parallel style traversals serialised
rule: Every parallel style traversal takes one process-wide lock, its poison recovered, so two documents resolving parallel at once are serialised.
why: Two concurrent parallel traversals otherwise panic `already mutably borrowed`; the lock adds no panic path.
@row_check n=243 row=decision/parallel-style-traversal-serialised sha=b41c2a9f64f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:158
quote: let _traversal_lock = parallel.then(|| {
at: packages/blitz-dom/src/stylo.rs:159
quote: PARALLEL_TRAVERSAL_LOCK
@row_add n=244 row=decision/dioxus-render-into-buffer area=error-handling rec=fct-7ni72vcj@3
name: Dioxus renders into a buffer before borrowing
rule: A Dioxus document renders mutations into a buffer with the document unborrowed, then borrows it once to replay, and flushes queued mounted events after the borrow is dropped.
why: So a task polled during render can reach the document through its mounted handle with no `RefCell already borrowed` panic.
guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_check n=244 row=decision/dioxus-render-into-buffer sha=9f9ce1d7f03a v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:318
quote: self.vdom.render_immediate(&mut self.buffered_mutations);
at: packages/dioxus-native-dom/src/dioxus_document.rs:324
quote: self.flush_queued_mounted_events();
guard_at: tests/blitz-tests/tests/dioxus_runtime_probe.rs:607
@row_add n=245 row=decision/skip-mounted-event-of-removed-element area=error-handling rec=fct-dpj6t54a@1
name: Mounted event of a removed element skipped
rule: Flushing queued mounted events skips an element removed since its event was queued, for every document.
why: Differs from upstream: upstream's test asserting no mounted event is left out, as it reads one on Dioxus 0.7.
@row_check n=245 row=decision/skip-mounted-event-of-removed-element sha=a8a83df82025 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:258-259
quote: let Some(node_id) = self.vdom_state.try_element_to_node_id(element_id) else { continue;
@row_add n=246 row=decision/dropped-anonymous-block-clears-parent area=error-handling rec=fct-5a4l32dj@1
name: A dropped anonymous block clears its children's parent
rule: Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a control whose block was dropped does not panic and still acts.
why: So the id does not go stale, curing every reader of a stale layout parent.
@row_check n=246 row=decision/dropped-anonymous-block-clears-parent sha=b21b1dea28ad v=holds guard=false judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1496-1500
quote: for child_id in children.iter().flatten().copied() { if let Some(child) = self.nodes.get(child_id) && child.layout_parent.get() == Some(anon_id) { child.layout_parent.set(None);
@row_add n=247 row=decision/agent-run-selection-allowlist area=input-validation rec=fct-biyghes6@1
name: agent-run.sh selection reaches cargo only as a test stem
rule: agent-run.sh exits 2 with usage before any cargo call on an unknown verb, a wrong argument count or a selection other than `stand`, `all` or an existing test-file stem matching `^[a-z0-9_]+$`.
@row_check n=247 row=decision/agent-run-selection-allowlist sha=5d4f3f46c85e v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:161
quote: [[ $# -ge 1 ]] && is_verb "$1" || usage
at: scripts/agent-run.sh:188
quote: [[ "$selection" =~ ^[a-z0-9_]+$ && -f "$TESTS_DIR/$selection.rs" ]] || usage
@row_add n=248 row=decision/cold-agent-call-validation area=input-validation rec=fct-gmgkgl3p@1
name: cold-agent.sh call and task allowlist
rule: cold-agent.sh exits 2 with usage on stderr and empty stdout before any precondition or `claude` call on an unknown verb, a wrong argument count or a task outside the allowlist `stand`.
@row_check n=248 row=decision/cold-agent-call-validation sha=2918a634a445 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:551
quote: [[ $# -ge 1 ]] && contains "$1" "${VERBS[@]}" || usage
at: scripts/cold-agent.sh:556
quote: [[ $# -eq 2 ]] && contains "$2" "${TASKS[@]}" || usage
at: scripts/cold-agent.sh:16
quote: TASKS=(stand)
@row_add n=249 row=decision/apt-install-argument-validation area=input-validation rec=fct-ixnvecdo@1
name: apt-install.sh validates every argument first
rule: apt-install.sh accepts only package names of a fixed form and bounded numeric option values, else prints usage and exits 2 before sudo, timeout, apt-get or dpkg runs.
guard: InstallScriptTest.test_a_bad_call_exits_2_before_anything_runs
@row_check n=249 row=decision/apt-install-argument-validation sha=b549ba290276 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:39
quote: [[ "$1" =~ ^[a-z0-9][a-z0-9+.-]+$ ]] || usage
at: .github/scripts/apt-install.sh:23
quote: [[ "$1" =~ ^(0|[1-9][0-9]{0,5})$ ]]
guard_at: .github/scripts/test_ci_workflows.py:504
@row_add n=250 row=decision/apt-install-no-secrets area=input-validation rec=fct-ixnvecdo@1
name: apt-install.sh touches no environment or secret
rule: apt-install.sh reads no environment variable of its own, writes no file and names no secret, token or GitHub context variable.
@row_check n=250 row=decision/apt-install-no-secrets sha=9d97e5073132 v=holds judge=sonnet head=edd0cfaea7f0
searched: GITHUB|secret|token|printenv|\$\{?[A-Z_]*ENV · in .github/scripts/apt-install.sh
searched: > file redirect other than >&2 · in .github/scripts/apt-install.sh
@row_add n=251 row=decision/apt-install-bounded-repair area=input-validation rec=fct-ixnvecdo@1
name: Bounded dpkg repair before a retry
rule: Every apt install attempt after the first first runs a time-bounded `dpkg --configure -a` repair, a failed repair failing that attempt; the first attempt runs no dpkg.
@row_check n=251 row=decision/apt-install-bounded-repair sha=ef2f5b7e7200 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:57-59
quote: if (($1 > 1)); then phase=repair repair || return
at: .github/scripts/apt-install.sh:53
quote: sudo timeout --kill-after="$GRACE" "$BOUND" dpkg --configure -a
@row_add n=252 row=decision/per-crate-coverage-floors area=input-validation rec=fct-jngrnarp@1
name: Per-crate line coverage floors
rule: The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90.
guard: coverage leg
@row_check n=252 row=decision/per-crate-coverage-floors sha=cd94bddcf9a9 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:48
quote: python3 "$SCRIPTS/coverage_floor.py" target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90
guard_at: .github/scripts/ci-leg.sh:48
@row_add n=253 row=decision/coverage-floor-input-validation area=input-validation rec=fct-jngrnarp@1
name: Coverage floor checker validates and prints no paths
rule: coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path.
guard: CoverageFloorTest
@row_check n=253 row=decision/coverage-floor-input-validation sha=0aac4b46c916 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/coverage_floor.py:33
quote: if percent > 100 or crate in floors:
at: .github/scripts/coverage_floor.py:84
quote: missing = [crate for crate in floors if crate not in sums]
at: .github/scripts/coverage_floor.py:94
quote: print(f"{crate} {covered}/{lines} = {reading:.2f} % (floor {floor:g} %)")
guard_at: .github/scripts/test_coverage_floor.py:17
@row_add n=254 row=decision/cold-agent-check-validation area=input-validation rec=fct-tbmm2cwc@3
name: Agent-written check validated before replay
rule: Only the reply's last fenced json block is taken, at most 65,536 bytes and 400 steps, each step of closed keys and a known verb; a check outside these is never replayed, and the check is never printed.
why: A boundary widening to a new input class, ratified.
@row_check n=254 row=decision/cold-agent-check-validation sha=80aa041243fc v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:277
quote: if not found or len(found[-1].encode("utf-8")) > MAX_CHECK_BYTES:
at: scripts/cold-agent.sh:373
quote: if len(check) > MAX_CHECK_STEPS or not all(valid_step(step) for step in check):
at: scripts/cold-agent.sh:289-290
quote: and set(step) <= {"tool", "arguments", "expect"} and step.get("tool") in VERBS
@row_add n=255 row=decision/cold-agent-replay-isolation area=input-validation rec=fct-tbmm2cwc@3
name: Agent-written check replayed on fresh servers
rule: A valid agent-written check is replayed on fresh MCP server children that validate every step again, each reply under a 30 s bound, each child killed and reaped, a failed child reading the check failed.
@row_check n=255 row=decision/cold-agent-replay-isolation sha=c9eeb4b855db v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:373
quote: if len(check) > MAX_CHECK_STEPS or not all(valid_step(step) for step in check):
at: scripts/cold-agent.sh:301-302
quote: self.proc = subprocess.Popen([binary, "mcp"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, encoding="utf-8", errors="replace")
at: scripts/cold-agent.sh:320
quote: line = self.lines.get(timeout=REPLY_BOUND)
at: scripts/cold-agent.sh:335-336
quote: self.proc.kill() self.proc.wait()
at: scripts/cold-agent.sh:378
quote: answers is not None
@row_add n=256 row=decision/session-socket-closed-grammar area=input-validation rec=fct-7jixhrbw@3
name: Session socket request is a closed grammar
rule: A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
@row_check n=256 row=decision/session-socket-closed-grammar sha=36757a5a9c10 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:180
quote: if line.len() > MAX_REQUEST_BYTES {
at: packages/escher-driver/src/wire.rs:108
quote: if plain(escaped) {
at: packages/escher-driver/src/wire.rs:193-194
quote: if !shaped { return Err(Reply::RefusedMalformed);
at: packages/escher-driver/src/host.rs:185
quote: Line::Unterminated => return Err(io::ErrorKind::UnexpectedEof.into()),
@row_add n=257 row=contract/session-socket-request/hello area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket request
item: hello
promise: `hello v3` asks for the session's lifecycle reply.
@row_check n=257 row=contract/session-socket-request/hello sha=e3d670be5925 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:204
quote: "hello" => Ok(Request::Hello),
at: packages/escher-driver/src/host.rs:188
quote: Ok(Request::Hello) => (
@row_add n=258 row=contract/session-socket-request/stop area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket request
item: stop
promise: `stop v3` stops the session once its reply is written.
@row_check n=258 row=contract/session-socket-request/stop sha=3ec45c35a06d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:205
quote: "stop" => Ok(Request::Stop),
at: packages/escher-driver/src/host.rs:197
quote: Ok(Request::Stop) => (Reply::Stopping, true),
at: packages/escher-driver/src/host.rs:212-213
quote: stream.write_all(reply.encode().as_bytes())?; Ok(stop)
@row_add n=259 row=contract/session-socket-request/call area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket request
item: call
promise: `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
@row_check n=259 row=contract/session-socket-request/call sha=888d8d00f3fb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:206-208
quote: _ => decode_call(&rest) .map(Request::Call) .ok_or(Reply::RefusedMalformed),
at: packages/escher-driver/src/wire.rs:128
quote: line.push_str("t:");
at: packages/escher-driver/src/wire.rs:131
quote: ArgValue::Number(number) => line.push_str(&format!("n:{number}")),
at: packages/escher-driver/src/wire.rs:132
quote: ArgValue::Flag(flag) => line.push_str(if *flag { "f:true" } else { "f:false" }),
at: packages/escher-driver/src/host.rs:199
quote: let answer = session.run(&call);
@row_add n=260 row=contract/session-socket-reply/refused-version area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: refused version
promise: A known word with another wire version; nothing runs.
@row_check n=260 row=contract/session-socket-reply/refused-version sha=08ae3dab316e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:196-198
quote: if version != VERSION { return Err(if is_version(version) { Reply::RefusedVersion
@row_add n=261 row=contract/session-socket-reply/refused-malformed area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: refused malformed
promise: A request line the grammar does not read or over 16,384 bytes; nothing runs.
@row_check n=261 row=contract/session-socket-reply/refused-malformed sha=710039a25474 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:180-181
quote: if line.len() > MAX_REQUEST_BYTES { return Err(Reply::RefusedMalformed);
@row_add n=262 row=contract/session-socket-reply/ok-accepted area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: ok v3 accepted
promise: The call ran and was accepted; its answer JSON follows on the line.
@row_check n=262 row=contract/session-socket-reply/ok-accepted sha=6cee8a7d73bd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:243
quote: Reply::Accepted(json) => format!("ok {VERSION} accepted {json}\n"),
@row_add n=263 row=contract/session-socket-reply/ok-refused area=input-validation rec=fct-7jixhrbw@3,fct-bzel2phc@3
boundary: Session socket reply
item: ok v3 refused
promise: The call was refused; the refusal JSON follows on the line.
@row_check n=263 row=contract/session-socket-reply/ok-refused sha=244bdd0dec81 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:244
quote: Reply::Refused(json) => format!("ok {VERSION} refused {json}\n"),
@row_add n=264 row=contract/session-socket-reply/ok-oversize area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: ok v3 oversize
promise: The call ran but its answer is over 1,048,576 bytes.
@row_check n=264 row=contract/session-socket-reply/ok-oversize sha=afd546819298 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:221-222
quote: if json.len() > MAX_ANSWER_BYTES { Reply::Oversize
@row_add n=265 row=contract/session-socket-reply/ok-recording-failed area=input-validation rec=fct-7jixhrbw@3
boundary: Session socket reply
item: ok v3 recording-failed
promise: The call ran and the host's recording ended; the client reads it as a recording-failed session error.
@row_check n=265 row=contract/session-socket-reply/ok-recording-failed sha=8f46054b26b3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:200-201
quote: let reply = if session.recording_failed() { Reply::RecordingFailed
at: packages/escher-driver/src/client.rs:106
quote: Reply::RecordingFailed => Err(SessionError::RecordingFailed),
@row_add n=266 row=decision/session-socket-bounds area=input-validation rec=fct-7jixhrbw@3
name: Session socket bounds
rule: One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes.
guard: an_answer_read_past_its_bound_is_a_timeout
@row_check n=266 row=decision/session-socket-bounds sha=a41f87832798 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:15
quote: pub(crate) const IO_BOUND: Duration = Duration::from_secs(2);
at: packages/escher-driver/src/wire.rs:18
quote: pub(crate) const ANSWER_BOUND: Duration = Duration::from_secs(30);
at: packages/escher-driver/src/wire.rs:23
quote: pub(crate) const MAX_REQUEST_BYTES: usize = 16_384;
at: packages/escher-driver/src/wire.rs:27
quote: pub(crate) const MAX_REPLY_BYTES: usize = 128;
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
at: packages/escher-driver/src/host.rs:146
quote: let mut stream = match listener.accept() {
guard_at: packages/escher-driver/src/client.rs:271
@row_add n=267 row=decision/socket-carries-screen-reading area=input-validation rec=fct-7jixhrbw@3
name: Session socket carries what the screen reads
rule: An answer carries what the screen reads, ids, accessible names, control values with password and file input masked, snapshot text, diff, lint findings, UI graph, never a NodeId or an attribute value as typed.
why: The crossing was ratified.
@row_check n=267 row=decision/socket-carries-screen-reading sha=caa6f6d42e24 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:13
quote: pub const MASKED_VALUE: &str = "••••••••";
at: packages/escher-driver/src/json.rs:505-506
quote: object.json("nodes", &list(graph.nodes.iter().map(graph_node))); object.json("states", &list(graph.states.iter().map(graph_state)));
@row_add n=268 row=decision/closed-value-field-no-widening area=input-validation rec=fct-7jixhrbw@3
name: A closed-value field in a ratified carrier is no widening
rule: A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does.
why: Ratified as the standing rule.
@row_check n=268 row=decision/closed-value-field-no-widening sha=c83410d86d81 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=269 row=contract/screenshot-answer/png area=input-validation rec=fct-7jixhrbw@3
boundary: Screenshot answer
item: png
promise: `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element's visible part.
@row_check n=269 row=contract/screenshot-answer/png sha=0d7fd22c3acb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:493-495
quote: Outcome::Shot { width, height, png } => { let mut object = shot(*width, *height); object.text("png", &image::base64(png));
@row_add n=270 row=contract/graph-answer/nodes-states area=input-validation rec=fct-7jixhrbw@3
boundary: Graph answer
item: nodes and states
promise: `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
@row_check n=270 row=contract/graph-answer/nodes-states sha=350ae1988d73 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:505-506
quote: object.json("nodes", &list(graph.nodes.iter().map(graph_node))); object.json("states", &list(graph.states.iter().map(graph_state)));
at: packages/dioxus-native-dom/src/graph.rs:85-86
quote: Self::Undeclared => "undeclared", Self::Unconfirmed => "unconfirmed",
@row_add n=271 row=contract/snapshot-text/selected area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: selected
promise: A bare word written for an element that states it is selected.
@row_check n=271 row=contract/snapshot-text/selected sha=66cd91612e65 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:113
quote: Some(true) => text.push_str(" selected"),
@row_add n=272 row=contract/snapshot-text/unselected area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: unselected
promise: A bare word written for an element that states it is not selected.
@row_check n=272 row=contract/snapshot-text/unselected sha=c749747d0068 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:114
quote: Some(false) => text.push_str(" unselected"),
@row_add n=273 row=contract/snapshot-text/pressed area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: pressed
promise: A bare word written for an element that states it is pressed.
@row_check n=273 row=contract/snapshot-text/pressed sha=1897397caf4e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:118
quote: Some(true) => text.push_str(" pressed"),
@row_add n=274 row=contract/snapshot-text/unpressed area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: unpressed
promise: A bare word written for an element that states it is not pressed.
@row_check n=274 row=contract/snapshot-text/unpressed sha=0f2e4b8a76f4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:119
quote: Some(false) => text.push_str(" unpressed"),
@row_add n=275 row=contract/snapshot-text/invalid area=input-validation rec=fct-7jixhrbw@3
boundary: Snapshot text state words
item: invalid
promise: A bare word written for an element that states it is invalid.
@row_check n=275 row=contract/snapshot-text/invalid sha=44136197e6b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:123
quote: text.push_str(" invalid");
@row_add n=276 row=contract/diff-node/selected area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: selected
promise: A boolean, written only where the element states it.
@row_check n=276 row=contract/diff-node/selected sha=56e921e466e5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:160-161
quote: if let Some(selected) = parts.selected { object.flag("selected", selected);
@row_add n=277 row=contract/diff-node/pressed area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: pressed
promise: A boolean, written only where the element states it.
@row_check n=277 row=contract/diff-node/pressed sha=e6f458c5dd7c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:163-164
quote: if let Some(pressed) = parts.pressed { object.flag("pressed", pressed);
@row_add n=278 row=contract/diff-node/invalid area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: invalid
promise: A boolean, always written.
@row_check n=278 row=contract/diff-node/invalid sha=6b0438927ca4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:166
quote: object.flag("invalid", parts.invalid);
@row_add n=279 row=contract/diff-node/differs area=input-validation rec=fct-7jixhrbw@3
boundary: Diff node keys
item: differs
promise: On a changed node only, after `bounds`: a list of the node-key names that changed; an added node is written without it.
@row_check n=279 row=contract/diff-node/differs sha=adf7e2b48a73 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:178-180
quote: if let Some(differs) = parts.differs { object.json( "differs",
@row_add n=280 row=decision/session-socket-unix-only area=input-validation rec=fct-7jixhrbw@3
name: Session socket is Unix only
rule: The driver session socket exists on Unix only; elsewhere a session is `Unsupported`.
@row_check n=280 row=decision/session-socket-unix-only sha=07026b6109ad v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:44
quote: #[cfg(unix)]
at: packages/escher-driver/src/host.rs:217
quote: #[cfg(not(unix))]
at: packages/escher-driver/src/host.rs:229
quote: Err(SessionError::Unsupported)
@row_add n=281 row=contract/snapshot-text/clickable area=input-validation rec=fct-i5kvbfe3@1
boundary: Snapshot text state words
item: clickable
promise: Written after `focused` only when the element's current listeners hold `click` and it is no disabled form control.
@row_check n=281 row=contract/snapshot-text/clickable sha=1ef434867332 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
at: packages/dioxus-native-dom/src/snapshot_text.rs:128-129
quote: if node.state.clickable { text.push_str(" clickable");
@row_add n=282 row=contract/diff-node/clickable area=input-validation rec=fct-i5kvbfe3@1
boundary: Diff node keys
item: clickable
promise: A boolean always written beside `focused`, true where the element's listeners hold `click` and it is no disabled form control; a changed node's `differs` can name it.
@row_check n=282 row=contract/diff-node/clickable sha=2c4d06a2aabd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:168
quote: object.flag("clickable", parts.clickable);
@row_add n=283 row=decision/state-dir-owner-only area=input-validation rec=fct-doehymzv@1
name: Session state directory is owner-only
rule: The session state directory is created mode 0700 and a present one granting group or others any bit is refused; the socket file is set 0600.
@row_check n=283 row=decision/state-dir-owner-only sha=4293824892e3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:92
quote: match DirBuilder::new().mode(0o700).create(state_dir) {
at: packages/escher-driver/src/host.rs:99
quote: if mode & 0o077 == 0 {
at: packages/escher-driver/src/host.rs:124
quote: fs::set_permissions(socket, Permissions::from_mode(0o600)).map_err(io_error)?;
@row_add n=284 row=decision/state-dir-deletes-only-own area=input-validation rec=fct-doehymzv@1
name: Session cleanup deletes only its own files
rule: Stopping or idle expiry removes the socket file and then the directory, non-recursively, and nothing else is ever deleted.
@row_check n=284 row=decision/state-dir-deletes-only-own sha=d137bacda4f0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:81-82
quote: let removed = fs::remove_file(&socket).map_err(io_error); let _ = fs::remove_dir(state_dir);
@row_add n=285 row=decision/state-dir-path-is-parameter area=input-validation rec=fct-doehymzv@1
name: Session state path is a parameter
rule: The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
@row_check n=285 row=decision/state-dir-path-is-parameter sha=b9473fddcf46 v=holds judge=sonnet head=edd0cfaea7f0
searched: env::var|current_dir|temp_dir|var_os · in packages/escher-driver/src/{cli,host,session,main,client}.rs (only a test in client.rs:276 uses temp_dir)
@row_add n=286 row=contract/session-errors/state-dir-too-long area=input-validation rec=fct-doehymzv@1
boundary: Session errors
item: state-dir-too-long
promise: The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.
@row_check n=286 row=contract/session-errors/state-dir-too-long sha=396cc3327b1f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/error.rs:66
quote: "the session state directory's path is too long for a socket: as given, it is at most 94 bytes";
at: packages/escher-driver/src/error.rs:69
quote: "the session state directory's path is too long for a socket: as given, it is at most 90 bytes";
at: packages/escher-driver/src/error.rs:128
quote: remedy: "name a shorter path for the state directory, or a relative one, which counts \
at: examples/seven_guis/tests/cli_commands.rs:925
quote: Some(3),
@row_add n=287 row=decision/recording-dir-owner-only area=input-validation rec=fct-ifbk2u2m@3
name: Recording directory is owner-only
rule: The recording directory is created mode 0700 and a present one open to group or others is refused `record-dir-not-private` with status 3, nothing written and nothing spawned.
why: Ratified.
@row_check n=287 row=decision/recording-dir-owner-only sha=d34cea28bf2b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:217
quote: if mode & 0o077 == 0 {
at: packages/escher-driver/src/recording.rs:220
quote: Err(SessionError::RecordDirNotPrivate)
at: packages/escher-driver/src/error.rs:164
quote: kind: "record-dir-not-private",
@row_add n=288 row=decision/recording-never-replaces-files area=input-validation rec=fct-ifbk2u2m@3
name: Recording never replaces or removes foreign files
rule: A recorded check file is created 0600 with create-new under the smallest free number, rewritten whole through a temp file and rename; no file is replaced and a file the recorder did not create is never removed.
@row_check n=288 row=decision/recording-never-replaces-files sha=25d0b8de62fb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:231-232
quote: .create_new(true) .mode(0o600)
at: packages/escher-driver/src/recording.rs:247
quote: Err(error) if error.kind() == io::ErrorKind::AlreadyExists => n += 1,
at: packages/escher-driver/src/recording.rs:264-266
quote: let renamed = written .write_all(text) .and_then(|()| fs::rename(temporary, file));
@row_add n=289 row=decision/recording-bounds-end-recording area=input-validation rec=fct-ifbk2u2m@3
name: A failed or overlong recording ends
rule: A failed write or a passed bound of 400 records or 1,048,576 bytes ends the recording, its check removed, the call run and not rolled back and answered `recording-failed`.
@row_check n=289 row=decision/recording-bounds-end-recording sha=4fe43bf494ae v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:182
quote: let written = self.records.len() <= MAX_CHECK_STEPS
at: packages/escher-driver/src/recording.rs:183
quote: && text.len() <= MAX_CHECK_BYTES
at: packages/escher-driver/src/recording.rs:186
quote: imp::remove(&self.file);
at: packages/escher-driver/src/host.rs:200-201
quote: let reply = if session.recording_failed() {                     Reply::RecordingFailed
@row_add n=290 row=decision/recording-dir-never-named area=input-validation rec=fct-ifbk2u2m@3
name: Recording directory is never exposed
rule: No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
why: Ratified.
@row_check n=290 row=decision/recording-dir-never-named sha=0472e62bb85c v=holds judge=sonnet head=edd0cfaea7f0
searched: record|tracing::|info!|debug!|warn!|error! · in packages/escher-driver/src/mcp.rs (only span.record of tool and ending fields)
searched: record_dir|recording_into · in packages/escher-driver/src/{session,host,wire,mcp}.rs (no result field or log names the directory)
@row_add n=291 row=decision/recorded-check-strict-reader area=input-validation rec=fct-v4gvn7w2@3
name: Recorded check is read strictly as untrusted
rule: A kept check is read as untrusted bytes, at most 1,048,576 bytes and 400 records of closed keys; anything else is refused `malformed` with no fault, holding nothing of the file, with no panic path.
@row_check n=291 row=decision/recorded-check-strict-reader sha=87f4c0005554 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:334
quote: if bytes.len() > MAX_CHECK_BYTES {
at: packages/escher-driver/src/recording.rs:340
quote: if records.len() > MAX_CHECK_STEPS {
at: packages/escher-driver/src/recording.rs:337
quote: let Ok(Value::Array(records)) = serde_json::from_slice::<Value>(bytes) else {
@row_add n=292 row=contract/recorded-check/start-record area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: start record
promise: The array opens with `{"tool":"start","arguments":{"app":…}}`.
@row_check n=292 row=contract/recorded-check/start-record sha=fd081bfc5667 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:404
quote: if record.get("tool")?.as_str()? != schema::START.name {
@row_add n=293 row=contract/recorded-check/tool area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: tool
promise: One of the ten instance verbs.
@row_check n=293 row=contract/recorded-check/tool sha=abf6eaa608b4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:425
quote: schema::verb(record.get("tool")?.as_str()?).filter(|verb| verb.level == Level::Instance)?;
@row_add n=294 row=contract/recorded-check/arguments area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: arguments
promise: Argument names drawn from that verb's schema.
@row_check n=294 row=contract/recorded-check/arguments sha=5a54f5bdfe0e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:426-427
quote: let names: Vec<&str> = verb.args.iter().map(|arg| arg.name).collect(); let arguments = object_of(record.get("arguments")?, &names)?;
@row_add n=295 row=contract/recorded-check/masked area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: masked
promise: True on a `type` record alone.
@row_check n=295 row=contract/recorded-check/masked sha=e1faf17200f3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:431
quote: Some(Value::Bool(true)) if verb.name == schema::TYPE.name => true,
@row_add n=296 row=contract/recorded-check/accepted area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: accepted
promise: One of the closed record keys, whether the call was accepted.
@row_check n=296 row=contract/recorded-check/accepted sha=b93745606a38 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:434
quote: let accepted = record.get("accepted")?.as_bool()?;
@row_add n=297 row=contract/recorded-check/digest area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: digest
promise: 16 lowercase hex characters.
@row_check n=297 row=contract/recorded-check/digest sha=cdb38d5097b0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:436-439
quote: let is_hex = digest.len() == 16 && digest .bytes() .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
@row_add n=298 row=contract/recorded-check/answer area=input-validation rec=fct-v4gvn7w2@3
boundary: Recorded check file
item: answer
promise: Required and a JSON object; key order is not enforced.
@row_check n=298 row=contract/recorded-check/answer sha=ba0e1bee26e2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:443
quote: let answer = record.get("answer").filter(|answer| answer.is_object())?;
@row_add n=299 row=decision/replay-through-session-run area=input-validation rec=fct-v4gvn7w2@3
name: Replay runs each step through the session
rule: A replayed check runs each kept step through the session's own validated run alone, runs nothing on a session of another app and stops at the first step whose acceptance or kept answer differs.
@row_check n=299 row=decision/replay-through-session-run sha=830527cdb6c1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/replay.rs:56
quote: if session.label() != self.app() {
at: packages/escher-driver/src/replay.rs:66
quote: let answer = session.run(step.call());
at: packages/escher-driver/src/replay.rs:70-71
quote: if change.is_some() { break;
@row_add n=300 row=decision/cold-agent-check-reader-counts-only area=input-validation rec=fct-v4gvn7w2@3
name: Cold-agent recording reader keeps counts only
rule: The cold-agent pipe's reader of its live child's kept checks holds bools and counts only, reads an unreadable directory as no check kept, and never prints or copies a check's content.
@row_check n=300 row=decision/cold-agent-check-reader-counts-only sha=472a8f6dacc4 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:262-263
quote: except OSError:         return 0, False
at: scripts/cold-agent.sh:271
quote: return len(numbered), bool(starts) and len(names) == len(numbered) and opened == starts
@row_add n=301 row=decision/session-label-form area=input-validation rec=fct-mybm64sc@1
name: Session label form
rule: A session label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs; it is never document text, and is the one caller string a reply carries.
@row_check n=301 row=decision/session-label-form sha=6b0df5666c7d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:56-59
quote: (1..=MAX_LABEL_BYTES).contains(&label.len()) && label .bytes() .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
at: packages/escher-driver/src/session.rs:153-155
quote: if !valid_label(label) { return Err(SessionError::InvalidLabel); }
@row_add n=302 row=decision/closed-driver-verb-schema area=input-validation rec=fct-awfxp663@3
name: Closed driver verb schema
rule: Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing.
why: Each widening of the verb table is a widening of a validated surface, ratified.
@row_check n=302 row=decision/closed-driver-verb-schema sha=756beb7774cb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:254
quote: pub fn validate(call: &Call) -> Result<Command, Refusal> {
at: packages/escher-driver/src/execute.rs:202
quote: let command = validate(call)?;
@row_add n=303 row=decision/one-validation-path-for-every-carrier area=input-validation rec=fct-awfxp663@3,fct-gucmzkph@3,fct-f4eyvu3v@3
name: One validation path for every carrier
rule: The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation.
why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
@row_check n=303 row=decision/one-validation-path-for-every-carrier sha=a7fef529339c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:199
quote: let answer = session.run(&call);
at: packages/escher-driver/src/client.rs:84
quote: if let Err(refusal) = validate(call) {
at: packages/escher-driver/src/mcp.rs:220
quote: return match validate(call) {
at: packages/escher-driver/src/execute.rs:202
quote: let command = validate(call)?;
@row_add n=304 row=contract/driver-argument-kinds/id area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: id
promise: Text of 1 to 1024 bytes.
@row_check n=304 row=contract/driver-argument-kinds/id sha=19ca4d8916d3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:348
quote: if !text.is_empty() && text.len() <= MAX_ID_BYTES {
at: packages/escher-driver/src/schema.rs:9
quote: pub const MAX_ID_BYTES: usize = 1024;
@row_add n=305 row=contract/driver-argument-kinds/text area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: text
promise: Text of 0 to 4096 bytes.
@row_check n=305 row=contract/driver-argument-kinds/text sha=ff1d88f3abcd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:355
quote: if text.len() <= MAX_TEXT_BYTES {
at: packages/escher-driver/src/schema.rs:12
quote: pub const MAX_TEXT_BYTES: usize = 4096;
@row_add n=306 row=contract/driver-argument-kinds/key area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: key
promise: One name of a closed twelve-name list.
@row_check n=306 row=contract/driver-argument-kinds/key sha=ba77b2c4c922 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:362
quote: Key::from_name(name).map(Admitted::Key).ok_or(out_of_bound)
at: packages/escher-driver/src/schema.rs:21
quote: pub const KEY_NAMES: [&str; 12] = [
@row_add n=307 row=contract/driver-argument-kinds/flag area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: flag
promise: True or false.
@row_check n=307 row=contract/driver-argument-kinds/flag sha=fd06129ad6f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:364
quote: (ArgKind::Flag, ArgValue::Flag(flag)) => Ok(Admitted::Flag(*flag)),
@row_add n=308 row=contract/driver-argument-kinds/milliseconds area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: milliseconds
promise: A whole number from 1 to 60000.
@row_check n=308 row=contract/driver-argument-kinds/milliseconds sha=695c879a5706 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:367
quote: .filter(|ms| (1..=MAX_MILLISECONDS).contains(ms))
at: packages/escher-driver/src/schema.rs:15
quote: pub const MAX_MILLISECONDS: u32 = 60_000;
@row_add n=309 row=contract/driver-argument-kinds/name area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: name
promise: Text of 1 to 32 bytes of a-z, 0-9 and -.
@row_check n=309 row=contract/driver-argument-kinds/name sha=026a17e046f0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:56-59
quote: (1..=MAX_LABEL_BYTES).contains(&label.len()) && label .bytes() .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
at: packages/escher-driver/src/session.rs:14
quote: const MAX_LABEL_BYTES: usize = 32;
@row_add n=310 row=contract/driver-argument-kinds/part area=input-validation rec=fct-awfxp663@3
boundary: Driver argument kinds
item: part
promise: A whole number from 1 to 1000.
@row_check n=310 row=contract/driver-argument-kinds/part sha=af493b7a1577 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:372
quote: .filter(|part| (1..=MAX_PART).contains(part))
at: packages/escher-driver/src/schema.rs:18
quote: pub const MAX_PART: u32 = 1000;
@row_add n=311 row=decision/unknown-id-refused-stale-or-not-found area=input-validation rec=fct-awfxp663@3
name: Unknown id refused as stale or not-found
rule: An id that names no element on the screen as it reads now is refused with nothing run: stale when the session's record of ids holds it, not-found when not.
@row_check n=311 row=decision/unknown-id-refused-stale-or-not-found sha=a4272dee596e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:473-474
quote: None if read_earlier => Err(Refusal::new(Cause::Stale)), None => Err(Refusal::new(Cause::NotFound)),
@row_add n=312 row=decision/target-refusal-order area=input-validation rec=fct-awfxp663@3
name: Fixed order of target refusals
rule: A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing.
why: The detection of refusal causes by code was ratified.
@row_check n=312 row=decision/target-refusal-order sha=4fbd8c8ed30a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:531-536
quote: if target.enabled == Some(false) { return Err(Refusal::new(Cause::Disabled)); } if !in_view(harness, target) { return Err(Refusal::new(Cause::OffScreen)); }
at: packages/escher-driver/src/execute.rs:542-543
quote: if covered(harness, target, page) { return Err(Refusal::new(Cause::Covered));
at: packages/escher-driver/src/execute.rs:282
quote: check_target(harness, &command, &target)?;
@row_add n=313 row=decision/advance-moves-virtual-clock-exactly area=input-validation rec=fct-awfxp663@3
name: Advance moves the virtual clock exactly
rule: The advance verb, accepted on every session, moves the session's virtual clock by exactly the milliseconds asked inside one settled step and reports exactly that time.
why: Ratified.
@row_check n=313 row=decision/advance-moves-virtual-clock-exactly sha=153a057f8273 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:268
quote: harness.advance_time(Duration::from_millis(u64::from(ms)));
at: packages/escher-driver/src/execute.rs:277
quote: advanced_ms: ms,
@row_add n=314 row=decision/id-record-bounded-and-session-fed area=input-validation rec=fct-awfxp663@3
name: Bounded id record fed by the session
rule: The session keeps no node handle between calls, only a record of at most 4096 id texts fed from its own snapshots and never from a call's argument.
@row_check n=314 row=decision/id-record-bounded-and-session-fed sha=eb94bbe4bc53 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:67
quote: pub(crate) const MAX_SEEN_IDS: usize = 4096;
at: packages/escher-driver/src/execute.rs:470
quote: record(seen, &screen);
@row_add n=315 row=decision/screen-pixels-in-band area=input-validation rec=fct-awfxp663@3,fct-gucmzkph@3,fct-f4eyvu3v@3
name: Screen pixels in band
rule: A screenshot's PNG travels inside the answer itself as base64 text, on the command line and as an MCP image block.
why: Ratified as the in-band crossing.
@row_check n=315 row=decision/screen-pixels-in-band sha=9879d772c677 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
at: packages/escher-driver/src/mcp/result.rs:92
quote: block.text("mimeType", "image/png");
@row_add n=316 row=decision/one-command-span-per-call area=input-validation rec=fct-awfxp663@3
name: One command span per call
rule: Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
@row_check n=316 row=decision/one-command-span-per-call sha=97f0b1747375 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:119
quote: tracing::info_span!(target: "escher_driver", "command", $($field = Empty),+)
at: packages/escher-driver/src/execute.rs:181-182
quote: let span = command_span(); let _entered = span.enter();
@row_add n=317 row=contract/driver-command-span/verb area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: verb
promise: The verb table's own word, never the caller's verb text.
@row_check n=317 row=contract/driver-command-span/verb sha=1dbc54429345 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:184
quote: span.record("verb", row.name);
@row_add n=318 row=contract/driver-command-span/cause area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: cause
promise: The name of the returned refusal's cause, unrecorded until it applies.
@row_check n=318 row=contract/driver-command-span/cause sha=b1bc7647eb99 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
@row_add n=319 row=contract/driver-command-span/settled area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: settled
promise: A boolean, unrecorded until it applies.
@row_check n=319 row=contract/driver-command-span/settled sha=a3239c373b87 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:372
quote: span.record("settled", settled);
@row_add n=320 row=contract/driver-command-span/busy area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: busy
promise: The schema's class word, unrecorded until it applies.
@row_check n=320 row=contract/driver-command-span/busy sha=a47848817ac9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:374
quote: span.record("busy", busy_word(busy));
@row_add n=321 row=contract/driver-command-span/passes area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: passes
promise: A count, unrecorded until it applies.
@row_check n=321 row=contract/driver-command-span/passes sha=70c3403f0a54 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:614
quote: span.record("passes", settled.passes);
@row_add n=322 row=contract/driver-command-span/added area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: added
promise: The length of the returned diff's added list.
@row_check n=322 row=contract/driver-command-span/added sha=446457f990b7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:376
quote: span.record("added", diff.added.len());
@row_add n=323 row=contract/driver-command-span/removed area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: removed
promise: The length of the returned diff's removed list.
@row_check n=323 row=contract/driver-command-span/removed sha=ab51871c0d9b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:377
quote: span.record("removed", diff.removed.len());
@row_add n=324 row=contract/driver-command-span/changed area=input-validation rec=fct-awfxp663@3
boundary: The driver command span
item: changed
promise: The length of the returned diff's changed list.
@row_check n=324 row=contract/driver-command-span/changed sha=3f7a67f5fa06 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:378
quote: span.record("changed", diff.changed.len());
@row_add n=325 row=contract/mcp-tool-call-span/tool area=input-validation rec=fct-awfxp663@3
boundary: The MCP tool_call span
item: tool
promise: The verb table's word for the called tool; the span parents the call's command span.
@row_check n=325 row=contract/mcp-tool-call-span/tool sha=8e91b1557ecf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:192
quote: span.record("tool", verb.name);
at: packages/escher-driver/src/mcp.rs:161
quote: let _entered = span.enter();
@row_add n=326 row=contract/mcp-tool-call-span/ending area=input-validation rec=fct-awfxp663@3
boundary: The MCP tool_call span
item: ending
promise: One of accepted, refused or error.
@row_check n=326 row=contract/mcp-tool-call-span/ending sha=7269b80754f6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:22
quote: Ending::Accepted => "accepted",
at: packages/escher-driver/src/mcp/result.rs:23
quote: Ending::Refused => "refused",
at: packages/escher-driver/src/mcp/result.rs:24
quote: Ending::Error => "error",
@row_add n=327 row=contract/driver-verbs/graph area=input-validation rec=fct-2njwkumb@1
boundary: Driver verbs
item: graph
promise: Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
@row_check n=327 row=contract/driver-verbs/graph sha=ff3908b78b7e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:280
quote: ("graph", []) => Ok(Command::Graph),
at: packages/escher-driver/src/json.rs:503
quote: Outcome::Graphed { graph } => {
@row_add n=328 row=contract/driver-verbs/actions area=input-validation rec=fct-gczaiqlz@1
boundary: Driver verbs
item: actions
promise: Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
@row_check n=328 row=contract/driver-verbs/actions sha=cef7aa960225 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:281
quote: ("actions", []) => Ok(Command::Actions),
at: packages/escher-driver/src/json.rs:253
quote: fn action_node(node: &ActionNode) -> String {
@row_add n=329 row=contract/cli-exit-codes/0 area=input-validation rec=fct-gucmzkph@3
boundary: CLI exit codes
item: 0
promise: Accepted: the answer as one line of JSON on stdout.
@row_check n=329 row=contract/cli-exit-codes/0 sha=cf2cdda44d6c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:343
quote: status: ACCEPTED,
@row_add n=330 row=decision/no-echo-of-caller-input area=input-validation rec=fct-gucmzkph@3,fct-jwfmxwld@1
name: No echo of caller input
rule: No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
@row_check n=330 row=decision/no-echo-of-caller-input sha=dd0a890e2d00 v=holds judge=sonnet head=edd0cfaea7f0
searched: Ending::Accepted|Ending::Refused|Ending::Usage|Ending::Failed written fields · in packages/escher-driver/src/cli.rs (writes only the answer, a refusal, a fixed usage line from the verb table and app names, or a fixed error message)
searched: Fault::Missing|Fault::WrongKind|Fault::OutOfBound text · in packages/escher-driver/src/refusal.rs (names only the schema's argument name, not the caller's value)
@row_add n=331 row=decision/closed-pipe-drops-answer area=input-validation rec=fct-gucmzkph@3
name: Closed pipe drops the answer
rule: A write to a closed stdout is dropped and the command keeps its exit status, never a panic.
guard: a_closed_stdout_drops_the_answer_and_keeps_the_ending
@row_check n=331 row=decision/closed-pipe-drops-answer sha=7b8798f2cb98 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:410-413
quote: let _ = stdout .write_all(line.as_bytes()) .and_then(|()| stdout.write_all(b"\n")) .and_then(|()| stdout.flush());
guard_at: examples/seven_guis/tests/cli_commands.rs:844
@row_add n=332 row=contract/cli-flags/id area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --id
promise: Followed by one value, whatever that word is.
@row_check n=332 row=contract/cli-flags/id sha=dd8ad6a0c1aa v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:292
quote: Some(value) => ArgValue::Text(value.clone()),
@row_add n=333 row=contract/cli-flags/text area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --text
promise: Followed by one value, whatever that word is.
@row_check n=333 row=contract/cli-flags/text sha=e0362a77f922 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:292
quote: Some(value) => ArgValue::Text(value.clone()),
@row_add n=334 row=contract/cli-flags/key area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --key
promise: Followed by one value, whatever that word is.
@row_check n=334 row=contract/cli-flags/key sha=7238c5477057 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:292
quote: Some(value) => ArgValue::Text(value.clone()),
@row_add n=335 row=contract/cli-flags/ms area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --ms
promise: Followed by one value, whatever that word is.
@row_check n=335 row=contract/cli-flags/ms sha=f8fd16642afe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:287
quote: Some(value) if matches!(flag.kind, ArgKind::Milliseconds | ArgKind::Part) => {
@row_add n=336 row=contract/cli-flags/part area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --part
promise: Followed by one value, whatever that word is.
@row_check n=336 row=contract/cli-flags/part sha=9dc086fd876d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:287
quote: Some(value) if matches!(flag.kind, ArgKind::Milliseconds | ArgKind::Part) => {
@row_add n=337 row=contract/cli-flags/shift area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --shift
promise: Bare, taking no value.
@row_check n=337 row=contract/cli-flags/shift sha=26d91b114242 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:282-283
quote: (Some(flag), _) if flag.kind == ArgKind::Flag => { (flag.name.to_string(), ArgValue::Flag(true))
@row_add n=338 row=contract/cli-flags/session area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --session
promise: Followed by a directory, given exactly once, else usage.
@row_check n=338 row=contract/cli-flags/session sha=588a67254fda v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:218-219
quote: let [Some(state_dir)] = state_dirs.as_slice() else { return Line::Usage;
@row_add n=339 row=contract/cli-flags/record area=input-validation rec=fct-gucmzkph@3
boundary: escher-session flags
item: --record
promise: Followed by a directory, taken at most once and only on start, the host role and the mcp role; anywhere else, twice or with no value is usage.
@row_check n=339 row=contract/cli-flags/record sha=e0a5b3598dd8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:225-226
quote: [Some(record)] if starts => Some(PathBuf::from(record)), _ => return Line::Usage,
at: packages/escher-driver/src/cli.rs:244
quote: [option, dir] if option == RECORD_OPTION => Some(Some(PathBuf::from(dir))),
@row_add n=340 row=contract/cli-roles/serve area=input-validation rec=fct-gucmzkph@3
boundary: escher-session roles
item: serve
promise: serve <app> --session <dir> in exactly that shape, optionally with one --record <dir> after it, else usage.
@row_check n=340 row=contract/cli-roles/serve sha=7f4094bc07ff v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=341 row=contract/cli-roles/mcp area=input-validation rec=fct-gucmzkph@3
boundary: escher-session roles
item: mcp
promise: mcp alone or with one --record <dir> selects the MCP role; any other word after it is usage.
@row_check n=341 row=contract/cli-roles/mcp sha=691dd409d97b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:177-181
quote: if word == MCP_ROLE { return match record_option(rest) { Some(record) => Line::Mcp { record }, None => Line::Usage, };
@row_add n=342 row=contract/cli-roles/coverage area=input-validation rec=fct-jwfmxwld@1
boundary: escher-session roles
item: coverage
promise: coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
@row_check n=342 row=contract/cli-roles/coverage sha=a2d45ba4c869 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:185-190
quote: [app, checks @ ..] if apps.contains(&app.as_str()) && !checks.is_empty() && !checks .iter() .any(|check| check == SESSION_OPTION || check == RECORD_OPTION) =>
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
at: packages/escher-driver/src/recording.rs:32
quote: pub(crate) const MAX_CHECK_BYTES: usize = 1_048_576;
@row_add n=343 row=contract/coverage-answer/line area=input-validation rec=fct-jwfmxwld@1
boundary: The coverage answer
item: line
promise: One line {"unexercised":[…],"checks":n,"held":m} on stdout, status 0, holding ids, owner ids, event names and the two counts only.
@row_check n=343 row=contract/coverage-answer/line sha=53db5c3c7e53 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:437-438
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX)); object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
@row_add n=344 row=decision/mcp-over-stdio area=input-validation rec=fct-f4eyvu3v@3
name: MCP over stdio only
rule: The MCP server speaks newline-delimited JSON-RPC 2.0 on the invoking process's stdin and stdout, with no listener, port, socket, authentication or environment read.
why: The MCP carrier and its bounds were ratified as is.
@row_check n=344 row=decision/mcp-over-stdio sha=2139e516b69a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:84-86
quote: pub fn serve_mcp( mut input: impl BufRead, mut output: impl Write,
at: packages/escher-driver/src/cli.rs:444-446
quote: Line::Mcp { record: None } => Ending::Spoken(serve_mcp( io::stdin().lock(), io::stdout().lock(),
@row_add n=345 row=contract/mcp-request/line-length area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP requests
item: request line length
promise: At most 65,536 bytes; a longer line is read to its terminator, dropped and answered -32700.
@row_check n=345 row=contract/mcp-request/line-length sha=b4877b8b8008 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:11
quote: pub(super) const MAX_MCP_REQUEST_BYTES: usize = 65_536;
at: packages/escher-driver/src/mcp.rs:107
quote: write_line(&mut output, &error_reply(&Id::Null, ProtocolError::Parse))
@row_add n=346 row=contract/mcp-request/id area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP requests
item: id
promise: Echoed only as an i64 integer or a string of at most 128 bytes, else -32600 with id null; the one client-sent value written back.
@row_check n=346 row=contract/mcp-request/id sha=42fc0b2ba949 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:81-82
quote: Value::Number(number) => number.as_i64().map(Id::Number), Value::String(text) if text.len() <= MAX_ID_BYTES => Some(Id::Text(text.clone())),
at: packages/escher-driver/src/mcp/framing.rs:169
quote: Some(None) => Envelope::Error(Id::Null, ProtocolError::InvalidRequest),
@row_add n=347 row=contract/mcp-request/notification area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP requests
item: a request with no id
promise: Treated as a notification: no reply, and nothing runs.
@row_check n=347 row=contract/mcp-request/notification sha=1e1814521d24 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:151
quote: Envelope::Notification => return Ok(()),
@row_add n=348 row=contract/mcp-errors/32700 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32700
promise: parse error, with no data: a non-UTF-8, non-JSON or overlong line.
@row_check n=348 row=contract/mcp-errors/32700 sha=d62ec541672d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:150
quote: return Envelope::Error(Id::Null, ProtocolError::Parse);
at: packages/escher-driver/src/mcp/framing.rs:153
quote: return Envelope::Error(Id::Null, ProtocolError::Parse);
at: packages/escher-driver/src/mcp.rs:107
quote: write_line(&mut output, &error_reply(&Id::Null, ProtocolError::Parse))
@row_add n=349 row=contract/mcp-errors/32600 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32600
promise: invalid request, with no data: not an object with jsonrpc "2.0" and a string method, or an id of another form.
@row_check n=349 row=contract/mcp-errors/32600 sha=7cd4d4951448 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:164
quote: return Envelope::Error(id, ProtocolError::InvalidRequest);
at: packages/escher-driver/src/mcp/framing.rs:169
quote: Some(None) => Envelope::Error(Id::Null, ProtocolError::InvalidRequest),
@row_add n=350 row=contract/mcp-errors/32601 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32601
promise: method not found, with no data: every method but initialize, ping, tools/list and tools/call.
@row_check n=350 row=contract/mcp-errors/32601 sha=a9ed855103da v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:156-158
quote: "initialize" => result_reply(&id, &initialized(params.as_ref())), "ping" => result_reply(&id, &Object::new().end()), "tools/list" => result_reply(&id, &tool_list(self.apps)),
at: packages/escher-driver/src/mcp.rs:174
quote: _ => error_reply(&id, ProtocolError::MethodNotFound),
@row_add n=351 row=contract/mcp-errors/32602 area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP protocol errors
item: -32602
promise: invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
@row_check n=351 row=contract/mcp-errors/32602 sha=661b9a52cee2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:187-189
quote: let Some(Value::String(name)) = params.get("name") else { return Err(ProtocolError::InvalidParams); };
at: packages/escher-driver/src/mcp.rs:198
quote: Some(_) => return Err(ProtocolError::InvalidParams),
at: packages/escher-driver/src/mcp.rs:200-201
quote: let Some(verb) = verb else { return Err(ProtocolError::UnknownTool);
@row_add n=352 row=contract/mcp-initialize/protocol-revision area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP initialize
item: protocol revision
promise: Matched against a closed table of four supported revisions; any other revision asked for is answered with the newest of the four.
@row_check n=352 row=contract/mcp-initialize/protocol-revision sha=7f0793c08ebd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:14
quote: const PROTOCOL_VERSIONS: [&str; 4] = [LATEST_VERSION, "2025-06-18", "2025-03-26", "2024-11-05"];
at: packages/escher-driver/src/mcp/listing.rs:37
quote: .unwrap_or(LATEST_VERSION);
@row_add n=353 row=contract/mcp-tool-arguments/value-kinds area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP tool arguments
item: value kinds
promise: A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
@row_check n=353 row=contract/mcp-tool-arguments/value-kinds sha=018587f8479d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:42-45
quote: Value::String(text) => Some(ArgValue::Text(text.clone())), Value::Bool(flag) => Some(ArgValue::Flag(*flag)), Value::Number(number) => number.as_i64().map(ArgValue::Number), Value::Null | Value::Array(_) | Value::Object(_) => None,
at: packages/escher-driver/src/command.rs:52
quote: return Err(Refusal::malformed(fault));
@row_add n=354 row=contract/mcp-tool-result/size-bound area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP tool results
item: size bound
promise: A result's text over 1,048,576 bytes is replaced by the answer-too-large result; a result holding an image is bounded by its whole written form.
@row_check n=354 row=contract/mcp-tool-result/size-bound sha=796b2b2c55d7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:75-80
quote: let size = match self.image { Some(_) => self.written().len(), None => self.text.len(), }; if size > MAX_ANSWER_BYTES { ToolResult::failed(SessionError::AnswerTooLarge)
@row_add n=355 row=contract/mcp-tool-result/screenshot area=input-validation rec=fct-f4eyvu3v@3
boundary: MCP tool results
item: screenshot
promise: An accepted shot is an image block (base64 PNG, image/png) then a text block of the shot's line without png, isError false; a refused shot is one text block with isError true.
@row_check n=355 row=contract/mcp-tool-result/screenshot sha=00ab372bb0a4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:88-92
quote: if let Some(data) = &self.image { let mut block = Object::new(); block.text("type", "image"); block.text("data", data); block.text("mimeType", "image/png");
at: packages/escher-driver/src/mcp/result.rs:101
quote: result.flag("isError", self.ending != Ending::Accepted);
@row_add n=356 row=decision/mcp-listing-fixed-text area=input-validation rec=fct-f4eyvu3v@3
name: MCP listing is fixed program text
rule: The MCP tool listing's descriptions are the program's own fixed table text: each verb's help and each result field's name, kind and help.
why: Ratified as the program's own text inside the ratified carrier.
@row_check n=356 row=decision/mcp-listing-fixed-text sha=dc99fc5f1c07 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:56
quote: .map(|field| format!("{} ({}): {}", field.name, field.kind.name(), field.help))
at: packages/escher-driver/src/mcp/listing.rs:58
quote: format!("{}. Returns: {}", verb.help, fields.join("; "))
@row_add n=357 row=decision/external-navigation-get-http-mailto area=input-validation rec=fct-3uvcy3lv@1
name: External navigation limited to safe schemes
rule: Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
@row_check n=357 row=decision/external-navigation-get-http-mailto sha=77ecca1755d4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/link_handler.rs:10-11
quote: if options.method == Method::GET && matches!(options.url.scheme(), "http" | "https" | "mailto")
@row_add n=358 row=decision/file-urls-only-from-file-documents area=input-validation rec=fct-kd7pemzk@1
name: file URLs load only from file documents
rule: A script fetch, a module import or a script src may load a file: URL only for a file: document, in every document, remote pages included.
why: A narrowing, not a boundary widening.
guard: remote_document_cannot_load_file_urls
@row_check n=358 row=decision/file-urls-only-from-file-documents sha=d338e0f52481 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:44-45
quote: url.scheme() != "file" || document_url.is_some_and(|document_url| document_url.scheme() == "file")
at: packages/blitz-vibey-script/src/runtime/fetch.rs:40
quote: let result = if document_may_load(base_url.as_ref(), &url) {
at: packages/blitz-vibey-script/src/runtime/modules.rs:77
quote: let result = if document_may_load(self.base_url.as_ref(), &url) {
at: packages/blitz-vibey-script/src/document.rs:163
quote: if !document_may_load(self.base_url.as_ref(), &url) {
guard_at: packages/blitz-vibey-script/tests/dom.rs:945
@row_add n=359 row=decision/default-fetcher-file-and-data-only area=input-validation rec=fct-kd7pemzk@1
name: Default script fetcher takes file and data only
rule: The default script fetcher takes only file: and data: URLs.
@row_check n=359 row=decision/default-fetcher-file-and-data-only sha=b834012ed1bd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:54
quote: "file" => {
at: packages/blitz-vibey-script/src/fetch.rs:60
quote: "data" => {
at: packages/blitz-vibey-script/src/fetch.rs:68
quote: scheme => Err(FetchError::UnsupportedScheme(scheme.to_string())),
@row_add n=360 row=absence/same-origin-and-cors-checks area=input-validation rec=fct-kd7pemzk@1
capability: Same-origin and CORS checks on script fetches
@row_check n=360 row=absence/same-origin-and-cors-checks sha=d0e3bafc91b0 v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)cors|same.origin|\.origin\( · in packages/blitz-vibey-script/src (only url.origin() serialisations at runtime/window.rs:74 and dom/hyperlink.rs:124, no check)
@row_add n=361 row=decision/html-parsed-without-scripting area=input-validation rec=fct-mf6hp2ln@1
name: HTML parsed with scripting disabled
rule: HTML is parsed with the parser's scripting flag disabled.
@row_check n=361 row=decision/html-parsed-without-scripting sha=7585780c476e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-html/src/html_sink.rs:109
quote: scripting_enabled: false, // Enables parsing of <noscript> tags
at: packages/blitz-html/src/html_sink.rs:149
quote: scripting_enabled: false, // Enables parsing of <noscript> tags
@row_add n=362 row=decision/failed-stylesheet-blocks-no-render area=input-validation rec=fct-3e6nbz2f@1
name: Failed stylesheet blocks no render
rule: In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render.
guard: failed_stylesheet_unblocks_rendering, empty_or_non_css_stylesheet_unblocks_rendering
@row_check n=362 row=decision/failed-stylesheet-blocks-no-render sha=2a37a723da63 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:149-150
quote: let Ok(css) = std::str::from_utf8(&bytes) else { return self.respond(resolved_url, Err(String::from("Invalid UTF8")));
at: packages/blitz-dom/src/net.rs:144-145
quote: fn error(self: Box<Self>, url: String, error: String) { self.respond(url, Err(error));
guard_at: tests/blitz-tests/tests/render_blocking_stylesheet.rs:41
@row_add n=363 row=decision/unresolvable-stylesheet-href-no-load area=input-validation rec=fct-zppva2v4@1
name: Unresolvable stylesheet href starts no load
rule: In every document, a stylesheet href that cannot be resolved starts no load and blocks no render.
why: Resolution goes through the non-panicking resolver instead of the one that panics.
guard: invalid_stylesheet_url_does_not_block_rendering
@row_check n=363 row=decision/unresolvable-stylesheet-href-no-load sha=e4765e44246f v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1392-1393
quote: let Some(url) = self.doc.url.resolve_relative(href) else { return;
guard_at: tests/blitz-tests/tests/render_blocking_stylesheet.rs:94
@row_add n=364 row=decision/hintless-font-face-source-sniffed area=input-validation rec=fct-kgjrj3js@1
name: Hint-less font-face source fetched and sniffed
rule: An @font-face source whose URL cannot be resolved is skipped; a source with no format hint and no extension is fetched and its format sniffed from its bytes.
why: Ratified as upstream's behaviour, with a check asked for.
guard: font_face_sniffed_source
@row_check n=364 row=decision/hintless-font-face-source-sniffed sha=cf8e6faece93 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:500-503
quote: let Some(url) = url_source.url.url() else { #[cfg(feature = "tracing")] tracing::warn!("Skipping @font-face source with unresolvable url"); return None;
at: packages/blitz-dom/src/net.rs:464-465
quote: if format == FontFaceSourceFormatKeyword::None && let Some((_, end)) = url_source.url.as_str().rsplit_once('.')
guard_at: tests/blitz-tests/tests/font_face_sniffed_source.rs:1
@row_add n=365 row=absence/url-taking-driver-verb area=input-validation rec=fct-kgjrj3js@1
capability: A driver verb that takes a URL
@row_check n=365 row=absence/url-taking-driver-verb sha=0f8531b604f7 v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)\burl\b|\bhref\b · in packages/escher-driver/src
searched: pub const VERBS · in packages/escher-driver/src/schema.rs (verbs: snapshot click type press advance scroll screenshot lint graph actions start status stop help)
@row_add n=366 row=decision/multiple-select-size-rows area=input-validation rec=fct-j5csdx44@1
name: Multiple select size sets its rows
rule: A multiple select's size, a whole number above zero capped at 65534, makes it that many rows tall and a key scrolls the next row in; zero or a non-numeric value gives no hint, and a single select's size is unread.
why: A boundary widening, ratified.
guard: size_reads_a_whole_number_above_zero_capped
@row_check n=366 row=decision/multiple-select-size-rows sha=ef27bbe43a36 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select_list.rs:129
quote: (rows > 0).then_some(rows.min(MAX_SIZE))
at: packages/blitz-dom/src/stylo.rs:1134-1136
quote: if *tag == local_name!("select") && *name == local_name!("size") && elem.has_attr(local_name!("multiple"))
guard_at: packages/blitz-dom/src/node/select_list.rs:460
@row_add n=367 row=decision/disabled-read-by-presence area=input-validation rec=fct-mbi5iago@2
name: disabled read by presence on form controls
rule: The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer and keyboard activation and the snapshot's enabled and clickable readings.
why: The presence reading was ratified.
@row_check n=367 row=decision/disabled-read-by-presence sha=c103540a65f2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:519
quote: self.can_be_disabled() && self.has_attr(local_name!("disabled"))
at: packages/dioxus-native-dom/src/snapshot.rs:182-183
quote: .can_be_disabled() .then(|| !element.has_attr(local_name!("disabled"))),
@row_add n=368 row=decision/custom-widget-initial-attribute-replay area=input-validation rec=fct-ka33jnh7@1
name: Custom widget initial attribute replay
rule: Behind the custom-widget feature, setting a custom widget replays each existing attribute, parsed page content included, to the widget with no old value, so the widget receives them at set-up and checking each value stays the widget's own job.
why: An engine change for every document behind the feature, ratified.
guard: custom_widget_replays_initial_attributes
@row_check n=368 row=decision/custom-widget-initial-attribute-replay sha=cc553e7eb0f8 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:904-905
quote: for attr in self.attrs.iter() { widget.attribute_changed(&attr.name.local, None, Some(&attr.value));
guard_at: packages/blitz-dom/src/mutator.rs:1658
@row_add n=369 row=contract/stable-element-id/author-key area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: author key
promise: An element's HTML `id` is its author key only when it is non-empty, holds no `/` and is the first element in document pre-order to claim it; an empty, `/`-bearing or later-duplicate value gives no key and anchors nothing.
@row_check n=369 row=contract/stable-element-id/author-key sha=503db7f2015b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:349-350
quote: .filter(|key| !key.is_empty() && !key.contains('/')) .filter(|key| claimed_keys.insert(key.to_string()));
@row_add n=370 row=contract/stable-element-id/anchored-path area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: anchored path
promise: An element with no author key reads `{key}//{segment}` (then `/{segment}` per further DOM level) under the nearest keyed element of its own component instance, else its component path, else its document path; an anchor acts inside one component only.
@row_check n=370 row=contract/stable-element-id/anchored-path sha=f4c6022d3203 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:353
quote: Some(key) => (key.to_string(), Some(format!("{key}/"))),
at: packages/dioxus-native-dom/src/element_id.rs:402
quote: Some(root) if owner != parent_owner => format!("{}/{segment}", root.owner.chain),
at: packages/dioxus-native-dom/src/element_id.rs:403
quote: _ => format!("{parent_path}/{segment}"),
@row_add n=371 row=decision/anchored-id-tier area=input-validation rec=fct-d2yejoup@5
name: Anchored stable-id tier
rule: An unkeyed element's stable id is anchored under the nearest author-keyed element of its own component instance before falling back to its component path or document path.
why: The anchored tier was ratified.
@row_check n=371 row=decision/anchored-id-tier sha=f1e5b8797e16 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: anchor · in packages/dioxus-native-dom/src (only doc comments and a test named the_nearest_keyed_ancestor_anchors; the doing line in placed_ids was not read)
why: no evidence found
@row_add n=372 row=contract/stable-element-id/no-collision area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: uniqueness
promise: An author key holds no `/`, an anchored path is the only id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal, a key spelled like a component name included.
@row_check n=372 row=contract/stable-element-id/no-collision sha=61f7f10e2321 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:349
quote: .filter(|key| !key.is_empty() && !key.contains('/'))
at: packages/dioxus-native-dom/src/element_id.rs:353
quote: Some(key) => (key.to_string(), Some(format!("{key}/"))),
@row_add n=373 row=contract/stable-element-id/unresolved-node area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: non-element, stale or detached node
promise: A non-element, stale or detached node reads no id, and no path computation panics.
@row_check n=373 row=contract/stable-element-id/unresolved-node sha=27a2218886af v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:421
quote: doc.get_node(node)?.element_data()?;
@row_add n=374 row=contract/stable-element-id/keyed-list-segment area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: keyed list row segment
promise: A keyed list row's segment carries its Dioxus key as `{tag}[{key}]`; a Dioxus key that is empty or holds `/` is refused and the row reads the positional `{tag}:{n}` segment instead.
@row_check n=374 row=contract/stable-element-id/keyed-list-segment sha=920f8430b9b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:394-399
quote: let key_segment = root .and_then(|root| root.key.as_deref()) .filter(|key| !key.is_empty() && !key.contains('/')) .map(|key| format!("{tag}[{key}]")) .filter(|segment| keyed.insert((owner, segment.clone()))); let segment = key_segment.unwrap_or_else(|| format!("{tag}:{index}"));
@row_add n=375 row=contract/stable-element-id/stability area=input-validation rec=fct-d2yejoup@5
boundary: Stable element id
item: stability
promise: A stable id is never a list index, pointer, hash, clock or process-local value, so a row reads the same id across a re-render, a remount and a second process.
@row_check n=375 row=contract/stable-element-id/stability sha=59f2a9671df8 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:145-146
quote: key: "{person.id}", id: "crud-person-{person.id}",
@row_add n=376 row=decision/no-process-handle-crosses area=input-validation rec=fct-d2yejoup@5
name: No process-local handle crosses with an id
rule: Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
@row_check n=376 row=decision/no-process-handle-crosses sha=87add18de7c1 v=holds judge=sonnet head=edd0cfaea7f0
searched: NodeId|ElementId|ScopeId · in packages/escher-driver/src/json.rs (0 hits)
@row_add n=377 row=decision/stable-id-not-logged area=input-validation rec=fct-d2yejoup@5
name: Stable ids kept out of logs
rule: Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level.
why: Those third-party targets are outside the sink's allowlist.
@row_check n=377 row=decision/stable-id-not-logged sha=dded2d132760 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:98-99
quote: if is_outside_target(target) { Verdict::Drop
at: packages/escher-telemetry/src/format.rs:94
quote: !is_engine_target(target) && !is_escher_target(target)
@row_add n=378 row=decision/author-id-on-accessibility-nodes area=input-validation rec=fct-d2yejoup@5
name: Stable id as accessibility author id
rule: Under the accessibility feature each element node of the accessibility tree carries its stable id as its AccessKit author id, never on a text run, the document root or the synthetic window node, so it leaves the process only through the platform adapter.
why: Ratified; the crossing text runs and labels already take.
@row_check n=378 row=decision/author-id-on-accessibility-nodes sha=8cae586c4c50 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:349-350
quote: if let Some(id) = ids.remove(&node_id.0) { node.set_author_id(id);
@row_add n=379 row=decision/accessibility-tree-refresh-on-change area=input-validation rec=fct-d2yejoup@5
name: Accessibility tree refreshed on change
rule: The platform accessibility adapter receives the tree on change as well as on its initial request: a poll that took a non-empty changed set rebuilds the platform tree.
why: Ratified: the same crossing and the same content classes, more often.
@row_check n=379 row=decision/accessibility-tree-refresh-on-change sha=eba4f9e5fee5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:379-380
quote: if !changed_nodes.is_empty() { self.accessibility.update_tree(&*self.doc);
@row_add n=380 row=decision/accessibility-adapter-content-classes area=input-validation rec=fct-d2yejoup@5,fct-q2xi4t6k@3
name: Content classes on the accessibility adapter crossing
rule: The platform accessibility adapter crossing carries four author-derived content classes: element ids, accessible names, control states and descriptions.
why: Each widening ratified; the states by ruling, the names' and descriptions' upstream additions ratified as upstream's behaviour.
@row_check n=380 row=decision/accessibility-adapter-content-classes sha=20c7cbe3fb57 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:350
quote: node.set_author_id(id);
at: packages/blitz-dom/src/accessibility.rs:163
quote: builder.set_label(label);
at: packages/blitz-dom/src/accessibility.rs:208
quote: Some(true) => builder.set_toggled(Toggled::True),
at: packages/blitz-dom/src/accessibility.rs:253
quote: builder.set_description(aria_description);
@row_add n=381 row=contract/snapshot-text/id-field area=input-validation rec=fct-d2yejoup@5
boundary: Snapshot text
item: id field
promise: Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
@row_check n=381 row=contract/snapshot-text/id-field sha=97d7fd372349 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:98
quote: write!(text, "{:?} {:?} id={:?}", node.role, node.name, node.id).unwrap();
@row_add n=382 row=decision/stable-id-exits area=input-validation rec=fct-d2yejoup@5
name: Ratified exits of the stable id
rule: The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files.
why: Each carrier ratified as a boundary widening.
@row_check n=382 row=decision/stable-id-exits sha=400daded0732 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:350
quote: node.set_author_id(id);
at: packages/escher-driver/src/cli.rs:341
quote: stdout: Some(answer.clone()),
at: packages/escher-driver/src/execute.rs:190
quote: && !recording.record(call, masked, &result)
searched: tracing:: · in packages/dioxus-native-dom/src/{element_id,snapshot,snapshot_diff,snapshot_text,actionable}.rs
@row_add n=383 row=decision/diff-counts-only-in-logs area=input-validation rec=fct-d2yejoup@5
name: Only diff sizes reach a log
rule: A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below.
why: Ratified: a diff's content never reaches a log while its three sizes may.
@row_check n=383 row=decision/diff-counts-only-in-logs sha=4ad31b1f0b49 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:376
quote: span.record("added", diff.added.len());
at: packages/escher-driver/src/execute.rs:377
quote: span.record("removed", diff.removed.len());
at: packages/escher-driver/src/execute.rs:378
quote: span.record("changed", diff.changed.len());
@row_add n=384 row=decision/id-lookup-anew-each-call area=input-validation rec=fct-d2yejoup@5
name: Caller-supplied id looked up anew
rule: An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
@row_check n=384 row=decision/id-lookup-anew-each-call sha=74df19748b6f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:467
quote: let screen = harness.doc.snapshot();
at: packages/escher-driver/src/execute.rs:473
quote: None if read_earlier => Err(Refusal::new(Cause::Stale)),
at: packages/escher-driver/src/execute.rs:474
quote: None => Err(Refusal::new(Cause::NotFound)),
@row_add n=385 row=contract/driver-id-refusal/stale area=input-validation rec=fct-d2yejoup@5
boundary: Driver id refusal
item: stale
promise: A call naming an id that no element on the current screen carries is refused as `stale` when the session's record holds that id.
@row_check n=385 row=contract/driver-id-refusal/stale sha=7c21034e4148 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:473
quote: None if read_earlier => Err(Refusal::new(Cause::Stale)),
@row_add n=386 row=contract/driver-id-refusal/not-found area=input-validation rec=fct-d2yejoup@5
boundary: Driver id refusal
item: not-found
promise: A call naming an id that no element on the current screen carries is refused as `not-found` when the session's record does not hold that id.
@row_check n=386 row=contract/driver-id-refusal/not-found sha=6225d5af24ac v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:474
quote: None => Err(Refusal::new(Cause::NotFound)),
@row_add n=387 row=contract/driver-call-id/length-bound area=input-validation rec=fct-d2yejoup@5
boundary: Driver call id
item: length bound
promise: An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
@row_check n=387 row=contract/driver-call-id/length-bound sha=8b8ae908253b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:348
quote: if !text.is_empty() && text.len() <= MAX_ID_BYTES {
at: packages/escher-driver/src/client.rs:84
quote: if let Err(refusal) = validate(call) {
at: packages/escher-driver/src/execute.rs:202
quote: let command = validate(call)?;
at: packages/escher-driver/src/mcp.rs:220
quote: return match validate(call) {
@row_add n=388 row=decision/session-id-record-bounded area=input-validation rec=fct-d2yejoup@5
name: Bounded in-process record of read ids
rule: A session keeps a private record of ids its own snapshots read, at most 4096 ids of at most 1024 bytes each, the oldest forgotten first, read by no public item and written to no log, event, socket or file.
why: Kept to tell the stale and not-found refusals apart.
guard: the_record_holds_no_id_longer_than_a_call_can_name
@row_check n=388 row=decision/session-id-record-bounded sha=f64e3fd5ea22 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:67
quote: pub(crate) const MAX_SEEN_IDS: usize = 4096;
at: packages/escher-driver/src/session.rs:93
quote: if id.len() > MAX_ID_BYTES {
at: packages/escher-driver/src/session.rs:102
quote: if self.last_read.len() > self.bound {
guard_at: packages/escher-driver/src/session.rs:294
@row_add n=389 row=decision/lint-findings-closed-content area=input-validation rec=fct-d2yejoup@5
name: Lint findings hold no page content
rule: A lint finding holds ids, closed defect words and numbers only, never page text, colour or a control value, and no log site, print, span or field carries a finding.
why: A ratified boundary widening.
@row_check n=389 row=decision/lint-findings-closed-content sha=5777f7d5e266 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:180
quote: pub id: String,
searched: tracing:: · in packages/escher-driver/src/lint.rs
searched: println! · in packages/escher-driver/src/lint.rs
@row_add n=390 row=contract/escher-session-coverage/answer-line area=input-validation rec=fct-ptvbryzl@1
boundary: escher-session coverage role
item: answer line
promise: The coverage answer holds handler references of ids, owner ids and event names and two counts, no process-local handle, and crosses only as one line on the command line's stdout, with no verb, socket or MCP carrier.
@row_check n=390 row=contract/escher-session-coverage/answer-line sha=bf869da96071 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:437-438
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX)); object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
at: packages/escher-driver/src/cli.rs:469
quote: handler_coverage(&checks, || boot(&app)).map(|coverage| coverage.to_json()),
@row_add n=391 row=contract/stable-element-id/virtual-node-form area=input-validation rec=fct-kc374vpe@1
boundary: Stable element id
item: virtual-node form
promise: The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/`; no served id holds `///`, and the anchored path stays the only served id holding `//`.
@row_check n=391 row=contract/stable-element-id/virtual-node-form sha=0f92e2bcb0cf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:46
quote: (!key.is_empty() && !key.contains('/')).then(|| format!("{owner}{VIRTUAL_KEY_SEPARATOR}{key}"))
at: packages/dioxus-native-dom/src/element_id.rs:776
quote: assert!(!id.contains(super::VIRTUAL_KEY_SEPARATOR), "{id:?}");
@row_add n=392 row=decision/aria-label-trimmed-once area=input-validation rec=fct-r5yt6pat@1
name: One trimmed aria-label read
rule: An `aria-label` non-empty after trimming becomes the node's label and a whitespace-only one names nothing; there is one trimmed, non-empty `aria-label` read.
why: Follows accname-1.2; upstream's unfiltered read was dropped in the merge.
guard: accessibility_label
@row_check n=392 row=decision/aria-label-trimmed-once sha=b110ad6a97ef v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:159-163
quote: if let Some(label) = element_data .attr(local_name!("aria-label")) .filter(|label| !label.trim().is_empty()) { builder.set_label(label);
guard_at: tests/blitz-tests/tests/accessibility_label.rs:1
@row_add n=393 row=decision/aria-labelledby-outranks-label area=input-validation rec=fct-r5yt6pat@1
name: aria-labelledby outranks label and aria-label
rule: An element's `aria-labelledby` targets' text, gathered cycle-guarded, is its computed label, outranking a `<label>` and an `aria-label`; an id naming no element is ignored and `aria-label` is the fallback when the targets hold no text.
why: Upstream's behaviour, ratified at the upstream sync.
guard: accessibility_label
@row_check n=393 row=decision/aria-labelledby-outranks-label sha=69b4ba1c6d53 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:83-84
quote: if !label.text.is_empty() { builder.set_label(label.text.as_str());
guard_at: tests/blitz-tests/tests/accessibility_label.rs:1
@row_add n=394 row=decision/accessible-names-not-logged area=input-validation rec=fct-r5yt6pat@1
name: Accessible names kept out of logs
rule: Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole.
why: That third-party target is outside the sink's allowlist.
guard: mcp_log
@row_check n=394 row=decision/accessible-names-not-logged sha=7a39f228ca09 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:19-21
quote: /// Together with [`ESCHER_TARGET_PREFIXES`] this is the sink's allowlist of targets: an event /// whose target starts with a prefix of neither set is dropped — nothing is written for it, at /// any level and whatever `RUST_LOG` names.
guard_at: examples/seven_guis/tests/mcp_log.rs:58
@row_add n=395 row=decision/accessible-name-exits area=input-validation rec=fct-r5yt6pat@1
name: Ratified exits of accessible names
rule: Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log.
why: Each carrier ratified.
@row_check n=395 row=decision/accessible-name-exits sha=045679e413e0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:163
quote: builder.set_label(label);
at: packages/escher-driver/src/cli.rs:341
quote: stdout: Some(answer.clone()),
@row_add n=396 row=contract/snapshot-text/name-field area=input-validation rec=fct-r5yt6pat@1
boundary: Snapshot text
item: name field
promise: Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
@row_check n=396 row=contract/snapshot-text/name-field sha=3967d2afd783 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:98
quote: write!(text, "{:?} {:?} id={:?}", node.role, node.name, node.id).unwrap();
@row_add n=397 row=decision/descriptions-adapter-only area=input-validation rec=fct-q2xi4t6k@3
name: Descriptions cross only to the accessibility adapter
rule: `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one.
why: Upstream's behaviour, ratified as a boundary widening by upstream merge.
guard: accessibility_description
@row_check n=397 row=decision/descriptions-adapter-only sha=992f1d773e48 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:98
quote: builder.push_described_by(NodeId(target.as_u64()));
at: packages/blitz-dom/src/accessibility.rs:253
quote: builder.set_description(aria_description);
searched: description · in packages/dioxus-native-dom/src/snapshot*.rs
guard_at: tests/blitz-tests/tests/accessibility_description.rs:8
@row_add n=398 row=decision/merged-aria-states-closed-parse area=input-validation rec=fct-lxb6gzob@1
name: Closed parse of expanded, disabled and level
rule: `aria-expanded`, `aria-disabled` and `aria-level` are read as closed values carrying none of the attribute's text and exit only on the accessibility adapter; `aria-disabled` moves no snapshot reading.
why: As upstream wrote them, inside the ratified states carrier.
guard: accessibility_merged_states
@row_check n=398 row=decision/merged-aria-states-closed-parse sha=e21e82312d31 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:248
quote: if element_data.attr(local_name!("aria-disabled")) == Some("true") {
at: packages/blitz-dom/src/accessibility.rs:256
quote: && let Ok(aria_level) = aria_level.parse::<usize>()
guard_at: tests/blitz-tests/tests/accessibility_merged_states.rs:1
@row_add n=399 row=decision/select-expanded-from-engine-state area=input-validation rec=fct-qmjsf46a@1
name: Select expansion from engine state only
rule: An author's `open` on a select and `aria-expanded` on a single select steer nothing: a single select's expanded state is the engine's own closed state, and `open` on a select does not match `:open`.
why: An engine change for every document.
guard: an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match
@row_check n=399 row=decision/select-expanded-from-engine-state sha=fe9c21f15425 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:241
quote: builder.set_expanded(self.select_list_for(node.id).is_some());
at: packages/blitz-dom/src/node/element.rs:546
quote: local_name!("details") | local_name!("dialog")
guard_at: tests/blitz-tests/tests/select_popup.rs:625
@row_add n=400 row=decision/aria-state-lowercase-exact-match area=input-validation rec=fct-agnqdsgq@3
name: Control-state values matched exactly in lower case
rule: `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` values are trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set; an unrecognized value sets nothing, but for `aria-invalid`, which reads invalid.
why: A choice: the specification as read states no case rule and no browser was measured; the `aria-invalid` fallback is WAI-ARIA 1.2's rule.
except: A select's option reads its chosen state as selected, never `aria-selected`.
@row_check n=400 row=decision/aria-state-lowercase-exact-match sha=91483bb70bde v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:190
quote: match selected.map(str::trim_ascii) {
at: packages/blitz-dom/src/accessibility.rs:235
quote: Some(_) => builder.set_invalid(Invalid::True),
at: packages/blitz-dom/src/accessibility.rs:187
quote: builder.set_selected(select::is_chosen(node));
@row_add n=401 row=decision/toggle-read-per-role area=input-validation rec=fct-agnqdsgq@3,fct-nkkmfss5@1
name: Toggle state read per role
rule: `aria-checked` sets the toggle only on the five checkable roles and `aria-pressed` only on every other role; a native checkbox or radio reads neither attribute and gives its own checked state under a checkable role and nothing under any other.
why: Ratified.
@row_check n=401 row=decision/toggle-read-per-role sha=5127b5828f21 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:214-217
quote: let toggled = if is_checkable(role) { element_data.attr(local_name!("aria-checked")) } else { element_data.attr(local_name!("aria-pressed"))
at: packages/blitz-dom/src/accessibility.rs:207
quote: match element_data.checkbox_input_checked() {
@row_add n=402 row=decision/control-states-closed-values-only area=input-validation rec=fct-agnqdsgq@3
name: Control states cross as closed values
rule: Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed.
why: A ratified widening of the adapter crossing by a new author-derived class.
@row_check n=402 row=decision/control-states-closed-values-only sha=dff6711bfd9d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:222
quote: Some("mixed") => builder.set_toggled(Toggled::Mixed),
at: packages/blitz-dom/src/accessibility.rs:235
quote: Some(_) => builder.set_invalid(Invalid::True),
@row_add n=403 row=contract/snapshot-text/state-words area=input-validation rec=fct-agnqdsgq@3
boundary: Snapshot text
item: state words
promise: Control states are written as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid`; in JSON answers they are booleans.
@row_check n=403 row=contract/snapshot-text/state-words sha=3c514d7631ad v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:113-114
quote: Some(true) => text.push_str(" selected"), Some(false) => text.push_str(" unselected"),
at: packages/dioxus-native-dom/src/snapshot_text.rs:118-119
quote: Some(true) => text.push_str(" pressed"), Some(false) => text.push_str(" unpressed"),
at: packages/dioxus-native-dom/src/snapshot_text.rs:123
quote: text.push_str(" invalid");
at: packages/escher-driver/src/json.rs:161
quote: object.flag("selected", selected);
@row_add n=404 row=decision/inert-subtree area=input-validation rec=fct-v47atmzs@1
name: Inert subtree
rule: An element carrying `inert`, or under an ancestor that does, is not focusable, takes no hit, leaves the accessibility tree and the snapshot, and a node holding focus when it comes under `inert` gives focus up; presence only, no value read.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_check n=404 row=decision/inert-subtree sha=e5ec6d69e78c v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:585
quote: .is_some_and(|element| element.has_attr(local_name!("inert")))
at: packages/blitz-dom/src/node/node.rs:574
quote: && !self.is_inert()
at: packages/blitz-dom/src/node/node.rs:1454-1459
quote: if self .element_data() .is_some_and(|element| element.has_attr(local_name!("inert"))) { return None; }
at: packages/blitz-dom/src/accessibility.rs:289
quote: element.has_attr(local_name!("inert"))
at: packages/blitz-dom/src/mutator.rs:1128-1135
quote: if self .doc .focus_node_id .and_then(|id| self.doc.get_node(id)) .is_some_and(|node| node.is_inert()) { self.doc.clear_focus(); }
guard_at: tests/blitz-tests/tests/inert_subtree.rs:78
@row_add n=405 row=decision/select-list-open-gate area=input-validation rec=fct-jhscqn7a@1
name: Which select opens a list
rule: A disabled, inert or hidden select, a select with no row to show, and one under a non-invertible transform open no list.
why: An engine change for every document and a boundary widening, ratified.
guard: a_disabled_inert_hidden_singular_or_empty_select_opens_nothing
@row_check n=405 row=decision/select-list-open-gate sha=a3926d9511b1 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:802-805
quote: && !element.is_disabled() && !node.is_inert() && shown() && invertible()
at: packages/blitz-dom/src/document.rs:825
quote: if !self.select_list_allowed(select) {
guard_at: tests/blitz-tests/tests/select_popup.rs:444
@row_add n=406 row=decision/select-list-row-choice area=input-validation rec=fct-jhscqn7a@1
name: What a select list row takes
rule: A disabled row of an open select list changes nothing and leaves the list open, and a group header chooses nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: a_disabled_row_changes_nothing_and_leaves_the_list_open
@row_check n=406 row=decision/select-list-row-choice sha=b05e9319df95 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:650-651
quote: if doc.get_node(row).is_some_and(select::option_disabled) { return;
guard_at: tests/blitz-tests/tests/select_popup.rs:378
@row_add n=407 row=decision/accessibility-focus-built-nodes-only area=input-validation rec=fct-dcrmx3m6@1
name: Accessibility focus names a built node
rule: The accessibility tree's focus names the focused node only when that node is among the built nodes, else the window, so the tree never names a node it did not build.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_check n=407 row=decision/accessibility-focus-built-nodes-only sha=89eef0c3604c v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:106-109
quote: let focus = self .focus_node_id .filter(|id| nodes.contains_key(id)) .map_or(u64::MAX, |id| id.as_u64());
guard_at: tests/blitz-tests/tests/inert_subtree.rs:134
@row_add n=408 row=decision/inert-focus-clear-silent area=input-validation rec=fct-dcrmx3m6@1,fct-v47atmzs@1
name: Inert focus clear fires no blur
rule: A node that loses focus by coming under `inert` fires no DOM `blur` or `focusout`.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_check n=408 row=decision/inert-focus-clear-silent sha=8d9cc19d4617 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1126-1127
quote: // both, with no `inert` arm on the attribute writes. No `blur` fires: the mutator holds // no route to dispatch one.
at: packages/blitz-dom/src/mutator.rs:1134
quote: self.doc.clear_focus();
guard_at: tests/blitz-tests/tests/inert_subtree.rs:78
@row_add n=409 row=decision/range-current-value-apart area=input-validation rec=fct-wujpgoms@1
name: Range input current value apart from its attribute
rule: A range input holds a current value apart from its `value` attribute; a `value` write re-seeds it and a `min`, `max` or `step` write or removal re-sanitizes it.
why: A departure from the spec's dirty-value flag, the cost the ratified ruling took on.
@row_check n=409 row=decision/range-current-value-apart sha=2f3c38953854 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:382
quote: element.reseed_range_input();
at: packages/blitz-dom/src/mutator.rs:353
quote: element.resanitize_range_input();
at: packages/blitz-dom/src/mutator.rs:522
quote: element.resanitize_range_input();
@row_add n=410 row=decision/range-value-sanitized area=input-validation rec=fct-wujpgoms@1
name: Range value always finite and on grid
rule: Every range seed, attribute write and move is clamped to its bounds and moved to the nearest grid value, the greater on a tie, so the value stays finite, bounded and on its grid on degenerate markup; unparsable numbers read as absent.
why: An engine change for every document and a boundary widening, ratified.
@row_check n=410 row=decision/range-value-sanitized sha=0944253fc6b0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/range.rs:145-149
quote: let value = self.clamp(if value.is_finite() { value } else { self.default_value() });
at: packages/blitz-dom/src/node/range.rs:158
quote: let nearest = (self.grid_index(value, step) + 0.5 + GRID_TOLERANCE).floor();
at: packages/blitz-dom/src/node/range.rs:93
quote: let parsed_min = min.and_then(parse_float);
@row_add n=411 row=decision/range-keyboard-actions area=input-validation rec=fct-wujpgoms@1
name: Range input key actions
rule: On a focused range input with no `disabled` attribute, `ArrowRight` or `ArrowUp` steps up, `ArrowLeft` or `ArrowDown` steps down, `Home` and `End` go to the minimum and maximum, and every other key does nothing.
why: Ratified.
@row_check n=411 row=decision/range-keyboard-actions sha=06077a9d2498 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:106-107
quote: } else if element_data.range_input_data().is_some() && !element_data.has_attr(local_name!("disabled"))
at: packages/blitz-dom/src/events/keyboard.rs:166
quote: Key::Named(NamedKey::ArrowRight) | Key::Named(NamedKey::ArrowUp) => RangeMove::StepUp,
at: packages/blitz-dom/src/events/keyboard.rs:167
quote: Key::Named(NamedKey::ArrowLeft) | Key::Named(NamedKey::ArrowDown) => RangeMove::StepDown,
at: packages/blitz-dom/src/events/keyboard.rs:168
quote: Key::Named(NamedKey::Home) => RangeMove::Minimum,
at: packages/blitz-dom/src/events/keyboard.rs:169
quote: Key::Named(NamedKey::End) => RangeMove::Maximum,
at: packages/blitz-dom/src/events/keyboard.rs:170
quote: _ => return,
@row_add n=412 row=decision/range-pointer-actions area=input-validation rec=fct-wujpgoms@1
name: Range input pointer actions
rule: A main-button pointerdown on an enabled range input sets the value from the hit's fraction of the content box, and a click focuses it and changes no value.
why: Ratified.
@row_check n=412 row=decision/range-pointer-actions sha=44c29691432b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:746
quote: if button == MouseEventButton::Main {
at: packages/blitz-dom/src/events/pointer.rs:749
quote: RangeMove::Fraction(f64::from(fraction)),
at: packages/blitz-dom/src/events/pointer.rs:842-850
quote: if el.range_input_data().is_some() { generate_focus_events( doc, &mut |doc| { doc.set_focus_to(node_id); }, dispatch_event, ); break 'matched true;
at: packages/blitz-dom/src/events/pointer.rs:472
quote: Some(el) if el.is_disabled() => ClickTarget::Disabled,
@row_add n=413 row=decision/range-value-exits area=input-validation rec=fct-wujpgoms@1
name: Range current value exits
rule: A range input's current value leaves only as a form submission entry, the `input` event's value and the snapshot's control value, and no accessibility node value or action changes.
why: An engine change for every document and a boundary widening, ratified.
@row_check n=413 row=decision/range-value-exits sha=491e39c7830d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:330
quote: } else if let Some(value) = element.range_input_value() {
at: packages/dioxus-native-dom/src/snapshot.rs:298
quote: .range_input_value()
at: packages/blitz-dom/src/accessibility.rs:549
quote: "range" => Some(Role::Slider),
@row_add n=414 row=decision/enter-activates-focused-control area=input-validation rec=fct-7zto53sb@1
name: Enter activates the focused control
rule: A pressed `Enter` on the focused node dispatches a click on it when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`.
why: An engine change for every document and a boundary widening, ratified.
@row_check n=414 row=decision/enter-activates-focused-control sha=a743206ba4f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:331-340
quote: fn activates_on_enter(element: &ElementData) -> bool { if element.is_disabled() { return false; } match element.name.local { local_name!("button") | local_name!("summary") => true, local_name!("input") => matches!( element.attr(local_name!("type")), Some("submit" | "reset" | "button") ),
at: packages/blitz-dom/src/events/keyboard.rs:341
quote: local_name!("a") => element.has_attr(local_name!("href")),
at: packages/blitz-dom/src/events/keyboard.rs:129
quote: dispatch_activation_click(doc, node_id, key_event.modifiers, &mut dispatch_event);
@row_add n=415 row=decision/space-keyup-activates-focused-control area=input-validation rec=fct-7zto53sb@1
name: Space release activates the focused control
rule: A released Space on the focused node dispatches a click for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled; a link, a text input and a range input take no Space.
why: An engine change for every document and a boundary widening, ratified.
@row_check n=415 row=decision/space-keyup-activates-focused-control sha=41a72b44fe4b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:348-357
quote: fn activates_on_space(element: &ElementData) -> bool { if element.is_disabled() { return false; } match element.name.local { local_name!("button") | local_name!("summary") => true, local_name!("input") => matches!( element.attr(local_name!("type")), Some("submit" | "reset" | "button" | "checkbox" | "radio") ),
at: packages/blitz-dom/src/events/keyboard.rs:358
quote: _ => false,
at: packages/blitz-dom/src/events/keyboard.rs:150-152
quote: .is_some_and(activates_on_space) { dispatch_activation_click(doc, target, event.modifiers, &mut dispatch_event);
@row_add n=416 row=decision/button-click-focuses area=input-validation rec=fct-7zto53sb@1
name: A button click focuses the button
rule: A click on a `button` of any type or an `input` of type `submit`, `reset` or `button` focuses it and submits its form owner when it is a submit button, a disabled form control stopping the walk first.
why: An engine change for every document and a boundary widening, ratified.
@row_check n=416 row=decision/button-click-focuses sha=c7f3f2fedcb1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:832-834
quote: if el.is_disabled() { break 'matched true; }
at: packages/blitz-dom/src/events/pointer.rs:1042-1044
quote: let submits = el.is_submit_button() || el.attr(local_name!("type")) == Some("submit"); click_button(doc, node_id, submits, dispatch_event);
at: packages/blitz-dom/src/events/pointer.rs:1178
quote: if submits && let Some(form_owner) = doc.controls_to_form.get(&node_id) {
@row_add n=417 row=decision/engine-change-event-checkable area=input-validation rec=fct-glmvut7v@1
name: Engine fires change on checkbox and radio
rule: A click on a checkbox always, and on a radio only where it was not checked before, sends `change` after `input` with the same value; the page script runtime synthesizes no `change`, so a script hears one, from the engine.
why: An engine change for every document and a boundary widening, ratified.
guard: checkable_change_event
@row_check n=417 row=decision/engine-change-event-checkable sha=18a4571f97c5 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:869-872
quote: dispatch_event(DomEvent::new( node_id, DomEventData::Change(BlitzInputEvent { value }), ));
at: packages/blitz-dom/src/events/pointer.rs:903-907
quote: if !was_checked { dispatch_event(DomEvent::new( node_id, DomEventData::Change(BlitzInputEvent { value }), ));
searched: DomEventData::Change · in packages/blitz-vibey-script/src (no synthesis; only the listener name "change" at dom/mod.rs:322)
guard_at: tests/blitz-tests/tests/checkable_change_event.rs:1
@row_add n=418 row=decision/single-select-selectedness area=input-validation rec=fct-43yggwqh@1
name: Single-choice select selectedness
rule: In every document, remote pages included, a select without multiple holds its choice as its options' checked state and settles every recorded select by HTML's selectedness setting algorithm: the last chosen, else the first option not disabled.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
except: An option of a closed single select has no click arm and stays refused covered; a single select's size is not read.
@row_check n=418 row=decision/single-select-selectedness sha=dd20b840d1ac v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select.rs:262-266
quote: pub(crate) fn settled(options: &[OptionReading]) -> Option<usize> { options .iter() .rposition(|option| option.chosen) .or_else(|| options.iter().position(|option| !option.disabled))
at: packages/blitz-dom/src/document.rs:1102
quote: pub(crate) fn settle_select_choice(&mut self, select: NodeId) {
at: packages/blitz-dom/src/mutator.rs:1341
quote: self.select_nodes.insert(node_id);
guard_at: tests/blitz-tests/tests/select_interaction.rs:1
@row_add n=419 row=decision/select-keys-closed-match area=input-validation rec=fct-43yggwqh@1,fct-dn4bmcz5@1,fct-cgy5awnj@1
name: Select keys pass a closed match
rule: A focused select's keys, an open list's keys and a multiple select's keys each pass one closed key match, and every key outside the match does nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup
@row_check n=419 row=decision/select-keys-closed-match sha=2b517ec8c2e7 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:112-117
quote: } else if select::is_single_select(element_data) && !element_data.is_disabled() && let KeyboardOrTextInputEvent::KeyPress(key_event) = &event && key_event.state.is_pressed() { press_select_key(doc, node_id, key_event, &mut dispatch_event);
at: packages/blitz-dom/src/events/keyboard.rs:245
quote: _ => None,
at: packages/blitz-dom/src/events/keyboard.rs:311
quote: _ => None,
guard_at: tests/blitz-tests/tests/select_interaction.rs:1
@row_add n=420 row=decision/select-typeahead-private area=input-validation rec=fct-43yggwqh@1
name: Select typeahead kept private and short-lived
rule: Select typeahead keeps the typed text in the document's private buffer, starts afresh for another select or after 1 s of the document's clock, and the text is compared only, never kept past 1 s and never carried out.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
@row_check n=420 row=decision/select-typeahead-private sha=281af220242a v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:380-381
quote: if typeahead.select == select && now.saturating_sub(typeahead.at) < TYPEAHEAD_TIMEOUT =>
at: packages/blitz-dom/src/node/select.rs:54
quote: pub(crate) const TYPEAHEAD_TIMEOUT: Duration = Duration::from_secs(1);
at: packages/blitz-dom/src/node/select.rs:303
quote: .position(|option| !option.disabled && option.label.to_lowercase().starts_with(&text))
guard_at: tests/blitz-tests/tests/select_interaction.rs:1
@row_add n=421 row=decision/select-choice-one-write-path area=input-validation rec=fct-43yggwqh@1,fct-dn4bmcz5@1,fct-cgy5awnj@1
name: Every select choice through one write path
rule: Every select choice, by key, click or open list, goes through one write path that marks the changed set and dispatches input then a non-cancelable, bubbling change, a choice of the already chosen option firing nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup
@row_check n=421 row=decision/select-choice-one-write-path sha=ed78cc0ab367 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:480-482
quote: if !self.set_select_choice(&options, chosen) { return; }
at: packages/blitz-dom/src/events/keyboard.rs:485-494
quote: dispatch_event(DomEvent::new( select, DomEventData::Input(BlitzInputEvent { value: value.clone(), }), )); dispatch_event(DomEvent::new( select, DomEventData::Change(BlitzInputEvent { value }), ));
at: packages/blitz-dom/src/events/pointer.rs:654
quote: doc.apply_select_choice(select, &[row], dispatch_event);
guard_at: tests/blitz-tests/tests/select_popup.rs:1
@row_add n=422 row=decision/select-choice-exits area=input-validation rec=fct-43yggwqh@1
name: Where a select's choice leaves
rule: A select's choice leaves only as form submission's entries, the input and change events' value, the snapshot's value and the options' accessibility-tree nodes, inside the ratified name and control-value crossings.
why: A boundary widening, ratified.
@row_check n=422 row=decision/select-choice-exits sha=31e55321a64c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:281
quote: if select::is_chosen(option) && !select::option_disabled(option) {
at: packages/blitz-dom/src/events/keyboard.rs:483
quote: let value = self.select_value(select).unwrap_or_default();
at: packages/dioxus-native-dom/src/snapshot.rs:280
quote: return doc.select_value(node_id);
at: packages/blitz-dom/src/accessibility.rs:186-187
quote: if select::is_select_option(node) { builder.set_selected(select::is_chosen(node));
@row_add n=423 row=decision/select-list-live-reader area=input-validation rec=fct-p63xuhfj@1
name: Open select list read through one live reader
rule: The open select list is one document field, read through one live-list reader everywhere but reconciliation, the removal arm and row hover, which read the field directly.
@row_check n=423 row=decision/select-list-live-reader sha=029b90837f65 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:810-811
quote: pub(crate) fn live_select_list(&self) -> Option<&select_list::SelectList> { let list = self.select_list.as_ref()?;
at: packages/blitz-dom/src/document.rs:905
quote: let Some(list) = self.select_list else {
at: packages/blitz-dom/src/events/pointer.rs:604
quote: match doc.select_list.as_mut() {
@row_add n=424 row=decision/select-list-tab-chooses-and-closes area=input-validation rec=fct-dn4bmcz5@1
name: Tab chooses and closes an open select list
rule: On an open select list, Tab chooses the current row and closes the list before focus moves, in either direction.
why: An engine change for every document and a boundary widening, ratified.
guard: tab_chooses_and_closes_before_focus_moves_either_way
@row_check n=424 row=decision/select-list-tab-chooses-and-closes sha=1e9037d733ed v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:31-34
quote: if event.key == Key::Named(NamedKey::Tab) { if let Some(select) = doc.open_select() { choose_current_and_close(doc, select, &mut dispatch_event); }
guard_at: tests/blitz-tests/tests/select_popup.rs:330
@row_add n=425 row=decision/select-list-pointer-release-chooses area=input-validation rec=fct-5eqwsk5u@1
name: Pointer in an open select list
rule: In an open select list a press starts no text drag and the release chooses the row under it.
why: An engine change for every document and a boundary widening, ratified.
guard: a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends
@row_check n=425 row=decision/select-list-pointer-release-chooses sha=27ee62719a85 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:416-417
quote: if doc.select_list_hit(x, y).is_some() { return;
at: packages/blitz-dom/src/events/pointer.rs:972-975
quote: if let Some(select) = doc.open_select() && click_lands_on_row(doc, node_id, event) { click_list_row(doc, select, node_id, dispatch_event);
guard_at: tests/blitz-tests/tests/select_popup.rs:780
@row_add n=426 row=decision/select-list-closing area=input-validation rec=fct-5eqwsk5u@1
name: What closes an open select list
rule: An open select list closes on a press outside it, a page scroll or a move of its box, while a list a click just opened survives a focus rule's restyle.
why: An engine change for every document and a boundary widening, ratified.
guard: escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event
@row_check n=426 row=decision/select-list-closing sha=9384e5460dee v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:419
quote: close_list_outside(doc, x, y);
at: packages/blitz-dom/src/document.rs:915-916
quote: if self.live_select_list().is_none() { self.close_select_list();
at: packages/blitz-dom/src/document.rs:908
quote: if list.fresh && self.select_list_allowed(list.select) {
guard_at: tests/blitz-tests/tests/select_popup.rs:402
@row_add n=427 row=decision/select-list-hostile-markup-finite area=input-validation rec=fct-iqeqvhsa@1
name: Hostile markup under an open select list
rule: A hostile size, thousands of options and very long labels under an open select list keep every box finite and panic nowhere.
why: An engine change for every document and a boundary widening, ratified.
guard: hostile_markup_keeps_every_box_finite_and_panics_nowhere
@row_check n=427 row=decision/select-list-hostile-markup-finite sha=a15a1d543606 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/select_popup.rs:911-914
quote: let options: String = (0..2000) .map(|row| format!(r#"<option value="{row}">Row {row}</option>"#)) .collect(); let label = "x".repeat(5500);
at: packages/blitz-dom/src/node/select_list.rs:241
quote: let wanted = row_count.min(MAX_ROWS);
guard_at: tests/blitz-tests/tests/select_popup.rs:935
@row_add n=428 row=decision/select-list-twenty-rows area=input-validation rec=fct-iqeqvhsa@1
name: Open select list shows at most twenty rows
rule: An open select list shows at most 20 rows, those past that scrolled.
why: An engine change for every document and a boundary widening, ratified.
guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
@row_check n=428 row=decision/select-list-twenty-rows sha=67de79bdbb42 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select_list.rs:15
quote: pub(crate) const MAX_ROWS: usize = 20;
at: packages/blitz-dom/src/node/select_list.rs:241
quote: let wanted = row_count.min(MAX_ROWS);
guard_at: tests/blitz-tests/tests/select_popup.rs:867
@row_add n=429 row=decision/multiple-select-keys-and-clicks area=input-validation rec=fct-cgy5awnj@1
name: Multiple select keys and clicks
rule: In a multiple select, a plain arrow, Home, End or click chooses that row alone, Shift spans from the anchor, Ctrl or Meta moves the current row by key and flips a row by click, and disabled rows are never chosen.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
except: Ctrl+A is not modelled.
@row_check n=429 row=decision/multiple-select-keys-and-clicks sha=74685b614417 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:320-326
quote: if modifiers.intersects(Modifiers::CONTROL | Modifiers::META) { RowGesture::Move(row) } else if modifiers.contains(Modifiers::SHIFT) { RowGesture::Span(row) } else { RowGesture::Alone(row) }
at: packages/blitz-dom/src/events/pointer.rs:1158
quote: let disabled = doc.get_node(row).is_some_and(select::option_disabled);
guard_at: tests/blitz-tests/tests/select_interaction.rs:1
@row_add n=430 row=decision/optgroup-options-and-label area=input-validation rec=fct-63uvlsf6@1
name: Optgroup options and label
rule: A select's options include its optgroup children's options in tree order, and a group's non-empty label is its name in the accessibility tree and a header painted in an open list.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup, select_popup_painted
@row_check n=430 row=decision/optgroup-options-and-label sha=8a0210c8b771 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select.rs:153-157
quote: } else if child_node .data .is_element_with_tag_name(&local_name!("optgroup")) { list.extend(child_node.children.iter().copied().filter(|grandchild| {
at: packages/blitz-dom/src/accessibility.rs:164-170
quote: } else if (select::is_select_option(node) || element_data.name.local == local_name!("optgroup")) && let Some(label) = element_data .attr(local_name!("label")) .filter(|label| !label.is_empty()) { builder.set_label(label);
at: packages/blitz-paint/src/render/select_list.rs:124
quote: synthesize_bold: true,
guard_at: tests/blitz-tests/tests/select_popup_painted.rs:1
@row_add n=431 row=decision/option-disabled-one-rule area=input-validation rec=fct-63uvlsf6@1
name: One option-disabled rule
rule: Whether an option is disabled is decided by one rule that the readings and form submission share.
why: An engine change for every document, ratified.
@row_check n=431 row=decision/option-disabled-one-rule sha=e7b0a9b61969 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:281
quote: if select::is_chosen(option) && !select::option_disabled(option) {
at: packages/blitz-dom/src/node/select.rs:213
quote: disabled: option_disabled(node),
@row_add n=432 row=decision/multiple-select-set-exits area=input-validation rec=fct-2gvxsqoe@1
name: Where a multiple select's chosen set leaves
rule: A multiple select's chosen set leaves only as form entries, as input and change values carrying one entry per chosen option, as the snapshot's value of the first chosen option in tree order, and as the options' tree nodes.
why: An engine change for every document and a boundary widening, ratified.
@row_check n=432 row=decision/multiple-select-set-exits sha=ec0a58665019 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:44-47
quote: doc.select_values(target) .into_iter() .map(|value| (name.to_string(), FormValue::Text(value))) .collect()
at: packages/blitz-dom/src/document.rs:744
quote: select::first_chosen_option(self, select)?
at: packages/blitz-dom/src/accessibility.rs:175
quote: builder.set_multiselectable();
@row_add n=433 row=decision/driver-type-on-select area=input-validation rec=fct-d4potb3l@1,fct-43yggwqh@1
name: Driver type on a select
rule: The driver's type on any select focuses it with no click and no DOM focus events, clears the typeahead, closes any list, and a text opening on a space types nothing.
why: A click on a multiple select would land on the centre row and a Space would flip a row; a boundary widening, ratified.
@row_check n=433 row=decision/driver-type-on-select sha=3d0ee8ea545d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:649-651
quote: doc.close_select_list(); doc.clear_select_typeahead(); doc.set_focus_to(select);
at: packages/escher-driver/src/execute.rs:253
quote: if !(select && text.starts_with(' ')) {
@row_add n=434 row=decision/select-list-label-never-logged area=input-validation rec=fct-t4jwjgu4@1
name: Select list row labels never logged
rule: The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
@row_check n=434 row=decision/select-list-label-never-logged sha=1d691002cd4f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:894
quote: pub fn close_select_list(&mut self) {
at: packages/blitz-dom/src/document.rs:1035
quote: pub fn select_list_view(&self) -> Option<SelectListView> {
searched: println!|eprintln!|tracing:: · in packages/blitz-dom/src/{document.rs,node/select_list.rs,node/select.rs,events/keyboard.rs} (only unrelated resource/focus-node sites, none on a row label)
@row_add n=435 row=decision/password-file-value-masked area=input-validation rec=fct-3l4zlejk@1
name: Password and file input values masked
rule: The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path.
why: Ratified.
guard: stand_act_filled, stand_mcp_masked
except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
@row_check n=435 row=decision/password-file-value-masked sha=c42bef980f99 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:252-253
quote: && element.attr(local_name!("type")).is_some_and(|input_type| { input_type.eq_ignore_ascii_case("password") || input_type.eq_ignore_ascii_case("file")
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
guard_at: tests/blitz-tests/tests/stand_act_filled.rs:1
@row_add n=436 row=contract/snapshot-value/masked-value area=input-validation rec=fct-3l4zlejk@1
boundary: The snapshot's value
item: MASKED_VALUE
promise: Eight U+2022 characters, one fixed marker standing for a password or file input's non-empty value on every carrier.
@row_check n=436 row=contract/snapshot-value/masked-value sha=6a10293209ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:13
quote: pub const MASKED_VALUE: &str = "••••••••";
@row_add n=437 row=decision/mask-holds-on-every-carrier area=input-validation rec=fct-3l4zlejk@1
name: Value mask holds on every carrier
rule: A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results.
why: Ratified.
guard: stand_act_filled, stand_mcp_masked
@row_check n=437 row=decision/mask-holds-on-every-carrier sha=4c294e9c5824 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=438 row=decision/password-painted-masked area=input-validation rec=fct-3l4zlejk@1
name: Password input painted masked
rule: A password input is painted as one bullet per character in every document and through every renderer that paints with the engine's painter, so a typed password never leaves as pixels in a screenshot.
why: An engine change for every document, ratified.
guard: password_painted_masked, stand_act_screenshot
except: The frame depends on the value's length; a file input's paint is unchanged.
@row_check n=438 row=decision/password-painted-masked sha=6a9e84e28eed v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:954
quote: if self.element.attr(local_name!("type")) == Some("password") {
guard_at: tests/blitz-tests/tests/password_painted_masked.rs:1
@row_add n=439 row=decision/password-caret-placed-by-bullets area=input-validation rec=fct-3l4zlejk@1
name: Password caret placed by the bullets
rule: A click inside a password field places the caret by the painted bullets, so no width of the real characters reaches the caret.
why: Ratified.
@row_check n=439 row=decision/password-caret-placed-by-bullets sha=45b24d54dcf3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:571-572
quote: if password { click_password_input(
@row_add n=440 row=decision/recorded-type-masked area=input-validation rec=fct-3l4zlejk@1
name: Recorded type to a masked control kept masked
rule: A recorded type whose element resolves to a masked control or to none is kept as a stand-in marked masked, never the text, its length or a digest of it, while any other type's text is kept as it crossed.
why: Ratified.
@row_check n=440 row=decision/recorded-type-masked sha=908ab265f104 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:234
quote: .is_none_or(|(_, target)| target.masked);
at: packages/escher-driver/src/recording.rs:121
quote: arguments.text(arg.name, MASKED_STAND_IN);
at: packages/escher-driver/src/recording.rs:123
quote: ArgValue::Text(text) => arguments.text(arg.name, text),
@row_add n=441 row=contract/recorded-check/masked-stand-in area=input-validation rec=fct-3l4zlejk@1
boundary: A recorded check
item: MASKED_STAND_IN
promise: The text "masked", with masked:true, stands for a recorded type to a masked or unresolved control.
@row_check n=441 row=contract/recorded-check/masked-stand-in sha=f398b8be99a6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:26
quote: pub const MASKED_STAND_IN: &str = "masked";
at: packages/escher-driver/src/recording.rs:120-121
quote: ArgValue::Text(_) if masked && arg.kind == ArgKind::Text => { arguments.text(arg.name, MASKED_STAND_IN);
@row_add n=442 row=decision/inline-box-heights-finite area=input-validation rec=fct-zdncpoo3@1
name: Inline-box heights kept finite
rule: Inline-box heights are kept finite.
why: Huge author lengths cannot stall the line breaker.
@row_check n=442 row=decision/inline-box-heights-finite sha=2b2dc49b1490 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/inline/layout.rs:475
quote: (margin_box_height * scale).min(f32::MAX)
@row_add n=443 row=decision/visible-region-through-transforms area=input-validation rec=fct-issnlebr@1
name: Visible region read through transforms
rule: A clipping box's visible region is read through its transforms, exact under translate and scale, a rotation reading the rotated box's bounding box, and a mapped box with a non-finite edge answers none.
why: An engine change for every document, ratified.
guard: visible_region_transformed, stand_act_transformed, transformed_client_rect
@row_check n=443 row=decision/visible-region-through-transforms sha=9d889eca2deb v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:809-811
quote: if let Some(shown) = self.transformed_bounds(box_id, padding_box) { let shown = shown - kurbo::Vec2::new(scroll.x, scroll.y); return shown.is_finite().then_some(shown);
at: packages/blitz-dom/src/scrolling.rs:779
quote: let shown = self.padding_box_in_view(box_id)?;
guard_at: tests/blitz-tests/tests/visible_region_transformed.rs:1
@row_add n=444 row=decision/visible-region-open-list-row area=input-validation rec=fct-issnlebr@1
name: Visible region of an open select list row
rule: For a row or a group of an open select list, the visible region answers the list's scrollport.
why: An engine change for every document, ratified.
@row_check n=444 row=decision/visible-region-open-list-row sha=c977fcab0e93 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:754
quote: if let Some(region) = self.select_list_region(node_id) {
at: packages/blitz-dom/src/document.rs:1030
quote: Some(select_list::geometry(self, list).scrollport)
@row_add n=445 row=decision/select-list-painted-over-page area=input-validation rec=fct-gkpzlahs@1
name: Select list painted over the page
rule: The open select list is painted after the root's render and before the devtools overlays, a non-finite list box left unpainted.
why: An engine change for every document and a boundary widening, ratified.
@row_check n=445 row=decision/select-list-painted-over-page sha=a8f416e798c7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:248
quote: self.paint_select_list(scene);
at: packages/blitz-paint/src/render/select_list.rs:51
quote: device.is_finite().then_some(device)
@row_add n=446 row=decision/non-invertible-transform-no-hit area=input-validation rec=fct-z6bo7mnt@1
name: Non-invertible transform takes no hit
rule: In every document, the whole subtree of a transform whose determinant is 0 or not finite takes no hit.
why: An engine change for every document, ratified.
guard: hit_non_invertible_transform
@row_check n=446 row=decision/non-invertible-transform-no-hit sha=af5ad9eede2e v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1488-1489
quote: if determinant == 0.0 || !determinant.is_finite() { return None;
guard_at: tests/blitz-tests/tests/hit_non_invertible_transform.rs:1
@row_add n=447 row=decision/border-radii-scaled-down area=input-validation rec=fct-updndazb@1
name: Oversized border radii scaled down
rule: Oversized border radii are scaled down uniformly.
why: So adjacent radii do not overlap.
@row_check n=447 row=decision/border-radii-scaled-down sha=77e7be7bfc27 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/kurbo_css/css_box.rs:74
quote: border_radii *= min_factor
@row_add n=448 row=decision/select-list-scroll-own-offset area=input-validation rec=fct-gvptjp26@1
name: Select list row scrolled into view by its own offset
rule: Scrolling an open select list's row into view writes the list's own offset at once, moving no scroll chain or viewport, the offset staying within its rows.
why: An engine change for every document and a boundary widening, ratified.
guard: a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement
@row_check n=448 row=decision/select-list-scroll-own-offset sha=001d6e2d6277 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:953
quote: list.scroll_y = scroll_y;
at: packages/blitz-dom/src/scrolling.rs:670-671
quote: if self.scroll_select_list_to(node_id) { return;
at: packages/blitz-dom/src/node/select_list.rs:300
quote: scroll_y.clamp(0.0, most)
guard_at: packages/blitz-dom/src/node/select_list.rs:497
@row_add n=449 row=decision/select-list-wheel area=input-validation rec=fct-gvptjp26@1
name: Wheel over and outside an open select list
rule: A wheel over an open select list scrolls the list, and a wheel elsewhere closes the list and then scrolls.
why: An engine change for every document and a boundary widening, ratified.
guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
@row_check n=449 row=decision/select-list-wheel sha=fa4c288a5e57 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1198
quote: doc.scroll_select_list_by(scroll_y);
at: packages/blitz-dom/src/events/pointer.rs:1201
quote: doc.close_select_list();
guard_at: tests/blitz-tests/tests/select_popup.rs:867
@row_add n=450 row=absence/dioxus-native-form-validation area=input-validation rec=fct-3ulwqhxa@1
capability: Form data validation in the Dioxus native bridge: form data always reads valid
@row_check n=450 row=absence/dioxus-native-form-validation sha=2cd521ce8dde v=holds judge=sonnet head=edd0cfaea7f0
searched: todo: actually implement validation here · in packages/dioxus-native-dom/src/events.rs (only the stub `fn valid(&self) -> bool { true }` at 327-329)
searched: fn valid · in packages/dioxus-native-dom/src/events.rs (returns true unconditionally)
@row_add n=451 row=contract/declared-write/target area=input-validation rec=fct-nzg4gdto@1
boundary: A DeclaredWrite of the signal registry
item: target
promise: An element's stable id, or a prefix ending in *.
@row_check n=451 row=contract/declared-write/target sha=883c28c90082 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:40
quote: match self.id.strip_suffix('*') {
@row_add n=452 row=contract/declared-write/event area=input-validation rec=fct-nzg4gdto@1
boundary: A DeclaredWrite of the signal registry
item: event
promise: An event name without the on prefix.
@row_check n=452 row=contract/declared-write/event sha=17b9f4200e7d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:27
quote: /// The handler's event name, without its `on` (`click`, `keydown`, `input`).
@row_add n=453 row=decision/fetch-get-head-only area=input-validation rec=fct-5u3gkmvy@1
name: Script fetch limited to GET and HEAD
rule: A page script's fetch rejects with TypeError any method other than GET or HEAD.
@row_check n=453 row=decision/fetch-get-head-only sha=823bf5df4ae9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:82-83
quote: if (method !== "GET" && method !== "HEAD") { reject(new TypeError("fetch: only GET and HEAD requests are supported"));
@row_add n=454 row=decision/search-params-native-parser-hidden area=input-validation rec=fct-vvmg4w6v@1
name: Native search-params parser hidden from pages
rule: The native search-params parser is deleted from the global object before any page script runs.
why: An engine change for every document, ratified.
guard: url_search_params
@row_check n=454 row=decision/search-params-native-parser-hidden sha=1d65e09d933f v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:4
quote: delete globalThis.__blitz_parse_search_params;
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:18
quote: include_str!("bootstrap/url_search_params.js"),
guard_at: packages/blitz-vibey-script/tests/url_search_params.rs:1
@row_add n=455 row=decision/url-search-params-bootstrap area=input-validation rec=fct-vvmg4w6v@1
name: URLSearchParams through a bootstrap script
rule: URLSearchParams and a URL's live searchParams, href and search reach page script through a bootstrap script, bad calls and receivers throwing TypeError and values passing USV conversion.
why: A realization on the registered script crossing, no boundary widening; ratified.
guard: url_search_params
@row_check n=455 row=decision/url-search-params-bootstrap sha=b5d92ca9e26c v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:17
quote: if (!data) throw new TypeError('Invalid URLSearchParams receiver');
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:50
quote: const input = init === null ? '' : usv(init);
guard_at: packages/blitz-vibey-script/tests/url_search_params.rs:1
@row_add n=456 row=decision/sql-values-bound-by-placeholders area=input-validation rec=fct-27ctq7ii@1
name: SQL values bound by placeholders
rule: Persistence statements bind every value through placeholders.
@row_check n=456 row=decision/sql-values-bound-by-placeholders sha=8174150b10ad v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:235
quote: params![&entry.title, favicon_str, visited_secs, id],
at: apps/browser/persistence/src/lib.rs:242
quote: params![url_str, &entry.title, favicon_str, visited_secs],
@row_add n=457 row=decision/inline-svg-serializes-well-formed area=input-validation rec=fct-oolxy7gn@1
name: Inline SVG serializes as well-formed XML
rule: Serialized inline SVG must be well-formed XML, escaped text and xlink:href still parsing.
guard: inline_svg_serialize
@row_check n=457 row=decision/inline-svg-serializes-well-formed sha=6368bfc3ffa0 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/serialize.rs:20-21
quote: /// Declare the `xlink` prefix on the root element so that the output is well-formed XML declare_xlink_ns: bool,
at: packages/blitz-dom/src/node/serialize.rs:3
quote: use html_escape::{encode_quoted_attribute_to_string, encode_text_to_string};
guard_at: tests/blitz-tests/tests/inline_svg_serialize.rs:41
@row_add n=458 row=decision/wpt-quarantine-entries-valid area=input-validation rec=fct-elu7gey4@1
name: WPT timeout-quarantine entries well-formed
rule: A WPT timeout-quarantine entry must hold a non-empty path without whitespace and a reason among the known reasons, and must not duplicate another.
@row_check n=458 row=decision/wpt-quarantine-entries-valid sha=b7b62a915327 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/mod.rs:33-41
quote: for line in include_str!("../../timeout-quarantine.txt").lines() { let (path, reason) = line .split_once(' ') .expect("timeout quarantine entries must contain a path and reason"); assert!( tests.insert(path, reason).is_none(),
at: wpt/runner/src/test_runners/mod.rs:428-435
quote: assert!( !path.is_empty() && !path.contains(char::is_whitespace), "invalid quarantined test path: {path:?}" ); assert!( KNOWN_REASONS.contains(reason), "unknown quarantine reason {reason:?} for {path} (add it to KNOWN_REASONS if intentional)" );
@row_add n=459 row=decision/cells-grid-keyboard area=input-validation rec=fct-ppqmcyh2@1
name: Cells grid keyboard model
rule: The 7GUIs Cells grid takes focus with tabindex 0 and a closed key match and no role: arrows move the current cell, clamped, Enter opens the editor on it, focused, and the editor's Enter commits, stops propagation and returns focus to the grid.
why: Ratified as to the grid.
@row_check n=459 row=decision/cells-grid-keyboard sha=b84449c3a3da v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:414
quote: tabindex: "0",
at: examples/seven_guis/src/tasks/cells.rs:427
quote: Key::Enter => {
at: examples/seven_guis/src/tasks/cells.rs:510
quote: evt.stop_propagation();
at: examples/seven_guis/src/tasks/cells.rs:516
quote: drop(grid.set_focus(true));
@row_add n=460 row=decision/stderr-only-telemetry-sink area=logging-monitoring rec=fct-4hqe6pzr@2
name: Escher's own stderr telemetry sink
rule: Both seven_guis binaries, the native app and the escher-session driver host, install escher's own telemetry sink first thing: a stderr-only subscriber whose formatter applies an allowlist of targets.
except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed, and the chained std panic hook still prints the raw panic message to stderr.
@row_check n=460 row=decision/stderr-only-telemetry-sink sha=38f90fe9fff8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:122
quote: init_with_writer(identity, io::stderr)
at: examples/seven_guis/src/session_host.rs:32
quote: if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
@row_add n=461 row=decision/engine-target-safe-fields area=logging-monitoring rec=fct-4hqe6pzr@2
name: Engine targets print only safe fields
rule: An event whose target starts with blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer or js_console prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line; every other field, the message included, prints as [redacted].
@row_check n=461 row=decision/engine-target-safe-fields sha=4e4ba775d039 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:22-29
quote: pub const ENGINE_TARGET_PREFIXES: &[&str] = &[ "blitz", "dioxus_native", "stylo_taffy", "accesskit_xplat", "debug_timer", "js_console", ];
at: packages/escher-telemetry/src/format.rs:38-46
quote: pub const SAFE_FIELDS: &[&str] = &[ "node_id", "status", "waiting_nodes", "property", "log.module_path", "log.file", "log.line", ];
at: packages/escher-telemetry/src/format.rs:100-105
quote: } else if is_engine_target(target) { if SAFE_FIELDS.contains(&field) { Verdict::Print } else { Verdict::Redact }
@row_add n=462 row=decision/escher-target-content-scrub area=logging-monitoring rec=fct-4hqe6pzr@2
name: Escher targets scrub content-named fields
rule: An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which print as [redacted].
@row_check n=462 row=decision/escher-target-content-scrub sha=7063f0014751 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:49
quote: pub const CONTENT_FIELDS: &[&str] = &[
at: packages/escher-telemetry/src/format.rs:59-60
quote: "error", "panic.payload",
at: packages/escher-telemetry/src/format.rs:106-107
quote: } else if CONTENT_FIELDS.contains(&field) { Verdict::Redact
@row_add n=463 row=decision/outside-target-dropped area=logging-monitoring rec=fct-4hqe6pzr@2
name: Outside targets dropped whole
rule: An event or bridged log record from a target outside the allowlist is not printed at all, zero bytes, at every level including WARN and ERROR, whatever RUST_LOG names.
why: The named cost: a third-party WARN or ERROR no longer prints either.
@row_check n=463 row=decision/outside-target-dropped sha=782dce10d6de v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:146-148
quote: if is_outside_target(metadata.target()) { return Ok(()); }
at: packages/escher-telemetry/src/format.rs:144
quote: let normalized = event.normalized_metadata();
@row_add n=464 row=decision/closed-span-lines area=logging-monitoring rec=fct-4hqe6pzr@2
name: One sink line per closed span
rule: The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored.
why: A boundary widening of the sink, ratified as built.
@row_check n=464 row=decision/closed-span-lines sha=912190a91278 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:111
quote: .with_span_events(FmtSpan::CLOSE)
at: packages/escher-telemetry/src/format.rs:146-148
quote: if is_outside_target(metadata.target()) { return Ok(()); }
at: packages/escher-telemetry/src/format.rs:235-237
quote: if !CONTENT_FIELDS.contains(&name) { spell(&mut self.0, value); }
@row_add n=465 row=contract/tool-call-span/line area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The tool_call span line
item: line
promise: One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
@row_check n=465 row=contract/tool-call-span/line sha=6f74416f594d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
@row_add n=466 row=contract/tool-call-span/tool area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The tool_call span line
item: tool
promise: The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.
@row_check n=466 row=contract/tool-call-span/tool sha=2c5c9b2006b0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:192
quote: span.record("tool", verb.name);
@row_add n=467 row=contract/tool-call-span/ending area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The tool_call span line
item: ending
promise: One of the closed words accepted, refused or error.
@row_check n=467 row=contract/tool-call-span/ending sha=4cbe2d1a94ae v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:22-24
quote: Ending::Accepted => "accepted", Ending::Refused => "refused", Ending::Error => "error",
@row_add n=468 row=contract/command-span/line area=logging-monitoring rec=fct-4hqe6pzr@2
boundary: The command span line
item: line
promise: A sink-installing host writes one closed command span line per call it runs.
@row_check n=468 row=contract/command-span/line sha=12e66981aa53 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:181
quote: let span = command_span();
@row_add n=469 row=contract/command-span/cause area=logging-monitoring rec=fct-4hqe6pzr@2,fct-5agrcrof@1
boundary: The command span line
item: cause
promise: Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
@row_check n=469 row=contract/command-span/cause sha=abb01e8a0e70 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
at: packages/escher-driver/src/refusal.rs:58
quote: Cause::FontsUnavailable => "fonts-unavailable",
@row_add n=470 row=decision/no-screen-content-in-logs area=logging-monitoring rec=fct-4hqe6pzr@2,fct-llvjoueu@1,fct-5agrcrof@1,fct-xenxqlq3@1
name: Screen content stays out of logs
rule: No driver call puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer and a coverage check reach no log line at any level.
guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
@row_check n=470 row=decision/no-screen-content-in-logs sha=17d418c6c4f4 v=holds guard=true judge=sonnet head=edd0cfaea7f0
searched: info_span!|debug_span!|warn_span!|error_span!|tracing::(info|warn|error|debug|trace)! · in packages/escher-driver/src
searched: command_span!(verb, cause, settled, busy, passes, added, removed, changed) · in packages/escher-driver/src/execute.rs (only closed-word and count fields; tool_call span holds tool, ending only, mcp.rs:120)
guard_at: examples/seven_guis/tests/host_log.rs:141
@row_add n=471 row=contract/escher-session-output/esc-byte area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: escher-session output
item: ESC byte
promise: No output line holds an ESC byte; output is uncoloured whatever the stream is.
@row_check n=471 row=contract/escher-session-output/esc-byte sha=095b630d4c56 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:30-31
quote: control if control < '\u{20}' => { let _ = write!(out, "\\u{:04x}", u32::from(control));
@row_add n=472 row=contract/escher-session-output/accepted-stderr area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: escher-session output
item: stderr of an accepted command
promise: An accepted command's stderr is empty with RUST_LOG unset.
@row_check n=472 row=contract/escher-session-output/accepted-stderr sha=d85eca0f9c6e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:340-342
quote: Ending::Accepted(answer) => Written { stdout: Some(answer.clone()), stderr: None,
@row_add n=473 row=contract/escher-session-mcp-stdout/protocol-lines area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: The escher-session mcp role's stdout
item: protocol lines
promise: The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.
@row_check n=473 row=contract/escher-session-mcp-stdout/protocol-lines sha=0f165617f527 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:197
quote: pub(super) fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> {
at: packages/escher-driver/src/cli.rs:360-362
quote: Ending::Served | Ending::Spoken(Ok(())) => Written { stdout: None, stderr: None,
@row_add n=474 row=contract/escher-session-coverage-line/content area=logging-monitoring rec=fct-uh4dhzjw@2
boundary: The escher-session coverage role's stdout line
item: content
promise: The coverage line holds ids, owner ids, event names and two counts, and nothing of a named path.
@row_check n=474 row=contract/escher-session-coverage-line/content sha=80034a295763 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:429
quote: entry.text("id", &handler.id);
at: packages/escher-driver/src/json.rs:433
quote: entry.text("event", &handler.event);
at: packages/escher-driver/src/json.rs:437-438
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX)); object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
@row_add n=475 row=contract/command-span/verb area=logging-monitoring rec=fct-llvjoueu@1,fct-5agrcrof@1
boundary: The command span line
item: verb
promise: Records the instance-level verb; a graph or actions call records the verb alone, plus cause when refused.
@row_check n=475 row=contract/command-span/verb sha=55d7435c7e3b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:184
quote: span.record("verb", row.name);
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
@row_add n=476 row=decision/cold-agent-logs-metadata-only area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent pipe logs run metadata only
rule: The cold-agent pipe's events and verdict carry counts, booleans and closed words only: no content-named field and no transcript text, tool argument, tool answer, reply or check.
guard: The scrub-set key check and four planted markers, under the shim
@row_check n=476 row=decision/cold-agent-logs-metadata-only sha=30de0a97fa46 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:40-41
quote: # Every event and the verdict are encoded by python's json module. No transcript text, tool argument, tool result, # final reply or check step is copied into either: the verdict carries counts, booleans and closed words only.
guard_at: .github/scripts/test_cold_agent.py:641
@row_add n=477 row=decision/cold-agent-transcript-raw area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent raw transcript kept unscrubbed
rule: The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.
@row_check n=477 row=decision/cold-agent-transcript-raw sha=dd920e4b683f v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: hygiene · in .github/scripts
searched: masked · in .github/scripts
why: no evidence found for the masking of committed copies; transcript.jsonl is written raw (scripts/cold-agent.sh:585) under gitignored target/ (.gitignore:23) but no masking code or hygiene gate found
@row_add n=478 row=decision/cold-agent-server-output-unprinted area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent server output never printed
rule: The cold-agent pipe reads its server's stderr log only by counting span lines and writes the agent's check as parsed, printing or copying neither; the replay children's stderr goes to /dev/null.
@row_check n=478 row=decision/cold-agent-server-output-unprinted sha=ff77644af1ff v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:302
quote: stderr=subprocess.DEVNULL, text=True, encoding="utf-8", errors="replace")
at: scripts/cold-agent.sh:504
quote: server = 'exec env RUST_LOG=info "$1" mcp --record "$3" 2>"$2"'
@row_add n=479 row=decision/cold-agent-recordings-owner-only area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent recordings owner-only
rule: The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.
@row_check n=479 row=decision/cold-agent-recordings-owner-only sha=50a0de1964bb v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:571
quote: mkdir -m 700 "$STATE_DIR/record"
at: scripts/cold-agent.sh:301
quote: self.proc = subprocess.Popen([binary, "mcp"], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
@row_add n=480 row=decision/cold-agent-wrong-call-bar area=logging-monitoring rec=fct-tqdgjkao@1
name: Cold-agent wrong-call bar
rule: The cold-agent verdict decides its wrong calls against a 5 % bar, failing with the reason wrong-calls-over-bar above it.
@row_check n=480 row=decision/cold-agent-wrong-call-bar sha=e1b8d282ba6d v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:487-488
quote: if wrong_calls * 100 > WRONG_CALL_BAR_PCT * v["tool_calls"]: reasons.append("wrong-calls-over-bar")
@row_add n=481 row=decision/ci-secrets-in-actions area=secret-management rec=fct-gwmzm7pv@1
name: CI secrets in GitHub Actions secrets and vars
rule: CI secrets live in GitHub Actions secrets and vars: signing keys, the keystore and its passwords, the Apple certificate, the cross-repo dispatch token and the workflow token are read from them.
@row_check n=481 row=decision/ci-secrets-in-actions sha=eb5ae4f5809e v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:108
quote: run: echo "${{ secrets.APPLE_API_KEY_CONTENT }}" > ${{ vars.APPLE_API_KEY_PATH }}
at: .github/workflows/publish-browser.yml:114
quote: base64 -d <<< "${{ secrets.ANDROID_KEYSTORE_BASE64 }}" > keystore.jks
at: .github/workflows/publish-browser.yml:118
quote: echo 'jks_password = "${{ secrets.ANDROID_KEY_PASSWORD }}"' >> Dioxus.toml
at: .github/workflows/publish-browser.yml:169
quote: APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
at: .github/workflows/wpt.yml:111
quote: token: ${{ secrets.WPT_GITHUB_TOKEN }}
at: .github/workflows/wpt-post-results.yml:28
quote: github-token: ${{ secrets.GITHUB_TOKEN }}
@row_add n=482 row=decision/ci-signing-material-removed area=secret-management rec=fct-gwmzm7pv@1
name: CI signing material removed after use
rule: A signing key or keystore that CI writes to disk from a secret is removed in an always() step.
@row_check n=482 row=decision/ci-signing-material-removed sha=25f9ace68ff8 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:180-181
quote: if: ${{ always() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/ci-test')) && startsWith(matrix.platform.os, 'macos') }} run: rm ${{ vars.APPLE_API_KEY_PATH }}
at: .github/workflows/publish-browser.yml:184-185
quote: if: ${{ always() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/ci-test')) && matrix.platform.target == 'aarch64-linux-android' }} run: rm ./apps/browser/keystore.jks
@row_add n=483 row=decision/iframe-nesting-depth-limit area=threat-model-summary rec=fct-utdigfl4@1
name: Iframe nesting depth limit
rule: Iframe nesting depth is limited.
why: Guards against infinitely recursive self-embedding pages.
@row_check n=483 row=decision/iframe-nesting-depth-limit sha=e5a6e2e574fe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1490
quote: if self.doc.subdocument_depth >= crate::iframe::MAX_SUBDOCUMENT_DEPTH {
@row_add n=484 row=decision/import-nesting-depth-limit area=threat-model-summary rec=fct-utdigfl4@1
name: CSS @import nesting depth limit
rule: CSS @import nesting depth is limited.
why: Prevents unbounded recursion, such as an import URL that grows geometrically per level.
@row_check n=484 row=decision/import-nesting-depth-limit sha=1218f94a38e9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:209
quote: if self.import_depth >= MAX_IMPORT_DEPTH || !supports.as_ref().is_none_or(|s| s.enabled) {
@row_add n=485 row=decision/stale-iframe-responses-discarded area=threat-model-summary rec=fct-utdigfl4@1
name: Stale iframe responses discarded
rule: An iframe response whose iframe was removed or re-navigated since the request is discarded by request id.
@row_check n=485 row=decision/stale-iframe-responses-discarded sha=9b242ba18e37 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/iframe.rs:164-170
quote: let is_current = self .iframe_loads .get(&node_id) .is_some_and(|load| load.request_id == Some(request_id)); if !is_current { return; }
@row_add n=486 row=decision/skip-snapshot-of-unstyled-node area=threat-model-summary rec=fct-utdigfl4@1
name: Never-styled nodes not snapshotted
rule: Snapshotting a never-styled node is skipped.
why: Stylo invalidation would otherwise panic.
@row_check n=486 row=decision/skip-snapshot-of-unstyled-node sha=e51d0bcbb81c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2007-2010
quote: let has_been_styled = node.primary_styles().is_some(); if !has_been_styled { return; }
@row_add n=487 row=decision/script-fetcher-file-data-only area=threat-model-summary rec=fct-qfjr2qtu@1
name: Script fetcher takes file and data URLs only
rule: The default script fetcher supports only file: and data: URLs; any other scheme returns UnsupportedScheme.
@row_check n=487 row=decision/script-fetcher-file-data-only sha=2f4cca2ac6e1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:68
quote: scheme => Err(FetchError::UnsupportedScheme(scheme.to_string())),
@row_add n=488 row=decision/file-url-only-for-file-documents area=threat-model-summary rec=fct-qfjr2qtu@1
name: File URLs load only for file documents
rule: A file: URL loads only for a file: document; for any other document, remote pages included, JS fetch, module import and script src alike refuse it with a FetchError.
@row_check n=488 row=decision/file-url-only-for-file-documents sha=44141e7d7fcc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:44-45
quote: url.scheme() != "file" || document_url.is_some_and(|document_url| document_url.scheme() == "file")
at: packages/blitz-vibey-script/src/runtime/fetch.rs:43
quote: Err(FetchError::Forbidden(url.clone()))
at: packages/blitz-vibey-script/src/runtime/modules.rs:80
quote: Err(FetchError::Forbidden(url.clone()))
at: packages/blitz-vibey-script/src/document.rs:166
quote: FetchError::Forbidden(url.clone())
@row_add n=489 row=decision/browser-javascript-opt-in area=threat-model-summary rec=fct-qfjr2qtu@1
name: Browser JavaScript opt-in
rule: In the browser app JavaScript execution is opt-in, behind the non-default javascript feature.
@row_check n=489 row=decision/browser-javascript-opt-in sha=238c5e92c338 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:36
quote: javascript = ["dep:blitz-vibey-script"]
at: apps/browser/Cargo.toml:14
quote: default = ["hybrid", "cookies", "cache", "screenshot", "apple-font-embolden", "scrollbars", "mimalloc"]
@row_add n=490 row=decision/bootstrap-native-helper-removed area=threat-model-summary rec=fct-qfjr2qtu@1
name: Bootstrap native helper hidden from pages
rule: The URLSearchParams bootstrap's native search-params helper is deleted from globalThis before any page script runs.
@row_check n=490 row=decision/bootstrap-native-helper-removed sha=611a980a3341 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:4
quote: delete globalThis.__blitz_parse_search_params;
@row_add n=491 row=decision/session-socket-carries-calls area=threat-model-summary rec=fct-ljkdert4@2
name: Session socket carries calls and answers
rule: The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses.
why: A boundary widening, ratified.
@row_check n=491 row=decision/session-socket-carries-calls sha=7ea361a1f20b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
at: packages/escher-driver/src/json.rs:145
quote: object.text("id", parts.id);
@row_add n=492 row=decision/snapshot-state-readings area=threat-model-summary rec=fct-ljkdert4@2
name: Three state readings per control
rule: The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, as closed words and booleans.
why: Inside the ratified crossing, ratified.
@row_check n=492 row=decision/snapshot-state-readings sha=686fa4a0b460 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:160-166
quote: if let Some(selected) = parts.selected { object.flag("selected", selected); } if let Some(pressed) = parts.pressed { object.flag("pressed", pressed); } object.flag("invalid", parts.invalid);
@row_add n=493 row=decision/lint-findings-crossing area=threat-model-summary rec=fct-ljkdert4@2,fct-gphdtjgv@1
name: Lint findings cross every ratified carrier
rule: A lint's findings cross every ratified carrier as stable ids, closed defect words and numbers, never page text, colour or value.
@row_check n=493 row=decision/lint-findings-crossing sha=f99dee3d66b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:500
quote: object.json("findings", &list(findings.iter().map(finding)));
@row_add n=494 row=decision/open-list-shot-pixels area=threat-model-summary rec=fct-ljkdert4@2,fct-gphdtjgv@1
name: Open select list shot carries labels as pixels
rule: A shot of an open select list carries the page's option labels and group labels as pixels, on the socket and MCP carriers alike, the shot's fields unmoved.
why: A boundary widening, ratified.
guard: cli_commands, pinning a hosted shot equal to an in-process one
@row_check n=494 row=decision/open-list-shot-pixels sha=d3b31f183249 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render/select_list.rs:100
quote: let Some(layout) = self.dom.select_row_layout(row.node, &row.label) else {
at: packages/escher-driver/src/mcp/result.rs:90
quote: block.text("type", "image");
guard_at: examples/seven_guis/tests/cli_commands.rs:426
@row_add n=495 row=decision/select-list-crossing area=threat-model-summary rec=fct-ljkdert4@2,fct-gphdtjgv@1
name: Select list crossing limited to bounds and pixels
rule: The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff and as pixels in a shot; node state holds no field of it and no expanded node key stands.
@row_check n=495 row=decision/select-list-crossing sha=d8c7b91dffa8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:51
quote: pub const NODE_FIELDS: [NodeFieldSpec; 14] = [
@row_add n=496 row=decision/graph-answer-crossing area=threat-model-summary rec=fct-fcig2nza@1,fct-algpllm7@1
name: Graph answer crossing
rule: A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, never a control value, typed text or NodeId.
why: A boundary widening, ratified.
@row_check n=496 row=decision/graph-answer-crossing sha=f5e5677ddc7e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:505
quote: object.json("nodes", &list(graph.nodes.iter().map(graph_node)));
at: packages/dioxus-native-dom/src/graph.rs:85
quote: Self::Undeclared => "undeclared",
@row_add n=497 row=decision/actions-answer-crossing area=threat-model-summary rec=fct-nvvkgxt4@1,fct-pxfsuahu@1
name: Actions answer crossing
rule: An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId.
why: A boundary widening, ratified.
@row_check n=497 row=decision/actions-answer-crossing sha=2a5cf9d54df7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:511
quote: object.json("nodes", &list(nodes.iter().map(action_node)));
@row_add n=498 row=contract/actions-answer/id area=threat-model-summary rec=fct-nvvkgxt4@1
boundary: The actions answer
item: id
promise: The node's id.
@row_check n=498 row=contract/actions-answer/id sha=d9f9307d2103 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:255
quote: object.text("id", &node.id);
@row_add n=499 row=contract/actions-answer/owner area=threat-model-summary rec=fct-nvvkgxt4@1
boundary: The actions answer
item: owner
promise: The node's owner id.
@row_check n=499 row=contract/actions-answer/owner sha=8a32c3b575db v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:256-257
quote: if let Some(owner) = &node.owner { object.text("owner", owner);
@row_add n=500 row=contract/actions-answer/verbs area=threat-model-summary rec=fct-nvvkgxt4@1
boundary: The actions answer
item: verbs
promise: A list of entries each holding verb, a word of the verb table, and cause, a closed cause word.
@row_check n=500 row=contract/actions-answer/verbs sha=0047298b04d0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:264
quote: entry.text("verb", reading.verb);
at: packages/escher-driver/src/json.rs:266
quote: entry.text("cause", cause.name());
@row_add n=501 row=decision/coverage-check-files-untrusted area=threat-model-summary rec=fct-cz244xre@1
name: Kept check files read as untrusted
rule: The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.
@row_check n=501 row=decision/coverage-check-files-untrusted sha=2e033406829c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:498
quote: let bound = u64::try_from(MAX_CHECK_BYTES).ok()?.saturating_add(1);
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
@row_add n=502 row=decision/coverage-answer-crossing area=threat-model-summary rec=fct-cz244xre@1
name: Coverage answer crossing
rule: The coverage answer line carries ids, owner ids, event names and the checks and held counts only.
why: A boundary widening, ratified.
@row_check n=502 row=decision/coverage-answer-crossing sha=ba2d3e59cd6f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:437-438
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX)); object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
@row_add n=503 row=decision/mcp-stdio-no-auth area=threat-model-summary rec=fct-gphdtjgv@1
name: MCP server over stdio, no authentication
rule: The driver's MCP server runs as a stdio child a client spawns under the invoking user, JSON-RPC 2.0 on stdin and stdout, no listener, port or socket; the spawning client is the trust boundary, with no authentication.
@row_check n=503 row=decision/mcp-stdio-no-auth sha=0fd1e0fed67f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:84-86
quote: pub fn serve_mcp( mut input: impl BufRead, mut output: impl Write,
@row_add n=504 row=decision/mcp-result-crossing area=threat-model-summary rec=fct-gphdtjgv@1
name: MCP result crossing
rule: An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's.
why: A boundary widening, ratified.
@row_check n=504 row=decision/mcp-result-crossing sha=655a378e07e3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
at: packages/escher-driver/src/mcp.rs:61
quote: /// A tool result holds one text block: the screen's text for `snapshot`, the document for
@row_add n=505 row=decision/cold-agent-screen-to-provider area=threat-model-summary rec=fct-gphdtjgv@1
name: Cold-agent runs carry the screen to the provider
rule: A cold-agent live run carries the stand's screen, element ids, accessible names and control values in snapshot texts and diffs, through the claude client to the model provider, and the model's reply returns as an agent-written check the pipe parses and replays.
why: A boundary widening, ratified.
@row_check n=505 row=decision/cold-agent-screen-to-provider sha=484b53c9276c v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:582
quote: timeout 600 claude -p "$STAND_PROMPT" --output-format stream-json --verbose --tools "" \
@row_add n=506 row=decision/capability-statement-mcp-crossing area=threat-model-summary rec=fct-r5teidpw@3
name: Committed statement of the MCP crossing
rule: A committed capability statement tells its readers what an MCP client carries onward: the spawning client as the boundary with no authentication, the carried set, and onward use as the client's; it adds no carrier.
@row_check n=506 row=decision/capability-statement-mcp-crossing sha=99c565cdd963 v=holds judge=sonnet head=edd0cfaea7f0
at: docs/capabilities.md:381
quote: ## What an MCP client carries onward
at: docs/capabilities.md:398
quote: No description crosses any driver carrier, since descriptions go to the platform adapter only.
@row_add n=507 row=decision/ci-token-read-only area=authentication-authorization rec=fct-dnpx5hao@1,fct-iqaxure2@1
name: Read-only CI tokens
rule: CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only.
guard: test_ci_workflows.py
@row_check n=507 row=decision/ci-token-read-only sha=0a831640eadf v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:16-17
quote: permissions: contents: read
guard_at: .github/scripts/test_ci_workflows.py:208
@row_add n=508 row=decision/cold-agent-fixed-prompt area=input-validation rec=fct-xl3lbcjg@1,fct-gmgkgl3p@1
name: Cold-agent task as a fixed prompt
rule: A cold-agent task reaches the agent session only as the fixed prompt, naming no element id and no app slug.
guard: test_the_prompt_names_no_stand_id
@row_check n=508 row=decision/cold-agent-fixed-prompt sha=335a4f4ed9ce v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:19
quote: read -r -d '' STAND_PROMPT <<'PROMPT' || true
guard_at: .github/scripts/test_cold_agent.py:607
@row_add n=509 row=decision/no-panic-on-call-input area=input-validation rec=fct-vehorezx@1,fct-awfxp663@3
name: No panic on call-supplied input
rule: Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
@row_check n=509 row=decision/no-panic-on-call-input sha=653b784ae5ed v=holds judge=sonnet head=edd0cfaea7f0
searched: unwrap\(|expect\(|panic!|unreachable! · in packages/escher-driver/src/execute.rs
searched: unwrap\(|expect\(|panic!|unreachable! · in packages/escher-driver/src/command.rs before line 386 (#[cfg(test)]; only hits are panic! at 879 and 890 inside tests)
@row_add n=510 row=contract/escher-session-exit-codes/0 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 0
promise: The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
@row_check n=510 row=contract/escher-session-exit-codes/0 sha=efafc42f7774 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:340-343
quote: Ending::Accepted(answer) => Written { stdout: Some(answer.clone()), stderr: None, status: ACCEPTED,
at: packages/escher-driver/src/cli.rs:469
quote: handler_coverage(&checks, || boot(&app)).map(|coverage| coverage.to_json()),
@row_add n=511 row=contract/escher-session-exit-codes/1 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 1
promise: The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
@row_check n=511 row=contract/escher-session-exit-codes/1 sha=ae457dad8ce4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:345-348
quote: Ending::Refused(refusal) => Written { stdout: Some(refusal.clone()), stderr: None, status: REFUSED,
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
@row_add n=512 row=contract/escher-session-exit-codes/2 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 2
promise: A usage error: the usage line goes to stderr and stdout stays empty.
@row_check n=512 row=contract/escher-session-exit-codes/2 sha=8a8d05d71218 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:350-353
quote: Ending::Usage => Written { stdout: None, stderr: Some(usage(apps)), status: USAGE,
@row_add n=513 row=contract/escher-session-exit-codes/3 area=input-validation rec=fct-4nuyglo3@1,fct-uh4dhzjw@2
boundary: escher-session exit codes
item: 3
promise: A session error: the session-error object on stdout and the session error's fixed message on stderr.
@row_check n=513 row=contract/escher-session-exit-codes/3 sha=2f0f178c2da0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:355-358
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR,
@row_add n=514 row=decision/kept-checks-owner-only area=data-protection rec=fct-kdtkdrex@1,fct-sw7lpzl2@1,fct-d2yejoup@5
name: Kept checks owner-only and uncommitted
rule: A kept check stays in an owner-only medium, is never logged and is never committed.
except: The two committed checks of the repository's own scripted stand-Counter sessions.
@row_check n=514 row=decision/kept-checks-owner-only sha=13f9c3a10f06 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
at: tests/blitz-tests/checks/counter-cli.json:1
quote: [
@row_add n=515 row=absence/served-network-api area=api-security rec=fct-s4jqcqqv@1,fct-svo6bgmr@3
capability: A served network API: no binary serves a TCP or UDP port
@row_check n=515 row=absence/served-network-api sha=f072c404de8d v=holds judge=sonnet head=edd0cfaea7f0
searched: TcpListener · in *.rs over the repository (only packages/blitz-net/src/lib.rs:526 and :534, in the test module)
searched: UdpSocket · in *.rs over the repository
searched: listen\( · in *.rs over the repository
@row_add n=516 row=absence/content-security-policy area=api-security rec=fct-5zz6ux3p@1,fct-ovh4s4cx@1
capability: CSP enforcement
@row_check n=516 row=absence/content-security-policy sha=a7a8ffdeb6bf v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)content-security|same-origin|\bcors\b · in packages/**/*.rs
searched: (?i)content-security|same-origin · in packages, examples, tests *.rs
@row_add n=517 row=absence/request-timeout area=api-security rec=fct-5zz6ux3p@1,fct-b7bygsx3@1
capability: A request timeout on outbound requests
@row_check n=517 row=absence/request-timeout sha=7b3d1c231598 v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)timeout · in packages/blitz-net/**/*.rs (only a test-side tokio::time::timeout at lib.rs:506, not on requests)
searched: \.timeout\( · in packages, examples, tests *.rs
@row_add n=518 row=decision/session-socket-owner-only area=api-security rec=fct-sn4avmoz@1,fct-t6pddbhw@1,fct-c3aavvn3@1,fct-ljkdert4@2
name: Session socket reachable by the owning user only
rule: Access to the driver session socket is the filesystem's alone: the state directory is 0700, the socket file 0600, a directory open to its group or others is refused, and there is no handshake secret, token or peer-credential check.
@row_check n=518 row=decision/session-socket-owner-only sha=2fb84fec6735 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:92
quote: match DirBuilder::new().mode(0o700).create(state_dir) {
at: packages/escher-driver/src/host.rs:102
quote: Err(SessionError::StateDirNotPrivate)
at: packages/escher-driver/src/host.rs:124
quote: fs::set_permissions(socket, Permissions::from_mode(0o600)).map_err(io_error)?;
@row_add n=519 row=decision/dependency-audit-cargo-deny area=dependency-security rec=fct-cappgruj@1,fct-a4qh6ioi@2
name: Dependency audit with cargo-deny
rule: cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push.
why: Ratified: audit the whole workspace and check licences.
except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
@row_check n=519 row=decision/dependency-audit-cargo-deny sha=5c5eb6f24550 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:40
quote: audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
at: deny.toml:15
quote: all-features = true
at: deny.toml:20
quote: ignore = [
@row_add n=520 row=decision/no-base64-crate area=dependency-security rec=fct-ei3z3lut@1,fct-q4iqo3xv@1
name: No base64 crate
rule: No base64 crate is taken as a dependency; a shot's base64 is written by hand.
@row_check n=520 row=decision/no-base64-crate sha=26d96a576a50 v=holds judge=sonnet head=edd0cfaea7f0
searched: base64 · in Cargo.toml, packages/*/Cargo.toml, apps/*/Cargo.toml, tests/*/Cargo.toml, examples/*/Cargo.toml
searched: base64 · in packages/escher-driver/src (hand-written at image.rs:19, no crate)
@row_add n=521 row=decision/licence-gate area=dependency-security rec=fct-k5ocwtgz@1,fct-n5tq3u6e@1
name: Licence gate on every push
rule: The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences.
why: An open question, decided as the fork's chosen option.
guard: the audit leg
except: upstream's own licenses job stays reverted
@row_check n=521 row=decision/licence-gate sha=636e3ea2a8ee v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: deny.toml:32-35
quote: [licenses] include-dev = true include-build = true confidence-threshold = 0.8
at: .github/workflows/ci.yml:237
quote: - run: bash .github/scripts/ci-leg.sh audit
guard_at: .github/scripts/ci-leg.sh:40
@row_add n=522 row=absence/authentication-authorization area=authentication-authorization rec=fct-iytem5zf@1,fct-xkfvr2q2@1,fct-yi4p2lut@1,fct-56jnxaxp@1,fct-a43rhdr5@1,fct-ayiqfjqx@1,fct-2pewmhrd@1,fct-sa6lfhsv@1,fct-lfns6t7f@1,fct-inooefgh@1,fct-q44pfj6r@1,fct-p5tk7ool@1
capability: Authentication or authorization code
@row_check n=522 row=absence/authentication-authorization sha=853bc48ad721 v=holds judge=sonnet head=edd0cfaea7f0
searched: (?i)authenticat|authoriz|login|credential|x-api-key|api_key · in packages, apps, examples, tests (*.rs)
@row_add n=523 row=decision/live-runs-use-operator-login area=secret-management rec=fct-jtwyhkf2@1,fct-ov4w6i7j@1
name: Live runs use the operator's own login
rule: The cold-agent live run and the MCP live trial reach the model provider only through the operator's own Claude Code login held outside the repository: no API key, no env read by the pipe, no CI secret, no new secret source and no auth library.
why: Ratified.
@row_check n=523 row=decision/live-runs-use-operator-login sha=7d63fd939edb v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:582
quote: timeout 600 claude -p "$STAND_PROMPT" --output-format stream-json --verbose --tools "" \
searched: os\.environ|getenv|environ|ANTHROPIC_ · in scripts/cold-agent.sh (none)
@row_add n=524 row=contract/cli-exit-codes/1 area=error-handling rec=fct-bzel2phc@3,fct-iis2bn6b@1,fct-gucmzkph@3
boundary: escher-session CLI exit codes
item: 1
promise: The call was refused; the refusal JSON is written to stdout.
@row_check n=524 row=contract/cli-exit-codes/1 sha=9bf3ef99481c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:345-348
quote: Ending::Refused(refusal) => Written { stdout: Some(refusal.clone()), stderr: None, status: REFUSED,
@row_add n=525 row=contract/cli-exit-codes/2 area=error-handling rec=fct-iis2bn6b@1,fct-gucmzkph@3
boundary: escher-session CLI exit codes
item: 2
promise: Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
@row_check n=525 row=contract/cli-exit-codes/2 sha=95600d33ef04 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:350-353
quote: Ending::Usage => Written { stdout: None, stderr: Some(usage(apps)), status: USAGE,
@row_add n=526 row=contract/cli-exit-codes/3 area=error-handling rec=fct-iis2bn6b@1,fct-gucmzkph@3
boundary: CLI exit codes
item: 3
promise: Session error: the error object with kind, message and remedy on stdout and its fixed message on stderr; a boot that fails ends with its session error.
@row_check n=526 row=contract/cli-exit-codes/3 sha=f53e522819d2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:355-358
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR,
@row_add n=527 row=decision/screenshot-in-band area=input-validation rec=fct-7jixhrbw@3,fct-ljkdert4@2,fct-gphdtjgv@1
name: Screenshot carried in band
rule: A screenshot's PNG crosses in band, as base64 inside its JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked, under the unchanged answer bound.
why: The in-band image was the ratified choice, a boundary widening.
@row_check n=527 row=decision/screenshot-in-band sha=d9a21178efdd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
at: packages/escher-driver/src/mcp/result.rs:90
quote: block.text("type", "image");
at: packages/blitz-paint/src/render.rs:954
quote: if self.element.attr(local_name!("type")) == Some("password") {
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
@row_add n=528 row=contract/actions-answer/nodes area=input-validation rec=fct-7jixhrbw@3,fct-nvvkgxt4@1
boundary: The actions answer
item: nodes
promise: `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
@row_check n=528 row=contract/actions-answer/nodes sha=1ba12a8830e4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:255
quote: object.text("id", &node.id);
at: packages/escher-driver/src/json.rs:259
quote: object.flag("keyed", node.keyed);
@row_add n=529 row=contract/actions-answer/keyed area=input-validation rec=fct-gczaiqlz@1,fct-nvvkgxt4@1
boundary: The actions answer
item: keyed
promise: A boolean on each listed node; the field is provisional.
@row_check n=529 row=contract/actions-answer/keyed sha=8608ae8c35b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:259
quote: object.flag("keyed", node.keyed);
@row_add n=530 row=decision/actions-keyed-field area=input-validation rec=fct-d2yejoup@5,fct-pxfsuahu@1
name: Keyed flag in the actions answer
rule: The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId.
why: A ratified boundary widening.
state: provisional
@row_check n=530 row=decision/actions-keyed-field sha=05e40d654655 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:255
quote: object.text("id", &node.id);
at: packages/escher-driver/src/json.rs:257
quote: object.text("owner", owner);
at: packages/escher-driver/src/json.rs:259
quote: object.flag("keyed", node.keyed);
at: packages/escher-driver/src/json.rs:264
quote: entry.text("verb", reading.verb);
at: packages/escher-driver/src/json.rs:266
quote: entry.text("cause", cause.name());
@row_none n=1 rec=fct-ljb2gi6k@1 type=S
@row_none n=2 rec=fct-lsqilazd@1 type=5
@row_none n=3 rec=fct-hqa62nne@1 type=4
@row_none n=4 rec=fct-d5xic4hg@1 type=S
@row_none n=5 rec=fct-gmnbkajs@1 type=7
@row_none n=6 rec=fct-4savfwrc@1 type=5
@row_none n=7 rec=fct-7ek3dcs2@1 type=5
@row_none n=8 rec=fct-k24bagpo@1 type=5
@row_none n=9 rec=fct-mgfmrmcl@1 type=S
@row_none n=10 rec=fct-skfeoxqz@1 type=4
@row_none n=11 rec=fct-mko7xxip@1 type=4
@row_none n=12 rec=fct-muwas3xf@1 type=4
@row_none n=13 rec=fct-c6ebacq5@1 type=4
@row_none n=14 rec=fct-uvqi5pjd@1 type=4
@row_none n=15 rec=fct-ofv5h6kr@1 type=4
@row_none n=16 rec=fct-tp3nl2za@1 type=4
@row_none n=17 rec=fct-iuhcm6lf@1 type=4
@row_none n=18 rec=fct-bjb5m6i6@1 type=4
@row_none n=19 rec=fct-mp7lxj2l@1 type=4
@row_none n=20 rec=fct-4mm5bgqu@1 type=4
@row_none n=21 rec=fct-lg2pk5x3@1 type=4
@row_none n=22 rec=fct-uy4tgw3u@1 type=4
@row_none n=23 rec=fct-wyy55z4s@1 type=4
@row_none n=24 rec=fct-56elu7kz@1 type=4
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
@row_none n=45 rec=fct-gj3e32rb@1 type=7
@row_none n=46 rec=fct-h6kn5x4s@1 type=S
@row_none n=47 rec=fct-63f2oo75@1 type=4
relied: Request URLs and the cache directory path appear in the logs.
@row_none n=48 rec=fct-oamkovdl@1 type=S
@row_none n=49 rec=fct-l5wivwzn@1 type=5
@row_none n=50 rec=fct-jac45bwt@1 type=S
@row_none n=51 rec=fct-ij663iad@1 type=4
relied: Clear history deletes every history row both in memory and on disk, and the HTTP cache can be cleared on demand.
@row_none n=52 rec=fct-kxutn4hq@1 type=S
@row_none n=53 rec=fct-kcaaghma@1 type=5
@row_none n=54 rec=fct-xqvxiqdb@1 type=S
@row_none n=55 rec=fct-7bl5xu2z@1 type=S
@row_none n=56 rec=fct-ms33hdhp@1 type=5
@row_none n=57 rec=fct-jgw23mmw@1 type=6
@row_none n=58 rec=fct-l3h5hlj3@1 type=5
@row_none n=59 rec=fct-yvy6uwir@1 type=5
@row_none n=60 rec=fct-ovf2eb7i@1 type=5
@row_none n=61 rec=fct-2cgb5j27@1 type=S
@row_none n=62 rec=fct-pxydvv3o@1 type=S
@row_none n=63 rec=fct-gpqv3vsg@1 type=S
@row_none n=64 rec=fct-p7pp73ko@1 type=7
@row_none n=65 rec=fct-77bajbne@1 type=S
@row_none n=66 rec=fct-4mvsrpxb@1 type=5
@row_none n=67 rec=fct-f54sui4c@1 type=5
@row_none n=68 rec=fct-wztu7kmt@1 type=5
@row_none n=69 rec=fct-hsmt2k5u@1 type=5
@row_none n=70 rec=fct-x42gr5q3@1 type=5
@row_none n=71 rec=fct-wjxtmswg@1 type=5
@row_none n=72 rec=fct-ddawzqlr@1 type=5
@row_none n=73 rec=fct-q6ezxofh@1 type=S
@row_none n=74 rec=fct-c2hnlfa7@1 type=S
@row_none n=75 rec=fct-ecpelafu@1 type=4
relied: A failed fetch shows the user error.html with the error's Debug text in its error paragraph.
@row_none n=76 rec=fct-fcagkjyj@1 type=4
relied: Error pages are flagged is_error and skip history recording and favicon probing.
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
@row_none n=90 rec=fct-wwenmixs@1 type=4
@row_none n=91 rec=fct-eoknrijk@1 type=4
@row_none n=92 rec=fct-7vi4wxp5@1 type=4
@row_none n=93 rec=fct-hrr3a26v@1 type=4
relied: Resolve and hit testing may run with no DOM and return early with a warning, and a UI event on a document with no target is ignored.
@row_none n=94 rec=fct-pepdhmys@1 type=4
@row_none n=95 rec=fct-rpgeuznq@1 type=4
@row_none n=96 rec=fct-c4rr3jfv@1 type=4
relied: Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic.
@row_none n=97 rec=fct-qe5j3bt2@1 type=4
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
@row_none n=110 rec=fct-hak4wvpc@1 type=S
@row_none n=111 rec=fct-i7a2ccfw@1 type=S
@row_none n=112 rec=fct-hgsjyuub@1 type=5
@row_none n=113 rec=fct-ysjsa4jp@1 type=4
@row_none n=114 rec=fct-o2zqeciv@1 type=4
@row_none n=115 rec=fct-lpzpej2o@1 type=5
@row_none n=116 rec=fct-lfei2kpx@1 type=S
@row_none n=117 rec=fct-byqhlzov@1 type=7
@row_none n=118 rec=fct-43xeyzsr@1 type=S
@row_none n=119 rec=fct-o2jlcnmm@1 type=S
@row_none n=120 rec=fct-gnqolrzj@1 type=S
@row_none n=121 rec=fct-d2grrx3s@1 type=4
@row_none n=122 rec=fct-rbvhsytv@1 type=4
@row_none n=123 rec=fct-p2nck7ms@1 type=4
relied: Aborting the configured AbortSignal cancels every in-flight fetch tied to the document.
@row_none n=124 rec=fct-bj5snojm@1 type=4
@row_none n=125 rec=fct-zyokxxre@1 type=4
@row_none n=126 rec=fct-3vrrikft@1 type=4
relied: Sub-document navigations route back to the parent document instead of navigating the host.
@row_none n=127 rec=fct-uljish3c@1 type=4
@row_none n=128 rec=fct-nhovh6p5@1 type=4
@row_none n=129 rec=fct-zdfj4pze@1 type=4
@row_none n=130 rec=fct-n5ajqukn@1 type=4
@row_none n=131 rec=fct-e3bbwhnz@1 type=4
@row_none n=132 rec=fct-4w3pesu2@1 type=6
@row_none n=133 rec=fct-3ntkhij6@1 type=6
@row_none n=134 rec=fct-zrg6urnw@1 type=8
@row_none n=135 rec=fct-nyplqb5c@1 type=8
@row_none n=136 rec=fct-uxldgqq5@1 type=8
@row_none n=137 rec=fct-37qbmwdh@1 type=8
@row_none n=138 rec=fct-sn3x6tqr@1 type=8
@row_none n=139 rec=fct-b44ukc3q@1 type=5
@row_none n=140 rec=fct-6rk5sgno@1 type=5
@row_none n=141 rec=fct-upgu656r@1 type=4
relied: The select list's public readers and writer read a row with no styles as None and add no panic path.
@row_none n=142 rec=fct-wogfa4ah@1 type=4
relied: The render-path recorder holds no unwrap, expect or panic and has no slicing panic path.
@row_none n=143 rec=fct-h5tukp3g@2 type=9
@row_none n=144 rec=fct-64ubemwx@1 type=4
relied: A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row.
@row_none n=145 rec=fct-5onuddg3@1 type=4
relied: A failed, empty or non-CSS stylesheet does not block render, and a net handler wrapper that does not forward error drops the failure silently.
@row_none n=146 rec=fct-rfjghjp2@1 type=4
@row_none n=147 rec=fct-nij6scrl@1 type=4
@row_none n=148 rec=fct-y5sakwj7@1 type=4
@row_none n=149 rec=fct-gnkizqzo@1 type=4
@row_none n=150 rec=fct-pbvi5fsv@2 type=4
@row_none n=151 rec=fct-36ipyrv2@1 type=4
relied: An open select list's highlighted row is in no snapshot and no tree, and the lint does not read its contrast.
@row_none n=152 rec=fct-dxkboam6@1 type=4
relied: Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused.
@row_none n=153 rec=fct-cfr7fwzo@1 type=4
relied: The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session.
@row_none n=154 rec=fct-uysume23@1 type=4
@row_none n=155 rec=fct-b5nx4un7@1 type=4
@row_none n=156 rec=fct-wqjbaxzx@1 type=4
@row_none n=157 rec=fct-qhaepvto@1 type=4
@row_none n=158 rec=fct-evgmbubw@1 type=4
@row_none n=159 rec=fct-hbpzpyqz@1 type=4
@row_none n=160 rec=fct-mgkywpyf@1 type=4
@row_none n=161 rec=fct-52dzlja5@1 type=4
@row_none n=162 rec=fct-nrsyi4y5@1 type=4
@row_none n=163 rec=fct-ktl6rbzh@1 type=4
@row_none n=164 rec=fct-c32xddbc@1 type=4
@row_none n=165 rec=fct-zd4tccjz@1 type=4
@row_none n=166 rec=fct-o7jypgh2@1 type=4
@row_none n=167 rec=fct-cbyszk2x@1 type=4
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
@row_none n=184 rec=fct-zba4zqnk@1 type=4
@row_none n=185 rec=fct-3obvilj2@1 type=4
@row_none n=186 rec=fct-lnusqzqb@1 type=4
relied: The :open pseudo-class matches a details or dialog by its open attribute's presence, kept current by attribute writes and removals.
@row_none n=187 rec=fct-ohebonan@1 type=4
@row_none n=188 rec=fct-riwkju6q@1 type=4
@row_none n=189 rec=fct-5bmmkpof@1 type=4
@row_none n=190 rec=fct-rc3jl2xp@1 type=4
relied: Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts.
@row_none n=191 rec=fct-6dak5se6@1 type=4
relied: The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`.
@row_none n=192 rec=fct-pnrbnsjg@2 type=4
relied: Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open.
@row_none n=193 rec=fct-exdlckxb@1 type=4
relied: Degenerate aspect ratios are discarded before layout uses them.
@row_none n=194 rec=fct-o56d7tkg@1 type=4
relied: The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout.
@row_none n=195 rec=fct-pwj6p2fo@1 type=4
relied: Geometry whose transformed bounds are non-finite or beyond the f32 range is culled before painting.
@row_none n=196 rec=fct-gkohgcpp@1 type=4
@row_none n=197 rec=fct-lk3nconf@1 type=4
@row_none n=198 rec=fct-gukbahaa@1 type=4
relied: Scroll targets are clamped to the scrollable range, and a programmatic scroll to a missing node does nothing.
@row_none n=199 rec=fct-ucse43ob@1 type=4
@row_none n=200 rec=fct-q43mkq7y@1 type=4
relied: Form submission excludes controls under a datalist, disabled controls, non-submitter buttons and unchecked checkboxes and radios.
@row_none n=201 rec=fct-skuqzr72@1 type=4
@row_none n=202 rec=fct-xdyrrrnw@1 type=4
@row_none n=203 rec=fct-omhuceqe@1 type=4
@row_none n=204 rec=fct-3atajnay@1 type=4
relied: The Dioxus event handler records a dispatch only where a signal registry is registered, and a page with none dispatches as before.
@row_none n=205 rec=fct-et47i5mq@1 type=4
relied: Dioxus dangerous_inner_html is applied as inner HTML.
@row_none n=206 rec=fct-t6dnzfpt@1 type=4
@row_none n=207 rec=fct-wlz77kgt@1 type=4
relied: A falsy value of a listed boolean attribute clears the attribute rather than writing it.
@row_none n=208 rec=fct-vewbxbie@1 type=4
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
@row_none n=218 rec=fct-uwwmkxhm@1 type=4
@row_none n=219 rec=fct-62emqkgs@1 type=4
relied: Stored URLs are re-parsed on load and unparsable rows are skipped.
@row_none n=220 rec=fct-txc2a4of@1 type=4
relied: Serialization escapes text and attribute values but writes text inside raw-text elements such as style and script unescaped.
@row_none n=221 rec=fct-anxwobo4@1 type=4
@row_none n=222 rec=fct-3u57ggad@1 type=4
@row_none n=223 rec=fct-w2qhbiox@1 type=4
@row_none n=224 rec=fct-f4erdbji@1 type=4
@row_none n=225 rec=fct-as2qocp2@1 type=4
@row_none n=226 rec=fct-zomjwwb3@1 type=4
@row_none n=227 rec=fct-ub5ruuk7@1 type=4
@row_none n=228 rec=fct-xfn3rqzp@1 type=4
@row_none n=229 rec=fct-iti42js5@1 type=4
@row_none n=230 rec=fct-z2c6uyfa@1 type=4
@row_none n=231 rec=fct-736ntlrb@1 type=4
@row_none n=232 rec=fct-qt4tvftd@1 type=5
@row_none n=233 rec=fct-4ck7btcl@1 type=4
relied: A failed load's error text stops in the load response and the redacted Resource load failed line, and no carrier takes it.
@row_none n=234 rec=fct-26u4zyfp@1 type=5
@row_none n=235 rec=fct-ksl3fslx@1 type=4
relied: A select list's option and group labels leave only as painted pixels and the snapshot's existing name, the list holding no log site, span or field.
@row_none n=236 rec=fct-4cylhur3@1 type=4
relied: The render-path mutation buffer logs nothing itself and its replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.
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
