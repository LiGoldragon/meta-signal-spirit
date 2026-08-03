use meta_signal_spirit::{
    ArchiveDatabaseTarget, ConfigureRequest, ImportedRecord, ImportedRecords, Input,
};
use signal_spirit::{
    Description, Domain, DomainScope, DomainScopes, Domains, Entry, Importance,
    Input as SpiritInput, InputRoute as SpiritInputRoute, Kind, Magnitude, OperationKind,
    RecordIdentifier,
};

fn universal_domain_scopes() -> DomainScopes {
    DomainScopes::new(vec![DomainScope::from(Domain::All)])
}

fn imported_records() -> ImportedRecords {
    ImportedRecords::new(vec![ImportedRecord {
        record_identifier: RecordIdentifier::new("0001"),
        entry: Entry {
            domains: Domains::new(vec![Domain::All]),
            kind: Kind::Decision,
            description: Description::new("four-field revision-2 import"),
            importance: Importance::new(Magnitude::Medium),
        },
    }])
}

#[test]
fn default_build_round_trips_configure_without_text_projection() {
    let request = Input::configure(ConfigureRequest::new(
        ArchiveDatabaseTarget::Default,
        None,
        None,
        None,
    ));

    let bytes = request.encode_signal_frame().expect("encode request");
    let (_route, decoded) = Input::decode_signal_frame(&bytes).expect("decode request");

    assert_eq!(decoded, request);
}

#[test]
fn default_build_round_trips_four_field_import_without_text_projection() {
    let request = Input::import(imported_records().into());

    let bytes = request.encode_signal_frame().expect("encode request");
    let (_route, decoded) = Input::decode_signal_frame(&bytes).expect("decode request");

    assert_eq!(decoded, request);
}

#[test]
fn default_build_round_trips_intent_dependency_without_text_projection() {
    let request = SpiritInput::intent(universal_domain_scopes());

    let bytes = request.encode_signal_frame().expect("encode request");
    let (route, decoded) = SpiritInput::decode_signal_frame(&bytes).expect("decode request");

    assert_eq!(route, SpiritInputRoute::Intent);
    assert_eq!(decoded, request);
    assert_eq!(OperationKind::from_input(&request), OperationKind::Intent);
}

#[test]
fn active_meta_schema_and_generated_contract_exclude_retired_collection_vocabulary() {
    for removed in ["RemovalCandidate", "CollectRemovalCandidates"] {
        assert!(!meta_signal_spirit::META_SIGNAL_SCHEMA_SOURCE.contains(removed));
        assert!(!meta_signal_spirit::META_SIGNAL_RUST_SOURCE.contains(removed));
    }
}
