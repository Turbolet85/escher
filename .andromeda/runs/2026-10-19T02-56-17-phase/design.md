# design extract

## Relevance
partial — the chunk renders no UI and no rendered-surface tokens apply, but recording touches the agent-read `escher-session` command line and its `mcp` role. Whether recording appears there as a verb, an argument, an answer key or a self-description key is a P4 question.

## Constraints
- Any command-line form recording adds (turning it on, getting the kept check back, a status reading) prints exactly one line of JSON on stdout, keyed by the driver schema's own words, and nothing else there (per design-system §Surface: cli §Component Patterns).
- No colour and no colour dependency; no ESC byte on any answer line; no code asks whether a stream is a terminal, so a recording-related answer is the same piped or not (per design-system §Surface: cli §Component Patterns).
- The exit status carries the class `0` accepted · `1` refused · `2` usage · `3` session error, read from the schema's `EXIT_STATUSES` table that `help` serves. A recording failure maps into these four classes. Whether a new class would be needed is P4's question, and a new exit would be a widening under CLAUDE.md's Critical Warnings (per design-system §Surface: cli §Component Patterns).
- A usage error prints one fixed line on stderr with stdout empty. A session error prints its fixed message on stderr, and its JSON object carries its fixed `remedy`. A recording-specific usage or session error keeps that shape and writes back nothing the caller typed (per design-system §Surface: cli §Component Patterns).
- In the `mcp` role, stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, every reply one uncoloured line with no ESC byte. An `Err` ending's fixed message goes to stderr with status `3`. Recording an MCP session leaves that stream unchanged (per design-system §Surface: cli §Component Patterns).
- `escher-session help` answers one uncoloured line of JSON with status `0`, reaching no session. If recording adds a verb or an argument, it shows up in that same line and nowhere else (per design-system §Surface: cli §Component Patterns).

## Patterns to follow
- The seven_guis checks' shared answer reader asserts the no-ESC reading on every command-line answer line. Run any new recording command through it (`examples/seven_guis/tests/common/mod.rs`) (per design-system §Surface: cli §Component Patterns).
- The shared `Speaker` asserts the no-ESC reading on every MCP reply. Drive a recorded MCP session through it (per design-system §Surface: cli §Component Patterns).
- Exit meanings come from one table, `EXIT_STATUSES` in `packages/escher-driver/src/schema.rs`, which `help` serves. A recording outcome's status is read from that table and is never a literal added at the call site (per design-system §Surface: cli §Component Patterns).
- Answers are written in one place, `packages/escher-driver/src/json.rs`, as uncoloured JSON lines. A recording answer goes through the same writer (per design-system §Surface: cli §Component Patterns).

## Anti-patterns to avoid
- Printing a progress note, a path notice or any other human-facing line on stdout next to the answer, for example "recording to …". stdout is the answer only (per design-system §Surface: cli §Component Patterns).
- Making output depend on the terminal (TTY detection, colour, ESC sequences) for anything recording prints (per design-system §Surface: cli §Component Patterns).

## Contract bindings
- CLI output form ↔ test harness: the no-ESC and one-line readings are pinned by the seven_guis shared reader and by `Speaker`. New recording paths join those checks (test-plan §3).
- Exit statuses ↔ the self-description: `EXIT_STATUSES` feeds `describe()`'s `exit_statuses` key, so a recording outcome's class has to match a served meaning (schema ↔ `help`).
- stderr's fixed-message rule ↔ observability: a recording error's stderr text stays a fixed table string, which is consistent with the sink rule that no id, name or typed text reaches stderr (obs-plan §3).
- No a11y token binding: the chunk adds no rendered UI, colour or motion.

## Acceptance criteria contributions
- Every stdout line from a recording-related `escher-session` command is one line of JSON with no ESC byte, asserted by the seven_guis shared answer reader (per design-system §Surface: cli §Component Patterns).
- Every MCP reply during a recorded `escher-session mcp` session is one line with no ESC byte, asserted by `Speaker` (per design-system §Surface: cli §Component Patterns).
- Every recording-related command exits with one of the four `EXIT_STATUSES` classes. A usage error leaves stdout empty and puts one fixed line on stderr (per design-system §Surface: cli §Component Patterns).
- A search of `packages/escher-driver/` still counts 0 terminal tests after the change (per design-system §Surface: cli §Component Patterns).
