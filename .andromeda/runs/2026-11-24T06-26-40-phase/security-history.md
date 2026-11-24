@history_item n=1 marker=amd-mb3325ds
**amd-mb3325ds** — 2026-11-19 upstream sync (the last two-parent merge before this one): `decision/tls-via-native-tls` became `decision/tls-via-rustls` and `decision/clipboard-via-arboard` became `decision/clipboard-via-winit`. This is the nearest precedent for a merge that brings an upstream dependency or transport move into the Data Protection rows. The cards were applied PROVISIONAL and later ratified.

@history_item n=2 marker=amd-zcqv7cvj
**amd-zcqv7cvj** — 2026-11-19 upstream sync: `decision/session-socket-sole-listener` was narrowed to "every default build" and `decision/cdp-server-opt-in-loopback` was added for the merged CDP listener. This is the precedent for registering a merge-borne input surface (here, the OS drop source in blitz-shell `dnd.rs`) as its own row, and it bounds this chunk's "no new crate, port, env var or listener of ours".

@history_item n=3 marker=amd-7rqmidgz
**amd-7rqmidgz** — 2026-11-19 upstream sync: added `decision/event-is-trusted-by-source`, `decision/programmatic-focus-events`, `decision/cancelled-pointerup-still-clicks` and `decision/svg-mask-url-fetch`, and amended `decision/accessibility-tree-refresh-on-change`. These are the events-driver rows that `22c18e38f` (preventDefault ignored on a non-cancelable event) and `82a6573ee` (drag events in blitz-dom's driver) touch. Each was put to the founder as an engine change for every document by upstream merge.

@history_item n=4 marker=amd-w2mopkh3
**amd-w2mopkh3** — 2026-11-19 upstream sync: `decision/engine-dependency-pins` moved to Stylo 0.24 and winit =0.31.0-beta.4, Dioxus held and Taffy unchanged, and `decision/skia-default-build-fetch` was added. This chunk's Taffy 0.16 move must amend the same pins row again, with Dioxus still held at 0.7.3.

@history_item n=5 marker=amd-j775z4uj
**amd-j775z4uj** — 2026-11-19 upstream sync: the merged devtools env reads were registered as contracts `contract/environment-variables/blitz-cdp-port` and `blitz-devtools-frontend-dir`. This is the precedent for the "sink allowlist re-checked … env read" sweep of the merged delta: any env read upstream brings gets a row.

@history_item n=6 marker=amd-a4exgumy
**amd-a4exgumy** — 2026-11-19 upstream sync: `absence/served-network-api` and `overview/api-security` were restated for default builds once the merge brought an opt-in TCP listener. It shows that a merge-borne surface also moves the absence and overview rows, not only the decisions.

@history_item n=7 marker=amd-rp2475ii
**amd-rp2475ii** — Ratification of the 2026-11-19 sync's Input Validation card (`decision/event-is-trusted-by-source`, `programmatic-focus-events`, `cancelled-pointerup-still-clicks`, `svg-mask-url-fetch`). The rows had been PROVISIONAL on a delegate's confirm until the founder's own word. Trap: a merge's widenings stay PROVISIONAL until the founder rules, and this chunk's drop input and file reads are recorded as ratified on his own word (D2 A).

@history_item n=8 marker=amd-wrwpi4o7
**amd-wrwpi4o7** — `decision/no-git-dependency` and `decision/no-git-dependencies` gained the Taffy-only git exception, and `decision/upstream-sync-dependency-set` gained the lock-only semver move. This exception is the one the Taffy 0.16 registry move retires (the founder's T2 A, "the git exception is retired in every row").

@history_item n=9 marker=amd-7p5dqgzq
**amd-7p5dqgzq** — Taffy moved to the upstream git pin. This changed `decision/engine-dependency-pins`, cut taffy from `decision/no-git-dependencies`'s registry list, made `decision/no-git-dependency`'s exception name a no-checksum rev pin, and let `decision/upstream-sync-dependency-set` take Taffy's own git pin. Every one of these clauses reverses when Taffy 0.16 comes back from crates.io with a checksum.

@history_item n=10 marker=amd-hvp26wf6
**amd-hvp26wf6** — `decision/engine-dependency-pins` moved the Taffy rev to `a3997d55`. That is the git pin the merged lock replaces with registry 0.16.0.

@history_item n=11 marker=amd-sqg7k5xq
**amd-sqg7k5xq** — `decision/upstream-sync-dependency-set` was restated in its standing form: a coupled registry set upstream moved, packages entering with it, git content only by Taffy's pin. This is the rule this take's lock diff is read against (taffy plus `url` in blitz-shell's list). Its "git content only by Taffy's own pin" clause goes stale with the git source gone.

@history_item n=12 marker=amd-vyxqxxcu
**amd-vyxqxxcu** — 2026-10-13 sync: the earlier Taffy git → crates.io 0.15 move ("no git dependency is left in the root manifest", a checksum in the lock), with the sync's dependency delta recorded and the audit green with `deny.toml` unchanged. This is the direct precedent for this chunk's Taffy 0.16 registry move, written as a source moving to the registry with no boundary widened.

@history_item n=13 marker=amd-pttxafen
**amd-pttxafen** — 2026-10-10 sync: every package entering the lock is reviewed by hand (licence, repository, build script, proc-macro, dependent). Unsafe code: the second `unsafe` sits in stylo_taffy's public `resolve_calc_value`, read through a raw pointer. That is the function whose Taffy 0.16 signature (`taffy::CalcHandle`) forces this take's `layout_reason.rs` adaptation.

@history_item n=14 marker=amd-yy74emme
**amd-yy74emme** — 2026-10-18 sync: upstream's Taffy-main commits naming git Taffy were left out to keep no git dependency, a left-out set named with its reason. This is the precedent for the left-out re-read and for the "Taffy history" behind this chunk's T1/T2 forks.

@history_item n=15 marker=amd-sjn5zfj5
**amd-sjn5zfj5** — 2026-10-24 sync lock record: each upstream sync keeps one lock record, a lock-only semver move (tokio) past an advisory is allowed, and Dioxus 0.8.0-alpha.3 was not taken. This is the precedent for this merge's own lock record and for the Dioxus hold.

@history_item n=16 marker=amd-padkl4a5
**amd-padkl4a5** — 2026-10-27 sync lock record: the pins stay (Dioxus "0.7.3" among them), the audit reads `advisories ok` and `licenses ok`, and every standing ignore is still encountered. This is the form this take's lock record follows.

@history_item n=17 marker=amd-khknn2kt
**amd-khknn2kt** — 2026-11-01 sync lock record: the twelve-commit take with left-out commits named by measured reason, manifests and lock byte-identical, audit green. It is the template for naming each left-out commit with its reason.

@history_item n=18 marker=amd-a7iv2ynn
**amd-a7iv2ynn** — 2026-10-17 sync: upstream's set was taken whole and entering crates reviewed by hand. External-decay advisory ignores (the zbus family) were taken by ID under the operator's word. A sync's audit can go red over an unchanged tree, which matters for this chunk's "audit green" criterion.

@history_item n=19 marker=amd-to6gvm6e
**amd-to6gvm6e** — Reviewed pin: cookie 0.18.3 by a lock-only move. Trap: `cargo update -p` would move the `syn` edges of bindgen and cssparser-macros to 2.0.119, so they were held at 3.0.6 by edit (residual `res-evkq636g`). The merged `Cargo.lock` must be read line by line for such edges.

@history_item n=20 marker=amd-fayh3yih
**amd-fayh3yih** — 2026-10-10 sync: a hint-less `@font-face` source arriving by upstream merge was recorded as a boundary widening, PROVISIONAL until the founder's word. Rule: a widening is never routine whatever brings it, even an upstream merge. This governs how the OS drop input and its `std::fs` file reads are recorded.

@history_item n=21 marker=amd-5qmlqgth
**amd-5qmlqgth** — 2026-10-10 sync: `file:` reads go through `Url::to_file_path`, a typed error rather than a panic, still with no path restriction. The `env!("CARGO_MANIFEST_DIR")` read in upstream's `tests/all.rs` was registered. It bears on the dropped-file reader over `std::fs` (no path restriction) and on the new `tests/all.rs` mod lines.

@history_item n=22 marker=amd-fo7llpvp
**amd-fo7llpvp** — 2026-10-06 sync: citations into merged upstream files were re-pointed by the merge's measured line map. Merged script-reachable accessors were judged realizations on a ratified crossing, not widenings. This is the precedent for the post-merge citation sweep and for the widening-vs-realization reading.

@history_item n=23 marker=amd-hynrrpjt
**amd-hynrrpjt** — 2026-11-13 sync: added `decision/untrusted-bytes-decoded-without-panic` and `decision/presentational-hints-typed-parse` for merge-borne readers of untrusted page content. A take that changes how untrusted input enters every document gets its own Input Validation row (as `DataTransfer` and the drop events will).

@history_item n=24 marker=amd-thmg2275
**amd-thmg2275** — 2026-10-17 sync: added upstream's `aria-labelledby` read, and upstream's unfiltered `aria-label` read was dropped in the merge, leaving one trimmed read. These are the origins of picked rows `decision/aria-label-trimmed-once` and `decision/aria-labelledby-outranks-label`, which a conflict resolution must keep.

@history_item n=25 marker=amd-s5z3p3gn
**amd-s5z3p3gn** — 2026-11-17 sync: added `decision/svg-image-href-fetch`, a merge-borne fetch validated by the stamp and the file-URL refusal. This is the precedent for checking any new fetch or read path a merge brings (the drop source's `url` dependency) against the stamping constructor and the `file:` rule.

@history_item n=26 marker=amd-6oryif7l
**amd-6oryif7l** — 2026-10-24 sync: the upstream root example `multi_doc`'s inputs and env reads were registered (validated in the examples' shape, no boundary widened). This is the precedent for the new root examples `examples/dnd.rs` and `examples/file_drop.rs`.

@history_item n=27 marker=amd-xyjfnhq4
**amd-xyjfnhq4** — A `TMPDIR` read at a second test-only site, brought by an upstream test helper, was registered in Secret Management's env list. The new upstream tests `drag_and_drop.rs` and `flex_br.rs` must be swept for env reads the same way.

@history_item n=28 marker=amd-f36s77cw
**amd-f36s77cw** — dioxus-cli `0.8.0-alpha.3` was installed in the upstream-only, repository-guarded publish workflow. This is Dioxus 0.8 context in CI for the held-Dioxus carry (`cry-twrvkwp6`) and for upstream workflow files staying guarded.

@history_item n=29 marker=amd-p4ayqnwc
**amd-p4ayqnwc** — 2026-11-01 sync: `decision/skip-mounted-event-of-removed-element`. Upstream's test was left out because it reads differently on Dioxus 0.7. This is the precedent for a merged upstream test that cannot hold under the Dioxus hold, relevant to `drag_and_drop.rs` and the `convert_modifiers` adaptation.

@history_item n=30 marker=amd-j2w3bb6f
**amd-j2w3bb6f** — 2026-11-01 sync: `NetHandler::error`'s do-nothing default hid upstream's fix behind escher's `CountedHandler` wrapper. Trap: sweep the implementers of every trait a take extends. This applies to the drop's new `DataTransfer` and drag-event traits in blitz-traits and the harness's wrappers.

@history_item n=31 marker=amd-uogznu3r
**amd-uogznu3r** — 2026-11-01 sync: two test-only loopback TCP binds were registered with "no served port" kept. Precedent: any bind or listener the merged delta adds is registered and classified, under this chunk's "no new port or listener" boundary.

@history_item n=32 marker=amd-xt7jff5x
**amd-xt7jff5x** — 2026-11-01 sync: a custom widget's attributes are replayed at set-up behind the `custom-widget` feature, ratified as an engine change behind the feature. This was the first custom-widget surface brought by upstream; this chunk's conflicts must keep the custom-widget paint, pointer frame and click synthesis.

@history_item n=33 marker=amd-ee4f2yah
**amd-ee4f2yah** — 2026-11-01 sync: the key-press panic was retired by `BaseDocument::deallocate_anonymous_block` clearing children's `layout_parent`. This is the origin of `decision/dropped-anonymous-block-clears-parent`. `f559f3fe2` changes anonymous flex-item construction in `layout/construct.rs`, so the fix must survive the merge.

@history_item n=34 marker=amd-p4m3ywea
**amd-p4m3ywea** — `decision/dropped-anonymous-block-clears-parent` was restated: a key press reaches a re-focused control. This is the current form of the anonymous-block row the `construct.rs` change touches.

@history_item n=35 marker=amd-epqq7ikd
**amd-epqq7ikd** — 2026-11-01 sync: `render_element` culling on the scaled size and the inline box's paint skip, with the frame gap carried. Precedent for a merged blitz-paint `render.rs` change (here `f65baeca5`'s one-axis clip) whose frame effect is measured or carried.

@history_item n=36 marker=amd-kivhkoye
**amd-kivhkoye** — `covered`'s meaning gained the measured one-axis clip limit: a box clipped on one axis stops a hit at its whole padding box. This is the stated limit `f65baeca5` touches. Research finds that the merge moves paint only, so the limit text stands.

@history_item n=37 marker=amd-gk7bouks
**amd-gk7bouks** — `covered`'s limits were restated (the `contain: paint` clause dropped, the no-area sentence added), and the password caret is placed by the bullets (`decision/password-caret-placed-by-bullets`). These are the fixed refusal texts and the caret rule a merged paint or clip change must not disturb.

@history_item n=38 marker=amd-uqyxh2wm
**amd-uqyxh2wm** — `visible_region` reads clipping boxes through transforms (`decision/visible-region-through-transforms`), and `off-screen`'s meaning was restated. Paint now agreeing with `visible_region` on a one-axis clip is the reading P3 measures for `f65baeca5`.

@history_item n=39 marker=amd-75o3vuj6
**amd-75o3vuj6** — Added `decision/non-invertible-transform-no-hit`, an engine change for every document, which is a hit-path rule the merge's conflicts must keep.

@history_item n=40 marker=amd-ncndaraz
**amd-ncndaraz** — Added `decision/custom-widget-click-synthesis`: the UA click is synthesized on a custom-widget target. Upstream's widget arm returned before the synthesis. This is the events-driver code `22c18e38f` (preventDefault) and `82a6573ee` (drag events in blitz-dom's driver) merge into.

@history_item n=41 marker=amd-dtadt6rg
**amd-dtadt6rg** — Added `decision/custom-widget-content-box-frame` (pointer events in the content-box frame) and `decision/custom-widget-document-font-context`. The new drag and drop pointer events must keep the widget pointer frame through the merge.

@history_item n=42 marker=amd-f6nycydd
**amd-f6nycydd** — Added `decision/canvas-content-crosses-as-shot-pixels`: canvas content crosses only as shot pixels on every carrier, with no new field or carrier. Any drop content reaching a widget must not open a new crossing outside it.

@history_item n=43 marker=amd-vo2f5aq7
**amd-vo2f5aq7** — Added `decision/harness-blitz-paint-custom-widget`: blitz-test-harness turns on blitz-paint's custom-widget feature, adding no package to the lock. The merged lock and feature set must keep it.

@history_item n=44 marker=amd-qdj6ykvq
**amd-qdj6ykvq** — The script fetch's `file:` refusal by document, recorded as a narrowing (`document_may_load`). This is one of the "stamped sub-resource request and the `file:` refusal" intents conflicts must keep. It contrasts with the drop reader's unrestricted `std::fs` reads.

@history_item n=45 marker=amd-gwlfkozm
**amd-gwlfkozm** — `decision/file-urls-only-from-file-documents` was widened to every engine sub-resource load, and `decision/one-stamping-request-constructor` (the blitz-dom clippy `disallowed-methods` gate) was added. Merged upstream code in blitz-dom must build requests only through the stamping constructor.

@history_item n=46 marker=amd-odgklv4o
**amd-odgklv4o** — `decision/file-urls-only-from-file-documents` was extended to the browser app's favicon and link navigation. This is the current rule a dropped file's path or the drop source's `url` must be checked against.

@history_item n=47 marker=amd-wo6r6akf
**amd-wo6r6akf** — `decision/file-url-only-for-file-documents` (threat model) was restated with the browser app's favicon and navigation drops. This is the threat-row form of the `file:` rule the drop input sits beside.

@history_item n=48 marker=amd-2j7nmvyg
**amd-2j7nmvyg** — Data Protection: a file input's host path stays in the DOM and form submission while every snapshot carrier reads the mask. This is the standing treatment of host paths, a reference when OS-dropped files (paths, bytes) reach a page.

@history_item n=49 marker=amd-w3ppduc7
**amd-w3ppduc7** — ci.yml carries `permissions: contents: read` (`decision/ci-token-read-only`), every `uses:` is pinned to a 40-hex SHA, and the cargo-deny audit leg was added. CI workflow edits in this merge must keep both.

@history_item n=50 marker=amd-mdzilbyy
**amd-mdzilbyy** — The publish, post-results and WPT jobs are guarded by `github.repository == 'DioxusLabs/blitz'` (`decision/publish-job-repository-guard`, `decision/publish-secrets-upstream-main-only`). Upstream workflow files taken in the merge must stay repository-guarded.

@history_item n=51 marker=amd-gapsbzhr
**amd-gapsbzhr** — `decision/ci-actions-pinned-to-sha`'s guard moved to WorkflowPinTest, which reads every workflow file, so any merged workflow file is pin-checked.

@history_item n=52 marker=amd-jtubqrjg
**amd-jtubqrjg** — The whole-workspace `--all-features` audit with the licences gate (`decision/dependency-audit-cargo-deny`, `decision/licence-gate`); upstream's own licenses job stays reverted. This sets the audit and licence check the merged lock must pass.

@history_item n=53 marker=amd-7dg423lc
**amd-7dg423lc** — `decision/local-audit-before-push`: CI audit can go red on advisories published after a green audit, so run `ci-leg.sh audit` locally before pushing the merge.

@history_item n=54 marker=amd-cx4jd2r7
**amd-cx4jd2r7** — `decision/advisory-ignores-by-id`: ignore reasons name their owner residual row by id, and no product file names the residuals file. This governs any ignore the merged graph would need.

@history_item n=55 marker=amd-266u5epx
**amd-266u5epx** — `decision/skia-default-build-fetch` (sha256-verified archive), `decision/dependency-audit-cargo-deny`'s memmap2 exception, and `decision/no-force-skia-binaries-download`. These are picked dependency rows the merged lock and CI legs must keep.

@history_item n=56 marker=amd-qtipl26p
**amd-qtipl26p** — Added `contract/environment-variables/skia-binaries-url`, a CI-set env read. It is part of the env registry the merged delta's env sweep is compared against.

@history_item n=57 marker=amd-22xprh5v
**amd-22xprh5v** — `decision/locked-builds` extended to the build-directory sweep's cargo runs. The directive's `cargo build --workspace --locked` on the merged tree is this rule.

@history_item n=58 marker=amd-hgy2l7ac
**amd-hgy2l7ac** — Added `contract/environment-variables/ci`, a content-free flag read only by the CI leg runner. It is part of the env registry against which "no new env var of ours" is checked.

@history_item n=59 marker=amd-ty73oviv
**amd-ty73oviv** — The `embolden` leg (`decision/embolden-locked-fast-leg`) is a locked cargo leg last in `FAST_LEGS`. This is the `embolden` witness the scope names among proofs that must still pass.

@history_item n=60 marker=amd-beg5lwf3
**amd-beg5lwf3** — The engine's password paint mask (`decision/password-painted-masked`) and `png` as the shot encoder with no `base64` crate (`decision/png-encodes-screenshots`, `decision/no-base64-crate`). The merged dependency set must not bring a base64 crate, and the paint changes must keep the mask.

@history_item n=61 marker=amd-rcntqdap
**amd-rcntqdap** — Added `decision/open-list-shot-pixels`: a shot of an open select list holds labels as pixels, the shot's fields unmoved. The screenshot readings re-measured after the merge sit on this ratified widening.

@history_item n=62 marker=amd-vv3gy3nf
**amd-vv3gy3nf** — The sink drops records from outside targets whole at every level (`decision/outside-target-dropped`, `decision/other-log-targets-dropped`), with `host_log` as the standing check. Trap: a "nothing in the logs" check must run against a sink-installing host. This governs the "sink allowlist re-checked" sweep of new tracing and log targets in the merged delta.

@history_item n=63 marker=amd-nckna2hd
**amd-nckna2hd** — Engine targets print only the seven safe fields (`decision/engine-target-safe-fields`). The upstream apps' `fmt::init` subscribers and `env_logger` stay unscrubbed. New engine `tracing` call sites from the merge (drag and drop in blitz-shell and blitz-dom) fall under it.

@history_item n=64 marker=amd-k6phd7fr
**amd-k6phd7fr** — The sink's second record class (`decision/closed-span-lines`) was ratified. Standing rule: a count may reach a log but no content. This applies to any span a merged commit adds.

@history_item n=65 marker=amd-txwauiiq
**amd-txwauiiq** — `password` was added to the escher-target content scrub (`decision/escher-target-content-scrub`, twelve names), and `decision/escher-telemetry-allowlist-sink`'s exception was restated. These are the sink rows the allowlist re-check reads against.

@history_item n=66 marker=amd-gztfqjsv
**amd-gztfqjsv** — `decision/driver-installs-no-subscriber` and `decision/driver-no-subscriber-no-env` gained the scoped-capture exception. Both are picked rows whose no-env-read clause the merged delta's env sweep checks.

@history_item n=67 marker=amd-a2mt6lyb
**amd-a2mt6lyb** — The select list holds no log site, span or field, and `SelectListRow.label` is never logged (`decision/select-list-label-never-logged`). It is pinned by `host_log` and `mcp_log`, which must stay at 0 needles after the merge.

@history_item n=68 marker=amd-gxlry3s2
**amd-gxlry3s2** — The overlay scrollbar fade reads the document's clock (`decision/scrollbar-fade-on-document-clock`, `decision/advance-moves-virtual-clock-exactly`, `decision/time-moves-only-on-advance`). This is the clock invariant the merge's conflicts must keep.

@history_item n=69 marker=amd-l3k66kea
**amd-l3k66kea** — `decision/driver-session-unix-socket-only` was restated: the socket exists on unix only, while in-process sessions run on linux, windows and macOS. This is the picked threat-model row the cross-OS CI readings rest on.

@history_item n=70 marker=amd-g325ihes
**amd-g325ihes** — The driver session socket: an owner-only Unix socket with no TCP or UDP port. This is the origin of `decision/session-socket-sole-listener` and `decision/driver-session-unix-socket-only`.

@history_item n=71 marker=amd-l3qkvs2z
**amd-l3qkvs2z** — The socket carries calls in and answers out (`decision/socket-carries-screen-reading`'s origin); the trust boundary is the owning user by file mode. Nothing the merge adds may reach these carriers.

@history_item n=72 marker=amd-qj7unb2e
**amd-qj7unb2e** — The MCP server's stdio crossing was ratified (`decision/mcp-server-stdio-only`, `decision/mcp-stdio-no-auth`, `contract/mcp-server/transport`). These are picked rows the merge must leave unmoved.

@history_item n=73 marker=amd-trieeiqe
**amd-trieeiqe** — The closed-field rule (`decision/boundary-widening-rule`, `decision/closed-value-field-no-widening`): a closed-value field inside a ratified carrier is no widening, while authored text, an open set, a new carrier or a new exit is one. This is the test for classifying each crossing the merge brings.

@history_item n=74 marker=amd-jehnjxny
**amd-jehnjxny** — Ids, names and values leave in an answer (`decision/stable-id-exits`, `decision/accessible-name-exits`). Typed text was measured in a host's log at 0. These are the ratified exits a merged change must not extend.

@history_item n=75 marker=amd-uju462dy
**amd-uju462dy** — The stable id's four tiers and three-way uniqueness argument (`decision/anchored-id-tier`, `contract/stable-element-id/anchored-path`, `no-collision`). Stable ids are one of the intents conflict resolution must keep.

@history_item n=76 marker=amd-z5quoiya
**amd-z5quoiya** — A Dioxus key that is empty or holds `/` is refused in the keyed-list segment, each case with a named witness (`contract/stable-element-id/keyed-list-segment`). This is a stable-id contract whose witnesses must hold after the merge.

@history_item n=77 marker=amd-zmfeomwg
**amd-zmfeomwg** — A stable id is never an index, pointer, hash, clock or process-local value (`contract/stable-element-id/stability`). It is a picked contract the merge's dioxus-native-dom changes must keep.

@history_item n=78 marker=amd-bkpsbxrm
**amd-bkpsbxrm** — Added the reserved virtual-node id form `{owner}///{key}` (`contract/stable-element-id/virtual-node-form`). It is a picked id contract.

@history_item n=79 marker=amd-bsoc45oo
**amd-bsoc45oo** — Refusal detection: stale or not-found, then disabled, off-screen, covered in that order (`decision/target-refusal-order`, `decision/unknown-id-refused-stale-or-not-found`). The hit-path changes the merge brings must leave this order intact.

@history_item n=80 marker=amd-hpwnb7tj
**amd-hpwnb7tj** — The snapshot's `clickable` reading (`contract/snapshot-text/clickable`, `contract/diff-node/clickable`) is a closed bool from current `click` listeners. New drag-event listeners must not change what `clickable` reads.

@history_item n=81 marker=amd-625eavpk
**amd-625eavpk** — Added `contract/diff-node/highlighted` and `contract/snapshot-text/highlighted`, and `decision/cells-grid-keyboard` gained aria-selected. These are picked snapshot contracts.

@history_item n=82 marker=amd-cmzcnw4j
**amd-cmzcnw4j** — `decision/select-list-crossing` and `decision/snapshot-state-readings` gained the highlighted reading. This is the current form of two picked threat-model rows.

@history_item n=83 marker=amd-gmjqtq6k
**amd-gmjqtq6k** — The `layout` verb: `decision/session-layout-read-in-process` widened to every carrier, `contract/driver-verbs/layout` was added, and `decision/socket-carries-screen-reading` names layout readings. The layout reason and its readings are a surface of this take through `layout_reason.rs`'s `CalcHandle` adaptation (T1 A).

@history_item n=84 marker=amd-ncav4r5x
**amd-ncav4r5x** — Added `decision/kept-check-layout-answer`: a kept layout step replays and stops on a changed placed value. `stand_replay` is the witness that Taffy 0.16 leaves layout answers unmoved or names what moved.

@history_item n=85 marker=amd-iqjzkagx
**amd-iqjzkagx** — The text extent reader is read-only, with `None` for an unresolved node (`decision/text-extent-read-only-none-when-unresolved`). It is a layout reader touched by the Taffy move.

@history_item n=86 marker=amd-r7gjmvn5
**amd-r7gjmvn5** — The buffered render (`decision/dioxus-render-into-buffer`) removed the spawned-task `MountedData` panic in dioxus-native-dom. It must survive the merge's dioxus-native-dom edits under the Dioxus hold.

@history_item n=87 marker=amd-ufb4xpzp
**amd-ufb4xpzp** — The Dioxus bridge dispatch (`DioxusEventHandler::handle_event`) records only where a signal registry stands (`decision/dispatch-recorded-only-with-signal-registry`). This is the dioxus-native-dom event path whose `events.rs` import blocks conflict and which the drag event's `convert_modifiers` adaptation joins.

@history_item n=88 marker=amd-p32xt5yc
**amd-p32xt5yc** — The reached record written in `handle_event` with no log site and no `NodeId` (`decision/reached-handler-record-without-log`). New drag and drop dispatches pass through the same handler.

@history_item n=89 marker=amd-kfybkc5a
**amd-kfybkc5a** — `decision/reached-record-deduplicated-survives-removal` was restated (innermost listener alone on a stopped dispatch). It is the current form of a picked row on the event-dispatch path the merge touches.

@history_item n=90 marker=amd-3fkrlw5u
**amd-3fkrlw5u** — The coverage line holds ids, owner ids, event names and counts only (`decision/coverage-line-no-user-data`, `contract/coverage-line/unexercised`). New drag event names would appear here as handler events.

@history_item n=91 marker=amd-fks6ccvb
**amd-fks6ccvb** — `decision/inert-focus-clear-silent` (two silent clears) and `decision/programmatic-focus-events`'s exceptions. `inert` and the focus model are intents the merge's events-driver changes must keep.

@history_item n=92 marker=amd-675kztsi
**amd-675kztsi** — `inert` is read by presence (`decision/inert-subtree`), and the toggle is read per role (`decision/toggle-read-per-role`). These are accessibility state reads the merge must keep.

@history_item n=93 marker=amd-3q26qbsw
**amd-3q26qbsw** — Keyboard activation (`decision/enter-activates-focused-control`, `decision/space-keyup-activates-focused-control`, `decision/button-click-focuses`), ratified as a widening. This is the keyboard model the events-driver merge must keep.

@history_item n=94 marker=amd-fx6xhyot
**amd-fx6xhyot** — The range input row (`decision/range-current-value-apart`, `range-keyboard-actions`, `range-pointer-actions`, `range-value-sanitized`, `range-value-exits`). The range model is an intent conflicts must keep, `pointer_leave.rs`'s area included.

@history_item n=95 marker=amd-vfmtreeb
**amd-vfmtreeb** — The single-choice `select` row (`decision/single-select-selectedness`, `decision/select-typeahead-private`, `decision/select-choice-exits`). The select model is an intent conflicts must keep.

@history_item n=96 marker=amd-gs5hhw4q
**amd-gs5hhw4q** — `multiple` select keys, clicks and chosen set (`decision/multiple-select-keys-and-clicks`, `decision/multiple-select-set-exits`). These are picked select rows.

@history_item n=97 marker=amd-xg3cd3vy
**amd-xg3cd3vy** — `optgroup` and its label (`decision/optgroup-options-and-label`, `decision/option-disabled-one-rule`). These are picked select rows.

@history_item n=98 marker=amd-4lvaf4op
**amd-4lvaf4op** — The open list's state, keys, pointer and close (`decision/select-list-live-reader`, `select-list-tab-chooses-and-closes`, `select-list-pointer-release-chooses`, `select-list-closing`). Pointer and drag changes in the events driver must keep the list's pointer rules (a press starts no text drag).

@history_item n=99 marker=amd-kp7dm775
**amd-kp7dm775** — The open list's visible region, paint and scroll (`decision/visible-region-open-list-row`, `select-list-painted-over-page`, `select-list-engine-colours`, `select-list-scroll-own-offset`, `select-list-wheel`). A paint change in `render.rs` (`f65baeca5`) must keep the list's paint order.

@history_item n=100 marker=amd-25p7cqlx
**amd-25p7cqlx** — Which select opens a list, what a row takes, and hostile markup kept finite (`decision/select-list-open-gate`, `select-list-row-choice`, `select-list-hostile-markup-finite`, `select-list-twenty-rows`). These are picked select-list rows.

@history_item n=101 marker=amd-a4bioxzi
**amd-a4bioxzi** — `size` on a `multiple` select is capped at 65534 (`decision/multiple-select-size-rows`, `decision/select-size-hint-cap`). These are picked rows.

@history_item n=102 marker=amd-hcikfp5z
**amd-hcikfp5z** — An author's `open` and `aria-expanded` on a select steer nothing (`decision/select-expanded-from-engine-state`). This is a picked row that keeps the accessibility state reads.

@history_item n=103 marker=amd-jm2ysvlc
**amd-jm2ysvlc** — The select list's readers and close add no panic (`decision/select-row-layout-none-without-styles`, `decision/select-list-closes-when-select-unfit`). These are picked Error Handling rows.

@history_item n=104 marker=amd-txxcahkq
**amd-txxcahkq** — The select readers and the list's public surface (`decision/select-readers-resolve-node-closed-select-none`, `decision/select-list-label-never-logged`). These are picked rows.

@history_item n=105 marker=amd-kfo2mltt
**amd-kfo2mltt** — The driver's `type` on any select (`decision/driver-type-on-select`), with focus via `focus_select` firing no DOM focus events. It is one of the silent focus paths named by `decision/programmatic-focus-events`.

@history_item n=106 marker=amd-yhsrsxmz
**amd-yhsrsxmz** — The single select sized as a list box restated `decision/single-select-selectedness`, `select-size-hint-cap`, `multiple-select-size-rows` and `select-list-open-gate`. This is the current form of picked select rows.

@history_item n=107 marker=amd-u7vd4egj
**amd-u7vd4egj** — `decision/mask-holds-on-every-carrier` gained the guard `stand_masks_on_new_answers`. The value mask is re-proven after the merge by this stand check.

@history_item n=108 marker=amd-qvkjxz5k
**amd-qvkjxz5k** — The password value is masked in the snapshot (`contract/snapshot-value/masked-value`, `decision/masked-input-values`). Trap recorded then: the first wire form re-reads the file input's host path, which bears on file paths reaching pages through a drop.
