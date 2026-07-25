# meta-signal-spirit — architecture

*MetaSignal contract for privileged Spirit lifecycle and policy.*

## Direction

`meta-signal-spirit` is the owner-only policy Signal contract for `spirit`.
Ordinary psyche statements, queries, and subscriptions live in
`signal-spirit`. This crate owns configuration, privileged import, removal
candidate collection, and log-head observation. Sema vocabulary is daemon-side
observation classification only and is not the public request spine.

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
surface used by `spirit`.

## Contract Surface

| Request | Meaning |
|---|---|
| `Configure(ConfigureRequest)` | Set the owner-controlled runtime policy targets: the archive database target, the optional mirror target, the optional local-criome gate target, and the optional guardian-prompt target. |
| `Import(ImportRequest)` | Restore pre-vetted records with stable identifiers, bypassing ordinary guardian admission by owner authority. |
| `CollectRemovalCandidates(CollectRemovalCandidatesRequest)` | Collect and archive removal candidates under owner authority. This capability does not exist on the ordinary working socket. |
| `ObserveHead` | Observe the current versioned log-head digest. |
| `ObserveHeadObject` | Observe the current versioned log-head object. |

| Reply | Meaning |
|---|---|
| `Configured(ConfigureReceipt)` | Report the applied owner configuration. |
| `Imported(ImportReceipt)` | Report the imported record count and resulting database marker. |
| `RemovalCandidatesCollected(RemovalCandidatesCollectedReceipt)` | Report archived, removed, and skipped candidates with the resulting database marker. |
| `Rejected(ConfigureRejection)` | Reject an invalid or unavailable configuration target. |
| `HeadObserved(VersionedLogHead)` | Report the optional current log-head digest. |
| `HeadObjectObserved(VersionedLogHeadObject)` | Report the optional current log-head object. |

`ConfigureRequest` and `ConfigureReceipt` each carry an optional
`SelectedGuardianPromptTarget`. `GuardianPromptTarget::Default` keeps the
daemon's compiled-in guardian role prompt; `GuardianPromptTarget::Prompt`
carries an owner-supplied role-section override that the daemon applies to the
live guardian without a rebuild. An absent target leaves the live guardian's
current prompt unchanged. The override is owner runtime policy, not durable
state: like the other Configure targets it lives in the running daemon and falls
back to the compiled-in default on restart until an owner re-sends it. The
closed rejection-reason catalogue and the NOTA verdict grammar stay
daemon-code-rendered from the wire enums, so a prompt override can never shift
the verdict vocabulary the daemon parses.

The wire form carries contract-local verbs only. Sema class labels are
daemon-side projections.

## Constraints

| Constraint | Witness |
|---|---|
| Owner configuration, import, removal-candidate collection, and head observation live only in the meta contract. | `wire_inventory.rs` compares both authored schemas and proves the ordinary `signal-spirit` roots have none of the owner-only variants. |
| The ordinary working socket has no deletion or removal capability. | `wire_inventory.rs` closes `Remove` and `CollectRemovalCandidates` on ordinary Input and closes removal replies on ordinary Output. |
| Every meta request is a contract-local verb. | `wire_inventory.rs` locks the complete authored and generated root inventory. Sema classification is daemon-side projection only. |
| Wire identity is stable. | `wire_inventory.rs` locks all request/reply route order, short headers, and archived route tags. |
| The checked-in Rust contract is generated from the current six-slot dotted/positional schema. | Every build runs the schema-rust freshness check; `wire_inventory.rs` also proves authored roots and generated heads converge. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, redb, sockets, or sema-engine code. |
| The contract imports shared Spirit nouns instead of duplicating them. | `schema/meta-signal.schema` imports `DatabaseMarker`, `Entry`, `RecordIdentifier`, `RecordCount`, `RemovalCandidateCollection`, and `RemovalCandidatesCollection` from `signal-spirit`. |
| One canonical micro-repository dependency world is used. | `dependency_boundary.rs` locks exact producer revisions and rejects duplicate, branch, patch, path, and legacy dependency sources. |

## Code Map

```text
schema/meta-signal.schema — source-of-truth meta policy schema
src/schema/meta_signal.rs — generated meta request/reply records and codecs
src/lib.rs              — generated contract re-exports and compatibility helpers
examples/canonical.nota — meta request/reply examples plus the ordinary signal-spirit `PublicIntent` dependency witness
tests/round_trip.rs     — rkyv frame + NOTA + verb mapping witnesses
tests/frame.rs          — default-feature rkyv frame witness
tests/dependency_boundary.rs — default binary-only dependency witness
tests/wire_inventory.rs — complete roots, headers, tags, and owner-only boundary
tests/true_schema_nota_projection.rs — authored schema lowering and structured NOTA witness
```
