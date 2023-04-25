#[cfg(feature = "hello")]
pub mod hello {
    tonic::include_proto!("howlapp.v1.hello");
}
