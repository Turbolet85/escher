@answer n=1 entity=standard-contracts#30.3 v=yes mode=stale
why: NodeState.clickable joins NODE_FIELDS, so the snapshot carries fourteen node keys, not thirteen
@answer n=2 entity=standard-contracts#30.4 v=yes mode=stale
why: snapshot.rs is reached whole and gains NodeState.clickable, so the cited lines 257-272 and 276-277 can have moved
@answer n=3 entity=standard-contracts#30.5 v=yes mode=stale
why: dioxus_document.rs is reached whole and its event path gains the registry dispatch, so the cited lines 33-47 can have moved
@answer n=4 entity=standard-contracts#30.6 v=yes mode=stale
why: dioxus_document.rs is reached whole and gains the graph reader and the dispatch change, so the cited lines 255-259 can have moved although flush_queued_mounted_events is unchanged
@answer n=5 entity=standard-contracts#31.1 v=yes mode=stale
why: graph joins VERBS as the ninth instance verb and the usage line gains it after lint, so the twelve verbs listed are stale
@answer n=6 entity=standard-contracts#31.2 v=no
why: the multi_doc example is not among the session's reached files and its timing, layout and env vars are unchanged
@answer n=7 entity=standard-contracts#32.1 v=no
why: the browser app is untouched and none of its files are among the session's reached files
@answer n=8 entity=standard-contracts#33.1 v=no
why: the WPT runner is untouched and none of its files are among the session's reached files
@answer n=9 entity=standard-contracts#34.1 v=yes mode=stale
why: the cold-agent pipe's escher server now carries thirteen tools with graph in the VERBS set, so the cited twelve tools are stale
@answer n=10 entity=standard-contracts#34.2 v=no
why: the embolden leg of ci-leg.sh gains no job or leg and the fast leg is unchanged, with ci-leg.sh not among the reached files
@answer n=11 entity=standard-contracts#34.3 v=no
why: the stand-repeat job gains no job or leg, so the disk-freeing pin in test_ci_workflows.py still holds
@answer n=12 entity=standard-contracts#34.4 v=no
why: the font-embolden feature and the select_popup_painted witness are unchanged by the session
@answer n=13 entity=standard-contracts#35.1 v=yes mode=stale
why: scripts/cold-agent.sh is reached whole and its VERBS set gains graph, so the cited lines 19-31, 178-271 and 428-506 can have moved
