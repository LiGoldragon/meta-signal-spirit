# meta-signal-spirit architecture

`meta-signal-spirit` is the owner-only policy contract for `spirit`. Version
0.9.0 defines wire revision 3 and depends on `signal-spirit` 0.16.0 (the
authority-sealed generation at d3690ae).

The contract owns four request roots:

| Request | Meaning |
|---|---|
| `Configure` | Select lifecycle archive, mirror, Criome gate, and guardian prompt targets. |
| `Import` | Restore pre-vetted v14 records with stable identifiers. |
| `ObserveHead` | Observe the versioned-log head digest and database marker. |
| `ObserveHeadObject` | Observe the versioned-log head object and database marker. |

`schema/meta.schema` is the authoritative Interface source, rescued from the
condemned `spirit-ethos` repository (761454c) and re-spelled to the blessed
5-section form (typed triple version, `/` imports, empty Refusal and Stream
sections). `build.rs` authorizes it through `SemaBootstrapAuthority` and
generates the Rust projection through `CommitBootstrap` with deterministic
source-digest freshness. Cross-source imports resolve by authorizing
signal-domain and signal-spirit dependency sources first.

`src/schema/meta/generated.rs` is the authority-sealed Rust projection.
This crate contains no runtime, persistence, sockets, policy execution, or
migration.

Known generation gaps (licensed breakage per hqu.14 replacement-kills):
- rust-logos does not yet emit `#[derive(...)]` attributes
- rust-logos does not yet emit Display/Error for Refusal types
- The old pipeline generated full derives, accessor methods, rkyv support,
  signal-frame integration, nota encoding, short headers, route enums, and
  From impls; this bare projection does not yet replicate that surface
- Manual Debug derives and Display/Error impls bridge the Refusal trait bound

Tests prove the authority-sealed source is available, imported spirit types
resolve, and retired collection vocabulary is absent.
