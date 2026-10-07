# design extract

## Relevance
partial — the chunk renders no new markup and declares no style, so no colour, spacing, radius, depth or icon rule binds it; what binds is the rendering environment the held instance keeps for its whole life (font loading, viewport scheme and scale, the animation clock) and the plan's terminal-output record for any new binary. The plan's brand, anti-pattern and project-wide token sections, and its CLI command-structure and platform notes, are marked unrecorded or unmeasured and give no coverage.

## Constraints
- The held instance must shape its text with the one bundled font registered for every generic with system fonts off, handed in through the harness options — the stand's headless font path, not a font context of the session's own (per design-system §Typography → Loading). Whether a session booted from a new crate reaches that path unchanged is research's question.
- The bundled font is decoded by a native feature of the stand crate (per design-system §Typography → Loading): a crate that depends on the stand must keep that feature on, or the instance boots with no usable font. How the feature is enabled today (default or named by the dependent) and whether a new dependent inherits it is research's question.
- A document whose font did not load measures text as empty and lets font-dependent assertions pass vacuously (per design-system §Typography → Loading): every session proof check that reads bounds, or compares state that depends on text measurement, must first hold a non-empty text measurement on the held instance.
- The instance's colour scheme, hidpi scale and zoom are viewport properties with recorded defaults, and a scheme change forces a full recascade (per design-system §Surface: desktop-native → Tokens (platform-specific); §Color Palette → Color model and scheme): the session fixes them at `start` and no lifecycle step (`attach`, `stop`, a lifecycle edge) may alter them on a live instance. Whether the stand's boot options pin scheme and scale explicitly or inherit the defaults is research's question.
- Document time on a headless instance is a controlled clock advanced by tick, and running animations, scroll animations and scrollbar fades keep a document in its animating state (per design-system §Motion → Animation runtime): the session must leave that clock as the only source of document time — no wall-clock read feeds the held instance — and keep the tick handle reachable for the session's life, which is what the later settle entry advances.
- The chunk adds no colour, type, spacing, radius or shadow value: the stand's recorded values stay exactly as recorded (per design-system §Color Palette → Core Colors / Semantic Colors; §Typography; §Spacing → Untokenized values; §Border Radius). The stand is recorded as untokenized, so no token mandate applies here and none is to be invented by this chunk.

## Patterns to follow
- Boot through the stand's existing headless font registration rather than building a font context beside it (per design-system §Typography → Loading).
- Treat the held instance's time the way the headless harness already does — advanced by tick from a controlled clock, never left to run (per design-system §Motion → Animation runtime).
- If the session binary ever writes human-facing decoration to a terminal, gate it on the stream being a terminal, as the recorded CLI binary does for its hyperlinks; plain text otherwise (per design-system §Surface: cli → Tokens (platform-specific)).
- Report failures on stderr and outcomes as plain one-line text, as the recorded CLI binaries do (per design-system §Surface: cli → Component Patterns). The shape of command results is the later Driver CLI entry's, not this chunk's.

## Anti-patterns to avoid
- Proving held state on an instance whose text measures empty — a missing font feature turns the checks green without evidence (per design-system §Typography → Loading).
- Reusing the conformance runner's result colours as a vocabulary for lifecycle outcomes: they are recorded as that runner's domain status colours only, and the plan records no colour convention for any other binary (per design-system §Color Palette → Domain status colors; §Surface: cli → Tokens (platform-specific)).
- Changing the viewport's scheme or scale on a live session to "reset" it between commands — it recascades the whole document and breaks the one-instance-held-across-commands claim (per design-system §Color Palette → Color model and scheme).

## Contract bindings
- design ↔ tests: the font-loaded precondition binds to the stand checks' harness contract (test-plan §3) — a bounds or text-dependent assertion in a session check is only evidence when the bundled font decoded; the both-layout-modes loop applies where a check asserts layout.
- design ↔ a11y: the snapshot's bounds come from text measured with that font, so the font path is part of what the snapshot reads; painted colours are a11y surface (SC 1.4.3) and this chunk must paint none new — the binding is "unchanged", to be confirmed by a11y's extract.
- design ↔ obs: a new binary's terminal output is shared ground — telemetry owns stderr-only bootstrap; design's record adds only that decoration is terminal-gated and errors go to stderr.
- design ↔ architecture: the font context and viewport reach the instance through harness options; a new crate's feature wiring toward the stand crate is an architecture decision at P4 that this domain's font constraint depends on.
- design ↔ layouts: the pinned viewport's size is layouts' subject; this extract binds only its scheme and scale.

## Acceptance criteria contributions
- A session-held instance, in both layout modes, reads a non-empty measured size for at least one text-bearing element before any command runs — the bundled font decoded (per design-system §Typography → Loading)
- A session-held instance read before any command matches a fresh stand boot of the same task in every text-dependent reading the checks use (bounds, snapshot text) — the session added no font, scheme or scale of its own (per design-system §Typography → Loading; §Surface: desktop-native → Tokens (platform-specific))
- The session's diff adds no stylesheet, inline style, colour, font or spacing declaration to the stand or to any new crate (per design-system §Color Palette → Core Colors; §Typography; §Spacing → Untokenized values)
- Document time on the held instance moves only when the tick handle is advanced: a lifecycle step or command that does not tick leaves the timer task's time-dependent readings unchanged (per design-system §Motion → Animation runtime)
