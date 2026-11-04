@answer n=1 entity=logging-monitoring#1.1 v=yes
why: the scope names escher's own sink as the probe meets it (item 11), with its target allowlist, field redaction and closed span lines, which the probe's runs pass through
@answer n=2 entity=logging-monitoring#1.2 v=no standing=S6
why: the convention that test diagnostics and debug dumps on stdout or stderr hold nothing typed or carried beyond the masked values
@answer n=3 entity=logging-monitoring#2.1 v=no
why: the probe fetches nothing remote (not_needed item 2), so the failed-load line never fires in it, and the sink's generic redaction is met through the unchanged allowlist
@answer n=4 entity=logging-monitoring#3.1 v=no
why: the select-list host and role tests are existing tests; the chunk names no select list, its probe opens none, and it changes no existing test
@answer n=5 entity=logging-monitoring#4.1 v=no
why: the select list's lack of log sites, span fields and logged labels is a property of blitz-dom, blitz-paint and the existing stand_act_spans, which the chunk neither uses nor changes
@answer n=6 entity=logging-monitoring#5.1 v=no standing=S5
why: each committed copy of evidence holds no host path (masked), the convention; the cold-agent pipe itself is not_needed (item 7), as the probe runs no model session
