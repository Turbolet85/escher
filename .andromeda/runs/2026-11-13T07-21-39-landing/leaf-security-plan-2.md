@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=7 class=omission
leaf-text: signing material written to disk in CI is removed in an `always()` step — keep that pairing.
source: dec-gkacphd2 «CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only.»
replace-with: signing material written to disk in CI is removed in an `always()` step — keep that pairing. `ci.yml` declares a workflow-level `contents: read` permission and no job-level grant, so every ci.yml job's `GITHUB_TOKEN` is read-only — keep it so.
@leaf_edit n=2 leaf=.claude/rules/security.md line=15 class=omission
leaf-text: same-origin/CORS checks are absent
source: dec-hbmegunx «A page script's fetch rejects with TypeError any method other than GET or HEAD.»
replace-with: a page script's `fetch()` takes GET or HEAD only, any other method rejected with a `TypeError`; same-origin/CORS checks are absent
@leaf_edit n=3 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: its colours are the engine's own, no page value
source: dec-gejjncmn «The open select list's border, highlight and disabled-text colours are the engine's own constants; its background and an ordinary row's text colour are the select's own page styles.»
replace-with: its border, highlight and disabled-text colours are the engine's own constants, its background and an ordinary row's text colour the select's own page styles
@leaf_edit n=4 leaf=.claude/rules/security.md line=21 class=omission
leaf-text: every answered request — an `attach` or a `status` included — starts the count again.
source: dec-jw7gytc7 «every answered request, refused or malformed included, restarts the count, and a connection broken before its request is whole does not.»
replace-with: every answered request — an `attach`, a `status`, a refused or a malformed one included — starts the count again, and a connection broken before its request is whole does not.
@leaf_edit n=5 leaf=.claude/rules/security.md line=22 class=stale
leaf-text: a `graph` records `verb` alone
source: ctr-jrtcmpib «Records the instance-level verb; a graph or actions call records the verb alone, plus cause when refused.»
replace-with: a `graph` records `verb` and, refused, `cause`
@leaf_edit n=6 leaf=.claude/rules/security.md line=28 class=omission
leaf-text: no git dependency is left in the root manifest
source: dec-jm3wczan «The workspace takes no git dependency.»
replace-with: the workspace takes no git dependency, the root manifest included
@leaf_edit n=7 leaf=.claude/rules/security.md line=28 class=omission
leaf-text: builds pass `--locked` — keep both.
source: dec-gg74dumo «Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment.»
replace-with: builds pass `--locked` — keep both. Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment — keep it so.
