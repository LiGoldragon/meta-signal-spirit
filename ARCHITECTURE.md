# owner-signal-persona-spirit — architecture

*OwnerSignal contract for privileged Persona spirit lifecycle and policy.*

## Role

`owner-signal-persona-spirit` is the owner-only Signal surface for
`persona-spirit`. It carries supervisor-issued lifecycle and policy orders.
Spirit has no cognitive owner; the supervisor owns only infrastructure
lifecycle.

Ordinary psyche statements, intent observations, clarification questions, and
subscriptions live in `signal-persona-spirit`.

## MUST IMPLEMENT — three-layer migration

This contract is migrating to the three-layer model affirmed
2026-05-20 per
`primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
and `primary/reports/designer/248-three-layer-changes-for-operators.md`.

**Layer 1 — Contract Operations on the wire (this crate).** Drop the
`Mutate StartOrder` / `Mutate DrainAndStopOrder` /
`Mutate ReloadBootstrapPolicyOrder` / `Mutate RegisterIdentity` /
`Retract RetireIdentity` wrapping entirely. Use contract-local owner
verbs directly. Candidate verbs:

- `Start` (for `StartOrder`),
- `Drain` or `Stop` (for `DrainAndStopOrder` — likely two distinct
  verbs: `Drain` to initiate the drain, and the stop is implicit on
  drain completion; or `Stop` carries a drain mode on the payload),
- `Reload` (for `ReloadBootstrapPolicyOrder` — payload names what to
  reload),
- `Register` (for `RegisterIdentity` — payload is `Identity` or
  `Registration`),
- `Retire` (for `RetireIdentity` — payload names the identity).

Drop the `*Order` suffix throughout — the crate's `owner-` prefix
already establishes these are authoritative orders.

**Layer 2 — Component Commands.** Lowering from contract operation to
Component Commands (`Start` → `MutateLifecycleRunning`,
`Drain` → `MutateDrainMode` then `RetractRunning`, etc.) lives in the
`persona-spirit` daemon, not in this contract.

**Layer 3 — Sema classification.** Each Component Command projects to
a payloadless Sema class label via `ToSemaOperation` for observation.
The owner socket is mandatory `Tap`/`Untap`-free in the contract
itself; introspect subscribes on the ordinary `signal-persona-spirit`
surface for the standardized observability per psyche intent
(observation isn't security-sensitive and isn't routed through the
owner socket).

**Frame layer.** The dependency on `signal-core` shifts to
`signal-frame`.

References:
- `primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
- `primary/reports/designer/248-three-layer-changes-for-operators.md`
- `primary/skills/component-triad.md` §"Verbs come in three layers"
- `primary/skills/contract-repo.md` §"Public contracts use contract-local operation verbs"

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — three-layer model (2026-05-XX)`
paragraph noting the shape change.

## Contract Surface (current shape — to be renamed per above)

| Request | Projected Sema class | Meaning |
|---|---|---|
| `StartOrder` | `Mutate` | Bring the spirit daemon into active service. |
| `DrainAndStopOrder` | `Mutate` | Drain work and stop spirit cleanly. |
| `ReloadBootstrapPolicyOrder` | `Mutate` | Reload bootstrap-policy content when allowed by runtime policy. |
| `RegisterIdentity` | `Mutate` | Register a psyche identity marker. |
| `RetireIdentity` | `Retract` | Retire a psyche identity marker. |

The wire form (post-migration) will carry contract-local verbs only;
the Sema class label is the daemon-side projection.

## Constraints

| Constraint | Witness |
|---|---|
| Lifecycle/configuration orders live only in the owner contract. | Ordinary `signal-persona-spirit::SpiritRequest` has no owner variants. |
| Every owner request is a contract-local verb in verb form (after migration). | `round_trip.rs` asserts each variant's NOTA head. Sema classification is daemon-side projection only. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, redb, sockets, or sema-engine code. |

## Code Map

```text
src/lib.rs              — owner request/reply records and signal_channel! invocation
examples/canonical.nota — owner request/reply examples
tests/round_trip.rs     — rkyv frame + NOTA + verb mapping witnesses
```
