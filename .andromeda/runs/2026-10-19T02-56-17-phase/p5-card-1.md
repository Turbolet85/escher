# P5 review — 2026-10-19-session-recording («Session recording»)

Plan: `escher-0.2.0/chunks/2026-10-19-session-recording/plan.md` (15 steps, 13 acceptance criteria, 17 gate entries) ·
research: `escher-0.2.0/chunks/2026-10-19-session-recording/research.md` · scope: `…/scope.md`

## Goal
An opt-in, launcher-named recording directory: `start … --record <dir>` (passed on to the host role) and `mcp --record
<dir>`. It is created `0700`, or refused `record-dir-not-private`, and each session gets one new `0600`
`check-{n}.json`. Each check is the calls as they crossed in, each with its acceptance and an FNV-1a 64 digest of the
driver's written answer. No screen reading is kept.
- A masked control's typed text becomes `MASKED_STAND_IN` with `masked:true`.
- The recorder sits inside `Session`, so the command line and MCP give byte-identical checks, and `serve` /
  `serve_mcp` signatures are unchanged.
- A strict bounded reader (400 steps, 1,048,576 bytes) and an in-process round trip on the four lean tasks, both
  layout modes.
- The CARRY: pan, the fling and a sub-document's config read the document's clock.

## Forks it rests on
- **Founder, by delegate the operator, PROVISIONAL** (inputs#I2, `.andromeda/runs/2026-10-19T02-56-17-phase/founder-forks.md`):
  F1 A · F2 B · F3 C. The wrap escalates each under «Boundary widening». The founder's own word may widen F3 to B,
  which the plan says.
- **Technical, the operator's** (P4 dialog): the recorder inside `Session`; a JSON array rewritten after each step;
  the reader and the round trip in this chunk.

## Confirm before the word — what is at rest, exactly
Under F3 C, «no id, name, value or pixel at rest» holds for every **answer**. The **calls** are kept as they crossed
in, because a replay needs them:
- a `click` / `type` / `scroll` / `screenshot` target's stable id;
- a `press`'s key and an `advance`'s `ms`;
- a `type`'s text into an **unmasked** control (a text input, a textarea — what the screen reads back as its value
  anyway).

A masked control's text never is. The security amendment states this class plainly. If the delegate's F3 answer meant
to exclude call ids too, say so: it would change F1/F3's design, since a check without targets cannot replay.

## Decisive leans (stated in the plan)
- v020-04 is not claimed. «Recorded-check replay» (`escher-0.2.0/working-route.md:29`) carries its CI replay leg.
  After the word, one dated `notes` line is written on v020-04 recording that this chunk ships the recording half.
- Two new session errors (13 → 15): `record-dir-not-private` and `recording-failed`. The second reports a failure
  after the call ran, the `AnswerTooLarge` precedent; the check file is then removed so no truncated check stands.
- The wire moves `v2` → `v3` for the one new fixed reply `RecordingFailed`.
- `call_of` moves from `mcp/result.rs` to `command.rs` (`pub(crate)`), so the reader and the server convert arguments
  one way.
- A public `is_masked_input` in dioxus-native-dom, the private snapshot reader using it.

## Top acceptance criteria
- A check is `0600` in a `0700` launcher-named directory, and a group-open one is refused with nothing written or
  spawned.
- A password sentinel occurs 0 times in the check, its step reads the stand-in with `masked:true`, and its length is
  nowhere.
- 0 accessible names, 0 control values and 0 snapshot-text lines are in the check; each answer is a 16-hex digest.
- A recording round-trips on every lean task, every digest and acceptance equal, in both modes. The CLI and MCP
  checks of the same steps are byte-identical.
- With recording on, `host_log` / `mcp_log` at trace, under both builds, read 0 sentinel, id, name, directory-name and
  digest needles. Span counts and fields are unchanged.
- `help` and `tools/list` are byte-identical with and without `--record`.
- The CARRY's clock checks, a11y unchanged, fast, doc, coverage ≥ 97 %, stand-repeat 3 of 3, CI green.

## Touchpoints
- 3 new files: `recording.rs`, `stand_session_recording.rs`, seven_guis `recording.rs`.
- 26 files to modify, across escher-driver (14), dioxus-native-dom (2), blitz-dom `iframe.rs`, the blitz-tests clock
  and session modules, the seven_guis tests (4) and the two CI scripts.
- No spec master is touched; seven expected amendments are listed for the wrap.

## Test commands (17)
- **Units:** escher-driver, dioxus-native-dom, blitz-dom `--lib iframe`.
- **Integration:** `stand_session_recording`, `virtual_clock_engine`, the standing session, mask and span targets,
  `-p seven_guis`.
- **Probes and legs:** the silence probe, fast, doc, stand-repeat, a11y, coverage, audit.
- **Operator:** hygiene → the pre-CI commit
  `chore(2026-10-19-session-recording): operator pre-CI commit, for the run this chunk's verdict reads` → fast,
  clean-tree check and push → `ci.py conclusion --wait 1800`.

**Baselines** on the untouched tree at `93336e88`:
- 9 green with their atoms;
- `--lib iframe` vacuous (0 selected);
- `stand_session_recording` red on an absent target;
- the silence probe red on absent files, its must-pass and must-fail controls holding over a scratch clone
  (`.andromeda/runs/2026-10-19T02-56-17-phase/p5-controls.md`).

## Concretization preview
None: no capability is claimed (v020-04 stays pooled with a dated note).

## Beyond the plan
none
