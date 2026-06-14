# meta-signal-spirit — architecture

*MetaSignal contract for privileged Spirit lifecycle and policy.*

## Role

`meta-signal-spirit` is the meta-policy Signal surface for
`spirit`. It carries owner-only policy and restore operations. Spirit has no
cognitive meta; the owner surface exists for infrastructure authority that
ordinary peers must not hold.

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
`Retract RetireIdentity` shape is retired. The later hand-written
`Start` / `Drain` / `Reload` / `Register` / `Retire` placeholder surface is
also retired. The live contract is schema-derived and carries the owner-only
`Configure` / `Import` surface used by `spirit`.

## Contract Surface

| Request | Meaning |
|---|---|
| `Configure(ConfigureRequest)` | Set the archive database target used by owner-controlled archival policy. |
| `Import(ImportRequest)` | Restore pre-vetted records with stable identifiers, bypassing ordinary guardian admission by owner authority. |

The wire form carries contract-local verbs only. Sema class labels are
daemon-side projections.

## Constraints

| Constraint | Witness |
|---|---|
| Lifecycle/configuration orders live only in the meta contract. | Ordinary `signal-spirit::Input` has no meta variants. |
| Every meta request is a contract-local verb. | `round_trip.rs` asserts each variant's NOTA head. Sema classification is daemon-side projection only. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, redb, sockets, or sema-engine code. |
| The contract imports shared Spirit nouns instead of duplicating them. | `schema/meta-signal.schema` imports `DatabaseMarker`, `Entry`, `RecordIdentifier`, and `RecordCount` from `signal-spirit`. |

## Code Map

```text
schema/meta-signal.schema — source-of-truth meta policy schema
src/schema/meta_signal.rs — generated meta request/reply records and codecs
src/lib.rs              — generated contract re-exports
examples/canonical.nota — meta request/reply examples
tests/round_trip.rs     — rkyv frame + NOTA + verb mapping witnesses
tests/frame.rs          — default-feature rkyv frame witness
tests/dependency_boundary.rs — default binary-only dependency witness
```
