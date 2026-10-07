# Playbook — escher

<!--
Amendment-validation rules consulted by /andromeda-wrap-session's main agent when it validates the
amendments its fan-out proposed. This file GROWS from dogfood — it starts near-empty. One rule per entry:

  - pattern: {the class of amendment this matches}
    verdict: routine | escalate       # routine → apply silently; escalate → halt + ask the user
    note: {why}

"Main is uneasy" (no rule matches but it looks strange) → escalate too; a confirmed escalation pattern
becomes a new rule here. Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).
-->

## Rules
- pattern: Sequencing deferral — a drift-detector flags a gap (a tool not yet installed, a capability not yet hardened, a dependent not yet wired, a span not yet emitted, a test tier not yet built) whose resolution is a LATER, still-pending chunk's defined job. NOT Foundation-only: it equally covers a primitive shipping before its driver, a config landing before the runtime that traces it, and a gate deferred at /implement that the wrap light gate then runs green.
  verdict: routine
  note: builds land capabilities first and harden / depend on them in later chunks — expected ordering, NOT spec drift (the spec is right; the resolving chunk just hasn't run yet). Apply silently. Auditable caution — it is sequencing ONLY if a later chunk genuinely owns the resolution; a gap with NO resolving chunk anywhere in the route is real drift → escalate. And ownership is a ROUTE annotation (a CARRY/PREREQ pinned on the owning entry at route-resolve), never this note or prose — a note cannot verify delivery, and an owner-chunk can complete without delivering unless the obligation rides its line. A dismissal citing a route-sequenced owner whose entry is already `complete` is NOT routine — the owner came and went; escalate it.

- pattern: Not this chunk's drift — the proposal names a symbol / dependency / section / surface that does NOT appear in the report's Changes as this chunk's work: a dependency the chunk never touched, a plan↔plan bind neither of whose sections it changed, an invariant already satisfied by shipped infrastructure it merely consumed, a pre-existing surface it only extended internally.
  verdict: routine
  note: reject — the report's Changes is the single source of what changed this wrap, so a detector firing on anything else has mis-fired (commonly by reading the doc's own history or rationale as a current gap). Verify against the report before dismissing. Caution: where the proposal is ACCURATE and the drift REAL but pre-existing across several artifacts the chunk did not touch, do NOT amend the one artifact the detector named — that leaves a worse spec-vs-impl mismatch; route the whole family to its OWNED channel instead: a `CARRY:` pin on the markerless entry that owns that surface (route-resolve, this wrap), or — when it belongs to a future version — an `.andromeda/residuals.md` append.

- pattern: Registry over-reach — a detector proposes registering per-item REALIZATION into a spec registry that tracks its subject at CATEGORY grain: a public API symbol, an internal module, a CLI verb or flag, a command / handler name, an individual config file, a framework permission granted inside an already-registered capability file, a second transport instance for an already-registered surface.
  verdict: routine
  note: reject — the arch registries track occupied RESOURCES (ports / sockets / endpoints / IPC / events / env vars / crates/packages / artifacts) and contract SHAPES, not the realization inside an already-registered category. Check what the registry actually enumerates, and whether earlier chunks of the same kind registered theirs — they usually did not. A genuinely new resource of a kind the registry DOES enumerate still registers normally.

- pattern: Accurate this-chunk addition — the proposal registers something the CURRENT chunk genuinely introduced or changed (its named symbols / values DO appear in the report's Changes), landing inside an existing section; or it reconciles a spec's illustrative wording / mechanism to the sound implementation the chunk shipped, where the report shows the invariant still holds.
  verdict: routine
  note: apply — the apply-side dual of the two reject rules above; without it a playbook only learns to dismiss. Bringing a body to current truth IS reconcile's job. The test is that the named thing is THIS chunk's and the invariant survives — only form or mechanism may differ. A REVERSAL of a locked decision is not this rule: escalate that once to ratify it.

- pattern: External decay — a gate turns red with NO in-diff cause: the lock / source / config it checks is un-drifted and the failure keys on the world moving while the project stood still (a freshly-fetched advisory DB, an expired tool or cert, a registry policy change) — typically surfacing after a pause.
  verdict: routine
  note: neither this chunk's drift (nothing in the diff caused it) nor sequencing (no future chunk owns it yet) — the world moved, the code didn't. It never blocks the chunk that DISCOVERED it, and it never resolves by silently widening an ignore list (actionable-with-fix items are not the non-actionable class an ignore legitimately absorbs). It ALWAYS produces an owner: a route entry, or a recorded bounded deferral. If the gate must pass meanwhile, an ID-scoped reasoned ignore NAMING its owning chunk is part of the pattern, not a shortcut — a permanently-red gate stops discriminating, so a NEW red becomes invisible. An operator-RATIFIED standing deferral may compact its re-pin and set a probe re-run interval — with the overlap probe NAMED and verified green every chunk, and interval skips recorded in the chunk report, never silent — or take the probe-auto-satisfy tier: the pin names its SIGNATURE (failing gate's exit + first diagnostic line + overlap result) and a byte-identical probe — read from the external artifact's current state, never only a local copy of it — satisfies it with a one-line record; any deviation restores the full form.

- pattern: Boundary widening — a chunk WIDENS what crosses an already-hardened boundary (a read-only channel gains a write, a validated surface admits a new input class, a subprocess/IPC boundary gains a new crossing) and the proposal records it.
  verdict: escalate
  note: always a human's call — never mint a routine rule for this class, however often it recurs: the recurrence is the reason it must keep reaching the operator (a routine verdict here silently widens a precedent). The escalation resolves on the operator's ratification, recorded in the sidecar.

- pattern: Provisional discharge — an amendment removes, or would leave standing past its epoch, a PROVISIONAL mark: a body, key-file or sidecar clause recorded on a relayed rendering or a delegate's answer to a call the founder owns (a boundary widening, a trajectory call, a behaviour change for every document).
  verdict: escalate
  note: PROVISIONAL items are discharged in one batch at each epoch boundary, on the founder's own word — given at that wrap or relayed verbatim — never one at a time as chunks pass and never by a wrap on its own judgement. The wrap that closes an epoch (or the first wrap after it) lists every standing mark with its sites — bodies, key files, sidecars and leaves — for the operator; each one the founder rules on is ratified in the same pass (the mark replaced by `ratified by the founder ({date})`, the ratification recorded in the sidecar, any part still owed — a witness, a measurement — kept in the body beside it) or reworked; one with no ruling stays marked and is named in the handoff. Set by the founder at the Epoch 2 boundary (2026-10-06, `architecture-amendments.md`, the founder-rulings entry); written here on the operator's direction, 2026-10-07.

- pattern: A proposal graded `escalate` by a detector's own severity, where that detector reports its invariant holding and the plan's reviewed list names the change, is applied without a halt.
  verdict: routine
  note: the grade is the detector's severity for a VIOLATION of its invariant; a return that reports the invariant holding and proposes only the doc's stale wording has found none, and the change was already in front of the operator in the plan's reviewed `Expected amendments (wrap)` list. A `dependent-of` proposal — another wording of the claim the named change retires — is validated with its primary. It never covers a boundary widening, which is judged by its subject and escalates whatever a plan or a direction says, and it never discharges a PROVISIONAL mark. The class reached four wraps, all on 2026-10-07 (the 06-51 wrap escalated it; the 08-23, 12-34 and 14-22 wraps applied it). Appended on the founder's word (2026-10-07, relayed verbatim by the overseer); the pattern is the sentence the 2026-10-07T12-34-00 wrap proposed it in.

_(more grow from escalations + resolved cases — the first five above were harvested from live projects that
derived them separately, the sixth is the never-routine class; anything genuinely project-specific still starts
here empty.)_
