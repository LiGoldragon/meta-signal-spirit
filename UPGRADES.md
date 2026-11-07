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
