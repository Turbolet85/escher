# arch · cascade read · group 2 of 7

## Carried
- (.andromeda/architecture-detail/established-decisions.md:32) The re-pinned stand snapshot, diff, part and act checks must read through dioxus-native-dom's default `accessibility` feature, so no stand change may drop it from the graph seven_guis reaches.
  > `dioxus-native-dom`: accessibility, svg, system-fonts, text-transform-icu (packages/dioxus-native-dom/Cargo.toml:13)
  (.andromeda/architecture.md:42)
  > **[Default features]** each crate fixes its default feature set in its manifest — blitz-dom: svg, woff, accessibility, system-fonts, file-input, custom-widget, text-transform-icu;
- (.andromeda/architecture-detail/established-decisions.md:39) The widened stand boot (`cells`, the select, a held load) must keep a `ColorScheme::Light` headless document, and every re-pinned stand check must run in both the default incremental and the non-incremental layout mode.
  > Incremental layout defaults to `true` (packages/blitz-dom/src/config.rs:56; packages/blitz-dom/src/document.rs:505)
  (.andromeda/architecture-detail/established-decisions.md:39)
  > headless documents and the WPT viewport use `ColorScheme::Light` (examples/screenshot.rs:76-81; examples/paint_bench.rs:94-99; wpt/runner/src/main.rs:552-557)
  (.andromeda/architecture.md:46)
  > **[Document defaults]** incremental layout on, CSS media type `screen`, documents always HTML in no-quirks mode, a fixed set of Stylo prefs enabled, an empty `data:` default `DocumentUrl`, and headless documents and the WPT viewport in `ColorScheme::Light`.
- (.andromeda/architecture-detail/established-decisions.md:45) A load the stand holds in flight (the P4 fork) must be a load the engine already makes — a render-blocking stylesheet keeps `resolve` from styling while pending, an image is cached by URL — dispatched through a pluggable `NetProvider` as the tests do.
  > `<head>` stylesheets are render-blocking critical resources and `resolve` returns before styling while any are pending (packages/blitz-dom/src/mutator.rs:1288-1292; packages/blitz-dom/src/resolve.rs:52-63; tests/blitz-tests/tests/render_blocking_stylesheet.rs:3-8; tests/blitz-tests/tests/render_blocking_stylesheet.rs:53-55)
  (.andromeda/architecture-detail/established-decisions.md:45)
  > loaded images are cached by URL with concurrent requests queued on one fetch (packages/blitz-dom/src/document.rs:380-387; packages/blitz-dom/src/mutator.rs:1327-1352)
  (.andromeda/architecture.md:48)
  > **[Resource loading]** `<head>` stylesheets are render-blocking and `resolve` returns before styling while any are pending;
  (.andromeda/architecture.md:255)
  > tests dispatch loads through a pluggable `NetProvider` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/link_rel_attribute.rs:34-41)
- (.andromeda/architecture-detail/established-decisions.md:67) Replacing the flight booker's one-way / return buttons with a select must re-pin `stand_keyboard`'s Enter and Space tables to the activation rule: a select takes neither key as a Click, while the remaining buttons keep it.
  > keyboard activation — a pressed `Enter` whose target is the focused node, after the text-input and range arms, dispatches a Click on it when it is no disabled form control ([DOM semantics]) and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`, and a released Space on the focused node dispatches the same Click, through the new `handle_key_up`, for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled — a link takes no Space, and a text input and a range input take neither key there
  (.andromeda/architecture-detail/established-decisions.md:68)
  > a click focuses a button — `handle_click`'s submit arm became a button arm that matches a `button` of any type or an `input` of type `submit`, `reset` or `button`, focuses it through `generate_focus_events` / `set_focus_to`, then submits the form owner for a submit button and ends the walk matched, where before a submit button submitted without breaking the walk and every other button fell through to the unmatched clear of focus; a disabled form control still stops the walk first, a `disabled` attribute on any other element not, a click on a `select` focuses it through its own arm of the walk (below), and a click on any other element still clears focus
  (.andromeda/architecture-detail/established-decisions.md:64)
  > the builder writes AccessKit's `selected`, `toggled` and `invalid` on an element's node from `aria-selected`, `aria-pressed` and `aria-invalid`, read by value (a select's option excepted, its `selected` its chosen state, [DOM semantics])
  (.andromeda/architecture-detail/established-decisions.md:69)
  > repeated clicks within 500 ms and 2 px, read on the document's clock
- (.andromeda/architecture-detail/established-decisions.md:76) The cold-agent goal of `advance` on the one virtual clock and the select's typeahead window must read the harness's `VirtualClock` through `DocumentConfig.clock`, the stand setting no clock of its own beside it.
  > every engine time read — the overlay scrollbar fade, the double-click window, pan and the fling's generation, the smooth-scroll start, the fling and smooth frames — goes through `DocumentConfig.clock`, wall time when it is unset and a harness's `VirtualClock` headlessly
  (.andromeda/architecture.md:60)
  > the document's clock — every engine time read goes through `DocumentConfig.clock`, wall time when it is unset —
- (.andromeda/architecture-detail/established-decisions.md:77) The flight booker's new select and the driver's `type` on it must work within the select model as it stands — arrows, `Home` / `End` and typeahead, a click that only focuses, every change through `apply_select_choice` — and the stand must not rely on a popup, a clicked option, `multiple` or an `optgroup`.
  > a focused single-choice `select` (no `multiple`) that is no disabled form control takes a pressed `ArrowDown` / `ArrowUp` to the next / previous enabled option and `Home` / `End` to the first / last (with none chosen, `ArrowDown` reads as `Home` and `ArrowUp` as `End`), and a character pressed with no Ctrl, Alt or Meta as typeahead — the first enabled option whose label starts with the buffered text, compared in lower case, the buffer (`select_typeahead`) starting afresh for another select or once the document's clock has moved 1 s (`TYPEAHEAD_TIMEOUT`); every other key does nothing — Enter and Space open nothing, and a released Space activates no select
  (.andromeda/architecture-detail/established-decisions.md:77)
  > A click on a select clears the typeahead and the document's text selection and focuses it through `generate_focus_events` / `set_focus_to`; the walk's disabled gate stops a disabled select first, so it neither focuses nor acts (packages/blitz-dom/src/events/pointer.rs:857-870). Every choice that changes the chosen option goes through the one path `apply_select_choice` — the `CHECKED` write, the changed set, `input` then `change`, each carrying `select_value`, and a redraw — and a choice of the chosen option fires nothing (packages/blitz-dom/src/events/keyboard.rs:271-297). Its limits: no popup is painted, an option is not clicked (a driver `click` on one is refused `covered`), and a `multiple` select or an `optgroup`'s options are not modelled.
  (.andromeda/architecture.md:62)
  > The select model: every choice that changes the chosen option goes through the one path `apply_select_choice`, and a choice of the chosen option fires nothing; no popup is painted, an option is not clicked, and a `multiple` select or an `optgroup`'s options are not modelled.
- (.andromeda/architecture-detail/established-decisions.md:97) The select and the return-date field must take focus, the pointer and the activation keys by `disabled` read on a form control alone, and the Tab tables and `unkeyed_actionable` move with that focusability.
  > `disabled` is read one way, by its presence — bare, `"false"` or `"true"` — on a form control alone (ratified by the founder, 2026-10-18): `ElementData::is_disabled` is `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and a `disabled` attribute present, and focusability (`ElementData::flush_is_focussable`), the pointer-move gate, `ClickTarget::Disabled`, `handle_click`'s walk gate and the Enter and Space activation gates read it
  (.andromeda/architecture-detail/established-decisions.md:97)
  > the Tab walk (`focus_next_node` / `focus_prev_node`), autofocus and `DioxusDocument::unkeyed_actionable` move with focusability, and the recompute on a `disabled` write is unchanged;
  (.andromeda/architecture-detail/established-decisions.md:98)
  > a node the Dioxus bridge removes is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so a re-render-removed node no longer resolves
  (.andromeda/architecture.md:70)
  > `disabled` is read one way, by its presence, on a form control alone — focusability, the pointer and click gates and the Enter and Space activation read it; a node the Dioxus bridge removes is dropped with its descendants and no longer resolves
- (.andromeda/architecture-detail/established-decisions.md:104) `stand_keyboard`, whose tables this chunk re-pins, must keep pinning the `inert` skip in the Tab walk and in `unkeyed_actionable`.
  > `inert` is honoured by its presence on the element or an ancestor element (ratified by the founder, 2026-10-18): `Node::is_focussable` also reads false for an inert node through the crate-private `Node::is_inert`, an ancestor walk, so the cached `ElementData::is_focussable` stays per element and no mutator hook is added, and the Tab walk `focus_next_node` / `focus_prev_node`, autofocus (`process_added_subtree`) and `DioxusDocument::unkeyed_actionable` skip it
  (.andromeda/architecture.md:72)
  > `inert` is honoured by its presence on the element or an ancestor — not focusable, not hit, out of the accessibility tree and the snapshot;
- (.andromeda/architecture-detail/established-decisions.md:105) A re-pinned diff check that moves focus onto or off the new select must expect exactly the node that lost focus and the node that gained it in the changed set.
  > The `FOCUS_WITHIN` writes go through `snapshot_node_and` and mark no changed-set entry, so a focus move still marks exactly the node that lost focus and the node that gained it
  (.andromeda/architecture.md:72)
  > focus keeps `:focus-within` on its ancestors;
- (.andromeda/architecture-detail/established-decisions.md:106) The flight booker's select must hold its choice as its options' `CHECKED` state and, as a controlled Dioxus app, write `selected` from its `input` event's value, and the re-pinned snapshot, element-id and contrast checks read its options in the tree and its user-agent box.
  > A `select`'s choice is held as its options' `CHECKED` element state, so `:checked` matches the chosen option; an option created with a `selected` attribute is seeded `CHECKED` (packages/blitz-dom/src/node/element.rs:472-478).
  (.andromeda/architecture-detail/established-decisions.md:106)
  > Dioxus writes a dynamic `selected` again on each re-render, so a controlled app writes `selected` from its `input` event's value or a choice reverts
  (.andromeda/architecture-detail/established-decisions.md:107)
  > The accessibility builder keeps an option whose parent is a `select` although it computes `display: none` — `hidden`, `inert` and `visibility: hidden` still exclude it —, writes its `selected` from its chosen state with no `aria-selected` read, and takes its non-empty `label` attribute as its `label` where it has no `aria-label`
  (.andromeda/architecture-detail/established-decisions.md:107)
  > The user-agent stylesheet gives `select` the border, padding and background of `input, textarea` and `display: inline-block`
  (.andromeda/architecture.md:72)
  > a `select`'s choice is held as its options' `CHECKED` state, settled by HTML's selectedness algorithm, and a controlled app writes `selected` from its `input` event's value or a choice reverts.
- (.andromeda/architecture-detail/established-decisions.md:143) `stand::options`, edited here, must keep the stand's user-agent sheet reaching the document once and resolve any stand-held load against the Dioxus document's base URL.
  > Document `base_url` defaults to `dioxus://index.html` (packages/dioxus-native-dom/src/dioxus_document.rs:90-94); `DioxusDocument::new` adds no stylesheet of its own: the default user-agent sheet reaches it once, through `DocumentConfig::default()`, and a config that sets `ua_stylesheets` is used as given (packages/blitz-dom/src/config.rs:35-39; packages/blitz-dom/src/config.rs:74-93)
  (.andromeda/architecture.md:88)
  > **[Dioxus integration]** `DioxusDocument` defaults its `base_url` to `dioxus://index.html` and adds no stylesheet of its own, the default user-agent sheet reaching it once through `DocumentConfig::default()` and a config that sets `ua_stylesheets` used as given;
- (.andromeda/architecture-detail/established-decisions.md:162) The new screens the live run carries (Cells' contents, the select) must cross only on the ratified carriers with no `NodeId`, the stand's new boots must stay in seven_guis with escher-driver depending on no app, the paged `cells` screen must cross as the ratified part, and the wrong-call count must come from the `tool_call` span alone.
  > the socket carries the session's lifecycle and, since 2026-10-10-driver-cli, a driver call and its answer — a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no `NodeId` crosses
  (.andromeda/architecture-detail/established-decisions.md:162)
  > The session library takes the app's boot from its caller and depends on no app: no manifest under `packages/` names an example, and the one edge runs `seven_guis → escher-driver` (packages/escher-driver/Cargo.toml:13-18; examples/seven_guis/Cargo.toml:35).
  (.andromeda/architecture-detail/established-decisions.md:164)
  > one `tool_call` span per MCP `tools/call` request, holding `tool` and `ending` only
  (.andromeda/architecture-detail/established-decisions.md:165)
  > while text from a page, a call or a screen, a new carrier or a new exit still widens
  (.andromeda/architecture-detail/established-decisions.md:166)
  > a screen over the snapshot budget crosses them in parts — `snapshot`'s optional `part` (a whole number, 1 to `MAX_PART` = 1000) in, one part's text with the closed counts `part` and `parts` out, a part past the last the empty text, a screen within the budget answering whole as before, over MCP a paged answer its JSON line — with no new verb, carrier, exit, cause, session error, span field or dependency: a ratified boundary widening
  (.andromeda/architecture.md:94)
  > **[Driver session]** a session crosses processes on a `std`-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio — no third-party IPC crate and no TCP or UDP port; ids, names, masked values, snapshot text, diffs, shots and lint findings cross, never a `NodeId`; the library depends on no app, emits no event, installs no subscriber and reads no env var; a hosted session expires when idle; a new carrier or exit widens the boundary.
- (.andromeda/architecture-detail/established-decisions.md:172) If the acceptance's recorded run is read as the driver's kept check, it must replay through `Session::run` alone and stay owner-only and uncommitted — only the two committed stand Counter checks are the exception.
  > A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check — `std` only, no dependency added, unix only like the socket: the calls as they crossed (target ids, a `press` key, an `advance`'s ms, text typed into an unmasked control), a masked or unresolved `type` as one fixed stand-in, each answer as a 64-bit digest and kept whole — its line as the driver wrote it, masked as on every carrier, a shot's without its pixels — so a kept check holds the screens' ids, names, values, snapshot text, diffs and a lint's findings at rest, never logged and never named by a tool call; a kept check replays through `Session::run` alone (`Check::replay`), and the repository's own two scripted stand checks are committed in git, the one exception to owner-only.
  (.andromeda/architecture.md:96)
  > A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call; the repository's two committed stand checks are the one exception to owner-only.
  (.andromeda/architecture.md:238)
  > checks/ — the two committed kept checks `stand_replay` replays
- (.andromeda/architecture-detail/conventions.md:8) Every Rust unit the chunk edits — the stand tasks, its boot and the re-pinned checks — must pass the fmt and clippy legs as run by `ci-leg.sh`.
  > **Formatting and lints:** `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings` — then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` — each run as its `.github/scripts/ci-leg.sh` leg in CI and on the dev host
  (.andromeda/architecture.md:102)
  > **Formatting and lints:** `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings`, then upstream's blitz-dom clippy line only when that is green, each a `.github/scripts/ci-leg.sh` leg in CI and on the dev host;
- (.andromeda/architecture-detail/conventions.md:21) Any stand module or check the chunk adds or edits must open with a `//!` doc that states what it covers, the re-pinned checks' docs moving with their tables.
  > **Documentation:** modules and files open with `//!` docs
  (.andromeda/architecture-detail/conventions.md:21)
  > examples open with a `//!` comment describing what they demonstrate
  (.andromeda/architecture.md:108)
  > **Documentation:** modules and files open with `//!` docs, public items carry `///`, spec URLs are cited above or next to implementations, doc comments state the rationale for constants, and examples open with a `//!` comment describing what they demonstrate.
- (.andromeda/architecture-detail/conventions.md:33) A net stand-in the stand might take at the P4 load fork must keep the naming rule — a no-op provider prefixed `Dummy`, a holding one not — and keep its internals `pub(crate)` / `pub(super)`.
  > crate internals are `pub(crate)` / `pub(super)`
  (.andromeda/architecture-detail/conventions.md:33)
  > no-op implementations prefixed `Dummy` (packages/blitz-traits/src/net.rs:165-167; packages/blitz-traits/src/navigation.rs:14; packages/blitz-traits/src/shell.rs:65)
  (.andromeda/architecture.md:112)
  > crate internals are `pub(crate)` / `pub(super)`; no-op implementations are prefixed `Dummy` and internal JS natives `__blitz_`; consuming builder methods `with_…` / `without_…`;
- (.andromeda/architecture-detail/conventions.md:53) Every re-pinned or added stand check must cover one behaviour per test-target file under a `//!` doc, run its scenarios in both layout modes, share tables through the non-target `common` / `session_common` modules, and first assert its fixture produces the condition.
  > each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug
  (.andromeda/architecture-detail/conventions.md:53)
  > scenarios run in both layout modes `for incremental in [false, true]`
  (.andromeda/architecture-detail/conventions.md:54)
  > while the seven_guis stand checks state the tables and helpers they share once, in the module `tests/blitz-tests/tests/common/mod.rs`, which a check reads by declaring `mod common;`
  (.andromeda/architecture-detail/conventions.md:54)
  > tests first assert the fixture produces the condition under test
  (.andromeda/architecture-detail/conventions.md:54)
  > blitz-tests keeps one test target per file directly under `tests/`
  (.andromeda/architecture.md:121)
  > each integration test file covers one behavior and opens with a `//!` doc naming it; scenarios run in both layout modes; stand checks share helpers through modules under `tests/` that are no test target; tests first assert the fixture produces the condition; blitz-tests keeps one test target per file, except upstream's `tests/all.rs`, declared and held at `test = false`.
- (.andromeda/architecture-detail/conventions.md:61) `fixtures/mod.rs`, reached by this chunk, must keep stating the fixture boot and hold, the shared pages and `HeldNet` once, and a page the stand takes over from a fixture leaves it when it has one reader left.
  > A third such module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; what it holds is test-plan §3's (Crate-local test helpers), and a page with one reader stays in its check's file
  (.andromeda/architecture.md:123)
  > `tests/blitz-tests/tests/fixtures/mod.rs` states the fixture boot and hold, the shared fixture pages and `HeldNet` once, and a page with one reader stays in its check's file.
- (.andromeda/architecture-detail/conventions.md:67) The flight booker's select and any stand task change must stay a Dioxus `#[component]` with signal or store state and its CSS in a `const` raw string.
  > reactive state uses `#[derive(Store)]` with `#[store]` impls
  (.andromeda/architecture-detail/conventions.md:67)
  > components with `#[component]`
  (.andromeda/architecture-detail/conventions.md:67)
  > state via `use_signal`, `use_store`, `use_memo`, `use_future`
  (.andromeda/architecture-detail/conventions.md:67)
  > example CSS held in `const CSS: &str` raw strings emitted in a `style` element
  (.andromeda/architecture.md:125)
  > reactive state uses `#[derive(Store)]`; assets come through `asset!`; Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks, their CSS in `const` raw strings.
- (.andromeda/architecture.md:241) The cold-agent pipe stands as a `.sh` and `.ps1` pair, so the `stand` task set's new goals edited in `cold-agent.sh` must be read against its PowerShell twin.
  > cold-agent.{sh,ps1} (cold-agent pipe)

## Bindings
- **Select model ↔ DOM semantics ↔ Apps and examples**: the flight booker's select is driven by the engine's key, click and `apply_select_choice` path (.andromeda/architecture-detail/established-decisions.md:77) and holds its choice as `CHECKED` that a controlled Dioxus component must write back from its `input` event (.andromeda/architecture-detail/established-decisions.md:106), written as a `#[component]` with signal or store state (.andromeda/architecture-detail/conventions.md:67).
- **Scrolling and input (keyboard activation) ↔ DOM semantics (`disabled`, `inert`) ↔ Tests**: the Enter / Space and Tab tables `stand_keyboard` re-pins follow the activation rule (.andromeda/architecture-detail/established-decisions.md:67) gated by `disabled` on a form control and `inert` (.andromeda/architecture-detail/established-decisions.md:97; .andromeda/architecture-detail/established-decisions.md:104), held in the shared `common` module and run in both layout modes (.andromeda/architecture-detail/conventions.md:54; .andromeda/architecture-detail/conventions.md:53).
- **Document's clock ↔ Select model ↔ Driver session**: `advance` moves the one `VirtualClock` every engine time read goes through (.andromeda/architecture-detail/established-decisions.md:76), which also bounds the select's 1 s typeahead (.andromeda/architecture-detail/established-decisions.md:77).
- **Resource loading ↔ Dioxus integration ↔ Fixtures ↔ Visibility and naming**: a stand-held load must be one the engine makes (.andromeda/architecture-detail/established-decisions.md:45) under the stand config's base URL and UA sheet (.andromeda/architecture-detail/established-decisions.md:143), modelled on the fixtures' `HeldNet` (.andromeda/architecture-detail/conventions.md:61) and named by the `Dummy` rule (.andromeda/architecture-detail/conventions.md:33).
- **Driver session ↔ Recording**: the live run's new screens and paged `cells` cross only ratified carriers (.andromeda/architecture-detail/established-decisions.md:166), and a kept check of that run stays owner-only and uncommitted (.andromeda/architecture-detail/established-decisions.md:172).
- **Default features ↔ Document defaults ↔ Tests**: the re-pinned checks read the snapshot through default `accessibility` (.andromeda/architecture-detail/established-decisions.md:32) on a Light headless document in both layout modes (.andromeda/architecture-detail/established-decisions.md:39; .andromeda/architecture-detail/conventions.md:53).
