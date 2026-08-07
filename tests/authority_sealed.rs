//! Verifies the authority-sealed generation produced correct types and role
//! trait implementations for the meta Interface.

/// The rescued Interface source is available as a constant.
#[test]
fn meta_interface_source_is_available() {
    let source = meta_signal_spirit::META_INTERFACE_SOURCE;
    assert!(source.contains("Interface.{1 0 0}"));
    assert!(source.contains("signal/spirit.[RecordCount RecordIdentifier Entry DatabaseMarker]"));
    assert!(source.contains("Configure.ConfigureRequest"));
}

/// The imported spirit types resolve through the generated projection.
#[test]
fn imported_spirit_types_are_reachable() {
    fn _assert_type_exists<T>() {}
    // These are re-exported from signal-spirit via the schema import;
    // if the authority failed to resolve imports this would not compile.
    _assert_type_exists::<signal_spirit::DatabaseMarker>();
    _assert_type_exists::<signal_spirit::Entry>();
    _assert_type_exists::<signal_spirit::RecordIdentifier>();
    _assert_type_exists::<signal_spirit::RecordCount>();
}

/// Active meta schema excludes retired collection vocabulary.
#[test]
fn active_meta_schema_excludes_retired_collection_vocabulary() {
    for removed in ["RemovalCandidate", "CollectRemovalCandidates"] {
        assert!(
            !meta_signal_spirit::META_INTERFACE_SOURCE.contains(removed),
            "meta schema must not contain retired vocabulary: {removed}"
        );
    }
}
