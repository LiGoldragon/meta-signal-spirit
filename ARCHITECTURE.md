# meta-signal-spirit — architecture

*MetaSignal contract for privileged Spirit lifecycle and policy.*

## Role

`meta-signal-spirit` is the meta-policy Signal surface for
`spirit`. It carries supervisor-issued lifecycle and policy orders.
Spirit has no cognitive meta; the supervisor owns only infrastructure
lifecycle.

Ordinary psyche statements, intent observations, clarification questions, and
subscriptions live in `signal-spirit`.

## Migration history — three-layer model

On 2026-05-20 this contract moved to the current three-layer model:

- wire operations are contract-local meta verbs;
- component commands are internal to `spirit`;
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
| Lifecycle/configuration orders live only in the meta contract. | Ordinary `signal-spirit::SpiritRequest` has no meta variants. |
| Every meta request is a contract-local verb. | `round_trip.rs` asserts each variant's NOTA head. Sema classification is daemon-side projection only. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, redb, sockets, or sema-engine code. |
| Unimplemented replies carry only the reason; the request/reply position already names the operation. | `meta_spirit_canonical_examples_round_trip` expects `(RequestUnimplemented (NotBuiltYet))`. |

## Code Map

```text
src/lib.rs              — meta request/reply records and signal_channel! invocation
examples/canonical.nota — meta request/reply examples
tests/round_trip.rs     — rkyv frame + NOTA + verb mapping witnesses
tests/frame.rs          — default-feature rkyv frame witness
tests/dependency_boundary.rs — default binary-only dependency witness
```
