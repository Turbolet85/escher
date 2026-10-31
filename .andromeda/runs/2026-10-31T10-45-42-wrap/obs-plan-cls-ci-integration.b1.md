# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT publish and its repository guards live in wpt.yml, which the session does not reach
ci-integration#1.2 · no · wptscores.json is computed and published by wpt.yml, which the session does not reach
ci-integration#1.3 · yes · stale · the free-disk step at ci.yml:311-316 shifts the upload citation at ci.yml:422-431 by six lines, and the stand-repeat edit can move the citation at ci-leg.sh:89-90
ci-integration#1.4 · no · the report names no new artifact or upload, so the coverage-report upload still reads true, and its cited lines at ci-leg.sh:40-47 and ci.yml:286-291 sit above the lines the session moved
ci-integration#1.5 · no · scripts/agent-run.sh is not reached, and the report names no new upload, so the state area, its cleanup and the uploaded-by-no-CI-leg claim still hold
ci-integration#1.6 · no · host.rs, session_common, the seven_guis common module and host_log.rs are not reached, and the session adds no session-directory check or upload
ci-integration#1.7 · no · recording.rs and session.rs are not reached, and the report adds no upload, verb, key or field, so the moved answer text stays a fixed answer kept whole
ci-integration#1.8 · no · scripts/cold-agent.sh is not reached, the report names no new upload or cold-agent leg, and the driver's two info_span sites read unmoved
ci-integration#1.9 · no · scripts/cold-agent.sh is not reached, and the session adds no committed check or upload, so the two Counter checks stay the only committed ones
ci-integration#1.10 · no · the two Counter checks and record_checks.sh are not reached, and the session adds no committed check or upload
