# tests · Unit Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:16) The statement's verb list must match the pinned verb table: twelve verbs, in that order, at three levels. CARRY 1's README fix («eleven» → twelve) must also match it.
  > schema_tests.rs (`schema::tests`) pins the verb table — twelve verbs in a stated order and three levels (`Level::Instance` · `Level::Session` · `Level::Tool`, `Level::name` reading `instance` · `session` · `tool`: the eight instance-level verbs, `screenshot` after `scroll` since 2026-10-12-headless-screenshot, `lint` after `screenshot`, then `start`, `status`, `stop`, then, since 2026-10-12-self-description, `help`)
- (.andromeda/test-plan-detail/unit-test-strategy.md:16) The snapshot's readings and the exit statuses must be stated as the pinned tables hold them: thirteen node keys, `differs` after `bounds`, and four exit statuses.
  > the thirteen node keys named in order, each with its help (`differs` after `bounds`), and the four exit statuses in status order
- (.andromeda/test-plan-detail/unit-test-strategy.md:16) The arguments, including the paged snapshot's `part`, must be stated with their pinned bounds, 1 to 1000.
  > the seven argument kinds with their bounds (the sixth, `ArgKind::Name`, 1 to 32 bytes of `a-z`, `0-9`, `-`; the seventh, `ArgKind::Part`, a whole number 1 to `MAX_PART` = 1000)
- (.andromeda/test-plan-detail/unit-test-strategy.md:16) Refusals must be stated as the closed set of eight causes, each with a fixed remedy. No other cause may be named.
  > refusal.rs pins the closed cause set — eight causes in a stated order (`fonts-unavailable` the eighth, ratified by the founder, 2026-10-18), each name, meaning and remedy a fixed text against a stated table, the remedies pairwise distinct and no `/` in any
- (.andromeda/test-plan-detail/unit-test-strategy.md:16) Session errors must be stated as fifteen, each with a kind, a meaning and a remedy.
  > error.rs pins, since that chunk, that each of the session errors — fifteen since 2026-10-19-session-recording
  >  — states a kind, a meaning and a remedy and that the state directory's message states its bound (94 bytes on Linux, 90 elsewhere);
- (.andromeda/test-plan-detail/unit-test-strategy.md:15) CARRY 2's limit, that the diff and the findings are not paged and are held only by the answer bound, must name the pinned bound of 1,048,576 bytes.
  > an answer at its 1,048,576-byte bound is carried while one byte over is not
- (.andromeda/test-plan-detail/unit-test-strategy.md:17) The load wait must be stated as the schema's help states it: `LOAD_WAIT` in whole seconds, carried by the five acting verbs.
  > schema_tests.rs that the `settled` help states `LOAD_WAIT`, whole seconds, that the five acting verbs carry it and that the wait ends before an answer read
- (.andromeda/test-plan-detail/unit-test-strategy.md:18) The visual lint must be stated with six defects in this order, and findings must be stated in document order, then defect order.
  > since 2026-10-19-visual-lint lint.rs that `Defect::ALL` names the six defects in order (`overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`) and that findings come in document order, then `Defect::ALL` order
- (.andromeda/test-plan-detail/unit-test-strategy.md:17) Session recording must be stated as these tests pin it: the masked `type` stand-in, owner-only files, and a recording that ends at its bound with its check removed.
  > recording_tests.rs (`recording::tests`, in `recording.rs` until 2026-10-19-test-splits) pins the FNV-1a 64 digest of the written answer, a recording's read-back, the masked `type` stand-in, the calls no record can spell left out, owner-only modes with no file replaced and no foreign temporary removed, a recording ending at its bound or on a failed write with its check removed
- (.andromeda/test-plan-detail/unit-test-strategy.md:17) Replay must be stated as pinned: it stops at the first differing step, a shot holds at its size whatever its pixels, and a check of another app runs nothing.
  > replay.rs that a check replayed on a fresh session holds, that an acceptance flip and a changed answer each stop the replay naming their keys, that a shot holds at its size whatever its pixels and that a check of another app runs nothing
- (.andromeda/test-plan-detail/unit-test-strategy.md:18) The statement of what an MCP client carries onward must describe a shot as pinned: an image block, then a text block, bounded by its whole written form.
  > mcp_tests.rs (`mcp::tests`) that `help` is answered by the server with or without a session and, since 2026-10-12-headless-screenshot, that a result holding an image is bounded by its whole written form and a shot is written as its image block then its text block
- (.andromeda/test-plan-detail/unit-test-strategy.md:18) What a carried PNG holds must be stated as pinned: the pixels, and no time or text chunk.
  > the PNG encoder — a PNG decodes to the pixels it was given and holds no time or text chunk
- (.andromeda/test-plan-detail/unit-test-strategy.md:19) Masked values in what the carriers write must be stated as the fixed `MASKED_VALUE`.
  > a node with the schema fields it has (a masked value as `MASKED_VALUE`)
- (.andromeda/test-plan-detail/unit-test-strategy.md:19) `press` must be stated over the pinned key table of twelve keys.
  > execute.rs pins the executor's key table — each of the schema's twelve keys stands for one keyboard key, pairwise distinct, the row count asserted
- (.andromeda/test-plan-detail/unit-test-strategy.md:49) The select model's capability must be stated as the pinned rules: selectedness, a step past disabled options, and typeahead by label prefix.
  > four `node/select.rs` tests pin the select's rules over stated tables — HTML's selectedness setting algorithm (the last chosen, else the first option not disabled), a step skipping disabled options and stopping at the ends, typed text choosing the first enabled option whose label starts with it and nothing for an empty text, and the strip and collapse of an option's text
- (.andromeda/test-plan-detail/unit-test-strategy.md:49) The range input's capability must be stated as the pinned value rules: finite, bounded and on its step grid.
  > seven `node/range.rs` tests pin the range value rules — the HTML float grammar, attribute defaults and bounds, the seeded value finite, bounded and on the grid, the moves on the grid, degenerate attributes, decimal formatting, and a re-seed and a re-sanitize on attribute writes
- (.andromeda/test-plan-detail/unit-test-strategy.md:14) The contrast part of the a11y reach must state the pinned thresholds: 3.0 for large text, else 4.5.
  > `required_ratio` over a stated table of 5 rows (3.0 for large text — at least 24 px, or 14 pt at weight 700 — else 4.5), each row count asserted
- (.andromeda/test-plan-detail/unit-test-strategy.md:28) The a11y reach's «`a11y` leg of fourteen files» must match the leg's pinned fourteen target words.
  > the `a11y` row asserting each of the leg's fourteen target words, beside `--workspace` and `--locked`, by one `assertIn` over the shimmed log, in no order
- (.andromeda/test-plan-detail/unit-test-strategy.md:42) The cold-agent pipe as the measured instance must be stated at twelve tools in its contract tests' fake, with `lint` answered by an empty findings list.
  > a fake `target/debug/escher-session` (`FAKE`, which speaks the twelve tools, answering `lint` with `{"findings":[]}` (.github/scripts/test_cold_agent.py:100-101), writes `tool_call` span lines and records its spawns)
- (.andromeda/test-plan.md:187) The statement must not claim a live twelve-tool run. The one live run measured eleven tools.
  > fired once: `passed`, `isolated` (the eleven `mcp__escher__*` tools, one server `escher`)
- (.andromeda/test-plan.md:187) The live run must be stated as not re-fired since `lint` joined.
  > the pipe requires twelve tools with the `lint` verb, its contract tests' fake answering `lint`, and the live run is not re-fired
- (.andromeda/test-plan.md:44) The boolean model's two limits must be stated as these, pinned at unit level and at the engine only.
  > the two limits of the boolean model — a mixed toggle reads unpressed, a grammar or spelling error reads invalid — are pinned here and at the engine only
- (.andromeda/test-plan.md:19) Stated limits must say whether a check asserts them: the text extent's are asserted, the select's are asserted by none.
  > the text extent's stated limits are asserted by blitz-tests' `text_extent`, while the select's are asserted by no check, nor is the process-wide `Parallel` traversal lock
- (.andromeda/test-plan.md:89) The shot's cross-OS limit must be stated as this: PNG bytes differ between OSes, so no byte-compared check across OSes holds a shot.
  > a shot's PNG bytes differ between OSes, so a check compared byte for byte across OSes holds no shot
- (.andromeda/test-plan.md:48) The a11y reach's «not measured» must name the owed check at an assistive technology and its residuals owner.
  > no check reads the accessibility states as the platform adapter hands them to an assistive technology — owed, owner: its `.andromeda/residuals.md` line (target: next)
- (.andromeda/test-plan.md:38) Where the statement says how a capability is proven, the executor's verbs, its refusals and the lint must be credited to integration checks, not unit tests.
  > the executor's verbs, its refusals and `Session::lint` are read by integration checks, not unit tests
- (.andromeda/test-plan.md:207) The body's broad line of what escher-driver's unit tests pin. Any «unit-tested» claim in the statement must stay inside this list.
  > its unit tests pin the wire grammar, the session's label rule and record of ids, the error and refusal texts, the verb schema, command validation, the executor's key table and span, the JSON forms, the command line, the client, MCP, recording, replay, image and lint

## Bindings
- **The verb table ↔ the chunk's CARRY 1 (README) and the body's §4**: the twelve verbs in order (.andromeda/test-plan-detail/unit-test-strategy.md:16) bind the README's count and the statement's list. The body's escher-driver line (.andromeda/test-plan.md:207) is the broad side.
- **The answer bound ↔ CARRY 2's unpaged diff and findings**: the 1,048,576-byte bound (.andromeda/test-plan-detail/unit-test-strategy.md:15) is the only hold on an unpaged diff or findings. The paged `part` bound (.andromeda/test-plan-detail/unit-test-strategy.md:16) is what `snapshot` has in their place.
- **The a11y leg's fourteen words ↔ a11y-plan §3 and the CI Integration category**: the shimmed `a11y` row (.andromeda/test-plan-detail/unit-test-strategy.md:28) must agree with the a11y reach's file count. The contrast thresholds (.andromeda/test-plan-detail/unit-test-strategy.md:14) bind SC 1.4.3's statement.
- **The owed assistive-technology check ↔ a11y-plan and `.andromeda/residuals.md`**: the unmeasured reading (.andromeda/test-plan.md:48) is the a11y reach's «not measured» side.
- **The cold-agent instance ↔ security-plan §Surfaces (cold-agent crossing)**: the live run's eleven tools and «not re-fired» (.andromeda/test-plan.md:187) and the fake's twelve (.andromeda/test-plan-detail/unit-test-strategy.md:42) bind how the statement names the measured MCP-client instance.
- **The stated limits ↔ the body's §1 and §2**: the boolean model's limits (.andromeda/test-plan.md:44), the select's unasserted limits beside the text extent's asserted ones (.andromeda/test-plan.md:19) and the shot's cross-OS bytes (.andromeda/test-plan.md:89) are the limits list's test-plan side, closed at P3 against the schema's help texts.
