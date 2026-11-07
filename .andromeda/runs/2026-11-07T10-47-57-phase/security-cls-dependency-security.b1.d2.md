@answer n=1 entity=dependency-security#1.0 v=yes
why: scope item 8 names the cargo-deny audit leg, which must still pass over the changed code, and this entity states that leg's command, graph and ignores
@answer n=2 entity=dependency-security#2.1 v=yes
why: the audit leg can go red on advisories published after a green audit, and scope item 8 requires it to pass, so the local audit run before a push binds
@answer n=3 entity=dependency-security#3.1 v=no
why: history of the 2026-10-10 sync's added and removed packages, with no present-tense pin or graph rule the chunk must keep
@answer n=4 entity=dependency-security#3.2 v=no
why: history of the 2026-10-13 sync's taffy, workspace version and cursor-icon changes, with no present-tense rule the chunk must keep
@answer n=5 entity=dependency-security#3.3 v=yes
why: states the live graph bound that Taffy stays the registry 0.15 with no git dependency and Dioxus 0.7.3 is unchanged, which scope item 8 keeps
@answer n=6 entity=dependency-security#3.4 v=yes
why: states the live bound that no dependency enters, leaves or moves and the root manifest has no git dependency, which scope item 8 keeps
@answer n=7 entity=dependency-security#3.5 v=yes
why: states the live bound that no dependency enters, leaves or moves and Taffy stays the registry with no git dependency, which scope item 8 keeps
@answer n=8 entity=dependency-security#3.6 v=yes
why: states that Dioxus stays at 0.7.3, that 0.8.0-alpha.3 from 286874cc1 is not taken, the exact bound scope item 8 gives, and that the lock keeps its 987 packages
@answer n=9 entity=dependency-security#3.7 v=yes
why: states the pins stay, Dioxus 0.7.3 among them, with no git dependency and no package entering, leaving or moving, the bound scope item 8 keeps
@answer n=10 entity=dependency-security#3.8 v=yes
why: states the pins stay, Dioxus 0.7.3 among them, and the lock unchanged, the bound scope item 8 keeps
@answer n=11 entity=dependency-security#4.1 v=no
why: serde_json is the reader of MCP stdin and of kept checks, which the chunk does not touch and whose feature set it does not change
@answer n=12 entity=dependency-security#5.1 v=no
why: the png encoder for driver and blitz-tests screenshots, which the chunk does not reach and whose lock entries it does not add
@answer n=13 entity=dependency-security#6.1 v=no
why: color is the luminance source of blitz-test-harness's contrast reader, which the chunk does not touch and whose dependency list it does not change
@answer n=14 entity=dependency-security#7.1 v=yes
why: the licence table is the licences half of the cargo-deny audit leg, which scope item 8 names and requires to pass over the changed code
@answer n=15 entity=dependency-security#7.2 v=no
why: a free-disk CI action in the Stand checks repeated job, while the chunk edits no workflow and adds no CI job, so it reaches nothing the chunk does
@answer n=16 entity=dependency-security#7.3 v=no
why: the matrix_test free-disk pin, while the chunk edits no workflow and adds no CI job, so it reaches nothing the chunk does
@answer n=17 entity=dependency-security#7.4 v=yes
why: a --locked cargo leg in LEGS and FAST_LEGS, and scope item 8 requires every --locked cargo leg to pass over the changed code
