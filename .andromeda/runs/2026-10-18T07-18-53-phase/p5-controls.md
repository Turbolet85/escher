# P5 controls — 2026-10-18-one-virtual-clock

Gate entry 8 (the wall-clock census), fired as the entry through `gate.py run --plan escher-0.2.0/chunks/2026-10-18-one-virtual-clock/plan.md --entry 8 --project-root $TMPDIR/{scratchpad}/census-clone`, over a `git clone --shared` of the project at HEAD `f45bb570a`. Recorded 2026-10-18T07:51Z.

- **clone, unmodified** (must read as the real tree): `8 probe red · last line '0' ✗ (last: '7') · exit 0` — the same as the real tree's baseline. The 7 hits are `events/pointer.rs:267,407,717`, `scrolling.rs:515,871,896` and `document.rs:1879`, the reads steps 3–5 move.
- **clone, must-pass input** (every `Instant::now()` / `SystemTime::now()` in `events/pointer.rs`, `scrolling.rs` and `document.rs` replaced by a call, and a `packages/blitz-dom/src/clock.rs` holding one `web_time::Instant::now()`): `8 probe green · exit 0` — the read inside `clock.rs` is excluded and nothing else is counted.
- **must-fail side**: the unmodified reading above (7 reads outside `clock.rs` → red).

The control's inputs live only in the scratch clone; only these verdicts are recorded here.
