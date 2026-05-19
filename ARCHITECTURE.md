# owner-signal-persona-spirit — architecture

*OwnerSignal contract for privileged Persona spirit lifecycle and policy.*

## Role

`owner-signal-persona-spirit` is the owner-only Signal surface for
`persona-spirit`. It carries supervisor-issued lifecycle and policy orders.
Spirit has no cognitive owner; the supervisor owns only infrastructure
lifecycle.

Ordinary psyche statements, intent observations, clarification questions, and
subscriptions live in `signal-persona-spirit`.

## MUST IMPLEMENT — signal architecture migration

This contract is migrating to contract-local verbs per
`primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
and `primary/reports/designer/239-signal-architecture-migration-plan.md`.

Drop the `Mutate StartOrder` / `Mutate DrainAndStopOrder` /
`Mutate ReloadBootstrapPolicyOrder` / `Mutate RegisterIdentity` /
`Retract RetireIdentity` wrapping. Use contract-local owner verbs
directly. Candidate verbs: `Start` (for `StartOrder`), `Drain` or
`Stop` (for `DrainAndStopOrder` — likely two distinct verbs: `Drain`
to initiate the drain, and the stop is implicit on drain completion;
or `Stop` carries a drain mode on the payload), `Reload` (for
`ReloadBootstrapPolicyOrder` — payload names what to reload),
`Register` (for `RegisterIdentity` — payload is `Identity` or
`Registration`), `Retire` (for `RetireIdentity` — payload names the
identity). Drop the `*Order` suffix throughout — the crate's
`owner-` prefix already establishes these are authoritative orders.
Move verb-to-Sema lowering (`Start` → `Mutate` lifecycle state,
`Drain` → `Mutate` drain state then `Retract` running state, etc.)
into `persona-spirit`.

References: `primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`,
`primary/reports/designer/239-signal-architecture-migration-plan.md`.

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — contract-local verbs (2026-05-XX)`
paragraph noting the shape change.

## Contract Surface

| Request | Signal verb | Meaning |
|---|---|---|
| `StartOrder` | `Mutate` | Bring the spirit daemon into active service. |
| `DrainAndStopOrder` | `Mutate` | Drain work and stop spirit cleanly. |
| `ReloadBootstrapPolicyOrder` | `Mutate` | Reload bootstrap-policy content when allowed by runtime policy. |
| `RegisterIdentity` | `Mutate` | Register a psyche identity marker. |
| `RetireIdentity` | `Retract` | Retire a psyche identity marker. |

## Constraints

| Constraint | Witness |
|---|---|
| Lifecycle/configuration orders live only in the owner contract. | Ordinary `signal-persona-spirit::SpiritRequest` has no owner variants. |
| Every owner request declares a Signal root verb. | `round_trip.rs` checks `signal_verb()` for every variant. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, redb, sockets, or sema-engine code. |

## Code Map

```text
src/lib.rs              — owner request/reply records and signal_channel! invocation
examples/canonical.nota — owner request/reply examples
tests/round_trip.rs     — rkyv frame + NOTA + verb mapping witnesses
```
