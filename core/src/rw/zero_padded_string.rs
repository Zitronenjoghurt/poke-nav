use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroPaddedString<const N: usize>(pub String);

impl<const N: usize> BinRead for ZeroPaddedString<N> {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let bytes = <[u8; N]>::read_options(reader, endian, args)?;
        let s = String::from_utf8(bytes.into())
            .map(|s| s.split('\0').next().unwrap_or("").to_string())
            .map_err(|e| binrw::Error::Custom {
                pos: 0,
                err: Box::new(e),
            })?;
        Ok(Self(s))
    }
}

impl<const N: usize> BinWrite for ZeroPaddedString<N> {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let mut buf = [0u8; N];
        let len = self.0.len().min(N);
        buf[..len].copy_from_slice(&self.0.as_bytes()[..len]);
        buf.write_options(writer, endian, args)
    }
}

impl<const N: usize> std::ops::Deref for ZeroPaddedString<N> {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> std::fmt::Display for ZeroPaddedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const N: usize> AsRef<str> for ZeroPaddedString<N> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<const N: usize> From<ZeroPaddedString<N>> for String {
    fn from(s: ZeroPaddedString<N>) -> Self {
        s.0
    }
}

impl<const N: usize> From<&str> for ZeroPaddedString<N> {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl<const N: usize> From<String> for ZeroPaddedString<N> {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl<const N: usize> PartialEq<str> for ZeroPaddedString<N> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<const N: usize> PartialEq<&str> for ZeroPaddedString<N> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
