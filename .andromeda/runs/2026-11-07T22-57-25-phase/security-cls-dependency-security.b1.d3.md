@answer n=1 entity=dependency-security#1.0 v=yes
why: the audit leg's form (cargo-deny advisories and licences, --locked, whole workspace graph) is the audit leg that scope item 16 names.
@answer n=2 entity=dependency-security#2.1 v=yes
why: the local reproduction of the audit leg to run before a push is the audit leg scope item 16 names.
@answer n=3 entity=dependency-security#3.1 v=no
why: a 2026-10-10 upstream sync record that added six packages; the chunk takes no sync and adds no package, so the review rule for added packages is not owed.
@answer n=4 entity=dependency-security#3.2 v=no
why: a 2026-10-13 upstream sync record of a Taffy move and a workspace version bump; the chunk takes no sync and moves no pin.
@answer n=5 entity=dependency-security#3.3 v=no
why: a 2026-10-17 upstream sync record that moved keyboard-types and test-that; the chunk takes no sync and moves no package.
@answer n=6 entity=dependency-security#3.4 v=yes
why: it states Cargo.toml, Cargo.lock and deny.toml stay unchanged with no git dependency and the audit leg green, the no-new-package and no-fork rule scope item 16 names.
@answer n=7 entity=dependency-security#3.5 v=yes
why: it states the same unchanged lock, no git dependency and audit-green state, the rule scope item 16 names.
@answer n=8 entity=dependency-security#3.6 v=yes
why: it states the pins stay (Dioxus 0.7.3, Taffy registry, no git dependency) with the package set unchanged, the no-fork and no-new-package rule scope item 16 names.
@answer n=9 entity=dependency-security#3.7 v=yes
why: it states the current pins (Dioxus 0.7.3, winit, Taffy registry, no git dependency) and an unchanged lock, the state the chunk's dioxus-core public-API use rests on under scope item 16.
@answer n=10 entity=dependency-security#3.8 v=yes
why: it states the current pins stay with no git dependency and the audit leg reading ok, the rule scope item 16 names.
@answer n=11 entity=dependency-security#4.1 v=yes
why: serde_json is the read-only, feature-less reader of the MCP server's stdin and of Check::from_bytes, both named by scope items 8 and 10.
@answer n=12 entity=dependency-security#5.1 v=no
why: the png encoder of a headless screenshot, which the chunk neither calls nor changes.
@answer n=13 entity=dependency-security#6.1 v=no
why: the blitz-test-harness contrast reader's color dependency, a path the chunk does not reach.
@answer n=14 entity=dependency-security#7.1 v=yes
why: licences run as part of the cargo-deny audit leg that scope item 16 names, over the whole workspace graph under --locked.
@answer n=15 entity=dependency-security#7.2 v=no
why: a CI action step in the Stand checks job of ci.yml, and CI action pinning is a not_needed item of this chunk.
@answer n=16 entity=dependency-security#7.3 v=no
why: the matrix_test free-disk step's absence in CI, a CI workflow the chunk does not touch.
@answer n=17 entity=dependency-security#7.4 v=no
why: the embolden locked test leg, a blitz-paint font-feature test the chunk does not touch.
