@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=64
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=6 class=omission
leaf-text: with a record directory it keeps one owner-only `check-{n}.json` per explored path.
source: ctr-5uc2ynp6 «contract/driver-session/layout · escher-driver session · Session::layout — An in-process read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId; it resolves its id»
replace-with: with a record directory it keeps one owner-only `check-{n}.json` per explored path. Since 2026-11-16-layout-explained the **layout read** (`Session::layout`, in process only — no verb, MCP tool, session socket call or kept check reaches it) answers a `LayoutExplanation` of String ids, closed words and numbers with no `NodeId`, resolving its id as the run path does (refusing `stale` or `not-found`), recording the screen's ids, and running no step, pump, tick or settle and opening no span.
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=49 class=omission
leaf-text: The command line's `coverage` role adds no span, each replayed step leaving its own `command` span through `Session::run`.
source: dec-rwdh7s5m «decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read»
replace-with: The command line's `coverage` role adds no span, each replayed step leaving its own `command` span through `Session::run`. An in-process read outside the run path opens no span: since 2026-11-16-layout-explained `Session::layout` opens none, and a layout answer reaches no log line at any level.
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=37 class=omission
leaf-text: every id it lists recorded, with no public reader
source: ctr-5uc2ynp6 «contract/driver-session/layout · escher-driver session · Session::layout — An in-process read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId; it resolves its id»
replace-with: every id it lists recorded, and, since 2026-11-16-layout-explained, the screen's ids a `Session::layout` read records, with no public reader
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=57 class=stale
leaf-text: `Role` and `BoundingRect` are not re-exported by dioxus-native-dom
source: ctr-ansro24a «contract/dioxus-native-dom-api/layout-reason-types · dioxus-native-dom public API · layout reason types and BoundingRect — Public and re-exported beside NodeId: the ten layout reason types of blitz-dom and BoundingRect.»
replace-with: `Role` is not re-exported by dioxus-native-dom (`BoundingRect` is, since 2026-11-16-layout-explained)
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=62 class=omission
leaf-text: `stand_session_quiet`'s host installs no log sink, so it proves nothing about a host that does
source: ctr-afdxnw5f «contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and»
replace-with: Since 2026-11-16-layout-explained a `Session::layout` read is checked to change nothing — no change pending, the snapshot, focus, diff and clock unchanged, an open list staying open and delivered Timer ticks unapplied — in both layout modes. `stand_session_quiet`'s host installs no log sink, so it proves nothing about a host that does
