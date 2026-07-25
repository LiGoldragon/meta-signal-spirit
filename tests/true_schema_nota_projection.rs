//! Direct structured NOTA projection for the decoded Spirit meta-signal TrueSchema.
//!
//! This proves the authored owner-only Spirit meta contract lowers through
//! `schema-language` to the semantic `TrueSchema` model and that shared ordinary
//! Spirit nouns are resolved from dependency schemas rather than duplicated here.

#![cfg(feature = "nota-text")]

use meta_signal_spirit::META_SIGNAL_SCHEMA_SOURCE;
use nota::{Document, NotaDecode, NotaEncode};
use schema_language::{ImportResolver, SchemaEngine, SchemaIdentity, SchemaSource, TrueSchema};
use signal_spirit::{DOMAIN_SCHEMA_SOURCE, SIGNAL_SCHEMA_SOURCE};

#[derive(Clone, Debug, Eq, PartialEq)]
struct DecodedMetaSignalSchema {
    meta_signal: TrueSchema,
}

impl DecodedMetaSignalSchema {
    fn from_authored_source() -> Self {
        let engine = SchemaEngine::default();
        let resolver = ImportResolver::new()
            .with_module_source("signal-domain", "domain", "0.1.0", DOMAIN_SCHEMA_SOURCE)
            .with_module_source("signal-spirit", "signal", "0.13.0", SIGNAL_SCHEMA_SOURCE);
        let source = SchemaSource::from_schema_text(META_SIGNAL_SCHEMA_SOURCE)
            .expect("meta signal schema source decodes");
        let meta_signal = engine
            .lower_schema_source_with_resolver(
                &source,
                SchemaIdentity::new("meta-signal-spirit:meta-signal", env!("CARGO_PKG_VERSION")),
                &resolver,
            )
            .expect("meta signal schema lowers to TrueSchema");
        Self { meta_signal }
    }
}

#[test]
fn decoded_meta_signal_true_schema_projects_to_structured_nota() {
    let source = SchemaSource::from_schema_text(META_SIGNAL_SCHEMA_SOURCE)
        .expect("meta signal schema source decodes");
    let input_names = source
        .input()
        .body()
        .as_enum()
        .expect("Input root remains an enum")
        .variants()
        .iter()
        .map(|variant| variant.name().as_str())
        .collect::<Vec<_>>();
    let output_names = source
        .output()
        .body()
        .as_enum()
        .expect("Output root remains an enum")
        .variants()
        .iter()
        .map(|variant| variant.name().as_str())
        .collect::<Vec<_>>();
    let declaration_names = source
        .types()
        .entries()
        .iter()
        .map(|entry| entry.name().as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        input_names,
        [
            "Configure",
            "Import",
            "CollectRemovalCandidates",
            "ObserveHead",
            "ObserveHeadObject",
        ]
    );
    assert_eq!(
        output_names,
        [
            "Configured",
            "Imported",
            "RemovalCandidatesCollected",
            "Rejected",
            "HeadObserved",
            "HeadObjectObserved",
        ]
    );
    assert!(declaration_names.contains(&"ConfigureRequest"));
    assert!(
        !declaration_names.contains(&"Entry"),
        "meta signal must import Entry rather than redeclare it"
    );

    let schema = DecodedMetaSignalSchema::from_authored_source();
    let rendered = schema.meta_signal.to_nota();

    let expected_prefix = format!(
        "{{{{meta-signal-spirit:meta-signal {}}} [",
        env!("CARGO_PKG_VERSION")
    );
    let prefix_excerpt = rendered.chars().take(256).collect::<String>();
    assert!(
        rendered.starts_with(&expected_prefix),
        "structured TrueSchema NOTA should begin with the meta identity; prefix was {prefix_excerpt}"
    );
    assert!(
        rendered.contains("{DatabaseMarker (Plain signal-spirit:signal:DatabaseMarker)}")
            && rendered.contains("{Entry (Plain signal-spirit:signal:Entry)}"),
        "structured TrueSchema NOTA should retain resolved signal-spirit imports"
    );

    let document = Document::parse(&rendered).expect("structured meta TrueSchema NOTA parses");
    assert_eq!(
        document.holds_root_objects(),
        1,
        "TrueSchema projection should be one NOTA root object"
    );
    let decoded = TrueSchema::from_nota_block(&document.root_objects()[0])
        .expect("structured meta TrueSchema NOTA decodes");
    assert_eq!(
        decoded, schema.meta_signal,
        "structured meta NOTA projection should decode back to the same TrueSchema"
    );
}
