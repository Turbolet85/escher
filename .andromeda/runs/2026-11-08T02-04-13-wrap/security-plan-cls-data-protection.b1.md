@answer n=1 entity=data-protection#1.1 v=yes mode=add-to
why: the kept check may now hold a graph step and its answer (ids, component and state names, event names, flag words), a reading the at-rest list does not carry yet.
@answer n=2 entity=data-protection#2.1 v=yes mode=stale
why: the two committed checks were re-recorded (counter-cli.json +4/-4, counter-mcp.json +6/-6), so the readings the entity describes have moved, though the strict replay still passes.
