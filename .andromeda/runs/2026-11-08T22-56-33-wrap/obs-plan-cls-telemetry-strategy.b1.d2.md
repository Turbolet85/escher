@answer n=1 entity=telemetry-strategy#1.1 v=yes mode=add-to
why: the coverage role now also runs Session::run per replayed step, so the command span's reaching roles must name it beside host and mcp
@answer n=2 entity=telemetry-strategy#2.1 v=no
why: feature wiring and the engine emitters are untouched (no manifest or feature change, blitz-dom not reached), and the measured host flow it cites is unchanged
@answer n=3 entity=telemetry-strategy#3.1 v=no
why: the session adds no dependency, manifest, Cargo.lock, workflow or script, so the searched backend set still holds
@answer n=4 entity=telemetry-strategy#3.2 v=no
why: the session adds no log site, span, event or print to the examples files it reaches, so the absence of tracing and log calls holds
@answer n=5 entity=telemetry-strategy#3.3 v=yes mode=stale
why: the new test-crate file stand_handler_coverage.rs moves the 61-file slice count it cites, and holds no subscriber, env read or print
@answer n=6 entity=telemetry-strategy#3.4 v=no
why: none of the twelve wpt runner files is in the change, and the searched span and tracing terms match nothing it adds
