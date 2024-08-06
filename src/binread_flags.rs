macro_rules! binread_flags {
    ($type: ty, $repr:ty) => {
        impl binrw::BinRead for $type {
            type Args<'a> = ();

            fn read_options<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                _endian: binrw::Endian,
                _args: Self::Args<'_>,
            ) -> binrw::BinResult<Self> {
                use binrw::BinReaderExt;
                let raw: $repr = reader.read_le()?;
                match Self::from_bits(raw) {
                    Some(res) => Ok(res),
                    None => Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!("unable to convert '0x{raw:x}' to {}", stringify!($type)),
                    }),
                }
            }
        }
    };
}

pub(crate) use binread_flags;
