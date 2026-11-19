@row_set n=1 row=decision/rust-tests-on-built-in-harness cell=except seen=1 detector=D-tests-framework
value: upstream's merged checks that return TestResult<()> and assert with test-that's verify_that!, under the same harness
basis: fct-bhfcjucw
because: The merge brings upstream's excludes_button_with_visibility_collapse in accessibility_hidden, which returns TestResult<()>, so naming only two checks in the exception leaves out the upstream checks that use the same form.
@row_add n=1 row=decision/upstream-defect-fixed-with-own-witness area=test-strategy fact=F1 detector=D-tests-coverage
name: Upstream defect found in a sync fixed with its own witness
rule: An upstream engine defect found during a sync is fixed in escher with its own blitz-dom witness, which is first run red on an upstream clone at the merge base, and the fix is flagged upstreamable.
why: Ratified.
guard: an_inline_block_in_text_no_font_can_shape_is_placed_once_its_own_font_loads
basis: fct-douoqnv6
because: The sync settled how an upstream engine defect is handled, the inline-box placeholder fix and its blitz-dom witness read red on an upstream clone first (fct-sfoxdkv6), and no test-strategy row says so.
@row_add n=2 row=decision/devtools-cdp-server-upstream-tests-only area=test-strategy fact=F2 detector=D-tests-coverage
name: Devtools CDP server tested by upstream's tests alone
rule: The devtools CDP server crate is tested by upstream's own integration tests alone, kept as written, and escher adds no test of its own to it.
why: The crate is opt-in behind the devtools feature and no default feature set builds it.
basis: fct-o37cj4zg
because: The merge adds the CDP server crate as a workspace member whose only tests are upstream's integration tests and whose env reads have none of escher's (fct-xcqgt2ri: no default feature set turns it on), so the test-plan must state that this new path's coverage is chosen, not missing.
