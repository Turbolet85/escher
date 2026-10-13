
## 2026-10-13-upstream-sync-polish-and-ship — the default sheet arrives through the config
**Section:** §Surface: desktop-native → Tokens (platform-specific)
**Change:** the user-agent-sheet line: was "every document gets the blitz `DEFAULT_CSS` user-agent stylesheet" by `DioxusDocument::new`'s own call; now every document gets it once, through `DocumentConfig::default()` (`UserAgentStylesheet::default_sheet()`), the call removed by upstream #1189.
**Why:** upstream's mechanism, merged; the claim stands and only its path changed. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's).
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/
