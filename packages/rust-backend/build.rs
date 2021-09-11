fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../rubickbase/src/proto/rubick.proto")?;
    Ok(())
}
