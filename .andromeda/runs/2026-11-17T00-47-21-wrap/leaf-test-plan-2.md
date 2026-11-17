@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file ou
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=30 class=omission
leaf-text: offline but for the load page, whose `net_provider` is the stand's `LoadNet`, every other field kept
source: dec-6mytiutr «except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.»
replace-with: offline but for the load page, whose `net_provider` is the stand's `LoadNet`, and the error-text check's failing page, whose `FailingNet` stand-in fails its fetch, every other field kept
@leaf_edit n=2 leaf=.claude/rules/testing.md line=18 class=omission
leaf-text: `is_noop` not overridden;
source: dec-6evisrby «every other stand boot having no network provider but the error-text check's failing page, whose stand-in fails its fetch.»
replace-with: `is_noop` not overridden; the error-text check's failing page's `FailingNet`, which fails its fetch;
@leaf_edit n=3 leaf=.claude/rules/testing.md line=44 class=omission
leaf-text: (fmt → clippy → test → CI scripts → embolden, stopping at the first red);
source: dec-pe4jrwxz «Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block.»
replace-with: (fmt → clippy → test → CI scripts → embolden, stopping at the first red; it runs no rustdoc, so a chunk that edits a public doc comment lists the doc leg in its local gate block too);
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=36 class=omission
leaf-text: its options' `net_provider` replaced by the stand's `LoadNet`, every other field kept;
source: dec-6mytiutr «except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.»
replace-with: its options' `net_provider` replaced by the stand's `LoadNet`, every other field kept, and the error-text check's failing page offline but for its `FailingNet` stand-in, which fails its fetch;
