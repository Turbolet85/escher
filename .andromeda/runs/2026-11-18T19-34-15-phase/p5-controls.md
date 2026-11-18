# P5 controls — 2026-11-18-cold-agent-live-run, revision 1 (entry 12 alone)

Entry 12 fired as itself from the store's plan rev 5. On the tree: `gate.py run --chunk 2026-11-18-cold-agent-live-run --entry 12 --run-dir .andromeda/runs/2026-11-18T19-34-15-phase`. In the clone: `gate.py run --chunk 2026-11-18-cold-agent-live-run --docs-root . --project-root $TMPDIR/{scratchpad}/ctl-rev --entry 12`, over a `git clone --shared` of the project at 512112879 with the five evidence files and docs/capabilities.md copied from the tree (cmp-equal); each planted line appended to the clone's transcript copy and the copy restored after (cmp-equal at the end). The tree's evidence was read, never written.

- entry 12 · the real evidence, on the tree · 12 probe       green · exit 0 · 0.11s · 2 B → 12.log
- entry 12 · must-pass: the real evidence copied, unplanted · clone · 12 probe       green · exit 0 · 0.11s · 2 B → 12.log
- entry 12 · must-fail: a planted `{"api_key":"sk-ant-ctl0000000000"}` line · clone · 12 probe       red · last line '0' ✗ (last: '1') · exit 0 · 0.19s · 2 B → 12.log
- entry 12 · must-fail: a planted `{"note":"Bearer ctl0000000000"}` line · clone · 12 probe       red · last line '0' ✗ (last: '1') · exit 0 · 0.19s · 2 B → 12.2.log
- entry 12 · must-fail: a planted e-mail `ctl.person@example.com` · clone · 12 probe       red · last line '0' ✗ (last: '1') · exit 0 · 0.19s · 2 B → 12.log
- entry 12 · must-fail: a planted `Authorization: ctl0000000000` header · clone · 12 probe       red · last line '0' ✗ (last: '1') · exit 0 · 0.19s · 2 B → 12.log
- entry 12 · must-pass: `{"apiKeySource":"none"}` planted a second time · clone · 12 probe       green · exit 0 · 0.2s · 2 B → 12.log

Per-arm counts on the tree's evidence (script, counts only): the take-up's `api[_-]?key` 1 (live-transcript.jsonl, the `apiKeySource` name, its value `none`), the revised api-key arm 0, `authorization:` 0, `bearer ` 0, e-mail 0; the whole pattern 1 before, 0 after.
