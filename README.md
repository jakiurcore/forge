# Forge

A 300-day autonomous engineering laboratory.

Forge follows:

```text
Understand → Build → Measure → Productize
```

## Mission

Forge is a single, continuously evolving engineering system. Over 300 days it builds:

1. **Engineering Atlas** — a deep, practical systems knowledge base.
2. **Internet** — networking and distributed-system components.
3. **Developer Toolkit** — useful CLI tools for engineers.

The contribution graph is only a side effect. Every automated change must have genuine engineering value.

## Phases

```text
                         FORGE
                           │
             ┌─────────────┼─────────────┐
             │             │             │
          ATLAS         INTERNET       TOOLKIT
        Day 1–100      Day 101–200    Day 201–300
             │             │             │
          Learn          Build          Ship
             │             │             │
             └─────────────┼─────────────┘
                           │
                    Engineering Lab
```

## Quick start

```bash
# Build the workspace
cargo build --workspace

# Run the CLI
cargo run --bin forge -- status
```

Example output:

```text
Forge
────────────────────────────

Day:       0
Phase:     Foundation
Progress:  0/300

Next:
Engineering Atlas

Status:
✓ Repository
✓ Workspace
✓ CI
✓ Curriculum
✓ Automation
```

## Repository layout

```text
forge/
├── .github/workflows/   # CI and daily automation
├── curriculum/          # 300-day curriculum YAML
├── atlas/               # Phase I knowledge base
├── internet/            # Phase II implementations
├── toolkit/             # Phase III tools
├── labs/                # Experiments
├── experiments/         # Measured investigations
├── benchmarks/          # Performance data
├── docs/                # Project documentation
├── scripts/             # Automation scripts
├── crates/
│   ├── forge-core/      # Core engine
│   └── forge-cli/       # Command-line interface
├── tests/               # Integration tests
├── README.md
├── CONTRIBUTING.md
├── SECURITY.md
└── LICENSE
```

## Technology

- **Rust** — core implementations and CLI
- **C** — low-level/system demonstrations
- **Python** — experiments and data analysis
- **Shell** — Linux/system experiments

## License

MIT — see [LICENSE](LICENSE).
