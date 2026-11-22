# Security policy

## Supported versions

Only the latest release receives security fixes.

## Reporting a vulnerability

**Please do not open a public issue.** Report vulnerabilities privately through
[GitHub's private vulnerability reporting](https://github.com/Yatsuuw/discord_presence_status/security/advisories/new).

Include a description of the problem, the steps to reproduce it and the
affected version. You will get an answer as soon as possible; once a fix is
released, the advisory is published with credit to you if you wish.

## Scope

Discord Presence Status only talks to the Discord desktop application running
on the same computer (through its local IPC pipe) and only reads and writes
files inside its activities folder. Reports about anything that breaks these
boundaries are especially welcome.
