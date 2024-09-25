use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .include_file("sila_protogen.rs")
        .compile(
            &[
                "proto/SiLABinaryTransfer.proto",
                "proto/SiLAFramework.proto",
                "proto/SiLACloudConnector.proto",
                "proto/features/core/SiLAService-v1_0.sila.proto",
                "proto/features/examples/GreetingProvider-v1_0.sila.proto"
            ], &["./proto"],
        )?;

    Ok(())
}