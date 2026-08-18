# Security Policy

## Reporting vulnerabilities

If you discover a security issue in Forge, please report it by opening a private security advisory on GitHub or by emailing the maintainers directly.

Do not open public issues for undisclosed security vulnerabilities.

## Secrets and credentials

- Never commit API keys, passwords, tokens, or private keys.
- Never expose credentials in logs, workflow output, or documentation.
- The `.forge/state.toml` file and `logs/` directory are gitignored to avoid accidental leaks.

## Cryptography

Forge uses established, well-maintained cryptographic libraries. Do not implement production cryptography from scratch.

## Automation safety

- Automated workflows never rewrite Git history or force-push.
- Automated workflows never delete unrelated work.
- Failing tests or validation stop automatic publishing.
