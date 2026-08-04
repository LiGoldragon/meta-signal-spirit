# meta-signal-spirit

MetaSignal contract for privileged `spirit` policy surface. Version 0.8.0 is
wire revision 2, generated
through `spirit`'s current wired-legacy schema/schema-rust pipeline. The
language was renamed Ethos on 2026-07-27 (S1R entry 7); legacy schema,
schema-language, and schema-rust die under their old names. This crate's
generation has not yet been ported onto Ethos-based generation.

This crate owns owner-only policy/configuration operations for Spirit:
`Configure` sets runtime targets, `Import` restores four-field v14 records with
stable identifiers, and the two head observations expose versioned-log state.
Ordinary psyche and intent
vocabulary remains in `signal-spirit`; daemon runtime, sockets, storage,
guardian calls, and lowering remain in `spirit`.

The default build is binary-only over `signal-frame`. Enable `nota-text` only
for examples, CLI-edge projection, and text round-trip tests.

`examples/canonical.nota` contains owner objects, not shell syntax. Its final
ordinary `Intent` line is explicitly a dependency-codec witness, not a
`meta-spirit` transcript. The public `meta-spirit` CLI accepts exactly one
owner object; `ObserveHead` and `ObserveHeadObject` are bare objects, while
paths and Unix flags are invalid.
