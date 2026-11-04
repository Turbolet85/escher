@answer n=1 entity=integration-test-strategy#1.1 v=no
why: pseudo-element flow-classification tests state engine layout for ::before/::after, which the probe does not read; its both-layout-modes run is owed through the stand's options (needs 9), not this procedure
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script's DOM and script tests; the probe runs over the Dioxus runtime and reaches no script engine
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: the radio change rule is script-side event synthesis in blitz-vibey-script's runtime/events.rs, not dioxus-native-dom's events.rs, and the stand has no radio the probe clicks
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: URLSearchParams tests in blitz-vibey-script; nothing the probe reads
@answer n=5 entity=integration-test-strategy#5.1 v=no
why: escher-driver Session behaviour (act, run, settle, load wait, refusals, screenshots); the probe runs in process through harness helpers and never through Session (needs 18), so no call or value it states is carried
@answer n=6 entity=integration-test-strategy#5.2 v=no
why: session host socket lifecycle on unix; the probe never runs through a session host, socket or the escher-session binary (needs 18)
@answer n=7 entity=integration-test-strategy#5.3 v=no
why: session host and driver-sink logging of ids, names and typed text; the probe installs no sink and drives no host, and its own value masking is owed under needs 12, not this entity
@answer n=8 entity=integration-test-strategy#6.1 v=no
why: host_log and mcp_log select-list logging through the host and the role; the probe has no host, role or select list
@answer n=9 entity=integration-test-strategy#7.1 v=no
why: cli_commands' hosted select-list click through a held host; the probe is not hosted and opens no list
@answer n=10 entity=integration-test-strategy#8.1 v=no
why: stand_act_select's multiple-select checks through a held driver session; the probe reads no multiple select and drives no driver
@answer n=11 entity=integration-test-strategy#9.1 v=no
why: stand_act_popup's driver-held list checks; the probe opens no select list through the driver
@answer n=12 entity=integration-test-strategy#10.1 v=no
why: select_popup's list rows and harness.press behaviour under a hidden ancestor; the probe's key path is press_with on a Counter or CRUD handler with no hidden ancestor, so the rule is not reached
@answer n=13 entity=integration-test-strategy#11.1 v=no
why: painted select-list frames, highlights and colours; the probe reads no frame or colour (not-needed 8)
@answer n=14 entity=integration-test-strategy#12.1 v=no
why: blitz-dom stylesheet render-blocking on failed or empty loads; the probe loads no stylesheet and its poll path reads no render-blocking state
@answer n=15 entity=integration-test-strategy#13.1 v=no standing=S1
why: unit tests sit in #[cfg(test)] mod tests inside src, the placement convention S1 owes; this search reads only the apps' test placement
