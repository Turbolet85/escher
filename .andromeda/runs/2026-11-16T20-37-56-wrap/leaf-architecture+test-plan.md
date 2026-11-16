@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=83
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=61 class=stale
leaf-text: a handler Err the app's root boundary takes checked on a session of its own, its sentinel check reading `caused` alone
source: dec-xd4p3k72 «decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain.»
replace-with: a handler Err the app's root boundary takes checked on a session of its own, where `caused` carries its error text and its chain
