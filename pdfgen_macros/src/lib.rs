use proc_macro::TokenStream;

mod identifier;
mod write_chain;

/// Generate one or more `const Identifier<&'static [u8]>` values from the given identifiers.
/// Identifiers should be specified in upper snake case and will be converted to pascal-cased PDF
/// names. All leters will be replaced with their lowercase equivalents, except the first letter
/// and any letter preceded by an underscore. For example:
///
/// * `IDENT` -> `Ident`
/// * `SECOND_IDENT` -> `SecondIdent`
///
/// # Example
///
/// ```ignore
/// use pdfgen_macros::const_identifiers;
///
/// pub struct SomeStruct;
///
/// impl SomeStruct {
///     const_identifiers!(TYPE, SUBTYPE, MEDIA_BOX);
///
///     // expands to
///     const TYPE: Identifier<&'static [u8]> = Identifier::from_static(b"Type");
///     const SUBTYPE: Identifier<&'static [u8]> = Identifier::from_static(b"Subtype");
///     const MEDIA_BOX: Identifier<&'static [u8]> = Identifier::from_static(b"MediaBox");
/// }
/// ```
#[proc_macro]
pub fn const_identifiers(token_stream: TokenStream) -> TokenStream {
    identifier::const_identifiers(token_stream)
}

/// Helper macro for counting the number of written bytes in multiple consecutive writes, where
/// each write returns a `Result<usize, std::io::Error>`
///
/// # Example
///
/// ```ignore
/// let mut writer = Vec::new();
/// let optional = Some(1);
/// let iterable = vec![1, 2, 3];
///
/// let written = write_chain! {
///     writer.write(b"Hello"),
///     writer.write(b", World!"),
///
///     if let Some(value) = optional {
///         writer.write(format!(value).as_bytes()),
///     },
///
///     for value in iterable {
///         writer.write(format!(value).as_bytes()),
///     },
/// };
///
/// assert_eq!(written, 17);
/// ```
#[proc_macro]
pub fn write_chain(token_stream: TokenStream) -> TokenStream {
    write_chain::write_chain(token_stream)
}
