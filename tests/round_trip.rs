use owner_signal_persona_mind::{
    AuthorityMode, ChoreographyMode, Configuration, ConfigurationRejected,
    ConfigurationRejectionReason, Configured, Frame, FrameBody, Inspection,
    IntentSynchronizationMode, Operation, OperationKind, OwnerMindReply, PolicyRevision,
    PolicySection, PolicySnapshot, Request, RequestUnimplemented, UnimplementedReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn configuration() -> Configuration {
    Configuration {
        authority: AuthorityMode::IssueOrders,
        choreography: ChoreographyMode::Decide,
        intent_synchronization: IntentSynchronizationMode::SummaryOnly,
    }
}

fn round_trip_request(request: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: OwnerMindReply) -> OwnerMindReply {
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

#[test]
fn owner_mind_requests_round_trip() {
    let configure = Operation::Configure(configuration());
    assert_eq!(round_trip_request(configure.clone()), configure);

    let inspect = Operation::Inspect(Inspection {
        section: PolicySection::All,
    });
    assert_eq!(round_trip_request(inspect.clone()), inspect);
}

#[test]
fn owner_mind_replies_round_trip() {
    let configured = OwnerMindReply::Configured(Configured {
        revision: PolicyRevision::new(7),
    });
    assert_eq!(round_trip_reply(configured.clone()), configured);

    let snapshot = OwnerMindReply::PolicySnapshot(PolicySnapshot {
        revision: PolicyRevision::new(7),
        configuration: configuration(),
    });
    assert_eq!(round_trip_reply(snapshot.clone()), snapshot);

    let rejected = OwnerMindReply::ConfigurationRejected(ConfigurationRejected {
        reason: ConfigurationRejectionReason::PolicyWouldBreakChoreography,
    });
    assert_eq!(round_trip_reply(rejected.clone()), rejected);

    let unimplemented = OwnerMindReply::RequestUnimplemented(RequestUnimplemented {
        operation: OperationKind::Configure,
        reason: UnimplementedReason::NotBuiltYet,
    });
    assert_eq!(round_trip_reply(unimplemented.clone()), unimplemented);
}

#[test]
fn owner_mind_operations_encode_as_contract_local_nota_heads() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

    let operation = Operation::Inspect(Inspection {
        section: PolicySection::All,
    });
    let mut encoder = Encoder::new();
    operation
        .into_request()
        .encode(&mut encoder)
        .expect("encode");
    let text = encoder.into_string();

    assert_eq!(text, "(Inspect (All))");
    assert!(!text.contains("Mutate"));
    assert!(!text.contains("Match"));

    let mut decoder = Decoder::new(&text);
    let decoded = Request::decode(&mut decoder).expect("decode");
    assert_eq!(decoded.payloads().head().kind(), OperationKind::Inspect);
}

#[test]
fn owner_mind_request_exposes_contract_owned_operation_kind() {
    let configure = Operation::Configure(configuration());
    assert_eq!(configure.kind(), OperationKind::Configure);

    let inspect = Operation::Inspect(Inspection {
        section: PolicySection::Authority,
    });
    assert_eq!(inspect.kind(), OperationKind::Inspect);
}
