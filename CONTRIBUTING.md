# Contributing to Forge

Thank you for your interest. Forge is a 300-day autonomous engineering laboratory, so contributions should extend the knowledge base, the implementation stack, or the developer toolkit in a coherent way.

## Development workflow

1. Open or review an issue describing the engineering goal.
2. Make the smallest robust change that achieves the goal.
3. Add or update tests.
4. Run `./scripts/validate.sh` before committing.
5. Write clear commit messages that describe real work.

## Commit quality

- Every commit must represent actual engineering work.
- Do not create meaningless changes to increase commit counts.
- Prefer small, reviewable commits.
- Never bypass failing tests just to complete a day.
- Never fabricate benchmark or experiment results.

## Curriculum changes

The curriculum lives in `curriculum/` as structured YAML. Do not silently modify the curriculum. Curriculum milestones should be reviewed deliberately.

## Security

See [SECURITY.md](SECURITY.md). Never commit secrets, API keys, or credentials.

## Code style

Rust code is formatted with `cargo fmt` and linted with `cargo clippy --all-targets --all-features`.
