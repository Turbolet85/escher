@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive to GitHub Pages and the dispatch to blitz-wpt-results is upstream CI publishing that the chunk's escher-driver, dioxus-native-dom and stand code never reaches
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed from the WPT run and published to Pages upstream only, which the chunk does not touch
@answer n=3 entity=ci-integration#1.3 v=no
why: ci-leg.sh's per-leg log is CI build output the chunk's code neither writes nor reads, and the silence the new checks owe comes from their own rule, not from this log
@answer n=4 entity=ci-integration#1.4 v=no
why: the lcov coverage report names no floor or measurement the chunk's scope changes, and the scope does not name the coverage leg
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run harness state under target/agent-run is written by agent-run.sh, which the chunk does not reach
@answer n=6 entity=ci-integration#1.6 v=yes
why: the new stand checks (covered, off-screen, owner-served) open sessions through session_common and the host, so their check names are instances this list must record, and the session directory must hold the socket alone with no screen content written into it
@answer n=7 entity=ci-integration#1.7 v=yes
why: recording.rs writes each answered call's accepted flag, answer digest and answer whole into the kept check, so the new read's ids and closed flag words land at rest here under the 400-record and 1,048,576-byte bounds
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe's state files and its span count by name are not reached, since the chunk changes neither cold-agent.sh nor the span names, the new read emitting only the existing command and tool_call spans
@answer n=9 entity=ci-integration#1.9 v=yes
why: the cold-agent live run's escher-session mcp --record child keeps the new read's answered calls in its kept check, so the answer must stay kept whole there and never be printed or copied, and no check of the live run may be committed
@answer n=10 entity=ci-integration#1.10 v=yes
why: replay.rs, reached whole, must still read the two committed Counter checks through stand_replay and the seven_guis freshness check, so the recording format change cannot break their read
