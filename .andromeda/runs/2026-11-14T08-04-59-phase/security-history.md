@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (signing jobs repository-guarded; CI legs locked) — `--locked` made to cover every ci.yml cargo leg, the flake and `dx bundle` (`examples/wasm_hello` excepted). This is where decision/locked-builds comes from. Any CI leg this chunk adds for the cross-OS reading proof must pass `--locked`.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (cargo-deny audit leg, SHA-pinned ci.yml) — set up cargo-deny 0.20.2 over the six ci.yml platforms and SHA-pinned every ci.yml `uses:`. It also added the one ignore RUSTSEC-2026-0192 (ttf-parser, a font-parsing crate, reached through winit). This is where decision/cargo-deny-audit-tool and dependency-audit-ci-gate come from. The scope promises no new crate, and font work must not pull a new font parser into the audited graph.

@history_item n=3 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's sink drops every record from a target outside its allowlist at every level and redacts content-named fields (decision/telemetry-sink-target-allowlist). Any font-loading or fontique log record that turning off system fonts brings up is judged by this sink. The scope allows no new log field or span.

@history_item n=4 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — fixed refusals as a closed cause set with fixed, path-free texts, so a refusal holds nothing a call supplied (decision/closed-refusal-cause-set, refusal-closed-cause-set, refusal-holds-no-call-input). `fonts-unavailable` is one of those causes. The scope keeps it and keeps its text unchanged.

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (one span, eight fixed-word or count fields) — only a refusal's cause name reaches a log, as the command span's `cause` field (decision/refusal-log-cause-name-only, contract/command-span/cause). A refused `fonts-unavailable` shows up only as that word. The chunk must add no span or field.

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (hint-less `@font-face` source fetched and sniffed, PROVISIONAL) — recorded the hint-less `@font-face` fetch-and-sniff as a boundary widening that came in by upstream merge. It also states that escher routes no agent- or user-supplied URL through the font-face source path, and that the stand boots offline on its bundled font. This is the origin of decision/no-agent-urls-through-font-path and hintless-font-face-source-sniffed. The bundled set must keep the font off any URL path.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (hint-less `@font-face` source ratified) — removed the PROVISIONAL mark, so the founder ratified the sniffed-source widening as upstream's behaviour, with a check owed. The routing sentence stays: no driver verb takes a URL, and the stand boots offline. That sentence is the trap if bundling moves fonts through a URL or `@font-face`.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings; the closed-field rule) — set the rule that a new field of a closed value set inside a ratified carrier is no widening, while authored text, an open set, a new carrier or a new exit still is (decision/boundary-widening-rule, closed-value-field-no-widening, snapshot-state-readings). This is the test to apply to any reading the cross-OS proof compares or records.

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and fixed text through the validated surfaces) — the program's own fixed `'static` text crossing out on ratified carriers is inside the ratified crossing, and page, call or screen text is not. This covers cause texts like `fonts-unavailable`'s, which the scope keeps unchanged.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (screen pixels on the three carriers) — added `screenshot`, and `fonts-unavailable` as the cause that refuses a shot of an app with no font it can paint text with, before anything is painted. The scope keeps exactly this refusal. A shot's pixels are not a closed value set, and the chunk compares readings, never shot bytes.

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the engine's password mask, nine causes, no image in a log, `png`) — blitz-paint paints a password as bullets laid out by `layout_as_text_input`. The frame still depends on the value's length, and the caret depended on glyph widths. The cause set gained `fonts-unavailable`, and blitz-test-harness took `blitz-paint`. With bundled fonts the bullet and caret geometry come from the bundled set. decision/mask-holds-on-every-carrier must still hold.

@history_item n=12 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness took `color` as a reviewed dependency and recorded that it takes no `tracing` and no `style` (decision/test-harness-no-tracing-no-style). The chunk changes `HarnessOptions::font_ctx` defaults. Any harness dependency change must keep that rule and be reviewed as a Pinning bullet.

@history_item n=13 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (password click caret; off-Linux bound measured) — a click-placed caret depended on the real characters' glyph widths, and the off-Linux state-dir bound was measured on the macOS leg. This is a precedent that font metrics leak through shots, and that macOS-leg measurements are how cross-OS facts get recorded.

@history_item n=14 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (whole-workspace audit, licence gate, owed checks discharged) — `cargo deny --locked --workspace --all-features check advisories licenses` and the licence gate on every push (decision/licence-gate, dependency-audit-cargo-deny). It also added `font_face_sniffed_source`, the guard of decision/hintless-font-face-source-sniffed. The font asset is a file and not a crate, so the licence gate does not read it. The chunk must not break that guard.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (eight causes, `advance` on every session, the fade on the document's clock) — `time-unavailable` was retired, leaving the closed cause set at eight, `fonts-unavailable` among the executor's. A determinism engine change made two shots of one state read the same pixels on one host. This is the precedent for host-independence work, and it keeps the cause set closed at eight.

@history_item n=16 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls, the password caret by the bullets) — the password caret is placed by laying out U+2022 bullets, so no width of the real characters reaches the caret or a shot. Bundling the font changes the face those bullets are measured in. The mask's guarantee must hold across hosts.

@history_item n=17 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check at rest, its directory, its reader and `--record`) — kept checks are `0600` in a `0700` directory, never logged, and read only by the strict, bounded `Check::from_bytes` (decision/kept-check-owner-only, recorded-check-strict-reader, recorded-check-untrusted). This applies if the cross-OS proof is held as a kept check, as the scope leaves open.

@history_item n=18 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest, the committed checks, the replay) — the only committed checks are the two stand-Counter sessions, with no typed text, host path, name, secret or `screenshot` step. A shot is kept as width and height only, never pixels. Replay runs through `Session::run` alone (decision/committed-counter-checks, kept-check-answer-whole, replay-through-session-run). A committed cross-OS reading would be a third committed check and goes beyond this exception.

@history_item n=19 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process lint and the text extent reader) — `BaseDocument::text_extent` is read-only and answers none when unresolved. Lint findings hold ids, closed defect words and numbers only (decision/text-extent-read-only-none-when-unresolved, lint-findings-closed-content). Text extents and lint findings are among the readings the chunk compares across OSes, and they depend directly on the font set.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb on every carrier) — `fonts-unavailable` also refuses a `lint` before anything is read, and findings cross every ratified carrier, never logged (decision/lint-findings-crossing). Keeping `fonts-unavailable` covers this lint arm too.

@history_item n=21 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — restated the cause set as eight, with the executor returning six, `fonts-unavailable` among them. The carried sets also name a lint's findings. This is the current count the chunk's "no new cause" boundary is read against.

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs, the load page's in-process stand-in) — the stand boots on its bundled font, offline except for the load page. That page's one fixed stylesheet is answered in process by `LoadNet`, with no `blitz_net`, no `@font-face` source and no agent- or user-supplied URL. This is the current statement of how the stand gets its font, which the chunk extends to every run path.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script fetch's `file:` refusal by document, a narrowing) — `document_may_load` lets a `file:` URL load only for a `file:` document, recorded as a narrowing and not a widening. This is the origin of decision/file-urls-only-from-file-documents and file-url-only-for-file-documents.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `matrix_test` free-disk step retired) — the `matrix_test` job holds no free-disk step, which `test_only_stand_repeat_frees_disk` pins (decision/free-disk-only-on-stand-repeat). A cross-OS check added to the `test` matrix must not reintroduce one.

@history_item n=25 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` leg, locked) — added `--features blitz-paint/font-embolden --test select_popup_painted` as a locked cargo leg with no CI job of its own, with macOS exercising the feature. This is a precedent for a font feature that differs by OS and for how such a leg is locked and placed.

@history_item n=26 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (advisories published after a green audit) — a CI audit can go red on new advisories over an unchanged lock, so the audit leg is run locally before a push (decision/local-audit-before-push).

@history_item n=27 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Threat Model Summary) — changed decision/file-url-only-for-file-documents so that blitz-net refuses the engine's stylesheet, @import, font, image and iframe `file:` loads before any disk read, except for a `file:` document or a top-level load. Bundled fonts must not reach a document through a `file:` font load that this rule refuses.

@history_item n=28 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Input Validation) — widened decision/file-urls-only-from-file-documents to every engine font load, and added the one stamping request constructor for sub-resource requests. Any font request the chunk builds in blitz-dom must go through that constructor.

@history_item n=29 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Error Handling) — added contract/blitz-net-provider-error/file-forbidden, a fixed, path-free refusal at the provider boundary. Applies if a font load meets the `file:` gate.

@history_item n=30 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — the sibling v030 host-independence chunk added decision/seeded-source-in-process-seed-only: its seed comes from in-process code only, with no outside input. This is the direct precedent for keeping the bundled font set in-process, with no new external input.
