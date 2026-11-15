@row_set n=1 row=decision/no-git-dependencies cell=except seen=1 detector=D-arch-decisions
value: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
basis: fct-xv7xpnit
because: The no-git-dependency rule now carries one exception, for the taffy crate alone, and this row states the rule in the architecture as its two security-plan twins do in their own except cells.
@row_set n=2 row=decision/upstream-take-engine-changes cell=rule seen=2 detector=D-arch-decisions
value: The upstream take read at `ea07ee716` — the `@font-face` `font-stretch` read reaching font matching and the registry taffy 0.15.0 → 0.15.1 move — is an engine behaviour change for every document.
basis: fct-ysqotf4p
because: This take's two behaviour changes for every document are ratified, and the row that records the ratified upstream take still names the previous take's twenty-nine commits.
@row_add n=1 row=decision/mutation-tier-excludes-demo-shells area=stack-and-technologies detector=D-arch-decisions
name: Demo shells outside the mutation tier
rule: The mutation tier does not run the rdme and blitz packages and owes them no test for it; churn, sizes and the other collectors keep both.
why: Both are upstream's demo shells and hold no tests.
basis: fct-wuhhml6l
because: The founder rules the two test-less units out of the mutation tier's scope, and the code audit reads its extra exclusions only from an architecture decision row.
