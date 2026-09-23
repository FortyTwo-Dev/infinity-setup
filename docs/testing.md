# Testing policy

Tests verify features work when they should, and fail when they should not.
Aim for the widest realistic case coverage.

## Principles

- **Test per unit and per feature**: each unit (function) gets dedicated tests,
  and each feature gets tests that cover its behavior as a whole. When you add
  or change a function or a feature, add or update its tests in the same change.
- **Test both sides**: verify the feature succeeds on valid input (positive
  cases) *and* fails on invalid input (negative cases). A feature must work
  when expected and not work when not expected.
- **Cover the edge cases**: empty input, missing values, duplicate values,
  boundary values, malformed input, unexpected keys, and default state.

## Where tests live

- Unit tests are placed in the same file as the code they test, in a
  `#[cfg(test)] mod tests` block (see `src/annotation.rs`).
- A test module should sit close to the unit it covers.

## What to test

| Target | Cases |
|--------|-------|
| Parsing (`annotation.rs`) | known keys, unknown keys, empty values, missing colon, repeated keys |
| Catalog (`catalog.rs`) | empty directory, missing directory, filtering by extension, sorting |
| Modules (`modules/`) | supported distro, unsupported distro, exit-code mapping |

## Rules

- Tests must be deterministic and not require a real system (no network, no
  package manager, no sudo). Keep side effects at the edges so the core logic
  is unit-testable.
- Run `cargo test` and ensure it passes before finishing a change.
- Do not reduce coverage to silence a failing test; fix the test or the code.
