@answer n=1 entity=visual-design-verification#8.6 v=no
why: the session adds no scrollbar-band or pointer-dispatch change and blitz-dom is untouched, so the engine click behaviour in that band still holds
@answer n=2 entity=visual-design-verification#8.7 v=yes mode=stale
why: the driver covered refusal for click now runs through the shared check_target in execute.rs, a reached file, so the cited refusal path moved although the refusal still holds
@answer n=3 entity=visual-design-verification#8.8 v=no
why: the blend-layer paint order is engine rendering that the session does not reach
@answer n=4 entity=visual-design-verification#8.9 v=no
why: the devtools overlay settings in blitz-traits are untouched by the driver verb change
