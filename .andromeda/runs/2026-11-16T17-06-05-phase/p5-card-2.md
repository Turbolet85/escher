# P5 review — 2026-11-16-error-text-in-the-answer · «Error text in the answer» (after your review)

**Plan:** `.andromeda/cache/docs/2026-11-16-error-text-in-the-answer/plan.md` (rev 4) — open it in an editor. No plan
stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-error-text-in-the-answer`.

## What your review changed
- **An engine record at `warn` and above crosses with its message as text.** One scrub now holds for every source: a
  field named in `CONTENT_FIELDS` reads `[redacted]`, and every message crosses, the engine's included. The sink's own
  log lines are unchanged.
- **The engine's `warn` and `error` sites, enumerated: 43.** That is 47 `git grep` hits, less 3 `compile_error!` and 1
  doc comment. 6 sites in 5 files interpolate an attribute value or text, and each has the value moved into a field
  the scrub hides, by a direct edit of the engine (new step 2, each flagged upstreamable):

  | site | value moved | field |
  |---|---|---|
  | `blitz-dom/src/events/pointer.rs:1144` (link `href` won't parse) | the raw `href` and the document URL | `href` · `url` |
  | `blitz-dom/src/form.rs:156` (form scheme not implemented) | the `action` scheme | `url` |
  | `blitz-dom/src/mutator.rs:1542` (iframe url unresolvable) | the raw `src` | `src` |
  | `blitz-dom/src/layout/mod.rs:248` (text node laid out alone) | the text, in a field named `data` | renamed `text` |
  | `dioxus-native/src/assets.rs:53` · `:60` (asset fetch) | the request's `Debug` (its URL) | `request` |

  The other 37 hold fixed text, closed values, the shell's winit variants, or the failure's own error text:
  html5ever's fixed strings with `exact_errors: false`, a script's uncaught error, an io error. Node ids may cross.
  research's §The engine's sites lists all 43.
- **Real framework and engine records, reached on a fixture.** A temporary probe, removed afterwards; logs
  `probe-reach-{tracing,package}.log`.
  - **Engine, workspace build.** A click on a block link with `href="http://[::1"` gives a `WARN` at
    `blitz_dom::events::pointer`, 205 B, the raw `href` in its message — the first site above. An invalid inline style
    value gives a `WARN` at `blitz_dom::node::element`, «Invalid property value», its value already in a `value` field.
    Both are absent in the package-alone build, where the engine's `tracing` is off.
  - **Framework, both builds.** A component whose render returns `Err` gives an `ERROR` at `dioxus_core::scope_arena`,
    99 B: «Error while rendering component `…`: invalid digit found in string».
- **The founder's fixture now carries both** (step 7). The engine case asserts per build through a new documented
  `blitz_dom::TRACING_COMPILED`: in the workspace build each warning crosses whole, its moved values `[redacted]` and
  the raw `href` 0 times in the answer; in the package-alone build there is no engine item.
  - A new gate entry runs the target with `--features dioxus-native-dom/tracing` locally. CI's workspace test leg runs
    the engine branch, and `stand-repeat` the package-alone one.
  - A blitz-tests feature was rejected: Cargo will not let a package's features enable a dev-dependency's.
- **A fifth one-shot control:** restoring the `href` interpolation reads the engine case red.

## Unchanged from card 1
- **The rest of the plan:**
  - per-record items with `caused_past`;
  - the 1024 B cut and the 16-item count (the longest admitted message measured is now 205 B, the engine link before
    its edit);
  - every boundary read through the root factory;
  - the chaining hook with your P4 note (step 8);
  - every carrier;
  - nothing in logs or span fields;
  - no new verb, exit, cause or session error.
- **Card 1's other leans (2–5)** stand:
  - framework and engine at `warn` and above;
  - a panic's payload crosses as its message;
  - `count` dropped for `caused_past`;
  - a message holds what its author wrote.

## Top acceptance criteria
- **The founder's question.** The written answer alone carries, as values, on both layout modes and every carrier:
  - each failure's message, its location and source, and its chain and boundary where it has them;
  - the panics, the root and own-boundary `Err`s and the framework's render error;
  - in the workspace build, the engine's two warnings.
- **The scrub.** One rule for every source by field name. `password`, `value`, and the engine's moved `href` and `url`
  read `[redacted]`. The typed password and the raw `href` occur 0 times. The snapshot reads `MASKED_VALUE`.
- **The engine sites.** The 6 carry the value in a hidden field, with a fixed message.
- **Bounds, logs, hook, no-move:** as on card 1.

## Test commands (15 entries)
Baselines, all on the untouched tree:
- **The two new entries:** red, their target not written yet.
- **The moved standing check:** green, 5.
- **The companions:** green — six targets, three telemetry checks, four seven_guis process checks.
- **`fast`:** seen green.
- **`stand-repeat`:** green, 857.9 s.
- **`coverage`:** green — escher-driver 97.94 %.
- **Operator entries:** hygiene, push, then `ci.py` with a 2400 s wait.

## Validation
- **Mechanical checks:** all re-run after the re-plan and pass.
  - The fence parses, 15 entries.
  - `planlint` reads 0 hits. The «unchanged» claim on the cross-OS digests and the Counter checks holds: no stand step
    causes a `warn` or above record.
  - `inputs#I1` reads unchanged.
  - Research's lists parse: 21 items.
  - All 11 new entries are baselined.
  - The engine paths exist.
- **Validation-1:** aligned. The scope was amended twice to carry your direction and the reach measurement.
- **The row read was not re-run.** The engine as a source was in the pinned scope ("the app, the UI framework and the
  engine"), and its scrub and direct-change rows are among the picked rows: arch ·
  `contract/telemetry-scrub/engine-targets`, `decision/direct-engine-changes-flagged-upstreamable`; security ·
  `decision/engine-target-safe-fields`.

## Capabilities
None claimed (v030-11, v030-12 and v030-15 belong to later entries).

## Expected amendments (wrap) — pattern «Boundary widening» (rul-izni57qv)
These are card 1's, plus the engine's:
- architecture `decision/direct-engine-changes-flagged-upstreamable` gains the six sites and `TRACING_COMPILED`;
- security-plan `decision/engine-target-field-allowlist` notes the moved values;
- obs-plan §8 notes the engine sites' fixed messages;
- test-plan §9 records that the founder's target asserts per build.

Who answered: the founder (`inputs#I1`, relayed verbatim by the overseer), and you at P4 and at this review.

## Beyond the plan
| proposal | owner |
|---|---|
| `docs/capabilities.md`'s account of `caused` and its text (it names no `caused` today) | the existing entry «Capability statement» |

Apply? (yes / review / cancel)
