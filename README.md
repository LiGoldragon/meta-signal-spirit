# meta-signal-spirit

MetaSignal contract for privileged `spirit` policy surface, generated
through `spirit`'s current wired-legacy schema/schema-rust pipeline. The
language was renamed Ethos on 2026-07-27 (S1R entry 7); legacy schema,
schema-language, and schema-rust die under their old names. This crate's
generation has not yet been ported onto Ethos-based generation.

This crate owns owner-only policy/configuration operations for Spirit:
`Configure` sets the archive database target, and `Import` restores
pre-vetted intent records with stable identifiers. Ordinary psyche and intent
vocabulary remains in `signal-spirit`; daemon runtime, sockets, storage,
guardian calls, and lowering remain in `spirit`.

The default build is binary-only over `signal-frame`. Enable `nota-text` only
for examples, CLI-edge projection, and text round-trip tests.
