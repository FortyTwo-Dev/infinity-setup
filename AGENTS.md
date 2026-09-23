# AGENTS.md

Guidelines for AI agents (and contributors) working on this repository.

## Project language

Everything in this repository is written in English: code, comments, commit
messages, documentation, ADRs, and user-facing messages.

## Architecture Decision Records (ADR)

Before making an architectural decision, read the existing records in
`docs/decisions/`.

- Each ADR documents a single decision: its context, the decision, its
  consequences, and the rejected alternatives.
- ADRs are written **together** with the user, never unilaterally. Propose,
  then wait for validation.
- Each ADR has a `Status` field and a `Validation` section at the top:

  | Status       | Meaning |
  |--------------|---------|
  | `proposed`   | written, awaiting user review |
  | `accepted`   | validated by the user |
  | `deprecated` | superseded by a newer ADR |
  | `archived`   | no longer relevant |

- When a decision changes, mark the old ADR `deprecated` (or `archived`) and
  add a new one rather than silently rewriting history.

## Code conventions

Read `docs/conventions.md` before writing code. In short: single responsibility,
no duplication, clear ownership, testability, and model complex data with
dedicated types.

## Testing

Read `docs/testing.md` before writing tests. In short: test per unit and per
feature, cover both positive and negative cases, aim for maximum case coverage.

## Dependencies

- Do not add a dependency without explicit validation from the user.
- Propose the crate, its justification, and alternatives; wait for approval
  before adding it to `Cargo.toml`.

## Naming

- Modules live in `src/modules/<name>.rs`.
- Scripts live in `scripts/`.
- ADRs live in `docs/decisions/ADR-NNNN-slug.md` (four-digit zero-padded
  number, short descriptive slug).

## Git workflow

Read `docs/git-workflow.md` before committing or creating branches. In short:
atomic commits with `type(scope): subject` messages, one branch per objective
based on `develop`, merge via pull request, and `develop` → `main` on stable
releases.

## Code quality (SonarQube)

Before pushing, run the local SonarQube analysis (see `docker/sonarqube/README.md`)
and check its findings. SonarQube is a helper, not an oracle: its reports are
not always correct. Review each finding with judgment and fix what is genuinely
relevant rather than blindly clearing every issue.
