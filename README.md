# meta-signal-spirit

Schema-derived MetaSignal contract for privileged `spirit` policy
surface.

This crate owns Spirit's owner-only `Configure`, `Import`,
`CollectRemovalCandidates`, `ObserveHead`, and `ObserveHeadObject` operations.
The ordinary working socket has no removal authority. Ordinary psyche and
intent vocabulary remains in `signal-spirit`; daemon runtime, sockets, storage,
guardian calls, and lowering remain in `spirit`.

The default build is binary-only over `signal-frame`. Enable `nota-text` only
for examples, CLI-edge projection, and text round-trip tests.
