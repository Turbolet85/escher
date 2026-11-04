@answer n=1 entity=telemetry-strategy#1.1 v=no
why: the change moves only Cargo.lock lines 6505 and 6507 (pulldown-cmark), so the lock's tracing 0.1.44 entry holds
@answer n=2 entity=telemetry-strategy#2.1 v=no
why: no Cargo.toml feature wiring moved and the new target installs no subscriber, so the workspace-versus-package feature split and the sink claim hold
@answer n=3 entity=telemetry-strategy#3.1 v=no
why: the absence search covers workflows, Cargo.toml, Cross.toml, flake.nix and the scripts, none of which the change moved
@answer n=4 entity=telemetry-strategy#3.2 v=no
why: the change reaches no file of the 32-file examples slice, the new probe sitting under tests/
@answer n=5 entity=telemetry-strategy#3.3 v=no
why: the probe carries no log site, span or logging crate term, and the change reaches none of the 61 slice files
@answer n=6 entity=telemetry-strategy#3.4 v=no
why: the change reaches none of the 12 wpt runner slice files
