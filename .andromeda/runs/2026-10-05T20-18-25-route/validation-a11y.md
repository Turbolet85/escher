# A11y validation — route draft

## Insert
- Between `Supply-chain and coverage legs` and `Headless stand`: **"A11y CI leg reached — existing AccessKit role and hidden-element tests run as a named a11y leg on fork CI"** (epoch: `Epoch 1`)
  Reason: The plan's Bootstrap item a11y-ci-gate-wire (a11y-plan §3 Bootstrap phases, §9 says CI a11y checks are absent) needs a basic CI leg that runs in Foundation once fork CI exists, and the draft currently adds the leg only in Polish.
- Between `Headless screenshot` and `Stand a11y assertions`: **"Stand contrast harness — text/background pair of every stand control measured against SC 1.4.3, per-pair result recorded"** (epoch: `Epoch 6`)
  Reason: The plan's Bootstrap item contrast-verification-harness-setup (a11y-plan §3 Bootstrap phases, §6 says contrast checks are absent) has no chunk of its own, and it must come before the full a11y CI gate.
- Between `Stand contrast harness` and `Stand a11y assertions`: **"Stand keyboard harness — Tab/Shift+Tab and activation keys dispatched headlessly on the stand, focused node read back per step"** (epoch: `Epoch 6`)
  Reason: a11y-plan §3 (Keyboard test harness) and §5 (Test harness pattern) record keyboard-event and Tab-order tests as absent, so the SC 2.1.1 / 2.4.3 assertions have no harness to run on.

## Rewrite
- `Stand a11y assertions`: "keyboard reach, focus order and text contrast on the stand, a11y CI leg" → "SC 2.1.1 keyboard reach, SC 2.4.3 focus order, SC 1.4.3 text contrast on every stand control; a11y CI leg gating merges"
  Reason: The verification chunk should name the Tier 0 SC IDs from a11y-plan §1 and act as the full blocking gate, since the harness setup now has its own chunks above.
