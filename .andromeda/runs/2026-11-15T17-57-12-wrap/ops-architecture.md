@row_set n=1 row=decision/justified-expect-unwrap cell=except seen=2 detector=none
value:
basis: fct-pval6zhh, fct-u52kaqtk
because: The favicon test module's expect_used allow now stands below its justification and no bare browser allow remains, so the except names no exception and must be left out.
@row_set n=2 row=decision/dummy-noop-prefix cell=except seen=1 detector=none
value: The browser document loader's test module names its no-op shell provider stub NoShell.
basis: fct-xvnlarmr
because: The chunk adds a no-op ShellProvider implementation named NoShell, not prefixed Dummy, so the convention's exception must name it.
@row_set n=3 row=decision/dummy-prefix cell=except seen=1 detector=none
value: The browser document loader's test module names its no-op shell provider stub NoShell.
basis: fct-xvnlarmr
because: The chunk adds a no-op ShellProvider implementation named NoShell, not prefixed Dummy, so the convention's exception must name it.
