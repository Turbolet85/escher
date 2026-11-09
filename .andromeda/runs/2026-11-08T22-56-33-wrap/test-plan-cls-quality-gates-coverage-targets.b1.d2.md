@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the 2026-10-05 first line-coverage reading is a dated measurement that the session does not move
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=no
why: the session holds the three coverage floors and does not re-read the per-crate figures, so the entity's floor claims still hold
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: the mutation_buffer.rs reading is a dated measurement of a file the session does not reach
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes mode=stale
why: stand_handler_coverage joins the stand_*.rs glob the stand-repeat leg runs, moving the 53 file count to 54 (task item 16)
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the scoped mutation runs cover timer.rs, mcp/framing.rs and mutation_writer.rs, none of which the session reaches, and the tier is unchanged
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=no
why: its complexity readings are dated 2026-10-26 measurements of functions the session does not name, and the task records no complexity move
