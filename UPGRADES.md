# 4.1.0 — first protos 0.33 text-cohort adoption

The authored Ethos, library and generated-freshness build script are
unchanged. Generated aliases become tuple newtypes with comparison/hash
traits. Tests construct the imported/local wrappers explicitly and use
checked reader budgets. The architecture description now reflects the
existing Ethos Zero source path and five request roots.

Selected cohort: datom-codec eabead4 (0.33.0), protos 3c7ade3 (0.33.0),
Signal 7afb68b (10.3.0), ordinary signal-spirit3a0b62f (4.1.0), and
ethos-zero5a5daca (19.2.0). The ordinary producer URL uses the same `.git`
spelling as its current consumers. There is one codec/protos/Signal/spirit
identity; two Ethos source spellings resolve to the same exact commit.
Compatible registry versions are preserved by `cargo update --workspace`.

Historical baseline b3f884f0915d101610136b47a8e6d11276a1e57b (4.0.0) runs
its own original source, manifest, lock and tools. The candidate runs its
selected cohort. Scratch copies add only witness-test suffixes. Exact
source/graph linkage is in `/tmp/01e4a4-runtime/meta-spirit-source-linkage.json`;
source, raw TSV outputs, witness/cross-read logs and exit files use the
`meta-spirit-{old,new}` names there. `link-witness.py` and
`compare-witness.py` reproduce the linkage and raw/token comparisons.

Eleven fixtures sample all five Query roots and all six Response roots,
including configuration data and empty import/optional values. Raw rkyv
archives and existing Signal frame bytes match on eleven of eleven; all
frames restore locally. Each cohort also restores the counterpart archives
and parses the counterpart printed text to the same typed values. Raw text
matches on four of eleven. A separate exact-token and balanced-string
inspection verifies whitespace-layout-only differences on the other seven,
independently of typed cross-reading. No compact fixture normalization is
introduced here.

Root coverage is sampled, not exhaustive nested-variant/value/refusal,
source-API, storage-consumer or running-deployment compatibility evidence.


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
