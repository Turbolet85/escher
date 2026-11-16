# P5 review — 2026-11-16-error-text-in-the-answer · «Error text in the answer»

**Plan:** `.andromeda/cache/docs/2026-11-16-error-text-in-the-answer/plan.md` (rev 2) — open it in an editor. No plan
stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-error-text-in-the-answer`.

## Goal
An acting answer's `caused` carries, per record, the text an agent needs to say what failed and where, for every
source. This follows the founder's answer and purpose, `inputs#I1`. Each item carries:
- a panic's payload and its file, line and column;
- an error's top line, its whole cause chain, and which boundary took it — the root one or one of the app's own;
- a log record's message, source (`app` · `framework` · `engine`), target, level, location and fields.

Every field passes the sink's own scrub by name. Each text is cut at 1024 B, with the bytes cut said. At most 16 items
go in an answer, and the rest are counted in `caused_past` by kind and source. The text rides every carrier `caused`
rides today and no log line or span field. There is no new verb, carrier, exit, cause or session error.

## Measured first (P3, `doc:2026-11-16-error-text-in-the-answer/research`)
- **Message lengths.** A temporary probe ran 39 steps over the stand and a real-failure fixture, under both builds; it
  was removed after its reading.
  - The longest message of an admitted class read 97 B, and at most 2 such records crossed a step. That is the basis
    for the 1024 B cut and the 16-item count.
  - No stand step caused a record at `warn` or above, under either build.
  - Below `warn`, the framework reads up to 4106 B. The engine logs a record on every focus move and every attribute
    write, holding attribute values as typed and `NodeId` text.
- **Every error boundary is readable.** The root boundary through its context. An app's own boundary through
  dioxus-core's boundary factory, provided at the document's root scope before the first build. Each error is read
  with its chain. Unread: an app that provides its own factory (fullstack's), which is stated in the help.

## Top acceptance criteria
- **The founder's question.** On a fixture that fails as apps fail — a `Vec` index past its end, an `unwrap` of a
  failed parse, a three-link `Err` chain at the root and under the app's own boundary — the written answer alone
  carries, as values:
  - the payload or message;
  - the location (the fixture's own `file!()`, the failing line);
  - the chain in order;
  - the boundary;
  - the handler.
  This holds in both layout modes, on `Session::run`, `serve_mcp`, the session socket and a kept check that replays.
- **The scrub.** `password` and `value` fields read `[redacted]` by name, while the message and other fields cross
  whole. A typed password occurs 0 times, and its snapshot value reads `MASKED_VALUE`.
- **The bounds.** A 3000 B record arrives cut by 1976 B, 16 items at most, the rest in `caused_past`, and the whole
  line is under 1,048,576 B.
- **Logs.** Under escher's own sink at `trace` the sentinels and payloads occur 0 times. The `command` span keeps its
  eight fields.
- **The hook.** A panic with no capture open runs the replaced hook exactly once and captures nothing. The hook
  installs once per process (your note at P4).
- **No moves.** `stand_cross_os_readings`' digests and the committed Counter checks are unchanged.

## Touchpoints
- **New:**
  - `tests/blitz-tests/tests/stand_act_error_text.rs`
  - `tests/blitz-tests/tests/capture_panic_hook.rs`
- **Modified:**
  - escher-telemetry: `capture.rs`, `format.rs`, `lib.rs`
  - dioxus-native-dom: `boundary.rs`, `dioxus_document.rs`, `lib.rs`
  - escher-driver: `caused.rs`, `execute.rs`, `json.rs`, `schema.rs`, `schema_tests.rs`, `explore.rs`, `lib.rs`
  - `tests/blitz-tests/tests/stand_act_errors.rs`

## Test commands (14 entries; baselines read at P5)
- **Unit tests** of the three crates: green, 21 · 3 · 121.
- **The new targets:** red — the target this chunk writes does not exist yet.
- **The moved standing check `stand_act_errors`:** green, 5.
- **Six companions:** green.
- **Three telemetry checks:** green.
- **Four seven_guis process checks:** green.
- **`fast`:** seen green.
- **`stand-repeat`:** green, 857.9 s.
- **`coverage`:** green — escher-driver 97.94 %, dioxus-native-dom 90.24 %, blitz-test-harness 96.08 %.
- **Operator entries:** hygiene, push, then `ci.py` with a 2400 s wait. The diff is OS-sensitive: a panic's file is
  spelled with `\` on Windows, and the check compares it with `file!()`.

Four one-shot controls at implement: the scrub, the cut, the hook's chaining, and the factory's provision.

## Your word at P4
«Capture's chaining hook (Recommended)», with the note that the plan checks that a panic outside a capture prints
exactly as before and that the hook installs once — step 7, `capture_panic_hook`.

## Leans the plan takes (decided by an artifact; say if you want any reversed)
1. **An engine record's message reads `[redacted]`.** The sink's own field rule hides an engine target's message: it
   is a field outside `SAFE_FIELDS`. P3 measured why: engine messages hold attribute values as typed (so a
   controlled password's) and `NodeId` text. App and framework messages cross whole.
2. **The framework and the engine are admitted at `warn` and above.** The framework already is (obs ·
   decision/capture-framework-targets-warn-up). Below `warn` they cross every step and exist under the workspace build
   alone, which would make stand answers, and the cross-OS digests, differ by build.
3. **A panic's payload crosses as its message,** though the sink redacts `panic.payload` on its log line. The founder's
   points name «a panic with its location» among the messages that cross (`inputs#I1`).
4. **`caused` becomes one item per record.** `count` is dropped and `caused_past` is added as a new result field on the
   five acting verbs (the program's own fixed help text). This reshapes `contract/acting-verb-results/caused`.
5. **An app's own message holds what the app wrote,** a masked control's value included if the app writes it. The
   scrub hides by field name and nothing else (`inputs#I1`). This is stated in the help, and as an Expected amendment
   on `decision/mask-holds-on-every-carrier`.
6. **No framework or engine record is on the fixture.** None at `warn` or above is reachable on a package-alone build.
   Each source's admission and scrub is pinned by the capture's unit table instead. The scope's acceptance was amended
   at validation-1 to say so (intent-incomplete).

## Validation
- **Mechanical checks:** all pass.
  - The gate fence parses, 14 entries.
  - `planlint` reads 0 hits. One «unchanged» claim, on the cross-OS digests and the committed Counter checks, holds by
    the P3 measurement.
  - `inputs#I1` reads unchanged, 9 citations.
  - Research's lists parse: 16 items.
  - Baselines are written on the 10 new entries.
- **Validation-1:** aligned, after the one intent-incomplete scope amendment above.

## Capabilities
None claimed. The pool's three belong to later route entries: v030-11 «Layout on the carriers», v030-12 «A component
in isolation», v030-15 «Cold-agent bar».

## Expected amendments (wrap) — pattern «Boundary widening» (rul-izni57qv)
The founder's own answer was relayed verbatim by the overseer (`inputs#I1`). The hook was your own answer at P4.
- **architecture:**
  - `contract/acting-verb-results/caused`;
  - `decision/driver-library-side-effect-free`, `decision/driver-spans-only` and
    `decision/escher-driver-spans-ungated` (the hook exception);
  - `contract/dioxus-native-dom-api/root-error-mark`;
  - `decision/accessibility-feature-gating`.
- **security-plan:**
  - `decision/caused-answer-crossing`;
  - `decision/root-boundary-err-as-screen-content`;
  - the kept-check content rows and `decision/no-screen-content-in-logs`;
  - `decision/mask-holds-on-every-carrier`'s stated limit.
- **obs-plan:**
  - `decision/capture-framework-targets-warn-up` (the engine added);
  - `decision/headless-stand-no-sink`'s exception;
  - `contract/telemetry-scrub-set/error`.
- **test-plan:**
  - `contract/driver-session/acting-verbs`;
  - `contract/session-recording/exploration-checks`;
  - `decision/root-boundary-err-on-own-session`.

## Beyond the plan
| proposal | owner |
|---|---|
| `docs/capabilities.md`'s account of `caused` and its text (it names no `caused` today) | the existing entry «Capability statement» |

Apply? (yes / review / cancel)
