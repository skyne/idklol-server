fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &["proto/chat.proto"],
        &["proto", "/usr/include", "/usr/local/include"],
    )?;
    Ok(())
}