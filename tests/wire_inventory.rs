use meta_signal_spirit::{
    Input, InputRoute, META_SIGNAL_RUST_SOURCE, META_SIGNAL_SCHEMA_SOURCE, MetaReply, MetaRequest,
    Output, OutputRoute, schema::meta_signal::short_header,
};
use signal_frame::SignalOperationHeads;
use std::marker::PhantomData;

const INPUT_ROUTES: [(&str, InputRoute); 5] = [
    ("Configure", InputRoute::Configure),
    ("Import", InputRoute::Import),
    (
        "CollectRemovalCandidates",
        InputRoute::CollectRemovalCandidates,
    ),
    ("ObserveHead", InputRoute::ObserveHead),
    ("ObserveHeadObject", InputRoute::ObserveHeadObject),
];

const OUTPUT_ROUTES: [(&str, OutputRoute); 6] = [
    ("Configured", OutputRoute::Configured),
    ("Imported", OutputRoute::Imported),
    (
        "RemovalCandidatesCollected",
        OutputRoute::RemovalCandidatesCollected,
    ),
    ("Rejected", OutputRoute::Rejected),
    ("HeadObserved", OutputRoute::HeadObserved),
    ("HeadObjectObserved", OutputRoute::HeadObjectObserved),
];

const INPUT_HEADERS: [u64; 5] = [
    short_header::INPUT_CONFIGURE,
    short_header::INPUT_IMPORT,
    short_header::INPUT_COLLECT_REMOVAL_CANDIDATES,
    short_header::INPUT_OBSERVE_HEAD,
    short_header::INPUT_OBSERVE_HEAD_OBJECT,
];

const OUTPUT_HEADERS: [u64; 6] = [
    short_header::OUTPUT_CONFIGURED,
    short_header::OUTPUT_IMPORTED,
    short_header::OUTPUT_REMOVAL_CANDIDATES_COLLECTED,
    short_header::OUTPUT_REJECTED,
    short_header::OUTPUT_HEAD_OBSERVED,
    short_header::OUTPUT_HEAD_OBJECT_OBSERVED,
];

#[test]
fn complete_owner_route_header_and_archive_tag_inventory_is_stable() {
    assert_eq!(
        Input::HEADS,
        INPUT_ROUTES
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
    );

    for (index, ((_, route), header)) in INPUT_ROUTES.iter().zip(INPUT_HEADERS).enumerate() {
        let expected_header = ((index as u64) << 48) | 0x0000_0001_0000_0002;
        assert_eq!(
            header, expected_header,
            "Input route header at index {index} moved"
        );
        let archived =
            rkyv::to_bytes::<rkyv::rancor::Error>(route).expect("archive Input route tag");
        assert_eq!(
            archived.as_ref(),
            &[index as u8],
            "Input route tag at index {index} moved"
        );
    }

    for (index, ((_, route), header)) in OUTPUT_ROUTES.iter().zip(OUTPUT_HEADERS).enumerate() {
        let expected_header = ((0x0100_u64 + index as u64) << 48) | 0x0000_0001_0000_0002;
        assert_eq!(
            header, expected_header,
            "Output route header at index {index} moved"
        );
        let archived =
            rkyv::to_bytes::<rkyv::rancor::Error>(route).expect("archive Output route tag");
        assert_eq!(
            archived.as_ref(),
            &[index as u8],
            "Output route tag at index {index} moved"
        );
    }
}

fn same_type<Type>(_: PhantomData<Type>, _: PhantomData<Type>) {}

#[test]
fn established_meta_request_and_reply_names_remain_exact_aliases() {
    same_type(PhantomData::<MetaRequest>, PhantomData::<Input>);
    same_type(PhantomData::<MetaReply>, PhantomData::<Output>);
}

fn root_variant_names(root: &schema_language::SourceRootEnum) -> Vec<&str> {
    root.body()
        .as_enum()
        .expect("wire root remains an enum")
        .variants()
        .iter()
        .map(|variant| variant.name().as_str())
        .collect()
}

#[test]
fn authored_roots_generated_heads_and_owner_boundary_are_closed() {
    let meta_source = schema_language::SchemaSource::from_schema_text(META_SIGNAL_SCHEMA_SOURCE)
        .expect("decode authored meta signal schema");
    let meta_input_names = root_variant_names(meta_source.input());
    let meta_output_names = root_variant_names(meta_source.output());

    assert_eq!(
        meta_input_names,
        INPUT_ROUTES
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        meta_output_names,
        OUTPUT_ROUTES
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
    );
    assert_eq!(Input::HEADS, meta_input_names);

    let ordinary_source =
        schema_language::SchemaSource::from_schema_text(signal_spirit::SIGNAL_SCHEMA_SOURCE)
            .expect("decode ordinary signal schema");
    let ordinary_input_names = root_variant_names(ordinary_source.input());
    let ordinary_output_names = root_variant_names(ordinary_source.output());

    for owner_only_input in [
        "Configure",
        "Import",
        "CollectRemovalCandidates",
        "ObserveHead",
        "ObserveHeadObject",
        "Remove",
    ] {
        assert!(
            !ordinary_input_names.contains(&owner_only_input),
            "ordinary working socket acquired owner-only input {owner_only_input}"
        );
    }
    for owner_only_output in [
        "Configured",
        "Imported",
        "RemovalCandidatesCollected",
        "HeadObserved",
        "HeadObjectObserved",
    ] {
        assert!(
            !ordinary_output_names.contains(&owner_only_output),
            "ordinary working socket acquired owner-only output {owner_only_output}"
        );
    }

    for retired_root in ["Watch", "Unwatch", "Remove", "Stream", "Family"] {
        assert!(!meta_input_names.contains(&retired_root));
        assert!(!meta_output_names.contains(&retired_root));
        assert!(!ordinary_input_names.contains(&retired_root));
        assert!(!ordinary_output_names.contains(&retired_root));
    }

    assert!(
        META_SIGNAL_SCHEMA_SOURCE.contains(
            "signal-spirit.signal.[\n    DatabaseMarker\n    Entry\n    RecordIdentifier"
        ),
        "meta schema must import shared nouns from the exact signal-spirit module"
    );
    assert!(
        META_SIGNAL_RUST_SOURCE.starts_with("// @generated by schema-rust"),
        "checked-in wire source must remain generator-owned"
    );
}
