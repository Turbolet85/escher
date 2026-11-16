@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file outp
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: and the restored tree reads green again;
source: dec-a7xrcpzo «decision/expected-value-pinned-as-literal · Expected value pinned as a literal — A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal.»
replace-with: and the restored tree reads green again; a check never derives its expected value from the constant its mutation control changes, but pins the measured value as a literal;
@leaf_edit n=2 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: and is not unix-gated but `mcp_server`'s two command-line comparisons;
source: dec-jvtfzuca «decision/mcp-result-decoded-whole · MCP tool result text decoded whole — A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper.»
replace-with: and is not unix-gated but `mcp_server`'s two command-line comparisons, and a check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper (un-escaping quotes alone leaves a Windows path's backslashes doubled);
@leaf_edit n=3 leaf=.claude/rules/testing.md line=30 class=omission
leaf-text: and assert unconditionally
source: dec-hmr2374t «decision/engine-items-asserted-on-tracing-build · Engine caused items asserted on a tracing build only — A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant»
replace-with: and assert unconditionally but engine-sourced caused items, asserted only on a build with the engine's tracing compiled in, read through the engine's own constant, the check running on both builds
