use meta_signal_spirit::{ArchiveDatabaseTarget, ConfigureRequest, Input};

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
