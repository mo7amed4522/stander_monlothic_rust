fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/grpc")
        .compile(
            &["src/proto/user-services.proto"],
            &["src/proto"],
        )?;
    
    println!("cargo:rerun-if-changed=src/proto/user-services.proto");
    Ok(())
}