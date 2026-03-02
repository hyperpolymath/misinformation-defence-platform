# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.x.x   | :white_check_mark: |

## Reporting a Vulnerability

**DO NOT** open a public issue for security vulnerabilities.

Instead, email security reports to: **security@hyperpolymath.org**

Include:

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Suggested fix (if you have one)

We will respond within 48 hours and work with you to understand and address the issue.

## Security Considerations

Algorithm Shield operates with sensitive permissions (content script injection, storage, active tab).

### What We Do

- ✅ All data stays local (chrome.storage.local)
- ✅ No external network requests (except user-initiated membrane breaches)
- ✅ Rate limiting on all automated actions
- ✅ User consent required for sensitive operations
- ✅ Transparent activity logging
- ✅ Source code is public and auditable

### What We Don't Do

- ❌ Never exfiltrate user data
- ❌ Never track users
- ❌ Never communicate with external servers
- ❌ Never modify pages without user awareness
- ❌ Never execute arbitrary code from external sources

## Known Limitations

1. **Platform DOM Changes**: If a platform changes their DOM structure, extraction may break
2. **Rate Limits**: Extensions can be detected by platform-side rate limiting
3. **WASM Security**: WASM module is compiled from Rust, but could theoretically be exploited

## Responsible Disclosure

If you discover a security issue:

1. Email us (don't open a public issue)
2. Wait for our response (48 hours)
3. Work with us on a fix
4. We'll credit you in the release notes (if you want)
5. Coordinated public disclosure after fix is released
