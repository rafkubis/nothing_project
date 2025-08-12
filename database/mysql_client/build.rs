fn main() -> Result<(), std::io::Error> {
    println!("build.rs");
    tonic_build::compile_protos("/workspace/interfaces/hello.proto")
}
