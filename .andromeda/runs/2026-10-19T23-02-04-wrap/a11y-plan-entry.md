
## 2026-10-19-shared-fixtures — the MCP masked password input reads the shared page
**Section:** §7 Screen Reader Support (the snapshot model's MCP masked-password clause, `stand_mcp_masked`)
**Change:** the MCP `type` password input was «a test-only element of `stand_mcp_masked`'s fixture» and is now a test-only element of the shared password page `fixtures::password_named`; its citation moves from the removed file-private page to `tests/blitz-tests/tests/fixtures/mod.rs`. The reading itself is unchanged: `MASKED_VALUE`, role `PasswordInput`, a non-empty name, the sentinel 0 times, both layout modes.
**Why:** the citation sweep read the cited lines deleted. The page moved into the shared module this chunk, with the same markup and ids (the playbook's «Accurate this-chunk addition»).
**Ref:** .andromeda/runs/2026-10-19T23-02-04-wrap/
