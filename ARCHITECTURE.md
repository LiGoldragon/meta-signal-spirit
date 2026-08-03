# meta-signal-spirit architecture

`meta-signal-spirit` is the owner-only policy contract for `spirit`. Version
0.8.0 defines wire revision 2 and depends exactly on `signal-spirit` 0.14.0.
Revision 1 frames are not accepted or upgraded online.

The contract owns four request roots:

| Request | Meaning |
|---|---|
| `Configure` | Select lifecycle archive, mirror, Criome gate, and guardian prompt targets. |
| `Import` | Restore pre-vetted v14 records with stable identifiers. |
| `ObserveHead` | Observe the versioned-log head digest and database marker. |
| `ObserveHeadObject` | Observe the versioned-log head object and database marker. |

`Import` accepts only the four-field `signal-spirit::Entry` shape. Offline
migration owns earlier storage decoding. The archive target remains because
the lifecycle archive remains.

This crate contains no runtime, persistence, sockets, policy execution, or
migration. `schema/meta-signal.schema` is authoritative and
`src/schema/meta_signal.rs` is generated. Default builds are binary/rkyv-only;
`nota-text` is an explicit edge projection.

Tests must prove the four request roots, four-field import round trips, exact
dependency pin, generated/schema convergence, rejected revision-1 collection
syntax, and absence of retired collection vocabulary from active artifacts.
