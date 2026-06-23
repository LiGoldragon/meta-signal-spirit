#![cfg(feature = "nota-text")]

use meta_signal_spirit::{
    ArchiveDatabaseTarget, ConfigureReceipt, ConfigureRequest, CriomeGateTarget, CriomeSocketPath,
    CriomeSocketPathText, ImportReceipt, ImportedRecords, Input, MirrorAddress, MirrorAddressText,
    MirrorTarget, Output,
};
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::SignalOperationHeads;
use signal_spirit::schema::signal::{CommitSequence, DatabaseMarker, RecordCount, StateDigest};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn database_marker() -> DatabaseMarker {
    DatabaseMarker {
        commit_sequence: CommitSequence::new(1),
        state_digest: StateDigest::new(2),
    }
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

    let recovered = NotaSource::new(&encoded)
        .parse::<Value>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn meta_spirit_inputs_round_trip() {
    let inputs = [
        Input::configure(ConfigureRequest {
            archive_database_target: ArchiveDatabaseTarget::Default,
            selected_mirror_target: None.into(),
            selected_criome_gate_target: None.into(),
        }),
        Input::configure(ConfigureRequest {
            archive_database_target: ArchiveDatabaseTarget::Default,
            selected_mirror_target: Some(MirrorTarget::Address(MirrorAddress::new(
                MirrorAddressText::new("100.64.0.7:7777"),
            )))
            .into(),
            selected_criome_gate_target: Some(CriomeGateTarget::Socket(CriomeSocketPath::new(
                CriomeSocketPathText::new("/run/user/1001/criome.sock"),
            )))
            .into(),
        }),
        Input::import(ImportedRecords::new(Vec::new()).into()),
    ];

    for input in inputs {
        assert_eq!(round_trip_input(input.clone()), input);
    }
}

#[test]
fn meta_spirit_outputs_round_trip() {
    let outputs = [
        Output::configured(ConfigureReceipt {
            archive_database_target: ArchiveDatabaseTarget::Default,
            selected_mirror_target: None.into(),
            selected_criome_gate_target: None.into(),
            database_marker: database_marker(),
        }),
        Output::imported(ImportReceipt {
            record_count: RecordCount::new(0),
            database_marker: database_marker(),
        }),
    ];

    for output in outputs {
        assert_eq!(round_trip_output(output.clone()), output);
    }
}

#[test]
fn meta_spirit_request_variants_are_contract_local_verbs() {
    assert_eq!(Input::HEADS, &["Configure", "Import"]);
}

#[test]
fn meta_spirit_canonical_examples_round_trip() {
    round_trip_nota(
        Input::configure(ConfigureRequest {
            archive_database_target: ArchiveDatabaseTarget::Default,
            selected_mirror_target: None.into(),
            selected_criome_gate_target: None.into(),
        }),
        "(Configure (Default None None))",
    );
    round_trip_nota(
        Input::configure(ConfigureRequest {
            archive_database_target: ArchiveDatabaseTarget::Default,
            selected_mirror_target: Some(MirrorTarget::Address(MirrorAddress::new(
                MirrorAddressText::new("100.64.0.7:7777"),
            )))
            .into(),
            selected_criome_gate_target: Some(CriomeGateTarget::Socket(CriomeSocketPath::new(
                CriomeSocketPathText::new("/run/user/1001/criome.sock"),
            )))
            .into(),
        }),
        "(Configure (Default (Some (Address 100.64.0.7:7777)) (Some (Socket /run/user/1001/criome.sock))))",
    );
    round_trip_nota(
        Input::import(ImportedRecords::new(Vec::new()).into()),
        "(Import [])",
    );
    round_trip_nota(
        Output::configured(ConfigureReceipt {
            archive_database_target: ArchiveDatabaseTarget::Default,
            selected_mirror_target: None.into(),
            selected_criome_gate_target: None.into(),
            database_marker: database_marker(),
        }),
        "(Configured (Default None None (1 2)))",
    );
}
