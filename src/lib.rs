//! Schema-derived meta signal contract for privileged PersonaMind policy.
//!
//! Ordinary mind graph, work graph, and subscription traffic lives in
//! `signal-mind`. This crate carries meta policy and
//! configuration operations issued by PersonaSpirit.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl PolicyRevision {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

impl Input {
    pub fn kind(&self) -> OperationKind {
        match self {
            Self::Configure(_) => OperationKind::Configure,
            Self::Inspect(_) => OperationKind::Inspect,
        }
    }
}

pub type Operation = Input;
pub type MetaMindReply = Output;
