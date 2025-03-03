#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
//! Beacon consensus implementation.

mod beacon_consensus;
mod builder;

pub use beacon_consensus::BeaconConsensus;
pub use builder::BeaconConsensusBuilder;
