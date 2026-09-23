# Conventions

Coding conventions applied throughout the project. These are general
principles, not architecture decisions (see `docs/decisions/` for those).

## Single responsibility

- A function does exactly one thing. If a function does two things, split it.
- This makes it easier to name, read, test, and reuse each function.

## Avoid duplication (DRY)

- Do not copy-paste logic. Extract the shared code into a function, a trait, or
  a helper module.
- When two modules need the same behavior, factor it out once and reuse it.

## Clear ownership

- Each concern has a single owner module/type. It must be obvious where a given
  behavior lives, and who calls whom.
- Avoid modules that know about everything; prefer focused, well-named units.

## Testability

- Prefer small, pure functions over large side-effecting ones.
- Keep I/O and process execution at the edges so the core logic can be unit
  tested without a real system.

## Model complex data

- Represent complex data with dedicated types (`struct`, `enum`, `trait`) rather
  than raw primitives (`String`, `Vec<i32>`, tuples, ...).
- A type gives the data a name, a shape, and invariants, and makes it testable.
