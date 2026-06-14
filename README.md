# meta-signal-spirit

Schema-derived MetaSignal contract for privileged `spirit` policy
surface.

This crate owns owner-only policy/configuration operations for Spirit:
`Configure` sets the archive database target, and `Import` restores
pre-vetted intent records with stable identifiers. Ordinary psyche and intent
vocabulary remains in `signal-spirit`; daemon runtime, sockets, storage,
guardian calls, and lowering remain in `spirit`.

The default build is binary-only over `signal-frame`. Enable `nota-text` only
for examples, CLI-edge projection, and text round-trip tests.
