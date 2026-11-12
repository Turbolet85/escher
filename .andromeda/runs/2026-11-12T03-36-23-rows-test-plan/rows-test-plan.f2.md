@row_add n=1 row=decision/telemetry-target-allowlist area=3-test-harness-contract rec=1,2
name: Telemetry target allowlist
rule: A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names.
guard: telemetry_scrub, telemetry_drop
