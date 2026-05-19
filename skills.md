# skills — owner-signal-persona-spirit

Read this before editing the owner-only spirit contract.

## Required Context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `signal-persona-spirit/ARCHITECTURE.md`

## Boundary

This crate owns privileged supervisor-to-spirit vocabulary. It has no runtime,
no actors, no sockets, no storage, and no classifier logic.

## Invariants

- Supervisor lifecycle and policy orders live here, not in the ordinary spirit contract.
- Every request variant declares a Signal root verb through `signal_channel!`.
- Shared spirit nouns are imported from `signal-persona-spirit`; do not duplicate them.
- Runtime interpretation stays in `persona-spirit`.
