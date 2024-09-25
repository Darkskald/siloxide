
mod sila_protogen {
    tonic::include_proto!("sila_protogen");
}

pub use sila_protogen::sila2::org::silastandard as sila_standard;
pub use sila_protogen::sila2::org::silastandard::core as core_features;
pub use sila_protogen::sila2::org::silastandard::examples as example_features;