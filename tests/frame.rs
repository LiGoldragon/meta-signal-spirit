use meta_signal_spirit::{
    ArchiveDatabaseTarget, ConfigureRequest, ContractMarker, Input, SignalFrameError,
};
use signal_spirit::{
    CertaintySelection, Domain, DomainMatch, DomainScope, DomainScopes, ImportanceSelection,
    Input as SpiritInput, InputRoute as SpiritInputRoute, Justification, KeywordMatch,
    OperationKind, PrivacySelection, Query, Reasoning, RecordQuery, ReferentSelection,
    RemovalCandidateCollection, SelectedKind, TextMatch,
};

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(7),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}

fn universal_domain_scopes() -> DomainScopes {
    DomainScopes::new(vec![DomainScope::from(Domain::All)])
}

fn universal_domain_removal_candidate_collection() -> RemovalCandidateCollection {
    RemovalCandidateCollection {
        record_query: RecordQuery::new(Query {
            domain_match: DomainMatch::partial(universal_domain_scopes()),
            keyword_match: KeywordMatch::Any,
            text_match: TextMatch::Any,
            referent_selection: ReferentSelection::Any,
            selected_kind: SelectedKind::new(None),
            privacy_selection: PrivacySelection::Any,
            certainty_selection: CertaintySelection::Any,
            importance_selection: ImportanceSelection::Any,
        }),
        justification: Justification {
            testimony: Vec::new().into(),
            reasoning: Reasoning::new("retire universal-domain matching candidates".to_owned()),
        },
    }
}

#[test]
fn default_build_round_trips_meta_request_without_nota_text() {
    let request = Input::configure(ConfigureRequest::new(
        ArchiveDatabaseTarget::Default,
        None,
        None,
        None,
    ));

    let bytes = request
        .clone()
        .encode_request_frame(exchange())
        .expect("encode bound request");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&bytes).expect("decode bound request");

    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, request);
}

#[test]
fn default_build_round_trips_domain_all_imported_query_without_nota_text() {
    let request =
        Input::collect_removal_candidates(universal_domain_removal_candidate_collection());

    let bytes = request
        .clone()
        .encode_request_frame(exchange())
        .expect("encode bound request");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&bytes).expect("decode bound request");

    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, request);
}

#[test]
fn default_build_round_trips_public_intent_dependency_without_nota_text() {
    let request = SpiritInput::public_intent(universal_domain_scopes());

    let bytes = request
        .clone()
        .encode_request_frame(exchange())
        .expect("encode bound request");
    let (decoded_exchange, decoded) =
        signal_spirit::ContractMarker::decode_single_request(&bytes).expect("decode bound request");

    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded.route(), SpiritInputRoute::PublicIntent);
    assert_eq!(decoded, request);
    assert_eq!(
        OperationKind::from_input(&request),
        OperationKind::PublicIntent
    );
}

#[test]
fn generated_frames_enforce_the_meta_signal_spirit_contract_binding() {
    let bytes = Input::ObserveHead
        .encode_request_frame(exchange())
        .expect("encode bound request");
    let header = u64::from_le_bytes(bytes[..8].try_into().expect("short header"));

    assert_eq!(header as u32, 2);
    assert_eq!((header >> 32) as u16, 1);

    let mut wrong_contract = bytes.clone();
    wrong_contract[..4].copy_from_slice(&1_u32.to_le_bytes());
    wrong_contract.truncate(8);
    assert!(matches!(
        ContractMarker::decode_frame(&wrong_contract),
        Err(SignalFrameError::ContractMismatch {
            expected: 2,
            found: 1,
        })
    ));

    let mut wrong_revision = bytes;
    wrong_revision[4..6].copy_from_slice(&2_u16.to_le_bytes());
    wrong_revision.truncate(8);
    assert!(matches!(
        ContractMarker::decode_frame(&wrong_revision),
        Err(SignalFrameError::UnsupportedWireRevision {
            contract_id: 2,
            expected: 1,
            found: 2,
        })
    ));
}
