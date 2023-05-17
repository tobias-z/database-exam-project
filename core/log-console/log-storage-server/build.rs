use std::io;

pub fn main() -> io::Result<()> {
    tonic_build::compile_protos("../contracts/logging-contract.proto")?;
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &["../../../lib/auth-proto/src/main/proto/schema.proto"],
            &["../../../lib/auth-proto/src/main/proto"],
        )?;
    Ok(())
}
