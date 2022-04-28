use crate::core::reader::ReaderError;
use tokio::sync::mpsc::error::SendError as TSendError;
use std::array::TryFromSliceError;
use tokio::time::error::Elapsed;
use std::sync::mpsc::SendError;
use std::io::Error as IOError;
use std::sync::PoisonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum P2pError {
    #[error("Peer disconnected")]
    Disconnected,
    #[error("Invalid handshake")]
    InvalidHandshake,
    #[error("Expected Handshake packet")]
    ExpectedHandshake,
    #[error("Invalid peer address, {}", _0)]
    InvalidPeerAddress(String), // peer address from handshake
    #[error("Invalid network ID")]
    InvalidNetworkID,
    #[error("Peer id {} is already used!", _0)]
    PeerIdAlreadyUsed(u64),
    #[error("Peer already connected: {}", _0)]
    PeerAlreadyConnected(String),
    #[error(transparent)]
    ErrorStd(#[from] IOError),
    #[error("Poison Error: {}", _0)]
    PoisonError(String),
    #[error("Send Error: {}", _0)]
    SendError(String),
    #[error(transparent)]
    TryInto(#[from] TryFromSliceError),
    #[error(transparent)]
    ReaderError(#[from] ReaderError),
    #[error("Invalid packet ID")]
    InvalidPacket,
    #[error("Packet size exceed limit")]
    InvalidPacketSize,
    #[error("Received valid packet with not used bytes")]
    InvalidPacketNotFullRead,
    #[error("Request sync chain too fast")]
    RequestSyncChainTooFast,
    #[error(transparent)]
    AsyncTimeOut(#[from] Elapsed)
}

impl<T> From<PoisonError<T>> for P2pError {
    fn from(err: PoisonError<T>) -> Self {
        Self::PoisonError(format!("{}", err))
    }
}

impl<T> From<SendError<T>> for P2pError {
    fn from(err: SendError<T>) -> Self {
        Self::SendError(format!("{}", err))
    }
}

impl<T> From<TSendError<T>> for P2pError {
    fn from(err: TSendError<T>) -> Self {
        Self::SendError(format!("{}", err))
    }
}