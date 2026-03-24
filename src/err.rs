#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Protocol
    #[error("Invalid ACK")]
    InvalidAck,
    #[error("Device didn't accept setup data :(")]
    StageSetupNotAccepted,
    #[error("Device didn't accept image :(")]
    StageNotAccepted,
    #[error("Device didn't accept jump data :(")]
    JumpNotAccepted,
    // USB
    #[error("{0}")]
    NUsb(#[from] nusb::Error),
    // IO
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("Port: {0}")]
    Port(#[from] simpleport::err::Error),
}
