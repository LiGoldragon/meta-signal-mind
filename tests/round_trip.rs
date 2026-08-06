use dotos::{DotosEncode, DotosSource};
use meta_signal_mind::*;
fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(1),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}
#[test]
fn authority_projected_request_round_trips_through_dotos_and_bound_frame() {
    let request = z2VU1V::z2VPpf(z2VaGZ {
        field_0: z2VdFT::z2VNyX,
        field_1: z2VSX6::z2VT4y,
        field_2: z2VYng::z2VZBx,
    });
    let text = request.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VU1V>()
            .expect("Dotos decodes"),
        request
    );
    let bytes = request
        .clone()
        .encode_request_frame(exchange())
        .expect("frame encodes");
    let (found_exchange, decoded) =
        ContractMarker::decode_single_request(&bytes).expect("frame decodes");
    assert_eq!(found_exchange, exchange());
    assert_eq!(decoded, request);
}
#[test]
fn authority_projected_reply_round_trips_through_dotos_and_archive() {
    let reply = z2VTar::z2VW97(z2Vaeg {
        field_0: z2VYqg::z2VPdd,
        field_1: z2VXPw::z2VWnx,
    });
    let text = reply.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VTar>()
            .expect("Dotos decodes"),
        reply
    );
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("archives");
    let recovered = rkyv::from_bytes::<z2VTar, rkyv::rancor::Error>(&bytes).expect("recovers");
    assert_eq!(recovered, reply);
}
