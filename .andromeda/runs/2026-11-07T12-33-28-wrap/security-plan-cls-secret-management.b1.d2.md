@answer n=1 entity=secret-management#1.1 v=no
why: the production CI secret reads sit in publish-browser.yml, wpt.yml and wpt-post-results.yml, none of which the session reached and no CI step moved
@answer n=2 entity=secret-management#1.2 v=no
why: the dev-host and cold-agent credential paths sit in the driver and the pipe, which the session did not reach and whose login path is unchanged
@answer n=3 entity=secret-management#2.1 v=no
why: the change adds no password, token, secret, cookie or authorization term to the swept files, so the absence still holds on the changed files
@answer n=4 entity=secret-management#2.2 v=no
why: the change adds no env read or credential term to the swept files, the new module reads no environment variable, so the absence still holds
@answer n=5 entity=secret-management#2.3 v=no
why: the change adds no api key, password, secret, token or authorization term to the swept files, so the absence still holds on the changed files
@answer n=6 entity=secret-management#2.4 v=no
why: the change adds no env read, secret, token, authorization or cookie term to the swept files, so the absence still holds on the changed files
@answer n=7 entity=secret-management#2.5 v=no
why: the change adds no env read or api key term to the swept files, the new module reads no environment variable, so the absence still holds
@answer n=8 entity=secret-management#2.6 v=no
why: the change adds no env read, secret or token term to the swept files, so the absence still holds on the changed files
@answer n=9 entity=secret-management#2.7 v=no
why: the change adds no password, secret, token, authorization or x-api-key term to the swept files, so the absence still holds on the changed files
@answer n=10 entity=secret-management#2.8 v=no
why: the change adds no env read or credential term to the swept files, the new module reads no environment variable, so the absence still holds
@answer n=11 entity=secret-management#2.9 v=no
why: the change adds no env read to the swept files, the new module reads no environment variable, so the absence still holds on the changed files
@answer n=12 entity=secret-management#2.10 v=no
why: the change adds no token, password, secret, authorization, cookie or env read term to the swept files, so the absence still holds on the changed files
@answer n=13 entity=secret-management#2.11 v=no
why: the change adds no token, password, secret, api key, authorization or cookie term to the swept files, so the absence still holds on the changed files
@answer n=14 entity=secret-management#2.12 v=no
why: the change adds no authorization, cookie, x-api-key, password, secret or token term to the swept files, so the absence still holds on the changed files
@answer n=15 entity=secret-management#2.13 v=no
why: the pinned capability files and the README sit outside the session's reach, and the change adds no home path, api key, bearer or e-mail domain
