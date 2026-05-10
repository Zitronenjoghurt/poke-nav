pub mod blz;

#[derive(Debug, thiserror::Error)]
pub enum CompressionError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    BinRw(#[from] binrw::Error),
    #[error("BLZ decompression out of bounds: src={src}, dst={dst}")]
    BlzOutOfBounds { src: usize, dst: usize },
    #[error("The data was not compressed")]
    NotCompressed,
}
