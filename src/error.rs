use std::io;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum PngError{
    #[error("Invalid chunk type")]
    ChunkTypeError,

    #[error("Invalid chunk block")]
    ChunkError,

    #[error("CRC rrror")]
    CRCError,

    #[error("Chunk header error")]
    ChunkHeaderError,

    #[error("No such chunk type")]
    NotFoundChunkType,
    
    #[error("IO error")]
    IOError(#[from] io::Error),

    #[error("From UTF8 Error")]
    FromUTF8Error(#[from]FromUtf8Error),

    #[error("Png Header Error")]
    InValidPngHeader
}