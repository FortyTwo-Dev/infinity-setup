# ADR-0001 : Annotation format for bash scripts

- **Status** : `accepted`
- **Date** : 2026-09-23

## Validation

| Field    | Value |
|----------|-------|
| Validated | yes |
| Validated by | FortyTwo_Dev |
| Validated on | 2026-09-23 |

## Context

The application orchestrates small bash scripts, each with a single
responsibility. The program needs metadata about each script to know what to
display, which module handles it, which distribution it targets, and whether it
requires privileges.

## Decision

Annotations are **prefixed comments** placed at the top of the script:

```bash
#!/usr/bin/env bash
# @module: packages
# @description: Installs base packages
# @distro: fedora
# @requires: sudo
# @exit-codes: 100
```

Rules:
- One annotation per line, formatted as `# @key: value`.
- The parser only reads lines starting with `# @`; the rest of the script is
  ignored by the parser (but executed as-is).
- Recognized keys: `module`, `description`, `distro`, `requires` (repeatable),
  `exit-codes` (repeatable, space-separated).
- Unknown keys are silently ignored (allows adding keys gradually without
  breaking existing scripts).

## Keys

| Key            | Repeatable | Role |
|----------------|------------|------|
| `@module`      | no         | name of the module that handles the script |
| `@description` | no         | text displayed before execution |
| `@distro`      | no         | target distribution (fedora, debian, ...) |
| `@requires`    | yes        | prerequisites (`sudo`, ...) |
| `@exit-codes`  | yes        | exit codes considered as success |

## Consequences

- No parsing dependency (hand-written in `src/annotation.rs`).
- The script remains a valid bash script, executable independently of the app.
- Adding a key = one branch in the `match` in `annotation::parse`.

## Rejected alternatives

- **YAML frontmatter** : structured but requires `serde_yaml`, less natural in
  a `.sh` file.
- **TOML/JSON header** : consistent with the project config but verbose in a
  shell file.
