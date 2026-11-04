@answer n=1 entity=design-philosophy#1.1 v=no
why: its citations sit in the root Cargo.toml, packages/, examples/ and ci.yml, none reached; the session moved only the probe file, Cargo.lock and deny.toml.
@answer n=2 entity=design-philosophy#2.1 v=no
why: its DioxusDocument citations (dioxus_document.rs:57, :83-84) and the poll flush claim hold at HEAD, and no packages/ line changed.
@answer n=3 entity=design-philosophy#3.1 v=no
why: its spec and browser-engine citations sit in packages/ and unchanged tests, and the probe carries no spec or engine claim.
@answer n=4 entity=design-philosophy#4.1 v=no
why: its incremental-pipeline citations sit in packages/ and unchanged tests, and no node-id, damage or layout behaviour moved.
@answer n=5 entity=design-philosophy#5.1 v=no
why: integration tests still live under tests/ and the new probe runs headless with no window, so the claim holds on the changed tree.
