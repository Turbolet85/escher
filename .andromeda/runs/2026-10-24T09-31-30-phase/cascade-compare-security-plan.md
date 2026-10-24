# Cascade compare — security-plan · phase · 2026-10-24-upstream-sync-polish-and-ship
E items 126 · held 32 · lost at classify 10 · lost at strong 16 · cascade only 6 · body 45 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### api-security#2.1 · lost at classify · .andromeda/security-plan-detail/api-security.md:14
- (.andromeda/security-plan-detail/api-security.md:14) The audit fix (`cargo update -p tokio` to ≥ 1.53.3, semver-compatible) moves the crate under blitz-net's per-host semaphore (`tokio::sync::Semaphore`), so the outbound limit of 6 concurrent requests per host must hold unchanged after the update.
  > - **Control:** Rate limiting (outbound)
  >   - **Configuration:** Concurrent requests are limited to 6 per host (packages/blitz-net/src/lib.rs:24; packages/blitz-net/src/lib.rs:188-197)
  >   - **Stack reference:** blitz-net per-host semaphore

### api-security#2.6 · lost at strong · .andromeda/security-plan-detail/api-security.md:30
- (.andromeda/security-plan-detail/api-security.md:30) The settle and load-wait bound must survive the take: a harness settle never waits on a load in flight, the acting step waits at most `LOAD_WAIT`, and its `animating` reading takes the overlay scrollbar fade off the document's clock — the ground of `scrolling.rs` (a predicted conflict) and of upstream's animation-set lookup commit `1c5265a8b`, so a conflict resolution keeps the fade on the document's clock.
  > - **Configuration:** A harness settle (`Harness::settle`) reports a load in flight at once — `NotSettled` naming the class `Loads` — and never waits on one: it waits on no clock and moves none — its `animating` reading reads the overlay scrollbar fade off the document's clock — so the absent request timeout above cannot hang it, and its pass loop is bounded at 64 passes;

### api-security#2.6 · lost at strong · .andromeda/security-plan-detail/api-security.md:31
- (.andromeda/security-plan-detail/api-security.md:31) Under the virtual clock two shots of one state read the same pixels until an `advance`; a take that moves Taffy or Stylo may move pixels, but the clock rule itself (only `tick`, `advance_time` and `advance` move the fade on a harness-built document) must stand on the merged tree.
  >  — on a harness-built document the harness's `VirtualClock`, which only `tick`, `advance_time` and the driver's `advance` move — so two shots of one state read the same pixels until an `advance` moves the fade, a document built with no clock still fading by wall time (ratified by the founder, 2026-10-18) (packages/blitz-test-harness/src/screenshot.rs:37-103)

### api-security#2.9 · lost at strong · .andromeda/security-plan-detail/api-security.md:39
- (.andromeda/security-plan-detail/api-security.md:39) Upstream's `fb297a38c` rewrites `StyleThreading`'s doc comments above `DocumentConfig` in `packages/blitz-dom/src/config.rs` (net −2 lines before the field block), so under a take that carries it the cited `config.rs:64-67` of the `AbortSignal` cancellation is re-read and re-pointed, the control itself kept.
  > - **Control:** Cancellation
  >   - **Configuration:** Requests can carry an `AbortSignal` (packages/blitz-traits/src/net.rs:59; packages/blitz-traits/src/net.rs:74-77); aborting the configured `AbortSignal` cancels every in-flight fetch tied to the document (packages/blitz-dom/src/config.rs:64-67)
  >   - **Stack reference:** blitz-traits

### authentication-authorization#2.2 · lost at strong · .andromeda/security-plan-detail/authentication-authorization.md:27
- (.andromeda/security-plan-detail/authentication-authorization.md:27) Whatever shape the take has, the upstream-only workflows must keep the token grants they hold now. Upstream's delta past `17de049b1` touches no `.github/` path as read at P1. A merge that later brings in a workflow edit must keep every grant as written and widen none.
  > | RBAC / permissions (CI tokens) | wpt-post-results.yml grants `pull-requests: write`, `actions: read`, `contents: read` (.github/workflows/wpt-post-results.yml:8-11) | GitHub Actions `permissions` |
  > | RBAC / permissions (CI tokens) | wpt.yml grants `contents: read`, `pages: write`, `id-token: write` (.github/workflows/wpt.yml:18-21) | GitHub Actions `permissions` |

### authentication-authorization#2.6 · lost at classify · .andromeda/security-plan-detail/authentication-authorization.md:31
- (.andromeda/security-plan-detail/authentication-authorization.md:31) The driver session socket must come out of the take with its access model unchanged: filesystem modes only (`0700` directory, a group- or other-open one refused `StateDirNotPrivate`, `0600` socket). The sync adds no handshake, token or other access path and opens no new listener (scope: "No new crate, port, env var or listener of ours").
  > | Local IPC access (driver session socket) | No handshake, token, secret or peer-credential check: access is the filesystem's — the state directory is created `0700`, a present one open to its group or to others is refused (`StateDirNotPrivate`), and the socket file is set `0600`, so only the owning user reaches it; `start` does not check that the answering pid is the child it spawned (packages/escher-driver/src/host.rs:91-102; packages/escher-driver/src/host.rs:124) | `std` file modes (unix) |

### authentication-authorization#2.7 · lost at classify · .andromeda/security-plan-detail/authentication-authorization.md:32
- (.andromeda/security-plan-detail/authentication-authorization.md:32) The MCP stdio server must answer only the process that holds its stdin and stdout, and must bind nothing, after the take. The Dioxus 0.8 move reaches the stand that boots it, so this access model is re-proven by the standing MCP checks and is not widened.
  > | Local stdio (driver MCP server) | Since 2026-10-10-mcp-surface: no handshake, token, secret or peer check — the server answers only the process holding its stdin and stdout, the MCP client that spawned `escher-session mcp` as the invoking user, and the session it boots lives in that process; it binds nothing, so the socket's `0700` / `0600` modes do not apply (packages/escher-driver/src/mcp.rs:44-112) — the crossing ratified by the founder (2026-10-10, «Ратифицировать как есть») | process stdio |

### data-protection#1.1 · lost at strong · .andromeda/security-plan-detail/data-protection.md:8
- (.andromeda/security-plan-detail/data-protection.md:8) The kept-check medium has to come through the sync unchanged: `0600` files in a `0700` directory, answers masked as on every carrier (`MASKED_VALUE`, `MASKED_STAND_IN`), never a pixel, never logged. The take must not move the recorder's contract, and no merge resolution may change it.
  > - **Files (kept checks):** since 2026-10-19-session-recording, under `--record <dir>` on `escher-session` (`start`, the host role, the `mcp` role) or `Session::recording_into` in process, one `check-{n}.json` (`0600`) per recorded session in a `0700` directory the launcher named — a JSON array: the app label, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer (a shot's PNG included) and, since 2026-10-19-recorded-check-replay, the answer itself as `answer` — the line the driver wrote, masked as on every carrier (a password's or file input's value as `MASKED_VALUE`), a shot's as `{"width","height"}` with no `png` — holding call content (the target ids, `press` keys, `advance` ms and text typed into unmasked controls; a masked or unresolved `type`'s text as `MASKED_STAND_IN`) and the screens' readings at rest — ids, names, values, snapshot text and diffs, never a pixel — never logged, and never removed by escher but for its own check when a recording ends; the call-content class ratified by the founder (2026-10-19, F1 A and F2 B) and each answer kept whole, a shot's pixels excepted, ratified by the founder (2026-10-19, F3 B) (packages/escher-driver/src/recording.rs:105-138; packages/escher-driver/src/recording.rs:145-190)

### error-handling#1.1 · lost at strong · .andromeda/security-plan-detail/error-handling.md:9
- (.andromeda/security-plan-detail/error-handling.md:9) The fixed `covered` and `off-screen` texts state measured hit limits, and upstream's `f102663b8` edits hit testing in `node/node.rs`: on the take's tree the three measuring checks must still pass in both layout modes so each fixed text stays true; a moved hit reading is re-measured and recorded, never a text loosened or reworded without a recorded word.
  > — a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows (measured by `stand_act_obstructed::a_target_beside_a_box_clipped_on_one_axis_is_refused_covered`, both layout modes, the instance unchanged; tests/blitz-tests/tests/stand_act_obstructed.rs:560-605), and a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it (tests/blitz-tests/tests/stand_act_ids.rs:60) — and `off-screen`'s that a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows (tests/blitz-tests/tests/transformed_client_rect.rs:183-220)

### error-handling#1.1 · lost at strong · .andromeda/security-plan-detail/error-handling.md:11
- (.andromeda/security-plan-detail/error-handling.md:11) The sink allowlist re-check must find a refusal's log footprint unchanged by the take: the `cause` field's fixed name alone, no `Fault`, meaning, remedy or `Display` text, the standing capture checks at 0 needles.
  > and since 2026-10-07-driver-command-spans exactly one thing of a refusal reaches a log: its cause's fixed name, recorded as the `cause` field of the call's `escher_driver` span at one site for every refusal `Session::run` returns, whichever of `validate` and the executor returned it; no `Fault`, meaning, remedy or `Display` text is fielded

### input-validation#1.10 · lost at classify · .andromeda/security-plan-detail/input-validation.md:39
- (.andromeda/security-plan-detail/input-validation.md:39) Any reading the take adds or moves is judged by this rule: a closed-value field inside a ratified carrier needs no ruling, while authored text, an open value set, a new carrier or a new exit is a widening for the founder.
  > a new field of a closed value set inside an already ratified carrier is not a widening and does not come to him, while a field holding authored text, an open value set, a new carrier or a new exit still does

### input-validation#1.35 · lost at classify · .andromeda/security-plan-detail/input-validation.md:125
- (.andromeda/security-plan-detail/input-validation.md:125) This is how an earlier sync handled new input behaviour that came with an upstream merge: it went to the founder for ratification as upstream's behaviour. Any new input class in this take's delta is handled the same way.
  > A boundary widening that arrived with the 2026-10-10 upstream merge (upstream #1109), ratified by the founder

### input-validation#1.2 · lost at classify · .andromeda/security-plan-detail/input-validation.md:13
- (.andromeda/security-plan-detail/input-validation.md:13) Upstream's new root example `examples/multi_doc.rs` reads `MULTI_DOC_N`. It must be read against the examples' existing CLI-input rows, which record numeric input parsed with default fallbacks. P3 decides whether it is recorded and how.
  > `parse().ok()` with default fallbacks (examples/paint_bench.rs:60-63; examples/screenshot.rs:61-64)

### input-validation#1.15 · lost at classify · .andromeda/security-plan-detail/input-validation.md:56
- (.andromeda/security-plan-detail/input-validation.md:56) The driver's `covered` refusal reads the engine's hit test, and `f102663b8` changes hit testing (dropped nodes skipped). The refusal order and its leave-unchanged guarantee must still hold in both layout modes.
  > then `covered` (the hit at that point never reaches it) — and each refused call leaves the snapshot and both focus readings as they were, in both layout modes;

### input-validation#1.52 · lost at strong · .andromeda/security-plan-detail/input-validation.md:176
- (.andromeda/security-plan-detail/input-validation.md:176) `e4ca5b9b5` adds class attribute atoms on `ElementData`. `attr_parsed` must still return `None` on a parse failure.
  > Returns `None` on parse failure, as for `tabindex` (packages/blitz-dom/src/node/element.rs:483-486);

### input-validation#1.54 · lost at strong · .andromeda/security-plan-detail/input-validation.md:183
- (.andromeda/security-plan-detail/input-validation.md:183) `mutator.rs` is a predicted conflict. The autofocus reading in it must survive the resolution unchanged.
  > On a focusable element, under blitz-dom's `autofocus` feature, enables autofocus when present with any value but `"false"`: a bare, an empty and a `"true"` attribute each focus the element, and `"false"` is ignored (packages/blitz-dom/src/mutator.rs:1106-1118;

### input-validation#1.59 · lost at strong · .andromeda/security-plan-detail/input-validation.md:210
- (.andromeda/security-plan-detail/input-validation.md:210) Descriptions must keep the platform adapter as their one exit: no driver, MCP, field, log or span carrier.
  > the one exit is the platform accessibility adapter — the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries it;

### input-validation#1.60 · lost at strong · .andromeda/security-plan-detail/input-validation.md:213
- (.andromeda/security-plan-detail/input-validation.md:213) The merged closed-value reads keep their pin, and `aria-disabled` must not move the snapshot's `enabled`.
  > the snapshot's `enabled` keeps reading the `disabled` attribute by presence, so `aria-disabled` moves no snapshot reading; `accessibility_merged_states`' 3 tests pin the three, in both layout modes

### input-validation#1.109 · lost at strong · .andromeda/security-plan-detail/input-validation.md:371
- (.andromeda/security-plan-detail/input-validation.md:371) `286874cc1` edits `examples/seven_guis/src/tasks/cells.rs`. The cells tokenizer's recorded validation must still hold.
  > cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:28-30; examples/seven_guis/src/tasks/cells.rs:146; examples/seven_guis/src/tasks/cells.rs:229-232);

### input-validation#1.69 · lost at classify · .andromeda/security-plan-detail/input-validation.md:251
- (.andromeda/security-plan-detail/input-validation.md:251) `text_extent` reads inline layout data, which the inline split restructures. It must stay read-only and non-panicking.
  > an id that does not resolve, a node that is no inline root or one with no inline layout data reads `None` through `get_node`, never an index; it mutates nothing and has no log site, print, env read or panic path;

### input-validation#1.39 · lost at strong · .andromeda/security-plan-detail/input-validation.md:137
- (.andromeda/security-plan-detail/input-validation.md:137) `layout/construct.rs` is a predicted conflict. The inline SVG parse-error path in it must stay non-panicking.
  > Parsed with `parse_svg_image`; a parse error is logged and the element left without image data (packages/blitz-dom/src/layout/construct.rs:470-490)

### input-validation#1.51 · lost at strong · .andromeda/security-plan-detail/input-validation.md:173
- (.andromeda/security-plan-detail/input-validation.md:173) The `<ol start>` parse in the conflicted `construct.rs` must survive the resolution.
  > Parsed as `usize` and has 1 subtracted (packages/blitz-dom/src/layout/construct.rs:494-500)

### input-validation#1.74 · lost at strong · .andromeda/security-plan-detail/input-validation.md:266
- (.andromeda/security-plan-detail/input-validation.md:266) `scrolling.rs` is a predicted conflict (CARRY 4). The scroll-target clamp and the missing-node no-op must survive it.
  > Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:164-167); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:566-568)

### input-validation#1.22 · lost at strong · .andromeda/security-plan-detail/input-validation.md:86
- (.andromeda/security-plan-detail/input-validation.md:86) The URL fragment's lossy decode is also in the conflicted `scrolling.rs`.
  > Percent-decoded lossily before matching (packages/blitz-dom/src/scrolling.rs:805-808)

### input-validation#1.40 · lost at classify · .andromeda/security-plan-detail/input-validation.md:140
- (.andromeda/security-plan-detail/input-validation.md:140) `3954ecb85` adds Stylo `text-indent` keywords and `word-break: break-word`. CSS input must stay validated by parsing, with invalid declarations ignored.
  > Validated by parsing; invalid declarations are ignored per CSSOM (packages/blitz-dom/src/resolved_style.rs:162-182; packages/blitz-dom/src/resolved_style.rs:218-251; packages/blitz-dom/src/cssom.rs:716-757)

### secret-management#1.1 · lost at strong · .andromeda/security-plan-detail/secret-management.md:8
- (.andromeda/security-plan-detail/secret-management.md:8) The per-secret values the take must not move: if a take carries any edit to `publish-browser.yml`, `wpt.yml` or `wpt-post-results.yml`, each write-from-secret and its `always()` removal must stay paired and the cites must be re-read.
  > - **Production (CI):** secrets live in GitHub Actions secrets and vars:
  >   - A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:106-108; .github/workflows/publish-browser.yml:166-168)
  >   - An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:110-120; .github/workflows/publish-browser.yml:170-172)
  >   - Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:154-159)
  >   - A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:108-111)
  >   - The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)

### secret-management#2.4 · lost at strong · .andromeda/security-plan-detail/secret-management.md:24
- (.andromeda/security-plan-detail/secret-management.md:24) Upstream's delta edits `packages/blitz-dom/src/accessibility.rs` (`06ecec886`, +1 line), so this absence search's one cited `password` mapping (`accessibility.rs:519`) must be re-read on the take's tree and re-pointed if it moves.
  > - searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files; the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:519)

### threat-model-summary#2.1 · lost at classify · .andromeda/security-plan-detail/threat-model-summary.md:18
- (.andromeda/security-plan-detail/threat-model-summary.md:18) After the take the script boundary must not widen: the default fetcher still takes only `file:` and `data:` and returns `UnsupportedScheme` for any other scheme. This holds whichever of upstream's six Stylo-moved packages (`06ecec886`) reach `blitz-vibey-script`.
  > - **Trust boundary:** The default script fetcher supports only `file:` and `data:` URLs; any other scheme returns `UnsupportedScheme` (packages/blitz-vibey-script/src/fetch.rs:36-58). A `file:` URL is read from the local filesystem with `std::fs::read_to_string` (packages/blitz-vibey-script/src/fetch.rs:42-47). Every event object created by the runtime sets `isTrusted` to true (packages/blitz-vibey-script/src/dom/event.rs:118). Same-origin or CORS checks are observed absent — searched: `same.origin|same_origin|cross.origin|CORS` over the 32 s10 slice files.

### threat-model-summary#2.1 · lost at classify · .andromeda/security-plan-detail/threat-model-summary.md:17
- (.andromeda/security-plan-detail/threat-model-summary.md:17) The script entry point names the WPT runner's script execution (`wpt/runner/src/test_runners/mod.rs:119-132`). A take that carries `f8723f495` (WPT runner) and `9eeebb302` (`mimalloc` in the WPT runner) must re-read these cites and re-point them where they moved.
  > - **Entry point:** The `blitz-vibey-script` crate executes JavaScript found in, or referenced by, the document's `<script>` tags (packages/blitz-vibey-script/src/lib.rs:3-6; packages/blitz-vibey-script/src/document.rs:144-200). The `<body onload>` attribute text is wrapped into a function source string and evaluated (packages/blitz-vibey-script/src/runtime.rs:1526-1567). JS `fetch()` and ES module imports go through the same ScriptFetcher as `<script src>` (packages/blitz-vibey-script/src/state.rs:99-100; packages/blitz-vibey-script/src/document.rs:137-142), and `fetch()` URLs resolve against the document base URL (packages/blitz-vibey-script/src/runtime.rs:2357-2361). In the browser app, JavaScript execution is opt-in behind the non-default `javascript` feature (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:34-35). The WPT runner executes JavaScript from test files, including fetched script sources, through `ScriptDocument` (wpt/runner/src/test_runners/mod.rs:119-132; wpt/runner/src/test_runners/harness_test.rs:96-111). The React reference page states React passes Trusted Types values (`TrustedHTML` / `TrustedScriptURL`) straight through to `innerHTML` and URL-bearing attributes when enabled (examples/preact/core_dom_apis.html:418-424)

## Held

- api-security#1.0
- authentication-authorization#2.4
- authentication-authorization#2.5
- data-protection#2.1
- dependency-security#1.0
- dependency-security#2.1
- dependency-security#2.3
- dependency-security#2.4
- dependency-security#2.5
- dependency-security#6.1
- error-handling#2.1
- error-handling#3.1
- input-validation#1.53
- input-validation#1.57
- input-validation#1.58
- input-validation#1.61
- input-validation#1.62
- input-validation#1.63
- input-validation#1.64
- input-validation#1.65
- input-validation#1.66
- input-validation#1.67
- input-validation#1.68
- input-validation#1.79
- input-validation#1.80
- input-validation#1.82
- input-validation#1.83
- logging-monitoring#1.1
- logging-monitoring#1.2
- threat-model-summary#1.1
- threat-model-summary#3.1
- threat-model-summary#4.1

## Cascade only

- dependency-security#2.2
- input-validation#1.102
- input-validation#1.13
- input-validation#1.19
- input-validation#1.81
- secret-management#2.2
