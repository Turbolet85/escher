
## 2026-10-10-driver-cli — `escher-session` is the driver's command line
**Section:** §Surface: cli → Primary screens (the `escher-session` bullet) · §Surface: desktop-native → Primary screens (the seven_guis bullet)
**Change:**
- cli: was "the session host … usage `escher-session <counter|flight-booker|timer|crud> <state-dir>`, a closed argv of exactly two arguments", exits 0 · 1 · 2, nothing on stdout; now every command is `escher-session <verb> [arguments] --session <dir>` over the driver's nine verbs — the six instance-level ones with their arguments as flags (`--id`, `--text`, `--key`, `--ms`, `--shift`), then `start <task>`, `status`, `stop` — with four endings: accepted, one line of uncoloured JSON on stdout, exit 0; refused, exit 1; usage, one fixed line on stderr, exit 2; session error, exit 3. The host role is `serve <task> --session <dir>`, spawned by `start`. The former two-argument form reads refused, `unknown-verb`. The two citations of the old `main` are replaced by the reader's and the endings'.
- desktop-native: the third consumer of the stand's mount is named as the binary in its host role; a client command of the same binary boots nothing. The layout statement is unchanged: no markup, id, class, style, wrapper or order added, a held instance reads as a fresh boot.
**Why:** the chunk made the binary an agent's command line. The operator ruled at this wrap that the id `flow-names-no-element`, which the counter flow names and no screen reads, is the flow's own refusal step and no surface — so this plan lists no such id.
**Kept:** unix only in 0.1.0; the host's stderr log lines still carry `service.name=seven_guis`.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/
