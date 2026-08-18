# GitHub Actions Setup

Forge uses GitHub Actions for CI and the daily autonomous run. Any commits made by automation must be correctly attributed to a GitHub account.

## Required repository variables

Before the daily workflow can commit, set these repository variables in GitHub:

| Variable | Example value | How to find yours |
|---|---|---|
| `FORGE_COMMITTER_NAME` | `Jakiur` | Your GitHub display name or the name you want on commits. |
| `FORGE_COMMITTER_EMAIL` | `243452307+jakiurcore@users.noreply.github.com` | Your GitHub noreply email. Go to **Settings > Emails** and copy the private noreply address. It usually looks like `<id>+<username>@users.noreply.github.com`. |

Set them at: **Repository Settings > Secrets and variables > Actions > Variables > New repository variable**.

## Why noreply?

Using a GitHub-provided noreply email keeps your personal email private while ensuring GitHub attributes the commit to your account and counts it on your contribution graph.

## Verification

After the next daily run, check a commit on GitHub. The commit header should show your name, and hovering over it should link to your profile.
