#[cfg(feature = "hello")]
pub mod hello {
    tonic::include_proto!("macrostack.v1.hello");
}
