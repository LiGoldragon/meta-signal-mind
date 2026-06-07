//! OwnerSignal contract for privileged PersonaMind policy.
//!
//! Ordinary mind graph, work graph, and subscription traffic lives in
//! `signal-mind`. This crate carries owner-only policy and
//! configuration operations issued by PersonaSpirit.

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct PolicyRevision(u64);

impl PolicyRevision {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl NotaDecode for PolicyRevision {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        Ok(Self(NotaBlock::new(block).parse_integer()?))
    }
}

impl NotaEncode for PolicyRevision {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum AuthorityMode {
    ObserveOnly,
    ProposeOrders,
    IssueOrders,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ChoreographyMode {
    RecordOnly,
    Recommend,
    Decide,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum IntentSynchronizationMode {
    Disabled,
    SummaryOnly,
    FullRecord,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Configuration {
    pub authority: AuthorityMode,
    pub choreography: ChoreographyMode,
    pub intent_synchronization: IntentSynchronizationMode,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum PolicySection {
    Authority,
    Choreography,
    IntentSynchronization,
    All,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct Inspection {
    pub section: PolicySection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Configured {
    pub revision: PolicyRevision,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PolicySnapshot {
    pub revision: PolicyRevision,
    pub configuration: Configuration,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ConfigurationRejected {
    pub reason: ConfigurationRejectionReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ConfigurationRejectionReason {
    SpiritAuthorityRequired,
    PolicyWouldBreakChoreography,
    IntentSynchronizationUnavailable,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
    PolicyStoreUnavailable,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel OwnerMind {
        operation Configure(Configuration),
        operation Inspect(Inspection),
    }
    reply OwnerMindReply {
        Configured(Configured),
        PolicySnapshot(PolicySnapshot),
        ConfigurationRejected(ConfigurationRejected),
        RequestUnimplemented(RequestUnimplemented),
    }
}

impl From<Configuration> for Operation {
    fn from(payload: Configuration) -> Self {
        Self::Configure(payload)
    }
}

impl From<Inspection> for Operation {
    fn from(payload: Inspection) -> Self {
        Self::Inspect(payload)
    }
}
