# Fan-out results — 2026-10-07-command-and-refusal-schema

Seven detectors, one per spec source, each given its document, the chunk report and its drift-base entries (15 detector entries over the seven prompts: architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 — the drift-base's own 15). test-plan, obs-plan and a11y-plan also received their keyed-contract render (`{doc}-contracts.md` in this run dir); architecture reads `NOT MIGRATED` and security-plan carries no keyed-contract section, so their line was dropped. Every return was taken from its agent's own final message, entity-decoded and probed: 0 entities left in each, and decoding changed none. Dispositions are the orchestrator's (amendment-flow §Validate).

## Verdicts

- **architecture** — 4 proposals (D-arch-resources 4; D-arch-decisions no drift). Beside the list: a trailing comment block with the D-arch-decisions verdict and nine sites swept and left unchanged, among them a judgement call on the three "no driver, CLI or MCP command exposes … yet" clauses of the Dioxus DOM bridge contract.
- **security-plan** — 3 proposals (D-security-input 3, graded `escalate` by the detector's own severity; D-security-auth and D-security-deps no drift). Beside the list: a leading comment block stating that D-security-input's invariant HOLDS — no external-input surface is added and the one check added is present and total — and that the three proposals are the report's expected amendments 3, 4 and 5; eight sites swept and left alone.
- **design-system** — `proposals: []`. Beside it: one comment line with the basis (no UI rendered; every `tokens` flag `n/a`). The return parses whole as YAML, so nothing was stripped and no twin was saved.
- **layout-templates** — `proposals: []`. Beside it: one comment line with the basis (no user-facing surface or region added; the `escher-session` CLI entry unaltered). No twin, as above.
- **test-plan** — 4 proposals (D-tests-coverage 4; D-tests-framework and D-tests-obs-harness no drift). Beside the list: a leading comment block with the two clean verdicts, a statement that the code-side invariant of D-tests-coverage holds, and eight sites swept and left unchanged.
- **obs-plan** — 2 proposals (D-obs-instrumentation 1 · D-obs-stack 1; D-obs-pii no violation). Beside the list: a trailing comment block with the D-obs-pii verdict and three sites left unchanged.
- **a11y-plan** — `proposals: []`. Beside it: a comment block with both detectors' bases and a note that the report's expected amendment 8 lies outside both detectors, naming `a11y-plan.md:270` as its site. No twin, as above.

Total: 13 proposals over four documents.

## Validation — the six checks
1. **Playbook.** Ten proposals match "Accurate this-chunk addition" (routine): each names a fact the report's Changes carry as this chunk's work — the schema's public items, the re-export list, the module count, the two moved citations, the silence of the new modules, the cause names, the workspace count — landing in an existing section, with the detector's invariant holding. "Registry over-reach" does not govern A1: §Standard Contracts states contract shapes, and the two chunks before this one registered theirs there (`Session::act`, `Harness::settle`). None is a boundary widening — the schema is held in process and nothing new crosses the socket — and none touches a PROVISIONAL mark: the answer S2 records is the operator's own, given at the plan's forks (plan §Provenance), so it is not provisional. The three `escalate` grades on security-plan are the detector's severity for a VIOLATION of D-security-input; the detector itself reports the invariant holding, and each of the three is a change the plan's reviewed `Expected amendments (wrap)` list names (entries 3, 4, 5) — the recorded direction settles them. Disposition: apply, no halt. No two rules collide.
   Re-derivation: T1 and T2 carry coordinates the report does not (`refusal.rs:187-314`, `schema.rs:277-444`, `command.rs:234-572`). Both were rejected as proposed, T3 with them as their dependent, and the three facts re-raised by the orchestrator from its own measurement (check 5): the three `#[cfg(test)]` lines read 187 · 277 · 234 and the files end at 314 · 444 · 572, which is what the proposals said. No coordinate written into a body was taken from a proposal.
2. **Cross-contradiction.** None. A1, A2 and A3 edit one contract line in compatible parts; T1 and T2 state one count in two sections.
3. **Intent-consistency.** The report does not diverge from the route entry or the plan's acceptance criteria; its six deviations are each justified in it, and the scope record holds no line (`scope: clean`, 0 recorded).
4. **Absence needs evidence.** The agents' "swept and left unchanged" notes were not relied on: the caught-all claim is the cascade sweep's (`cascade-dispositions.md`), run over the masters, key files, leaves, curation homes and judgment bases after the last body edit and again after the leaves. Line profile known before any hit was dispositioned (`splice.py summary`): the longest amended lines run to 8 415 chars (`architecture.md:133`) and 7 153 (`test-plan.md:324`), each read by an offset window.
5. **Expected amendments (8 entries).** 1 architecture Driver session — proposed (A1, A2, A3) · 2 architecture Existing Scopes — proposed (A4) · 3 security-plan §Input Validation — proposed (S1) · 4 security-plan §API Security — proposed (S2) · 5 security-plan §Error Handling — proposed (S3) · 6 test-plan §4 and §9 — proposed (T1, T2, T4; T1 and T2 re-raised as above) · 7 obs-plan §3 and §4 — proposed (O1, O2) · 8 a11y-plan §7 — proposed by no detector; raised by the orchestrator as routine, the report carrying the cause's stated meaning (Y1).
6. **Disproved claims.** The report lists none.

One correction to the report came out of this pass: its site search for the crate's unit-test count (`\b13 unit|error\.rs 3`) did not match the keyed contract's wording, "escher-driver unit tests 13", and the report called that key file "no change". T3 found the site. The report carries the correction, marked.

Escalations: 0.

## Dispositions
Numbering is the order of each doc's list below. The applied text was re-derived from the report and the site as read, never pasted from a `change` line.

- **architecture** — A1 Driver session, the schema stated: apply · A2 the re-export clause and its citation `lib.rs:21-34` → `27-49`: apply · A3 the "Not built" clause: apply · A4 Existing Scopes → escher-driver, eight modules and `lib.rs:23-34` → `29-49`: apply.
- **security-plan** — S1 §Input Validation, the new row: apply · S2 §API Security, the settle row: apply · S3 §Error Handling, the `Refusal` bullet: apply.
- **test-plan** — T1 §1 the count and the three files: rejected as proposed (coordinates outside the report), re-raised and applied · T2 §4 the three pins: rejected as proposed (the same coordinates), re-raised and applied · T3 key Session lifecycle → `session-proof`: rejected with its primary, re-raised from the orchestrator's own read of the key file's line 11 and applied (`registry.py check`: 0 defects) · T4 §9 the re-count link: apply.
- **obs-plan** — O1 §4 the observed-absent bullet: apply · O2 §3 the silence clause: apply.
- **a11y-plan** — Y1 §7 the `enabled` clause, raised by the orchestrator (check 5): apply.

Applied: 10 proposals as proposed and 4 raised by the orchestrator (3 of them re-raises of rejected proposals) — 14 edits' worth over five masters and one key file, recorded in five sidecar entries (one per amended doc). Rejected as proposed: 3. Escalated: 0.

## The parsed lists
Each return as it arrived, whole and unaltered.

### architecture

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session"
    change: >-
      Add, after the non-unix sentence and before the "Not built" clause, the command and refusal schema the crate now exports, held in process only (no item reachable from the socket, no `Session` method takes one): `validate(call: &Call) -> Result<Command, Refusal>` (packages/escher-driver/src/command.rs:161; it receives no `Session`) over `Call { verb: String, args: Vec<(String, ArgValue)> }` with `ArgValue` `Text(String)` · `Number(i64)` · `Flag(bool)`; `Command` exactly `Snapshot` · `Click { id }` · `Type { id, text }` · `Press { key, shift }` · `Advance { ms: u32 }` with `spec() -> &'static VerbSpec`; `Key` (12 variants mirroring `KEY_NAMES`: `tab` · `enter` · `escape` · `backspace` · `delete` · `space` · `arrow-up` · `arrow-down` · `arrow-left` · `arrow-right` · `home` · `end`; `ALL`, `name`, `from_name`); the `'static` table `VERBS` (5 verbs in order `snapshot` · `click` · `type` · `press` · `advance`) of `VerbSpec { name, help, args, fields }` / `ArgSpec { name, kind, required, help }` / `FieldSpec { name, kind, always, help }`, looked up by `verb(name) -> Option<&'static VerbSpec>` (exact name only); arguments — `snapshot` none, `click` `id`, `type` `id` + `text`, `press` `key` + optional `shift` (false when absent), `advance` `ms`; `ArgKind` `Id` (1 to `MAX_ID_BYTES` = 1024 bytes) · `Text` (0 to `MAX_TEXT_BYTES` = 4096 bytes) · `Key` (one name of `KEY_NAMES`) · `Flag` · `Milliseconds` (1 to `MAX_MILLISECONDS` = 60000); result fields — `snapshot` `text`; the four acting verbs `settled` (flag), `busy` (one of `BUSY_CLASSES` `render` · `layout` · `loads`, present only when `settled` is false), `added` (nodes), `removed` (ids), `changed` (nodes), and `advance` also `advanced_ms`; `FieldKind` `Text` · `Flag` · `Milliseconds` · `Busy` · `Ids` · `Nodes`; `NODE_FIELDS` `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `focused` · `bounds`; a step that ran and did not go quiet is a result (`settled: false` with its class), not a refusal; `Refusal` (private fields; `new(cause)`, `malformed(fault)`, `cause()`, `fault()`; `Display` `{name}: {remedy}` or `{name} ({fault}): {remedy}`; `std::error::Error`) over the closed `Cause` set, `CAUSES: [Cause; 8]` — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable`, each with a fixed `name()`, `meaning()` and `remedy()` — and `Fault` `Missing` · `Unnamed` · `Repeated` · `WrongKind` · `OutOfBound` (each naming the schema's own argument name where it carries one); no `String` field in `Cause`, `Fault` or `Refusal`; `validate` answers once per call in a fixed order — the verb by exact name (absent → `unknown-verb`), then the call's arguments in the order passed (a name the verb lacks → `malformed` / `Unnamed`, a repeated name → `Repeated`), then the verb's arguments in schema order (`Missing`, `WrongKind`, `OutOfBound`), and a listed verb with no `Command` shape reads `unknown-verb`; `Command`, `Call` and `ArgValue` derive `Debug`, `Clone`, `PartialEq`, `Eq` and their `Debug` prints the id and text they hold; the crate root exports this `Command` while `start` still takes a `std::process::Command`; the modules carry no `cfg` gate and no serialization crate — as measured at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md
    sidecar: >-
      2026-10-07-command-and-refusal-schema — Driver session contract registers the in-process command and refusal schema (validate, Call/ArgValue/Command/Key, VERBS and its specs, kinds and bounds, Refusal/Cause/Fault and the eight causes).
    rationale: >-
      Report Changes → Symbols / APIs (report.md:9-16) lands 22 new public items re-exported from the escher-driver crate root; the Driver session contract registers only the lifecycle and `act` surface and names none of them, so each is an unregistered public API. The report's own Expected amendment 1 (report.md:35) names this site.
    basis: .andromeda/architecture.md:133
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session"
    change: >-
      The opening re-export clause should read: the crate re-exports 32 names under `#![deny(missing_docs)]` — `Session`, `SessionError`, `serve`, `start`, `attach`, `stop`, `Hello` and `Started` from the lifecycle modules; `ArgValue`, `Call`, `Command`, `Key` and `validate` from `command`; `Cause`, `CAUSES`, `Fault` and `Refusal` from `refusal`; `ArgKind`, `ArgSpec`, `FieldKind`, `FieldSpec`, `VerbSpec`, `VERBS`, `verb`, `KEY_NAMES`, `BUSY_CLASSES`, `NODE_FIELDS`, `MAX_ID_BYTES`, `MAX_TEXT_BYTES` and `MAX_MILLISECONDS` from `schema`, all private modules; and `Busy` and `Settled` from blitz-test-harness (packages/escher-driver/src/lib.rs:27-49) — replacing the 10-name list and its `lib.rs:21-34` citation.
    sidecar: >-
      2026-10-07-command-and-refusal-schema — Driver session contract's re-export clause grows from 10 names to 32 and its citation moves lib.rs:21-34 → 27-49.
    rationale: >-
      Report Counts / qualifiers moved (report.md:25-26): the root re-export list grows 10 → 32 (+5 command, +4 refusal, +13 schema) and the cited range `21-34` maps to `27-49` (`#![deny(missing_docs)]` 21 → 27, re-exports 30-34 → 39-49). The clause states the 10 as the crate's re-exports and cites the old lines.
    basis: .andromeda/architecture.md:133
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session"
    change: >-
      The "Not built" clause should drop "a verb set" as unbuilt and read: Not built: anything that runs a `Command` (no `Session` method takes one, so the verb table is stated and validated in process and executed nowhere), a verb, a settle verb or busy-source reply on the socket (the wire still reads `hello` and `stop` only; a step settles in process only, through `act`), act by id, CLI JSON, an MCP tool, an idle expiry — keeping the rest of the clause and its measured-at trailer, with the schema's report appended.
    sidecar: >-
      2026-10-07-command-and-refusal-schema — Driver session "Not built" clause no longer lists the verb set; it now lists the executor and any verb on the socket.
    rationale: >-
      Report Changes → Symbols / APIs: `VERBS` holds 5 verbs and `validate` checks calls against it (report.md:11-14), so "Not built: a verb set" is retired; the same bullets keep the rest true — "no item is reachable from the socket, and no `Session` method takes any of them" and "The lifecycle wire still reads `hello` and `stop` only" (report.md:9, :16).
    basis: .andromeda/architecture.md:133
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → escher-driver"
    change: >-
      The row should read as the driver's session and its command and refusal schema, with eight private modules — session, host, client, wire and error as now, plus command (`ArgValue`, `Call`, `Key`, `Command`, `validate`), refusal (`Cause`, `CAUSES`, `Fault`, `Refusal`) and schema (the `'static` verb table `VERBS` with its argument and field specs, the kinds and bounds, the key, busy-class and node-field lists), the three new ones with no `cfg` gate and held in process only — re-exported at the crate root beside blitz-test-harness's `Busy` and `Settled`; the dependency clause stays as it is; citation `packages/escher-driver/src/lib.rs:23-34` → `packages/escher-driver/src/lib.rs:29-49`.
    sidecar: >-
      2026-10-07-command-and-refusal-schema — Existing Scopes escher-driver row names eight private modules (adds command, refusal, schema) and its citation moves lib.rs:23-34 → 29-49.
    rationale: >-
      Report Changes → Crates / modules (report.md:17): the crate gains three private modules, "eight where there were five" (basis lib.rs:29-37); Counts (report.md:24, :26) names this row and maps its cited range `23-34` → `29-49`. The row lists five modules and describes the crate as the session only. Report's Expected amendment 2 (report.md:36).
    basis: .andromeda/architecture.md:257
    dependent-of: D-arch-resources

# D-arch-decisions: no drift. Report Dependencies reads "none added, none bumped" with Cargo.toml, Cargo.lock,
# deny.toml and the crate manifest byte-identical (report.md:18); the new modules are `std`-only `'static` data,
# install no subscriber, read no env var or clock, carry no `tracing`/`log` dependency and name no app, stand type
# or `dioxus-native-dom` feature (report.md:69-70, :80, :83) — all as §Established Decisions → Driver session
# (.andromeda/architecture.md:101) requires. The wire still carries lifecycle messages only.
#
# Swept and left unchanged (the claim stays true per the report's Changes):
# - architecture.md:101 §Established Decisions → Driver session — "lifecycle messages only", no env var, no tracing,
#   no idle expiry; its `lib.rs:13-19` citation is unmoved (report.md:26).
# - architecture.md:146 §Occupied Resources → Network ports and listeners — socket serves `hello`, `stop` only (report.md:16).
# - architecture.md:154 Names and :224 Publishability — no crate, binary or feature added (report.md:16-17).
# - architecture.md:150 — `escher_driver::start` spawns the caller's `std::process::Command`; still true. The crate root
#   now also exports a schema `Command` (report.md:64) — a naming hazard for readers, not a retired claim.
# - architecture.md:136 §Standard Contracts → Dioxus DOM bridge, three sites — "no driver, CLI or MCP command exposes"
#   the snapshot / its text, the diff, the check "yet". Judgement call: the schema now declares a `snapshot` verb with a
#   `text` result and `added`/`removed`/`changed` fields, but nothing executes a `Command` and nothing reaches it from a
#   socket, CLI or MCP tool (report.md:9, :43), so I read all three as still true and proposed nothing. If the
#   orchestrator wants them qualified ("declared in the driver's schema, executed nowhere"), they are three dependents.
# - architecture.md:141 §Standard Contracts → CI contracts — the cold-agent stub's own causes `not-found` · `disabled` ·
#   `not-pressable` · `malformed` and tool `press`; the stub is untouched (report.md:8) and its set is not the driver's.
# - architecture.md:207, :210 §Project Intent — README's "driver ... planned"; README not in Changes.
# - No hit in architecture.md for the unit-test count (13 → 25) or the workspace count (598 → 610); those sites are in
#   test-plan and distillations, outside arch.
```

### security-plan

```yaml
# D-security-auth: no drift — the report touches no identity, token, key or secret; `validate` takes no `Session`, the socket's access model and the env reads are unchanged (report.md:9, :16, :80).
# D-security-deps: no drift — "Dependencies: none added, none bumped"; manifests, Cargo.lock and deny.toml byte-identical (report.md:18).
# D-security-input: no unvalidated boundary. The chunk adds no external-input surface (report.md:9, :16, :43) and the one caller-input check it adds, `validate`, is present and total (report.md:44).
#   What is out of step is the doc: it has no row or entry for the new mechanism. The three proposals below are the report's Expected amendments 3, 4, 5 (report.md:37-39).
#   Proposals 2 and 3 sit outside §Input Validation; they are additions, not duplicate occurrences of a retired claim, so they carry no dependent-of.
# Swept and left alone, still true per the report (nothing executes a verb; no item reachable from socket, CLI or MCP):
#   security-plan.md:25, :193 (wire is `hello`/`stop` only); :115 x3 ("no driver, CLI or MCP command exposes ... yet"); :116, :117 ("reaches no ... command"); :116, :377 ("no command can type yet"); :294 (no env read).
#   Those lines will need re-reading when a later chunk makes the `snapshot` / `type` verbs executable — the schema now names them.
# Line cites in the change text are the report's own; I did not re-read the source files. The report gives no line for schema.rs.
proposals:
  - detector: D-security-input
    severity: escalate
    section: §Input Validation
    change: >-
      Add a row after the three driver-session rows (after "Driver session label (escher-driver)"):
      | Driver command schema (escher-driver) | A caller-built `Call` — a verb name and named arguments (`ArgValue` text, number or flag) |
      `validate(&Call) -> Result<Command, Refusal>` checks a call against a closed `'static` table of five verbs (`snapshot`, `click`, `type`, `press`, `advance`)
      and five argument kinds with bounds — `id` 1 to 1024 bytes of text, `text` 0 to 4096 bytes, `key` one name of a twelve-name key list, `flag` true or false,
      `milliseconds` a whole number from 1 to 60000 — in one fixed order with one answer per call: the verb by exact name (absent → `unknown-verb`), then the call's
      arguments in the order passed (a name the verb lacks → `malformed` / `Unnamed`, a name seen before → `Repeated`), then the verb's arguments in the schema's order
      (required and absent → `Missing`, a value of another kind → `WrongKind`, a value outside its bound or a text that is no key name → `OutOfBound`); no `unwrap`,
      `expect`, `panic!`, `unreachable!` or index on anything a call supplies; it takes no `Session`, so a refused call changes nothing by construction, and it reads
      no clock; in process only — no item is reachable from the socket, a CLI or an MCP tool and the wire still reads `hello` and `stop` only, so this is not yet an
      external-input surface but the check every later surface's input passes through; `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the
      typed text they hold, and nothing in the crate prints, logs or fields them (packages/escher-driver/src/command.rs:161) — as measured at
      escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md
    sidecar: >-
      2026-10-07-command-and-refusal-schema — §Input Validation gains a "Driver command schema (escher-driver)" row: `validate(&Call)` against a closed five-verb table,
      five bounded argument kinds, a total refusal order; in process only, not yet an external-input surface.
    rationale: >-
      Not an unvalidated boundary — the report states the mechanism is present (report.md:44, "validation mechanism✓") and that no external-input surface is added
      (report.md:9, :16, :43). The drift is a missing row: §Input Validation lists escher-driver's other caller-supplied inputs (the request line, state directory and
      label at security-plan.md:73-75) and has no row for the schema's argument validation, which the report carries as Expected amendment 3 (report.md:37) with the
      mechanism in Symbols / APIs (report.md:12, :14) and the kinds and bounds in Schema / config (report.md:19). The `Debug` clause is the report's forward fact
      (report.md:15, :63). Severity is the detector's; the operator may read it as a doc-coverage addition rather than a security finding.
    basis: escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md:12-19, :37, :44; .andromeda/security-plan.md:73-75 (the rows it joins)
  - detector: D-security-input
    severity: escalate
    section: §API Security
    change: >-
      Extend the "Settle and loads in flight" row's Configuration cell with: the driver's command schema keeps this — none of its five verbs (`snapshot`, `click`,
      `type`, `press`, `advance`) has an argument or a result field of a wait, and `validate` reads no clock; a step that ran and did not go quiet is a result
      (`settled: false` with its busy class, one of `render`, `layout`, `loads`), not a refusal; time moves only when the caller steps it with `advance`, bounded at
      1 to 60000 ms; no driver command waits on a load (the operator, 2026-10-07) — as measured at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md
    sidecar: >-
      2026-10-07-command-and-refusal-schema — §API Security "Settle and loads in flight" row records that no verb of the command schema waits on a load or reads a
      clock (the operator's answer, 2026-10-07).
    rationale: >-
      The report's Expected amendment 4 (report.md:38) names this row (security-plan.md:202, the one hit of `Settle and loads`). The row states today that a settle
      never waits on a load; the chunk fixes the same property for the command surface — no verb has an argument or field of a wait, the `Instant::|SystemTime` census
      reads 0 (report.md:75), a not-quiet step is a result not a refusal (report.md:13), and the answer is recorded as the operator's (report.md:59). An addition to
      the row; nothing it says today is retired.
    basis: escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md:13, :38, :59, :75; .andromeda/security-plan.md:202
  - detector: D-security-input
    severity: escalate
    section: §Error Handling
    change: >-
      Add a bullet under "Error format (typed errors)": escher-driver's command refusal is a typed value — `Refusal`, with private fields (a `Cause` and an optional
      `Fault`), built only by `Refusal::new(cause)` or `Refusal::malformed(fault)`, so a fault sits only beside `malformed`; the cause set is closed at eight
      (`unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen`, `time-unavailable`), each with a fixed non-empty `&'static str` name,
      meaning and remedy that holds no path; a `Fault` (`Missing`, `Unnamed`, `Repeated`, `WrongKind`, `OutOfBound`) carries at most the schema's own argument name;
      none of `Cause`, `Fault` and `Refusal` has a `String` field, so a refusal cannot hold anything a call supplied; `Display` prints `{name}: {remedy}` or
      `{name} ({fault}): {remedy}` and `Refusal` implements `std::error::Error`; in process only — nothing prints, logs or sends one yet
      (packages/escher-driver/src/refusal.rs:115-126; packages/escher-driver/src/refusal.rs:144-147) — as measured at
      escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md
    sidecar: >-
      2026-10-07-command-and-refusal-schema — §Error Handling "Error format (typed errors)" gains escher-driver's `Refusal` / `Cause` / `Fault`: a closed eight-cause
      set with fixed texts and no `String` field.
    rationale: >-
      The report's Expected amendment 5 (report.md:39): `Refusal|refusal` reads 2 hits in security-plan.md (:72 the cold-agent stub's, :73 the socket's), neither
      naming this type, and "Error format (typed errors)" (security-plan.md:324-331) lists no escher-driver type. The type is the validation's failure output; the
      report carries it in Symbols / APIs (report.md:10), the private-field construction in Deviation 3 (report.md:51), and the no-call-content property as a met
      security criterion (report.md:74). An addition; no existing bullet is retired.
    basis: escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md:10, :39, :51, :74; .andromeda/security-plan.md:324-331
```

### test-plan

```yaml
# D-tests-framework: no drift — the report's gates run `cargo test -p escher-driver --locked [--lib refusal::|schema::|command::]`, `cargo test -p blitz-tests --locked --test …` and `ci-leg.sh fast|doc|a11y` (report.md:87-97); libtest, inline `#[cfg(test)] mod tests`, `assert!` — the runner of test-plan §2 (:44), §3 Rust tests (:80) and §4 Test file location (:136). Dependencies bullet: none added (report.md:18), so no third-party test framework.
# D-tests-obs-harness: no drift — "Harness / gate surface: none" (report.md:29); no agent-run script, status shape, event or log format changed; the new modules print and log nothing (report.md:15, :80). §3 and obs-plan §3 are untouched by this chunk.
# D-tests-coverage: the code-side invariant HOLDS (each new module carries inline unit tests at tier 0: refusal 3 · schema 4 · command 5; report.md:44-46, :76). The drift is the doc's own coverage statements, now stale against the report's "Counts / qualifiers moved" (report.md:22-23) and Expected amendment 6 (report.md:40). Four sites below.
# Swept and left unchanged (still true per report.md:9, :16): test-plan.md:102 and :181 "typed text is not measured (no command can type into the host's instance yet)"; :209 "it answers `hello` and `stop` only"; :21 "host.rs and client.rs hold no unit test"; :115 `run stand` counts (no `stand_*` check added, report.md:29); :122/:146 the cold-agent stub's own refusal causes (a different surface); :324's earlier links incl. "+13 `escher-driver` unit tests in a new lib result line" (dated history, the baseline); :326 a11y 6 · 6 · 3 (re-read equal, report.md:83).
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: >-
      Was "13 inline unit tests in three files"; now "25 inline unit tests in six files" — error.rs 3, session.rs 2, wire.rs 8 as stated, and since 2026-10-07-command-and-refusal-schema refusal.rs 3 (the eight causes named in order; each cause's meaning and remedy a non-empty fixed text holding no path; a refusal reading as its cause then its remedy), schema.rs 4 (the five verbs `snapshot` · `click` · `type` · `press` · `advance` named in order; every verb stating its help, its arguments and a result, result-field names unique; a verb found by its exact name only; each verb's stated shapes, the kind names and bound texts, the twelve key names, `NODE_FIELDS`, and `BUSY_CLASSES` against the harness's `Busy`) and command.rs 5 (an admitted call becomes its command — 26 rows, all twelve keys; an unknown verb refused as `unknown-verb` — 8 × 2; a malformed call refused as `malformed` with the rule it broke — 34 rows; a call breaking two rules gets the first in `validate`'s order — 4 pairs and 1 unknown-verb case; every verb of the table reached and naming its own spec); "host.rs and client.rs hold no unit test" and "its doc-test line reads 0" stay; the citation list gains packages/escher-driver/src/refusal.rs:187-314, schema.rs:277-444, command.rs:234-572, and the "as measured at" clause gains escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md for the three new files.
    sidecar: >-
      §1: escher-driver unit tests 13 in three files → 25 in six (refusal.rs 3 · schema.rs 4 · command.rs 5 added to error.rs 3 · session.rs 2 · wire.rs 8); the command and refusal schema's pins named.
    rationale: >-
      report.md:22 — "`escher-driver` inline unit tests 13 → 25, in six files where there were three: `refusal.rs` 3, `schema.rs` 4, `command.rs` 5", basis `cargo test -p escher-driver --locked`, `25 passed` in both block runs, and it names `.andromeda/test-plan.md:21` as the stated site; what each test pins is report.md:44-46 (Coverage of new surfaces) and Deviation 4 (report.md:52). The tests exist at the unit tier, so the detector's code-side invariant holds — the doc's count and file list are what drifted.
    basis: ".andromeda/test-plan.md:21 (the claim); report.md:22 (the measure); packages/escher-driver/src/refusal.rs:187, schema.rs:277, command.rs:234 (the three `#[cfg(test)] mod tests`, 3 · 4 · 5 `#[test]`)"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: >-
      The row, which today names wire.rs, session.rs and error.rs only, also states the three new pins: refusal.rs pins the closed cause set — eight causes in the stated order, each name a class word, each meaning and remedy a non-empty fixed text holding no path, and a refusal's printed form; schema.rs pins the verb table — the five verbs in order, each with help, arguments and a result, result-field names unique, `verb` by exact name only (a near-miss such as an upper-case name finds nothing), each verb's argument and result shapes, the kind names and bound texts, the twelve key names, `NODE_FIELDS` and `BUSY_CLASSES` held against the harness's `Busy`, and no argument or field of a wait; command.rs pins `validate` — admitted calls (26 rows, the twelve keys and `Key`'s mirror of `KEY_NAMES`), unknown verbs refused as `unknown-verb` (8 × 2), malformed calls refused as `malformed` with the fault naming the rule broken (34 rows), the refusal order (4 pairs and 1 unknown-verb case), and table and `Command` in agreement; every assertion over a call carries a row index only, so a failing log prints no argument value; the schema is held in process — no test drives it over the socket or through a `Session`; citations add packages/escher-driver/src/refusal.rs:187-314, schema.rs:277-444, command.rs:234-572 — as measured at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md (four mutation controls each turned its named test red, evidence/controls.md).
    sidecar: >-
      §4: the escher-driver row gains the refusal, schema and command pins (3 · 4 · 5) beside the lifecycle's wire, session and error pins.
    rationale: >-
      The row is the second statement of what the crate's unit tests cover and lists three files where the crate now tests six — report.md:40 names `.andromeda/test-plan.md:142` as an expected-amendment site ("the unit-test count and the three new pins"); contents from report.md:44-46, :52-53 (Deviations 4 and 5), :72-77 (the tests criteria, controls 4 of 4) and :9 (held in process only).
    basis: ".andromeda/test-plan.md:142; report.md:40, :44-46"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (key file .andromeda/registries/contracts/test-plan/session-lifecycle.md, label session-proof)"
    change: >-
      Was "escher-driver unit tests 13"; now "escher-driver's lifecycle unit tests 13 (error.rs 3 · session.rs 2 · wire.rs 8) of the crate's 25 — the other 12 are the command and refusal schema's, no part of this contract"; the existing "as measured at" clauses stay, with the 25 as measured at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md.
    sidecar: >-
      §3 → Session lifecycle, session-proof: "escher-driver unit tests 13" scoped to the lifecycle's 13 of the crate's 25.
    rationale: >-
      A second occurrence of the retired claim that the crate holds 13 unit tests, worded "unit tests 13" — so the report's site pattern `\b13 unit|error\.rs 3` (report.md:22, "2 hits") does not match it, and report.md:40 calls the contract "no change" on the ground that it cites no moved line, which is true of its citations but not of this count. Unqualified, the token now reads as the crate total and is false (25, report.md:22); the 13 lifecycle tests themselves are unchanged (error.rs, session.rs, wire.rs byte-identical, report.md:16). Orchestrator's call if it prefers the report's no-change reading (the clause is dated "as measured at … driver-session/report.md"); a one-site apply at :21 leaves this one standing.
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Local baseline → `cargo test --workspace` (the re-count chain)"
    change: >-
      Append one link after the 2026-10-07-settle-detection link (earlier links untouched): "; re-counted at 2026-10-07-command-and-refusal-schema: 143 result lines, 610 passed · 0 failed · 8 ignored (+12 `escher-driver` unit tests — refusal 3 · schema 4 · command 5 — in the crate's existing lib result line; no result line added), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run in its operator pass on the tree its pre-CI commit 772c770f carries (escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md)".
    sidecar: >-
      §9 Local baseline: workspace 143 result lines · 598 passed · 0 failed · 8 ignored → 143 · 610 · 0 · 8 (+12 escher-driver unit tests, no new result line).
    rationale: >-
      report.md:23 — "Workspace tests 598 → 610 passed · 0 failed · 8 ignored, over 143 result lines, unchanged in number", basis `target/ci-logs/test.log` from the operator pass's fast leg (2026-10-07T12:23Z), and it names `.andromeda/test-plan.md:324` as the stated site; the chain's last link still ends at 598. A separate fact from the crate count (workspace total), hence its own primary. Sweep hazard per report.md:65: count sites by the phrase, not the bare `598`/`143` — in test-plan.md the only `598` hit is this line.
    basis: ".andromeda/test-plan.md:324 (last link: settle-detection, 143 · 598 · 0 · 8); report.md:23"
```

### obs-plan

```yaml
proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage"
    change: >-
      Extend the `packages/escher-driver` observed-absent bullet (obs-plan.md:105) so its parenthetical names the crate as the session library AND the command schema — add, beside `Session::start`, `Session::act`, `serve`, `start`, `attach`, `stop` and the lifecycle wire: "and the command schema in the private modules `command`, `refusal`, `schema` — `validate(&Call) -> Result<Command, Refusal>`, the five-verb table `VERBS`, and the refusal type `Refusal` / `Cause` / `Fault`, held in process only, reachable from no socket and taken by no `Session` method"; state that `validate` carries no span or event (silent by its chunk's constraint, like the rest of the crate), and that the closed set of eight cause names returned by `Cause::name` (`unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable` — fixed `&'static str` class words holding nothing a call supplied) is the value domain the span field of the still-owed route entry "Driver command spans" reads; append the citation `packages/escher-driver/src/command.rs:161` and "the command schema at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md" to the as-measured list. The "one span per driver command … is owed by the route entry 'Driver command spans'" clause stays as written.
    sidecar: >-
      2026-10-07-command-and-refusal-schema — §4: the escher-driver no-span record now covers the command schema (`validate`, `VERBS`, `Refusal`/`Cause`/`Fault`); `Cause::name`'s eight class words recorded as the value domain for the owed "Driver command spans" field.
    rationale: >-
      The report's Symbols / APIs bullet adds `validate` (command.rs:161), `VERBS`, `Command`, `Cause`, `Fault`, `Refusal` to `escher-driver`, and its Coverage line reads "`validate` … instrumentation n/a (the crate is silent by spec; the span is the 'Driver command spans' entry's)"; the Outcome's (obs) criterion is met with "no span, event, `tracing` or `log` dependency" and Dependencies reads "none added, none bumped". §4's bullet enumerates the crate as "the session library: `Session::start`, `Session::act`, `serve`, `start`, `attach`, `stop` and the lifecycle wire" — an enumeration the chunk made incomplete: the new operation `validate` is uninstrumented and §4 does not record it as a deliberate, owed absence, nor the cause set the later span field reads. The report's Expected amendment 7 names this site (`.andromeda/obs-plan.md:105`, ×2). No instrumentation is missing against a §4–§6 requirement (those sections require none for this crate); the drift is the doc's record, not the code.
    basis: ".andromeda/obs-plan.md:105 ; packages/escher-driver/src/command.rs:161 ; packages/escher-driver/src/lib.rs:29-49 ; report.md:12, :41, :44, :80"
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation)"
    change: >-
      In the headless-stand bullet (obs-plan.md:69), widen the clause "the same holds for the session library `packages/escher-driver` (no `tracing` dependency, no subscriber, no env read, no print) — its settled step `Session::act` included" to read "… — its settled step `Session::act` and its command schema included (the three private modules `command`, `refusal`, `schema`: `validate`, the verb table and the refusal type install no subscriber, read no env var and no clock, and print or log nothing; `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold, and nothing in the crate prints, logs or fields them)"; append "the command schema at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md" to the bullet's as-measured list. The existing citation `packages/escher-driver/Cargo.toml:13-15` stands unchanged (the manifest is byte-identical).
    sidecar: >-
      2026-10-07-command-and-refusal-schema — §3 Logging stack: the escher-driver silence clause now names the three new modules (`command`, `refusal`, `schema`) and records the `Debug` derive on `Command` / `Call` / `ArgValue` as unprinted.
    rationale: >-
      Same retired claim as the §4 primary — the doc's reading of `packages/escher-driver` as the session lifecycle only — restated in §3 at obs-plan.md:69, which singles out `Session::act` as "included" and names nothing of the schema. The report's Crates / modules bullet: "`escher-driver` gains three private modules — `command`, `refusal`, `schema` … eight where there were five"; Symbols / APIs: "`Command`, `Call` and `ArgValue` derive `Debug` … Nothing in the crate prints, logs or fields them (basis: the plan's census entry, 0)"; Outcome (obs): "the new modules and the root install no subscriber, read no env var or clock, print or log nothing". The stack itself matches §3 (Dependencies: none added, no `tracing` or `log` dependency) — no off-spec logger; the amendment only extends the silence record to the surface the chunk added. Named by the report's Expected amendment 7 (`.andromeda/obs-plan.md:69`, ×2).
    basis: ".andromeda/obs-plan.md:69 ; packages/escher-driver/src/lib.rs:29-37 ; packages/escher-driver/Cargo.toml:13-15 ; report.md:15, :17, :18, :80"
    dependent-of: D-obs-instrumentation

# Not proposed (evaluated, no drift):
# - D-obs-pii (escalate): no violation. The report adds no logging: Coverage reads "PII n/a (nothing is logged; `Refusal` and `Fault` hold nothing of the call)" and "PII redacted by type (no `String` field)"; Schema / config reads "no scrub or redaction shape". The `Debug`-prints-id-and-text hazard is a forward fact for the later span entry (report.md:63), not a change this chunk — it is carried in the §3 proposal's wording only.
# - obs-plan.md:290 ("Typed text is NOT measured — no command can type into the held instance yet") still holds: the `type` verb is in-process schema data, reachable from no socket and taken by no `Session` method (report.md:9). No change.
# - obs-plan.md:325 (§9 Session state directory, "belongs to the session library and its checks") and :294 (`escher_driver` in `ESCHER_TARGET_PREFIXES`): unaffected — `host.rs` is byte-identical and no scrub shape moved.
# - Keyed contract "Bootstrap phases (derive for route / setup-project)" (otel-sdk-install, pii-scrubbing-wire): read whole; untouched by the chunk (no dependency, no OTel, no sink or scrub change).
```
