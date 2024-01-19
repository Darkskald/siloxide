fn main() -> Result<(), Box<dyn std::error::Error>> {

    tonic_build::configure()
        .include_file("sila_codegen.rs")
        .compile(
            &[
                 "./proto/SiLAService-v1_0.sila.proto",
                "./proto/GreetingProvider-v1_0.sila.proto"
            ], &["./proto"]
        )?;

    Ok(())
}