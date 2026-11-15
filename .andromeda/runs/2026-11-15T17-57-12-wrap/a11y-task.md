@task_prose part=session
The session added a private unit-test module to the browser app's document loader, proving the document config it hands a page carries the page URL, the passed net provider and the navigation provider.
Two of the new tests read the file-URL navigation gate in both directions through that config: a remote page's navigation to a file URL loads nothing, a file page's navigation to a file URL loads.
It added a justification comment above the two bare clippy expect_used allows left in the browser app, so every browser expect_used allow now stands beside its reason.
Measured: the browser crate's unit tests 35 to 38; bare expect_used allows in the browser app 2 to 0; the document config builder's mutants 11 missed to 7 missed, 4 caught.
No public API, dependency, feature, manifest, schema, CI gate or spec master changed; nothing was retired.
@task_file n=1 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds the document config (base URL, net provider, navigation provider) a loaded page runs with, and now its own unit tests of that hand-off
@task_file n=2 path=apps/browser/src/favicon.rs
what: the browser app's favicon fetch and its test module, whose expect_used allow now stands below a justification comment
@task_file n=3 path=apps/browser/src/tab.rs
what: the browser app's tab and its tab web view, whose expect_used allow now stands below a two-line justification comment
@task_item n=1 part=touches
The convention that every expect_used or unwrap allow carries its justification beside it: its one recorded exception, the browser favicon test module's bare allow, no longer holds, so a row stating that exception is stale.
@task_item n=2 part=touches
The rule that file URLs load only for file documents: its browser-navigation clause gains two guards, remote_page_config_navigation_to_file_url_loads_nothing and file_page_config_navigation_to_file_url_loads, so a row whose guard cell names this rule's tests may add them.
@task_item n=3 part=touches
The browser's document loading hand-off (base URL, net provider, navigation provider in the document config) is now guarded by config_carries_the_page_url_and_the_passed_net_provider; a row over the browser's document config or its providers may name it as its guard.
@task_item n=4 part=touches
A row holding the browser crate's unit-test count would move from 35 to 38; a row listing the browser's tested source files would gain the document loader.
@task_item n=5 part=touches
Indirect: any rule over the browser tab's web view or the browser favicon fetch reaches only their comment lines; their behaviour is unchanged.
@task_item n=1 part=untouched
The accessibility tree, its roles, states, hidden-content and ARIA-property contracts: no tree building, role mapping or ARIA read changed.
@task_item n=2 part=untouched
Keyboard navigation, focus, activation, select lists, range inputs and the seven_guis grid and list contracts: no input or focus behaviour changed.
@task_item n=3 part=untouched
Visual design verification, contrast thresholds, the contrast lint and its finding fields: no paint, colour or lint code changed.
@task_item n=4 part=untouched
Screen-reader support, the snapshot, the driver's actions, refusals and screenshots, the MCP results and the UI graph: no reader, driver or wire form changed.
@task_item n=5 part=untouched
The accessibility feature defaults, the platform adapter choice and the browser's accessibility feature being off by default: no manifest, feature or dependency moved.
@task_item n=6 part=untouched
The a11y CI leg, the slow tier, the stand checks repeated three rounds and accessibility checks gating CI: no gate, leg or CI surface changed.
@task_item n=7 part=untouched
Cognitive accessibility and the stand's shared tab-order table: no stand page or expectation table changed.
