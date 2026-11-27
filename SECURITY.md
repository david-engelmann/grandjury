# Security Policy

## Scope and Trust Model

`grandjury` runs offline as a local command line tool and Rust library. It opens no network sockets, phones home nowhere, and reads only the local file paths handed to it on the command line without mutating them. Untrusted JSON records and text inputs pass through whole-input schema validators and line-break fences before any report line renders, preventing crafted strings from forging report headers or summary counts.

## Supported Versions

Security fixes land on the latest `0.1.x` release series and on `main`.

## Reporting a Vulnerability

Please report suspected vulnerabilities privately through GitHub Security Advisories at <https://github.com/david-engelmann/grandjury/security/advisories/new> rather than opening a public issue. Include a minimal synthetic input demonstrating the parser, validator, or terminal-fencing defect along with the commit SHA or release tag tested. Every report receives an initial triage response within seven days.
