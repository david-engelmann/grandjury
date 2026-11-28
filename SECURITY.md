# Security Policy

## Scope and Trust Model

`grandjury` runs offline as a local command line tool and Rust library. It opens no network sockets, phones home nowhere, and reads only the local file paths handed to it on the command line without mutating them. Untrusted JSON records and text inputs pass through whole-input schema validators and single-line fences (`\n`, `\r`, and unpaired UTF-16 surrogates) before any report line renders, preventing crafted strings from forging extra report lines or summary counts. Operators displaying untrusted model strings directly in a terminal should still pipe output through a pager or sanitizer.

## Supported Versions

Security fixes land on the latest `0.1.x` release series and on `main`.

## Reporting a Vulnerability

Please report suspected vulnerabilities privately through GitHub Security Advisories at <https://github.com/david-engelmann/grandjury/security/advisories/new> rather than opening a public issue. Include a minimal synthetic input demonstrating the parser, validator, or line-break-fencing defect along with the commit SHA or release tag tested. Every report receives an initial triage response within seven days.
