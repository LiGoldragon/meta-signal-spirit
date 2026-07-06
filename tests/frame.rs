use meta_signal_spirit::{ArchiveDatabaseTarget, ConfigureRequest, Input};
use signal_spirit::{
    CertaintySelection, Domain, DomainMatch, DomainScope, DomainScopes, ImportanceSelection,
    Justification, KeywordMatch, PrivacySelection, Query, Reasoning, RecordQuery,
    ReferentSelection, RemovalCandidateCollection, SelectedKind, TextMatch,
};

fn universal_domain_removal_candidate_collection() -> RemovalCandidateCollection {
    RemovalCandidateCollection {
        record_query: RecordQuery::new(Query {
            domain_match: DomainMatch::partial(DomainScopes::new(vec![DomainScope::from(
                Domain::All,
            )])),
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

    let bytes = request.encode_signal_frame().expect("encode request");
    let (_route, decoded) = Input::decode_signal_frame(&bytes).expect("decode request");

    assert_eq!(decoded, request);
}

#[test]
fn default_build_round_trips_domain_all_imported_query_without_nota_text() {
    let request =
        Input::collect_removal_candidates(universal_domain_removal_candidate_collection().into());

    let bytes = request.encode_signal_frame().expect("encode request");
    let (_route, decoded) = Input::decode_signal_frame(&bytes).expect("decode request");

    assert_eq!(decoded, request);
}
