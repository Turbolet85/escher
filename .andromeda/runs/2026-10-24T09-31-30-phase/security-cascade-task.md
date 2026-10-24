# Task — security · 2026-10-24-upstream-sync-polish-and-ship

## The chunk
The seventh upstream sync: upstream/main (DioxusLabs/blitz, f9cbb2756, 16 commits past the last sync's 17de049b1) taken whole, as engine-code cherry-picks onto the registry pins, or not at all. Which shape is chosen at P4.
The upstream delta moves engine code in blitz-dom (inline layout split, class atoms on ElementData, out-of-flow flag on Node, serialised style traversal in config.rs, aspect-ratio natural height, hit testing that skips dropped nodes, Stylo keywords). It also moves pins (git Taffy, git Stylo v0.23-pre, Dioxus 0.8.0-alpha.3 with a resync of dioxus-native and dioxus-native-dom) and upstream-only surfaces (mimalloc in apps/browser and the WPT runner, flake.nix).
It fixes the red Dependency audit (RUSTSEC-2026-0386, tokio 1.53.2, reached through accesskit_unix ← accesskit_xplat ← blitz-shell) by a semver-compatible tokio update. clipboard-win 5.4.1's yanked warning stays a warning.
It re-reads the sink allowlist and the env reads against upstream's delta, re-reads the per-OS tallies and resolves the predicted conflicts in Cargo.lock, layout/construct.rs, mutator.rs and scrolling.rs without changing escher's contracts.

## Needs
- The dependency audit: cargo-deny over the whole workspace graph with deny.toml. An advisory is fixed by a semver-compatible update or ignored by ID with a written reason, never by a blanket allow. The tokio advisory is fixed here, and the standing ignores (paste, the zip trio, the zbus family) and the unsound default (memmap2) are re-read against the new lock.
- The pinning rule: no git dependency in the root manifest, with taffy as a registry version. Upstream's git Taffy, git Stylo and pre-release Dioxus would reverse it, so the pin shape is a fork decided at P4.
- How each sync takes dependencies: upstream's set whole, or by cherry-pick moving none. The lock is read name by name against upstream's or against the tree before it, and the audit leg is green after it.
- The coupled pins (html5ever ↔ stylo web_atoms, skrifa ↔ parley/vello) move as one set. A one-sided bump is never allowed.
- Cargo.lock stays committed and every build passes --locked, including flake.nix, which upstream edits. This is lockfile verification under supply-chain integrity.
- The new crate mimalloc (apps/browser, WPT runner) is reviewed by hand and run through the licence check in deny.toml's [licenses] table, which gates on every push.
- Manifest shape across the six packages the Stylo move touches: workspace = true inheritance and the inline versions listed per crate (dioxus-native's cfg-if and android-activity, blitz-shell's android-activity) can move with upstream's manifests.
- CI tooling pins that a Dioxus 0.8 move can reach: dioxus-cli pinned at 0.7.8, and SHA-pinned uses: entries if upstream's workflow edits come in.
- The repository guard on upstream-only workflows (publish-browser, wpt, wpt-post-results) stays if the take carries any workflow edit (the WPT runner commit f8723f495). A ref-only if: would re-arm their secrets on the fork.
- The unsafe calc-pointer blocks in stylo_taffy convert.rs and blitz-dom layout (mod.rs, table.rs). A Stylo version move can reach them.
- escher's log sink (escher-telemetry target allowlist: engine targets with safe fields only, escher_* targets redacted, every other target dropped) is re-checked against every new tracing or log call site, target and print in the delta. Engine telemetry stays behind each crate's tracing feature, and dioxus-native's tracing feature also enables it in the crates below dioxus-native. The standing capture checks stay at 0 needles.
- The registry of environment values read (secret management). examples/multi_doc.rs (#1243) adds a MULTI_DOC_N read and a read named by a parameter. Whether an upstream root example's env read must be registered, as an Occupied Resources item and in the env list, is P3's reading.
- Stdout output: examples print through println!, and the delta adds one println! in examples/multi_doc.rs. No escher stdout carrier changes.
- The remote-content boundary through the engine code the take carries: iframe and @import recursion caps, the discard of stale iframe responses, typed parse failures and no new panic path on input. This applies to mutator.rs (the iframe src skip), layout/construct.rs and the hit-testing change in node/node.rs.
- Markup-attribute readers in files the delta reaches (accessibility.rs, events/pointer.rs, events/keyboard.rs, node/element.rs, mutator.rs, document.rs). Each keeps its rules: disabled and inert read by presence, the ARIA states and the toggle by role read through closed sets with no text carried out, and the range and select models held within their rules. The select model's merge surface (CARRY 5) and the engine-debts merge surface (CARRY 3) are re-read per file.
- An attribute read or input path that upstream adds in this merge is a boundary widening by upstream merge. Like the earlier font-face sniff and aria-describedby reads, it needs a closed shape or a recorded ratification as upstream's behaviour.
- The stable element id and the masked value readings in dioxus-native-dom (MASKED_VALUE for password and file inputs in the snapshot, its text and its diff). The Dioxus 0.8 resync replaces upstream's side of dioxus_document.rs, events.rs, lib.rs and mutation_writer.rs and adds mutation_buffer.rs. stand_element_ids, stand_id_persistence, stand_id_edits and the snapshot checks decide.
- The Dioxus mutation input rules: data-dioxus-id parsed as usize, unsupported attribute value types ignored, falsy boolean attributes removed, dangerous_inner_html through set_inner_html. These live in mutation_writer.rs, which the resync rewrites, and the new mutation_buffer.rs is read against them.
- Error handling in dioxus-native-dom: try_doc, try_element_to_node_id, the NodeHandle panics, the element_to_node_id unwrap, and the event-data conversion unwraps and todo! paths. Under a take of 286874cc1 their cited files move or are deleted (keyboard_compat.rs, whose retirement drops the coverage leg's filename exclusion). upstream's stale_node_mapping.rs test already appears there as a stale-id panic guard.
- Graceful degradation and panic paths in blitz-dom: stale node ids skipped in resolve.rs, the hit-testing early return, and the unreachable! sites in layout/construct.rs and layout/inline.rs. The inline layout split (build.rs, layout.rs) moves those citations, and the new hit_test_dropped_nodes.rs test lands beside them.
- The document clock invariant: no Instant::now() or SystemTime::now() in blitz-dom outside clock.rs, and time reads carry no epoch unwrap. It is re-read on the take's tree in config.rs, document.rs, events/pointer.rs and the conflicted scrolling.rs (CARRY 4).
- File-input path handling: the chosen file's path sits in the value attribute (events/pointer.rs) and is masked in the snapshot. Its data-protection citations move if pointer.rs or snapshot.rs move.
- No new crate, port, env var, listener or workspace member of ours. Any members change or new package in the lock goes through the Occupied Resources registration first.
- The two committed stand-Counter checks (tests/blitz-tests/checks/, the one exception to owner-only). A Taffy or Stylo move can change the bounds they keep. If they are re-recorded through record_checks.sh, they still hold no typed text, host path, name, secret or screenshot step.

## Not needed
- The driver session socket, its wire v3, its state directory, labels, idle expiry and answer bounds: the chunk adds no verb, request or carrier.
- The MCP server's framing, bounds, protocol errors and echoed-id rule: serve_mcp is not changed.
- The driver command schema, validate, the refusal causes, session errors and the command and tool_call span fields: no schema change.
- The recording directory, Check::from_bytes and replay rules beyond the committed checks: recording is not changed.
- The cold-agent pipe, the operator's Claude Code login and the model-provider crossing: not run by a sync.
- CI signing material, the Signed Builds environment and the publish job's secrets: the fork runs none of them, and only the repository guard can be reached.
- The browser app's sqlite history, HTTP cache, urlbar, external-browser navigation and error pages: mimalloc changes only their allocator, not their inputs or storage.
- rdme markdown rendering and the CLI-argument rules of the screenshot, paint_bench and preact_script examples: not in the delta.
- The WPT runner's local file reads, its quarantine and report checks: its upstream commit changes the runner and its allocator, not these input rules.
- The CLI-argument rules of agent-run.sh, cold-agent.sh, apt-install.sh and coverage_floor.py: the scripts are not changed. Only ci-leg.sh's coverage filename exclusion can move.
- The serde_json, png and color dependency entries: no escher-driver or harness dependency changes.
- Authentication and authorization: none exists, and the sync adds none.
- Fixture pages' subresource integrity, fixture tokens and fixture fetches: no fixture changes.
- The JS console, script fetcher and blitz-vibey-script runtime rules: no upstream commit in the delta targets script execution. Only a Stylo manifest move can reach that crate.
