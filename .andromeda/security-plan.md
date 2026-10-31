## Threat Model Summary

Detail: .andromeda/security-plan-detail/threat-model-summary.md — read whole.

**Security tier:** `0`

> NOT YET MEASURED — no slice states an application-level threat model; data classification, auth model, infrastructure and compliance triggers were not gathered for this summary (the stored-data and auth facts the reading did reach are in the sections below).

**Attack surface:**

- **Vector:** remote web content (HTML, CSS, images, fonts, sub-documents) — the browser and its net provider fetch and parse arbitrary remote documents, `file:` and `data:` URLs, SVG data and sub-documents; iframe and `@import` nesting depths are limited and stale iframe responses discarded by request id; per-input checks are listed under Input Validation.

- **Vector:** script execution — `blitz-vibey-script` executes a document's scripts, `<body onload>` included, and JS `fetch()` and module imports share its ScriptFetcher; in the browser app JavaScript is opt-in behind the non-default `javascript` feature, and the WPT runner executes test scripts; the default fetcher takes only `file:` and `data:` URLs, every runtime event reads `isTrusted` true, and same-origin or CORS checks are observed absent.

- **Vector:** CLI input
  - **Entry point:** `screenshot` and `paint_bench` fetch and render an arbitrary URL given on the command line (examples/screenshot.rs:27-54; examples/paint_bench.rs:58-85). `preact_script` executes the scripts of an arbitrary local HTML file given on the command line (examples/preact_script.rs:15-35). rdme fetches remote URLs and treats a `.md` suffix as markdown (apps/readme/src/main.rs:186-215)
  - **Trust boundary:** argument parsing is listed under Input Validation.
- **Vector:** markdown rendering (rdme)
  - **Entry point:** rdme's comrak renderer sets `unsafe: true`, passing raw HTML in markdown through (apps/readme/src/markdown/comrak.rs:27-31)
- **Vector:** WPT runner local file reads
  - **Entry point:** Non-`data:` request URLs have their path joined onto the WPT base path and the file is read (wpt/runner/src/net_provider.rs:70-78)
- **Vector:** CI workflow triggered by pull requests
  - **Entry point / Trust boundary:** The post-results workflow runs only in `DioxusLabs/blitz` (repository guard), only for successful `pull_request`-triggered WPT runs, and checks out scripts from the default branch, a step named "Checkout trusted scripts" (.github/workflows/wpt-post-results.yml:3-6; .github/workflows/wpt-post-results.yml:15-21)

- **Vector:** local IPC — the driver session's socket — `escher_driver::serve` binds one Unix-domain socket, `session.sock`, in a caller-named state directory, no TCP or UDP port, unix only; trust boundary the owning user (directory `0700`, a group- or other-open one refused, socket `0600`), no other authentication; it carries the lifecycle and calls with their answers — ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots, a lint's findings — and no `NodeId`.

- **Vector:** local stdio — the driver's MCP server — `escher-session mcp`, a stdio child an MCP client spawns, speaks JSON-RPC 2.0 on its stdin and stdout, no listener, port or socket; trust boundary the spawning client process, no authentication; a result carries ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings to the client, and what a client does with them, a model provider included, is the client's — the cold-agent live runs carried the screen so, a screenshot and a lint's findings among it.

---

## Authentication & Authorization

Detail: .andromeda/security-plan-detail/authentication-authorization.md — read whole.

**Auth approach:** none observed. Application authentication or authorization code is observed absent in every slice that searched:

The s02 fixtures hold no auth code; the google fixture only shows a "Sign in" link to accounts.google.com (examples/assets/google.html:3100-3107).

Auth controls by requirement: with feature `cookies` the reqwest client keeps a cookie store; CI tokens are scoped by GitHub Actions `permissions` per workflow — every ci.yml job's `GITHUB_TOKEN` read-only, the publish job repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.

The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check: the socket's access is the filesystem's (state directory `0700`, a group- or other-open one refused, socket `0600`), and `start` does not check the answering pid; the MCP server answers only the process holding its stdin and stdout and binds nothing.

Other `password` occurrences are not credentials:
- A `password` accessor (the URL password component) is defined on the element prototype for `<a>`/`<area>`, read from and written into the resolved `href` (packages/blitz-vibey-script/src/dom/hyperlink.rs:32; packages/blitz-vibey-script/src/dom/hyperlink.rs:127; packages/blitz-vibey-script/src/dom/hyperlink.rs:170-172)
- An `<input type="password">` fixture is mapped to `Role::PasswordInput` in an accessibility test (tests/blitz-tests/tests/accessibility_roles.rs:140; tests/blitz-tests/tests/accessibility_roles.rs:153)

---

## Input Validation

Detail: .andromeda/security-plan-detail/input-validation.md — read whole.

CLI arguments (examples) — numeric arguments fall back to defaults; a URL (retried with `https://`) or `preact_script` path that cannot be resolved panics, as does an unknown `multi_doc` mode; an output filename keeps only the URL's ASCII alphanumerics.

CLI arguments (agent-run.sh) — a verb outside `boot run status cleanup logs`, a wrong argument count or a selection other than `stand`, `all` or an existing test-file stem prints usage and exits 2 before any cargo call.

CLI arguments (cold-agent.sh) — a verb outside `run status cleanup logs`, a wrong argument count or a task outside the allowlist `stand` exits 2 before any precondition or `claude` call; the task reaches the agent session only as the fixed prompt, naming no element id and no app slug.

CLI arguments (apt-install.sh) — package names and option values must match closed forms, else usage and exit 2 before `sudo`, `timeout`, `apt-get` or `dpkg` runs; the script reads no environment variable of its own, writes no file and names no secret; every attempt after the first runs a bounded `dpkg --configure -a` repair.

CLI arguments (coverage_floor.py) — each floor must match `CRATE_DIR=PERCENT` (a percent at most 100, no crate twice) and every crate must have an `SF` record, else exit 2; it prints only crate directories and counts, never a source or lcov path.

Cold-agent check (cold-agent.sh, the agent-written check) — the model's reply is a new input class: only its last fenced json block, bounded in bytes and steps, each step checked against closed keys; a check outside these is never replayed, a valid one replayed on fresh `escher-session mcp` children, every step validated again by the server under a reply bound; `check.json` is never printed; the live child's kept checks (`--record`) are read for bools and counts only and never printed, and a paged answer is decoded only as exactly `text` · `part` · `parts`.

Driver session socket (escher-driver) — one request line per connection, parsed against a closed grammar under a size bound; anything else is refused with nothing run, and a decoded call runs through `Session::run` alone; one connection at a time under fixed bounds; bounded replies carry ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings, the state readings as closed words and booleans, never an attribute's text, and no `NodeId`.

Driver session socket (escher-driver) — the widening rule: a new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.

Driver session state directory (escher-driver) — created `0700`, a present one open to its group or others refused, the socket `0600`, a path too long for a socket address refused with a fixed message holding no path; only the socket file and then the directory are ever deleted, non-recursively; the path is a parameter, never an env read or a cwd-relative default.

Driver recording directory (escher-driver) — created `0700`, a group- or other-open one refused with nothing written or spawned; each check created `0600` and never replaced; a failed write or a passed bound ends the recording; a file the recorder did not create is never removed; unix only; no tool call, verb, argument or result field names the directory, and nothing of it is logged.

Recorded check (escher-driver `Check::from_bytes`) — a kept check's bytes are untrusted: a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused `malformed` holding nothing of the file, with no panic path; every replayed step still passes `validate` inside `Session::run`.

Driver session label (escher-driver) — 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs; never text read from the document; the one caller string a reply carries.

Driver command schema (escher-driver) — `validate` checks every call against a closed `'static` table of twelve verbs and seven bounded argument kinds, returning a typed `Command` or a `Refusal`, the first failure in a fixed order; nothing public runs a `Command` that did not come from `validate`, and the socket, the command line and MCP all pass through it; an `id` naming no element, or an element that cannot take the call, is refused with nothing run.

Driver command schema (escher-driver) — no `unwrap`, `expect` or panic on anything a call supplies, no clock read by the schema or its validation, no `NodeId` kept between calls; one `command` span per call holds fixed words, bools and counts only, never a call's content, the screen text, an image or findings; the crate installs no subscriber and reads no env var.

CLI arguments (escher-session) — the argv is read by `escher_driver::command_line`, a closed reader with fixed rules; a call is checked by `validate` or `validate_session` before the session address is asked for; four endings with fixed statuses (0 accepted, 1 refused, 2 usage, 3 session error); no ending writes back anything the caller typed, no path on argv panics, and the command line installs no subscriber and reads no environment variable.

MCP server (escher-driver `serve_mcp`, the `escher-session mcp` role) — newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, no listener, port, authentication or env read; request lines and results bounded; protocol errors a fixed code and message with no `data`; tool arguments typed, then `validate` or `validate_session`; the admitted request `id` the one client-sent value written back; no print, `unwrap`, `expect` or `panic!` outside the tests.

Browser urlbar — typed text is parsed as a URL, then as a dotted space-free host with https, else turned into a search query.

Navigation to external browser — limited to GET requests with http, https or mailto schemes.

Link `href` — resolved against the document URL; an unparseable href is not navigated.

Document URL — a parse error is returned instead of a panic.

URL fragment — percent-decoded lossily before matching.

`file:` URLs (net provider) — read from local disk with no path restriction; a `file:` URL that is no file path is a typed error, no panic.

`data:` URLs (net provider) — decoded by the `data_url` crate, errors mapped to `ProviderError`.

`data:` URLs (shell) — only the `data` scheme is handled; a failed parse or decode returns silently.

Script fetch — invalid `data:` URLs or non-UTF-8 script bytes are `FetchError::InvalidData`; a `file:` URL loads only for a `file:` document, `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError` — a narrowing: the default fetcher takes only `file:` and `data:`, and same-origin and CORS checks are absent.

Script type — only an empty, `text/javascript`, `application/javascript` or `module` type is executed.

HTML parsing — HTML is parsed with scripting disabled.

Fetched stylesheets — must be valid UTF-8, else the load errs.

Fetched iframe HTML — decoded lossily as UTF-8.

Fetched images — decoded with a guessed format, then an SVG parse fallback, else an error.

Favicon — accepted only when it decodes as a raster image or SVG.

Fetched fonts — the format is sniffed from the first four bytes when no hint is given; bytes shorter than 4 pass through, and a failed WOFF decompression keeps the original bytes.

`@font-face` source — a source whose URL cannot be resolved is skipped, no panic; a hint-less source with no extension (a `data:` URL) is fetched and its format sniffed; escher routes no agent- or user-supplied URL through this path.

SVG data — non-UTF-8 input rejected, XML parsed with DTDs allowed, gzip-compressed SVG detected and decompressed before parsing.

SVG `viewBox` — exactly four finite non-negative numbers; a zero width or height is flagged degenerate.

Inline SVG source — a parse error is logged and the element left without image data.

CSS declarations — validated by parsing: invalid declarations are ignored, and unsupported property names or invalid values refused.

Selectors — an invalid selector list leaves the rule unchanged; query parse errors are returned.

`<style>` element — HTML entities are decoded before parsing.

Legacy attributes — `bgcolor`, dimensions, SVG `width` / `height`, `border` and body margin pass closed parsers; anything else is ignored or rejected.

Markup numeric attributes — parsed with default fallbacks; `rowspan` clamped to 1..=65534, `<ol start>` parsed as `usize`.

Markup attributes — read from parsed content, remote pages included, by in-process readers with no log site or panic path: `disabled` and `inert` by presence, ARIA states and closed values through closed parses carrying no attribute text, the range and select controls held within their rules, Enter activating buttons and links and Space buttons as a click, names and descriptions as page text; they leave escher only by the ratified carriers, the page's own form submission and events aside.

Markup attributes — the stable element id: the HTML `id` as an author key when non-empty, `/`-free and first in document order, else an anchored, component or document path, so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.

Markup attributes — a password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path; a password input paints a bullet per character, and a recorded masked `type` is kept as `MASKED_STAND_IN`; the DOM and form submission keep the value.

Layout values — degenerate aspect ratios are discarded, inline-box heights kept finite, and `text_extent` is a read-only reader returning geometry only.

Paint values — non-finite geometry is culled, gradient overflow clamped, oversized radii scaled down, and hidden inputs not painted.

Scrolling — targets are clamped to the scroll range; a programmatic scroll to a missing node does nothing.

Forms — `method` and `enctype` fall back to defaults; submitted controls are filtered, line endings normalized to CRLF, and a file input's `accept` is not applied as a filter.

Forms (dioxus-native) — form data validity is not implemented: `NativeFormData::valid` always returns true.

Dioxus mutations — `data-dioxus-id` parsed as `usize`, unsupported attribute value types ignored, falsy boolean attributes removed, and `dangerous_inner_html` applied with `set_inner_html`.

JS API arguments — the script runtime's native APIs check or coerce their arguments, throwing a RangeError, TypeError, SyntaxError or JS error on a bad call or falling back to a default; `fetch()` takes GET or HEAD only.

Persistence (sqlite) — values are bound through `params!` placeholders; stored URLs re-parsed on load, unparsable rows skipped.

Serialization output — text and attribute values are escaped, but raw-text elements' text is written unescaped; inline SVG must serialize as well-formed XML.

Test harness queries — selector parsing returns a `Result`, while the harness query helper panics on an invalid selector; a text or comment scope matches nothing.

WPT runner — quarantine entries, fuzzy ranges, `checkLayout` values, `data:` URLs and page messages are checked, a bad entry panicking, dropped or failing its test or request; wrapper HTML is escaped; only JavaScript script types run.

CI scripts — the PR lookup takes only an open PR whose head sha equals the run's, and the comment splice replaces only when both markers exist in order.

Example apps — the 7GUIs inputs, the widget's `color` attribute and the TodoMVC input are validated or fall back.

Example fixtures — the graphite and google fixtures constrain their form inputs (required, `type=email`, `maxlength`).

---

## Data Protection

Detail: .andromeda/security-plan-detail/data-protection.md — read whole.

**In transit:**
- **TLS:** TLS is provided through reqwest's `native-tls` feature (`native-tls-vendored` on Android) (packages/blitz-net/Cargo.toml:26; packages/blitz-net/Cargo.toml:37-38)
- Encryption or TLS code is observed absent in s10 — searched: `encrypt|cipher|rustls|tls` over the 32 s10 files — and in s11 — searched: `encrypt|crypt|tls|rustls|cert` over the 21 listed s11 files.

> NOT YET MEASURED — HSTS, TLS version requirements and certificate pinning: no slice gathered them.

**At rest — per medium:**
- **Database:** History is written to a sqlite file opened with `Connection::open` (apps/browser/persistence/src/lib.rs:280-291). Database encryption is observed absent — searched: `encrypt|cipher|sqlcipher` (case-insensitive) over the 86 s04 files. On mobile, history is in-memory only and does not persist across launches (apps/browser/persistence/src/lib.rs:12-13)
- **Files / object storage (HTTP cache):** With feature `cache`, HTTP responses are cached on disk through `CACacheManager` at the platform cache directory (packages/blitz-net/src/lib.rs:38-60; packages/blitz-net/src/lib.rs:92-93); the cache policy is evaluated as a private (`shared: false`) cache (packages/blitz-net/src/lib.rs:109-112). Cookies and HTTP cache are default browser features (apps/browser/Cargo.toml:14; apps/browser/Cargo.toml:31-32)

- **Files (kept checks):** under `--record <dir>` on `escher-session` or `Session::recording_into`, one `check-{n}.json` (`0600`) per recorded session in a `0700` directory the launcher named — per call its verb, arguments, acceptance, digest and masked answer, never a pixel; owner-only, except the two committed checks (below); never logged, never removed by escher but for its own check when a recording ends; the cold-agent pipe's live child keeps its checks so in the owner-only `target/cold-agent/record/`, read by the pipe for bools and counts only, never printed, copied or committed, removed by its `cleanup`.

- **Files (committed checks):** two kept checks of the repository's own scripted stand-Counter sessions are committed, `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` — the one exception to a kept check's owner-only medium, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`; a check of any other session stays owner-only and is never committed.

- **Logs:** request URLs and the cache directory path are logged (see Logging & Monitoring)
- Encryption, redaction or sanitization of data is observed absent in s05 — searched: `encrypt|crypt|sanitiz|redact|scrub` over the 15 s05 files; escher's own log sink now redacts and, for a record from a target outside its allowlist, drops (escher-telemetry's target allowlist — see Logging & Monitoring).

**Local user data handled:**
- Copy writes the selected document text to the clipboard only when no text input is focused (packages/blitz-dom/src/events/keyboard.rs:46-64); clipboard text is read and written through arboard on desktop OSes behind the clipboard feature (packages/blitz-shell/src/lib.rs:162-196)
- The native file dialog uses rfd with optional name/extension filters behind the file-dialog feature (packages/blitz-shell/src/lib.rs:198-225)
- A chosen file's full path is stored in the file input's `value` attribute (packages/blitz-dom/src/events/pointer.rs:959-962); `FormData` file entries carry a filesystem path that is serialized as its string form (packages/blitz-traits/src/net.rs:116-143); the attribute and form submission keep the path, while under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` reads that value as `MASKED_VALUE`, so neither the snapshot, its text (`Snapshot::to_text`) nor a diff of two snapshots (`Snapshot::diff`, which takes its values from the snapshots only) holds it (packages/dioxus-native-dom/src/snapshot.rs:258-285; packages/dioxus-native-dom/src/snapshot_diff.rs:108) — the reader's half as measured at escher-0.1.0/chunks/2026-10-06-compact-snapshot-serialization/report.md on an authored `value`; the diff's half is by construction, no file-input diff having been run; the dialog's write of the path is read from the code, not run
- Fixtures: the google fixture sets a referrer meta of "origin" (examples/assets/google.html:5); the servo fixtures show Cloudflare email-protection links (`/cdn-cgi/l/email-protection`) in place of plain addresses (examples/assets/servo-new.html:358; examples/assets/servo.html:263; examples/assets/servo.html:324)

**Data lifecycle:**
- **Deletion:** The about:history page offers "Clear history", which deletes all rows in memory and on disk (apps/browser/src/about_pages.rs:135-141; apps/browser/src/browser_history.rs:140-144; apps/browser/persistence/src/lib.rs:261-265). A "Clear Cache" menu item calls `clear_cache` (apps/browser/src/toolbar.rs:298-310); the cache can be cleared through `Provider::clear_cache` (packages/blitz-net/src/lib.rs:137-145)

> NOT YET MEASURED — key management, backups, retention periods and anonymization: no slice gathered them.

---

## API Security

Detail: .andromeda/security-plan-detail/api-security.md — read whole.

No served network API exists — no TCP or UDP port; the one listener is local, the driver session's Unix-domain socket `{state_dir}/session.sock`, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results; the controls below govern outbound requests and in-document navigation.

Outbound requests: at most 6 concurrent per host; caller-supplied headers forwarded with `User-Agent`; non-success statuses typed `ProviderError::HttpStatus`; cancellable through an `AbortSignal`; CORS and same-origin checks, CSP, a response size limit and a request timeout are observed absent.

Settle and loads in flight: a harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`, so the absent request timeout cannot hang a settle or a command; no verb has an argument or a result field of a wait.

Navigation and fetch: sub-resource requests are GETs and JS-facing fetch GET or HEAD only; form submission goes by scheme and method, the `dialog` method rejected; iframe navigations route back to the parent document; external targets go to the system browser; the `dioxus` scheme is served from the local file system; dioxus-native JS evaluation goes to a no-op document.

Outbound HTTP in the examples and apps goes through reqwest or blitz-net; CI API calls go through the `gh api` CLI with the workflow token.

Local session socket: reachable by the owning user only (`0700` directory, `0600` socket, a group- or other-open directory refused), no handshake secret, token or peer-credential check; one connection at a time under fixed read, write and size bounds; a host that answers no request for its idle expiry ends as on `stop`.

Fixture pages also issue requests: the graphite fixture's script calls api.github.com with fetch and no request headers (examples/assets/graphite.html:1862); the servo.org snapshot fixture issues an XMLHttpRequest GET to `https://api.github.com/repos/servo/servo-nightly-builds/releases/latest` (examples/assets/servo.html:351-388).

---

## Dependency Security

Detail: .andromeda/security-plan-detail/dependency-security.md — read whole.

**Audit tool:** cargo-deny 0.20.2 — `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph, configured by the root `deny.toml` (the six platforms ci.yml builds, all features; ignores per ID only, each with a written reason, no blanket allow; `unsound` at its default, which keeps memmap2's unsound advisory out of the gate); each ignore re-read on every push; the reach is cargo-deny's resolved graph, not the whole lockfile.

**Pinning:**
- No git dependency is left in the root manifest: taffy, the one there was, pinned by commit rev until 2026-10-13-upstream-sync-polish-and-ship, is a crates.io registry version since that chunk, "0.15" resolved 0.15.0 with a checksum in `Cargo.lock` (Cargo.toml:105); parley is a registry version, "0.12" (Cargo.toml:115)

- Each upstream sync took upstream's dependency set whole or, by cherry-pick, entered no package — one of them moving lock edges alone (one `mimalloc` edge added, twelve re-pointed to versions already locked) beside a lock-only `tokio` update past RUSTSEC-2026-0386; its lock read name by name, against upstream's or against the tree before it, the audit leg green after it — one of them only with the zbus-family ignores (Audit tool).

- Builds pass `--locked` in the flake, in `dx bundle` and in every ci.yml cargo leg but `examples/wasm_hello`, which has no `Cargo.lock` (flake.nix:100; .github/workflows/publish-browser.yml:157; .github/scripts/ci-leg.sh:20-66; .github/workflows/ci.yml:423)
- Most dependency versions are inherited with `workspace = true` (apps/browser/Cargo.toml:46-75; packages/blitz-dom/Cargo.toml:48-108); every dependency in the blitz-html and blitz-net manifests does so (packages/blitz-html/Cargo.toml:19-26; packages/blitz-net/Cargo.toml:22-41), as do the blitz-traits and blitz manifests (packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:24-39) and every dev-dependency of blitz-tests (tests/blitz-tests/Cargo.toml:15-42)
- Inline versions outside the workspace: examples pin `idna_adapter` to exactly 1.0.0 (examples/counter/Cargo.toml:31-33); android-activity is pinned at "0.6.0" in blitz-shell (packages/blitz-shell/Cargo.toml:53-54); `cfg-if` is pinned at "1.0.4" and `android-activity` at "0.6" in dioxus-native (packages/dioxus-native/Cargo.toml:120; packages/dioxus-native/Cargo.toml:127); `log = "0.4"` is the one inline version in blitz-vibey-script (packages/blitz-vibey-script/Cargo.toml:39); the WPT runner's non-workspace dependencies declare explicit versions, and `dify` and `wptreport` disable default features (wpt/runner/Cargo.toml:36-53)
- rusqlite is built with its `bundled` feature (apps/browser/persistence/Cargo.toml:9)

- `serde_json = "1"` is a root `[workspace.dependencies]` entry inherited by escher-driver with no feature (no `preserve_order`): the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's own answer line the replay compares, writing nothing; no ban list exists (§Security Anti-Patterns).

- `png` is taken by escher-driver, the encoder of a shot, and by blitz-tests as a dev-dependency; blitz-test-harness takes `blitz-paint`, `anyrender` and `anyrender_vello_cpu`; no package enters `Cargo.lock` and no `base64` crate is taken — a shot's base64 is written by hand.

- blitz-test-harness takes `color = { workspace = true }`, already in the graph through blitz-dom, as the luminance source of its contrast reader; no package enters `Cargo.lock`, and blitz-test-harness takes no `tracing` and no `style`.

- CI tooling: `cross` is installed from a pinned git rev; the upstream-only publish workflow installs dioxus-cli `0.8.0-alpha.3`, or on a platform flagged `use_git_cli` from git at upstream's pinned rev, behind its `DioxusLabs/blitz` repository guard; wpt cli is pinned to 0.0.14 (.github/workflows/ci.yml:390; .github/workflows/publish-browser.yml:122-130; .github/workflows/wpt.yml:63); ci.yml installs `cargo-deny@0.20.2` (the audit tool) and `cargo-llvm-cov@0.9.1` (the coverage tool) through `taiki-e/install-action` (.github/workflows/ci.yml:234-236; .github/workflows/ci.yml:281-283); every ci.yml `uses:` is pinned to a 40-hex commit SHA with its ref kept as a trailing comment, asserted by `test_ci_workflows.py` (.github/scripts/test_ci_workflows.py:186-192) — `awalsh128/cache-apt-pkgs-action` included (.github/workflows/ci.yml:410); the upstream-only publish-browser and wpt workflows no longer use it: each installs its packages with a bare `apt-get` line, behind its `DioxusLabs/blitz` repository guard (.github/workflows/publish-browser.yml:148-150; .github/workflows/wpt.yml:42-43)
- Vendored JS: Preact is vendored as an unmodified copy of its UMD builds (examples/preact/index.html:46-48); each vendored Preact file ends with a `sourceMappingURL` comment for a `.map` file (examples/preact/vendor/preact.min.js:2; examples/preact/vendor/hooks.umd.js:2)
- The blitz-tests crate is not published (`publish = false`) (tests/blitz-tests/Cargo.toml:4)

**Update policy:** automated update tooling is observed absent — searched: `dependabot` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py.

**CI integration:** ci.yml's `audit` job ("Dependency audit") runs `bash .github/scripts/ci-leg.sh audit` on every pull request and push, in the slow tier behind the four fast jobs, uncached, its log uploaded on failure as `ci-log-audit` (.github/workflows/ci.yml:225-244).

**Unsafe code:** `stylo_taffy` holds two `unsafe` blocks — one builds a taffy `LengthPercentage` from a raw calc pointer, and one, in its public `resolve_calc_value`, reads a calc value back through such a pointer (packages/stylo_taffy/src/convert.rs:87-92; packages/stylo_taffy/src/convert.rs:284-287); two `unsafe` raw-pointer operations exist for calc values in layout (packages/blitz-dom/src/layout/mod.rs:77; packages/blitz-dom/src/layout/table.rs:190-193)

> NOT YET MEASURED — a critical-CVE response SLA: none is stated. The lockfile's advisory state is read on every push by the audit leg, within cargo-deny's resolved graph (Audit tool above).

### Supply chain integrity

Detail: .andromeda/security-plan-detail/dependency-security.md — read whole.

- **Signed artifacts:** The publish job runs only in `DioxusLabs/blitz` and there uses environment "Signed Builds" only on main or `ci-test` branches, so the fork produces no signed artifact (.github/workflows/publish-browser.yml:37-41); a macOS signing key and an Android keystore are written from secrets for the build and removed in `always()` steps (.github/workflows/publish-browser.yml:106-108; .github/workflows/publish-browser.yml:179-181; .github/workflows/publish-browser.yml:110-120; .github/workflows/publish-browser.yml:183-185)
- **Lockfile verification:** builds pass `--locked` (flake.nix:100; .github/workflows/publish-browser.yml:157), and so does every ci.yml cargo leg — the leg script's build, test, clippy (both lines), doc, msrv, counter, wasm, audit, a11y, coverage and, since 2026-10-14-quality-gates, stand-repeat commands and the matrix command — except `examples/wasm_hello`, a standalone workspace with no `Cargo.lock` (.github/scripts/ci-leg.sh:20-66; .github/workflows/ci.yml:423)
- **Subresource integrity (fixtures):** the gosub fixture's Font Awesome link carries an integrity hash with crossorigin=anonymous and referrerpolicy=no-referrer (examples/assets/gosub.html:86); the servo.org snapshot fixture loads jQuery with an `integrity` hash and `crossorigin="anonymous"` (examples/assets/servo.html:337); other remote stylesheets are linked without integrity attributes (examples/assets/pseudo.html:4; examples/assets/newservo.html:4; examples/assets/servo-new-reduced.html:7; examples/assets/servo-new.html:18; examples/assets/servo-new.html:20) — integrity attributes outside gosub.html are observed absent, searched: `integrity=` over the 21 s02 files; the servo.org snapshot fixture references third-party CDN resources: Font Awesome v5.12.0, Google Fonts, prismjs@1.20.0 on unpkg, jquery-3.4.1 on code.jquery.com (examples/assets/servo.html:26-29; examples/assets/servo.html:337)

- **Licence compliance:** the root `deny.toml` holds upstream's `[licenses]` table byte for byte, and the audit leg runs it on every push over the whole workspace's graph, so the fork gates on licences; upstream's own `licenses` job stays reverted.

> NOT YET MEASURED — SBOM generation and base image scanning: no slice gathered them.

---

## Bootstrap phases (derive for route / setup-project)

- **auth-scaffolding-baseline:** authentication or authorization code is recorded absent — see Authentication & Authorization.
- **dep-audit-tooling-install:** discharged — cargo-deny 0.20.2 with the root `deny.toml` is the audit tool — see Dependency Security.
- **logging-redaction-wire:** discharged for escher's own log sink — `escher_telemetry::init`'s target allowlist redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level, in the two binaries that install it, `seven_guis_native` and `escher-session` (the drop delivered by the route entry "Sink target allowlist", the founder's ruling, 2026-10-07) — see Logging & Monitoring; still open for the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger`, which stay unscrubbed.
- **dep-security-ci-gate:** discharged — ci.yml's `audit` job runs the audit on every push — see Dependency Security.

---

## Secret Management

Detail: .andromeda/security-plan-detail/secret-management.md — read whole.

**Storage:**

- **Production (CI):** secrets live in GitHub Actions secrets and vars — a macOS signing key and an Android keystore written from secrets for the build and removed in `always()` steps, the Apple certificate and keystore passwords passed from secrets, and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens passed to the steps that use them.

- **Development (operator host, the cold-agent live run and the MCP live trial only):** both reach the model provider through the operator's own Claude Code login, held by the `claude` CLI outside the repository — no API key, no env read by the pipe, no CI secret, no new secret source and no auth library; the committed live evidence holds no key, auth header, credential-file name, token field or e-mail address.

**Never in code:** secret reads in source are observed absent in every slice that searched:

**Environment values read (none secret-bearing):**
- `env!("CARGO_MANIFEST_DIR")` at build time (examples/screenshot.rs:172), in a test (packages/blitz-vibey-script/tests/preact.rs:13), since 2026-10-19-session-recording in escher-driver's unit tests, which reach the workspace `target/tmp` through it (packages/escher-driver/src/mcp_tests.rs:650-651) and in upstream's `tests/all.rs` of blitz-tests, a target held at `test = false` and never built (tests/blitz-tests/tests/all.rs:78-95)
- `HOME`, used for the iOS cache path (packages/blitz-net/src/lib.rs:46)
- `PAINT_TREE_BENCH_HTML`, an HTML file path for an ignored benchmark (tests/blitz-tests/tests/paint_tree_bench.rs:262-269)
- `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`, read by the root example `multi_doc` alone through two `std::env::var` call sites: a document count parsed with a default fallback and two flags read through its `env_flag` (examples/multi_doc.rs:17-19; examples/multi_doc.rs:22; examples/multi_doc.rs:36-40; examples/multi_doc.rs:55-58)
- `WPT_DIR` (plus compile-time `CARGO_MANIFEST_DIR`) (wpt/runner/src/main.rs:472; wpt/runner/src/main.rs:494)
- `RUST_LOG`, the tracing filter directives escher-telemetry's `EnvFilter` reads, defaulting to `warn` when unset or unparsable — not secret (packages/escher-telemetry/src/lib.rs:144)
- `env!("CARGO_BIN_EXE_escher-session")` and `env!("CARGO_TARGET_TMPDIR")` at build time in the session checks — the host binary's path and the directory their state directories live under; the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init` (examples/seven_guis/tests/host_binary.rs:16; examples/seven_guis/tests/host_log.rs:24; examples/seven_guis/tests/common/mod.rs:17-19; tests/blitz-tests/tests/session_common/mod.rs:334)
- `TMPDIR`, through `std::env::temp_dir()` — on unix that variable, else `/tmp` — at two test-only sites: escher-driver's unix-only client unit test `client::imp::tests`, which places its test-only listener's directory `eab-{pid}` there and removes it at the end — a `UnixListener` that accepts one connection and never answers, served by no binary and reached by no caller, so no crossing — and blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files`, which makes the directory of files its `file:`-load tests read; test code only, not secret (packages/escher-driver/src/client.rs:263-308; packages/blitz-vibey-script/tests/dom.rs:900-909; since 2026-10-14-quality-gates, registered as test-only on the operator's answer at that chunk's wrap, 2026-10-15)

**Fixture tokens:** the google fixture carries an http-equiv origin-trial meta with an encoded value (examples/assets/google.html:7)

**What counts as secret (as observed):** CI signing material — the macOS signing key, the Android keystore and its passwords, the Apple certificate and its password — and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens (see Storage above); on the operator host, the operator's Claude Code login the cold-agent live run and the 2026-10-10-mcp-surface live trial use, held by the `claude` CLI — no artifact, event or verdict carries it.

> NOT YET MEASURED — development secret storage (beyond the fact that the cold-agent run's login is held by the `claude` CLI; its at-rest location is unmeasured), secret scanning in CI, rotation cadence and access auditing: no slice gathered them.

---

## Error Handling

Detail: .andromeda/security-plan-detail/error-handling.md — read whole.

**External responses (to end users):**
- A failed fetch renders `error.html` with the error's Debug text in the `#error` paragraph (apps/browser/src/document_loader.rs:151-177; apps/browser/assets/error.html:19-20)
- An empty response body renders the bundled 404 page and is flagged `is_error` (apps/browser/src/document_loader.rs:131-137; apps/browser/assets/404.html:12); error pages skip history recording and favicon probing (apps/browser/src/tab.rs:140-149)
- CLI tools exit with status 1 after an stderr message: rdme when the argument is neither URL nor file, or no README.md is found (apps/readme/src/main.rs:180-182; apps/readme/src/main.rs:232-235); `paint_bench` prints `Unknown backend ...` for an unknown backend (examples/paint_bench.rs:320-323)
- Fixtures: the graphite fixture's stars script wraps the fetch in try/catch and removes the target element on any failure (examples/assets/graphite.html:1861-1872); the iframe page A fixture links to a missing page labelled "Broken link (should stay usable)" (examples/assets/iframe_page_a.html:25)

**Internal logging (errors logged and dropped):**
- Persistence errors are logged at warn and swallowed; callers never see a `Result` (apps/browser/persistence/src/lib.rs:14-15; apps/browser/persistence/src/lib.rs:106-108; apps/browser/persistence/src/lib.rs:200-204)
- Failed resource loads are logged (under `tracing`) and dropped; pending image waiters for the URL are removed (packages/blitz-dom/src/document.rs:1466-1487)
- An unresolvable iframe `src` is logged and skipped (packages/blitz-dom/src/mutator.rs:1443-1447); WOFF decompression failure is logged and the original bytes are kept (packages/blitz-dom/src/net.rs:343-366)
- In `NetProvider::fetch`, a failed fetch is logged when tracing is enabled, and otherwise discarded; the handler is not called on error (packages/blitz-net/src/lib.rs:307-319); a failure to clear the cache is logged at error level when tracing is enabled, and otherwise discarded (packages/blitz-net/src/lib.rs:138-144)
- Parse errors are collected into `DocumentHtmlParser::errors` and emitted at error level only with feature `tracing` (packages/blitz-html/src/html_sink.rs:51-52; packages/blitz-html/src/html_sink.rs:181-190)
- `eval`/`eval_module` log but do not propagate uncaught errors (packages/blitz-vibey-script/src/runtime.rs:127-138; packages/blitz-vibey-script/src/runtime/modules.rs:94-131); script URL resolve or fetch failures are recorded and that script is skipped (packages/blitz-vibey-script/src/document.rs:157-177)
- A failed asset fetch is logged at warn and the handler is not called (packages/dioxus-native/src/assets.rs:51-54); a failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:12-16)
- Unsupported winit variants fall back to a default value and log tracing::error when tracing is enabled (packages/blitz-shell/src/convert_events.rs:41-45; packages/blitz-shell/src/convert_events.rs:78-82; packages/blitz-shell/src/window.rs:850-854)
- Table descendants without styles are skipped with an info log rather than a panic (packages/blitz-dom/src/layout/table.rs:535-539)
- Results discarded outright: channel send results with `let _ =` (packages/blitz-dom/src/net.rs:123; packages/blitz-dom/src/iframe.rs:40); the clipboard write result (packages/blitz-dom/src/events/keyboard.rs:61); IME update and window drag results (packages/blitz-shell/src/lib.rs:125-129; packages/blitz-shell/src/lib.rs:133; packages/blitz-shell/src/lib.rs:159); the proxy's channel send result (packages/blitz-shell/src/event.rs:93-96)

**Error format (typed errors):**
- Stylo `RulesMutateError` values map to `CssomError` (packages/blitz-dom/src/cssom.rs:71-80); CSSOM errors map to DOMException names, and `selectorText` and `cssText` assignment never throws (packages/blitz-vibey-script/src/dom/stylesheet.rs:92-94; packages/blitz-vibey-script/src/dom/stylesheet.rs:253-273)
- `FetchError` implements `Display` with scheme, IO, invalid-data and forbidden-load messages, and `std::error::Error` (packages/blitz-vibey-script/src/fetch.rs:17-30); module resolve or fetch failures become JS TypeErrors (packages/blitz-vibey-script/src/runtime/modules.rs:65-86)
- `ClipboardError` is a unit struct with a TODO to "fill out with meaningful errors" (packages/blitz-traits/src/shell.rs:5-7); arboard::Clipboard::new is unwrapped while get/set errors are mapped to ClipboardError (packages/blitz-shell/src/lib.rs:175-177; packages/blitz-shell/src/lib.rs:193-195)
- Missing nodes in mounted-element operations return `MountedError::OperationFailed` wrapping `NodeNotExistErr` (packages/dioxus-native-dom/src/events.rs:179-193; packages/dioxus-native-dom/src/events.rs:222-225; packages/dioxus-native-dom/src/events.rs:239-244)
- WPT runner net errors are typed as `WptNetProviderError` (Io, DataUrl, DataUrlBase64, HandlerPanic) and logged with `warn!` (wpt/runner/src/net_provider.rs:107-111; wpt/runner/src/net_provider.rs:116-141)
- wasm_hello maps event-loop errors into `JsValue` (examples/wasm_hello/src/lib.rs:120; examples/wasm_hello/src/lib.rs:143-145)
- `gh_api` runs `subprocess.run(..., check=True)`, raising on non-zero exit (.github/scripts/wpt_diff_to_pr.py:263-270); CI matrix jobs set `fail-fast: false` (.github/workflows/ci.yml:339-340; .github/workflows/publish-browser.yml:43-44)

- escher-driver's command refusal is a typed value, `Refusal`: a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and a `Fault` only beside `malformed`; no `String` field, so a refusal cannot hold anything a call supplied; a `SessionError` is written from a fixed table likewise; MCP protocol errors are fixed codes and messages with no `data`; of a refusal only its cause's fixed name reaches a log.

**Graceful degradation (no panic):**
- Stale node ids in layout children are skipped rather than panicking (packages/blitz-dom/src/resolve.rs:255-259; packages/blitz-dom/src/resolve.rs:304-308); stale node ids resolve to `None` through `get` (packages/blitz-dom/src/tree.rs:89-97)
- `resolve` and hit testing return early with a warning when there is no DOM (packages/blitz-dom/src/resolve.rs:40-47; packages/blitz-dom/src/document.rs:2060-2067); `handle_ui_event` returns early when a document has no event target (packages/blitz-dom/src/events/driver.rs:229-234)
- `stylo_to_cursor_icon` returns a default cursor for `Auto` instead of panicking (packages/blitz-dom/src/stylo_to_cursor_icon.rs:6-12); an unknown alignment flag is mapped rather than panicking — to Taffy's `AlignContent::NORMAL` for content alignment, to none for item alignment (packages/stylo_taffy/src/convert.rs:483-484; packages/stylo_taffy/src/convert.rs:597-598)
- An unrecognized font format yields `Resource::None`, with a comment asking whether it should be an error (packages/blitz-dom/src/net.rs:368-371)
- `try_doc` returns `None` when the document is mutably borrowed (packages/dioxus-native-dom/src/events.rs:155-177); `try_element_to_node_id` returns an option (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
- Stale events after suspend are dropped (packages/blitz-shell/src/application.rs:61-69)
- Tests guard against panics from stale ids ("invalid SlotMap key used", "invalid key") and `unreachable!()` (tests/blitz-tests/tests/stale_interaction_state.rs:117-118; tests/blitz-tests/tests/animations.rs:4-7; tests/blitz-tests/tests/custom_widget_layout.rs:3-5; tests/blitz-tests/tests/stale_node_mapping.rs:118-136); a stale animation entry for a removed animated node is to be skipped safely on the next resolve (tests/blitz-tests/tests/animations.rs:26-27)
- Query APIs return `Result<Option<NodeId>>`; missing elements are `None` (tests/blitz-tests/tests/comment_layout.rs:37; tests/blitz-tests/tests/harness_smoke.rs:25); unknown fragment targets return `None`/`false` and leave scroll at 0 (tests/blitz-tests/tests/fragment_navigation.rs:107-111; tests/blitz-tests/tests/fragment_navigation.rs:140-145)
- The WPT runner catches per-test panics and converts them to a CRASH result (wpt/runner/src/main.rs:629-647); net handler panics are caught and mapped to `WptNetProviderError::HandlerPanic` with the panic message (wpt/runner/src/net_provider.rs:82-91); a harness that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest, otherwise TIMEOUT (wpt/runner/src/test_runners/harness_test.rs:163-181)

**Panic paths:**
- URL handling: `resolve_url` panics when a URL cannot be resolved against the base URL (packages/blitz-dom/src/document.rs:1276-1283); `set_base_url` unwraps `Url::parse` (packages/blitz-dom/src/document.rs:606-609); `launch_url` panics on an invalid URL and unwraps the fetch result and UTF-8 decoding (packages/blitz/src/lib.rs:51; packages/blitz/src/lib.rs:65-68)
- Network: building the reqwest client (`unwrap`), resolving the cache directory (`expect`), acquiring the per-host semaphore (`expect`), and reading a multipart form file (`expect`) (packages/blitz-net/src/lib.rs:90; packages/blitz-net/src/lib.rs:46; packages/blitz-net/src/lib.rs:53; packages/blitz-net/src/lib.rs:194-197; packages/blitz-net/src/lib.rs:442-445)
- Parsing: the HTML, XML and fragment parse drivers `unwrap` their `read_from` result (packages/blitz-html/src/html_sink.rs:115-118; packages/blitz-html/src/html_sink.rs:132-135; packages/blitz-html/src/html_sink.rs:155-158); image decoding uses `.expect("IO errors impossible with Cursor")` (packages/blitz-dom/src/net.rs:572-575)
- DOM internals: mutex locks on the font context are `.unwrap()`ed (packages/blitz-dom/src/font_metrics.rs:61; packages/blitz-dom/src/document.rs:1547); time reads go through the document's `Clock` and carry no epoch `unwrap` (packages/blitz-dom/src/clock.rs:14-19); invariant violations use `expect` with messages (packages/blitz-dom/src/events/keyboard.rs:417-422; packages/blitz-dom/src/stylo.rs:446); finding a non-anonymous ancestor panics when none exists (packages/blitz-dom/src/traversal.rs:130-134); `universal_accessors`, `layout_data` and `guard` panic when called on node kinds that lack the field (packages/blitz-dom/src/node/node.rs:151-155; packages/blitz-dom/src/node/node.rs:192-198; packages/blitz-dom/src/node/node.rs:354-361); `layout_style()` panics if the node has no computed styles (packages/blitz-dom/src/node/node.rs:1158-1167)

- Unimplemented paths: Stylo hooks, twelve dioxus event-data conversions and the accesskit_xplat macOS and Windows adapters for UiKit and WinRt handles reach `todo!`, `unimplemented!` or `panic!`; no physical key code hits a `todo!()`.

- Layout: a table root without styles panics ("Ignoring table because it has no styles") (packages/blitz-dom/src/layout/table.rs:217-219); a node flagged table root without a `TableContext` panics (packages/blitz-dom/src/layout/mod.rs:524-531); `unreachable!()` is used for states the code treats as impossible (packages/blitz-dom/src/layout/mod.rs:471; packages/blitz-dom/src/layout/construct.rs:876; packages/blitz-dom/src/layout/construct.rs:1040; packages/blitz-dom/src/layout/inline/layout.rs:732); anchor-positioning sizes and non-breadth `fit-content()` track limits hit `unreachable!()` (packages/stylo_taffy/src/convert.rs:122-124; packages/stylo_taffy/src/convert.rs:901-905); painting a node flagged as inline root without inline layout data panics (packages/blitz-paint/src/render.rs:850-857)
- Debug-only checks: under `debug_assertions`, layout-parent consistency is asserted and a missing child panics (packages/blitz-dom/src/resolve.rs:324-349); a `debug_assert` checks that compared nodes share the root (packages/blitz-dom/src/traversal.rs:340-343); ResumeReady arriving before the renderer is ready trips a debug_assert (packages/blitz-shell/src/application.rs:61-69)

- Shell and runtime setup: event loop construction, window creation, Android app setup, the JS runtime's context and extensions, the timer thread, the tokio runtime and the raw window handle `unwrap` or `expect` and panic on failure.

- Dioxus nodes: supported event-data conversions `unwrap` the platform-data downcast (packages/dioxus-native-dom/src/events.rs:47-129); `NodeHandle::node`/`node_mut` panic with "Node does not exist in the Document" (packages/dioxus-native-dom/src/events.rs:155-177); `element_to_node_id` unwraps (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
- Apps and examples: rdme unwraps fetch results and UTF-8 decoding (apps/readme/src/main.rs:196-212; apps/readme/src/readme_application.rs:84-85); AnyRender scene capture unwraps archive and file errors (apps/browser/src/capture.rs:75-78); network and file errors in `screenshot` are `unwrap()`ed (examples/screenshot.rs:41-42; examples/screenshot.rs:46-52; examples/screenshot.rs:138); GPU setup and rendering failures in `paint_bench` `expect` with messages "No compatible device found", "Failed to create vello renderer", "Failed to create wgpu device", "Failed to render to texture" (examples/paint_bench.rs:130; examples/paint_bench.rs:140; examples/paint_bench.rs:218; examples/paint_bench.rs:174; examples/paint_bench.rs:310); `preact_script` panics with `could not resolve {raw_path}: {err}` and `could not read {path}: {err}` (examples/preact_script.rs:20; examples/preact_script.rs:22); `multi_doc` panics on an unknown mode (examples/multi_doc.rs:108)
- WPT runner: failure to run `git rev-parse HEAD` panics (wpt/runner/src/report.rs:12-21); a Crash subtest status is `unreachable!()` in report conversion and expectations output (wpt/runner/src/report.rs:74; wpt/runner/src/report.rs:138); a test-file read error other than invalid UTF-8 panics (wpt/runner/src/test_runners/mod.rs:212); the attribute test runner unwraps the node it reads, panicking on a node id that does not resolve (wpt/runner/src/test_runners/attr_test.rs:135-136)

**Error reporting integration:** observed absent — searched: `counter!|histogram!|gauge!|info_span|debug_span|#\[instrument|tracing::span|sentry|opentelemetry` over the 15 s05 files; `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 s12 files.

---

## Logging & Monitoring

Detail: .andromeda/security-plan-detail/logging-monitoring.md — read whole.

**What is logged:**
- The browser logs successful loads at info with the resolved URL (apps/browser/src/document_loader.rs:121); load failures, script fetch failures and JS errors at error (apps/browser/src/document_loader.rs:152; apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244); external-browser open failures at error (apps/browser/src/nav.rs:36-38); persistence warnings are prefixed `history_store:` (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:298-300)
- Network logging (feature `tracing`) records the request URL as field `url`, plus `status` or `error` (packages/blitz-net/src/lib.rs:219-224; packages/blitz-net/src/lib.rs:245-252; packages/blitz-net/src/lib.rs:284-285; packages/blitz-net/src/lib.rs:310-315); the cache directory path is logged at info level as field `path` (packages/blitz-net/src/lib.rs:57-58)
- Resource load failures log structured fields `url`, `waiting_nodes`, `error` (packages/blitz-dom/src/document.rs:1471-1477)
- Asset-fetch logs print the full request with Debug formatting (packages/dioxus-native/src/assets.rs:48; packages/dioxus-native/src/assets.rs:53; packages/dioxus-native/src/assets.rs:60)
- JS console output (log/info/warn/error) goes to the `log` crate at debug level, target `js_console`, keeping stdout/stderr clean (packages/blitz-vibey-script/src/runtime.rs:29-51); with the `tracing` feature, uncaught JS errors are logged with `tracing::error!` (packages/blitz-vibey-script/src/runtime/events.rs:20-21; packages/blitz-vibey-script/src/document.rs:265-266)
- A devtools hover-highlight mode logs the clicked node instead of handling the click (packages/blitz-dom/src/events/pointer.rs:689-699)
- WPT runner: WPT_DIR value (info) and its absence (error) (wpt/runner/src/main.rs:473-478); glob failure (error) (wpt/runner/src/main.rs:298); net load errors with URL and path at warn, pending requests at debug (wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:110; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220); skips — quarantined (debug), non-UTF-8 (warn), unsupported testdriver (debug), unresolvable/unreadable refs (warn) (wpt/runner/src/test_runners/mod.rs:189; wpt/runner/src/test_runners/mod.rs:203; wpt/runner/src/test_runners/harness_test.rs:160; wpt/runner/src/test_runners/ref_test.rs:119; wpt/runner/src/test_runners/ref_test.rs:136; wpt/runner/src/test_runners/ref_test.rs:141)
- CI publish sets `CARGO_LOG: info` and runs `dx bundle --verbose --trace`, on upstream `DioxusLabs/blitz` only — the `release-cli` job is repository-guarded (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:37; .github/workflows/publish-browser.yml:157)

**Log format and backends:**
- Engine crates log through the `tracing` crate behind a `tracing` feature (packages/blitz-dom/src/lib.rs:26-29; packages/blitz-dom/src/layout/mod.rs:247-252); the macros compile only with that feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:929-935; packages/blitz-dom/src/util.rs:26-27; packages/blitz-shell/src/convert_events.rs:41-45; packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:104-111)
- dioxus-native's `tracing` feature also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net (packages/dioxus-native/Cargo.toml:69; packages/dioxus-native-dom/Cargo.toml:19)
- The WPT runner's logger is `env_logger::init()` (wpt/runner/src/main.rs:467)

- escher's own sink: both seven_guis binaries install `escher_telemetry::init`, a stderr-only allowlist subscriber — an engine target only seven allowlisted fields, the rest `[redacted]`; an `escher_` target all but eleven content-named fields; any other target dropped whole at every level, spans by the same rule; its reach is that sink only: upstream's `fmt::init()` subscribers, `env_logger` and the panic hook stay unscrubbed, and an allowlisted `log.file` can still carry a host path.

- Stdout output: examples print through `println!` and `eprintln!`; `escher-session` writes one uncoloured JSON line per command on stdout, its `mcp` role JSON-RPC lines only, usage and error messages on stderr, and no line holds anything the caller typed but the `mcp` role's echoed request `id`; debug dumps, timings and test diagnostics also go to stdout.

- Console logging calls are observed absent in the fixtures — searched: `console\.` over the 21 s02 files
- The agent-run test contract (`scripts/agent-run.sh`) logs harness metadata only: its JSON-line events in `target/agent-run/events.jsonl` carry no content-named field (`event ts outcome cargo_exit head selection files file test passed failed ignored booted boot_ts run state`) and no captured test output or panic text; its `target/agent-run/run.log` holds the raw cargo/libtest output, unscrubbed like `target/ci-logs/`, never printed by `logs`, gitignored under `target/` (scripts/agent-run.sh:80-142; as measured at escher-0.1.0/chunks/2026-10-06-stand-test-contract/report.md — a live `logs` read held 0 scrub-set keys)

- The cold-agent pipe (`scripts/cold-agent.sh`) logs run metadata only: its events and verdict carry counts, booleans and closed words, never transcript text, a tool argument or answer, a reply or the check; its server log, check, raw transcript and the live child's kept checks under `target/cold-agent/record/` are never printed, the transcript unscrubbed by design and gitignored, each committed copy's host paths masked.

**Monitoring + alerts:** observed absent — searched: `counter!|histogram!|gauge!|info_span|debug_span|#\[instrument|tracing::span|sentry|opentelemetry` over the 15 s05 files; a logging or monitoring crate is observed absent in the test crate — searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 s12 files.

> NOT YET MEASURED — security-event logging (auth, authorization or validation failures), log retention, access controls and tamper evidence: no slice gathered them.

---

## Compliance Controls

> NO RECORDED INTENT

---

## Security Anti-Patterns (NEVER do these)

> NO RECORDED INTENT

---

<!-- subjects: boundary-remote-content · boundary-script-execution · boundary-session-socket · boundary-mcp-stdio · auth · input · data-rest · api · deps · dep-serde-json · dep-png · dep-color · deps-supply-chain · secrets · errors · logging -->

## Security Decisions Log

> NO RECORDED INTENT
