@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=9 class=omission
leaf-text: blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files` (test code only, not secret).
source: ctr-6cbfwi7h «Holds no secret, a local file:// path template; set by the CI leg runner only in the skia-building legs and before the fast leg's sweep, overwriting any value a caller set, and by the CI matrix step»
replace-with: blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files` (test code only, not secret). The CI scripts read `CI` (a content-free flag the CI leg runner's fast arm tests only for empty or unset) and `SKIA_BINARIES_URL` (no secret, a local `file://` path template the CI leg runner and the CI matrix step set, read by skia's build script).
@leaf_edit n=2 leaf=.claude/rules/security.md line=15 class=omission
leaf-text: same-origin/CORS checks are absent
source: dec-hbmegunx «A page script's fetch rejects with TypeError any method other than GET or HEAD.»
replace-with: a page script's `fetch()` rejects with `TypeError` any method other than GET or HEAD; same-origin/CORS checks are absent
@leaf_edit n=3 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: a failed, empty or non-CSS stylesheet blocks no render
source: dec-iigg6kvs «In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render.»
replace-with: a failed, empty, non-CSS or non-UTF-8 stylesheet blocks no render
@leaf_edit n=4 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: holds its choice as its options' `CHECKED` state;
source: dec-hsllrojk «a select without multiple holds its choice as its options' checked state and settles every recorded select by HTML's selectedness setting algorithm: the last chosen, else the first option not disabled.»
replace-with: holds its choice as its options' `CHECKED` state, every recorded select settled by HTML's selectedness setting algorithm (the last chosen, else the first option not disabled);
@leaf_edit n=5 leaf=.claude/rules/security.md line=28 class=omission
leaf-text: builds pass `--locked` — keep both.
source: dec-jlrj566g «The pins stay: Stylo 0.24, Taffy the git pin of upstream DioxusLabs/taffy at commit a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.4.»
replace-with: builds pass `--locked` — keep both. The engine pins stay: Stylo 0.24, Taffy at a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.4.
