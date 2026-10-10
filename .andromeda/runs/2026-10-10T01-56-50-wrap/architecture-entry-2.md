
## 2026-10-10-upstream-sync-agent-surfaces — `text-transform-icu` in four default lists, `writing-mode` opt-in, the `text-indent` sentence marked not established
**Section:** §Established Decisions → [Default features], [Document defaults], [Unsupported features] · §Occupied Resources → Names
**Change:**
- [Default features]: blitz-dom's default list was svg, woff, accessibility, system-fonts, file-input, custom-widget; it now ends with `text-transform-icu`. blitz's was `net`, `accessibility`, `tracing`; dioxus-native's and dioxus-native-dom's lists likewise gain `text-transform-icu`. A new sentence: `writing-mode`, CSS vertical writing modes, is in no default list — `stylo_taffy`'s is an empty feature, blitz-dom's forwards to it, dioxus-native-dom's and blitz's forward to blitz-dom's, and dioxus-native's to blitz-dom's and dioxus-native-dom's.
- [Document defaults]: the Stylo prefs list gains `layout.writing-mode.enabled`, set only under the `writing-mode` feature.
- [Unsupported features]: was "`text-indent` `hanging`/`each-line` do not work because their parsing is cfg'd out in Stylo"; now the two flags are read from the style and handed to Parley, and that they do not work is `recorded, not established`.
- Names: blitz's feature names were `net`, `accessibility`, `tracing`, `scrollbars`; now also `text-transform-icu` and `writing-mode`.
**Why:** upstream's features, arrived by the merge. The `text-indent` sentence quoted an upstream code comment that the merge removed; the code passes both flags as before, stylo is unmoved at 0.22.0, and the chunk took no reading of either flag — the operator, 2026-10-10, through the question dialog of this wrap, kept the sentence with its status stated instead of dropping it. A wrap that later measures either flag restates it as measured.
**Kept:** Names lists only the feature names it already listed crates for — blitz-dom's, stylo_taffy's and the Dioxus crates' feature names were never in it and are not added. The by-runner reading of `writing-mode` is test-plan §9's.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
