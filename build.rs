use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "meta-signal-mind",
        "0.1.0",
        "META_SIGNAL_MIND_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
