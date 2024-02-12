use binread::BinRead;
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use getset::Getters;

#[cfg(feature="lnk2json")]
use serde::Serialize;

use crate::strings::FixedSizeString;


/// The IconEnvironmentDataBlock structure specifies the path to an
/// icon. The path is encoded using environment variables, which makes
/// it possible to find the icon across machines where the locations
/// vary but are expressed using environment variables.
#[derive(Clone, Debug, BinRead, Getters)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[br(import(block_size: u32), pre_assert(block_size == 0x0000_00314))]
#[get(get = "pub")]
#[allow(unused)]
pub struct IconEnvironmentDataBlock {
    /// A NULL-terminated string, defined by the system default code
    /// page, which specifies a path that is constructed with
    /// environment variables.
    #[br(args(260, WINDOWS_1252), map=|s:FixedSizeString| s.to_string())]
    target_ansi: String,
    /// An optional, NULL-terminated, Unicode string that specifies a
    /// path that is constructed with environment variables.
    #[br(args(520, UTF_16LE), map=|s: FixedSizeString| if s.is_empty() {None} else {Some(s.to_string())})]
    target_unicode: Option<String>,
}
