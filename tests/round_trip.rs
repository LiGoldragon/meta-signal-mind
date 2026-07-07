use meta_signal_mind::{
    AuthorityMode, ChoreographyMode, Configuration, ConfigurationRejected,
    ConfigurationRejectionReason, Frame, FrameBody,
    IntentSynchronizationMode, MetaMindReply, Operation, OperationKind, PolicyRevision,
    PolicySection, PolicySnapshot, RequestUnimplemented, UnimplementedReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    RootCode, SubReply, VariantCode, WireRoute,
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
        authority_mode: AuthorityMode::IssueOrders,
        choreography_mode: ChoreographyMode::Decide,
        intent_synchronization_mode: IntentSynchronizationMode::SummaryOnly,
    }
}

fn round_trip_request(request: Operation) -> Operation {
    let frame = request.into_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: MetaMindReply) -> MetaMindReply {
    let frame = Frame::new(
        WireRoute::new(RootCode::new(0), VariantCode::new(0)),
        FrameBody::Reply {
            exchange: exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
        },
    );
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
fn meta_mind_requests_round_trip() {
    let configure = Operation::configure(configuration());
    assert_eq!(round_trip_request(configure.clone()), configure);

    let inspect = Operation::inspect(PolicySection::All);
    assert_eq!(round_trip_request(inspect.clone()), inspect);
}

#[test]
fn meta_mind_replies_round_trip() {
    let configured = MetaMindReply::Configured(PolicyRevision::new(7));
    assert_eq!(round_trip_reply(configured.clone()), configured);

    let snapshot = MetaMindReply::PolicySnapshot(PolicySnapshot {
        policy_revision: PolicyRevision::new(7),
        configuration: configuration(),
    });
    assert_eq!(round_trip_reply(snapshot.clone()), snapshot);

    let rejected = MetaMindReply::ConfigurationRejected(ConfigurationRejected::new(
        ConfigurationRejectionReason::PolicyWouldBreakChoreography,
    ));
    assert_eq!(round_trip_reply(rejected.clone()), rejected);

    let unimplemented = MetaMindReply::RequestUnimplemented(RequestUnimplemented {
        operation_kind: OperationKind::Configure,
        unimplemented_reason: UnimplementedReason::NotBuiltYet,
    });
    assert_eq!(round_trip_reply(unimplemented.clone()), unimplemented);
}

#[cfg(feature = "dotos-text")]
#[test]
fn meta_mind_operations_encode_as_contract_local_dotos_heads() {
    use meta_signal_mind::Request;
    use dotos::{DotosEncode, DotosSource};

    let operation = Operation::inspect(PolicySection::All);
    let text = operation.into_request().to_dotos();

    assert_eq!(text, "Inspect.All");
    assert!(!text.contains("Mutate"));
    assert!(!text.contains("Match"));

    let decoded = DotosSource::new(&text).parse::<Request>().expect("decode");
    assert_eq!(decoded.payloads().head().kind(), OperationKind::Inspect);
}

#[test]
fn meta_mind_request_exposes_contract_owned_operation_kind() {
    let configure = Operation::configure(configuration());
    assert_eq!(configure.kind(), OperationKind::Configure);

    let inspect = Operation::inspect(PolicySection::Authority);
    assert_eq!(inspect.kind(), OperationKind::Inspect);
}
