use std::io;

pub fn main() -> io::Result<()> {
    tonic_build::compile_protos("../contracts/logging-contract.proto")?;
    tonic_build::compile_protos("../../../lib/auth-proto/src/main/proto/schema.proto")?;
    Ok(())
}
