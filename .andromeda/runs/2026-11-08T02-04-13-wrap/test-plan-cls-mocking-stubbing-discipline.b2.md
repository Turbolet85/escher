@answer n=1 entity=mocking-stubbing-discipline#1.26 v=no
why: the WPT diff script it cites sits in the unreached WPT pipeline, which the session did not touch
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: its blitz-dom hover tests sit in a byte-unchanged crate the session did not reach
@answer n=3 entity=mocking-stubbing-discipline#3.1 v=yes mode=stale
why: test_cold_agent.py is reached whole under the .github/scripts/*.py glob and gains the cold-agent fake server, so the mock grep over that glob must be re-read
@answer n=4 entity=mocking-stubbing-discipline#3.2 v=no
why: the apps tree is unreached and no mocking library or dev-dependency was added
@answer n=5 entity=mocking-stubbing-discipline#3.3 v=yes mode=stale
why: the new cold-agent fake server matches this search's fake term, so the no-mock-or-stub reading can move
@answer n=6 entity=mocking-stubbing-discipline#3.4 v=no
why: the session adds no mock or fixture term to this search's scope and no mocking library
@answer n=7 entity=mocking-stubbing-discipline#3.5 v=no
why: the session adds no mock or fixture term to this search's scope and no mocking library
@answer n=8 entity=mocking-stubbing-discipline#3.6 v=yes mode=stale
why: the new cold-agent fake server matches this no-match search's fake term, so the empty result can move
@answer n=9 entity=mocking-stubbing-discipline#3.7 v=yes mode=stale
why: the new cold-agent fake server matches this search's fake term, so the no-mock-or-stub reading can move
@answer n=10 entity=mocking-stubbing-discipline#3.8 v=yes mode=stale
why: the new cold-agent fake server matches this search's fake term, so the no-mock-or-stub reading can move
