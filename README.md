# infinity-setup

A Rust CLI that orchestrates annotated bash scripts to automate the setup and
configuration of Linux machines (desktop and server), across distributions.

## Concept

Instead of a single large bash script for a whole machine, the project uses
many small bash scripts, each with a single responsibility. Scripts carry
prefixed annotations (`# @key: value`) that tell the program what to display,
which module handles them, and which distribution they target.

See `docs/` for conventions, architecture decisions, testing policy, and git
workflow.

## Usage

```
infinity-setup list              # list available annotated scripts
infinity-setup run <script>      # run one script via its module
```

## Development

- `cargo build`
- `cargo test`

Read `AGENTS.md` before contributing.
