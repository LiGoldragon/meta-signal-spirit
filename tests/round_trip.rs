use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_persona_spirit::{
    BootstrapPolicy, BootstrapPolicyReloaded, Drain, DrainedAndStopped, Frame, FrameBody,
    Generation, IdentityName, IdentityRegistered, IdentityRetired, OperationKind, OwnerSpiritReply,
    OwnerSpiritRequest, Registration, RequestUnimplemented, Retirement, Start, Started,
    UnimplementedReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn registration() -> Registration {
    Registration {
        name: IdentityName::new("author"),
    }
}

fn retirement() -> Retirement {
    Retirement {
        name: IdentityName::new("author"),
    }
}

fn round_trip_request(request: OwnerSpiritRequest) -> OwnerSpiritRequest {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: OwnerSpiritReply) -> OwnerSpiritReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota text");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = Value::decode(&mut decoder).expect("decode nota text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn owner_spirit_requests_round_trip() {
    let requests = [
        OwnerSpiritRequest::Start(Start {
            generation: Generation::new(1),
        }),
        OwnerSpiritRequest::Drain(Drain {}),
        OwnerSpiritRequest::Reload(BootstrapPolicy {}),
        OwnerSpiritRequest::Register(registration()),
        OwnerSpiritRequest::Retire(retirement()),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn owner_spirit_replies_round_trip() {
    let replies = [
        OwnerSpiritReply::Started(Started {
            generation: Generation::new(1),
        }),
        OwnerSpiritReply::DrainedAndStopped(DrainedAndStopped {}),
        OwnerSpiritReply::BootstrapPolicyReloaded(BootstrapPolicyReloaded {}),
        OwnerSpiritReply::IdentityRegistered(IdentityRegistered {
            name: IdentityName::new("author"),
        }),
        OwnerSpiritReply::IdentityRetired(IdentityRetired {
            name: IdentityName::new("author"),
        }),
        OwnerSpiritReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::Start,
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn owner_spirit_request_variants_are_contract_local_verbs() {
    let cases = [
        (
            OwnerSpiritRequest::Start(Start {
                generation: Generation::new(1),
            }),
            OperationKind::Start,
        ),
        (OwnerSpiritRequest::Drain(Drain {}), OperationKind::Drain),
        (
            OwnerSpiritRequest::Reload(BootstrapPolicy {}),
            OperationKind::Reload,
        ),
        (
            OwnerSpiritRequest::Register(registration()),
            OperationKind::Register,
        ),
        (
            OwnerSpiritRequest::Retire(retirement()),
            OperationKind::Retire,
        ),
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn owner_spirit_request_heads_have_no_universal_verb_wrapper() {
    round_trip_nota(
        OwnerSpiritRequest::Start(Start {
            generation: Generation::new(1),
        }),
        "(Start (1))",
    );
    round_trip_nota(OwnerSpiritRequest::Drain(Drain {}), "(Drain ())");
    round_trip_nota(
        OwnerSpiritRequest::Reload(BootstrapPolicy {}),
        "(Reload ())",
    );
    round_trip_nota(
        OwnerSpiritRequest::Register(registration()),
        "(Register (author))",
    );
    round_trip_nota(
        OwnerSpiritRequest::Retire(retirement()),
        "(Retire (author))",
    );
}

#[test]
fn owner_spirit_canonical_examples_round_trip() {
    round_trip_nota(
        OwnerSpiritReply::Started(Started {
            generation: Generation::new(1),
        }),
        "(Started (1))",
    );
    round_trip_nota(
        OwnerSpiritReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::Start,
            reason: UnimplementedReason::NotBuiltYet,
        }),
        "(RequestUnimplemented (Start NotBuiltYet))",
    );
}
