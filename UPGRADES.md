# Upgrades

# 3.0.1 to 4.0.0

datom-codec 0.27.0 gives arity back to `Compositional`: it now carries
`const ARITY` and `from_positions`, and states a positional type's
positions. The derive macro is renamed to match, so
`datom_codec::Compositional` becomes `datom_codec::Composing` in every
generated `#[cfg_attr(feature = "datom", derive(...))]` attribute.
ethos-zero 9.0.0 emits the new derive name, and because it is shorter
the attribute no longer wraps onto four lines, so every ethos-zero
generated file in this crate changed.

There is no compatibility path. A consumer using the generated
`#[derive(datom_codec::Compositional)]` bound directly changes it to
`datom_codec::Composing`; consumers going through the crate's public
`Query`/`Response` API are unaffected beyond the recompile.

# 1.0.1 to 2.0.0

The Signal frame type and its three kinds left this crate. `Signal<T>`,
`Signalizable`, `ByteViewable`, and `Restorable<T>` were defined here, in a
copy byte-identical to the one in every other contract crate. They now live
once, generically, in the `signal` repository, and this crate depends on it.

There is no compatibility path. A consumer changes its imports:

```rust
-use meta_signal_spirit::{ByteViewable, Restorable, Signal, Signalizable, Query, Response};
+use signal::{ByteViewable, Restorable, Signal, Signalizable};
+use meta_signal_spirit::{Query, Response};
```

and adds the dependency:

```toml
signal = { git = "https://github.com/LiGoldragon/signal", rev = "626e407be520a7a12f39b1d06c56ec423f3b3d09" }
```

The behavior is unchanged: the same rkyv bytes, the same validation on
restore. `Signalizable` and `Restorable<T>` are blanket implementations now,
so every contract type has them without the crate writing anything.
