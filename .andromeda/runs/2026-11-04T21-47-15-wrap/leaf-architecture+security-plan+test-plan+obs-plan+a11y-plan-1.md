@leaf_group group=architecture+security-plan+test-plan+obs-plan+a11y-plan-1
@leaf_read leaf=CLAUDE.md lines=153
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=98 class=omission
leaf-text: a leg with no CI job of its own), as CI runs them
source: fct-6a266lkz «the local `fast` leg runs no audit — `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push»
replace-with: a leg with no CI job of its own), as CI runs them; it runs no audit, and a CI Dependency audit can go red on advisories published after the base's green one with `Cargo.lock` and `deny.toml` unchanged — `bash .github/scripts/ci-leg.sh audit` is the local reproduction to run before a push
