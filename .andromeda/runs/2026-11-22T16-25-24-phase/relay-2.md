# The operator's P4 answer (question dialog, 2026-11-22)

Question: which bar the plan holds `build_accessibility_node` to. The function cannot read exactly upstream's 19
by keeping upstream's text plus one call: the fork deleted three of upstream's reads (bool-parsed `aria-selected`,
unfiltered `aria-label`, plain `aria-expanded`) and replaced them, under the a11y rule «one read per attribute — keep
it so». On scratch copies, scored with rust-code-analysis-cli: upstream's text minus those three reads plus one call
reads 13; upstream's text verbatim plus one overriding call reads 19.

Options:
1. At most 19 (Recommended) — the body is upstream's text without the three reads escher replaced, plus one call into
   escher's own functions, reading 13; one read per attribute kept; acceptance «at most upstream's 19».
2. Exactly 19 — upstream's text verbatim with its three reads restored, then one overriding call; each of the three
   attributes read twice, against the a11y rule.

Answer: «At most 19 (Recommended)»
