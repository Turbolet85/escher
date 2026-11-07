@answer n=1 entity=test-scope-summary#41.1 v=no
why: the session changes no CI leg, script or stand-repeat membership, and ci-leg.sh and test_ci_workflows.py are outside its reached files
@answer n=2 entity=test-scope-summary#42.1 v=no
why: the session adds no stand_cells_served or stand_load_page test and leaves the stand-repeat leg and the probe's exclusion from it unchanged
@answer n=3 entity=test-scope-summary#42.2 v=no
why: the flight booker's select and the host_log tests live in seven_guis, which no reached file touches
@answer n=4 entity=test-scope-summary#42.3 v=no
why: the stand module's unit tests and their stand:: count of 5 live in examples/seven_guis, which the session did not reach
@answer n=5 entity=test-scope-summary#42.4 v=no
why: escher-driver's lint and its select-box skip are untouched, and no reached file holds the lint
@answer n=6 entity=test-scope-summary#43.1 v=no
why: the seven_guis cli, host and mcp log tests are not reached by the session's dioxus-native-dom or probe changes
@answer n=7 entity=test-scope-summary#44.1 v=no
why: stand_cells_served's keyboard walk and the driver's twelve key names are untouched, the session changing only the dioxus-native-dom render path and the probe
@answer n=8 entity=test-scope-summary#45.1 v=no
why: the WPT runner and its unit test counts sit outside every reached file
