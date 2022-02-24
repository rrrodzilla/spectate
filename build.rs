use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["src/spectate.proto"], &["src/"])?;
    //prost_build::compile_protos(&["src/spectate.proto"], &["src/"])?;
    Ok(())
}
