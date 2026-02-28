fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["proto/characters.proto"],
        &["proto", "/usr/include", "/usr/local/include"],
    )?;
    Ok(())
}