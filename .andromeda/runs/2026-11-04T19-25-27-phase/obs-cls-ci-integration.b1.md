@answer n=1 entity=ci-integration#1.1 v=no
why: WPT report and score archiving to Pages and the upstream dispatch are upstream-only WPT publishing, named not needed; the probe touches none of it
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json computed and published to Pages is upstream WPT score publishing, named not needed
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg merged CI log and its failure-only artifact upload are CI telemetry artifact handling, named not needed; the probe prints nothing into that log
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov artifact and its keyboard_compat exclusion are merged coverage artifact handling, named not needed; the probe adds no counter or coverage rule
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run harness state under target/agent-run is the agent-run harness log and verdict state, named not needed; the chunk's probe is a stand check, not an agent-run event
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory belongs to the session library and its named checks; the probe boots the headless stand in process with no host socket, so the directory rule is not reached
@answer n=7 entity=ci-integration#1.7 v=no
why: the recording directory is the driver's recording surface, named not needed; the probe records nothing and names no recorder or Session::recording_into
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe state is the cold-agent pipe log, named not needed; the probe runs no cold-agent pipe
@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent recording directory is the cold-agent pipe's live recording, named not needed; the probe runs no live recording
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed kept checks are read by stand_replay and the freshness check and named not needed as kept checks at rest; the probe writes and reads no check file
