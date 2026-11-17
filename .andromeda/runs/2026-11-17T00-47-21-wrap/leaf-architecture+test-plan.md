@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=53 class=omission
leaf-text: the docs gate over every workspace library crate (green; CI's docs job runs the same leg)
source: dec-pe4jrwxz «decision/doc-leg-gates-public-doc-edits · Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block.»
replace-with: the docs gate over every workspace library crate (green; CI's docs job runs the same leg; `fast` runs no rustdoc, so a chunk that edits a public doc comment lists this leg in its local gate block)
@leaf_edit n=2 leaf=.claude/docs/services/blitz-test-harness.md line=21 class=omission
leaf-text: an offline harness (every stand boot but `boot_load`'s) and a document passed to `wrap` carry none
source: dec-6mytiutr «decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot,»
replace-with: an offline harness (every stand boot but `boot_load`'s and the error-text check's failing page, whose `FailingNet` stand-in fails its fetch) and a document passed to `wrap` carry none
