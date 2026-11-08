@answer n=1 entity=pii-scrubbing-compliance#1.1 v=yes
why: the host_log and mcp_log readings at trace and the command and tool_call span fields bind through the new verb (scope items 1, 2, 9)
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=yes
why: the graph answer shape (stable ids, closed flag words, booleans only, no state value) is the sibling of the new Outcome variant, held to the same ids-and-booleans-only rule (scope item 5)
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: the multiple-select value crosses the engine's form entries and snapshot value, which the new answer neither reads nor carries
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: the size attribute read on a multiple select is not read by the actionable read or carried in its answer
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: the optgroup label stays inside the accessibility name crossing, which the answer's ids, verb names, cause words and booleans never carry
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the open list row's bounds crossing the snapshot and diff is not read by the actionable read
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=yes
why: the actionable read reaches the closed clickable bool through actionable.rs and carries it only as a boolean, so its rule binds
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: SelectListRow.label is page text that the answer never carries and the chunk does not read the open list row type
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: the checkbox and radio change event is an engine input path that the read neither dispatches nor reads
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: URLSearchParams and the search-params crossing are a page script surface the chunk does not touch
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no standing=S1
why: the probe's no-print, no-env, no-subscriber, no-listener and readings-into-assertions rule is the convention every new stand check owes
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: MutationBuffer is crate-private to the render path, which the read does not reach
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=no
why: SignalRegistry holds state values and names that the answer never carries and the read does not touch
@answer n=14 entity=pii-scrubbing-compliance#2.1 v=yes
why: the command span and tool_call span fields that the new read and verb emit fall under this allowlist rule (scope items 1, 2)
@answer n=15 entity=pii-scrubbing-compliance#2.2 v=no
why: the failed-load error text travels the resource-load path, which the chunk does not reach
@answer n=16 entity=pii-scrubbing-compliance#3.1 v=yes
why: the escher-session host installs this sink, and the new verb's host_log reading at trace under both builds, with the command line on stdout, runs through it (scope items 8, 9)
@answer n=17 entity=pii-scrubbing-compliance#4.1 v=no
why: the cold-agent pipe log and transcript sit outside the chunk's files and scope, and the new answer is not a cold-agent surface the chunk changes
@answer n=18 entity=pii-scrubbing-compliance#5.1 v=no
why: the absence of redaction over the apps slice, which the chunk does not reach, states no rule the chunk must honour
@answer n=19 entity=pii-scrubbing-compliance#5.2 v=no
why: the absence of scrubbing over the s05 files, which the chunk does not reach, states no rule the chunk must honour
@answer n=20 entity=pii-scrubbing-compliance#5.3 v=no
why: the absence of redaction of logged values over the s06 listed files, which the chunk does not reach, states no rule the chunk must honour
@answer n=21 entity=pii-scrubbing-compliance#5.4 v=no
why: the absence of scrubbing over the layout slice, which the chunk does not reach, states no rule the chunk must honour
@answer n=22 entity=pii-scrubbing-compliance#5.5 v=no
why: the absence of redaction over the 16 listed files states no rule the chunk must honour
@answer n=23 entity=pii-scrubbing-compliance#5.6 v=no
why: the absence of scrubbing code over the s09 files, which the chunk does not reach, states no rule the chunk must honour
@answer n=24 entity=pii-scrubbing-compliance#5.7 v=no
why: the absence of redaction over the blitz-vibey-script and blitz slice, which the chunk does not reach, states no rule the chunk must honour
@answer n=25 entity=pii-scrubbing-compliance#5.8 v=no
why: the absence of scrubbing over the s11 files, which the chunk does not reach, states no rule the chunk must honour
