use protos::WireContractFamily;
use schema_rust::build::{ContractCrateBuild, CrateName, SchemaVersion, UpdateEnvironmentVariable};

fn main() {
    ContractCrateBuild::from_environment(
        CrateName::new("meta-signal-mind"),
        SchemaVersion::new("0.1.0"),
        UpdateEnvironmentVariable::new("META_SIGNAL_MIND_UPDATE_SCHEMA_ARTIFACTS"),
        WireContractFamily::MetaSignalSpirit,
    )
    .expect_fresh();
}
