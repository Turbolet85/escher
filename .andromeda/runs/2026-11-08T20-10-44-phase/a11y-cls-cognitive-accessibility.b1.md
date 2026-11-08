@answer n=1 entity=cognitive-accessibility#1.1 v=no
why: error and 404 page copy lives in apps/browser/assets, which the chunk's coverage read and replay test never reach
@answer n=2 entity=cognitive-accessibility#1.2 v=yes
why: the flight booker's Book button disables on an invalid date (enabled reads Some(false)) and aria-invalid sits on flight-start and flight-return-date in the accessibility tree; the disabled refusal (need 7) and the snapshot model (need 5) read those values, so a refusal or replay over that app must honour them
@answer n=3 entity=cognitive-accessibility#2.1 v=no
why: history relative time labels in apps/browser/src/browser_history.rs; no chunk file or check reaches them
@answer n=4 entity=cognitive-accessibility#2.2 v=no
why: tab tooltip in apps/browser; the chunk changes no tab strip and reads no tooltip
@answer n=5 entity=cognitive-accessibility#2.3 v=no
why: status bar hovered-link text in apps/browser; the chunk adds no status cue
@answer n=6 entity=cognitive-accessibility#2.4 v=no
why: seven_guis card description and tag text; the coverage result carries ids, component and event names only, and no chunk check reads card text
@answer n=7 entity=cognitive-accessibility#2.5 v=no
why: the load page's Load button and load-done paragraph carry no enabled-false value or named call the chunk's refusal or replay scope names, and the chunk's checks name no such page
