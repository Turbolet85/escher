a11y-plan: none · ops-a11y-plan.md — reason: no interactive UI element, violation schema or log site added; the a11y leg unmoved 14 · 115 · 0
architecture: add 5 · set 5 · retire 1 · none 0 · ops-architecture.md
design-system: add 2 · set 0 · retire 0 · none 4 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 0 · ops-layout-templates.md
obs-plan: none · ops-obs-plan.md — reason: the take adds no log site, span, field, metric or telemetry dependency
security-plan: add 2 · set 1 · retire 0 · none 1 · ops-security-plan.md
test-plan: add 0 · set 1 · retire 0 · none 1 · ops-test-plan.md

ops-architecture.md
decision/programmatic-scroll-one-box · apply — check 1: routine; the scroll state's signed min/max clamp is in the report's Symbols / APIs (fct-w7kvqm7c)
contract/scroll-hit/hit-clip · apply — check 1: routine; the clip stop's content-box frame is step 2 of the report, an expected amendment (hit-stops-at-paint-clip / scroll-hit)
decision/client-rect-own-box · apply — check 1: routine; the kept-out scroll-shifted second body and self match are the report's Reverted / negative API facts
contract/scroll-hit/scroll-into-view · apply — check 1: routine; scroll_into_view's zero-range skip is in the report's Symbols / APIs, the expected contract/scroll-hit amendment
decision/scenarios-both-layout-modes · reject — check 2: the fact's home is test-plan decision/layout-tests-in-both-layout-modes (its except already names the sync-taken one-mode targets; test-plan's set carries this take's)
decision/deferred-upstream-dependency-bumps · apply — check 1: routine; the Taffy deferral ended at a3997d55 (report Dependencies), the expected deferred-bumps amendment; its Stylo and Dioxus clauses go to the heir
decision/reversed-scroll-origins · apply — check 1: routine; the ratified reversed-origin behaviour change (inputs#I2), the expected upstream-take-engine-changes amendment as a sibling row
decision/unstyled-elements-hit-tested · apply — check 1: routine; upstream ebacd68bd and the is_scroll_container seam are in the report's Symbols / APIs
decision/scroll-size-readings · apply — check 1: routine; scroll_width/height's CSSOM size and MountedData kept on Taffy's extent are both in the report
contract/nodes-widgets/try-layout-data · apply — check 1: routine; the one widened public read of blitz-dom, Symbols / APIs
decision/deferred-stylo-dioxus-bumps · apply — check 1: routine; the retired row's heir, keeping its standing Stylo and Dioxus deferrals

ops-design-system.md
decision/reversed-boxes-scroll-to-negative-offsets · reject — check 2: the fact's home is architecture decision/reversed-scroll-origins
decision/scroll-size-is-cssom-scroll-size · reject — check 2: the fact's home is architecture decision/scroll-size-readings

ops-layout-templates.md
decision/reversed-box-negative-scroll-offsets · reject — check 2: the fact's home is architecture decision/reversed-scroll-origins

ops-security-plan.md
decision/engine-dependency-pins · apply — check 1: routine; the Taffy git rev e90751fd → a3997d55, the expected engine-dependency-pins amendment
decision/reversed-scroll-origins · reject — check 2: the fact's home is architecture decision/reversed-scroll-origins
decision/unstyled-node-hit-and-geometry-no-panic · reject — check 2: the fact's home is architecture decision/unstyled-elements-hit-tested

ops-test-plan.md
decision/layout-tests-in-both-layout-modes · reject — check 7: not_master type 5 — the set only names one set of upstream tests the standing exception already covers, an inventory item the test code and the sidecar hold; the standing row stays true
