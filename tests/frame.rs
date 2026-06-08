use meta_signal_spirit::{Frame, FrameBody, Generation, Operation, Reply, Start, Started};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SubReply,
};

#[test]
fn default_build_round_trips_meta_request_without_nota_text() {
    let request = Operation::Start(Start {
        generation: Generation::new(1),
    });
    let frame = Frame::new(FrameBody::Request {
        exchange: ExchangeIdentifier::new(
            SessionEpoch::new(1),
            ExchangeLane::Connector,
            LaneSequence::first(),
        ),
        request: request.clone().into_request(),
    });

    let bytes = frame.encode_length_prefixed().expect("encode request");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");

    let FrameBody::Request {
        request: decoded, ..
    } = decoded.into_body()
    else {
        panic!("expected request frame");
    };
    assert_eq!(decoded.payloads().head(), &request);
}

#[test]
fn default_build_round_trips_meta_reply_without_nota_text() {
    let reply = Reply::Started(Started {
        generation: Generation::new(1),
    });
    let frame = Frame::new(FrameBody::Reply {
        exchange: ExchangeIdentifier::new(
            SessionEpoch::new(1),
            ExchangeLane::Connector,
            LaneSequence::first(),
        ),
        reply: FrameReply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });

    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");

    let FrameBody::Reply { reply: decoded, .. } = decoded.into_body() else {
        panic!("expected reply frame");
    };
    let FrameReply::Accepted { per_operation, .. } = decoded else {
        panic!("expected accepted reply");
    };
    assert_eq!(per_operation.into_head(), SubReply::Ok(reply));
}
