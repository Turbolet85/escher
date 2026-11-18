@leaf_group group=architecture+security-plan+test-plan-2
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=63 class=omission
leaf-text: it declares no `mod session_common;`).
source: dec-b7dcgjx4 «decision/typed-password-never-returned · Typed passwords never returned — A password typed through the driver and a file input's path occur nowhere in what a call returns, graph, actions and layout answers included»
replace-with: it declares no `mod session_common;`) and, since 2026-11-17-stand-requirement-sweep, `stand_masks_on_new_answers` (a password typed through the driver and a file input's path occurring nowhere in what a call returns, graph, actions and layout answers included, nor in a kept or an explored path's check, in both layout modes, the value reading the mask).
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=61 class=omission
leaf-text: and `WITNESSES`, 16 rows naming each requirement's witness tests
source: dec-k35v6qhg «decision/requirement-witness-inside-stand-repeat · Every requirement witnessed inside stand-repeat — Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red.»
replace-with: and `WITNESSES`, 16 rows naming each requirement's witness tests inside the stand-repeat leg, each pinned by its test's function name, so a renamed witness reads red (a requirement with no screen naming none)
