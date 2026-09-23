# Git workflow

Inspired by the Linux kernel conventions, slightly relaxed. The goal is a
clean, readable history that is easy to review.

## Commit philosophy

- **Atomic commits**: one commit = one logical change. Do not mix unrelated
  changes in the same commit. A reviewer should understand the commit from its
  message alone.
- Keep each commit in a buildable state (as far as possible).

## Commit message format

```
type(scope): subject
```

- `type` : the kind of change.
- `scope` : the area of the code concerned (optional but encouraged).
- `subject` : short imperative summary, lowercase, no trailing period.

Types:

| Type       | Purpose |
|------------|---------|
| `feat`     | new feature |
| `fix`      | bug fix |
| `docs`     | documentation only |
| `refactor` | code change without behavior change |
| `test`     | adding or updating tests |
| `chore`    | build, tooling, maintenance |
| `style`    | formatting, no behavior change |

Examples:

```
feat(packages): add apt module
fix(annotation): parse multiple exit-codes
docs(decisions): record annotation format decision
```

A body may follow (blank line after the subject) to explain *why* when the
subject is not enough.

## Branching model

- `main` : stable, releasable state only.
- `develop` : integration branch, always the base for new work.

For each objective:

1. Create a branch from `develop`. Name it after the objective, e.g.
   `feature/apt-module`, `fix/exit-code-parsing`.
2. Group all commits of that objective on the branch.
3. Push the branch and open a **pull request** targeting `develop`.
4. The PR is validated (review) before merging.

## Releasing

When the state on `develop` is considered a **stable version**:

1. Merge `develop` into `main`.
2. Tag the release on `main` (e.g. `v0.1.0`).

## Ignoring files

List in `.gitignore` everything that must not be versioned:

- Build output (`/target`).
- IDE / editor files (`/.idea/`, `*.iml`).
- Local environment files (`.env`, `.env.*`).
- Secrets and sensitive material: keys, certificates, credentials
  (`*.key`, `*.pem`, `*.crt`, `credentials.json`, ...).

Never commit a secret, even temporarily. If one is committed, rotate it
immediately and rewrite history to remove it.

## Rules

- Never commit directly to `main`.
- Do not force-push a shared branch.
- Keep PRs focused on a single objective.
