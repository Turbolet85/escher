@answer n=1 entity=log-coverage#9.5 v=yes
why: names host_log's select-list trace check at 0 option needles, which task item 10 requires to still hold under both builds
@answer n=2 entity=log-coverage#9.6 v=yes
why: names mcp_log's trace check at 0 option needles, which task item 10 requires to still hold under both builds
@answer n=3 entity=log-coverage#9.7 v=yes
why: option_needles in the shared common module is the needle set that the 0-needle trace re-measure of host_log and mcp_log reads
@answer n=4 entity=log-coverage#9.8 v=yes
why: the silence census over tests/common/mod.rs and tests/fixtures/mod.rs is the served-stand census that task item 11 says must still read 0 over the shared modules
@answer n=5 entity=log-coverage#9.9 v=yes
why: records escher-driver's 2 info_span! sites, which task item 6 names as the scope's own obligation that the probe adds no span
@answer n=6 entity=log-coverage#9.10 v=no
why: blitz-vibey-script tests/dom.rs is not reached by the probe, and its test-only TMPDIR read is not the probe's code
@answer n=7 entity=log-coverage#9.11 v=yes
why: the upstream take census names host_log and mcp_log at 0 needles at trace under both builds and escher-driver's spans, which task items 4, 6 and 10 name for this chunk at HEAD after the upstream sync
@answer n=8 entity=log-coverage#9.12 v=no
why: the silence census over five existing engine checks is a record over files the probe does not touch, and the scope names none of them
@answer n=9 entity=log-coverage#9.13 v=no
why: the census over the three popup checks is not the probe's file, stand.rs or a shared module that task item 11 names, and the probe reads none of them
