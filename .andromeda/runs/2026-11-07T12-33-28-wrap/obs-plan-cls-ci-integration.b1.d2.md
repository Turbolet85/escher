@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT workflow wpt.yml and its archive path are not in the session's changed files
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT run in wpt.yml, which the session does not reach
@answer n=3 entity=ci-integration#1.3 v=no
why: ci-leg.sh and ci.yml are not reached, and the buffer adds no log site so the leg's merged stdout and stderr keep their kind
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage leg's scripts are not reached, and the lcov still holds line counts of public source with keyboard_compat.rs excluded by name
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run.sh is not reached and the probe stays out of the run stand, so the harness output does not carry the renamed pins
@answer n=6 entity=ci-integration#1.6 v=no
why: host.rs and the session_common and seven_guis check modules are not reached, and the buffer writes nothing into the session directory
@answer n=7 entity=ci-integration#1.7 v=no
why: recording.rs and session.rs are not reached, and the buffer holds its contents in process memory only, never at rest in a recording
@answer n=8 entity=ci-integration#1.8 v=no
why: cold-agent.sh is not reached, and the replay re-fires MutationWriter's existing trace calls unchanged so the server stderr span counts stand
@answer n=9 entity=ci-integration#1.9 v=no
why: cold-agent.sh is not reached, and no recording writer or replay child flag is changed by the session
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed Counter checks and record_checks.sh are not reached, and the session writes no new check content
