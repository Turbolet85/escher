# What Escher can do

This statement is current with Escher 0.2.0, the `build/escher-0.2.0` branch. It is written for a consumer planning
work on Escher, Viola's frontend first. It says what Escher can do, the limits it states, how far its accessibility
reach is proven and what an MCP client carries onward.

The driver's own `help` is the live source for every verb, argument, node key, refusal cause and session error:
`target/debug/escher-session help` answers one line of JSON written from the program's tables, and an MCP client gets
the same from the `help` tool. Where this page and `help` disagree, `help` is right. A CI-script check,
`.github/scripts/test_capability_statement.py`, holds this page's verb and cause names and its counts equal to those
tables.

Every measured claim below names its witness: a check by file name, a CI run, or the chunk report that measured it.
Every limit says how it stands: pinned by a named check, asserted by no check, or unmeasured, with the
`.andromeda/residuals.md` line that owns it where one exists.

## What it can do

### The driver

One `escher_driver::Session` holds one headless app instance. Every call on it goes through `Session::run`, which
checks it with `validate` before anything runs. The same session is carried four ways:

- **in process**, by a Rust caller holding the `Session`;
- **over a local unix-domain socket**: `start` · `attach` · `stop` · `call`, wire `v3`, the host ending at `stop` or
  after 1,800 s with no request answered;
- **by the `escher-session` command line**: one command per run, one line of JSON on stdout, no colour and no ESC
  byte, exit `0` accepted · `1` refused · `2` usage · `3` session error;
- **by the `escher-session mcp` role**: an MCP server on its own stdin and stdout, newline-delimited JSON-RPC 2.0
  lines, spawned by the MCP client.

A kept check is a fifth medium, at rest (see Session recording and replay below).

The schema holds twelve verbs in three levels:

- asked of the held instance: `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint`;
- asked of the session: `start` · `status` · `stop`;
- asked of the tool itself: `help`.

A refused call runs nothing and changes nothing. It names one of eight refusal causes, each with a fixed remedy, in
this order: `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` ·
`fonts-unavailable`. A session edge ends a command with one of fifteen session errors, each with its kind, meaning
and fixed remedy. The command line has four exit statuses. Arguments are of seven kinds, each bounded: an id of 1 to
1,024 bytes, typed text of up to 4,096 bytes, a key from a fixed list of key names, `press`'s `shift` flag,
`advance`'s 1 to 60,000 ms, `start`'s app name of 1 to 32 bytes and `snapshot`'s `part` of 1 to 1,000. A socket request is at most 16,384 bytes,
an MCP request line at most 65,536 and an answer at most 1,048,576.

Witnesses: `stand_session_lifecycle`, `stand_session_state`, `stand_session_fresh`, `stand_act_refused`,
`stand_mcp_tools`; seven_guis' `host_binary`, `cli_commands`, `cli_flow` and `mcp_server`; the `help` text is
checked by escher-driver's own unit tests.

### Stable element ids

Every element of a Dioxus app on Escher has a stable id, computed on demand in four tiers: the author's HTML `id`,
else a path anchored at the nearest keyed element of the same component, else the element's component path, else
its document path. The id stays the same across a re-render, a remount, a fresh process and edits of the code around
the element. It is carried on the element's accessibility node as AccessKit `author_id`, and no engine `NodeId`
crosses any carrier. The check that every element an agent can act on is keyed, `DioxusDocument::unkeyed_actionable`,
runs in process only and has no driver verb.

Witnesses: `stand_element_ids`, `stand_id_persistence`, `stand_id_edits`, `stand_actionable_keys`,
`stand_accessibility_ids`.

### The snapshot

`snapshot` reads the screen as a tree of nodes, each with id · role · name · state · bounds, written under thirteen
node keys. Its text form is one line per node, nested by indent, under a 10,000-byte budget. A screen over the budget
is read in parts with `snapshot --part` (1 to 1,000): runs of whole lines that concatenate to the whole text. Each
acting verb returns the diff of the screen before and after its step, by stable id: the nodes added, removed and
changed, a changed node's `differs` naming the readings that moved.

Witnesses: `stand_snapshot`, `stand_snapshot_state`, `stand_snapshot_text`, `stand_snapshot_parts`, `stand_diff`,
`stand_act_diff`.

### Acting and settling

`click`, `type` and `scroll` name their target by stable id; `press` acts on what holds focus. Each acting verb is one
settled step and returns its diff. An id the screen does not read is refused `stale` when an earlier screen of the
session read it, else `not-found`; a `click` or a `type` on a target that cannot take it is refused `disabled`,
`off-screen` or `covered`, in that order. `type` replaces what a control holds, and on a `select` it chooses the first
enabled option whose label starts with the text, on a `multiple` select that option alone with no click on a row. A
`click` on a `multiple` select's row chooses it. `scroll` brings its target into view through every scrolling box that holds
it and answers `in_view`. The five acting verbs wait on a page load in flight for up to `LOAD_WAIT`, 5 s of wall
time, moving no time on the instance; a step that does not go quiet answers `settled: false` with its busy class.

Witnesses: `stand_act_ids`, `stand_act_keys`, `stand_act_filled`, `stand_act_disabled`, `stand_act_obstructed`,
`stand_act_scroll`, `stand_act_select`, `stand_act_states`, `stand_settle`, `stand_load_wait`.

### One virtual clock

Every harness-built document reads one virtual clock. `advance` moves the session's clock by exactly `ms` inside one
settled step, and the stand's Timer through its own time step. Two `click`s on one point with no `advance` of at
least 500 ms between them read as a double click.

Witnesses: `virtual_clock_engine`, `stand_virtual_clock`, `stand_act_timer`, seven_guis' `host_timer`.

### The headless screenshot

`screenshot` paints with no display and returns a PNG equal to the harness's raster: the whole screen, or with `--id`
the visible part of one element. It changes nothing. An app with no `sans-serif` font family is refused
`fonts-unavailable` before anything is painted. A password input paints masked in every document.

Witnesses: `stand_act_screenshot`, `stand_screenshot_repeat`, `password_painted_masked`.

### The visual lint

`lint` reads the held screen for six defects: overlap, clipped, truncated, off-screen, zero-size and contrast. It
returns one finding per defect, each holding a stable id, a closed defect word and numbers only, and changes nothing.
On the stand the four lean tasks read 0 findings at boot: an option or a group of a closed `select`, which reads the
select's own bounds, overlaps nothing, while a `multiple` select's rows are linted like any element. After twelve
Creates the last CRUD row, `crud-person-14`, reads `clipped` inside `crud-list`.

Witnesses: `stand_lint`, `text_extent`.

### Session recording and replay

A session started with `--record <dir>` keeps its answered calls as one owner-only replayable check, `check-{n}.json`
(`0600`) in a `0700` directory. A `type` into a masked control is kept as a fixed stand-in, never its text, and a
shot is kept without its pixels. `Check::replay` runs a kept check again through `Session::run` alone and stops at the
first step whose acceptance or answer differs. The two committed Counter checks under `tests/blitz-tests/checks/` are
the one public exception to owner-only.

Witnesses: `stand_session_recording`, `stand_replay`, seven_guis' `recording`.

### Engine behaviours every document gets

Escher changed the engine, for every document and not the stand alone, in these ways:

- **a range input model**: arrows, `Home` and `End` step it, a pointerdown sets it, each change fires one `input`
  event;
- **keyboard activation**: Enter and Space on a focused button and its kin dispatch a click;
- **click focus**: a click focuses a button or a select;
- **a single-choice `select` model**: arrows, `Home`, `End` and typeahead;
- **a `multiple` `select` model**: its rows have boxes of their own and take a click, plain, with Ctrl or Meta and with
  Shift; its arrows, `Home` and `End` choose a row alone, with Shift span and with Ctrl or Meta move the current row; a
  Space flips the current row; typed text chooses its match alone; each change fires one `input` and one `change`, a
  Dioxus handler reading the set through `evt.values()`;
- **`optgroup`**: its options are in its select's list, chosen by keys and text, a disabled group's rows skipped and left
  out of a form's entries;
- **`inert`** is honoured: an inert subtree is skipped by Tab, passed through by a hit and left out of the
  accessibility tree, and a node that holds focus when it comes under `inert` gives focus up at the mutator's flush, with
  no `blur` fired;
- **the tree's focus** is the window when the focused node was not built, so the tree never names a node it does not
  hold;
- **`disabled`** is read by its presence on a form control alone;
- **`change`** fires on a checkbox and, where its state moved, on a radio, after their `input`;
- **a native checkbox or radio** states its own checked state as `toggled` where its role is checkable;
- **a transform** moves what a box clips: `visible_region` reads a clipping box where its transforms show it, and a box
  under a transform with no inverse takes no hit.

Witnesses: `range_input_interaction`, `stand_act_range`, `stand_keyboard`, `select_interaction`, `stand_act_select`,
`inert_subtree`, `focusability_updates`, `checkable_change_event`, `accessibility_states`, `visible_region_transformed`,
`hit_non_invertible_transform`, `open_state`.

The snapshot, its text, parts and diff, and the actionable-key check stand behind the default `accessibility`
feature, which escher-driver names unconditionally.

### The stand it is proven on

Everything above is proven on the 7GUIs stand. Its four lean tasks (counter, flight booker, timer, CRUD) are booted
headless and offline at a pinned 800 × 600 viewport, scale 1.0, Light, with one bundled font, DejaVu Sans. Author ids
stand on all seven tasks.

### Building against it

Escher is a Rust 2024 workspace with `rust-version` 1.91.0. Its version, 0.2.0, is kept apart from the crates' shared
workspace version, 0.3.0-beta.4. The engine stack stays at Stylo 0.22.0, Taffy 0.15 and Dioxus 0.7.3; the upgrades
waiting upstream are next-version work. The escher crates set `publish = false`, so a consumer reaches them from the
repository, not from crates.io.

## Stated limits

### Readings the tool states as off

The driver's own refusal texts state three readings that are off:

- **`covered` beside a box clipped on one axis.** A box clipped on one axis stops a hit at its whole padding box, so an
  element lying outside it along the axis it does not clip can read `covered` though it shows. Pinned by
  `stand_act_obstructed` and `hit_clipped_at_scrolling_box`.
- **`covered` on a box with no width or height.** Such a box is hit at its centre on the element that holds it, so it
  can read `covered` though nothing covers it. Stated in the tool's text; asserted by no check.
- **An element in a rotated box's corner.** Under a rotation a box's visible part reads as the bounding box of its
  rotated padding box, so an element in a corner it does not show can read in view. Pinned by
  `visible_region_transformed`.

### Pinned by a check

- **The boolean model's two limits.** A mixed toggle reads unpressed, and a grammar or spelling error reads invalid.
  These are pinned at unit and engine level only: dioxus-native-dom's unit tests and the engine's accessibility
  checks. No stand screen shows either.
- **`text_extent`, which the lint reads.** RTL and vertical text read as left-to-right horizontal lines, and a
  transformed box reads at its client rect. Pinned by `text_extent`. Vertical writing modes are in no default feature
  set.
- **The focus fixup fires no `blur`.** A node that holds focus when it comes under `inert` gives focus up at the
  mutator's flush, and no DOM `blur` or `focusout` fires. Pinned by `inert_subtree`.
- **The `select` model's limits.** No popup is painted, and Enter and Space open nothing on a single select; an option of
  a closed select is not clicked (a click on one is refused `covered`, under a group too); only a `multiple` select's
  rows take a click. Pinned by `select_interaction` and `stand_act_select`.
- **`change` on a text input and a range.** They fire `input` and no `change`. Pinned by `text_range_no_change_event`.
- **A shot's PNG bytes.** They repeat byte for byte on one host and differ between linux x86_64 and macOS aarch64, as
  measured at CI#39225375577. A check compared byte for byte across operating systems holds no `screenshot` step.

### Asserted by no check

- **Unpaged diffs and findings.** An acting verb's diff and a lint's findings are not paged: on a large screen either
  can pass 10,000 bytes, held only by the 1,048,576-byte answer bound. `snapshot` alone reads a screen in parts. As
  measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md.
- **The `select` model's other limits.** `size` is unread, so a `multiple` select lists every row; a group's label is
  named in the tree and unpainted; Ctrl+A is not modelled; `press` holds no Ctrl, so a set of rows that are not adjacent
  cannot be built through the driver. A controlled app must write `selected` back, or a choice reverts. A single-choice
  select shows its chosen option's label alone. A closed select's contrast pair is unmeasured.
- **The bounded record of ids.** The session keeps the last 4,096 ids its screens read, to tell `stale` from
  `not-found`. Past that bound a once-read id can read `not-found`.
- **Selectors that never match.** `:focus-visible`, `:valid`, `:invalid`, `:required` and `:target` always read false.
- **The Dioxus bridge.** Focus and blur events are not queued. IME events are not handled. Animation and transition
  event data are `unimplemented!()`, among twelve event-data conversions that are.
- **Loading pages.** No verb takes a URL. The engine reads `file:` URLs with no path restriction and has no request
  timeout; the load wait bounds the driver. There is no same-origin or CORS check. A document handed no providers runs
  on the no-op defaults.

### Platform and carriers

- **Unix only.** The socket host, its client and session recording are unix only and answer `Unsupported` elsewhere.
  A session held in process and the MCP stdio role run elsewhere too.
- **The session state directory.** It is named by the caller, owner-only, holds the socket alone, and is removed at
  `stop` or idle expiry.
- **WebAssembly.** The wasm builds of the examples are browser-target builds through Trunk. The driver does not run on
  them.

### Telemetry

Escher's telemetry has two driver spans, `command` and `tool_call`, whose fields hold closed words, booleans and counts
only, never an id, name, typed text or screen content. The sink writes to stderr only, and at the default `warn` it
prints no driver line. Records from targets outside its two prefix sets are dropped at every level, a third-party
WARN or ERROR included; the rest are scrubbed. The scrub's limits:

- the chained std panic hook prints the raw panic message;
- `log.file` on a bridged `js_console` record can carry a host path;
- the upstream apps' `fmt::init()` and the WPT runner's logger are unscrubbed.

Both binaries of the seven_guis package log `service.name=seven_guis`. There is no OTel export or trace propagation,
and no log schema or log file. Engine tracing is behind each crate's `tracing` feature, while escher's spans are
ungated. A paged `part` read through the host's sink, `serve cells` at `trace`, reads 0 id and 0 name needles under
both builds, as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md.

Witnesses: `stand_act_spans`, `telemetry_scrub`, `telemetry_drop`, `telemetry_stdout_silent`,
`telemetry_panic_hook`, seven_guis' `host_log` and `mcp_log`.

### Not built

- A settle verb, or a reply naming the busy source.
- A snapshot field saying an element reacts to a click.

### Unmeasured

- Windowed end-to-end runs, and typed text in the windowed binary's log.
- Reduced-motion handling, observed absent.
- The engine's Web Platform Tests conformance. One report-only reading exists: `css/cssom-view` moved from 966 to 967
  subtests passed of 2,292 run.

## Accessibility reach

The target is tier 0: WCAG SC 2.1.1 (keyboard), SC 1.4.3 (contrast) and SC 2.4.3 (focus order). No other criterion
is claimed as met. The three are proven headless on the stand: by `stand_keyboard` (each lean task's Tab order, and
Enter and Space on each stand button), `stand_contrast` (every stand control's text and background pair, row for row)
and `stand_accessibility_ids`.

The `a11y` CI leg runs fourteen files and read 14 · 84 · 0 · 0 (result lines · passed · failed · ignored) on
CI#40134371519. It is no required status check. Eight of its files also run three rounds in `stand-repeat`.

Contrast: the stand's accent controls are white on `#3a5ce5` at 5.47, on its hover shade at 6.83 and on its active
shade at 8.27, against SC 1.4.3's 4.5. The lint's `contrast` defect uses the same reader and the same exempt rule.

What is not reached:

- **What an assistive technology reads is unmeasured.** No stand binary builds a platform accessibility adapter
  (probe 0, as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md). The windowed witness is owned by a
  `.andromeda/residuals.md` line whose disposition reads `re-carried:0.2.0`; it is owed at an active assistive
  technology, which this version does not reach. The platform adapters have their
  own gaps: Android's `set_focus` and `set_window_bounds` are empty, Windows' `set_focus` and macOS'
  `set_window_bounds` are no-ops.
- `aria-activedescendant` is unread (SC 4.1.2, outside tier 0), pinned by `accessibility_active_descendant_unread`.
- Descriptions (`aria-describedby`, `aria-description`) reach the platform adapter only, never the snapshot.
- A `role` attribute reaches the tree through the engine's one role-name mapping. The role, name and state mappings
  are stated in `.andromeda/a11y-plan.md`, not here.
- No skip link, focus ring, screen-reader or windowed reach is claimed.

## What an MCP client carries onward

The `escher-session mcp` role's trust boundary is the MCP client that spawned it and holds its stdin and stdout. There
is no authentication beyond holding that stdio, no listener and no port. The socket carrier likewise has no
authentication beyond its `0700` state directory and `0600` socket.

A tool result carries:

- element ids and accessible names;
- control values, with a password's and a file input's value as a fixed mask;
- snapshot text and diffs;
- a shot as an MCP `image` block (a PNG, base64), with any password input painted masked;
- a lint's findings: ids, closed defect words and numbers.

No description crosses any driver carrier, since descriptions go to the platform adapter only.

What the client does with a result is the client's, a model provider included. Escher's boundary ends at the stdio
it answers on.

The measured instance is the 2026-10-25 cold-agent live run. Through the operator's own login, the stand's six apps
reached the model provider: ids, names and values in snapshot texts, paged parts and diffs, Cells' contents, the
`flight-type` select's options and value, a lint's findings and one screenshot. It was one run with twelve tools: it
passed, all six apps met, in 66 tool calls with 0 wrong against a bar of at most 5 % wrong, one `lint` and one
`screenshot` among them, through the `claude` client 2.1.308. Its verdict holds counts, booleans and closed words only.
The earlier 2026-10-15 run drove the four lean tasks with the eleven tools of its day.

## Where each claim is proven

The checks named above live in `tests/blitz-tests/tests/` and `examples/seven_guis/tests/`, one file per behaviour.
They run in these CI legs (`.github/scripts/ci-leg.sh`):

- **`test`**: the whole workspace, on linux, macOS and windows.
- **`a11y`**: the fourteen accessibility files.
- **`stand-repeat`**: the `stand_*` files, fifteen named blitz-tests targets and seven_guis' eight process checks, three
  rounds under a zero-failure budget; with 48 files and ten named targets it read 198 · 837 · 0 · 21 on CI#40134371519.
  Among the fifteen: `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`,
  `checkable_change_event`, `text_range_no_change_event` and `transformed_client_rect`, whose
  `a_target_inside_a_translated_scrolling_box_reads_inside_its_visible_region` holds the transformed box's reading.
- **`ci-scripts`**: the CI-script checks, this page's pin among them.

The 0.2.0 requirement record is `WITNESSES_020` in `stand_requirement_sweep`: nine rows, 46 witnesses.
