old: the port's shape as read, not run, at the probe** [inferred]: «upstream's
new: the port's shape as read, not run, at the probe** (verified at P3 — research, Patterns detected, «The equality the port needs»): «upstream's
old:   that nothing in the render reads the document's state mid-render — is P3's to close.
new:   that user code the render runs now sees the document as the last applied render left it, where any borrow from it panicked — holds at HEAD: the 16 methods return nothing to the vdom, their arguments copy or clone (`Template: Copy`, `AttributeValue: Clone`), today's write is one `MutationWriter` per render (research, Patterns detected).
old: - **The hazard's mechanism, as the probe measured it** [inferred]: `DioxusDocument::poll` holds `inner.borrow_mut()`
new: - **The hazard's mechanism, as the probe measured it** (spot-checked at HEAD, P3 — research, «The hazard's frame, re-derived»): `DioxusDocument::poll` holds `inner.borrow_mut()`
old: kept byte-identical to upstream at `434e0661`** where the port allows [inferred]; a change
new: kept byte-identical to upstream at `434e0661`** — the port allows it, the buffer living on `DioxusDocument` (verified at P3 — research, Patterns detected); a change
old: `lib.rs`'s module list, `mutation_writer.rs`'s `DioxusState` if the
new: `lib.rs`'s module list. [premise-corrected: the buffer lives on `DioxusDocument` as a `pub(crate)` field, so `mutation_writer.rs`'s `DioxusState` is untouched — research, Patterns detected] Not
old:   buffer lives on it as upstream's does [inferred].
new:   touched: `mutation_writer.rs`.
