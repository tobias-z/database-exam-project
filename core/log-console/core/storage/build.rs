use std::io;

pub fn main() -> io::Result<()> {
    tonic_build::compile_protos("../../contracts/logging-contract.proto")?;
    Ok(())
}
