@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=28 class=stale
leaf-text: upstream DioxusLabs/taffy pinned at a full commit rev (e90751fd)
source: dec-jlrj566g «decision/engine-dependency-pins · Engine dependency pins — The pins stay: Stylo 0.22.0, Taffy the git pin of upstream DioxusLabs/taffy at commit a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.3.»
replace-with: upstream DioxusLabs/taffy pinned at a full commit rev (a3997d55)
