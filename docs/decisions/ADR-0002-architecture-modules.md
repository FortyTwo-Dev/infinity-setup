# ADR-0002 : Module architecture and script execution

- **Status** : `accepted`
- **Date** : 2026-09-23

## Validation

| Field    | Value |
|----------|-------|
| Validated | yes |
| Validated by | FortyTwo_Dev |
| Validated on | 2026-09-23 |

## Context

The program must orchestrate bash scripts of varied responsibilities (package
management, download, configuration...). We want to extend capabilities without
reworking the core.

## Decision

A **module** architecture behind a common trait:

```rust
pub trait Module {
    fn name(&self) -> &str;                       // "packages"
    fn supports(&self, distro: &str) -> bool;     // "fedora" -> true
    fn run(&self, script: &ScriptEntry) -> Result<()>;
}
```

- Each module lives in `src/modules/<name>.rs`.
- `main.rs` selects the module via the `@module` annotation, then delegates
  `run`.
- The raw execution logic is shared via `modules::execute()`.

## Execution (approach A)

The script is **executed as-is** via `bash` (with `sudo bash` if `@requires`
contains `sudo`). The module does not rewrite the command; it acts as a guard
(checks `supports(distro)`) and display (description).

Exit code handling:
- `0` = success.
- Any code listed in `@exit-codes` = success (e.g. dnf returns `100` when
  updates are available).
- Any other code = error surfaced.

## Multi-distribution handling

At this stage, "multi-distro" only means a module **accepts** several
distributions (`supports()`), not that it translates a command from one package
manager to another. Each distribution has **its own script**
(`packages-fedora.sh`, `packages-debian.sh`, ...).

The generic translation (a declarative script the module rewrites into
`dnf`/`apt`/`pacman`) is a future evolution: approach B, described in a later
ADR.

## Consequences

- Adding a module = create a file + one branch in the `match` in `main.rs`.
- Common logic (sudo, description, exit-codes) is not duplicated.
- Scripts remain executable outside of the application.

## Rejected alternatives

- **Command rewriting now (approach B)** : more powerful but premature for the
  PoC; we prefer to validate the flow first.
