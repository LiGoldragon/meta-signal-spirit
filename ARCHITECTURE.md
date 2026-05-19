# owner-signal-persona-spirit — architecture

*OwnerSignal contract for privileged Persona spirit lifecycle and policy.*

## Role

`owner-signal-persona-spirit` is the owner-only Signal surface for
`persona-spirit`. It carries supervisor-issued lifecycle and policy orders.
Spirit has no cognitive owner; the supervisor owns only infrastructure
lifecycle.

Ordinary psyche statements, intent observations, clarification questions, and
subscriptions live in `signal-persona-spirit`.

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
