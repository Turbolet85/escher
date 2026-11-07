@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT workflow (.github/workflows/wpt.yml) and its upstream-only archive are not reached by the buffered-render change, and no CI step moved
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT run in wpt.yml, which the session does not touch
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg log written by ci-leg.sh and the ci.yml upload steps are unchanged, and the session adds no log site or CI step
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage leg's lcov upload and its keyboard_compat exclusion stand as before, the session cites no coverage figure in this row and leaves the upload itself unchanged
@answer n=5 entity=ci-integration#1.5 v=no
why: scripts/agent-run.sh and the agent-run harness state are unchanged, and the probe stays out of the run stand
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory is created and held by escher-driver's host.rs and the session_common and seven_guis helpers, none of which the session changes
@answer n=7 entity=ci-integration#1.7 v=no
why: recording.rs and session.rs are unchanged, and the in-process mutation buffer writes nothing to the recording directory at rest
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe's state files and cold-agent.sh are unchanged, and the session reaches no cold-agent run
@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent recording directory and its live-run mcp --record child are unchanged, and the session does not reach the pipe
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed Counter checks and their record_checks.sh writer are unchanged, and stand_replay reads them as before
