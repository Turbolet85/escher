@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=44
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=28 class=stale
leaf-text: serving six tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — its usage line's `apps:` and the MCP `start` tool's `enum` listing all six
source: ctr-wpaelzph «order is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.»
replace-with: serving seven tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` · `order` — its usage line's `apps:` and the MCP `start` tool's `enum` listing all seven, `order` after `load`
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=28 class=stale
leaf-text: six apps — counter, flight booker, timer, CRUD, cells and the load page —
source: ctr-gtdbz4a4 «cold-agent.sh <verb> [task], run from the repository root, with verbs run, status, cleanup, logs and the one task stand: seven apps in one client session under a fixed prompt.»
replace-with: seven apps — counter, flight booker, timer, CRUD, cells, the load page and the order page —
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=27 class=stale
leaf-text: and the load page reads 0,
source: dec-emcqlk3f «At boot the counter, the flight booker, the timer, CRUD, the load page and the order page read zero visual-lint findings, in both layout modes.»
replace-with: and the load page and the order page read 0,
@leaf_edit n=4 leaf=.claude/docs/design-summary.md line=27 class=omission
leaf-text: under `main#main` at the pinned 800 × 600 Light viewport;
source: dec-rorvals5 «Submit returns an error for a quantity not a whole number from 10 to 20, a good Submit logs one info record order placed, and Export panics with export is not built yet.»
replace-with: under `main#main` at the pinned 800 × 600 Light viewport; the order page's controls are built to cause, with stable element ids `order-quantity` · `order-submit` · `order-export` · `order-status` · `order-failed` · `order-retry`: Submit returns an error for a quantity not a whole number from 10 to 20 (`order-failed` showing Order failed, `order-retry` present only once Submit fails), a good Submit logs one info record order placed and `order-status` reads Order placed, and Export panics with export is not built yet;
