use meta_signal_spirit::{ByteViewable, Query, Restorable, Signal, Signalizable};

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
