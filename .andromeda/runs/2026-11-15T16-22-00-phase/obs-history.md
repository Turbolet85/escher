@history_item n=1 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — ~ decision/no-file-loads-from-non-file-documents: the rule now says the browser app's favicon and link navigation drop a non-file: document's file: load silently, adding no log site (was «refused as a typed forbidden error»). This is the gate and the `make_doc_config` hand-off this chunk proves by tests. The chunk changes no behaviour, so the row stands as written, and the new tests must add no log site, print or typed error.

@history_item n=2 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — ~ decision/no-file-loads-from-non-file-documents: the rule gained the engine sub-resource refusal (blitz-net `FileForbidden`, fixed text naming no URL or path, reported through the handler's error path, adding no log site). This is the `net_provider` side of the config whose dropped-`net_provider` mutant this chunk kills, so a test reading the refusal must assert it without any URL showing up in output.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the `file:` refusal in the script error surface; the take read at `ebacd68bd` silent) This entry is the origin of the no-file-loads row: `document_may_load` refuses a file: load from a non-file: document with no log site of its own. It also set the precedent that the file:-refusal tests `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` were recorded as an observed-absent row (no print or log site), their one census hit being a test-only `TMPDIR` read through `std::env::temp_dir()`. The new browser tests follow the same pattern; a temp-dir read in them would be the same kind of test-only env site.

@history_item n=4 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — ~ decision/checks-silent · except: a re-run child now prints ids or digests its parent reads. The row binds the browser app's new tests: no subscriber, no env read, no print, readings into assertions only. None of the named exceptions (telemetry checks, MCP recorder, paint-tree bench env var, re-run child) covers a browser test.

@history_item n=5 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — Recorded `focus_redraw`'s `RecordingShell` as a counting `ShellProvider`, not a sink. This is the precedent for the stand-in `ShellProvider` context this chunk's `make_doc_config` test needs (it calls `consume_context::<Arc<dyn ShellProvider>>()`). The stand-in must count or no-op only and must not install a subscriber or print, so checks-silent and headless-stand-no-sink hold.

@history_item n=6 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the Dioxus runtime probe's silence row) A test target that runs a Dioxus runtime was recorded silent by the silence census (`println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`, exit 1, reading 0) in a log-coverage observed-absent row alone, with §3's body unchanged. This is the model for recording the browser's new `VirtualDom`-hosted tests; their failure messages should name no URL or path read.

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the chunk's surfaces in §6, the census's carve-out) An inline `#[cfg(test)]` module (blitz-shell `event.rs`) was held out of the added-lines census under `packages/`, so that census's 0 did not cover it. The browser tests will likely land as inline test modules in `apps/browser/src/` (`document_loader.rs`, alongside `history.rs`'s), so the silence reading has to be run over those files explicitly, not inferred from an added-lines census.

@history_item n=8 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — Origin of decision/headless-stand-no-sink: a boot made in process installs no subscriber, prints nothing and reads no env var, so it has no escher sink. A browser document built in a test through `make_doc_config` is such an in-process boot and must install nothing.

@history_item n=9 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — Narrowed «a headless boot has no escher sink» to a boot made in process, a boot by the `escher-session` binary running under its sink. This is the current wording of headless-stand-no-sink and the second-installer half of scrub-sink-reach (two seven_guis binaries). Trap it recorded: a zero read from a host that installs no sink says nothing about one that does. It also added the session state directory row (local, uploaded by no CI leg) under local-state-never-uploaded.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — Added the one exception to the stand's no-subscriber census, `stand_act_spans`' two re-run children installing the sink over an in-memory capture. This is the except clause of headless-stand-no-sink. A browser test is not among the exceptions.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — Recorded `stand_mcp_tools`' in-process recorder as a second exception to «installs no subscriber». That exception is now in the except clauses of checks-silent and headless-stand-no-sink, and a browser test adds none.

@history_item n=12 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — The first exception to the stand checks' no-`println!` rule: a re-executed child's stdout captured and read only into its parent's assertion is not a log channel. This is the root of checks-silent's re-run-child except. The browser tests need no such child.

@history_item n=13 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — Widened the no-subscriber census to cover the checks' shared modules, not just the check files. This is the «shared modules and the stand-ins they use» of decision/checks-silent: any helper or `ShellProvider` stand-in the browser tests share is held to the same silence.

@history_item n=14 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — Built escher-telemetry's sink and allowlist scrub, which reaches escher's sink only, the upstream apps' stdout `fmt::init()` staying unscrubbed. This is the origin of decision/scrub-sink-reach: the browser app is an upstream app outside the scrub, so a log site added there would print unscrubbed, one reason the chunk adds none.

@history_item n=15 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — Set the sink's three outcomes (engine targets scrubbed, `escher_` targets printed with content fields redacted, every other target dropped). Rule for later chunks: a target is admitted only together with a scrub rule for it. This bears on scrub-sink-reach, and the chunk admits no target.

@history_item n=16 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — Added the agent-run harness state `target/agent-run/` as local only and uploaded by no CI leg. This is the first member of decision/local-state-never-uploaded, which the browser tests' runs (agent-run or CI `test` leg) leave unchanged.

@history_item n=17 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — Added the cold-agent pipe state as local only, gitignored and uploaded by no CI leg. This is a member of decision/local-state-never-uploaded.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — Added the Recording directory row (`0700`, local only, uploaded by no CI leg). This is the recording-directory member of decision/local-state-never-uploaded.

@history_item n=19 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — Each ci.yml leg's output goes to `target/ci-logs/{leg}.log`, uploaded only on failure as `ci-log-{job id}`, unscrubbed build output carrying no user data. The browser's new tests run in the workspace `test` leg, so their failure messages land in that artifact and must carry no page URL or path.

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — Converted the master's text into rows, verified against the code. Every picked row (checks-silent, headless-stand-no-sink, no-file-loads-from-non-file-documents, scrub-sink-reach, local-state-never-uploaded) took its current form here, and later changes reach it only as `~`/`+`/`−` row amendments.
