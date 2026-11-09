@answer n=1 entity=dependency-security#1.0 v=no
why: the cargo-deny audit config lives in root deny.toml and ci-leg.sh, neither reached, and Cargo.toml and Cargo.lock stay byte-identical
@answer n=2 entity=dependency-security#2.1 v=no
why: the audit's CI and fast-leg behaviour lives in ci-leg.sh and ci.yml, neither reached
@answer n=3 entity=dependency-security#3.1 v=no
why: the 2026-10-10 upstream dependency take lives in Cargo.toml and Cargo.lock, both byte-identical to before this session
@answer n=4 entity=dependency-security#3.2 v=no
why: the 2026-10-13 upstream dependency take lives in Cargo.toml and Cargo.lock, both byte-identical to before this session
@answer n=5 entity=dependency-security#3.3 v=no
why: the 2026-10-17 upstream dependency take lives in Cargo.toml and Cargo.lock, both byte-identical to before this session
@answer n=6 entity=dependency-security#3.4 v=no
why: the 2026-10-18 upstream cherry-pick left Cargo.toml, Cargo.lock and deny.toml unchanged, and this session leaves them unchanged too
@answer n=7 entity=dependency-security#3.5 v=no
why: the 2026-10-19 upstream cherry-picks left Cargo.toml, Cargo.lock and deny.toml unchanged, and this session leaves them unchanged too
@answer n=8 entity=dependency-security#3.6 v=no
why: the upstream sync's lock and manifest edits are historical and Cargo.toml, Cargo.lock and deny.toml are byte-identical under this session
@answer n=9 entity=dependency-security#3.7 v=no
why: the upstream sync's lock and manifest edits are historical and Cargo.toml, Cargo.lock and deny.toml are byte-identical under this session
@answer n=10 entity=dependency-security#3.8 v=no
why: the upstream sync's lock and manifest edits are historical and Cargo.toml, Cargo.lock and deny.toml are byte-identical under this session
@answer n=11 entity=dependency-security#4.1 v=yes mode=add-to
why: the coverage role reads caller-named kept-check files through Check::from_bytes, a new untrusted reader path for serde_json, which still writes nothing and gains no dependency
@answer n=12 entity=dependency-security#5.1 v=no
why: png's manifest and lock lines are byte-identical and the session takes no dependency, the reached files being source only
@answer n=13 entity=dependency-security#6.1 v=no
why: the color dependency of blitz-test-harness sits in a manifest not reached, Cargo.lock is byte-identical and blitz-tests gains no dependency
@answer n=14 entity=dependency-security#7.1 v=no
why: deny.toml's [licenses] table is unchanged and the session touches no licence input
@answer n=15 entity=dependency-security#7.2 v=no
why: the ci.yml free-disk step is unchanged and no CI workflow is reached
@answer n=16 entity=dependency-security#7.3 v=no
why: test_ci_workflows.py and the matrix_test job in ci.yml are untouched by the session
@answer n=17 entity=dependency-security#7.4 v=no
why: the embolden leg in ci-leg.sh is unchanged and the session adds no locked cargo leg
