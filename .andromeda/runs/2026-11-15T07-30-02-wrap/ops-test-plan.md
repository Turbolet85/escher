@row_add n=1 row=decision/mutation-tier-excludes-demo-shells area=10-quality-gates-coverage-targets fact=F1 detector=none
name: Demo shells outside the mutation tier
rule: The mutation tier does not run the rdme and blitz packages, so no test is owed them for that tier; churn, sizes and the other collectors keep both.
why: Both are upstream's demo shells and hold no tests.
basis: fct-wuhhml6l
because: The founder ruled rdme and blitz out of the mutation tier's scope, a scope decision no standing mutation row of this section states.
