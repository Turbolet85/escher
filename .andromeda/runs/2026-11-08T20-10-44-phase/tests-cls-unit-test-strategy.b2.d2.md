@answer n=1 entity=unit-test-strategy#25.1 v=yes
why: the scope names the reserved virtual form of the stable element id and that no virtual node is produced (task item 8), which the walk and the coverage count must honour.
@answer n=2 entity=unit-test-strategy#26.1 v=yes
why: it states the one actionable rule read by actionable_nodes, the reading the actions verb carries (task item 5) and the agent-visible count of item 8 is over.
@answer n=3 entity=unit-test-strategy#27.1 v=no
why: WPT runner tests fall under the not_needed scope (task item 1), the chunk adding nothing rendered against WPT.
@answer n=4 entity=unit-test-strategy#28.1 v=no
why: an absence of tests over slice s03's files; no named chunk file is among them and the scope states no rule over that slice.
@answer n=5 entity=unit-test-strategy#28.2 v=no
why: an absence of tests over the other seven files of slice s07; no named chunk file is among them and the scope states no rule over that slice.
@answer n=6 entity=unit-test-strategy#28.3 v=no
why: the absence of tests in blitz-vibey-script falls under the not_needed scope (task item 3).
@answer n=7 entity=unit-test-strategy#28.4 v=no standing=S5
why: the placement of unit tests in tests/blitz-tests is S5's convention, and the chunk's unit tests sit in dioxus-native-dom source files, adding no in-file cfg(test) module there.
