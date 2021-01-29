#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error {0}")]
    IO(#[from] std::io::Error),

    #[error("Capnp error {0}")]
    Capnp(#[from] capnp::Error),

    #[error("System time error {0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error("JSON error {0}")]
    Json(#[from] serde_json::Error),
}
