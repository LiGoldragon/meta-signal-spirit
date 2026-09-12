use meta_signal_spirit::Query;
use signal::{ByteViewable, Restorable, Signal, Signalizable};

#[test]
fn observe_head_round_trips_through_fresh_signal_bytes() {
    let query = Query::ObserveHead;
    let received = Signal::<Query>::from(query.signalize().expect("signalize").bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), query);
}

#[cfg(feature = "datom")]
#[test]
fn observe_head_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::ObserveHead;
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(text);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget { remaining: 1024 },
                depth: 0,
                maximum_depth: 1024
            })
            .expect("actualize"),
        query
    );
}

fn daemon_configuration() -> signal_spirit::SpiritNexusConfiguration {
    signal_spirit::SpiritNexusConfiguration {
        socket_path: "/run/spirit/ordinary.sock".into(),
        optional_meta_socket_path: Some("/run/spirit/meta.sock".into()),
        optional_trace_socket_path: None,
        authorization_mode: signal_spirit::AuthorizationMode::Gating,
        optional_spirit_guardian_agent_configuration: None,
    }
}

#[test]
fn meta_configuration_and_reversal_use_fresh_signal_bytes() {
    let configure = Query::Configure(meta_signal_spirit::ConfigureRequest {
        spirit_nexus_configuration: daemon_configuration(),
        archive_database_target: meta_signal_spirit::ArchiveDatabaseTarget::Default,
        selected_mirror_target: None,
        selected_criome_gate_target: None,
        selected_guardian_prompt_target: None,
    });
    let received = Signal::<Query>::from(
        configure
            .signalize()
            .expect("signalize meta configuration")
            .bytes()
            .to_vec(),
    );
    assert_eq!(
        received.restore().expect("restore meta configuration"),
        configure
    );

    let reversal = Query::ReverseMetaConfiguration;
    let received = Signal::<Query>::from(
        reversal
            .signalize()
            .expect("signalize reversal")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("restore reversal"), reversal);
}

#[cfg(feature = "datom")]
#[test]
fn meta_reversal_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = Query::ReverseMetaConfiguration;
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(text);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget { remaining: 1024 },
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("actualize reversal"),
        query
    );
}
