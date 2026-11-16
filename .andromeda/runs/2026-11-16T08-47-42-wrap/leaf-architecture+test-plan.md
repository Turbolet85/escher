@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/tests
@leaf_edit n=1 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_start_in_state --test stand_explore`
source: dec-xd4p3k72 «decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, its sentinel check reading caused alone.»
replace-with: --test stand_start_in_state --test stand_explore --test stand_act_errors` (`stand_act_errors`: a handler Err the app's root boundary takes checked on a session of its own, its sentinel check reading `caused` alone)
