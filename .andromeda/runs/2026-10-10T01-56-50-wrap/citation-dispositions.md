sweep: base: b03a5fc7 — the previous sweep (2026-10-09T20-50-14-wrap) · re-pointed 780 · changed 52 · stretched 73
stretched .andromeda/architecture.md:48 Cargo.toml:105-115 → 105-115 — changed inside → holds
changed .andromeda/architecture.md:48 packages/blitz-dom/src/resolve.rs:444-445 → ? — both ends rewritten «taffy::comp…» → holds — re-pointed by hand to 455-459 — the two taffy calls, now over a `LayoutPassState`
changed .andromeda/architecture.md:49 Cargo.toml:116-119 → ? — the start rewritten «parley = { git = "https://github.c…» → claim false — raised at Validate: line 116 reads `parley = { version = "0.12", … }`, a registry version; skrifa stands at 117-119
stretched .andromeda/architecture.md:49 packages/blitz-dom/src/node/text.rs:24-32 → 24-36 — changed inside → holds
stretched .andromeda/architecture.md:53 packages/accesskit_xplat/Cargo.toml:16-35 → 16-35 — changed inside → claim false — raised at Validate: lines 27 and 30 read accesskit_unix 0.24.0 and accesskit_android 0.9.0
changed .andromeda/architecture.md:62 apps/readme/Cargo.toml:60-63 → ? — the start rewritten «comrak = { version = "0.…» → claim false — raised at Validate: line 60 reads comrak "0.56"; the range stands at 60-63
stretched .andromeda/architecture.md:64 tests/blitz-tests/Cargo.toml:14-25 → 14-25 — changed inside → holds
stretched .andromeda/architecture.md:66@c36 .github/workflows/ci.yml:34-401 → 34-400 — changed inside → holds
changed .andromeda/architecture.md:73 Cargo.toml:106 → ? — rewritten «taffy = { git = "https://github.com/DioxusLabs/t…» → holds — re-pointed by hand to 106 (the line read; it is the number as it stood, so no digit moved) — taffy is still a git dependency pinned by `rev`
changed .andromeda/architecture.md:73 Cargo.toml:116 → ? — rewritten «parley = { git = "https://github.com/linebender/…» → claim false — raised at Validate: parley is a registry version, no git `rev`
stretched .andromeda/architecture.md:80 packages/blitz-dom/Cargo.toml:14-21 → 14-22 — changed inside → claim false — raised at Validate: the default list gains `text-transform-icu` (line 21)
changed .andromeda/architecture.md:80 packages/blitz/Cargo.toml:13-14 → ? — the end rewritten «[features]» → claim false — raised at Validate: the default list gains `text-transform-icu`; the range stands at 13-14
changed .andromeda/architecture.md:80 packages/dioxus-native/Cargo.toml:13 → ? — rewritten «default = ["accessibility"…» → claim false — raised at Validate: the default list gains `text-transform-icu`; the line stands at 13
changed .andromeda/architecture.md:80 packages/dioxus-native-dom/Cargo.toml:13 → ? — rewritten «default = ["accessibil…» → claim false — raised at Validate: the default list gains `text-transform-icu`; the line stands at 13
stretched .andromeda/architecture.md:82 packages/blitz-dom/src/document.rs:408-416 → 408-418 — changed inside → claim false — raised at Validate: the enumerated prefs lack `layout.writing-mode.enabled`, set under the `writing-mode` feature (416-417); the range as written holds
stretched .andromeda/architecture.md:85 packages/stylo_taffy/src/convert.rs:268-274 → 270-283 — changed inside → holds — the doc comment (270-272) and the function now stand either side of the new `resolve_calc_value`
changed .andromeda/architecture.md:86 packages/blitz-dom/src/layout/inline.rs:437-439 → ? — both ends rewritten → claim false — raised at Validate: the comment that stated it left the file; the flags are read from the style and handed to Parley at 473-476
changed .andromeda/architecture.md:89@c130 packages/blitz-dom/src/layout/table.rs:739-758 → ? — the start rewritten → holds — re-pointed by hand to 770-789
stretched .andromeda/architecture.md:93 packages/blitz-net/src/lib.rs:153-164 → 153-173 — changed inside → holds
stretched .andromeda/architecture.md:100@c1042 wpt/runner/src/test_runners/mod.rs:240-336 → 242-337 — changed inside → holds
stretched .andromeda/architecture.md:100@c1331 wpt/runner/src/test_runners/mod.rs:365-375 → 366-373 — changed inside → holds — the parse site; the extension list moved, unchanged, to wpt/runner/src/test_variants.rs:23-27
stretched .andromeda/architecture.md:100@c2278 wpt/runner/src/test_runners/ref_test.rs:114-139 → 115-143 — changed inside → holds
stretched .andromeda/architecture.md:100@c2357 wpt/runner/src/test_runners/ref_test.rs:123-125 → 124-127 — changed inside → holds
stretched .andromeda/architecture.md:100@c2479 wpt/runner/src/test_runners/ref_test.rs:179-211 → 184-217 — changed inside → holds
changed .andromeda/architecture.md:100@c3038 wpt/runner/src/main.rs:677-686 → ? — both ends deleted «// JS-file tests …» → holds — re-pointed by hand to 687-688 — the result is named `test.url`; the wrapper name itself is built at wpt/runner/src/test_variants.rs:29-38
stretched .andromeda/architecture.md:106 Cargo.toml:65-184 → 65-184 — changed inside → holds
stretched .andromeda/architecture.md:107 examples/wasm_hello/Cargo.toml:1-5 → 1-6 — changed inside → claim false — raised at Validate: line 5 reads `license.workspace = true`
stretched .andromeda/architecture.md:110 packages/blitz-vibey-script/src/lib.rs:32-42 → 32-43 — changed inside → holds
stretched .andromeda/architecture.md:110 packages/blitz-vibey-script/src/runtime.rs:1386-1402 → 1386-1408 — changed inside → holds
stretched .andromeda/architecture.md:110 packages/stylo_taffy/src/convert.rs:3-62 → 3-64 — changed inside → holds
changed .andromeda/architecture.md:113 examples/screenshot.rs:41-51 → ? — the start rewritten «let file_content = std:…» → holds — re-pointed by hand to 41-51 (the line read; it is the number as it stood, so no digit moved)
changed .andromeda/architecture.md:115@c3715 wpt/runner/src/main.rs:608-612 → ? — the start rewritten «let relative_pa…» → holds — re-pointed by hand to 618-622
stretched .andromeda/architecture.md:115@c3859 .github/scripts/test_wpt_diff_to_pr.py:51-88 → 104-174 — changed inside → holds
stretched .andromeda/architecture.md:121@c3284 packages/blitz-dom/src/lib.rs:74-124 → 74-125 — changed inside → holds
stretched .andromeda/architecture.md:122@c1525 packages/blitz-dom/src/resolved_style.rs:334-341 → 336-343 — changed inside → holds
stretched .andromeda/architecture.md:124@c2942 packages/blitz-dom/src/scrolling.rs:741-805 → 741-807 — changed inside → holds
changed .andromeda/architecture.md:125 packages/blitz-dom/src/layout/mod.rs:467-742 → ? — the start rewritten → claim false — raised at Validate: the ten trait impls are on `LayoutPassState<'_>` (608-866) and `PrintTree` on `TaffyDebugTree` (911-941), none on `BaseDocument`
changed .andromeda/architecture.md:125 packages/blitz-dom/src/layout/mod.rs:498-508 → ? — the start rewritten → holds — re-pointed by hand to 642-664 — written with the amendment of this sentence
changed .andromeda/architecture.md:125 packages/blitz-dom/src/layout/table.rs:739-820 → ? — the start rewritten → holds — re-pointed by hand to 770-851
stretched .andromeda/architecture.md:125 packages/stylo_taffy/src/lib.rs:6-13 → 6-19 — changed inside → claim false — raised at Validate: the enumerated exports lack `to_taffy_style_in`, the `writing_mode` module, `WritingMode` and `WritingModeExt` (12-17); the range as written holds
stretched .andromeda/architecture.md:125 packages/stylo_taffy/src/wrapper.rs:26-33 → 35-56 — changed inside → holds
stretched .andromeda/architecture.md:125 packages/stylo_taffy/src/convert.rs:898-899 → 937-950 — changed inside → holds — the doc line (937) and `to_taffy_style` (950) stand either side of the new `to_taffy_style_in`
stretched .andromeda/architecture.md:125 packages/stylo_taffy/src/wrapper.rs:16-24 → 19-33 — changed inside → holds
stretched .andromeda/architecture.md:136@c11894 packages/blitz-dom/src/document.rs:2249-2265 → 2273-2290 — changed inside → claim false — raised at Validate: the reader calls `physical_unrounded_geometry`, not `unrounded_absolute_position(0, 0)`; the range as written holds `get_client_bounding_rect`
changed .andromeda/architecture.md:136@c11940 packages/blitz-dom/src/node/node.rs:1583-1590 → ? — both ends deleted → claim false — raised at Validate: `Node::unrounded_absolute_position` is removed; its subject is gone
stretched .andromeda/architecture.md:138@c1010 examples/screenshot.rs:34-54 → 34-54 — changed inside → holds
changed .andromeda/architecture.md:138@c1613 wpt/runner/src/main.rs:240-250 → ? — the start rewritten «let mut suites:…» → holds — re-pointed by hand to 239-250 — `collect_tests` takes the suites and reads `full` there; the non-dash filter that makes them now stands in `main`, at 476-479, and that citation is added beside this one (the one hand edit of this sweep that writes more than digits: the claim's two halves no longer share a range)
changed .andromeda/architecture.md:138@c1733 wpt/runner/src/main.rs:473-479 → ? — the start rewritten «// `--list` pri…» → holds — re-pointed by hand to 483-489
changed .andromeda/architecture.md:140@c596 wpt/runner/src/test_runners/ref_test.rs:33-35 → ? — the end rewritten → holds — re-pointed by hand to 34-36 — `{test}` is `artifact_name(test)`, the path, or `{path}-variant-{hash}` for a URL with a query or fragment
changed .andromeda/architecture.md:140@c643 wpt/runner/src/test_runners/ref_test.rs:163-170 → ? — the end rewritten → holds — re-pointed by hand to 167-175
changed .andromeda/architecture.md:140@c692 wpt/runner/src/test_runners/ref_test.rs:202-204 → ? — both ends rewritten → holds — re-pointed by hand to 207-210
stretched .andromeda/architecture.md:141@c543 .github/scripts/wpt_diff_to_pr.py:186-191 → 274-280 — changed inside → claim false — raised at Validate: the enumerated CLI lacks `--areas` (line 279); the range as written holds
stretched .andromeda/architecture.md:141@c638 .github/scripts/wpt_diff_to_pr.py:17-18 → 17-19 — changed inside → holds
stretched .andromeda/architecture.md:152 packages/blitz-shell/src/window.rs:651-685 → 651-685 — changed inside → holds
stretched .andromeda/architecture.md:153 .github/scripts/wpt_diff_to_pr.py:186-191 → 274-280 — changed inside → holds
changed .andromeda/architecture.md:154@c2375 packages/blitz/Cargo.toml:14-18 → ? — the start rewritten «default = ["ne…» → claim false — raised at Validate: the enumerated blitz features lack `text-transform-icu` (17) and `writing-mode` (20); the block stands at 14-20
stretched .andromeda/architecture.md:159 deny.toml:1-21 → 1-21 — changed inside → holds — `[graph]` and `[advisories]` are unchanged inside the range; the `[licenses]` table stands after it (22-42)
stretched .andromeda/architecture.md:180@c384 .github/workflows/ci.yml:34-401 → 34-400 — changed inside → holds
stretched .andromeda/architecture.md:180@c3889 .github/workflows/wpt.yml:54-106 → 46-100 — changed inside → holds
changed .andromeda/architecture.md:181@c1454 packages/blitz-dom/src/layout/mod.rs:576-605 → ? — the start rewritten → holds — re-pointed by hand to 732-761
changed .andromeda/architecture.md:181@c1548 packages/blitz-dom/src/layout/mod.rs:87-102 → ? — the start rewritten → holds — re-pointed by hand to 166-198 — the wrapper method and the out-of-flow call it reaches (193-196)
changed .andromeda/architecture.md:181@c1857 packages/blitz-dom/src/layout/mod.rs:204 → ? — rewritten «&mut self.font_…» → holds — re-pointed by hand to 321
stretched .andromeda/architecture.md:186 wpt/runner/src/main.rs:529-533 → 539-543 — changed inside → holds
stretched .andromeda/architecture.md:186 wpt/runner/src/main.rs:616-634 → 626-644 — changed inside → holds
stretched .andromeda/architecture.md:192 packages/dioxus-native/Cargo.toml:16-31 → 16-33 — changed inside → holds
stretched .andromeda/architecture.md:215 packages/dioxus-native/Cargo.toml:16-31 → 16-33 — changed inside → holds
changed .andromeda/architecture.md:219 Cargo.toml:106 → ? — rewritten «taffy = { git = "https://github.com/DioxusLabs/…» → holds — re-pointed by hand to 106 (the line read; it is the number as it stood, so no digit moved)
changed .andromeda/architecture.md:219 Cargo.toml:116 → ? — rewritten «parley = { git = "https://github.com/linebender…» → claim false — raised at Validate: parley is a registry version, no git rev
stretched .andromeda/architecture.md:226 packages/dioxus-native/Cargo.toml:16-31 → 16-33 — changed inside → holds
stretched .andromeda/architecture.md:236 packages/blitz-dom/src/resolved_style.rs:159-341 → 161-343 — changed inside → holds
stretched .andromeda/architecture.md:238 packages/blitz-dom/src/document.rs:2248-2357 → 2272-2389 — changed inside → holds
stretched .andromeda/architecture.md:239 packages/blitz-dom/src/debug.rs:6-153 → 7-155 — changed inside → holds
stretched .andromeda/architecture.md:241 packages/blitz-dom/src/scrolling.rs:659-805 → 659-807 — changed inside → holds
stretched .andromeda/architecture.md:244 packages/blitz-dom/src/layout/construct.rs:408-653 → 409-654 — changed inside → holds
stretched .andromeda/architecture.md:244 packages/blitz-dom/src/layout/construct.rs:1035-1108 → 1036-1130 — changed inside → holds
changed .andromeda/architecture.md:244 packages/blitz-dom/src/layout/inline.rs:82-193 → ? — the start rewritten → holds — re-pointed by hand to 84-195
stretched .andromeda/architecture.md:244 packages/blitz-dom/src/layout/mod.rs:104-464 → 219-605 — changed inside → holds
stretched .andromeda/architecture.md:249 packages/blitz-vibey-script/src/lib.rs:32-39 → 32-40 — changed inside → claim false — raised at Validate: the enumerated modules lack `inner_text` (line 37); the range as written holds
stretched .andromeda/architecture.md:249 packages/blitz-vibey-script/src/dom/mod.rs:9-15 → 9-16 — changed inside → claim false — raised at Validate: the enumerated dom submodules lack `selection` (line 14); the range as written holds
stretched .andromeda/security-plan.md:9 packages/blitz-net/src/lib.rs:153-164 → 153-173 — changed inside → holds
stretched .andromeda/security-plan.md:15 examples/screenshot.rs:27-54 → 27-54 — changed inside → holds
stretched .andromeda/security-plan.md:84 packages/blitz-net/src/lib.rs:159-161 → 159-170 — changed inside → claim false — raised at Validate: off wasm the path is `request.url.to_file_path()`, an `InvalidInput` error when the URL is no file path (162-165); `request.url.path()` is the wasm branch alone (167-168)
changed .andromeda/security-plan.md:217 .github/workflows/wpt-post-results.yml:29-49 → ? — the end rewritten → holds — re-pointed by hand to 29-49 (the line read; it is the number as it stood, so no digit moved)
stretched .andromeda/security-plan.md:226 deny.toml:1-21 → 1-21 — changed inside → holds — `[graph]` and `[advisories]` are unchanged inside the range; the `[licenses]` table stands after it (22-42)
changed .andromeda/security-plan.md:229 Cargo.toml:106 → ? — rewritten «taffy = { git = "https://github.com/DioxusLabs…» → holds — re-pointed by hand to 106 (the line read; it is the number as it stood, so no digit moved)
changed .andromeda/security-plan.md:229 Cargo.toml:116 → ? — rewritten «parley = { git = "https://github.com/linebende…» → claim false — raised at Validate: parley is a registry version, no git rev
stretched .andromeda/security-plan.md:231 apps/browser/Cargo.toml:45-74 → 46-75 — changed inside → holds
stretched .andromeda/security-plan.md:231 packages/blitz-dom/Cargo.toml:42-95 → 48-108 — changed inside → holds
stretched .andromeda/security-plan.md:231 tests/blitz-tests/Cargo.toml:15-40 → 15-40 — changed inside → holds
changed .andromeda/security-plan.md:234 .github/workflows/publish-browser.yml:148 → ? — rewritten «- uses: awalsh128/c…» → claim false — raised at Validate: the action is gone from the file; line 148 names a step that runs a bare `apt-get` install (150)
changed .andromeda/security-plan.md:234 .github/workflows/wpt.yml:48 → ? — rewritten «uses: awalsh128/cache-apt-pkgs-a…» → claim false — raised at Validate: the action is gone from the file; the install is a bare `apt-get` line (43)
stretched .andromeda/security-plan.md:345 wpt/runner/src/main.rs:616-634 → 626-644 — changed inside → holds
changed .andromeda/security-plan.md:357 examples/screenshot.rs:41-42 → ? — the start rewritten «let file_content = std…» → holds — re-pointed by hand to 41-42 (the line read; it is the number as it stood, so no digit moved)
changed .andromeda/design-system.md:25 packages/blitz-dom/assets/default.css:42-45 → ? — the start rewritten → holds — re-pointed by hand to 43-46 — the rule now selects `a[href]`; the wording goes to Validate with the plan's expected amendment
changed .andromeda/design-system.md:72 packages/blitz-dom/assets/default.css:42-45 → ? — the start rewritten → holds — re-pointed by hand to 43-46
changed .andromeda/design-system.md:133 tests/blitz-tests/Cargo.toml:17 → ? — rewritten «blitz-dom = { workspace = tru…» → holds — re-pointed by hand to 17 (the line read; it is the number as it stood, so no digit moved)
stretched .andromeda/layout-templates.md:45 packages/blitz-dom/src/scrolling.rs:659-805 → 659-807 — changed inside → holds
stretched .andromeda/test-plan.md:28 wpt/runner/src/test_runners/mod.rs:240-327 → 242-328 — changed inside → holds
stretched .andromeda/test-plan.md:56 wpt/runner/src/test_runners/ref_test.rs:25-102 → 26-103 — changed inside → holds
stretched .andromeda/test-plan.md:56 wpt/runner/src/test_runners/ref_test.rs:179-211 → 184-217 — changed inside → holds
changed .andromeda/test-plan.md:57 .github/workflows/wpt.yml:76-86 → ? — the end rewritten «- name: Fetch main-branch …» → holds — re-pointed by hand to 68-79
changed .andromeda/test-plan.md:57 .github/scripts/wpt_diff_to_pr.py:116-161 → ? — the start rewritten «def render(dif…» → holds — re-pointed by hand to 192-249
changed .andromeda/test-plan.md:75 .github/workflows/wpt.yml:70-81 → ? — the end rewritten «- name: Install wpt cli» → holds — re-pointed by hand to 62-73
changed .andromeda/test-plan.md:76 wpt/runner/src/main.rs:240-250 → ? — the start rewritten «let mut suites: Vec<_> = …» → holds — re-pointed by hand to 239-250
stretched .andromeda/test-plan.md:76 wpt/runner/src/main.rs:461-479 → 467-489 — changed inside → holds
stretched .andromeda/test-plan.md:97 packages/blitz-vibey-script/Cargo.toml:42-48 → 44-50 — changed inside → holds
stretched .andromeda/test-plan.md:143 .github/scripts/test_wpt_diff_to_pr.py:51-88 → 104-174 — changed inside → holds — the three named coverages stand; the two tests upstream added inside the range ride the count amendment
changed .andromeda/test-plan.md:194 .github/workflows/wpt.yml:44-51 → ? — the end rewritten «# fonts-tlwg-loma-otf pro…» → holds — re-pointed by hand to 39-43
changed .andromeda/test-plan.md:194 .github/workflows/wpt-post-results.yml:43-49 → ? — the end rewritten «- name: Post…» → holds — re-pointed by hand to 43-49 (the line read; it is the number as it stood, so no digit moved)
changed .andromeda/test-plan.md:219 wpt/runner/src/main.rs:463-470 → ? — the end rewritten «let wpt_dir = path::absolu…» → holds — re-pointed by hand to 469-480
stretched .andromeda/test-plan.md:317 .github/workflows/wpt.yml:82-94 → 75-87 — changed inside → holds — the upload list gains a third file at line 88, past the range
changed .andromeda/test-plan.md:320 packages/blitz/Cargo.toml:14 → ? — rewritten «default = ["net", "accessibility", "…» → holds — re-pointed by hand to 14 (the line read; it is the number as it stood, so no digit moved) — `tracing` is still in the default list
stretched .andromeda/obs-plan.md:12 packages/blitz-dom/src/debug.rs:6-153 → 7-155 — changed inside → holds
stretched .andromeda/obs-plan.md:16 packages/blitz-shell/src/window.rs:667-685 → 667-685 — changed inside → holds
changed .andromeda/obs-plan.md:41 packages/blitz/Cargo.toml:14 → ? — rewritten «default = ["net", "accessibility", "tr…» → holds — re-pointed by hand to 14 (the line read; it is the number as it stood, so no digit moved) — `tracing` is still in the default list
stretched .andromeda/a11y-plan.md:7 packages/accesskit_xplat/Cargo.toml:2-6 → 2-6 — changed inside → holds
stretched .andromeda/a11y-plan.md:8 packages/blitz-dom/Cargo.toml:14-22 → 14-23 — changed inside → holds
changed .andromeda/a11y-plan.md:12 packages/blitz/Cargo.toml:14 → ? — rewritten «default = ["net", "accessibility", "t…» → holds — re-pointed by hand to 14 (the line read; it is the number as it stood, so no digit moved) — `accessibility` is still in the default list
changed .andromeda/a11y-plan.md:14@c244 packages/dioxus-native-dom/Cargo.toml:13 → ? — rewritten «default = ["accessib…» → holds — re-pointed by hand to 13 (the line read; it is the number as it stood, so no digit moved) — `accessibility` is still in the default list
changed .andromeda/a11y-plan.md:14@c370 packages/dioxus-native/Cargo.toml:13 → ? — rewritten «default = ["accessibilit…» → holds — re-pointed by hand to 13 (the line read; it is the number as it stood, so no digit moved) — `accessibility` is still in the default list
changed .andromeda/a11y-plan.md:16 tests/blitz-tests/Cargo.toml:17 → ? — rewritten «blitz-dom = { workspace = true, fe…» → holds — re-pointed by hand to 17 (the line read; it is the number as it stood, so no digit moved)
stretched .andromeda/a11y-plan.md:147 packages/blitz-dom/src/mutator.rs:1008-1017 → 1021-1033 — changed inside → claim false — raised at Validate: the attribute enables autofocus when present with any value but "false" (1024-1028), not only `autofocus="true"`; the range as written holds the handling
stretched .andromeda/a11y-plan.md:172 packages/blitz-vibey-script/src/dom/element.rs:473-491 → 475-492 — changed inside → claim false — raised at Validate: the reflection still writes "true", but the comment giving the reason left the file and blitz-dom no longer expects that value; the range as written holds the getter and setter
stretched escher-0.1.0/working-route.md:73@c5882 packages/blitz-dom/src/document.rs:2249-2265 → 2273-2290 — changed inside · listed, not written: route → route — P5
changed escher-0.1.0/working-route.md:73@c5931 packages/blitz-dom/src/node/node.rs:1583-1590 → ? — both ends deleted → route — P5
moved escher-0.1.0/working-route.md:73@c6903 packages/blitz-dom/src/node/node.rs:1330-1363 → 1385-1418 — listed, not written: route → route — P5
moved escher-0.1.0/working-route.md:79 construct.rs:451 → 452 — listed, not written: route → route — P5
moved escher-0.1.0/working-route.md:79 layout/mod.rs:241 → 358 — listed, not written: route → route — P5
moved escher-0.1.0/working-route.md:86@c1950 construct.rs:441-459 → 442-460 — listed, not written: route → route — P5
