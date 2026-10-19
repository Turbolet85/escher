# layouts extract

## Relevance
partial — the chunk builds no product surface and moves no stand layout. Its layout ties are two. First, the fixture pages it gathers boot into the stand's pinned viewport and mount. Second, if the agent-run fork adds a selection, `scripts/agent-run.sh`'s CLI usage changes.

## Constraints
- A fixture beside the stand boots into the same viewport the stand pins: 800 × 600, scale 1.0, Light, with the bundled DejaVu Sans, through `seven_guis::stand`. Folding the file-private boots (`hold`, `hold_fixture`, `typed_shot`'s boot) into one shared boot must keep that viewport for every reader (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell bullet).
- A held instance reads as a fresh boot, and the TaskShell mount (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) and the pinned viewport hold after commands, in both layout modes. A shared session-holding boot must not change that reading for the stand tasks it also holds (per layout-templates §Surface: desktop-native → Primary screens).
- The stand's tasks gain no markup, id, class, style, wrapper or order. Fixture cases stay beside the stand, never inside a lean task (per layout-templates §Surface: desktop-native → Primary screens, the repeated "adding no class, style, wrapper or order" rule on each task's ids and attributes).
- The pinned viewport itself is scrolled by a driver `scroll` only on a fixture taller than the viewport. The plan currently names this an "in-file fixture", so moving it to a shared module changes that wording. Its height must stay above 600 so the viewport-scroll case still exists (per layout-templates §Surface: desktop-native → IA notes, scrolling-boxes bullet).
- An app fixture mounts into the fixed document skeleton `<html><head></head><body><main id="main"></main></body></html>`. Whether each HTML-string fixture goes through the same skeleton or through another document path is research's question (per layout-templates §Surface: desktop-native → IA notes, skeleton bullet).
- The test harness's default viewport is 800 × 600 at scale 1 in light mode. If a shared fixture boot builds its harness from defaults instead of `stand::options(incremental)`, the two must still agree. Which path each reader uses today is research's question (per layout-templates §Surface: desktop-native → IA notes, harness-default bullet).
- `scripts/agent-run.sh` keeps its usage `agent-run.sh <verb> [selection]`: verbs `boot · run · status · cleanup · logs`, selections `stand · all · <blitz-tests file name>`, stdout JSON lines only, raw cargo output in `target/agent-run/run.log`, and the usage error on stderr with exit 2. If the fork adds a fixture selection, it extends this usage line (per layout-templates §Surface: cli → Primary screens, `scripts/agent-run.sh` bullet).

## Patterns to follow
- Boot through the stand's own entry (`seven_guis::stand`, pinned viewport, bundled font, offline), as the `escher-session` host and `mcp` roles do. A shared fixture boot is one more user of that boot, not a second viewport setup (per layout-templates §Surface: desktop-native → Primary screens).
- Every control a check acts on carries an author `id` as its stable element id, and no CSS selects by it, as the stand's controls do. A merged password fixture keeps the ids its readers address (per layout-templates §Surface: desktop-native → Primary screens).
- agent-run's output shape: one JSON object per event on stdout, raw output kept in its log file, and the usage line on stderr (per layout-templates §Surface: cli → Primary screens).

## Anti-patterns to avoid
- Putting a fixture case into a lean task's markup, or adding an id, class, style, wrapper or Tab stop to the stand to hold one (per layout-templates §Surface: desktop-native → Primary screens).
- Changing a moved fixture's geometry when it is gathered. Examples: a viewport-scroll fixture no longer taller than 600, or an obstructed or transformed fixture's boxes placed differently. Those geometries are what the moved checks measure (per layout-templates §Surface: desktop-native → IA notes).
- Writing anything other than JSON lines to agent-run's stdout for a new selection or bootstrap step (per layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts §Surface: cli → agent-run row ↔ test-plan §3. The agent-run exit grammar and event schema live in test-plan §3, so a new selection moves both documents. `.claude/rules/verification-harness.md` follows through the wrap's cascade.
- layouts §Surface: desktop-native → Primary screens (CRUD Tab sequence, focus on the list) ↔ a11y SC 2.4.3. `common::tab_order` and `common::ENABLED_TAB_ORDER` sit in `tests/blitz-tests/tests/common/mod.rs`, which the chunk edits, so the stand's Tab tables must not move. A fixture adds no focusable element to a stand task.
- layouts §Surface: desktop-native → IA notes (viewport-scroll fixture) ↔ test-plan §7/§8. The fixture's new shared home is recorded in test-plan §7, and the plan's "in-file fixture" wording is amended by the wrap.

## Acceptance criteria contributions
- (layouts) Every moved fixture check boots through `seven_guis::stand::options(incremental)` at 800 × 600, scale 1.0, Light, with the bundled font, and reads as it did, in both layout modes (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) No stand layout moves: the diff over `examples/seven_guis/src/` is empty, and `stand_keyboard`'s Tab tables and the stand's ids read unchanged (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The viewport-scroll fixture stays taller than the 600-px viewport, and `stand_act_scroll` passes with its viewport scroll intact after the move (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) If the fork adds an agent-run selection, a bad selection still prints the usage line on stderr with empty stdout and exit 2, and each verb's stdout stays JSON lines. With no new selection, the usage line is unchanged (per layout-templates §Surface: cli → Primary screens).
