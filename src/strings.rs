use std::fmt::Display;

use binread::{BinRead, BinReaderExt, NullWideString};
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use log::trace;

use crate::LinkFlags;

pub fn trim_nul_terminated_string<S: Into<String>>(s: S) -> String {
    let s = s.into();
    let end_index = s.find('\0').unwrap_or(0);
    s[..end_index].to_string()
}

#[derive(Copy, Clone, Debug)]
pub enum StringEncoding {
    CodePage,
    Unicode,
}

impl From<LinkFlags> for StringEncoding {
    fn from(link_flags: LinkFlags) -> Self {
        if link_flags & LinkFlags::IS_UNICODE == LinkFlags::IS_UNICODE {
            Self::Unicode
        } else {
            Self::CodePage
        }
    }
}


#[derive(Clone, Debug)]
pub struct SizedString(String);


#[derive(Clone, Debug)]
pub struct NullTerminatedString(String);

impl BinRead for SizedString {
    type Args = (StringEncoding,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        let count_characters: u16 = reader.read_le()?;
        trace!("reading sized string of size '{count_characters}' at 0x{:08x}", reader.stream_position()?);

        match args.0 {
            StringEncoding::CodePage => {
                let mut buffer = vec![0; count_characters.into()];
                reader.read_exact(&mut buffer)?;
                let (cow, _, had_errors) = WINDOWS_1252.decode(&buffer);
                if had_errors {
                    return Err(binread::error::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!("unable to decode String to CP1252 from buffer {buffer:?}"),
                    });
                }
                Ok(Self(cow.to_string()))
            }
            StringEncoding::Unicode => {
                let mut buffer = vec![0; (count_characters*2).into()];
                reader.read_exact(&mut buffer)?;
                let (cow, _, had_errors) = UTF_16LE.decode(&buffer);
                if had_errors {
                    return Err(binread::error::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!("unable to decode String to UTF-16LE from buffer {buffer:?}"),
                    });
                }
                Ok(Self(cow.to_string()))
            }
        }
    }
}

impl Display for SizedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}


impl AsRef<str> for SizedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl BinRead for NullTerminatedString {
    type Args = (StringEncoding,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        match args.0 {
            StringEncoding::CodePage => {
                let mut buffer = Vec::new();
                loop {
                    let c: u8 = reader.read_le()?;
                    if c == 0 {
                        break;
                    } else {
                        buffer.push(c);
                    }
                }
                let (cow, _, had_errors) = WINDOWS_1252.decode(&buffer);
                if had_errors {
                    return Err(binread::error::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!("unable to decode String to CP1252 from buffer {buffer:?}"),
                    });
                }
                Ok(Self(cow.to_string()))
            }
            StringEncoding::Unicode => {
                let s: NullWideString = reader.read_le()?;
                Ok(Self(s.to_string()))
            }
        }
    }
}

impl Display for NullTerminatedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for NullTerminatedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}