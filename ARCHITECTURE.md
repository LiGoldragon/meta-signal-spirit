# meta-signal-spirit architecture

`meta-signal-spirit` is the owner policy contract for `spirit`. Version4.1.0
imports the ordinary `signal-spirit`4.1.0 types.

`ethos/signal.ethos` is the authored structural source. `build.rs` generates
its Rust projection with Ethos Zero and checks it against the committed
`src/generated/signal.rs`; `src/lib.rs` re-exports that projection.

The five request roots are `ObserveHead`, `Configure`,
`ReverseMetaConfiguration`, `ObserveHeadObject`, and `Import`. The six reply
roots report configuration, reopening ordinary configuration, head and head
object observations, configuration rejection, and imported record count.
The contract carries typed policy/configuration and observation data;
runtime execution and storage belong to its consumers.

The selected producer cohort and the bounded historical archive/frame/text
witnesses for4.1.0 are documented in `UPGRADES.md`.
