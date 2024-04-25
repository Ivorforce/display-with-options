use std::fmt::Formatter;

/// Like `Display`, but given options for formatting.
/// Does not auto-derive Debug. To easily use objects with this trait in format strings,
///  use `with_options`.
pub trait DisplayWithOptions<Options> {
    fn fmt(&self, f: &mut Formatter, options: &Options) -> std::fmt::Result;
}
