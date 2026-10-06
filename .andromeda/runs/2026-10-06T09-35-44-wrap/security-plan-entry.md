
## 2026-10-06-stable-element-ids — the HTML `id` gains the stable-element-id reader
**Section:** §Input Validation (Markup attributes)
**Change:** a new `Markup attributes | id` row: besides CSS matching and `getElementById`, a Dioxus document's `element_id` / `element_ids` read the HTML `id` as an author key only when non-empty, `/`-free and the first in document pre-order; an empty, `/`-bearing or later-duplicate value gives no key and the element reads its `/`-bearing path, so no `id` makes two ids equal; a non-element, stale or detached node reads `None`; no path panics; the id is computed on demand, written nowhere, and carries no engine id or pointer.
**Why:** the chunk added an in-process reader of an already-admitted attribute; no new input class, crossing or write — not a boundary widening. The rule is the reader's validation.
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/
