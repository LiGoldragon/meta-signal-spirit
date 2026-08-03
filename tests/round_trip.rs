#![cfg(feature = "nota-text")]

use meta_signal_spirit::{
    ArchiveDatabaseTarget, ConfigureReceipt, ConfigureRequest, CriomeGateTarget, CriomeSocketPath,
    CriomeSocketPathText, GuardianPrompt, GuardianPromptTarget, GuardianPromptText, HeadDigestHex,
    ImportReceipt, ImportedRecords, Input, MirrorAddress, MirrorAddressText, MirrorTarget, Output,
    SelectedHeadDigest, VersionedLogHead,
};
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::SignalOperationHeads;
use signal_spirit::{
    CommitSequence, DatabaseMarker, Domain, DomainScope, DomainScopes, Input as SpiritInput,
    InputRoute as SpiritInputRoute, OperationKind, RecordCount, StateDigest,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn database_marker() -> DatabaseMarker {
    DatabaseMarker {
        commit_sequence: CommitSequence::new(1),
        state_digest: StateDigest::new(2),
    }
}

fn universal_domain_scopes() -> DomainScopes {
    DomainScopes::new(vec![DomainScope::from(Domain::All)])
}

fn round_trip_input(input: Input) -> Input {
    let frame = input.encode_signal_frame().expect("encode input");
    let (_route, decoded) = Input::decode_signal_frame(&frame).expect("decode input");
    decoded
}

fn round_trip_output(output: Output) -> Output {
    let frame = output.encode_signal_frame().expect("encode output");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode output");
    decoded
}

fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);
    assert_eq!(
        NotaSource::new(&encoded)
            .parse::<Value>()
            .expect("decode text projection"),
        value
    );
    assert!(CANONICAL.contains(expected));
}

#[test]
fn meta_spirit_inputs_round_trip() {
    let inputs = [
        Input::configure(ConfigureRequest::new(
            ArchiveDatabaseTarget::Default,
            None,
            None,
            None,
        )),
        Input::configure(ConfigureRequest::new(
            ArchiveDatabaseTarget::Default,
            Some(MirrorTarget::Address(MirrorAddress::new(
                MirrorAddressText::new("100.64.0.7:7777"),
            ))),
            Some(CriomeGateTarget::Socket(CriomeSocketPath::new(
                CriomeSocketPathText::new("/run/user/1001/criome.sock"),
            ))),
            Some(GuardianPromptTarget::Prompt(GuardianPrompt::new(
                GuardianPromptText::new("You are the Guardian of Spirit. Default to refusal."),
            ))),
        )),
        Input::import(ImportedRecords::new(Vec::new()).into()),
        Input::ObserveHead,
        Input::ObserveHeadObject,
    ];

    for input in inputs {
        assert_eq!(round_trip_input(input.clone()), input);
    }
}

#[test]
fn meta_spirit_outputs_round_trip() {
    let outputs = [
        Output::configured(ConfigureReceipt::new(
            ArchiveDatabaseTarget::Default,
            None,
            None,
            None,
            database_marker(),
        )),
        Output::imported(ImportReceipt {
            record_count: RecordCount::new(0),
            database_marker: database_marker(),
        }),
        Output::head_observed(VersionedLogHead {
            database_marker: database_marker(),
            selected_head_digest: SelectedHeadDigest::new(Some(HeadDigestHex::new(
                "5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a",
            ))),
        }),
    ];

    for output in outputs {
        assert_eq!(round_trip_output(output.clone()), output);
    }
}

#[test]
fn revision_2_request_variants_are_contract_local_verbs() {
    assert_eq!(
        Input::HEADS,
        &["Configure", "Import", "ObserveHead", "ObserveHeadObject"]
    );
}

#[test]
fn intent_dependency_round_trips_from_the_exact_signal_revision() {
    let input = SpiritInput::intent(universal_domain_scopes());
    let frame = input.encode_signal_frame().expect("encode spirit input");
    let (route, decoded) = SpiritInput::decode_signal_frame(&frame).expect("decode spirit input");

    assert_eq!(route, SpiritInputRoute::Intent);
    assert_eq!(decoded, input);
    assert_eq!(OperationKind::from_input(&input), OperationKind::Intent);
}

#[test]
fn canonical_text_projection_round_trips() {
    round_trip_nota(
        Input::configure(ConfigureRequest::new(
            ArchiveDatabaseTarget::Default,
            None,
            None,
            None,
        )),
        "(Configure (Default None None None))",
    );
    round_trip_nota(
        Input::import(ImportedRecords::new(Vec::new()).into()),
        "(Import [])",
    );
    round_trip_nota(
        SpiritInput::intent(universal_domain_scopes()),
        "(Intent [All])",
    );
    round_trip_nota(
        Output::configured(ConfigureReceipt::new(
            ArchiveDatabaseTarget::Default,
            None,
            None,
            None,
            database_marker(),
        )),
        "(Configured (Default None None None (1 2)))",
    );
}

#[test]
fn revision_1_collection_syntax_fails_to_decode() {
    assert!(
        NotaSource::new("(CollectRemovalCandidates ignored)")
            .parse::<Input>()
            .is_err()
    );
    assert!(
        NotaSource::new("(RemovalCandidatesCollected ignored)")
            .parse::<Output>()
            .is_err()
    );
}
