# owner-signal-persona-spirit — architecture

*OwnerSignal contract for privileged Persona spirit lifecycle and policy.*

## Role

`owner-signal-persona-spirit` is the owner-only Signal surface for
`persona-spirit`. It carries supervisor-issued lifecycle and policy orders.
Spirit has no cognitive owner; the supervisor owns only infrastructure
lifecycle.

Ordinary psyche statements, intent observations, clarification questions, and
subscriptions live in `signal-persona-spirit`.

## Migration history — three-layer model

On 2026-05-20 this contract moved to the current three-layer model:

- wire operations are contract-local owner verbs;
- component commands are internal to `persona-spirit`;
- Sema labels are payloadless observation classifications projected by
  runtime code, not wrappers around wire operations.

The old `Mutate StartOrder` / `Mutate DrainAndStopOrder` /
`Mutate ReloadBootstrapPolicyOrder` / `Mutate RegisterIdentity` /
`Retract RetireIdentity` shape is retired. The crate now depends on
`signal-frame` rather than `signal-core`.

## Contract Surface

| Request | Meaning |
|---|---|
| `Start(Start)` | Bring the spirit daemon into active service. |
| `Drain(Drain)` | Drain work and stop spirit cleanly. |
| `Reload(BootstrapPolicy)` | Reload bootstrap-policy content when allowed by runtime policy. |
| `Register(Registration)` | Register a psyche identity marker. |
| `Retire(Retirement)` | Retire a psyche identity marker. |

The wire form carries contract-local verbs only. Sema class labels are
daemon-side projections.

## Constraints

| Constraint | Witness |
|---|---|
| Lifecycle/configuration orders live only in the owner contract. | Ordinary `signal-persona-spirit::SpiritRequest` has no owner variants. |
| Every owner request is a contract-local verb. | `round_trip.rs` asserts each variant's NOTA head. Sema classification is daemon-side projection only. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, redb, sockets, or sema-engine code. |
| Unimplemented replies carry only the reason; the request/reply position already names the operation. | `owner_spirit_canonical_examples_round_trip` expects `(RequestUnimplemented (NotBuiltYet))`. |

## Code Map

```text
src/lib.rs              — owner request/reply records and signal_channel! invocation
examples/canonical.nota — owner request/reply examples
tests/round_trip.rs     — rkyv frame + NOTA + verb mapping witnesses
```
