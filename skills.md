# skills — meta-signal-spirit

Read this before editing the meta-policy spirit contract.

## Required Context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `signal-spirit/ARCHITECTURE.md`

## Boundary

This crate owns privileged supervisor-to-spirit vocabulary. It has no runtime,
no actors, no sockets, no storage, and no classifier logic.

## Invariants

- Supervisor lifecycle and policy orders live here, not in the ordinary spirit contract.
- Every request variant is declared as a schema root in `schema/meta-signal.schema`.
- Shared spirit nouns are imported from `signal-spirit`; do not duplicate them.
- Version 0.8.0 is wire revision 2 and imports only four-field v14 entries.
- The request roots are `Configure`, `Import`, `ObserveHead`, and
  `ObserveHeadObject`; do not add compatibility forms or defaults.
- Runtime interpretation stays in `spirit`.
