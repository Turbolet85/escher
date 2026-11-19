@row_retire n=1 row=decision/platform-tree-rebuild seen=1 heir=decision/a11y-rebuild-on-nonempty-changed-set detector=none
why: A poll no longer rebuilds the platform tree; the redraw after resolve does, which decision/a11y-rebuild-on-nonempty-changed-set states.
basis: fct-xcqgt2ri, fct-lfuqrgmb
because: The proposer's set read not_master, and the standing rule names the poll site, false since the refresh moved to View::redraw after resolve.
@row_retire n=2 row=decision/stylo-prefs-set-once seen=1 detector=none
why: Escher's own prefs set by Stylo 0.22 names was replaced by upstream's set under the 0.24 names, so the rule's set and its difference from upstream are false.
basis: fct-wakes6es, fct-ubjyh5cg
because: Upstream's stylo/prefs.rs set replaces escher's init_stylo_prefs, and the heir the proposer offered read not_master: the code holds the set, so the row retires with no heir.
