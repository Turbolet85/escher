@account n=1 entity=threat-model-summary#3.1 v=proposed
why: its socket host role still holds (no socket surface moved, the boot closure and command_line call unchanged), but the binary's new coverage role reading caller-named kept checks is F3, a new vector beside it (E5 names this entity's site)
@account n=2 entity=authentication-authorization#1.2 v=not_touched
why: a past slice's absence search; the report adds no auth, login, token or credential code, the coverage role carrying no authentication, so the absence still holds
@account n=3 entity=authentication-authorization#1.4 v=not_touched
why: a past slice's absence search; the report adds no authentication, authorization or credential code, so the absence still holds
@account n=4 entity=authentication-authorization#1.5 v=not_touched
why: a past slice's absence search; the report adds no auth, permission or login code, so the absence still holds
