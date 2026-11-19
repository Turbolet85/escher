@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/stack.md line=15 class=stale
leaf-text: Servo Stylo 0.22.0
source: dec-qy3gqtf5 «decision/stylo-css-engine · Servo Stylo as the CSS engine — Servo Stylo 0.24, with selectors and cssparser, does styling and selector matching.»
replace-with: Servo Stylo 0.24
@leaf_edit n=2 leaf=.claude/docs/stack.md line=18 class=stale
leaf-text: Parley 0.12 (registry)
source: dec-qvdsgthp «decision/parley-text-layout · Parley for text and fonts — Parley 0.13 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.»
replace-with: Parley 0.13 (registry)
@leaf_edit n=3 leaf=.claude/docs/stack.md line=21 class=stale
leaf-text: winit `=0.31.0-beta.3`
source: dec-odccjf3r «decision/winit-exact-beta-pin · winit pinned exactly to a beta — winit is pinned exactly to a beta version, 0.31.0-beta.4.»
replace-with: winit `=0.31.0-beta.4`
@leaf_edit n=4 leaf=.claude/docs/stack.md line=23 class=stale
leaf-text: reqwest 0.13 (native-tls)
source: dec-qrst54im «decision/blitz-net-reqwest-fetching · reqwest-based resource fetching in blitz-net — blitz-net fetches HTTP, filesystem and data-URI resources, HTTP through reqwest on rustls, with native-tls vendored on Android.»
replace-with: reqwest 0.13 (rustls; native-tls vendored on Android)
@leaf_edit n=5 leaf=.claude/docs/workflow.md line=15 class=retired
leaf-text: its take's commits taken by no-commit cherry-picks with the merge base unmoved and each commit left out carrying its measured reason
source: dec-baoqp7za «decision/upstream-sync-by-two-parent-merge · Upstream synced by a two-parent merge — An upstream sync merges DioxusLabs/blitz main with two parents, and every upstream commit left out is named with its reason.»
replace-with: an upstream sync merging DioxusLabs/blitz main with two parents, every upstream commit left out named with its reason
@leaf_edit n=6 leaf=.claude/docs/workflow.md line=15 class=retired
leaf-text: git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0" and Dioxus "0.7.3".
source: dec-ikcsb5y4 «decision/dioxus-0-8-move-held · Dioxus 0.8 move held — Dioxus is held at 0.7.3, and the Dioxus 0.8 move stays out of the upstream take.»
replace-with: Dioxus is held at 0.7.3, the Dioxus 0.8 move kept out of the upstream take.
