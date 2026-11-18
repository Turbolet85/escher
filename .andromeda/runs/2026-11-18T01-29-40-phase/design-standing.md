# design · standing conventions

- S1 · each new stand check moves time only through the driver's advance on the one virtual clock, with settle and load waits moving no clock and driver scroll instant · motion-calibrated-to-expression-level-unrecorded · motion
  decision/animation-driven-by-virtual-clock · Animation driven by the virtual clock — The test harness drives animation from the VirtualClock its documents read, and settle reads animation time only through that clock, never advancing it.
  decision/driver-scroll-instant · Driver scroll is instant — The driver's scroll is instant in every scrolling box and in the viewport, made inside one settled step. · why: Ratified.
  decision/load-wait-moves-no-clock · Load wait moves no clock — While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
  decision/settle-does-not-wait-on-animation · Settle does not wait on animation — A harness settle answers every member of the document's animating set the same way: none holds it open, none is waited on or advanced; settle returns Settled with animating reading the document's animating flag.
- S2 · each new check that reads painted text reads it under the test engine's font setup, with a missing font answered by the screenshot's fonts-unavailable refusal, never a screenshot with text skipped · typography
