@account n=1 entity=existing-scopes#1.3 v=not_in_scope
why: blitz-traits' module list; the probe reaches no provider trait and no `NodeId` crosses, so it only keeps holding
@account n=2 entity=existing-scopes#1.4 v=not_in_scope
why: blitz-dom's module list; the probe reads the document through the bridge and changes no blitz-dom module
@account n=3 entity=existing-scopes#1.7 v=not_in_scope
why: hit testing, the select list and CSSOM geometry; no link of the UI graph reads geometry or a select
@account n=4 entity=existing-scopes#1.9 v=not_in_scope
why: blitz-dom's event submodules are only driven by the probe's input; the hazard's `events.rs` is dioxus-native-dom's
@account n=5 entity=existing-scopes#1.12 v=not_in_scope
why: node storage, traversal and URL handling; the probe reads none of them as a link's answer
@account n=6 entity=existing-scopes#1.13 v=not_in_scope
why: blitz-dom's layout modules; the probe runs in both layout modes but reads and changes no layout module
@account n=7 entity=existing-scopes#1.14 v=not_in_scope
why: node data, range and select modules; the probe reads no select and no node module's state
@account n=8 entity=existing-scopes#1.19 v=not_in_scope
why: blitz-shell's module list; the windowed path is read, not run, and no shell module is named by the scope
@account n=9 entity=existing-scopes#1.21 v=not_in_scope
why: stylo_taffy's modules; the probe reaches no style-to-layout conversion
@account n=10 entity=existing-scopes#1.22 v=not_in_scope
why: dioxus-native's module list; its launch path is only read beside the hazard, and the scope names the bridge, not these modules
@account n=11 entity=existing-scopes#1.23 v=carried
why: the bridge's modules are the probe's surface for links (1) and (2) and the hazard's `dioxus_document.rs` and `events.rs`
@account n=12 entity=existing-scopes#1.24 v=not_in_scope
why: the harness's module list; the probe drives it unchanged and the entity states none of the `poll(None)` behaviour the scope names
@account n=13 entity=existing-scopes#1.25 v=not_in_scope
why: the telemetry bootstrap's modules; the stand installs no telemetry and the probe adds no log line, field or span
@account n=14 entity=existing-scopes#1.26 v=carried
why: the served task the probe runs beside a lean one is held through the driver's `Session`, to which nothing is added
@account n=15 entity=existing-scopes#1.31 v=carried
why: blitz-tests is where a probe check lives; it holds the stand checks over Counter and CRUD and the pinned count of input-reacting elements
@account n=16 entity=existing-scopes#1.32 v=carried
why: seven_guis is the stand the probe measures on: the lean tasks and the served ones behind `escher-session`
@account n=17 entity=existing-scopes#2.1 v=carried
why: the served-page checks boot and hold the served task the probe runs beside a lean one
@account n=18 entity=existing-scopes#3.2 v=carried
why: a probe check reading a shared module of blitz-tests is one more reader the stated counts record
@account n=19 entity=existing-scopes#4.1 v=not_in_scope
why: engine-fact targets for transforms, open state and change events; the probe adds no engine fact and reads none of them
