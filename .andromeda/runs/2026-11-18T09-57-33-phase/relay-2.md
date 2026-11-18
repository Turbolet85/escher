The operator's answers to the P4 forks of 2026-11-18-served-error-page, given by question dialog:

Q: Served error page — what should its controls ask of a cold agent?
A: Goal needs caused (Recommended) — a small order form: a quantity field, Submit and Export; Submit with a bad quantity returns an Err whose fixed text names the rule, the screen showing only a generic failure; a good Submit logs at info and shows "Order placed" (the done state); Export panics with a fixed message.

Q: Which error boundary should take the page's Err?
A: App's own, with Retry (Recommended) — an ErrorBoundary inside the page shows a generic fallback and a Retry that clears it; the app stays on screen; caused reads boundary own.

Q: How should the cold-agent verdict count caused?
A: By kind, each required (Recommended) — caused_by_kind, an object of the three closed words (panic, error, log) to the count of accepted acting answers carrying an item of that kind, read from the transcript; capability-unused fails a run where any kind reads 0.
