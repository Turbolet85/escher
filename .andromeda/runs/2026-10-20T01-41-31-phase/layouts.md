# layouts extract

## Relevance
partial — the chunk changes no screen; it reaches the cli surface's `escher-session` output (the acting verbs' answers, `help`'s self-description, the MCP role's tool listing) and adds an in-file fixture page beside the stand, whose own mount must stay as it is.

## Constraints
- An acting command that waits on a load still has one of the four endings, each read from `EXIT_STATUSES`, and an accepted answer is one uncoloured line of JSON on stdout with exit 0. The bound running out adds no fifth ending or exit status. Whether each acting verb today returns `settled: false` with `busy: "loads"` in its accepted line is research's question. (per layout-templates §Surface: cli → Primary screens, `escher-session`)
- The stated bound has to reach `help` as fixed table text inside the self-description: `escher_driver::describe()`, one line of JSON with exactly seven keys (`verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`), every text a fixed string. No eighth key, and `help` still reaches no session. (per layout-templates §Surface: cli → Primary screens, `escher-session`)
- The `mcp` role still lists the eleven verbs in order, `help` last. Each tool's description is the verb's help, then `. Returns: `, then its result fields, so bound text added to a verb's help shows up in that tool's description. (per layout-templates §Surface: cli → Primary screens, `escher-session` `mcp` role)
- The argument grammar stays closed: flags are spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms`, `--shift`, `--session`, `--record`), and no ending writes back anything the caller typed. A bound is not a call argument, so it adds no flag and no usage-line word. (per layout-templates §Surface: cli → Primary screens, `escher-session`)
- The headless stand mount stays as it is: one lean task in TaskShell (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`), the pinned 800 × 600 viewport at scale 1.0 in Light, and a held instance that reads as a fresh boot in both layout modes. The load fixture is an in-file page beside the stand and moves no stand markup, id, class, style, wrapper or order. (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell)
- The fixture's viewport is the harness default, 800 × 600 at scale 1 in light mode, unless the fixture states otherwise. (per layout-templates §Surface: desktop-native → IA notes)

## Patterns to follow
- An in-file fixture page beside the stand, used where the stand lacks a case, as the viewport-scrolling fixture taller than 800 × 600 is. (per layout-templates §Surface: desktop-native → IA notes)
- Every element the check reads carries an author `id` as its stable element id, so the loaded content shows up in the returned diff and the next snapshot by id, with no class, style, wrapper or order added for it. (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell)
- The tool's self-description is extended the way the `--record` and `screenshot` additions were: fixed table text that reads the same with and without a session or recording. (per layout-templates §Surface: cli → Primary screens, `escher-session`)

## Anti-patterns to avoid
- Adding a key to `help`'s seven-key line, a result field to an acting verb's answer line, or an exit status for the bound running out. (per layout-templates §Surface: cli → Primary screens, `escher-session`)
- Changing stand markup, ids, classes or styles to host the load case. The lean tasks must still read 0 lint findings at boot. (per layout-templates §Surface: desktop-native → IA notes)

## Contract bindings
- cli `escher-session` ↔ test-plan §3 Session lifecycle: the session contract the binary serves, with its exit grammar. (per layout-templates §Surface: cli → Primary screens)
- `help`'s and the MCP tool descriptions' bound text ↔ the arch Driver session contract and security-plan «Settle and loads in flight», where the scope says the bound is also stated. (per layout-templates §Surface: cli → Primary screens)

## Acceptance criteria contributions
- (layouts) `escher-session help` still writes one uncoloured line of JSON with exactly the seven keys, and the stated bound appears there as fixed text. (per layout-templates §Surface: cli → Primary screens)
- (layouts) `tools/list` in the `mcp` role still holds eleven tools in verb-table order, `help` last, each description in the form `{help}. Returns: {fields}`. (per layout-templates §Surface: cli → Primary screens)
- (layouts) On the arm whose load is never answered, an acting command run through `escher-session` ends accepted: one line on stdout, exit 0, no new exit status. (per layout-templates §Surface: cli → Primary screens)
- (layouts) The four lean tasks still boot in TaskShell at the pinned 800 × 600 with no stand markup, id, class or style moved, and read 0 lint findings at boot in both layout modes. (per layout-templates §Surface: desktop-native → IA notes)
