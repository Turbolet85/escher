# design extract

## Relevance
partial — the chunk writes a document and paints nothing new; design-system binds only the capability and limit claims the statement makes about surfaces the plan already describes (the agent-read command line and `mcp` role, the stand's measured accent contrast, font loading, motion and the clipped-box hit).

## Constraints
- design-system §Surface: cli › Component Patterns requires `escher-session` and its `mcp` role to be described as surfaces an agent reads, not a person: stdout holds only JSON (one line per command, or newline-delimited JSON-RPC 2.0 lines), no colour or ESC byte, no terminal test, and the exit status carries the class `0` · `1` · `2` · `3`. Any statement of the carriers' output form must agree with it. Whether README.md or the new statement already says this is research's question.
- design-system §Surface: cli › Component Patterns also fixes the written forms the statement may name: a `screenshot` answer as one line with its PNG as base64, an MCP `screenshot` as an `image` block (`image/png`) then a text block, and a paged `snapshot` as `{"text","part","parts"}`. The statement names those forms as the plan words them and adds no new form.
- design-system §Color Palette › Core Colors records the seven_guis accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), with white button text on the five accent controls measured at 5.47 / 6.83 / 8.27 by `stand_contrast`. If the statement's a11y reach cites contrast, it must use these measured stand values and scope them to the stand's controls.
- design-system §Typography › Loading requires a `screenshot` of an app whose font context resolves no `sans-serif` family to be refused `fonts-unavailable` with nothing painted. That belongs in the stated limits; the headless stand's bundled DejaVu Sans is per design-system §Typography (detail `.andromeda/design-system-detail/typography.md`).
- design-system §Motion › This project's values requires a programmatic into-view scroll to be instant in every nested scrolling box, with a requested `Smooth` applying to the viewport alone, and the driver's `scroll` instant everywhere. A statement of what `scroll` does must not claim it animates.
- design-system §Motion › Animation runtime requires that settle neither waits on nor advances an animation, that `advance` alone moves the `VirtualClock` (by exactly its `ms`), and that the load wait moves no clock. These are capability-and-limit facts the statement states as the plan words them. design-system §Motion › Reduced motion records reduced-motion handling as observed absent, which is a candidate stated limit. Whether the masters' limit list already carries it is research's question.
- design-system §Depth Strategy › Engine depth behavior requires a hit to stop at the padding box of a node that paint clips (by `overflow` or a computed `contain` including `paint`). This is the basis of the `covered` limit on a box clipped on one axis, so the statement's wording of that limit must agree with it.

## Patterns to follow
- State each claim in the driver schema's own closed words, as design-system §Surface: cli › Component Patterns does ("keyed by the driver schema's own words"): verbs, causes, exit statuses and defect words verbatim, with no paraphrased synonyms.
- Cite a measured value together with its witness and scope, the way design-system §Color Palette › Core Colors pairs `#3a5ce5` / 5.47 with `stand_contrast` and the five named controls. A claim with no witness reads as a target, not a capability.
- Point to the plan's detail file rather than copying it, the way the plan's body points to `.andromeda/design-system-detail/{category}.md`, so the statement stays current as the detail moves.

## Anti-patterns to avoid
- Generalising the stand's measured accent contrast into a project-wide or every-document colour guarantee. design-system §Color Palette › Core Colors records the pairs on the stand's accent controls alone.
- Describing the command line or MCP output as human-facing, coloured or terminal-aware (design-system §Surface: cli › Component Patterns).

## Contract bindings
- Token contrast ↔ a11y-plan §Contrast (SC 1.4.3): the statement's a11y reach cites the `stand_contrast` rows that design-system §Color Palette › Core Colors records. The a11y extractor owns the criterion claim.
- Motion ↔ a11y-plan §Animation (SC 2.3.3): the reduced-motion absence in design-system §Motion › Reduced motion binds to any a11y-reach claim about animation.
- CLI/MCP output form ↔ security-plan §Surfaces: the content each carrier holds (ids, names, masked values, base64 PNG, findings) is security's crossing, and its written form is design-system §Surface: cli's. The statement's "what an MCP client carries onward" section must agree with both.

## Acceptance criteria contributions
- The statement's description of the command line's and the `mcp` role's output agrees with design-system: one JSON line (or JSON-RPC lines) on stdout, no colour or ESC byte, exit `0` · `1` · `2` · `3`. Checked by a manual read against the plan (per design-system §Surface: cli › Component Patterns).
- Any contrast claim in the statement names the stand's accent `#3a5ce5` with its measured 5.47 (hover 6.83, active 8.27), scoped to the stand's accent controls, and no wider claim (per design-system §Color Palette › Core Colors).
- The stated limits include the `fonts-unavailable` refusal of a `screenshot` (and of a `lint`) of an app with no `sans-serif` family, with nothing painted (per design-system §Typography › Loading).
- Any statement of `scroll` or `advance` matches the plan: `scroll` is instant in every box, `advance` alone moves the clock, and the load wait moves none (per design-system §Motion › This project's values and §Motion › Animation runtime).
