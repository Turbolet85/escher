
## 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — the upstream focus checks are silent
**Section:** §3 Observability Harness Contract (Logging stack — the `absence · logs` group)
**Change:** a bullet is added: `focus_within` and `focus_redraw`, upstream's own targets, install no subscriber, read no env var and print nothing (0 · 0 · 0 each). `focus_redraw`'s `RecordingShell` is a counting `ShellProvider`, not a sink. The engine changes they pin (`:focus-within` matching, the not-in-document focus refusal, the focus-move redraw; PROVISIONAL) add no log site, span, field or env read. The `Focussed node` INFO line stands after the new gate, and `host_log` and `mcp_log` pass under both builds.
**Why:** the silence census records every new check of the tree (the playbook's «Accurate this-chunk addition»). No raw input reaches a log. The engine take is PROVISIONAL on the operator's delegate answer to F1 (2026-10-19).
**Ref:** .andromeda/runs/2026-10-19T19-54-24-wrap/
