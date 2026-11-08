@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT report archive runs only in wpt.yml on upstream; the chunk changes no workflow and writes no WPT artifact.
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed from the WPT run, which the chunk does not touch.
@answer n=3 entity=ci-integration#1.3 v=yes
why: the owed replay test's failure message lands in the leg's uploaded-on-failure log as unscrubbed build output, so it must keep to the step and differing key and carry no captured answer.
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov file holds line counts of public source, and the chunk changes no coverage input or upload.
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run state is local only and the scope names it untouched (not_needed n=7).
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory holds the socket alone and is a component the chunk uses unchanged; the kept check goes to the recording directory, not the session directory.
@answer n=7 entity=ci-integration#1.7 v=yes
why: the recording directory's carve-out (0700, refusal when open to group or others, call content at rest, no log line, no CI upload) is need n=7, and the recording module's kept check (need n=5) is written into it.
@answer n=8 entity=ci-integration#1.8 v=yes
why: the cold-agent server.log is escher's sink counted by span name, so need n=13 and a new verb's tool word (needs n=2 and n=12) bind its counted verbs list.
@answer n=9 entity=ci-integration#1.9 v=no
why: the never-printed, never-copied rule over the cold-agent record directory belongs to the pipe script, which the chunk does not change, and the chunk commits no kept check.
@answer n=10 entity=ci-integration#1.10 v=no standing=S5
why: a committed kept check holds no typed text, host path or secret; the chunk commits no check, its owed kept step living in the recording directory.
