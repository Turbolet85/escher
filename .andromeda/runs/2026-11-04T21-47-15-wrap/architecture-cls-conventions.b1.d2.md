@answer n=1 entity=conventions#1.1 v=no
why: the fmt and clippy legs are unchanged, and the CI-green 17/17 run on 9b3b29714 already covers the new target under them.
@answer n=2 entity=conventions#2.1 v=no
why: no Cargo.toml moved and no dependency is declared, and the Cargo.lock change is a lock-only pulldown-cmark bump that the manifest rule does not cover.
@answer n=3 entity=conventions#3.1 v=no
why: no packages/ or examples/ file moved and the new test file adds no public item, so the cited `//!` and `///` sites are unmoved.
@answer n=4 entity=conventions#4.1 v=no
why: every cited TODO/FIXME marker sits in an untouched packages/ source, and the new should_panic pins add no marker.
@answer n=5 entity=conventions#5.1 v=no
why: no pub or pub(crate) item and no re-export moved, and the new target is an integration test reading public API only.
@answer n=6 entity=conventions#6.1 v=no
why: no feature, cfg gate or tracing site moved, the probe installs no subscriber, and the lock-only bump adds no menu or muda package.
@answer n=7 entity=conventions#7.1 v=no
why: the cited blitz-dom workaround sites are unmoved and still hold, and the unworked MountedData borrow conflict is recorded in Established Decisions rather than in this workaround list.
@answer n=8 entity=conventions#8.1 v=yes mode=add-to
why: the new integration target dioxus_runtime_probe.rs is a non-stand_ test file in both layout modes asserting hazard_ready first, which the list of stand_ checks does not name.
@answer n=9 entity=conventions#9.1 v=no
why: the escher-driver test modules cited here are in sources the session did not touch.
@answer n=10 entity=conventions#9.2 v=no
why: fixtures/mod.rs is unmoved, and the probe's own pages (toggled_listener, shared_state, hazard) have one reader each, so they stay in the probe's file as the rule says.
@answer n=11 entity=conventions#9.3 v=no
why: the two cited row-walk tests, mcp_tests.rs and stand_requirement_sweep.rs, are untouched by the session.
@answer n=12 entity=conventions#10.1 v=no
why: no examples/ or apps/ file moved, and the probe reads the seven_guis stand without changing it.
@answer n=13 entity=conventions#11.1 v=no
why: no examples/assets HTML fixture moved and no fixture was added.
