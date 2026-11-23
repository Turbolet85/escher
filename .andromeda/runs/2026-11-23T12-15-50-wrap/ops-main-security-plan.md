@row_add n=1 row=decision/blitz-tests-parley-dev-dependency area=dependency-security detector=D-security-deps
name: blitz-tests takes parley as a dev-dependency
rule: The integration-test crate takes the workspace's one registry version of parley as a dev-dependency, adding one dependency line to the lockfile and no package.
why: A reviewed pin: the version is the one the lockfile already held, and the audit stays green with deny.toml unchanged.
basis: fct-wgpw7fkq
because: The plan's expected amendment for Dependency Security names this dev-dependency, and no proposer raised it.
