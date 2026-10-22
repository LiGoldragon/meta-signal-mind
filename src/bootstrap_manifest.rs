//! Producer-owned authority state for the Mind meta-policy Interface.
//!
//! Every identity and canonical-order value is an allocated opaque seat.
//! None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    32, 115, 250, 228, 22, 238, 110, 213, 162, 97, 94, 22, 6, 69, 167, 192, 203, 95, 195, 146, 66,
    158, 62, 15, 233, 126, 96, 106, 192, 188, 218, 242,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 53266;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 53185;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 36447, 0x56fa3e99f9eebfc6);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 48576, 0x77836c853c39c668);
pub const MEMORY_SEAT: AuthoritySeat = AuthoritySeat::new("Memory", 37252, 0x1f9ae8e49c7c49da);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 54725, 0x541f758f0a87f69c);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 7537, 0x6f68b00159454d2e);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 40357, 0x891c52bee4a7c210);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 5498, 0xd193edec767eddc2);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 26017, 0x0ff37adcbb6c5cc4);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 55693, 0xa5161d8cea874f96);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 11257, 0x954f03a175743ab8);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 4879, 0xedbfd0fc7bfa36aa);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 11379, 0x353f8635a55d0fec);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 34821, 0x5b154508bb0466fe);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 24603, 0x56d921fe3b37d060);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 55414, 0xcd8ad537e7f7f492);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 29003, 0x67ebc590653cb014);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    18960, 6944, 59374, 5055, 1545, 16853, 37376, 54369, 27976, 50386,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "MindMetaRequest", 27736, 0x7ee2a65ff88ad2b0),
    DeclarationSeat::new(None, "MindMetaReply", 26307, 0x74976a1eebc51f62),
    DeclarationSeat::new(None, "Configure", 36777, 0xa37eedca3b173764),
    DeclarationSeat::new(None, "Inspect", 52266, 0xa9238b8752266b36),
    DeclarationSeat::new(None, "AuthorityMode", 58822, 0x0d9850cc594f7f58),
    DeclarationSeat::new(None, "ChoreographyMode", 22725, 0x10569901014bcc4a),
    DeclarationSeat::new(None, "IntentSynchronizationMode", 43813, 0xa98a19d3531b5e8c),
    DeclarationSeat::new(None, "Configuration", 48794, 0xc9f5bad203bc169e),
    DeclarationSeat::new(None, "PolicySection", 165, 0xc659d5e04675c900),
    DeclarationSeat::new(None, "Inspection", 50642, 0xccc42a6385c35e32),
    DeclarationSeat::new(None, "PolicySnapshotPayload", 4499, 0x04738b2f1a20f2b4),
    DeclarationSeat::new(
        None,
        "ConfigurationRejectionReason",
        39517,
        0x746b9fdad746f706,
    ),
    DeclarationSeat::new(
        None,
        "ConfigurationRejectedPayload",
        37354,
        0x69e7b9216b9a4fa8,
    ),
    DeclarationSeat::new(None, "OperationKind", 43987, 0x276ce105e2d9751a),
    DeclarationSeat::new(None, "UnimplementedReason", 39130, 0xf828d21b744e93dc),
    DeclarationSeat::new(
        None,
        "RequestUnimplementedPayload",
        50077,
        0x861f2e57ee0eac6e,
    ),
    DeclarationSeat::new(None, "PolicyRevision", 7052, 0x8243fe73da0db350),
    DeclarationSeat::new(Some(27736), "Configure", 13652, 0x5d43a3d8b70fb102),
    DeclarationSeat::new(Some(27736), "Inspect", 35875, 0xb9dd06be9ebee204),
    DeclarationSeat::new(Some(26307), "Configured", 19817, 0x6f55b4e1436ed6d6),
    DeclarationSeat::new(Some(26307), "PolicySnapshot", 24837, 0x6a7be6d6a35493f8),
    DeclarationSeat::new(
        Some(26307),
        "ConfigurationRejected",
        25093,
        0xc4be878ef43bb1ea,
    ),
    DeclarationSeat::new(
        Some(26307),
        "RequestUnimplemented",
        34906,
        0x4a1c195872007d2c,
    ),
    DeclarationSeat::new(Some(58822), "ObserveOnly", 10802, 0xcd8bc936b357163e),
    DeclarationSeat::new(Some(58822), "ProposeOrders", 3029, 0xe4d1700443a791a0),
    DeclarationSeat::new(Some(58822), "IssueOrders", 16186, 0xd4e15d174e0717d2),
    DeclarationSeat::new(Some(22725), "RecordOnly", 26801, 0xfbf1c7f474960554),
    DeclarationSeat::new(Some(22725), "Recommend", 60495, 0x677f135350cb0aa6),
    DeclarationSeat::new(Some(22725), "Decide", 24574, 0x5e050c00dc734c48),
    DeclarationSeat::new(Some(43813), "Disabled", 45163, 0x500e0d17f86f82ba),
    DeclarationSeat::new(Some(43813), "SummaryOnly", 17181, 0x5d3d6b46bd761a7c),
    DeclarationSeat::new(Some(43813), "FullRecord", 5753, 0xe42a0d080c62540e),
    DeclarationSeat::new(Some(165), "Authority", 137, 0x97fc08e03ed863f0),
    DeclarationSeat::new(Some(165), "Choreography", 51125, 0x6c443dc6b64692a2),
    DeclarationSeat::new(
        Some(165),
        "IntentSynchronization",
        29911,
        0x8471839da68b5ca4,
    ),
    DeclarationSeat::new(Some(165), "All", 64592, 0xa20fd517aac89276),
    DeclarationSeat::new(
        Some(39517),
        "SpiritAuthorityRequired",
        50175,
        0xe102cc4dc62b7898,
    ),
    DeclarationSeat::new(
        Some(39517),
        "PolicyWouldBreakChoreography",
        44631,
        0xee61ddb737b1e78a,
    ),
    DeclarationSeat::new(
        Some(39517),
        "IntentSynchronizationUnavailable",
        48860,
        0xa0c4ce124f346bcc,
    ),
    DeclarationSeat::new(Some(43987), "Configure", 13012, 0xbad5a60e0b3d65de),
    DeclarationSeat::new(Some(43987), "Inspect", 39153, 0xe65059a002752a40),
    DeclarationSeat::new(Some(39130), "NotBuiltYet", 7477, 0x831a3379c8ab2172),
    DeclarationSeat::new(Some(39130), "DependencyNotReady", 29293, 0xadefb161eec3e7f4),
    DeclarationSeat::new(
        Some(39130),
        "PolicyStoreUnavailable",
        37101,
        0x78f15a4cbe146e46,
    ),
];
