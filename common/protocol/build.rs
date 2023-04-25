fn main() {
    println!("cargo:rerun-if-changed=proto/**/*.proto");
    // compile protos
    tonic_build::configure()
        .compile(&["proto/howlapp/v1/hello.proto"], &["proto"])
        .expect("failed to compile protos");
}
