#[derive(Debug)]
pub enum RaknetError {
    SetRaknetRawSocketError,
    NotListen,
    BindAddressError,
    ConnectionClosed,
    NotSupportVersion,
    IncorrectReply,
    PacketParseError,
    SocketError,
    IncorrectReliability,
    IncorrectPacketID,
    ReadPacketBufferError,
    PacketSizeExceedMTU,
    PacketHeaderError,
}

pub type Result<T> = std::result::Result<T, RaknetError>;
